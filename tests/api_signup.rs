//! The public sign-up endpoint (M10, L1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test]
async fn l1_a_request_gets_the_same_answer_for_any_address_and_sends_nothing(pool: PgPool) {
    let app = TestApp::new(pool);
    app.logged_in_firm("Santos Bookkeeping").await;
    let before: i64 = sqlx::query_scalar("select count(*) from outbox_messages")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    let request = |email: &str| {
        let body = json!({ "firm_name": "Cruz Bookkeeping", "email": email, "accepted_terms": true });
        app.api(Method::POST, "/api/signup", None, Some(body))
    };

    let new = request("mia@cruz.test").await;
    let existing = request("santos.bookkeeping@firm.test").await;

    assert_eq!(new.status, StatusCode::ACCEPTED, "{}", new.body);
    assert_eq!(
        (existing.status, existing.body.as_str()),
        (new.status, new.body.as_str())
    );
    let after: i64 = sqlx::query_scalar("select count(*) from outbox_messages")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(after, before, "nothing is emailed until the operator approves");
}

#[sqlx::test]
async fn l1_a_request_without_the_terms_is_refused_with_every_reason(pool: PgPool) {
    let app = TestApp::new(pool);

    let refused = app
        .api(
            Method::POST,
            "/api/signup",
            None,
            Some(json!({ "firm_name": "", "email": "nope", "accepted_terms": false })),
        )
        .await;

    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(refused.json()["problems"].as_array().unwrap().len(), 3);
}
