//! Seam 2 (M5): Bounces and complaints against Postgres (G3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use common::{clock, firm, nena};
use kompleto::engine::clock::{Clock, FixedClock};
use kompleto::engine::webhooks::DeliveryEvent;
use kompleto::kompleto::bounces::{Bounces, Recorded};
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{ClientId, Clients, NewClient};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;
use time::Duration;

fn employer() -> NewClient {
    NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    }
}

/// October's first Reminder went out on 1 Nov as provider message `re_1`;
/// on 2 Nov the next one is waiting.
async fn sent_and_waiting(pool: &PgPool) -> (FirmId, ClientId, Arc<FixedClock>, Scheduler) {
    let clock = clock();
    clock.advance(Duration::days(11));
    let santos = firm(pool, "Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(pool, clock.clone())).unwrap();
    scheduler.run_once().await.unwrap();
    sqlx::query(
        "update outbox_messages set status = 'sent', approved_hash = '\\x00', provider_message_id = 're_1', sent_at = $1",
    )
    .bind(clock.now())
    .execute(pool)
    .await
    .unwrap();
    clock.advance(Duration::days(1));
    scheduler.run_once().await.unwrap();
    (santos, client, clock, scheduler)
}

async fn statuses(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("select status from outbox_messages order by created_at")
        .fetch_all(pool)
        .await
        .unwrap()
}

fn bounce(id: &str, permanent: bool) -> DeliveryEvent {
    DeliveryEvent::Bounced {
        provider_message_id: id.to_owned(),
        permanent,
        reason: "Mailbox does not exist".to_owned(),
    }
}

#[sqlx::test]
async fn g3_a_permanent_bounce_stops_the_clients_reminders_until_its_email_changes(pool: PgPool) {
    let (santos, client, clock, scheduler) = sent_and_waiting(&pool).await;
    let bounces = Bounces::new(pool.clone(), clock.clone());

    let recorded = bounces.record(&bounce("re_1", true)).await.unwrap();

    assert_eq!(recorded, Recorded::Stopped);
    assert_eq!(statuses(&pool).await, ["sent", "cancelled"]);
    clock.advance(Duration::days(2)); // 4 Nov: the day before the Cut-off
    scheduler.run_once().await.unwrap();
    assert_eq!(statuses(&pool).await.len(), 2, "no Reminder to a bouncing address");
    let clients = Clients::new(pool.clone(), clock.clone());
    let moved = NewClient {
        email: "nena.new@example.test".to_owned(),
        ..employer()
    };
    clients.update(santos, client, &moved).await.unwrap();
    scheduler.run_once().await.unwrap();
    assert_eq!(statuses(&pool).await.len(), 3, "a new address gets Reminders again");
    assert_eq!(bounces.for_client(santos, client).await.unwrap()[0].kind, "bounce");
}

#[sqlx::test]
async fn g3_a_late_bounce_for_an_old_address_leaves_reminders_to_the_new_one(pool: PgPool) {
    let (santos, client, clock, scheduler) = sent_and_waiting(&pool).await;
    let moved = NewClient {
        email: "nena.new@example.test".to_owned(),
        ..employer()
    };
    Clients::new(pool.clone(), clock.clone())
        .update(santos, client, &moved)
        .await
        .unwrap();
    clock.advance(Duration::days(2)); // 4 Nov: a Reminder to the new address
    scheduler.run_once().await.unwrap();

    let recorded = Bounces::new(pool.clone(), clock)
        .record(&bounce("re_1", true))
        .await
        .unwrap();

    assert_eq!(recorded, Recorded::Noted);
    assert_eq!(
        statuses(&pool).await,
        ["sent", "cancelled", "draft"],
        "only the old address's"
    );
}

#[sqlx::test]
async fn g3_a_complaint_stops_reminders_and_a_temporary_bounce_is_only_noted(pool: PgPool) {
    let (_, _, clock, _) = sent_and_waiting(&pool).await;
    let bounces = Bounces::new(pool.clone(), clock);

    assert_eq!(bounces.record(&bounce("re_1", false)).await.unwrap(), Recorded::Noted);
    assert_eq!(statuses(&pool).await, ["sent", "draft"]);
    let complaint = DeliveryEvent::Complained {
        provider_message_id: "re_1".to_owned(),
    };
    assert_eq!(bounces.record(&complaint).await.unwrap(), Recorded::Stopped);
    assert_eq!(statuses(&pool).await, ["sent", "cancelled"]);
}

#[sqlx::test]
async fn g3_only_events_for_reminders_we_sent_count(pool: PgPool) {
    let (_, _, clock, _) = sent_and_waiting(&pool).await;
    let bounces = Bounces::new(pool.clone(), clock);

    let unknown = bounces.record(&bounce("re_somebody_else", true)).await.unwrap();
    let delivered = bounces.record(&DeliveryEvent::Other).await.unwrap();

    assert_eq!((unknown, delivered), (Recorded::Ignored, Recorded::Ignored));
    let kept: i64 = sqlx::query_scalar("select count(*) from bounces")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(kept, 0);
    assert_eq!(statuses(&pool).await, ["sent", "draft"]);
}
