//! The `kompleto` program.
//!
//! ```text
//! kompleto serve          web server + background sender
//! kompleto migrate        apply database migrations
//! kompleto outbox-drain   send what's due once, then exit
//! kompleto firm add --name NAME --email EMAIL
//!                         create a Firm (there is no sign-up page)
//! kompleto llm-costs --month YYYY-MM
//!                         every Firm's AI cost for a month (J1)
//! kompleto gen-key        print a new FILE_MASTER_KEY
//! kompleto dev-schedule --date YYYY-MM-DD
//!                         (development) create the reminder Drafts due on that date
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
use kompleto::engine::clock::{Clock, FixedClock, SystemClock};
use kompleto::engine::cost_log;
use kompleto::engine::files::{FileCipher, Files, LocalDisk, MasterKey};
use kompleto::engine::ids::TenantId;
use kompleto::engine::mail::Email;
use kompleto::engine::outbox::{NoExtraChecks, Outbox, Sender};
use kompleto::kompleto::calendar::Calendar;
use kompleto::kompleto::closing;
use kompleto::kompleto::firms::Firms;
use kompleto::kompleto::import::xlsx::{self, XlsxReader};
use kompleto::kompleto::presend::KompletoChecks;
use kompleto::kompleto::reminders::Scheduler;
use kompleto::kompleto::replies;
use kompleto::kompleto::schedule::MANILA;
use kompleto::kompleto::upload_links::UploadLinks;
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

    if std::env::args().nth(1).as_deref() == Some("gen-key") {
        println!("{}", MasterKey::generate_hex()?);
        return Ok(());
    }
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
            let links = UploadLinks::new(
                pool.clone(),
                clock.clone(),
                &config.master_key()?,
                &config.public_base_url,
            );
            let checks = Arc::new(KompletoChecks::new(
                pool.clone(),
                clock.clone(),
                &config.public_base_url,
                links,
            ));
            let mut sender = Sender::new(pool, config.mailer()?, clock, checks);
            if let Some(budget) = config.send_budget() {
                sender = sender.with_budget(budget);
            }
            let report = sender.drain_once(DRAIN_BATCH).await?;
            tracing::info!(?report, "outbox drained");
            Ok(())
        }
        Some("firm") => add_firm(pool, clock).await,
        Some("llm-costs") => llm_costs(pool, clock).await,
        Some("demo") => demo(&config, pool, clock).await,
        Some("dev-schedule") => dev_schedule(&config, pool).await,
        other => {
            bail!(
                "unknown command {other:?}; use serve, migrate, outbox-drain, firm add, firm reopen, llm-costs, gen-key, demo or dev-schedule"
            )
        }
    }
}

async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(pool).await.context("running migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}

async fn serve(config: &Config, pool: PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    let master = config.master_key()?;
    migrate(&pool).await?;
    let today = clock.now().date();
    if !Calendar::built_in()?.holidays_cover(today, today + time::Duration::days(365)) {
        tracing::warn!("bir-calendar.toml: national holidays aren't listed for the next 12 months");
    }
    let xlsx = XlsxReader::new(std::env::current_exe().context("finding this program for the Excel reader")?);
    let files = Files::new(
        pool.clone(),
        clock.clone(),
        Arc::new(LocalDisk::new(&config.upload_dir)),
        Arc::new(FileCipher::new(&master)),
    );
    let state = AppState::new(
        pool.clone(),
        clock.clone(),
        &config.public_base_url,
        xlsx,
        &master,
        files.clone(),
    )?;
    let state = match &config.resend_webhook_secret {
        Some(secret) => state.with_delivery_events(secret.clone()),
        None => state,
    };
    let state = match &config.support_email {
        Some(address) => state.with_support_email(address),
        None => state,
    };
    let state = match &config.inbound_secret {
        Some(secret) => state.with_inbound(secret.clone(), config.reply_domain()),
        None => {
            tracing::warn!("INBOUND_SECRET isn't set; client replies won't be taken in");
            state
        }
    };
    let wake_up = state.sender_wake_up();
    let model = config.language_model()?;
    if model.is_none() {
        tracing::warn!("ANTHROPIC_API_KEY isn't set; every reminder draft will be a template");
    }
    let links = UploadLinks::new(pool.clone(), clock.clone(), &master, &config.public_base_url)
        .with_replies(config.reply_domain());
    let scheduler = Scheduler::with_model(pool.clone(), clock.clone(), links.clone(), model)?;
    let (sweep_pool, sweep_clock) = (pool.clone(), clock.clone());
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(SCHEDULE_EVERY);
        loop {
            tick.tick().await;
            match scheduler.run_once().await {
                Ok(created) => tracing::info!(created, "reminder drafts created"),
                Err(err) => tracing::error!(%err, "creating reminder drafts failed"),
            }
            match files.sweep().await {
                Ok(deleted) => tracing::info!(deleted, "files past their deadline deleted"),
                Err(err) => tracing::error!(%err, "deleting files failed"),
            }
            match replies::forget_old_text(&sweep_pool, sweep_clock.now()).await {
                Ok(forgotten) => tracing::info!(forgotten, "reply text past 90 days deleted"),
                Err(err) => tracing::error!(%err, "deleting old reply text failed"),
            }
            match closing::erase_due(&sweep_pool, &files, sweep_clock.now()).await {
                Ok(0) => {}
                Ok(erased) => tracing::info!(erased, "closed accounts erased"),
                Err(err) => tracing::error!(%err, "erasing closed accounts failed"),
            }
        }
    });
    let checks = Arc::new(KompletoChecks::new(
        pool.clone(),
        clock.clone(),
        &config.public_base_url,
        links,
    ));
    let mut sender = Sender::new(pool, config.mailer()?, clock, checks);
    if let Some(budget) = config.send_budget() {
        sender = sender.with_budget(budget);
    }
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
    if let (Some("reopen"), Some(email)) = (args.first().map(String::as_str), value("--email")) {
        if closing::reopen(&pool, &clock, email).await? {
            tracing::info!("account reopened; {email} can log in again");
            return Ok(());
        }
        bail!("no closed account still waiting to be erased logs in as {email}");
    }
    let (Some("add"), Some(name), Some(email)) = (args.first().map(String::as_str), value("--name"), value("--email"))
    else {
        bail!("usage: kompleto firm add --name NAME --email EMAIL, or kompleto firm reopen --email EMAIL");
    };
    migrate(&pool).await?;
    let id = Firms::new(pool, clock).create(name, email).await?;
    tracing::info!(%id, "firm created; they can now log in at /login with {email}");
    Ok(())
}

