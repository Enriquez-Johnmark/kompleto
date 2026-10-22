//! Seam 3 (M6): closing a Firm's account in-process (I2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;
use time::Duration;

async fn waiting(app: &TestApp, firm: FirmId) -> Vec<String> {
    sqlx::query_scalar(
        "select m.status from outbox_messages m join scheduled_drafts s on s.message_id = m.id
         where s.firm_id = $1 order by m.created_at",
    )
    .bind(firm.as_uuid())
    .fetch_all(&app.pool)
    .await
    .unwrap()
}

#[sqlx::test]
async fn i2_closing_an_account_stops_everything_at_once(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let (reyes, _) = app.logged_in_firm("Reyes CPA").await;
    let employer = NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    let clients = Clients::new(app.pool.clone(), app.clock.clone());
    clients.create(santos, &employer).await.unwrap();
    clients.create(reyes, &employer).await.unwrap();
    app.clock.advance(Duration::days(11)); // 1 Nov: October's first requests
    let scheduler = Scheduler::new(
        app.pool.clone(),
        app.clock.clone(),
        common::upload_links(&app.pool, app.clock.clone()),
    )
    .unwrap();
    scheduler.run_once().await.unwrap();
    let link: String =
        sqlx::query_scalar("select body_text from outbox_messages where to_address = 'nena@example.test' limit 1")
            .fetch_one(&app.pool)
            .await
            .unwrap();
    let link = link
        .split_whitespace()
        .find(|word| word.starts_with("http://kompleto.test/u/"))
        .unwrap()
        .trim_start_matches(common::web::BASE_URL)
        .to_owned();

    let mistyped = app.post("/settings/close", &cookie, "confirm=Santos").await;
    assert_eq!(mistyped.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(waiting(&app, santos).await, ["draft"], "nothing changed");
    let closed = app
        .post("/settings/close", &cookie, "confirm=+santos+bookkeeping+")
        .await;

    assert_eq!(closed.status, StatusCode::OK);
    assert!(
        closed.body.contains("Sun 8 Nov 2026"),
        "deleted 7 days later: {}",
        closed.body
    );
    assert_eq!(waiting(&app, santos).await, ["cancelled"]);
    assert_eq!(waiting(&app, reyes).await, ["draft"], "another Firm's untouched");
    assert_eq!(app.get("/", &cookie).await.status, StatusCode::SEE_OTHER, "logged out");
    let before: i64 = sqlx::query_scalar("select count(*) from login_tokens")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    app.request(Method::POST, "/login", None, Some("email=santos.bookkeeping@firm.test"))
        .await;
    let after: i64 = sqlx::query_scalar("select count(*) from login_tokens")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(before, after, "no login link for a closed account");
    let upload_page = app.request(Method::GET, &link, None, None).await;
    assert_eq!(upload_page.status, StatusCode::NOT_FOUND, "its Upload links are gone");
    app.clock.advance(Duration::days(3)); // 4 Nov: the next Reminder's day
    scheduler.run_once().await.unwrap();
    assert_eq!(waiting(&app, santos).await, ["cancelled"], "no new Drafts");
    let logged: i64 =
        sqlx::query_scalar("select count(*) from audit_events where tenant_id = $1 and action = 'firm.closed'")
            .bind(santos.as_uuid())
            .fetch_one(&app.pool)
            .await
            .unwrap();
    assert_eq!(logged, 1);
}
