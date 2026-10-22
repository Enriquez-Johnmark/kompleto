//! Payroll chasing through the API (P1, P2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::clients::Clients;
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
