//! Login through the API (A1), as the React app does it.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode, header};
use common::web::TestApp;
use kompleto::kompleto::firms::Firms;
use serde_json::json;
use sqlx::PgPool;

async fn santos(app: &TestApp) {
    Firms::new(app.pool.clone(), app.clock.clone())
        .create("Santos Bookkeeping", "ana@santos.test")
        .await
        .unwrap();
}

#[sqlx::test]
async fn a1_asking_for_a_link_never_reveals_who_has_an_account(pool: PgPool) {
    let app = TestApp::new(pool);
    santos(&app).await;

    let known = app
        .api(
            Method::POST,
            "/api/login",
            None,
            Some(json!({"email": "ana@santos.test"})),
        )
        .await;
    let unknown = app
        .api(
            Method::POST,
            "/api/login",
            None,
            Some(json!({"email": "nobody@example.test"})),
        )
        .await;

    assert_eq!(known.status, StatusCode::ACCEPTED, "{}", known.body);
    assert_eq!((known.status, known.body), (unknown.status, unknown.body));
    app.login_token_for("ana@santos.test").await;
}

#[sqlx::test]
async fn a1_a_login_link_logs_in_once(pool: PgPool) {
    let app = TestApp::new(pool);
    santos(&app).await;
    app.api(
        Method::POST,
        "/api/login",
        None,
        Some(json!({"email": "ana@santos.test"})),
    )
    .await;
    let token = app.login_token_for("ana@santos.test").await;

    let logged_in = app.api(Method::POST, &format!("/api/login/{token}"), None, None).await;

    assert_eq!(logged_in.status, StatusCode::OK, "{}", logged_in.body);
    let set_cookie = logged_in.headers.get(header::SET_COOKIE).unwrap().to_str().unwrap();
    assert!(
        set_cookie.contains("HttpOnly") && set_cookie.contains("SameSite=Lax"),
        "{set_cookie}"
    );
    let cookie = logged_in.session_cookie().unwrap();
    let me = app.api(Method::GET, "/api/me", Some(&cookie), None).await;
    assert_eq!(me.status, StatusCode::OK);

    let again = app.api(Method::POST, &format!("/api/login/{token}"), None, None).await;
    assert_eq!(again.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(again.session_cookie(), None);
    assert_eq!(
        again.json()["message"],
        "This login link has expired or was already used. Ask for a new one."
    );
}

#[sqlx::test]
async fn logging_out_ends_the_session(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let response = app.api(Method::POST, "/api/logout", Some(&cookie), None).await;

    assert_eq!(response.status, StatusCode::NO_CONTENT);
    let cleared = response.headers.get(header::SET_COOKIE).unwrap().to_str().unwrap();
    assert!(cleared.starts_with("kompleto_session=;"), "{cleared}");
    let me = app.api(Method::GET, "/api/me", Some(&cookie), None).await;
    assert_eq!(me.status, StatusCode::UNAUTHORIZED);
}
