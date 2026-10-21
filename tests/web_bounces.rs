//! Seam 2 (M5): delivery events through the app's signed endpoint (G3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use common::nena;
use common::web::{Response, TestApp, WEBHOOK_SECRET};
use hmac::{Hmac, KeyInit, Mac};
use kompleto::engine::clock::Clock;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::reminders::Scheduler;
use sha2::Sha256;
use sqlx::PgPool;
use time::Duration;

const BOUNCE: &str = r#"{"type":"email.bounced","data":{"email_id":"re_1","bounce":{"type":"Permanent","message":"Mailbox does not exist"}}}"#;

/// October's first Reminder, sent on 1 Nov as provider message `re_1`.
async fn sent(app: &TestApp) {
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
    sqlx::query("update outbox_messages set status = 'sent', approved_hash = '\\x00', provider_message_id = 're_1', sent_at = $1")
        .bind(app.clock.now())
        .execute(&app.pool)
        .await
        .unwrap();
}

async fn deliver(app: &TestApp, body: &str, secret: &str, timestamp: i64) -> Response {
    let key = STANDARD.decode(secret.trim_start_matches("whsec_")).unwrap();
    let mac = <Hmac<Sha256> as KeyInit>::new_from_slice(&key).unwrap();
    let digest = mac
        .chain_update(format!("msg_1.{timestamp}.{body}").as_bytes())
        .finalize()
        .into_bytes();
    let request = Request::builder()
        .method(Method::POST)
        .uri("/hooks/resend")
        .header("svix-id", "msg_1")
        .header("svix-timestamp", timestamp.to_string())
        .header("svix-signature", format!("v1,{}", STANDARD.encode(digest)))
        .body(Body::from(body.to_owned()))
        .unwrap();
    app.send(request).await
}

#[sqlx::test]
async fn g3_a_signed_bounce_is_recorded_and_a_forged_or_old_one_refused(pool: PgPool) {
    let app = TestApp::new(pool);
    sent(&app).await;
    let now = app.clock.now().unix_timestamp();

    let forged = deliver(&app, BOUNCE, "whsec_b3RoZXI=", now).await;
    let old = deliver(&app, BOUNCE, WEBHOOK_SECRET, now - 600).await;
    let good = deliver(&app, BOUNCE, WEBHOOK_SECRET, now).await;

    assert_eq!(forged.status, StatusCode::UNAUTHORIZED);
    assert_eq!(old.status, StatusCode::UNAUTHORIZED);
    assert_eq!(good.status, StatusCode::NO_CONTENT, "{}", good.body);
    let kinds: Vec<String> = sqlx::query_scalar("select kind from bounces")
        .fetch_all(&app.pool)
        .await
        .unwrap();
    assert_eq!(kinds, ["bounce"]);
}
