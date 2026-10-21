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
