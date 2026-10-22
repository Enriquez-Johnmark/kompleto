//! M10: requesting an account, which opens only once the operator approves
//! it (L1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::clock;
use kompleto::kompleto::firms::Firms;
use kompleto::kompleto::members::{Members, Role};
use kompleto::kompleto::signups::{SignupError, Signups};
use sqlx::PgPool;

#[sqlx::test]
async fn l1_a_request_creates_nothing_until_the_operator_approves_it(pool: PgPool) {
    let signups = Signups::new(pool.clone(), clock());

    signups
        .request(" Cruz Bookkeeping ", " Mia@Cruz.test ", true)
        .await
        .unwrap();
    let firms_before: i64 = sqlx::query_scalar("select count(*) from firms")
        .fetch_one(&pool)
        .await
        .unwrap();
    let pending = signups.pending().await.unwrap();
    let firm = signups.approve(pending[0].id).await.unwrap();

    assert_eq!(firms_before, 0);
    assert_eq!(pending.len(), 1);
    assert_eq!(
        (pending[0].firm_name.as_str(), pending[0].email.as_str()),
        ("Cruz Bookkeeping", "Mia@Cruz.test")
    );
    assert_eq!(
        Firms::new(pool.clone(), clock()).get(firm).await.unwrap().name,
        "Cruz Bookkeeping"
    );
    let members = Members::new(pool.clone(), clock()).list(firm).await.unwrap();
    assert_eq!(
        (members[0].email.as_str(), members[0].role),
        ("Mia@Cruz.test", Role::Owner)
    );
    assert!(
        signups.pending().await.unwrap().is_empty(),
        "approved requests are deleted"
    );
}

#[sqlx::test]
async fn l1_a_request_names_a_firm_and_an_email_and_accepts_the_terms(pool: PgPool) {
    let signups = Signups::new(pool.clone(), clock());

    let refused = signups.request("", "not an email", false).await;

    let Err(SignupError::Invalid(problems)) = refused else {
        panic!("{refused:?}");
    };
    assert_eq!(problems.len(), 3, "{problems:?}");
    assert!(signups.pending().await.unwrap().is_empty());
}

#[sqlx::test]
async fn l1_the_answer_never_reveals_who_has_an_account(pool: PgPool) {
    Firms::new(pool.clone(), clock())
        .create("Santos Bookkeeping", "bea@santos.test")
        .await
        .unwrap();
    let signups = Signups::new(pool.clone(), clock());

    let existing = signups.request("Santos Again", "BEA@santos.test", true).await;
    let first = signups.request("Cruz Bookkeeping", "mia@cruz.test", true).await;
    let again = signups.request("Cruz Bookkeeping", "mia@cruz.test", true).await;

    assert!(existing.is_ok() && first.is_ok() && again.is_ok());
    let pending = signups.pending().await.unwrap();
    assert_eq!(
        pending.len(),
        1,
        "no request for an existing account, and one per address"
    );
    assert_eq!(pending[0].email, "mia@cruz.test");
}

#[sqlx::test]
async fn l1_a_declined_request_is_deleted(pool: PgPool) {
    let signups = Signups::new(pool.clone(), clock());
    signups
        .request("Cruz Bookkeeping", "mia@cruz.test", true)
        .await
        .unwrap();
    let id = signups.pending().await.unwrap()[0].id;

    signups.decline(id).await.unwrap();

    assert!(signups.pending().await.unwrap().is_empty());
    assert!(matches!(signups.decline(id).await, Err(SignupError::NotFound)));
}
