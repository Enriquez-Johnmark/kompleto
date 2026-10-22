//! Seam 3 (M6): the Firm's activity log page in-process (J2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
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

    let page = app.get("/activity", &cookie).await;

    assert_eq!(page.status, StatusCode::OK);
    let at = |text: &str| {
        page.body
            .find(text)
            .unwrap_or_else(|| panic!("no {text:?} in {}", page.body))
    };
    assert!(
        at("Deleted an uploaded file") < at("Marked an upload received"),
        "newest first"
    );
    assert!(page.body.contains("santos.bookkeeping@firm.test"));
    assert!(page.body.contains("Kompleto"), "the system named plainly");
    assert!(!page.body.contains("Edited a message"), "older than 90 days");
    assert!(!page.body.contains("Cancelled"), "another Firm's");
    assert!(page.body.contains(r#"href="/activity""#), "in the menu");
}

#[sqlx::test]
async fn j2_a_login_is_logged(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let page = app.get("/activity", &cookie).await;

    let actors: Vec<String> =
        sqlx::query_scalar("select actor from audit_events where tenant_id = $1 and action = 'firm.logged_in'")
            .bind(santos.as_uuid())
            .fetch_all(&app.pool)
            .await
            .unwrap();
    assert_eq!(actors, ["santos.bookkeeping@firm.test"]);
    assert!(page.body.contains("Logged in"), "{}", page.body);
}
