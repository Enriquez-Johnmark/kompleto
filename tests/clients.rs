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
