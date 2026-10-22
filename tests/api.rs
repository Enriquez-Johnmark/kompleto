//! The JSON API's shared behaviour: errors are JSON, and every Firm
//! endpoint needs a session.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode, header};
use common::web::TestApp;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test]
async fn an_unknown_api_path_is_a_json_not_found(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let response = app.api(Method::GET, "/api/nothing-here", Some(&cookie), None).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json()["message"], "Not found.");
}

#[sqlx::test]
async fn without_a_session_the_api_answers_not_logged_in(pool: PgPool) {
    let app = TestApp::new(pool);

    let response = app.api(Method::GET, "/api/me", None, None).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers[header::CONTENT_TYPE],
        "application/json",
        "{}",
        response.body
    );
    assert_eq!(response.json()["message"], "Log in first.");
}

#[sqlx::test]
async fn me_names_the_logged_in_firm(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let response = app.api(Method::GET, "/api/me", Some(&cookie), None).await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.body);
    assert_eq!(
        response.json(),
        json!({"firm_name": "Santos Bookkeeping", "login_email": "santos.bookkeeping@firm.test"})
    );
}

#[sqlx::test]
async fn an_api_change_from_another_site_is_refused(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let request = axum::http::Request::builder()
        .method(Method::POST)
        .uri("/api/logout")
        .header(header::COOKIE, &cookie)
        .header(header::ORIGIN, "https://evil.example")
        .header(header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from("{}"))
        .unwrap();

    let no_origin = axum::http::Request::builder()
        .method(Method::POST)
        .uri("/api/logout")
        .header(header::COOKIE, &cookie)
        .body(axum::body::Body::empty())
        .unwrap();

    assert_eq!(app.send(request).await.status, StatusCode::FORBIDDEN);
    assert_eq!(app.send(no_origin).await.status, StatusCode::FORBIDDEN);
    let me = app.api(Method::GET, "/api/me", Some(&cookie), None).await;
    assert_eq!(me.status, StatusCode::OK, "still logged in");
}

#[sqlx::test]
async fn a_body_that_cant_be_read_is_refused_in_the_apis_words(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = app
        .api(
            Method::POST,
            "/api/clients",
            Some(&cookie),
            Some(json!({"business_name": "Shop", "contact_first_name": "Ana",
                "email": "a@b.test", "language": "english", "forms": ["2551Q"]})),
        )
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();

    let fraction = app
        .api(
            Method::PUT,
            &format!("/api/clients/{client}/schedule"),
            Some(&cookie),
            Some(json!({"cut_off_offset_days": null, "reminder_days": [7.5], "paused": false})),
        )
        .await;

    assert_eq!(fraction.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        fraction.json()["message"],
        "That wasn't understood; check what you typed and try again."
    );
}

#[sqlx::test]
async fn a_malformed_id_in_the_address_is_not_found(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let response = app
        .api(Method::GET, "/api/clients/not-an-id", Some(&cookie), None)
        .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json()["message"], "Not found.");
}
