//! Seam 3 (M6): security headers on every page, and no inline scripts for
//! the Content-Security-Policy to allow.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use sqlx::PgPool;

#[sqlx::test]
async fn every_page_carries_the_security_headers(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    for page in [
        app.request(Method::GET, "/login", None, None).await,
        app.api(Method::GET, "/api/drafts", Some(&cookie), None).await,
    ] {
        let csp = page.headers["content-security-policy"].to_str().unwrap();
        for directive in [
            "default-src 'none'",
            "script-src 'self'",
            "frame-ancestors 'none'",
            "form-action 'self'",
        ] {
            assert!(csp.contains(directive), "{directive} missing from {csp}");
        }
        assert!(!csp.contains("script-src 'self' 'unsafe-inline'"), "{csp}");
        assert_eq!(page.headers["x-content-type-options"], "nosniff");
        assert_eq!(page.headers["x-frame-options"], "DENY");
        assert_eq!(page.headers["referrer-policy"], "same-origin");
        assert!(
            !page.headers.contains_key("strict-transport-security"),
            "plain http in tests"
        );
        assert!(!page.body.contains("onclick="), "inline handler on {}", page.body);
    }
    let script = app.request(Method::GET, "/static/kompleto.js", None, None).await;
    assert_eq!(script.status, StatusCode::OK);
    assert_eq!(script.headers["content-type"], "text/javascript; charset=utf-8");
    assert!(script.body.contains("FormData"), "the upload page's script");
}

#[sqlx::test]
async fn https_sites_tell_browsers_to_stay_on_https(pool: PgPool) {
    let app = TestApp::at(pool, "https://app.getkompleto.test");

    let page = app.request(Method::GET, "/login", None, None).await;

    assert_eq!(page.headers["strict-transport-security"], "max-age=31536000");
}
