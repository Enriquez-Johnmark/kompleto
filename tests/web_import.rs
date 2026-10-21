//! Seam 4: importing clients through the web pages (B2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use common::web::{BASE_URL, Response, TestApp};
use kompleto::kompleto::clients::Clients;
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
    let request = Request::post("/import")
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

/// The preview page's hidden `rows` field, unescaped and URL-encoded for posting.
fn confirm_form(preview: &str) -> String {
    let start = preview.find(r#"name="rows" value=""#).unwrap() + r#"name="rows" value=""#.len();
    let html = &preview[start..start + preview[start..].find('"').unwrap()];
    let json = html
        .replace("&quot;", "\"")
        .replace("&#34;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");
    let encoded: String = json
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect();
    format!("rows={encoded}")
}

#[sqlx::test]
async fn b2_an_upload_is_previewed_and_only_confirming_adds_the_valid_rows(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let clients = Clients::new(pool, app.clock.clone());

    let preview = upload(&app, &cookie, "clients.csv", CSV.as_bytes()).await;

    assert_eq!(preview.status, StatusCode::OK, "{}", preview.body);
    for expected in ["Add 2 clients", "Row 3", "Business name: is required", "Lito Hardware"] {
        assert!(
            preview.body.contains(expected),
            "missing {expected:?} in {}",
            preview.body
        );
    }
    assert!(
        clients.list(firm).await.unwrap().is_empty(),
        "nothing is added before confirming"
    );

    let confirmed = app.post("/import/confirm", &cookie, &confirm_form(&preview.body)).await;

    assert_eq!(confirmed.status, StatusCode::SEE_OTHER, "{}", confirmed.body);
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

    let wrong_columns = upload(&app, &cookie, "clients.csv", b"Name,Email\nShop,a@b.test\n").await;
    let wrong_type = upload(&app, &cookie, "clients.pdf", b"%PDF-1.7").await;

    assert_eq!(wrong_columns.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        wrong_columns.body.contains("the first row needs these columns"),
        "{}",
        wrong_columns.body
    );
    assert!(
        wrong_type.body.contains("Upload a .csv or .xlsx file"),
        "{}",
        wrong_type.body
    );
}

#[sqlx::test]
async fn b2_the_template_file_has_the_import_columns(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let template = app.get("/import/template.csv", &cookie).await;

    assert_eq!(template.status, StatusCode::OK);
    assert!(
        template
            .body
            .starts_with("Business name,Contact first name,Email,Language,Forms\n")
    );
}
