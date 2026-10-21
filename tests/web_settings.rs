//! Seam 4: firm settings and the calendar page.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::web::TestApp;
use sqlx::PgPool;

#[sqlx::test]
async fn the_firms_default_cut_off_offset_applies_to_clients_without_their_own(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = app
        .post(
            "/clients",
            &cookie,
            "business_name=Shop&contact_first_name=Ana&email=a%40b.test&language=english&forms=2551Q",
        )
        .await
        .location()
        .to_owned();

    let saved = app.post("/settings", &cookie, "cut_off_offset_days=10").await;

    assert_eq!(saved.status, StatusCode::SEE_OTHER, "{}", saved.body);
    assert!(app.get("/settings", &cookie).await.body.contains(r#"value="10""#));
    // Q4 2026: 2551Q due Mon 25 Jan 2027, minus 10 days.
    assert!(
        app.get(&client, &cookie)
            .await
            .body
            .contains("Cut-off: Fri 15 Jan 2027")
    );
    assert_eq!(
        app.post("/settings", &cookie, "cut_off_offset_days=31").await.status,
        StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[sqlx::test]
async fn b3_the_calendar_page_lists_every_due_date_for_the_next_12_months(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let page = app.get("/calendar", &cookie).await;

    assert_eq!(page.status, StatusCode::OK);
    for expected in [
        "Mon 26 Oct 2026",
        "Tue 3 Nov 2026",
        "1601-EQ",
        "Mon 11 Oct 2027",
        "September 2027",
    ] {
        assert!(page.body.contains(expected), "missing {expected:?} in {}", page.body);
    }
}
