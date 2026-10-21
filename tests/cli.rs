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
