//! Seam 2 (M6): deleting a closed account's data (I2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::{clock, firm, nena};
use kompleto::engine::clock::{Clock, FixedClock};
use kompleto::engine::files::{FileCipher, Files, LocalDisk};
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::closing::{close, erase_due};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::payors::Payors;
use kompleto::kompleto::uploads::{Label, Uploads};
use sqlx::PgPool;
use time::Duration;
use time::macros::date;

/// Every table that holds a Firm's rows, and its column naming the Firm.
const TABLES: [(&str, &str); 18] = [
    ("firms", "id"),
    ("clients", "firm_id"),
    ("payors", "firm_id"),
    ("client_extra_items", "firm_id"),
    ("client_removed_items", "firm_id"),
    ("due_date_overrides", "firm_id"),
    ("scheduled_drafts", "firm_id"),
    ("upload_links", "firm_id"),
    ("uploads", "firm_id"),
    ("replies", "firm_id"),
    ("bounces", "firm_id"),
    ("sessions", "firm_id"),
    ("login_tokens", "firm_id"),
    ("outbox_messages", "tenant_id"),
    ("jobs", "tenant_id"),
    ("llm_calls", "tenant_id"),
    ("stored_files", "tenant_id"),
    ("audit_events", "tenant_id"),
];

async fn rows(pool: &PgPool, firm: FirmId) -> i64 {
    let mut total = 0;
    for (table, column) in TABLES {
        let sql = format!("select count(*) from {table} where {column} = $1");
        let count: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(sql))
            .bind(firm.as_uuid())
            .fetch_one(pool)
            .await
            .unwrap();
        total += count;
    }
    total
}

/// A Firm with a Client, a Payor, an uploaded file and a Reply.
async fn busy_firm(pool: &PgPool, clock: &Arc<FixedClock>, files: &Files, name: &str) -> FirmId {
    let id = firm(pool, name).await;
    let client = Clients::new(pool.clone(), clock.clone())
        .create(id, &nena())
        .await
        .unwrap();
    Payors::new(pool.clone(), clock.clone())
        .add(id, client, "Jollibee Foods Corp")
        .await
        .unwrap();
    let links = common::upload_links(pool, clock.clone());
    let url = links
        .for_period(id, client, "2026-09", date!(2026 - 10 - 26))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    Uploads::new(pool.clone(), clock.clone(), files.clone())
        .add(
            &link,
            &[],
            Label::SomethingElse {
                note: "resibo".to_owned(),
            },
            b"%PDF-1.7",
        )
        .await
        .unwrap();
    sqlx::query(
        "insert into replies (id, firm_id, client_id, period, from_address, automatic, received_at)
         values ($1, $2, $3, '2026-09', 'nena@example.test', false, now())",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(id.as_uuid())
    .bind(client.as_uuid())
    .execute(pool)
    .await
    .unwrap();
    id
}

#[sqlx::test]
async fn i2_a_closed_accounts_data_and_files_are_all_deleted_after_7_days(pool: PgPool) {
    let clock = clock();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(uuid::Uuid::now_v7().to_string());
    let files = Files::new(
        pool.clone(),
        clock.clone(),
        Arc::new(LocalDisk::new(&root)),
        Arc::new(FileCipher::new(&common::master_key())),
    );
    let santos = busy_firm(&pool, &clock, &files, "Santos Bookkeeping").await;
    let reyes = busy_firm(&pool, &clock, &files, "Reyes CPA").await;
    let (santos_rows, reyes_rows) = (rows(&pool, santos).await, rows(&pool, reyes).await);
    let dynamic: Arc<dyn kompleto::engine::clock::Clock> = clock.clone();
    close(
        &pool,
        &dynamic,
        santos,
        "Santos Bookkeeping",
        "santos.bookkeeping@firm.test",
    )
    .await
    .unwrap();

    clock.advance(Duration::days(6));
    assert_eq!(
        erase_due(&pool, &files, clock.now()).await.unwrap(),
        0,
        "not before 7 days"
    );
    assert!(rows(&pool, santos).await > santos_rows, "still there, plus the closing");
    clock.advance(Duration::days(1));
    assert_eq!(erase_due(&pool, &files, clock.now()).await.unwrap(), 1);
    assert_eq!(erase_due(&pool, &files, clock.now()).await.unwrap(), 0, "once");

    assert_eq!(rows(&pool, santos).await, 0, "nothing of the Firm left");
    assert!(
        !root.join(santos.to_string()).exists()
            || std::fs::read_dir(root.join(santos.to_string()))
                .unwrap()
                .next()
                .is_none()
    );
    assert_eq!(rows(&pool, reyes).await, reyes_rows, "another Firm's untouched");
    assert!(
        std::fs::read_dir(root.join(reyes.to_string()))
            .unwrap()
            .next()
            .is_some()
    );
    let refused = sqlx::query("delete from audit_events where tenant_id = $1")
        .bind(reyes.as_uuid())
        .execute(&pool)
        .await;
    assert!(refused.is_err(), "the log stays append-only otherwise");
}

#[sqlx::test]
async fn i2_an_operator_can_reopen_a_closed_account_before_it_is_erased(pool: PgPool) {
    use kompleto::kompleto::closing::reopen;
    use kompleto::kompleto::firms::Firms;

    let clock = clock();
    let files = common::files(&pool, clock.clone());
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let dynamic: Arc<dyn kompleto::engine::clock::Clock> = clock.clone();
    close(
        &pool,
        &dynamic,
        santos,
        "Santos Bookkeeping",
        "santos.bookkeeping@firm.test",
    )
    .await
    .unwrap();

    assert!(reopen(&pool, &dynamic, "SANTOS.bookkeeping@firm.test").await.unwrap());
    assert!(!reopen(&pool, &dynamic, "nobody@firm.test").await.unwrap());

    let firms = Firms::new(pool.clone(), clock.clone());
    assert!(
        firms
            .find_by_login_email("santos.bookkeeping@firm.test")
            .await
            .unwrap()
            .is_some(),
        "can log in"
    );
    clock.advance(Duration::days(8));
    assert_eq!(erase_due(&pool, &files, clock.now()).await.unwrap(), 0, "not erased");
    let logged: i64 =
        sqlx::query_scalar("select count(*) from audit_events where tenant_id = $1 and action = 'firm.reopened'")
            .bind(santos.as_uuid())
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(logged, 1);
}

#[sqlx::test]
async fn i2_nothing_goes_out_for_a_closed_account_not_even_a_retry(pool: PgPool) {
    use kompleto::engine::ids::MessageId;
    use kompleto::engine::outbox::{CheckFailed, PreSendCheck};

    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let dynamic: Arc<dyn kompleto::engine::clock::Clock> = clock.clone();
    close(
        &pool,
        &dynamic,
        santos,
        "Santos Bookkeeping",
        "santos.bookkeeping@firm.test",
    )
    .await
    .unwrap();
    let checks = common::kompleto_checks(&pool, clock);

    let refused = checks.check(santos, MessageId::new(), &common::sample_email()).await;

    assert!(
        matches!(&refused, Err(CheckFailed::Refused(reason)) if reason.contains("closed")),
        "{refused:?}"
    );
}
