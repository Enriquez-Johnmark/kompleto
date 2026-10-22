//! M10: requesting an account, which opens only once the operator approves
//! it (L1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::clock;
use kompleto::kompleto::firms::Firms;
use kompleto::kompleto::members::{Members, Role};
use kompleto::kompleto::signups::{MAX_PENDING, MAX_PER_HOUR, SignupError, Signups};
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

#[sqlx::test]
async fn l1_a_full_queue_answers_members_and_strangers_alike(pool: PgPool) {
    let clock = clock();
    Firms::new(pool.clone(), clock.clone())
        .create("Santos Bookkeeping", "bea@santos.test")
        .await
        .unwrap();
    let signups = Signups::new(pool.clone(), clock.clone());
    for n in 0..MAX_PENDING {
        // Spread over the hours, under the hourly limit.
        if n % MAX_PER_HOUR == 0 {
            clock.advance(time::Duration::hours(1));
        }
        signups
            .request("Junk", &format!("junk{n}@spam.test"), true)
            .await
            .unwrap();
    }

    let member = signups.request("Santos Again", "bea@santos.test", true).await;
    let stranger = signups.request("Cruz Bookkeeping", "mia@cruz.test", true).await;

    assert!(matches!(member, Err(SignupError::TooMany)), "{member:?}");
    assert!(matches!(stranger, Err(SignupError::TooMany)), "{stranger:?}");
}

#[sqlx::test]
async fn l1_requests_are_limited_per_hour_and_expire_after_30_days(pool: PgPool) {
    let clock = clock();
    let signups = Signups::new(pool.clone(), clock.clone());
    for n in 0..MAX_PER_HOUR {
        signups
            .request("Junk", &format!("junk{n}@spam.test"), true)
            .await
            .unwrap();
    }

    let over = signups.request("Cruz Bookkeeping", "mia@cruz.test", true).await;
    clock.advance(time::Duration::days(31));
    let later = signups.request("Cruz Bookkeeping", "mia@cruz.test", true).await;

    assert!(matches!(over, Err(SignupError::TooMany)), "{over:?}");
    assert!(later.is_ok());
    let pending = signups.pending().await.unwrap();
    assert_eq!(pending.len(), 1, "the old ones expired");
}

#[sqlx::test]
async fn l1_the_owner_of_a_closed_firm_can_ask_again(pool: PgPool) {
    let firms = Firms::new(pool.clone(), clock());
    let santos = firms.create("Santos Bookkeeping", "bea@santos.test").await.unwrap();
    sqlx::query("update firms set closed_at = now() where id = $1")
        .bind(santos.as_uuid())
        .execute(&pool)
        .await
        .unwrap();
    let signups = Signups::new(pool.clone(), clock());

    signups
        .request("Santos Bookkeeping", "bea@santos.test", true)
        .await
        .unwrap();

    assert_eq!(signups.pending().await.unwrap().len(), 1);
}
