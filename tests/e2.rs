//! Seam 6 (M6): what every Reminder shows its Contact (E2). Whether SPF,
//! DKIM and DMARC pass is checked on the real domain (docs/deploy.md §9).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::engine::ids::MessageId;
use kompleto::engine::mail::Email;
use kompleto::engine::outbox::{CheckFailed, PreSendCheck};
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;
use time::Duration;

#[sqlx::test]
async fn e2_a_reminder_comes_from_the_firm_answers_to_kompleto_and_says_sent_via_kompleto(pool: PgPool) {
    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov: October's first request
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let employer = NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer)
        .await
        .unwrap();
    Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone()))
        .unwrap()
        .run_once()
        .await
        .unwrap();

    let (id, from, reply_to, body): (uuid::Uuid, String, String, String) =
        sqlx::query_as("select id, from_name, reply_to, body_text from outbox_messages")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(from, "Santos Bookkeeping", "the Firm's name as sender");
    assert!(
        reply_to.starts_with("reply+") && reply_to.ends_with("@getkompleto.test"),
        "{reply_to}"
    );
    assert!(body.ends_with("\n--\nSent via Kompleto\n"), "{body}");

    // A Draft edited to drop the footer isn't sent.
    let edited = Email {
        to_address: "nena@example.test".to_owned(),
        from_name: from,
        reply_to: Some(reply_to),
        subject: "Documents".to_owned(),
        body_text: body.replace("\n--\nSent via Kompleto\n", "\n"),
    };
    let checks = common::kompleto_checks(&pool, clock);
    let refused = checks.check(santos, MessageId::from(id), &edited).await;
    assert!(
        matches!(&refused, Err(CheckFailed::Refused(reason)) if reason.contains("Sent via Kompleto")),
        "{refused:?}"
    );
}