/// `kompleto llm-costs --month YYYY-MM`: every Firm's model cost for the
/// month (Manila time), most expensive first.
async fn llm_costs(pool: PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(2).collect();
    let month = args.iter().position(|a| a == "--month").and_then(|i| args.get(i + 1));
    let Some((year, month)) = month.and_then(|m| m.split_once('-')).and_then(|(y, m)| {
        Some((
            y.parse::<i32>().ok()?,
            time::Month::try_from(m.parse::<u8>().ok()?).ok()?,
        ))
    }) else {
        bail!("usage: kompleto llm-costs --month YYYY-MM");
    };
    let (from, to) = kompleto::kompleto::dates::manila_month(year, month);
    let firms = Firms::new(pool.clone(), clock);
    let mut total = cost_log::Totals::default();
    println!("AI cost for {year}-{:02} (Manila time)", u8::from(month));
    for (tenant, totals) in cost_log::totals_by_tenant(&pool, from, to).await? {
        let name = firms
            .get(tenant)
            .await
            .map_or_else(|_| tenant.to_string(), |firm| firm.name);
        println!("{name:<40} {:>6} calls  US${:.4}", totals.calls, totals.cost_usd());
        total.calls += totals.calls;
        total.cost_nano_usd += totals.cost_nano_usd;
    }
    println!("{:<40} {:>6} calls  US${:.4}", "Total", total.calls, total.cost_usd());
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
    outbox
        .approve(tenant, id, "demo@kompleto.test", &seen, &NoExtraChecks)
        .await?;
    let report = Sender::new(pool, config.mailer()?, clock, Arc::new(NoExtraChecks))
        .drain_once(DRAIN_BATCH)
        .await?;
    tracing::info!(?report, "demo email sent; open http://localhost:8025 to see it");
    Ok(())
}

/// `kompleto dev-schedule --date YYYY-MM-DD` (development only): creates the
/// reminder Drafts due on that date, as if it were 07:30 there in Manila, so
/// the whole loop can be tried before real dates come round. They are then
/// approved and sent from the Drafts page as usual.
async fn dev_schedule(config: &Config, pool: PgPool) -> anyhow::Result<()> {
    if !matches!(config.mail, MailConfig::Mailpit { .. }) {
        bail!("dev-schedule is for development only and runs only with MAIL_PROVIDER=mailpit");
    }
    let args: Vec<String> = std::env::args().skip(2).collect();
    let date = args
        .iter()
        .position(|a| a == "--date")
        .and_then(|i| args.get(i + 1))
        .and_then(|d| time::Date::parse(d, time::macros::format_description!("[year]-[month]-[day]")).ok());
    let Some(date) = date else {
        bail!("usage: kompleto dev-schedule --date YYYY-MM-DD");
    };
    migrate(&pool).await?;
    let clock: Arc<dyn Clock> = Arc::new(FixedClock::new(date.with_hms(7, 30, 0)?.assume_offset(MANILA)));
    let links = UploadLinks::new(
        pool.clone(),
        clock.clone(),
        &config.master_key()?,
        &config.public_base_url,
    )
    .with_replies(config.reply_domain());
    let created = Scheduler::new(pool, clock, links)?.run_once().await?;
    println!("{created} reminder draft(s) created as of {date}; approve them on the Drafts page.");
    Ok(())
}
