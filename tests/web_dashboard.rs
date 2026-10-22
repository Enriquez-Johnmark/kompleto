//! Seam 3 (M5): the dashboard in-process with the test clock (H1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{ClientId, Clients, NewClient, ScheduleSettings};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::schedule::ReminderDays;
use kompleto::kompleto::uploads::{Label, Uploads};
use sqlx::PgPool;
use time::macros::date;

fn employer(name: &str) -> NewClient {
    NewClient {
        business_name: name.to_owned(),
        forms: vec![BirForm::F1601C],
        ..nena()
    }
}

async fn add(app: &TestApp, firm: FirmId, name: &str, offset: Option<u8>, paused: bool) -> ClientId {
    let clients = Clients::new(app.pool.clone(), app.clock.clone());
    let id = clients.create(firm, &employer(name)).await.unwrap();
    let settings = ScheduleSettings {
        cut_off_offset_days: offset,
        reminder_days: ReminderDays::new(&[7, 3, 1]).unwrap(),
        paused,
    };
    clients.set_schedule(firm, id, &settings).await.unwrap();
    id
}

#[sqlx::test]
async fn h1_the_dashboard_lists_clients_nearest_cut_off_first_with_whats_missing(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    // October's 1601-C is due 10 Nov: Alpha's Cut-off is 5 Nov, Bravo's
    // (a 12-day offset) 1 Nov, the earliest it can be. Charlie is paused.
    let alpha = add(&app, santos, "Alpha Trading", None, false).await;
    let bravo = add(&app, santos, "Bravo Hardware", Some(12), false).await;
    add(&app, santos, "Charlie Foods", None, true).await;
    let links = common::upload_links(&app.pool, app.clock.clone());
    let url = links
        .for_period(santos, alpha, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    let statements = "Bank and e-wallet (GCash, Maya) statements".to_owned();
    Uploads::new(app.pool.clone(), app.clock.clone(), app.files.clone())
        .add(
            &link,
            std::slice::from_ref(&statements),
            Label::Owed {
                document: statements.clone(),
                payor: None,
            },
            b"%PDF-1.7",
        )
        .await
        .unwrap();
    sqlx::query(
        "insert into replies (id, firm_id, client_id, period, from_address, automatic, received_at)
         values ($1, $2, $3, '2026-10', 'nena@example.test', false, now())",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(santos.as_uuid())
    .bind(bravo.as_uuid())
    .execute(&app.pool)
    .await
    .unwrap();

    // Charlie's address bounced permanently.
    let charlie: uuid::Uuid = sqlx::query_scalar("select id from clients where business_name = 'Charlie Foods'")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    sqlx::query(
        "insert into bounces (id, firm_id, client_id, message_id, address, kind, at)
         values ($1, $2, $3, $1, 'nena@example.test', 'bounce', now())",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(santos.as_uuid())
    .bind(charlie)
    .execute(&app.pool)
    .await
    .unwrap();

    let page = app.get("/", &cookie).await;

    assert_eq!(page.status, StatusCode::OK);
    let at = |text: &str| {
        page.body
            .find(text)
            .unwrap_or_else(|| panic!("no {text:?} in {}", page.body))
    };
    assert!(at("Bravo Hardware") < at("Alpha Trading"), "nearest Cut-off first");
    assert!(at("Alpha Trading") < at("Charlie Foods"), "paused last");
    assert!(page.body.contains("Sun 1 Nov 2026") && page.body.contains("Thu 5 Nov 2026"));
    let alpha_row = &page.body[at("Alpha Trading")..at("Charlie Foods")];
    assert!(alpha_row.contains("Payroll register"), "{alpha_row}");
    assert!(!alpha_row.contains("GCash, Maya"), "already uploaded: {alpha_row}");
    let bravo_row = &page.body[at("Bravo Hardware")..at("Alpha Trading")];
    assert!(bravo_row.contains("replied"), "{bravo_row}");
    let charlie_row = &page.body[at("Charlie Foods")..];
    assert!(charlie_row.contains("email bounced"), "g3: {charlie_row}");
    assert!(!alpha_row.contains("email bounced"));
}

#[sqlx::test]
async fn the_dashboard_says_when_approved_messages_are_waiting_to_go_out(pool: PgPool) {
    use kompleto::engine::clock::Clock;
    use kompleto::engine::outbox::{NoExtraChecks, Outbox};

    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let outbox = Outbox::new(app.pool.clone(), app.clock.clone());
    let email = kompleto::engine::mail::Email {
        to_address: "nena@example.test".to_owned(),
        from_name: "Santos Bookkeeping".to_owned(),
        reply_to: None,
        subject: "Documents for October".to_owned(),
        body_text: "Hi Nena".to_owned(),
    };
    let mut tx = app.pool.begin().await.unwrap();
    let deadline = app.clock.now() + time::Duration::days(5);
    let id = outbox
        .create_draft_in(&mut tx, santos, &email, Some(deadline))
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let seen = outbox.get(santos, id).await.unwrap().content_hash();
    outbox
        .approve(santos, id, "a@firm.test", &seen, &NoExtraChecks)
        .await
        .unwrap();

    assert!(
        !app.get("/", &cookie).await.body.contains("go out yet"),
        "just approved"
    );
    app.clock.advance(time::Duration::minutes(30));
    let page = app.get("/", &cookie).await;

    assert!(
        page.body.contains("1 approved reminder hasn't gone out yet"),
        "{}",
        page.body
    );
    assert!(page.body.contains("at most 90 reminders a day"));
}
