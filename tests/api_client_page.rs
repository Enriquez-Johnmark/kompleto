//! One Client's page through the API (B4, and the D1, B3 and checklist
//! changes made from it).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use serde_json::{Value, json};
use sqlx::PgPool;

/// A logged-in firm with Nena's store (2551Q and 1701Q); its page's path.
async fn nena(app: &TestApp) -> (String, String) {
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let input = json!({
        "business_name": "Nena's Store", "contact_first_name": "Nena",
        "email": "nena@example.test", "language": "taglish", "forms": ["2551Q", "1701Q"],
    });
    let id = app
        .api(Method::POST, "/api/clients", Some(&cookie), Some(input))
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    (cookie, format!("/api/clients/{id}"))
}

async fn page(app: &TestApp, cookie: &str, path: &str) -> Value {
    let page = app.api(Method::GET, path, Some(cookie), None).await;
    assert_eq!(page.status, StatusCode::OK, "{}", page.body);
    page.json()
}

#[sqlx::test]
async fn b4_the_client_page_lists_each_periods_owed_documents_and_cut_off(pool: PgPool) {
    let app = TestApp::new(pool);
    let (cookie, path) = nena(&app).await;

    let page = page(&app, &cookie, &path).await;

    assert_eq!(page["business_name"], "Nena's Store");
    assert_eq!(page["language"], "taglish");
    // Today is 21 Oct 2026: Q3's cut-off is today (2551Q due 26 Oct, minus 5 days).
    let q3 = &page["periods"][0];
    assert_eq!(q3["label"], "Q3 2026");
    assert_eq!(q3["cut_off"], "2026-10-21");
    let filings: Vec<(&str, &str)> = q3["filings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["form"].as_str().unwrap(), f["due"].as_str().unwrap()))
        .collect();
    assert_eq!(filings, [("2551Q", "2026-10-26"), ("1701Q", "2026-11-16")]);
    let owed = q3["owed"].to_string();
    assert!(
        owed.contains("Sales invoices issued, including cancelled ones"),
        "{owed}"
    );
    assert!(owed.contains("2307s received from customers"), "{owed}");
    assert_eq!(page["periods"][1]["label"], "Q4 2026");
    assert_eq!(page["periods"][1]["cut_off"], "2027-01-20");
    assert_eq!(
        page["schedule"],
        json!({"cut_off_offset_days": null, "firm_offset_days": 5, "reminder_days": [7, 3, 1]})
    );
}
