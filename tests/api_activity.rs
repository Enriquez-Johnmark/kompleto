//! The Firm's activity log through the API (J2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use kompleto::engine::audit::{self, AuditEvent};
use kompleto::engine::clock::Clock;
use kompleto::kompleto::firms::FirmId;
use sqlx::PgPool;
use time::Duration;

async fn logged(app: &TestApp, firm: FirmId, days_ago: i64, actor: &str, action: &str) {
    audit::record(
        &app.pool,
        &AuditEvent {
            tenant: firm,
            at: app.clock.now() - Duration::days(days_ago),
            actor,
            action,
            subject_type: "outbox_message",
            subject_id: uuid::Uuid::now_v7(),
            details: serde_json::json!({}),
        },
    )
    .await
    .unwrap();
}

#[sqlx::test]
async fn j2_the_firm_sees_its_last_90_days_of_activity_newest_first(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let (reyes, _) = app.logged_in_firm("Reyes CPA").await;
    logged(&app, santos, 2, "santos.bookkeeping@firm.test", "upload.received").await;
    logged(&app, santos, 1, audit::SYSTEM, "file.deleted").await;
    logged(&app, santos, 91, "santos.bookkeeping@firm.test", "outbox.edited").await;
    logged(&app, reyes, 1, "reyes.cpa@firm.test", "outbox.cancelled").await;

    let page = app.api(Method::GET, "/api/activity", Some(&cookie), None).await;

    assert_eq!(page.status, StatusCode::OK, "{}", page.body);
    let page = page.json();
    assert_eq!(page["days"], 90);
    let entries: Vec<(&str, &str)> = page["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| (entry["who"].as_str().unwrap(), entry["what"].as_str().unwrap()))
        // Leave out the login itself and its email, drafted and approved
        // by Kompleto.
        .filter(|(_, what)| !["Logged in", "Drafted a message", "Approved a message"].contains(what))
        .collect();
    assert_eq!(
        entries,
        [
            ("Kompleto", "Deleted an uploaded file"),
            ("santos.bookkeeping@firm.test", "Marked an upload received"),
        ],
        "newest first; nothing older than 90 days or from another Firm"
    );
}

#[sqlx::test]
async fn j2_a_login_is_logged(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let page = app.api(Method::GET, "/api/activity", Some(&cookie), None).await;

    let actors: Vec<String> =
        sqlx::query_scalar("select actor from audit_events where tenant_id = $1 and action = 'firm.logged_in'")
            .bind(santos.as_uuid())
            .fetch_all(&app.pool)
            .await
            .unwrap();
    assert_eq!(actors, ["santos.bookkeeping@firm.test"]);
    assert!(
        page.json()["entries"].to_string().contains("Logged in"),
        "{}",
        page.body
    );
}
