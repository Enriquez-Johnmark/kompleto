//! The `kompleto` program.
//!
//! ```text
//! kompleto serve          web server + background sender
//! kompleto migrate        apply database migrations
//! kompleto outbox-drain   send what's due once, then exit
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
use kompleto::kompleto::import::xlsx;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

/// How many messages one drain sends at most.
const DRAIN_BATCH: usize = 50;
/// How often `serve` drains the outbox.
const DRAIN_EVERY: Duration = Duration::from_secs(30);

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
        Some("demo") => demo(&config, pool, clock).await,
        other => bail!("unknown command {other:?}; use serve, migrate, outbox-drain or demo"),
    }
}

async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(pool).await.context("running migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}

async fn serve(config: &Config, pool: PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    migrate(&pool).await?;
    let sender = Sender::new(pool, config.mailer()?, clock);
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(DRAIN_EVERY);
        loop {
            tick.tick().await;
            match sender.drain_once(DRAIN_BATCH).await {
                Ok(report) => tracing::debug!(?report, "outbox drained"),
                Err(err) => tracing::error!(%err, "outbox drain failed"),
            }
        }
    });
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .layer(TraceLayer::new_for_http());
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
