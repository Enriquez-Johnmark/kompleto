//! The 2307 tracker through the API, with the test clock (H2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::checklist::ANOTHER_PAYOR_2307;
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::payors::Payors;
use kompleto::kompleto::uploads::{Label, Uploads};
use serde_json::json;
use sqlx::PgPool;
use time::macros::date;

#[sqlx::test]
async fn h2_the_tracker_page_shows_the_last_quarter_and_adds_an_unexpected_payor(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(app.pool.clone(), app.clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let payors = Payors::new(app.pool.clone(), app.clock.clone());
    payors.add(santos, client, "Jollibee Foods Corp").await.unwrap();
    let links = common::upload_links(&app.pool, app.clock.clone());
    let url = links
        .for_period(santos, client, "2026-09", date!(2026 - 10 - 26))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    Uploads::new(app.pool.clone(), app.clock.clone(), app.files.clone())
        .add(
            &link,
            &[ANOTHER_PAYOR_2307.to_owned()],
            Label::Owed {
                document: ANOTHER_PAYOR_2307.to_owned(),
                payor: Some("Robinsons Land".to_owned()),
            },
            b"%PDF-1.7",
        )
        .await
        .unwrap();

    // 21 Oct 2026: the last quarter that ended is Q3.
    let tracker = |quarter: &str| {
        let path = if quarter.is_empty() {
            "/api/tracker".to_owned()
        } else {
            format!("/api/tracker?quarter={quarter}")
        };
        call(&app, path, cookie.clone())
    };
    let page = tracker("").await;

    assert_eq!(page.status, StatusCode::OK, "{}", page.body);
    let page = page.json();
    assert_eq!(page["quarter"], "Q3 2026");
    assert_eq!(page["earlier"], "2026-Q2");
    assert_eq!(page["later"], serde_json::Value::Null, "Q4 hasn't ended");
    let nena = &page["clients"][0];
    assert_eq!(nena["business_name"], "Nena's Sari-Sari Store");
    assert_eq!(
        nena["payors"],
        json!([{"name": "Jollibee Foods Corp", "status": "missing"}])
    );
    assert_eq!(nena["unexpected"], json!(["Robinsons Land"]));

    let added = app
        .api(
            Method::POST,
            &format!("/api/clients/{client}/payors"),
            Some(&cookie),
            Some(json!({"name": "Robinsons Land"})),
        )
        .await;
    assert_eq!(added.status, StatusCode::CREATED);
    let page = tracker("2026-Q3").await.json();
    assert_eq!(page["clients"][0]["unexpected"], json!([]), "listed now");
    assert!(
        page["clients"][0]["payors"]
            .to_string()
            .contains(r#"{"name":"Robinsons Land","status":"uploaded"}"#),
        "{page}"
    );

    let earlier = tracker("2026-Q2").await.json();
    assert_eq!(earlier["quarter"], "Q2 2026");
    assert!(
        earlier["clients"][0]["payors"]
            .to_string()
            .contains(r#"{"name":"Robinsons Land","status":"missing"}"#),
        "{earlier}"
    );
    assert_eq!(tracker("2026-Q5").await.status, StatusCode::NOT_FOUND);
}

async fn call(app: &TestApp, path: String, cookie: String) -> common::web::Response {
    app.api(Method::GET, &path, Some(&cookie), None).await
}
