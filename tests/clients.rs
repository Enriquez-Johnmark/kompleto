//! Seam 2: Firm and Client repositories against real Postgres (B1, A2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{ClientError, Clients, Language};
use kompleto::kompleto::firms::{FirmError, Firms};
use sqlx::PgPool;

#[sqlx::test]
async fn b1_a_client_added_by_form_keeps_every_field(pool: PgPool) {
    let clients = Clients::new(pool.clone(), clock());
    let santos = firm(&pool, "Santos Bookkeeping").await;

    let id = clients.create(santos, &nena()).await.unwrap();

    let client = clients.get(santos, id).await.unwrap();
    assert_eq!(client.details, nena());
    assert_eq!(client.details.language, Language::Taglish);
    assert_eq!(client.details.forms, [BirForm::F2551Q, BirForm::F1701Q]);
}

#[sqlx::test]
async fn b1_business_names_are_unique_within_a_firm_ignoring_case(pool: PgPool) {
    let clients = Clients::new(pool.clone(), clock());
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    clients.create(santos, &nena()).await.unwrap();
    let mut shouting = nena();
    shouting.business_name = shouting.business_name.to_uppercase();

    let again = clients.create(santos, &shouting).await;

    assert!(matches!(again, Err(ClientError::DuplicateName(_))), "{again:?}");
    clients.create(reyes, &shouting).await.unwrap();
}

#[sqlx::test]
async fn a2_another_firms_client_is_not_found(pool: PgPool) {
    let clients = Clients::new(pool.clone(), clock());
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let id = clients.create(santos, &nena()).await.unwrap();

    assert!(matches!(clients.get(reyes, id).await, Err(ClientError::NotFound)));
    assert!(matches!(
        clients.update(reyes, id, &nena()).await,
        Err(ClientError::NotFound)
    ));
    assert!(clients.list(reyes).await.unwrap().is_empty());
    assert_eq!(clients.list(santos).await.unwrap().len(), 1);
}

#[sqlx::test]
async fn a_login_email_belongs_to_one_firm(pool: PgPool) {
    let firms = Firms::new(pool.clone(), clock());
    firms.create("Santos Bookkeeping", "ana@santos.test").await.unwrap();

    let again = firms.create("Other", "ANA@santos.test").await;

    assert!(matches!(again, Err(FirmError::LoginEmailTaken)), "{again:?}");
    let found = firms.find_by_login_email(" Ana@Santos.test ").await.unwrap().unwrap();
    assert_eq!(found.name, "Santos Bookkeeping");
}

fn numbered(n: usize) -> kompleto::kompleto::clients::NewClient {
    let mut client = nena();
    client.business_name = format!("Client {n}");
    client
}

#[sqlx::test]
async fn b2_an_import_adds_every_valid_row(pool: PgPool) {
    let clients = Clients::new(pool.clone(), clock());
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let rows: Vec<_> = (1..=60).map(numbered).collect();

    let added = clients.create_all(santos, &rows).await.unwrap();

    assert_eq!(added.len(), 60);
    assert_eq!(clients.list(santos).await.unwrap().len(), 60);
}

#[sqlx::test]
async fn b2_a_failed_import_changes_nothing(pool: PgPool) {
    let clients = Clients::new(pool.clone(), clock());
    let santos = firm(&pool, "Santos Bookkeeping").await;
    // Added after the preview was checked, so row 30 now clashes.
    clients.create(santos, &numbered(30)).await.unwrap();
    let rows: Vec<_> = (1..=60).map(numbered).collect();

    let result = clients.create_all(santos, &rows).await;

    assert!(matches!(result, Err(ClientError::DuplicateName(_))), "{result:?}");
    assert_eq!(clients.list(santos).await.unwrap().len(), 1);
}

