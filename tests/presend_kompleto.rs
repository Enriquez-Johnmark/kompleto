//! Seam 3 (M3): Kompleto's own pre-send rules (D7) against Postgres.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::engine::ids::MessageId;
use kompleto::engine::mail::Email;
use kompleto::engine::outbox::{Outbox, OutboxError};
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;
use time::Duration;

const BASE_URL: &str = "http://kompleto.test";

/// A firm with one client and that client's first scheduled Draft.
async fn reminder(pool: &PgPool) -> (FirmId, MessageId, Outbox) {
    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov 2026: October's first request is due
    let santos = firm(pool, "Santos Bookkeeping").await;
    let client = NewClient {
        forms: vec![kompleto::kompleto::calendar::BirForm::F1601C],
        ..nena()
    };
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &client)
        .await
        .unwrap();
    Scheduler::new(pool.clone(), clock.clone(), common::upload_links(pool, clock.clone()))
        .unwrap()
        .run_once()
        .await
        .unwrap();
    let id: uuid::Uuid = sqlx::query_scalar("select message_id from scheduled_drafts")
        .fetch_one(pool)
        .await
        .unwrap();
    (santos, id.into(), Outbox::new(pool.clone(), clock))
}

async fn approve(pool: &PgPool, outbox: &Outbox, firm: FirmId, id: MessageId) -> Result<(), OutboxError> {
    let seen = outbox.get(firm, id).await.unwrap().content_hash();
    outbox
        .approve(
            firm,
            id,
            "ana@santos.test",
            &seen,
            &common::kompleto_checks(pool, clock()),
        )
        .await
}

fn reason(result: Result<(), OutboxError>) -> String {
    match result {
        Err(OutboxError::CheckFailed(reason)) => reason,
        other => panic!("expected a failed check, got {other:?}"),
    }
}

#[sqlx::test]
async fn d7_an_unchanged_reminder_to_the_clients_stored_email_passes(pool: PgPool) {
    let (firm, id, outbox) = reminder(&pool).await;

    approve(&pool, &outbox, firm, id).await.unwrap();
}

#[sqlx::test]
async fn d7_a_reminder_to_anyone_but_the_clients_stored_email_is_refused(pool: PgPool) {
    let (firm, id, outbox) = reminder(&pool).await;
    let clients = Clients::new(pool.clone(), clock());
    let client = &clients.list(firm).await.unwrap()[0];
    let moved = NewClient {
        email: "nena.new@example.test".to_owned(),
        ..client.details.clone()
    };
    clients.update(firm, client.id, &moved).await.unwrap();

    assert!(reason(approve(&pool, &outbox, firm, id).await).contains("isn't the client's stored email"));
}

#[sqlx::test]
async fn d7_a_reminder_with_a_link_that_isnt_the_clients_upload_link_is_refused(pool: PgPool) {
    let (firm, id, outbox) = reminder(&pool).await;
    let mut email = outbox.get(firm, id).await.unwrap().email;
    let own = upload_link(&email.body_text);
    email.body_text = email.body_text.replace(&own, "https://drive.example/folder");
    outbox.edit(firm, id, &email, "ana@santos.test").await.unwrap();

    assert!(reason(approve(&pool, &outbox, firm, id).await).contains("isn't the client's own upload link"));
}

#[sqlx::test]
async fn d7_a_reminder_without_the_clients_upload_link_is_refused(pool: PgPool) {
    let (firm, id, outbox) = reminder(&pool).await;
    let mut email = outbox.get(firm, id).await.unwrap().email;
    let own = upload_link(&email.body_text);
    email.body_text = email.body_text.replace(&own, "(link to follow)");
    outbox.edit(firm, id, &email, "ana@santos.test").await.unwrap();

    assert!(reason(approve(&pool, &outbox, firm, id).await).contains("doesn't have the client's upload link"));
}

fn upload_link(body: &str) -> String {
    body.split_whitespace()
        .find(|word| word.starts_with("http://kompleto.test/u/"))
        .unwrap()
        .to_owned()
}

#[sqlx::test]
async fn d7_a_login_email_must_go_to_the_firms_login_address_with_one_login_link(pool: PgPool) {
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let outbox = Outbox::new(pool.clone(), clock());
    let login = |to: &str, link: &str| Email {
        to_address: to.to_owned(),
        from_name: "Kompleto".to_owned(),
        reply_to: None,
        subject: "Your Kompleto login link".to_owned(),
        body_text: format!("Open this link to log in:\n\n{link}\n"),
    };
    let check = |email: Email| {
        let (pool, outbox) = (pool.clone(), outbox.clone());
        async move {
            let id = outbox.create_draft(santos, &email).await.unwrap();
            approve(&pool, &outbox, santos, id).await
        }
    };

    check(login(
        "santos.bookkeeping@firm.test",
        &format!("{BASE_URL}/login/abc123"),
    ))
    .await
    .unwrap();
    let elsewhere = check(login("someone@else.test", &format!("{BASE_URL}/login/abc123"))).await;
    let wrong_link = check(login(
        "santos.bookkeeping@firm.test",
        "https://evil.example/login/abc123",
    ))
    .await;

    assert!(
        reason(elsewhere).contains("Kompleto doesn't recognise"),
        "a stray message is never sent"
    );
    assert!(reason(wrong_link).contains("Kompleto doesn't recognise"));
    check(login(
        "santos.bookkeeping@firm.test",
        &format!("{BASE_URL}/replies/01a15f0b-6a2c-7529-908c-4bf9cf7406a0"),
    ))
    .await
    .unwrap();
}
