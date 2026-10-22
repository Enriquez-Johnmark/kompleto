//! Staff in the API (M9): what they may do, and who the log names (K2, K3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::reminders::Scheduler;
use serde_json::json;
use sqlx::PgPool;
use time::Duration;
use uuid::Uuid;

#[sqlx::test]
async fn k2_staff_do_the_daily_work_but_cant_export_or_close_the_account(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, _) = app.logged_in_firm("Santos Bookkeeping").await;
    let joy = app.logged_in_staff(firm, "joy@santos.test").await;

    let me = app.api(Method::GET, "/api/me", Some(&joy), None).await;
    let clients = app.api(Method::GET, "/api/clients", Some(&joy), None).await;
    let export = app.api(Method::GET, "/api/settings/export", Some(&joy), None).await;
    let close = app
        .api(
            Method::POST,
            "/api/settings/close",
            Some(&joy),
            Some(json!({ "confirm": "Santos Bookkeeping" })),
        )
        .await;

    assert_eq!(me.json()["email"], "joy@santos.test");
    assert_eq!(me.json()["role"], "staff");
    assert_eq!(me.json()["firm_name"], "Santos Bookkeeping");
    assert_eq!(clients.status, StatusCode::OK);
    assert_eq!(export.status, StatusCode::FORBIDDEN);
    assert_eq!(close.status, StatusCode::FORBIDDEN);
    assert_eq!(close.json()["message"], "Only the firm's owner can do this.");
    let closed: bool = sqlx::query_scalar("select closed_at is not null from firms where id = $1")
        .bind(firm.as_uuid())
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert!(!closed);
}

#[sqlx::test]
async fn k3_an_approval_names_the_staff_member_who_gave_it(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, _) = app.logged_in_firm("Santos Bookkeeping").await;
    let joy = app.logged_in_staff(firm, "joy@santos.test").await;
    let lito = NewClient {
        business_name: "Lito Hardware".to_owned(),
        email: "lito@example.test".to_owned(),
        forms: vec![BirForm::F1601C],
        ..common::nena()
    };
    Clients::new(app.pool.clone(), app.clock.clone())
        .create(firm, &lito)
        .await
        .unwrap();
    app.clock.advance(Duration::days(11));
    Scheduler::new(
        app.pool.clone(),
        app.clock.clone(),
        common::upload_links(&app.pool, app.clock.clone()),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();
    let draft: Uuid = sqlx::query_scalar("select message_id from scheduled_drafts")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    let shown = app
        .api(Method::GET, &format!("/api/drafts/{draft}"), Some(&joy), None)
        .await;

    let approved = app
        .api(
            Method::POST,
            &format!("/api/drafts/{draft}/approve"),
            Some(&joy),
            Some(json!({ "hash": shown.json()["hash"] })),
        )
        .await;

    assert_eq!(approved.status, StatusCode::NO_CONTENT, "{}", approved.body);
    let approver: String = sqlx::query_scalar("select approved_by from outbox_messages where id = $1")
        .bind(draft)
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(approver, "joy@santos.test");
}

#[sqlx::test]
async fn k1_the_owner_invites_staff_who_get_their_own_login_link(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, owner) = app.logged_in_firm("Santos Bookkeeping").await;

    let invited = app
        .api(
            Method::POST,
            "/api/staff",
            Some(&owner),
            Some(json!({ "email": "joy@santos.test" })),
        )
        .await;
    let taken = app
        .api(
            Method::POST,
            "/api/staff",
            Some(&owner),
            Some(json!({ "email": "JOY@santos.test" })),
        )
        .await;

    assert_eq!(invited.status, StatusCode::CREATED, "{}", invited.body);
    assert_eq!(taken.status, StatusCode::CONFLICT);
    let (subject, body): (String, String) =
        sqlx::query_as("select subject, body_text from outbox_messages where to_address = 'joy@santos.test'")
            .fetch_one(&app.pool)
            .await
            .unwrap();
    assert_eq!(subject, "You've been added to Santos Bookkeeping on Kompleto");
    assert!(body.contains("santos.bookkeeping@firm.test added you"), "{body}");
    let token = app.login_token_for("joy@santos.test").await;
    let logged_in = app.api(Method::POST, &format!("/api/login/{token}"), None, None).await;
    assert!(
        logged_in.session_cookie().is_some(),
        "the invitation's link logs them in"
    );
    let listed = app.api(Method::GET, "/api/staff", Some(&owner), None).await.json();
    assert_eq!(listed["max_staff"], 5);
    assert_eq!(listed["members"][1]["email"], "joy@santos.test");
    assert_eq!(listed["members"][1]["role"], "staff");
}

#[sqlx::test]
async fn k2_only_the_firms_owner_manages_its_staff(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, owner) = app.logged_in_firm("Santos Bookkeeping").await;
    let (_, other_owner) = app.logged_in_firm("Reyes CPA").await;
    let joy = app.logged_in_staff(santos, "joy@santos.test").await;
    let listed = app.api(Method::GET, "/api/staff", Some(&owner), None).await.json();
    let joy_id = listed["members"][1]["id"].as_str().unwrap().to_owned();
    let remove = |cookie: &str| {
        let cookie = cookie.to_owned();
        let path = format!("/api/staff/{joy_id}");
        let app = &app;
        async move { app.api(Method::DELETE, &path, Some(&cookie), None).await }
    };

    let by_staff = app
        .api(
            Method::POST,
            "/api/staff",
            Some(&joy),
            Some(json!({ "email": "kim@santos.test" })),
        )
        .await;
    let listed_by_staff = app.api(Method::GET, "/api/staff", Some(&joy), None).await;
    let by_other_firm = remove(&other_owner).await;
    let by_owner = remove(&owner).await;

    assert_eq!(by_staff.status, StatusCode::FORBIDDEN);
    assert_eq!(listed_by_staff.status, StatusCode::FORBIDDEN);
    assert_eq!(by_other_firm.status, StatusCode::NOT_FOUND);
    assert_eq!(by_owner.status, StatusCode::NO_CONTENT);
    let after = app.api(Method::GET, "/api/me", Some(&joy), None).await;
    assert_eq!(after.status, StatusCode::UNAUTHORIZED, "removed: logged out at once");
}
