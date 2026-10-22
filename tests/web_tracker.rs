//! Seam 3 (M5): the 2307 tracker page in-process with the test clock (H2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::checklist::ANOTHER_PAYOR_2307;
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::payors::Payors;
use kompleto::kompleto::uploads::{Label, Uploads};
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
    let page = app.get("/tracker", &cookie).await;

    assert_eq!(page.status, StatusCode::OK);
    for expected in [
        "Q3 2026",
        "Nena&#39;s Sari-Sari Store",
        "Jollibee Foods Corp",
        "missing",
        "Robinsons Land",
        "unexpected",
        r#"href="/tracker?quarter=2026-Q2""#,
    ] {
        assert!(page.body.contains(expected), "missing {expected:?} in {}", page.body);
    }
    let added = app
        .api(
            axum::http::Method::POST,
            &format!("/api/clients/{client}/payors"),
            Some(&cookie),
            Some(serde_json::json!({"name": "Robinsons Land"})),
        )
        .await;
    assert_eq!(added.status, StatusCode::CREATED);
    let page = app.get("/tracker?quarter=2026-Q3", &cookie).await;
    assert!(!page.body.contains("unexpected"), "listed now: {}", page.body);
    assert!(page.body.contains("uploaded"), "{}", page.body);

    let earlier = app.get("/tracker?quarter=2026-Q2", &cookie).await;
    assert!(earlier.body.contains("Q2 2026"), "{}", earlier.body);
    assert!(
        earlier.body.contains("Robinsons Land: <strong>missing"),
        "{}",
        earlier.body
    );
    assert_eq!(
        app.get("/tracker?quarter=2026-Q5", &cookie).await.status,
        StatusCode::NOT_FOUND
    );
}
