//! Payroll chasing through the API (P1, P2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::reminders::Scheduler;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test]
async fn p1_payroll_chasing_is_turned_on_and_shown_on_the_client_page(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(pool, app.clock.clone())
        .create(firm, &nena())
        .await
        .unwrap();
    let uri = format!("/api/clients/{client}/payroll");
    let page = format!("/api/clients/{client}");

    let off = app.api(Method::GET, &page, Some(&cookie), None).await.json();
    let set = app
        .api(
            Method::PUT,
            &uri,
            Some(&cookie),
            Some(json!({ "pay_cycle": "twice-monthly", "philhealth_due_day": 15, "pagibig_due_day": 31 })),
        )
        .await;
    let on = app.api(Method::GET, &page, Some(&cookie), None).await.json();

    assert_eq!(
        off["payroll"],
        json!({ "pay_cycle": null, "philhealth_due_day": 20, "pagibig_due_day": 31 })
    );
    assert_eq!(set.status, StatusCode::NO_CONTENT, "{}", set.body);
    assert_eq!(
        on["payroll"],
        json!({ "pay_cycle": "twice-monthly", "philhealth_due_day": 15, "pagibig_due_day": 31 })
    );
}

#[sqlx::test]
async fn p2_an_unknown_pay_cycle_or_due_day_is_refused(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(pool, app.clock.clone())
        .create(firm, &nena())
        .await
        .unwrap();
    let uri = format!("/api/clients/{client}/payroll");

    for change in [
        json!({ "pay_cycle": "weekly", "philhealth_due_day": 20, "pagibig_due_day": 31 }),
        json!({ "pay_cycle": "monthly", "philhealth_due_day": 0, "pagibig_due_day": 31 }),
    ] {
        let refused = app.api(Method::PUT, &uri, Some(&cookie), Some(change)).await;
        assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY, "{}", refused.body);
    }
}

/// The status of each of the Client's scheduled Drafts, by Period.
async fn draft_statuses(pool: &PgPool) -> Vec<(String, String)> {
    sqlx::query_as(
        "select s.period, m.status from scheduled_drafts s join outbox_messages m on m.id = s.message_id
         order by s.period",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

#[sqlx::test]
async fn p1_p2_waiting_drafts_for_paydays_or_receipts_no_longer_chased_are_cancelled(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), app.clock.clone())
        .create(firm, &nena())
        .await
        .unwrap();
    let uri = format!("/api/clients/{client}/payroll");
    let set = |cycle: Option<&str>| json!({ "pay_cycle": cycle, "philhealth_due_day": 20, "pagibig_due_day": 31 });
    app.api(Method::PUT, &uri, Some(&cookie), Some(set(Some("twice-monthly"))))
        .await;
    // 7 Nov 2026, 07:30 Manila: the 15 Nov payday's first request, and a
    // reminder for September's SSS and Pag-IBIG receipts.
    app.clock
        .advance(time::Duration::days(16) + time::Duration::minutes(870));
    Scheduler::new(
        pool.clone(),
        app.clock.clone(),
        common::upload_links(&pool, app.clock.clone()),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();
    let waiting = |period: &str| (period.to_owned(), "draft".to_owned());
    let cancelled = |period: &str| (period.to_owned(), "cancelled".to_owned());
    assert!(draft_statuses(&pool).await.contains(&waiting("payroll-2026-11-15")));

    app.api(Method::PUT, &uri, Some(&cookie), Some(set(Some("monthly"))))
        .await;
    let monthly = draft_statuses(&pool).await;
    app.api(Method::PUT, &uri, Some(&cookie), Some(set(None))).await;
    let off = draft_statuses(&pool).await;

    assert!(monthly.contains(&cancelled("payroll-2026-11-15")), "{monthly:?}");
    assert!(monthly.contains(&waiting("remit-2026-09-sss-pagibig")), "{monthly:?}");
    assert!(off.contains(&cancelled("remit-2026-09-sss-pagibig")), "{off:?}");
}
