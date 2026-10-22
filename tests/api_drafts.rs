//! The Drafts endpoints (D2, D3, D4, D7, E3, A2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use kompleto::engine::outbox::{MessageStatus, Outbox};
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{ClientId, Clients, NewClient};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::reminders::Scheduler;
use serde_json::{Value, json};
use sqlx::PgPool;
use time::Duration;
use uuid::Uuid;

fn employer(name: &str, email: &str) -> NewClient {
    NewClient {
        business_name: name.to_owned(),
        contact_first_name: "Lito".to_owned(),
        email: email.to_owned(),
        forms: vec![BirForm::F1601C],
        ..common::nena()
    }
}

/// Creates the Drafts due on 1 Nov 2026 for `firm`'s Clients.
async fn schedule(app: &TestApp, firm: FirmId) -> Vec<Uuid> {
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
    sqlx::query_scalar("select message_id from scheduled_drafts where firm_id = $1 order by message_id")
        .bind(firm.as_uuid())
        .fetch_all(&app.pool)
        .await
        .unwrap()
}

/// A logged-in firm with one Client and its first reminder Draft.
async fn firm_with_draft(app: &TestApp) -> (FirmId, String, ClientId, Uuid) {
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = Clients::new(app.pool.clone(), app.clock.clone())
        .create(firm, &employer("Lito Hardware", "lito@example.test"))
        .await
        .unwrap();
    let drafts = schedule(app, firm).await;
    (firm, cookie, client, drafts[0])
}

/// Two Clients' Drafts for one firm.
async fn firm_with_two_drafts(app: &TestApp) -> (FirmId, String, Vec<Uuid>) {
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let clients = Clients::new(app.pool.clone(), app.clock.clone());
    for (name, email) in [
        ("Lito Hardware", "lito@example.test"),
        ("Ben Bakery", "ben@example.test"),
    ] {
        clients.create(firm, &employer(name, email)).await.unwrap();
    }
    let drafts = schedule(app, firm).await;
    (firm, cookie, drafts)
}

async fn status(app: &TestApp, firm: FirmId, draft: Uuid) -> MessageStatus {
    Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap()
        .status
}

async fn drafts(app: &TestApp, cookie: &str) -> Value {
    let list = app.api(Method::GET, "/api/drafts", Some(cookie), None).await;
    assert_eq!(list.status, StatusCode::OK, "{}", list.body);
    list.json()
}

async fn approve(app: &TestApp, cookie: &str, draft: Uuid, hash: &str) -> common::web::Response {
    app.api(
        Method::POST,
        &format!("/api/drafts/{draft}/approve"),
        Some(cookie),
        Some(json!({ "hash": hash })),
    )
    .await
}

#[sqlx::test]
async fn the_drafts_list_shows_each_drafts_recipient_full_text_and_label(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, _, draft) = firm_with_draft(&app).await;
    let email = Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap()
        .email;

    let list = drafts(&app, &cookie).await;

    let shown = &list["waiting"][0];
    assert_eq!(shown["id"], draft.to_string());
    assert_eq!(shown["client"], "Lito Hardware");
    assert_eq!(shown["to"], "lito@example.test");
    assert_eq!(shown["source"], "template");
    assert_eq!(shown["cut_off"], "2026-11-05");
    assert_eq!(shown["subject"], email.subject);
    assert_eq!(shown["body"], email.body_text, "the full text is shown");
}

#[sqlx::test]
async fn e3_copy_for_viber_is_exactly_the_message_text_with_its_upload_link(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, _, draft) = firm_with_draft(&app).await;
    let body = Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap()
        .email
        .body_text;

    let shown = app
        .api(Method::GET, &format!("/api/drafts/{draft}"), Some(&cookie), None)
        .await
        .json();

    assert_eq!(shown["body"], body);
    assert!(body.contains("/u/"), "the upload link is in the text: {body}");
}

