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

/// An API call owning its path, so closures can return it.
async fn call(app: &TestApp, method: Method, path: String, cookie: &str, body: Option<Value>) -> common::web::Response {
    app.api(method, &path, Some(cookie), body).await
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

#[sqlx::test]
async fn d1_the_schedule_changes_cut_off_reminders_and_pause(pool: PgPool) {
    let app = TestApp::new(pool);
    let (cookie, path) = nena(&app).await;
    let schedule = |change: Value| call(&app, Method::PUT, format!("{path}/schedule"), &cookie, Some(change));

    let saved = schedule(json!({"cut_off_offset_days": 8, "reminder_days": [10, 2], "paused": true})).await;

    assert_eq!(saved.status, StatusCode::NO_CONTENT, "{}", saved.body);
    let page = page(&app, &cookie, &path).await;
    assert_eq!(page["schedule"]["cut_off_offset_days"], 8);
    assert_eq!(page["schedule"]["reminder_days"], json!([10, 2]));
    assert_eq!(page["paused"], true);
    // Q4 2026: 2551Q due 25 Jan 2027, minus 8 days.
    assert_eq!(page["periods"][0]["cut_off"], "2027-01-17");

    let refused = schedule(json!({"cut_off_offset_days": 5, "reminder_days": [7, 3, 2, 1], "paused": false})).await;
    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        refused.json()["message"]
            .as_str()
            .unwrap()
            .contains("choose at most 3 reminder days"),
        "{}",
        refused.body
    );
}

#[sqlx::test]
async fn b3_a_due_date_set_for_one_client_moves_only_that_clients_dates(pool: PgPool) {
    let app = TestApp::new(pool);
    let (cookie, path) = nena(&app).await;
    let other = json!({
        "business_name": "Other Store", "contact_first_name": "Ana",
        "email": "ana@example.test", "language": "english", "forms": ["2551Q"],
    });
    let other = app
        .api(Method::POST, "/api/clients", Some(&cookie), Some(other))
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let set = |due: Value| {
        let change = json!({"form": "2551Q", "period": "2026-Q4", "due": due});
        call(&app, Method::PUT, format!("{path}/due-dates"), &cookie, Some(change))
    };

    let saved = set(json!("2027-01-29")).await;

    assert_eq!(saved.status, StatusCode::NO_CONTENT, "{}", saved.body);
    let q4 = page(&app, &cookie, &path).await["periods"][1].clone();
    assert_eq!(q4["filings"][0]["due"], "2027-01-29");
    assert_eq!(q4["filings"][0]["overridden"], true);
    assert_eq!(q4["cut_off"], "2027-01-24");
    let others = page(&app, &cookie, &format!("/api/clients/{other}")).await;
    assert_eq!(others["periods"][1]["cut_off"], "2027-01-20");

    set(Value::Null).await;
    assert_eq!(page(&app, &cookie, &path).await["periods"][1]["cut_off"], "2027-01-20");
}

#[sqlx::test]
async fn b4_the_checklist_can_be_changed_for_one_client(pool: PgPool) {
    let app = TestApp::new(pool);
    let (cookie, path) = nena(&app).await;
    let bank = "Bank and e-wallet (GCash, Maya) statements";
    let owed = |page: &Value| page["periods"][0]["owed"].to_string();
    let post = |to: &str, body: Value| call(&app, Method::POST, format!("{path}/{to}"), &cookie, Some(body));

    post("checklist/remove", json!({"key": "bank-statements"})).await;
    let added = post(
        "checklist/extras",
        json!({"label": "Lazada payout reports", "recurs": "every-period"}),
    )
    .await;

    assert_eq!(added.status, StatusCode::CREATED, "{}", added.body);
    let changed = page(&app, &cookie, &path).await;
    assert!(
        !owed(&changed).contains(bank) && owed(&changed).contains("Lazada payout reports"),
        "{changed}"
    );
    assert!(
        changed["checklist"]["defaults"]
            .to_string()
            .contains(r#""removed":true"#)
    );

    let item = changed["checklist"]["extras"][0]["id"].as_str().unwrap().to_owned();
    app.api(
        Method::DELETE,
        &format!("{path}/checklist/extras/{item}"),
        Some(&cookie),
        None,
    )
    .await;
    post("checklist/restore", json!({"key": "bank-statements"})).await;

    let restored = page(&app, &cookie, &path).await;
    assert!(
        owed(&restored).contains(bank) && !owed(&restored).contains("Lazada"),
        "{restored}"
    );
}

#[sqlx::test]
async fn h2_the_firm_lists_a_clients_payors(pool: PgPool) {
    let app = TestApp::new(pool);
    let (cookie, path) = nena(&app).await;
    let add = |name: &str| {
        call(
            &app,
            Method::POST,
            format!("{path}/payors"),
            &cookie,
            Some(json!({ "name": name })),
        )
    };

    let added = add("Jollibee Foods Corp").await;
    let refused = add("www.pay-here.ph").await;

    assert_eq!(added.status, StatusCode::CREATED);
    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        refused.json()["message"]
            .as_str()
            .unwrap()
            .contains("can't look like a web address"),
        "{}",
        refused.body
    );
    let page = page(&app, &cookie, &path).await;
    assert_eq!(page["payors"][0]["name"], "Jollibee Foods Corp");

    let payor = page["payors"][0]["id"].as_str().unwrap().to_owned();
    let removed = app
        .api(Method::DELETE, &format!("{path}/payors/{payor}"), Some(&cookie), None)
        .await;
    assert_eq!(removed.status, StatusCode::NO_CONTENT);
}
