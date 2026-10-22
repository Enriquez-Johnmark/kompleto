//! Seam 2 (M6): what Kompleto writes to the activity log (J2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::engine::clock::Clock;
use kompleto::kompleto::activity;
use kompleto::kompleto::calendar::{BirForm, TaxPeriod};
use kompleto::kompleto::clients::{Clients, NewClient, ScheduleSettings};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::schedule::ReminderDays;
use sqlx::PgPool;
use time::macros::date;

/// The Firm's logged (actor, action) pairs, oldest first.
async fn logged(pool: &PgPool, firm: FirmId) -> Vec<(String, String)> {
    let since = clock().now() - time::Duration::days(1);
    let mut entries: Vec<(String, String)> = activity::since(pool, firm, since)
        .await
        .unwrap()
        .into_iter()
        .map(|entry| (entry.actor, entry.action))
        .collect();
    entries.reverse();
    entries
}

#[sqlx::test]
async fn j2_client_changes_are_logged_with_the_firm_as_actor(pool: PgPool) {
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock());

    let id = clients.create(santos, &nena()).await.unwrap();
    let renamed = NewClient {
        business_name: "Nena's Mini Mart".to_owned(),
        ..nena()
    };
    clients.update(santos, id, &renamed).await.unwrap();
    let paused = ScheduleSettings {
        cut_off_offset_days: None,
        reminder_days: ReminderDays::new(&[3]).unwrap(),
        paused: true,
    };
    clients.set_schedule(santos, id, &paused).await.unwrap();
    let q3 = TaxPeriod::Quarter { year: 2026, quarter: 3 };
    clients
        .set_due_override(santos, id, BirForm::F2551Q, q3, Some(date!(2026 - 10 - 30)))
        .await
        .unwrap();
    clients
        .create_all(
            santos,
            &[NewClient {
                business_name: "Bravo Hardware".to_owned(),
                ..nena()
            }],
        )
        .await
        .unwrap();

    let actor = common::firm_email("Santos Bookkeeping");
    let expected: Vec<(String, String)> = [
        "client.created",
        "client.updated",
        "client.schedule_changed",
        "client.due_date_set",
        "client.created",
    ]
    .iter()
    .map(|action| (actor.clone(), (*action).to_owned()))
    .collect();
    assert_eq!(logged(&pool, santos).await, expected);
}

#[sqlx::test]
async fn j2_checklist_and_payor_changes_are_logged(pool: PgPool) {
    use kompleto::kompleto::checklist::{ExtraItem, Recurs};
    use kompleto::kompleto::payors::Payors;

    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock());
    let id = clients.create(santos, &nena()).await.unwrap();
    let payors = Payors::new(pool.clone(), clock());

    clients
        .remove_default_item(santos, id, "bank-statements")
        .await
        .unwrap();
    clients
        .restore_default_item(santos, id, "bank-statements")
        .await
        .unwrap();
    let lazada = ExtraItem {
        label: "Lazada payout reports".to_owned(),
        recurs: Recurs::EveryPeriod,
    };
    let item = clients.add_extra_item(santos, id, &lazada).await.unwrap();
    clients.delete_extra_item(santos, id, item).await.unwrap();
    let sm = payors.add(santos, id, "SM Supermalls").await.unwrap();
    payors.remove(santos, id, sm).await.unwrap();

    let actions: Vec<String> = logged(&pool, santos)
        .await
        .into_iter()
        .skip(1)
        .map(|(_, action)| action)
        .collect();
    assert_eq!(
        actions,
        [
            "checklist.item_removed",
            "checklist.item_restored",
            "checklist.item_added",
            "checklist.item_deleted",
            "payor.added",
            "payor.removed",
        ]
    );
}
