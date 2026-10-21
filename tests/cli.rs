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
