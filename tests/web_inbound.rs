//! Seam 1 (M5): inbound email through the app's signed endpoint (G1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use common::nena;
use common::web::{INBOUND_SECRET, Response, TestApp};
use hmac::{Hmac, KeyInit, Mac};
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::reminders::Scheduler;
use sha2::Sha256;
use sqlx::PgPool;
use time::Duration;

/// October's first Reminder for an employer Client, waiting; returns its
/// reply address.
async fn reminded(app: &TestApp) -> String {
    let santos = common::firm(&app.pool, "Santos Bookkeeping").await;
    let employer = NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    Clients::new(app.pool.clone(), app.clock.clone())
        .create(santos, &employer)
        .await
        .unwrap();
    app.clock.advance(Duration::days(11));
    Scheduler::new(
        app.pool.clone(),
        app.clock.clone(),
        common::upload_links(&app.pool, app.clock.clone()),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();
    sqlx::query_scalar("select reply_to from outbox_messages")
        .fetch_one(&app.pool)
        .await
        .unwrap()
}

fn signature(body: &[u8]) -> String {
    let mac = <Hmac<Sha256> as KeyInit>::new_from_slice(INBOUND_SECRET.as_bytes()).unwrap();
    let bytes: [u8; 32] = mac.chain_update(body).finalize().into_bytes().into();
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Posts `body` as the inbound provider would: no browser, no Origin.
async fn deliver(app: &TestApp, body: Vec<u8>, signature: &str) -> Response {
    let request = Request::builder()
        .method(Method::POST)
        .uri("/hooks/inbound-email")
        .header("x-kompleto-signature", signature)
        .body(Body::from(body))
        .unwrap();
    app.send(request).await
}

async fn count(app: &TestApp, sql: &'static str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(&app.pool).await.unwrap()
}

#[sqlx::test]
async fn g1_a_signed_reply_is_taken_in_and_stops_the_periods_reminders(pool: PgPool) {
    let app = TestApp::new(pool);
    let reply_to = reminded(&app).await;
    let body = common::inbound("human_reply", &reply_to);

    let taken = deliver(&app, body.clone(), &signature(&body)).await;

    assert_eq!(taken.status, StatusCode::NO_CONTENT, "{}", taken.body);
    assert_eq!(count(&app, "select count(*) from replies").await, 1);
    assert_eq!(
        count(&app, "select count(*) from outbox_messages where status = 'cancelled'").await,
        1
    );
}

#[sqlx::test]
async fn g1_the_firms_notice_links_to_the_reply_under_the_apps_path(pool: PgPool) {
    let app = TestApp::at(pool, "http://kompleto.test/app");
    let reply_to = reminded(&app).await;
    let body = common::inbound("human_reply", &reply_to);

    deliver(&app, body.clone(), &signature(&body)).await;

    let (status, text): (String, String) =
        sqlx::query_as("select status, body_text from outbox_messages where reply_to is null")
            .fetch_one(&app.pool)
            .await
            .unwrap();
    assert_eq!(status, "approved", "passes the pre-send checks");
    assert!(text.contains("http://kompleto.test/app/replies/"), "{text}");
}

#[sqlx::test]
async fn g1_unsigned_or_wrongly_signed_mail_is_refused_and_nothing_kept(pool: PgPool) {
    let app = TestApp::new(pool);
    let reply_to = reminded(&app).await;
    let body = common::inbound("human_reply", &reply_to);
    let mut tampered = body.clone();
    tampered.extend_from_slice(b"P.S. click this\r\n");

    for (case, body, signature) in [
        ("no signature", body.clone(), String::new()),
        ("signature of another body", tampered, signature(&body)),
        ("not hex", body.clone(), "zz".to_owned()),
    ] {
        let refused = deliver(&app, body, &signature).await;
        assert_eq!(refused.status, StatusCode::UNAUTHORIZED, "{case}");
    }
    assert_eq!(count(&app, "select count(*) from replies").await, 0);
}

#[sqlx::test]
async fn g1_the_firm_reads_a_reply_as_plain_text_data(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let santos: uuid::Uuid = sqlx::query_scalar("select id from firms")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    let employer = NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    Clients::new(app.pool.clone(), app.clock.clone())
        .create(santos.into(), &employer)
        .await
        .unwrap();
    app.clock.advance(Duration::days(11));
    Scheduler::new(
        app.pool.clone(),
        app.clock.clone(),
        common::upload_links(&app.pool, app.clock.clone()),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();
    let reply_to: String = sqlx::query_scalar("select reply_to from outbox_messages where reply_to is not null")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    let body = String::from_utf8(common::inbound("human_reply", &reply_to))
        .unwrap()
        .replace("Salamat!", "Salamat! <script>alert(1)</script>")
        .into_bytes();
    deliver(&app, body.clone(), &signature(&body)).await;
    let notice: String = sqlx::query_scalar("select body_text from outbox_messages where to_address = 'santos.bookkeeping@firm.test' and subject like '%replied%'")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    let link = notice
        .split_whitespace()
        .find(|word| word.starts_with("http://kompleto.test/replies/"))
        .unwrap()
        .trim_start_matches(common::web::BASE_URL);

    // The link opens the app's page, which reads the Reply from the API.
    let reply = app
        .api(axum::http::Method::GET, &format!("/api{link}"), Some(&cookie), None)
        .await;

    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert_eq!(reply.headers["content-type"], "application/json", "data, never a page");
    let text = reply.json()["text"].as_str().unwrap().to_owned();
    assert!(text.contains("Ipapadala ko po bukas"), "{text}");
    assert!(
        text.contains("<script>alert(1)</script>"),
        "kept as typed, for the app to show as text"
    );
    assert_eq!(reply.json()["business_name"], "Nena's Sari-Sari Store");
}
