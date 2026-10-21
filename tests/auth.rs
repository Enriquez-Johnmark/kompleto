//! Seam 2/4: login links and sessions (A1), with the test clock.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use common::clock;
use kompleto::engine::clock::FixedClock;
use kompleto::kompleto::auth::{Auth, LoginOutcome};
use kompleto::kompleto::firms::{FirmId, Firms};
use sqlx::PgPool;
use time::Duration;

const BASE_URL: &str = "http://kompleto.test";

async fn setup(pool: &PgPool) -> (Auth, Arc<FixedClock>, FirmId) {
    let clock = clock();
    let firm = Firms::new(pool.clone(), clock.clone())
        .create("Santos Bookkeeping", "ana@santos.test")
        .await
        .unwrap();
    (Auth::new(pool.clone(), clock.clone(), BASE_URL), clock, firm)
}

/// Every login email in the outbox: (recipient, status, body).
async fn login_emails(pool: &PgPool) -> Vec<(String, String, String)> {
    sqlx::query_as("select to_address, status, body_text from outbox_messages order by created_at")
        .fetch_all(pool)
        .await
        .unwrap()
}

/// The token in the newest login email.
async fn latest_token(pool: &PgPool) -> String {
    let (_, _, body) = login_emails(pool).await.pop().unwrap();
    let start = body.find(&format!("{BASE_URL}/login/")).unwrap() + BASE_URL.len() + "/login/".len();
    body[start..].chars().take_while(char::is_ascii_alphanumeric).collect()
}

#[sqlx::test]
async fn a1_a_login_link_is_emailed_approved_and_works_once(pool: PgPool) {
    let (auth, _, firm) = setup(&pool).await;

    assert_eq!(
        auth.request_link(" ANA@santos.test ").await.unwrap(),
        LoginOutcome::Sent
    );
    let emails = login_emails(&pool).await;
    assert_eq!(emails.len(), 1);
    assert_eq!(
        (emails[0].0.as_str(), emails[0].1.as_str()),
        ("ana@santos.test", "approved")
    );
    let token = latest_token(&pool).await;

    let session = auth.log_in(&token).await.unwrap().unwrap();
    assert_eq!(auth.firm_for_session(&session).await.unwrap(), Some(firm));
    assert_eq!(auth.log_in(&token).await.unwrap(), None, "a link works only once");
}

#[sqlx::test]
async fn a1_a_login_link_expires_after_15_minutes(pool: PgPool) {
    let (auth, clock, _) = setup(&pool).await;
    auth.request_link("ana@santos.test").await.unwrap();
    let token = latest_token(&pool).await;

    clock.advance(Duration::minutes(15));

    assert_eq!(auth.log_in(&token).await.unwrap(), None);
}

#[sqlx::test]
async fn a1_an_address_with_no_account_gets_no_email(pool: PgPool) {
    let (auth, _, _) = setup(&pool).await;

    assert_eq!(
        auth.request_link("stranger@example.test").await.unwrap(),
        LoginOutcome::NotSent
    );

    assert!(login_emails(&pool).await.is_empty());
}

#[sqlx::test]
async fn a1_at_most_five_login_emails_per_firm_per_hour(pool: PgPool) {
    let (auth, clock, _) = setup(&pool).await;
    for _ in 0..5 {
        assert_eq!(auth.request_link("ana@santos.test").await.unwrap(), LoginOutcome::Sent);
    }

    assert_eq!(
        auth.request_link("ana@santos.test").await.unwrap(),
        LoginOutcome::NotSent
    );
    clock.advance(Duration::hours(1));
    assert_eq!(auth.request_link("ana@santos.test").await.unwrap(), LoginOutcome::Sent);
    assert_eq!(login_emails(&pool).await.len(), 6);
}

#[sqlx::test]
async fn a1_a_session_lasts_30_days_or_until_logout(pool: PgPool) {
    let (auth, clock, firm) = setup(&pool).await;
    auth.request_link("ana@santos.test").await.unwrap();
    let first = auth.log_in(&latest_token(&pool).await).await.unwrap().unwrap();
    auth.request_link("ana@santos.test").await.unwrap();
    let second = auth.log_in(&latest_token(&pool).await).await.unwrap().unwrap();

    auth.log_out(&second).await.unwrap();
    assert_eq!(auth.firm_for_session(&second).await.unwrap(), None);
    clock.advance(Duration::days(30) - Duration::seconds(1));
    assert_eq!(auth.firm_for_session(&first).await.unwrap(), Some(firm));
    clock.advance(Duration::seconds(1));
    assert_eq!(auth.firm_for_session(&first).await.unwrap(), None);
    assert_eq!(auth.firm_for_session("not-a-session").await.unwrap(), None);
}

#[sqlx::test]
async fn a1_parallel_requests_still_get_at_most_five_login_emails(pool: PgPool) {
    let (auth, _, _) = setup(&pool).await;

    let requests: Vec<_> = (0..12)
        .map(|_| {
            let auth = auth.clone();
            tokio::spawn(async move { auth.request_link("ana@santos.test").await.unwrap() })
        })
        .collect();
    let mut sent = 0;
    for request in requests {
        sent += usize::from(request.await.unwrap() == LoginOutcome::Sent);
    }

    assert_eq!(sent, 5);
    assert_eq!(login_emails(&pool).await.len(), 5);
}