#[sqlx::test]
async fn d2_approving_a_draft_binds_the_text_shown(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, _, draft) = firm_with_draft(&app).await;

    let refused = approve(&app, &cookie, draft, &"00".repeat(32)).await;
    assert_eq!(refused.status, StatusCode::CONFLICT);
    assert_eq!(
        refused.json()["message"],
        "This draft changed since you saw it; check it again."
    );
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Draft);

    let hash = drafts(&app, &cookie).await["waiting"][0]["hash"]
        .as_str()
        .unwrap()
        .to_owned();
    let approved = approve(&app, &cookie, draft, &hash).await;

    assert_eq!(approved.status, StatusCode::NO_CONTENT, "{}", approved.body);
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Approved);
    let list = drafts(&app, &cookie).await;
    assert_eq!(list["waiting"].as_array().unwrap().len(), 0);
    assert_eq!(list["approved"][0]["status"], "approved");
}

#[sqlx::test]
async fn a_discarded_draft_leaves_the_list(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, _, draft) = firm_with_draft(&app).await;

    let discarded = app
        .api(
            Method::POST,
            &format!("/api/drafts/{draft}/discard"),
            Some(&cookie),
            None,
        )
        .await;

    assert_eq!(discarded.status, StatusCode::NO_CONTENT);
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Cancelled);
    assert_eq!(drafts(&app, &cookie).await["waiting"], json!([]));
}

#[sqlx::test]
async fn d4_an_expired_draft_leaves_the_list_and_cant_be_approved(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, _, draft) = firm_with_draft(&app).await;
    let hash = drafts(&app, &cookie).await["waiting"][0]["hash"]
        .as_str()
        .unwrap()
        .to_owned();

    app.clock.advance(Duration::days(5)); // past the end of Thu 5 Nov

    assert_eq!(drafts(&app, &cookie).await["waiting"], json!([]));
    let late = approve(&app, &cookie, draft, &hash).await;
    assert_eq!(late.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        late.json()["message"],
        "This draft expired at its cut-off and can't be sent."
    );
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Expired);
}

#[sqlx::test]
async fn d2_editing_a_draft_changes_its_text_and_voids_any_approval(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, _, draft) = firm_with_draft(&app).await;
    let hash = drafts(&app, &cookie).await["waiting"][0]["hash"]
        .as_str()
        .unwrap()
        .to_owned();
    approve(&app, &cookie, draft, &hash).await;

    let saved = app
        .api(
            Method::PUT,
            &format!("/api/drafts/{draft}"),
            Some(&cookie),
            Some(json!({"subject": "Kulang pa po", "body": "Hi Lito,\r\npaki-send po ang October receipts."})),
        )
        .await;

    assert_eq!(saved.status, StatusCode::OK, "{}", saved.body);
    assert_eq!(saved.json()["status"], "draft", "the earlier approval is void");
    assert_ne!(saved.json()["hash"], hash.as_str());
    let message = Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap();
    assert_eq!(message.status, MessageStatus::Draft);
    assert_eq!(message.email.subject, "Kulang pa po");
    assert_eq!(message.email.body_text, "Hi Lito,\npaki-send po ang October receipts.");
    assert_eq!(
        message.email.to_address, "lito@example.test",
        "the recipient can't be edited"
    );
    let editor: String = sqlx::query_scalar("select actor from audit_events where action = 'outbox.edited'")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(editor, "santos.bookkeeping@firm.test");
}

#[sqlx::test]
async fn d3_a_batch_approves_every_reviewed_draft_against_the_text_shown(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, ids) = firm_with_two_drafts(&app).await;
    let reviewed: Vec<Value> = drafts(&app, &cookie).await["waiting"]
        .as_array()
        .unwrap()
        .iter()
        .map(|draft| json!({"id": draft["id"], "hash": draft["hash"]}))
        .collect();
    for id in &ids {
        assert_eq!(
            status(&app, firm, *id).await,
            MessageStatus::Draft,
            "reviewing approves nothing"
        );
    }

    let result = app
        .api(
            Method::POST,
            "/api/drafts/approve",
            Some(&cookie),
            Some(json!({ "approvals": reviewed })),
        )
        .await;

    assert_eq!(result.status, StatusCode::OK, "{}", result.body);
    assert_eq!(result.json(), json!({"approved": 2, "refused": []}));
    for id in &ids {
        assert_eq!(status(&app, firm, *id).await, MessageStatus::Approved);
    }
}

