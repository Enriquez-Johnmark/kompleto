//! Seam 4 (M3): the Drafts pages (D2, D3, D4, E3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::web::TestApp;
use kompleto::engine::outbox::{MessageStatus, Outbox};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;
use time::Duration;
use uuid::Uuid;

const EMPLOYER: &str = "business_name=Lito+Hardware&contact_first_name=Lito&email=lito%40example.test\
                        &language=english&forms=1601-C";

/// A logged-in firm with one client and its first reminder Draft (1 Nov 2026).
async fn firm_with_draft(app: &TestApp, name: &str) -> (FirmId, String, Uuid) {
    let (firm, cookie) = app.logged_in_firm(name).await;
    app.post("/clients", &cookie, EMPLOYER).await;
    app.clock.advance(Duration::days(11));
    Scheduler::new(app.pool.clone(), app.clock.clone())
        .unwrap()
        .run_once()
        .await
        .unwrap();
    let draft = sqlx::query_scalar("select message_id from scheduled_drafts where firm_id = $1")
        .bind(firm.as_uuid())
        .fetch_one(&app.pool)
        .await
        .unwrap();
    (firm, cookie, draft)
}

fn hash_hex(app: &TestApp, firm: FirmId, draft: Uuid) -> impl Future<Output = String> {
    let outbox = Outbox::new(app.pool.clone(), app.clock.clone());
    async move {
        let hash = outbox.get(firm, draft.into()).await.unwrap().content_hash();
        hash.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
    }
}

async fn status(app: &TestApp, firm: FirmId, draft: Uuid) -> MessageStatus {
    Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap()
        .status
}

