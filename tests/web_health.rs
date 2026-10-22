//! Seam 3 (M6): the health check an uptime monitor polls.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::clamd::{Behaviour, FakeClamd, nobody_listening};
use common::web::TestApp;
use sqlx::PgPool;

#[sqlx::test]
async fn the_health_check_answers_without_login_and_fails_without_the_database(pool: PgPool) {
    let app = TestApp::new(pool);

    let healthy = app.request(Method::GET, "/health", None, None).await;
    assert_eq!(healthy.status, StatusCode::OK);
    assert_eq!(healthy.body, "ok");

    app.pool.close().await;
    let sick = app.request(Method::GET, "/health", None, None).await;
    assert_eq!(sick.status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(!sick.body.contains("pool"), "no details for strangers: {}", sick.body);
}

#[sqlx::test]
async fn the_health_check_fails_while_the_virus_scanner_is_down(pool: PgPool) {
    let clamd = FakeClamd::start(Behaviour::Scans).await;
    let scanning = TestApp::scanning(pool.clone(), &clamd.addr());
    let unscanned = TestApp::scanning(pool, &nobody_listening().await);

    let healthy = scanning.request(Method::GET, "/health", None, None).await;
    let sick = unscanned.request(Method::GET, "/health", None, None).await;

    assert_eq!(healthy.status, StatusCode::OK);
    assert_eq!(sick.status, StatusCode::SERVICE_UNAVAILABLE);
}