#[sqlx::test]
async fn b3_a_due_date_override_is_kept_for_one_client_only(pool: PgPool) {
    use kompleto::kompleto::calendar::TaxPeriod;
    use time::macros::date;

    let clients = Clients::new(pool.clone(), clock());
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let nena_id = clients.create(santos, &nena()).await.unwrap();
    let other = clients.create(santos, &numbered(1)).await.unwrap();
    let q3 = TaxPeriod::Quarter { year: 2026, quarter: 3 };

    clients
        .set_due_override(santos, nena_id, BirForm::F2551Q, q3, Some(date!(2026 - 10 - 30)))
        .await
        .unwrap();

    let overrides = clients.due_overrides(santos, nena_id).await.unwrap();
    assert_eq!(overrides.get(&(BirForm::F2551Q, q3)), Some(&date!(2026 - 10 - 30)));
    assert!(clients.due_overrides(santos, other).await.unwrap().is_empty());
    let foreign = clients
        .set_due_override(reyes, nena_id, BirForm::F2551Q, q3, None)
        .await;
    assert!(matches!(foreign, Err(ClientError::NotFound)), "{foreign:?}");
    assert!(matches!(
        clients.due_overrides(reyes, nena_id).await,
        Err(ClientError::NotFound)
    ));

    clients
        .set_due_override(santos, nena_id, BirForm::F2551Q, q3, None)
        .await
        .unwrap();
    assert!(clients.due_overrides(santos, nena_id).await.unwrap().is_empty());
}

#[sqlx::test]
async fn d1_a_clients_schedule_can_be_changed_and_paused(pool: PgPool) {
    use kompleto::kompleto::clients::ScheduleSettings;
    use kompleto::kompleto::schedule::ReminderDays;

    let clients = Clients::new(pool.clone(), clock());
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let id = clients.create(santos, &nena()).await.unwrap();
    let settings = ScheduleSettings {
        cut_off_offset_days: Some(8),
        reminder_days: ReminderDays::new(&[10, 2]).unwrap(),
        paused: true,
    };

    clients.set_schedule(santos, id, &settings).await.unwrap();

    let client = clients.get(santos, id).await.unwrap();
    assert_eq!(client.cut_off_offset_days, Some(8));
    assert_eq!(client.reminder_days.days(), [10, 2]);
    assert!(client.paused);
    assert!(matches!(
        clients.set_schedule(reyes, id, &settings).await,
        Err(ClientError::NotFound)
    ));
}

#[sqlx::test]
async fn b4_a_clients_checklist_edits_are_kept_for_that_client(pool: PgPool) {
    use kompleto::kompleto::checklist::{ExtraItem, Recurs};

    let clients = Clients::new(pool.clone(), clock());
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let id = clients.create(santos, &nena()).await.unwrap();
    let other = clients.create(santos, &numbered(1)).await.unwrap();
    let lazada = ExtraItem {
        label: "Lazada payout reports".to_owned(),
        recurs: Recurs::EveryPeriod,
    };

    clients
        .remove_default_item(santos, id, "bank-statements")
        .await
        .unwrap();
    let extra = clients.add_extra_item(santos, id, &lazada).await.unwrap();

    let edits = clients.checklist_edits(santos, id).await.unwrap();
    assert!(edits.removed.contains("bank-statements"));
    assert_eq!(edits.extra, std::slice::from_ref(&lazada));
    assert_eq!(
        clients.checklist_edits(santos, other).await.unwrap(),
        Default::default()
    );
    assert!(matches!(
        clients.add_extra_item(reyes, id, &lazada).await,
        Err(ClientError::NotFound)
    ));
    assert!(matches!(
        clients.delete_extra_item(reyes, id, extra).await,
        Err(ClientError::NotFound)
    ));
    assert!(matches!(
        clients.remove_default_item(santos, id, "no-such-item").await,
        Err(ClientError::Invalid(_))
    ));

    clients
        .restore_default_item(santos, id, "bank-statements")
        .await
        .unwrap();
    clients.delete_extra_item(santos, id, extra).await.unwrap();
    assert_eq!(clients.checklist_edits(santos, id).await.unwrap(), Default::default());
}
