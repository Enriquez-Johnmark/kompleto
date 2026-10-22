//! The `kompleto` command line, run as a real process.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::process::{Command, Output};

use common::clock;
use kompleto::kompleto::firms::Firms;
use sqlx::PgPool;

async fn kompleto(pool: &PgPool, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kompleto"));
    cmd.args(args)
        .env("DATABASE_URL", common::database_url(pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@getkompleto.test")
        .env("RUST_LOG", "warn");
    tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap()
}

#[sqlx::test]
async fn firm_add_creates_a_firm_that_can_log_in(pool: PgPool) {
    let added = kompleto(
        &pool,
        &[
            "firm",
            "add",
            "--name",
            "Santos Bookkeeping",
            "--email",
            "ana@santos.test",
        ],
    )
    .await;

    assert!(added.status.success(), "{}", String::from_utf8_lossy(&added.stderr));
    let firm = Firms::new(pool.clone(), clock())
        .find_by_login_email("ana@santos.test")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(firm.name, "Santos Bookkeeping");

    let again = kompleto(&pool, &["firm", "add", "--name", "Other", "--email", "ana@santos.test"]).await;
    assert!(!again.status.success());
    assert!(String::from_utf8_lossy(&again.stderr).contains("already uses that login email"));
}

#[sqlx::test]
async fn j1_llm_costs_lists_every_firms_cost_for_a_month(pool: PgPool) {
    use kompleto::engine::cost_log::{self, LlmCall, Outcome};
    use kompleto::engine::llm::Usage;
    use time::macros::datetime;

    let santos = common::firm(&pool, "Santos Bookkeeping").await;
    let reyes = common::firm(&pool, "Reyes CPA").await;
    for (tenant, input) in [(santos, 1_000_000), (reyes, 2_000_000)] {
        let call = LlmCall {
            tenant,
            at: datetime!(2026-11-10 02:00 UTC),
            model: "claude-haiku-5-5".to_owned(),
            usage: Usage {
                input_tokens: input,
                output_tokens: 0,
            },
            request_id: None,
            outcome: Outcome::Ok,
        };
        cost_log::record(&pool, &call).await.unwrap();
    }

    let report = kompleto(&pool, &["llm-costs", "--month", "2026-11"]).await;

    assert!(report.status.success(), "{}", String::from_utf8_lossy(&report.stderr));
    let out = String::from_utf8_lossy(&report.stdout);
    let lines: Vec<&str> = out.lines().collect();
    assert!(lines[0].contains("2026-11"), "{out}");
    assert!(
        lines[1].contains("Reyes CPA") && lines[1].contains("US$0.2000"),
        "{out}"
    );
    assert!(
        lines[2].contains("Santos Bookkeeping") && lines[2].contains("US$0.1000"),
        "{out}"
    );
    assert!(out.contains("Total") && out.contains("US$0.3000"), "{out}");
}

#[test]
fn gen_key_prints_a_new_random_master_key() {
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_kompleto"))
            .arg("gen-key")
            .output()
            .unwrap()
    };
    let (one, two) = (run(), run());

    assert!(one.status.success(), "{}", String::from_utf8_lossy(&one.stderr));
    let key = String::from_utf8(one.stdout).unwrap();
    kompleto::engine::files::MasterKey::from_hex(&key).unwrap();
    assert_ne!(key, String::from_utf8(two.stdout).unwrap());
}

#[sqlx::test]
async fn f3_the_server_refuses_to_start_without_a_file_master_key(pool: PgPool) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kompleto"));
    cmd.arg("serve")
        .env("DATABASE_URL", common::database_url(&pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@getkompleto.test")
        .env("FILE_MASTER_KEY", "")
        .env("BIND_ADDR", "127.0.0.1:0");
    let out = tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap();

    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("FILE_MASTER_KEY"));
}

#[sqlx::test]
async fn dev_schedule_creates_the_drafts_due_on_a_chosen_date(pool: PgPool) {
    let santos = common::firm(&pool, "Santos Bookkeeping").await;
    let employer = kompleto::kompleto::clients::NewClient {
        forms: vec![kompleto::kompleto::calendar::BirForm::F1601C],
        ..common::nena()
    };
    kompleto::kompleto::clients::Clients::new(pool.clone(), clock())
        .create(santos, &employer)
        .await
        .unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kompleto"));
    cmd.args(["dev-schedule", "--date", "2026-11-01"])
        .env("DATABASE_URL", common::database_url(&pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@getkompleto.test")
        .env("PUBLIC_BASE_URL", "http://127.0.0.1:3000")
        .env("FILE_MASTER_KEY", "ab".repeat(32))
        .env("RUST_LOG", "warn");

    let out = tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap();

    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let body: String = sqlx::query_scalar("select body_text from outbox_messages")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(body.contains("http://127.0.0.1:3000/u/"), "{body}");
    assert!(String::from_utf8_lossy(&out.stdout).contains("1 reminder draft"));
}

#[sqlx::test]
async fn dev_schedule_only_runs_against_the_local_test_mailbox(pool: PgPool) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kompleto"));
    cmd.args(["dev-schedule", "--date", "2026-11-01"])
        .env("DATABASE_URL", common::database_url(&pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@getkompleto.test")
        .env("MAIL_PROVIDER", "resend")
        .env("RESEND_API_KEY", "re_test")
        .env("FILE_MASTER_KEY", "ab".repeat(32));

    let out = tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap();

    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("development only"));
}

#[sqlx::test]
async fn f1_the_live_server_wont_start_without_a_virus_scanner(pool: PgPool) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kompleto"));
    cmd.arg("serve")
        .env("DATABASE_URL", common::database_url(&pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@getkompleto.test")
        .env("MAIL_PROVIDER", "resend")
        .env("RESEND_API_KEY", "re_test")
        .env("FILE_MASTER_KEY", "ab".repeat(32))
        .env("BIND_ADDR", "127.0.0.1:0")
        .env_remove("CLAMD_ADDR");

    let out = tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap();

    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("CLAMD_ADDR must be set"));
}
