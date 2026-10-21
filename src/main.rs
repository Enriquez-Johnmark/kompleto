//! The `kompleto` program.
//!
//! ```text
//! kompleto serve          web server + background sender
//! kompleto migrate        apply database migrations
//! kompleto outbox-drain   send what's due once, then exit
//! kompleto firm add --name NAME --email EMAIL
//!                         create a Firm (there is no sign-up page)
//! kompleto demo           (development) put one approved email through the outbox
//! kompleto parse-xlsx     (internal) the Excel import's sandboxed parser
//! ```

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, bail};
use axum::Router;
use axum::routing::get;
use kompleto::config::Config;
use kompleto::config::MailConfig;
use kompleto::engine::clock::{Clock, SystemClock};
use kompleto::engine::ids::TenantId;
use kompleto::engine::mail::Email;
use kompleto::engine::outbox::{Outbox, Sender};
use kompleto::kompleto::calendar::Calendar;
use kompleto::kompleto::firms::Firms;
use kompleto::kompleto::import::xlsx::{self, XlsxReader};
use kompleto::kompleto::reminders::Scheduler;
use kompleto::kompleto::web::{AppState, router};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

/// How many messages one drain sends at most.
const DRAIN_BATCH: usize = 50;
/// How often `serve` drains the outbox.
const DRAIN_EVERY: Duration = Duration::from_secs(30);
/// How often `serve` creates the Drafts that are due.
const SCHEDULE_EVERY: Duration = Duration::from_secs(60 * 60);

fn main() -> ExitCode {
    // The Excel parser runs before anything else starts (threads, the
    // database), so the limits it sets apply to nothing but the parsing.
    if std::env::args().nth(1).as_deref() == Some(xlsx::CHILD_COMMAND) {
        return xlsx::run_child();
    }
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("starting the async runtime")
        .and_then(|runtime| runtime.block_on(run()));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        // Development convenience only; production sets real variables.
        dotenvy::dotenv().ok();
    }
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let config = Config::from_env().context("reading configuration")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("connecting to Postgres")?;
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);

    match std::env::args().nth(1).as_deref() {
        Some("serve") => serve(&config, pool, clock).await,
        Some("migrate") => migrate(&pool).await,
        Some("outbox-drain") => {
            let report = Sender::new(pool, config.mailer()?, clock)
                .drain_once(DRAIN_BATCH)
                .await?;
            tracing::info!(?report, "outbox drained");
            Ok(())
        }
        Some("firm") => add_firm(pool, clock).await,
        Some("demo") => demo(&config, pool, clock).await,
        other => bail!("unknown command {other:?}; use serve, migrate, outbox-drain, firm add or demo"),
    }
}

async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(pool).await.context("running migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}

async fn serve(config: &Config, pool: PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    migrate(&pool).await?;
    let today = clock.now().date();
    if !Calendar::built_in()?.holidays_cover(today, today + time::Duration::days(365)) {
        tracing::warn!("bir-calendar.toml: national holidays aren't listed for the next 12 months");
    }
    let xlsx = XlsxReader::new(std::env::current_exe().context("finding this program for the Excel reader")?);
    let state = AppState::new(pool.clone(), clock.clone(), &config.public_base_url, xlsx)?;
    let wake_up = state.sender_wake_up();
    let scheduler = Scheduler::new(pool.clone(), clock.clone())?;
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(SCHEDULE_EVERY);
        loop {
            tick.tick().await;
            match scheduler.run_once().await {
                Ok(created) => tracing::info!(created, "reminder drafts created"),
                Err(err) => tracing::error!(%err, "creating reminder drafts failed"),
            }
        }
    });
    let sender = Sender::new(pool, config.mailer()?, clock);
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(DRAIN_EVERY);
        loop {
            // Drain on the timer, or at once when an email was just approved.
            tokio::select! {
                _ = tick.tick() => {}
                () = wake_up.notified() => {}
            }
            match sender.drain_once(DRAIN_BATCH).await {
                Ok(report) => tracing::debug!(?report, "outbox drained"),
                Err(err) => tracing::error!(%err, "outbox drain failed"),
            }
        }
    });
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .merge(router(state));
    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .context("binding the web server")?;
    tracing::info!(addr = %config.bind_addr, "listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .context("web server")?;
    Ok(())
}

async fn shutdown() {
    // If the signal handler can't be installed, keep serving until killed.
    if tokio::signal::ctrl_c().await.is_err() {
        std::future::pending::<()>().await;
    }
}

/// `kompleto firm add --name NAME --email EMAIL`
async fn add_firm(pool: PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(2).collect();
    let value = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1));
    let (Some("add"), Some(name), Some(email)) = (args.first().map(String::as_str), value("--name"), value("--email"))
    else {
        bail!("usage: kompleto firm add --name NAME --email EMAIL");
    };
    migrate(&pool).await?;
    let id = Firms::new(pool, clock).create(name, email).await?;
    tracing::info!(%id, "firm created; they can now log in at /login with {email}");
    Ok(())
}

/// Puts one sample email through draft → approve → send, so a developer can
/// see it arrive in Mailpit.
async fn demo(config: &Config, pool: PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    // The demo drains the whole outbox; never let it reach a real provider.
    if !matches!(config.mail, MailConfig::Mailpit { .. }) {
        bail!("demo only runs with MAIL_PROVIDER=mailpit");
    }
    migrate(&pool).await?;
    let outbox = Outbox::new(pool.clone(), clock.clone());
    let tenant = TenantId::new();
    let message = Email {
        to_address: "nena@example.test".to_owned(),
        from_name: "Santos Bookkeeping".to_owned(),
        reply_to: Some("reply@getkompleto.test".to_owned()),
        subject: "Kulang pa po ang September receipts".to_owned(),
        body_text: "Hi Nena,\n\nPaki-upload po ang September sales receipts at GCash screenshots \
                    bago Oct 20 para ma-file namin ang 2550Q on time.\n\nSalamat po!\nSantos Bookkeeping"
            .to_owned(),
    };
    let id = outbox.create_draft(tenant, &message).await?;
    let seen = outbox.get(tenant, id).await?.content_hash();
    outbox.approve(tenant, id, "demo@kompleto.test", &seen).await?;
    let report = Sender::new(pool, config.mailer()?, clock)
        .drain_once(DRAIN_BATCH)
        .await?;
    tracing::info!(?report, "demo email sent; open http://localhost:8025 to see it");
    Ok(())
}
