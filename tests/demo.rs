//! The demo Firm (M8): its history is replayed through Kompleto's own code,
//! and nothing goes anywhere but `.test` addresses.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use common::RecordingMailer;
use kompleto::engine::clock::FixedClock;
use kompleto::engine::outbox::Sender;
use kompleto::kompleto::demo::{self, DemoServices, DemoSummary};
use sqlx::PgPool;
use time::OffsetDateTime;
use time::macros::{date, datetime};

/// Seeds the demo Firm as of 4 January 2027, with the clock starting at `now`.
async fn seed(pool: &PgPool, now: OffsetDateTime) -> (DemoSummary, RecordingMailer) {
    let clock = Arc::new(FixedClock::new(now));
    let mailer = RecordingMailer::idempotent();
    let checks = Arc::new(common::kompleto_checks(pool, clock.clone()));
    let sender = Sender::new(pool.clone(), mailer.clone(), clock.clone(), checks);
    let services = DemoServices {
        pool: pool.clone(),
        clock: clock.clone(),
        links: common::upload_links(pool, clock.clone()),
        files: common::files(pool, clock.clone()),
        base_url: "http://kompleto.test".to_owned(),
    };
    let summary = demo::seed(
        &services,
        "Santos Bookkeeping",
        "demo@santos.test",
        date!(2027 - 01 - 04),
        async || sender.drain_once(50).await,
    )
    .await
    .unwrap();
    (summary, mailer)
}

#[sqlx::test]
async fn the_demo_firm_has_clients_history_and_drafts_waiting(pool: PgPool) {
    let (summary, mailer) = seed(&pool, datetime!(2026-10-21 09:00 UTC)).await;

    assert_eq!(summary.clients, 15);
    assert!(summary.sent >= 10, "{summary:?}");
    assert!(summary.uploads >= 10, "{summary:?}");
    assert!(summary.replies >= 2, "{summary:?}");
    assert!(summary.waiting >= 5, "drafts for the Drafts page: {summary:?}");
    let sent = mailer.calls();
    assert!(!sent.is_empty());
    assert!(
        sent.iter().all(|(email, _)| email.to_address.ends_with(".test")),
        "only undeliverable addresses"
    );
    let bounced: i64 = sqlx::query_scalar("select count(*) from bounces")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(bounced >= 1, "one client's address bounces");
    // The bookkeeper kept up, except with the last few days' uploads.
    let (unchecked_earlier, unchecked_lately): (i64, i64) = sqlx::query_as(
        "select count(*) filter (where uploaded_at < '2027-01-01 00:00+08'),
                count(*) filter (where uploaded_at >= '2027-01-01 00:00+08')
         from uploads where received_at is null",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(unchecked_earlier, 0);
    assert!(unchecked_lately >= 1);
}

#[sqlx::test]
async fn the_demo_firm_gets_its_history_even_when_made_after_its_date(pool: PgPool) {
    // As with `DEMO_DATE` set, or a demo date that has passed.
    let (summary, _) = seed(&pool, datetime!(2027-02-01 09:00 UTC)).await;

    assert!(summary.sent >= 10, "{summary:?}");
    assert!(summary.uploads >= 10, "{summary:?}");
}
