//! Payors from a sales export through the API (H3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use common::nena;
use common::web::{BASE_URL, Response, TestApp};
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::payors::Payors;
use serde_json::json;
use sqlx::PgPool;

const SALES: &str = "Customer,Invoice,Amount,CWT,TIN\n\
                     Jollibee Foods,INV-1,\"10,000.00\",200.00,000-123-456-000\n\
                     Puregold,INV-2,\"5,000.00\",0.00,\n\
                     SM Prime,INV-3,\"8,000.00\",160.00,\n";

async fn preview(app: &TestApp, cookie: &str, filename: &str, bytes: &[u8], every_row: bool) -> Response {
    let boundary = "kompleto-test-boundary";
    let mut body = String::new();
    if every_row {
        body.push_str(&format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"every_row\"\r\n\r\ntrue\r\n"
        ));
    }
    let mut body = format!(
        "{body}--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(bytes);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    let request = Request::post("/api/payors/import")
        .header(header::COOKIE, cookie)
        .header(header::ORIGIN, BASE_URL)
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();
    app.send(request).await
}

#[sqlx::test]
async fn h3_a_sales_export_gives_the_names_and_only_the_ticked_ones_become_payors(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), app.clock.clone())
        .create(firm, &nena())
        .await
        .unwrap();
    let payors = Payors::new(pool.clone(), app.clock.clone());
    payors.add(firm, client, "Jollibee Foods").await.unwrap();

    let names = preview(&app, &cookie, "sales.csv", SALES.as_bytes(), false).await;

    assert_eq!(names.status, StatusCode::OK, "{}", names.body);
    assert_eq!(names.json(), json!({ "names": ["Jollibee Foods", "SM Prime"] }));
    assert!(!names.body.contains("000-123"), "no TIN comes back");
    assert_eq!(payors.list(firm, client).await.unwrap().len(), 1, "nothing added yet");

    let added = app
        .api(
            Method::POST,
            &format!("/api/clients/{client}/payors/batch"),
            Some(&cookie),
            Some(json!({ "names": ["JOLLIBEE FOODS", "SM Prime", "https://evil.example"] })),
        )
        .await;

    assert_eq!(added.status, StatusCode::OK, "{}", added.body);
    let added = added.json();
    assert_eq!(added["added"], 1);
    assert_eq!(added["already_listed"], 1);
    assert_eq!(added["refused"].as_array().unwrap().len(), 1, "{added}");
    let listed: Vec<String> = payors
        .list(firm, client)
        .await
        .unwrap()
        .into_iter()
        .map(|p| p.name)
        .collect();
    assert_eq!(listed, ["Jollibee Foods", "SM Prime"]);
}

#[sqlx::test]
async fn h3_a_report_of_the_withholding_account_takes_every_row_when_asked(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let report = "Transaction Detail by Account\n\nDate,Name,Amount\n07/15/2026,Jollibee Foods,200.00\n";

    let refused = preview(&app, &cookie, "cwt.csv", report.as_bytes(), false).await;
    let names = preview(&app, &cookie, "cwt.csv", report.as_bytes(), true).await;

    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY, "{}", refused.body);
    assert!(refused.body.contains("header row"), "{}", refused.body);
    assert_eq!(names.json(), json!({ "names": ["Jollibee Foods"] }));
}

#[sqlx::test]
async fn h3_a_batch_beyond_the_limit_adds_nothing(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), app.clock.clone())
        .create(firm, &nena())
        .await
        .unwrap();
    let names: Vec<String> = (0..=200).map(|i| format!("Customer {i}")).collect();

    let refused = app
        .api(
            Method::POST,
            &format!("/api/clients/{client}/payors/batch"),
            Some(&cookie),
            Some(json!({ "names": names })),
        )
        .await;

    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY, "{}", refused.body);
    let payors = Payors::new(pool, app.clock.clone());
    assert!(payors.list(firm, client).await.unwrap().is_empty());
}
