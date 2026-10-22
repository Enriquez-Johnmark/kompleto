//! The BIR calendar through the API (B3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use sqlx::PgPool;

#[sqlx::test]
async fn b3_the_calendar_lists_every_due_date_for_the_next_12_months(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let calendar = app.api(Method::GET, "/api/calendar", Some(&cookie), None).await;

    assert_eq!(calendar.status, StatusCode::OK, "{}", calendar.body);
    let entries = calendar.json();
    let entries = entries.as_array().unwrap();
    let has = |due: &str, form: &str| entries.iter().any(|e| e["due"] == due && e["form"] == form);
    // 21 Oct 2026 to 21 Oct 2027, earliest first.
    assert_eq!(entries[0]["due"], "2026-10-26");
    assert!(
        has("2026-10-26", "2551Q") && has("2026-11-03", "1601-EQ"),
        "{entries:?}"
    );
    assert!(entries.iter().any(|e| e["period"] == "September 2027"), "{entries:?}");
    assert!(entries.iter().all(|e| e["due"].as_str().unwrap() <= "2027-10-21"));
}
