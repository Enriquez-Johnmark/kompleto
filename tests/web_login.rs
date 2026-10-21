//! Seam 4: login through the web pages (A1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use common::web::TestApp;
use kompleto::kompleto::firms::Firms;
use sqlx::PgPool;

#[sqlx::test]
async fn a1_the_check_your_email_screen_never_reveals_who_has_an_account(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    Firms::new(pool.clone(), app.clock.clone())
        .create("Santos Bookkeeping", "ana@santos.test")
        .await
        .unwrap();

    let known = app
        .request(Method::POST, "/login", None, Some("email=ana%40santos.test"))
        .await;
    let unknown = app
        .request(Method::POST, "/login", None, Some("email=nobody%40example.test"))
        .await;

    assert_eq!(known.status, StatusCode::OK);
    assert!(known.body.contains("Check your email"), "{}", known.body);
    assert_eq!((known.status, known.body), (unknown.status, unknown.body));
}

#[sqlx::test]
async fn a1_opening_a_link_shows_a_button_and_only_the_button_logs_in(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    Firms::new(pool.clone(), app.clock.clone())
        .create("Santos Bookkeeping", "ana@santos.test")
        .await
        .unwrap();
    app.request(Method::POST, "/login", None, Some("email=ana%40santos.test"))
        .await;
    let token = app.latest_login_token().await;

    // A link scanner opening the link twice doesn't use it up.
    for _ in 0..2 {
        let page = app.request(Method::GET, &format!("/login/{token}"), None, None).await;
        assert_eq!(page.status, StatusCode::OK);
        assert!(
            page.body.contains(&format!(r#"action="/login/{token}""#)),
            "{}",
            page.body
        );
        assert_eq!(page.session_cookie(), None);
    }
    let logged_in = app.request(Method::POST, &format!("/login/{token}"), None, None).await;

    assert_eq!(logged_in.status, StatusCode::SEE_OTHER);
    assert_eq!(logged_in.location(), "/clients");
    let set_cookie = logged_in.headers.get(header::SET_COOKIE).unwrap().to_str().unwrap();
    assert!(
        set_cookie.contains("HttpOnly") && set_cookie.contains("SameSite=Lax"),
        "{set_cookie}"
    );
    let cookie = logged_in.session_cookie().unwrap();
    assert_eq!(app.get("/clients", &cookie).await.status, StatusCode::OK);

    let again = app.request(Method::POST, &format!("/login/{token}"), None, None).await;
    assert_eq!(again.session_cookie(), None);
    assert!(again.body.contains("expired or was already used"), "{}", again.body);
}

#[sqlx::test]
async fn pages_send_visitors_without_a_session_to_login(pool: PgPool) {
    let app = TestApp::new(pool);

    let response = app.request(Method::GET, "/clients", None, None).await;

    assert_eq!(response.status, StatusCode::SEE_OTHER);
    assert_eq!(response.location(), "/login");
}

#[sqlx::test]
async fn logging_out_ends_the_session(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    app.post("/logout", &cookie, "").await;

    assert_eq!(app.get("/clients", &cookie).await.status, StatusCode::SEE_OTHER);
}

#[sqlx::test]
async fn form_posts_from_other_sites_are_refused(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let forged = Request::post("/logout")
        .header(header::COOKIE, &cookie)
        .header(header::ORIGIN, "https://evil.example")
        .body(Body::empty())
        .unwrap();
    let no_origin = Request::post("/logout")
        .header(header::COOKIE, &cookie)
        .body(Body::empty())
        .unwrap();

    assert_eq!(app.send(forged).await.status, StatusCode::FORBIDDEN);
    assert_eq!(app.send(no_origin).await.status, StatusCode::FORBIDDEN);
    assert_eq!(app.get("/clients", &cookie).await.status, StatusCode::OK);
}
