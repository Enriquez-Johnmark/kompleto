//! Seam 1 (M6): the daily send budget under Resend Free's 100-a-day cap.
//! Messages with a deadline (Reminders) stop at the budget, nearest
//! deadline first; ones without (login links, notices) may use a small
//! reserve above it.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use common::{RecordingMailer, clock, sample_email};
use kompleto::engine::clock::Clock;
use kompleto::engine::ids::TenantId;
use kompleto::engine::mail::Email;
use kompleto::engine::outbox::{NoExtraChecks, Outbox, SendBudget, Sender};
use sqlx::PgPool;
use time::Duration;

/// An approved message to `to`, with a deadline `days` from now if any.
async fn approved(
    pool: &PgPool,
    outbox: &Outbox,
    clock: &Arc<kompleto::engine::clock::FixedClock>,
    to: &str,
    days: Option<i64>,
) {
    let tenant = TenantId::new();
    let email = Email {
        to_address: to.to_owned(),
        ..sample_email()
    };
    let mut tx = pool.begin().await.unwrap();
    let expires = days.map(|days| clock.now() + Duration::days(days));
    let id = outbox.create_draft_in(&mut tx, tenant, &email, expires).await.unwrap();
    tx.commit().await.unwrap();
    let seen = outbox.get(tenant, id).await.unwrap().content_hash();
    outbox
        .approve(tenant, id, "a@firm.test", &seen, &NoExtraChecks)
        .await
        .unwrap();
}

#[sqlx::test]
async fn send_budget_holds_reminders_past_it_nearest_deadline_first(pool: PgPool) {
    let clock = clock();
    let outbox = Outbox::new(pool.clone(), clock.clone());
    for (to, days) in [("d5@x.test", 5), ("d2@x.test", 2), ("d9@x.test", 9), ("d1@x.test", 1)] {
        approved(&pool, &outbox, &clock, to, Some(days)).await;
    }
    approved(&pool, &outbox, &clock, "login1@x.test", None).await;
    let mailer = RecordingMailer::idempotent();
    let sender = Sender::new(pool.clone(), mailer.clone(), clock.clone(), Arc::new(NoExtraChecks))
        .with_budget(SendBudget { per_day: 3, reserve: 1 });
    let sent = |mailer: &RecordingMailer| -> Vec<String> {
        mailer.calls().into_iter().map(|(email, _)| email.to_address).collect()
    };

    sender.drain_once(50).await.unwrap();
    assert_eq!(sent(&mailer), ["login1@x.test", "d1@x.test", "d2@x.test"]);

    // The Reminders' budget is used up; a login link still has the reserve.
    approved(&pool, &outbox, &clock, "login2@x.test", None).await;
    approved(&pool, &outbox, &clock, "login3@x.test", None).await;
    sender.drain_once(50).await.unwrap();
    assert_eq!(sent(&mailer).len(), 4);
    assert_eq!(sent(&mailer)[3], "login2@x.test");

    clock.advance(Duration::hours(24));
    sender.drain_once(50).await.unwrap();
    assert_eq!(sent(&mailer)[4..], ["login3@x.test", "d5@x.test", "d9@x.test"]);
}
