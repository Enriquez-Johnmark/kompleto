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
