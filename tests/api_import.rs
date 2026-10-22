//! Importing clients through the API (B2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use common::web::{BASE_URL, Response, TestApp};
use kompleto::kompleto::clients::Clients;
use serde_json::json;
use sqlx::PgPool;

const CSV: &str = "Business name,Contact first name,Email,Language,Forms\n\
                   Nena's Store,Nena,nena@example.test,Taglish,\"2551Q, 1701Q\"\n\
                   ,Ramon,ramon@example.test,English,2550Q\n\
                   Lito Hardware,Lito,lito@example.test,English,2550Q; 0619-E\n";

async fn upload(app: &TestApp, cookie: &str, filename: &str, bytes: &[u8]) -> Response {
    let boundary = "kompleto-test-boundary";
    let mut body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(bytes);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    let request = Request::post("/api/import")
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
async fn b2_an_upload_is_previewed_and_only_confirming_adds_the_valid_rows(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let clients = Clients::new(pool, app.clock.clone());

    let preview = upload(&app, &cookie, "clients.csv", CSV.as_bytes()).await;

    assert_eq!(preview.status, StatusCode::OK, "{}", preview.body);
    let preview = preview.json();
    let valid: Vec<&str> = preview["valid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["business_name"].as_str().unwrap())
        .collect();
    assert_eq!(valid, ["Nena's Store", "Lito Hardware"]);
    assert_eq!(preview["invalid"][0]["row"], 3);
    assert!(
        preview["invalid"][0]["reasons"]
            .to_string()
            .contains("Business name: is required"),
        "{preview}"
    );
    assert!(
        clients.list(firm).await.unwrap().is_empty(),
        "nothing is added before confirming"
    );

    let confirmed = app
        .api(
            Method::POST,
            "/api/import/confirm",
            Some(&cookie),
            Some(json!({ "rows": preview["rows"] })),
        )
        .await;

    assert_eq!(confirmed.status, StatusCode::OK, "{}", confirmed.body);
    assert_eq!(confirmed.json()["added"], 2);
    let names: Vec<_> = clients
        .list(firm)
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.details.business_name)
        .collect();
    assert_eq!(names, ["Lito Hardware", "Nena's Store"]);
}

#[sqlx::test]
async fn b2_a_file_that_cant_be_read_is_refused_whole(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let wrong_columns = upload(&app, &cookie, "clients.csv", b"Phone,Notes\n0917,regular\n").await;
    let wrong_type = upload(&app, &cookie, "clients.pdf", b"%PDF-1.7").await;

    assert_eq!(wrong_columns.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        wrong_columns.json()["message"]
            .as_str()
            .unwrap()
            .contains("the first row needs these columns"),
        "{}",
        wrong_columns.body
    );
    assert_eq!(wrong_type.json()["message"], "Upload a .csv or .xlsx file.");
}

#[sqlx::test]
async fn b2_the_template_file_has_the_import_columns(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let template = app.get("/api/import/template.csv", &cookie).await;

    assert_eq!(template.status, StatusCode::OK);
    assert!(
        template
            .body
            .starts_with("Business name,Contact first name,Email,Language,Forms\n")
    );
}

#[sqlx::test]
async fn b2_a_confirmation_over_the_row_limit_is_refused(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let row = json!({
        "business_name": "C", "contact_first_name": "A", "email": "a@b.test",
        "language": "English", "forms": ["2551Q"],
    });
    let rows: Vec<_> = (0..=500).map(|_| row.clone()).collect();

    let refused = app
        .api(
            Method::POST,
            "/api/import/confirm",
            Some(&cookie),
            Some(json!({ "rows": rows })),
        )
        .await;

    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        refused.json()["message"]
            .as_str()
            .unwrap()
            .contains("more than 500 rows"),
        "{}",
        refused.body
    );
    assert!(
        Clients::new(pool, app.clock.clone())
            .list(firm)
            .await
            .unwrap()
            .is_empty()
    );
}