#[sqlx::test]
async fn d3_a_draft_changed_after_the_review_is_not_approved(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, ids) = firm_with_two_drafts(&app).await;
    let shown = drafts(&app, &cookie).await;
    let approvals: Vec<Value> = shown["waiting"]
        .as_array()
        .unwrap()
        .iter()
        .map(|draft| json!({"id": draft["id"], "hash": draft["hash"]}))
        .collect();
    let changed = ids[0];
    app.api(
        Method::PUT,
        &format!("/api/drafts/{changed}"),
        Some(&cookie),
        Some(json!({"subject": "Changed", "body": "Changed text."})),
    )
    .await;

    let result = app
        .api(
            Method::POST,
            "/api/drafts/approve",
            Some(&cookie),
            Some(json!({ "approvals": approvals })),
        )
        .await
        .json();

    assert_eq!(result["approved"], 1);
    assert_eq!(result["refused"][0]["id"], changed.to_string());
    assert_eq!(
        result["refused"][0]["reason"],
        "This draft changed since you saw it; check it again."
    );
    assert_eq!(status(&app, firm, ids[0]).await, MessageStatus::Draft);
    assert_eq!(status(&app, firm, ids[1]).await, MessageStatus::Approved);
}

#[sqlx::test]
async fn a2_a_batch_with_another_firms_draft_approves_nothing(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, ids) = firm_with_two_drafts(&app).await;
    let (_, reyes) = app.logged_in_firm("Reyes CPA").await;
    let shown = drafts(&app, &cookie).await;
    let approvals: Vec<Value> = shown["waiting"]
        .as_array()
        .unwrap()
        .iter()
        .map(|draft| json!({"id": draft["id"], "hash": draft["hash"]}))
        .collect();

    let result = app
        .api(
            Method::POST,
            "/api/drafts/approve",
            Some(&reyes),
            Some(json!({ "approvals": approvals })),
        )
        .await;

    assert_eq!(result.status, StatusCode::NOT_FOUND);
    for id in ids {
        assert_eq!(status(&app, firm, id).await, MessageStatus::Draft);
    }
}

#[sqlx::test]
async fn d7_a_reminder_blocked_at_sending_stays_listed_with_its_reason(pool: PgPool) {
    use std::sync::Arc;

    let app = TestApp::new(pool);
    let (firm, cookie, client, draft) = firm_with_draft(&app).await;
    let hash = drafts(&app, &cookie).await["waiting"][0]["hash"]
        .as_str()
        .unwrap()
        .to_owned();
    approve(&app, &cookie, draft, &hash).await;
    // The client's address changes before the message goes out.
    Clients::new(app.pool.clone(), app.clock.clone())
        .update(firm, client, &employer("Lito Hardware", "lito@new.test"))
        .await
        .unwrap();
    let checks = Arc::new(common::kompleto_checks(&app.pool, app.clock.clone()));
    kompleto::engine::outbox::Sender::new(
        app.pool.clone(),
        common::RecordingMailer::idempotent(),
        app.clock.clone(),
        checks,
    )
    .drain_once(10)
    .await
    .unwrap();
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Blocked);

    let shown = &drafts(&app, &cookie).await["approved"][0];

    assert_eq!(shown["client"], "Lito Hardware");
    assert_eq!(shown["status"], "blocked");
    assert!(
        shown["problem"]
            .as_str()
            .unwrap()
            .contains("the recipient isn't the client's stored email"),
        "{shown}"
    );
}