fn unescape(html: &str) -> String {
    html.replace("&#34;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

#[sqlx::test]
async fn the_drafts_page_shows_each_drafts_recipient_full_text_and_label(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;
    let body = Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap()
        .email
        .body_text;

    let page = app.get("/drafts", &cookie).await;

    assert_eq!(page.status, StatusCode::OK);
    for expected in [
        "Lito Hardware",
        "lito@example.test",
        "Template",
        "Cut-off: Thu 5 Nov 2026",
    ] {
        assert!(page.body.contains(expected), "missing {expected:?} in {}", page.body);
    }
    assert!(unescape(&page.body).contains(&body), "the full text is shown");
}

#[sqlx::test]
async fn e3_copy_for_viber_holds_exactly_the_message_text(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;
    let body = Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap()
        .email
        .body_text;

    let page = app.get("/drafts", &cookie).await.body;

    let marker = format!(r#"id="viber-{draft}""#);
    let start = page.find(&marker).unwrap();
    let text_start = start + page[start..].find('>').unwrap() + 1;
    let text_end = text_start + page[text_start..].find("</textarea>").unwrap();
    assert_eq!(unescape(&page[text_start..text_end]), body);
}

#[sqlx::test]
async fn d2_approving_a_draft_binds_the_text_shown(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;
    let stale = "00".repeat(32);

    let refused = app
        .post(&format!("/drafts/{draft}/approve"), &cookie, &format!("hash={stale}"))
        .await;
    assert_eq!(refused.status, StatusCode::CONFLICT);
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Draft);

    let hash = hash_hex(&app, firm, draft).await;
    let approved = app
        .post(&format!("/drafts/{draft}/approve"), &cookie, &format!("hash={hash}"))
        .await;
    assert_eq!(approved.status, StatusCode::SEE_OTHER, "{}", approved.body);
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Approved);
}

#[sqlx::test]
async fn a_discarded_draft_leaves_the_list(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;

    app.post(&format!("/drafts/{draft}/discard"), &cookie, "").await;

    assert_eq!(status(&app, firm, draft).await, MessageStatus::Cancelled);
    assert!(!app.get("/drafts", &cookie).await.body.contains("Lito Hardware"));
}

#[sqlx::test]
async fn d4_an_expired_draft_leaves_the_list_and_cant_be_approved(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;
    let hash = hash_hex(&app, firm, draft).await;

    app.clock.advance(Duration::days(5)); // past the end of Thu 5 Nov

    assert!(!app.get("/drafts", &cookie).await.body.contains("Lito Hardware"));
    let late = app
        .post(&format!("/drafts/{draft}/approve"), &cookie, &format!("hash={hash}"))
        .await;
    assert_eq!(late.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(late.body.contains("expired"), "{}", late.body);
    assert_eq!(status(&app, firm, draft).await, MessageStatus::Expired);
}

#[sqlx::test]
async fn d2_editing_a_draft_changes_its_text_and_voids_any_approval(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;
    let hash = hash_hex(&app, firm, draft).await;
    app.post(&format!("/drafts/{draft}/approve"), &cookie, &format!("hash={hash}"))
        .await;
    let form = app.get(&format!("/drafts/{draft}/edit"), &cookie).await;
    assert!(form.body.contains("Documents for October 2026"), "{}", form.body);

    let saved = app
        .post(
            &format!("/drafts/{draft}"),
            &cookie,
            "subject=Kulang+pa+po&body=Hi+Lito%2C+paki-send+po+ang+October+receipts.",
        )
        .await;

    assert_eq!(saved.status, StatusCode::SEE_OTHER, "{}", saved.body);
    let message = Outbox::new(app.pool.clone(), app.clock.clone())
        .get(firm, draft.into())
        .await
        .unwrap();
    assert_eq!(message.status, MessageStatus::Draft, "the earlier approval is void");
    assert_eq!(message.email.subject, "Kulang pa po");
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

/// Two clients' Drafts for one firm, on 1 Nov 2026.
async fn firm_with_two_drafts(app: &TestApp) -> (FirmId, String, Vec<Uuid>) {
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    app.post("/clients", &cookie, EMPLOYER).await;
    app.post(
        "/clients",
        &cookie,
        &EMPLOYER
            .replace("Lito+Hardware", "Ben+Bakery")
            .replace("lito%40", "ben%40"),
    )
    .await;
    app.clock.advance(Duration::days(11));
    Scheduler::new(app.pool.clone(), app.clock.clone())
        .unwrap()
        .run_once()
        .await
        .unwrap();
    let drafts = sqlx::query_scalar("select message_id from scheduled_drafts where firm_id = $1 order by message_id")
        .bind(firm.as_uuid())
        .fetch_all(&app.pool)
        .await
        .unwrap();
    (firm, cookie, drafts)
}

/// The hidden `approve` fields of a review page, as a form body.
fn approval_fields(review: &str) -> String {
    review
        .match_indices(r#"name="approve" value=""#)
        .map(|(at, marker)| {
            let value = &review[at + marker.len()..];
            format!("approve={}", &value[..value.find('"').unwrap()])
        })
        .collect::<Vec<_>>()
        .join("&")
}

#[sqlx::test]
async fn d3_batch_approval_shows_every_draft_in_full_before_approving_any(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, drafts) = firm_with_two_drafts(&app).await;
    let outbox = Outbox::new(app.pool.clone(), app.clock.clone());

    let review = app
        .post(
            "/drafts/review",
            &cookie,
            &format!("ids={}&ids={}", drafts[0], drafts[1]),
        )
        .await;

    assert_eq!(review.status, StatusCode::OK, "{}", review.body);
    for draft in &drafts {
        let body = outbox.get(firm, (*draft).into()).await.unwrap().email.body_text;
        assert!(unescape(&review.body).contains(&body), "every draft is shown in full");
        assert_eq!(
            status(&app, firm, *draft).await,
            MessageStatus::Draft,
            "reviewing approves nothing"
        );
    }
    assert!(review.body.contains("Approve these 2"), "{}", review.body);

    let approved = app
        .post("/drafts/approve", &cookie, &approval_fields(&review.body))
        .await;

    assert_eq!(approved.status, StatusCode::OK, "{}", approved.body);
    assert!(approved.body.contains("2 approved"), "{}", approved.body);
    for draft in &drafts {
        assert_eq!(status(&app, firm, *draft).await, MessageStatus::Approved);
    }
}

#[sqlx::test]
async fn d3_a_draft_changed_after_the_review_is_not_approved(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, cookie, drafts) = firm_with_two_drafts(&app).await;
    let review = app
        .post(
            "/drafts/review",
            &cookie,
            &format!("ids={}&ids={}", drafts[0], drafts[1]),
        )
        .await;
    app.post(
        &format!("/drafts/{}", drafts[0]),
        &cookie,
        "subject=Changed&body=Changed+text.",
    )
    .await;

    let result = app
        .post("/drafts/approve", &cookie, &approval_fields(&review.body))
        .await;

    assert!(
        result.body.contains("1 approved") && result.body.contains("changed since you saw it"),
        "{}",
        result.body
    );
    assert_eq!(status(&app, firm, drafts[0]).await, MessageStatus::Draft);
    assert_eq!(status(&app, firm, drafts[1]).await, MessageStatus::Approved);
}

#[sqlx::test]
async fn a2_batch_pages_refuse_another_firms_drafts(pool: PgPool) {
    let app = TestApp::new(pool);
    let (firm, _, drafts) = firm_with_two_drafts(&app).await;
    let (_, reyes) = app.logged_in_firm("Reyes CPA").await;
    let hash = hash_hex(&app, firm, drafts[0]).await;

    let review = app.post("/drafts/review", &reyes, &format!("ids={}", drafts[0])).await;
    let approve = app
        .post("/drafts/approve", &reyes, &format!("approve={}:{hash}", drafts[0]))
        .await;

    assert_eq!(
        (review.status, approve.status),
        (StatusCode::NOT_FOUND, StatusCode::NOT_FOUND)
    );
    assert_eq!(status(&app, firm, drafts[0]).await, MessageStatus::Draft);
}

#[sqlx::test]
async fn d7_a_reminder_blocked_at_sending_stays_on_the_drafts_page_with_its_reason(pool: PgPool) {
    use std::sync::Arc;

    use kompleto::kompleto::presend::KompletoChecks;

    let app = TestApp::new(pool);
    let (firm, cookie, draft) = firm_with_draft(&app, "Santos Bookkeeping").await;
    let hash = hash_hex(&app, firm, draft).await;
    app.post(&format!("/drafts/{draft}/approve"), &cookie, &format!("hash={hash}"))
        .await;
    let client: Uuid = sqlx::query_scalar("select id from clients")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    // The client's address changes before the message goes out.
    let moved = app
        .post(
            &format!("/clients/{client}"),
            &cookie,
            &EMPLOYER.replace("lito%40example.test", "lito%40new.test"),
        )
        .await;
    assert_eq!(moved.status, StatusCode::SEE_OTHER);
    let checks = Arc::new(KompletoChecks::new(
        app.pool.clone(),
        app.clock.clone(),
        common::web::BASE_URL,
    ));
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

    let page = app.get("/drafts", &cookie).await.body;

    assert!(page.contains("Lito Hardware") && page.contains("blocked"), "{page}");
    assert!(
        unescape(&page).contains("the recipient isn't the client's stored email"),
        "{page}"
    );
}
