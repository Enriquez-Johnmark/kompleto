//! The built React app is served for every page address; its files from
//! `/assets/` are cached for good, since their names change with content.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode, header};
use common::web::TestApp;
use sqlx::PgPool;

fn built_app() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "<!doctype html><div id=root></div>").unwrap();
    std::fs::create_dir(dir.path().join("assets")).unwrap();
    std::fs::write(dir.path().join("assets/index-abc123.js"), "console.log(1)").unwrap();
    std::fs::write(dir.path().join("favicon.svg"), "<svg/>").unwrap();
    dir
}

#[sqlx::test]
async fn an_app_address_gets_the_app_page_never_cached(pool: PgPool) {
    let dir = built_app();
    let app = TestApp::with_web_dir(pool, dir.path());

    let response = app.request(Method::GET, "/welcome/tour", None, None).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.contains("<div id=root>"), "{}", response.body);
    assert_eq!(response.headers[header::CACHE_CONTROL], "no-cache");
    assert!(response.headers.contains_key(header::CONTENT_SECURITY_POLICY));
}

#[sqlx::test]
async fn the_apps_files_are_cached_for_good(pool: PgPool) {
    let dir = built_app();
    let app = TestApp::with_web_dir(pool, dir.path());

    let script = app.request(Method::GET, "/assets/index-abc123.js", None, None).await;
    let missing = app.request(Method::GET, "/assets/gone.js", None, None).await;
    let icon = app.request(Method::GET, "/favicon.svg", None, None).await;

    assert_eq!(script.status, StatusCode::OK);
    assert_eq!(script.body, "console.log(1)");
    assert_eq!(
        script.headers[header::CACHE_CONTROL],
        "public, max-age=31536000, immutable"
    );
    assert_eq!(missing.status, StatusCode::NOT_FOUND);
    assert_eq!(icon.body, "<svg/>");
}

#[sqlx::test]
async fn files_outside_the_app_folder_are_never_served(pool: PgPool) {
    let dir = built_app();
    let app = TestApp::with_web_dir(pool, dir.path());

    let response = app.request(Method::GET, "/assets/../../etc/passwd", None, None).await;

    assert!(!response.body.contains("root:"), "{}", response.body);
}

#[sqlx::test]
async fn without_a_built_app_the_server_says_so(pool: PgPool) {
    let app = TestApp::with_web_dir(pool, std::path::Path::new("/nonexistent/kompleto-web"));

    let response = app.request(Method::GET, "/welcome/tour", None, None).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert!(response.body.contains("pnpm --dir web build"), "{}", response.body);
}
