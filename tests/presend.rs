//! Seam 3 (M3): pre-send checks (D7) at approval and at sending. The
//! engine's own rules apply to every message; a product adds its own
//! through `PreSendCheck`.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use common::{RecordingMailer, clock, sample_email};
use kompleto::engine::ids::{MessageId, TenantId};
use kompleto::engine::mail::Email;
use kompleto::engine::outbox::{
    BoxFuture, CheckFailed, MessageStatus, NoExtraChecks, Outbox, OutboxError, PreSendCheck, Sender,
};
use sqlx::PgPool;

/// A product rule that refuses everything, as if the recipient had changed.
struct RefuseAll;

impl PreSendCheck for RefuseAll {
    fn check<'a>(&'a self, _: TenantId, _: MessageId, _: &'a Email) -> BoxFuture<'a, Result<(), CheckFailed>> {
        Box::pin(async { Err(CheckFailed("the recipient isn't the client's stored email".to_owned())) })
    }
}

async fn approve(pool: &PgPool, email: &Email) -> Result<(), OutboxError> {
    let outbox = Outbox::new(pool.clone(), clock());
    let tenant = TenantId::new();
    let id = outbox.create_draft(tenant, email).await.unwrap();
    let seen = outbox.get(tenant, id).await.unwrap().content_hash();
    outbox
        .approve(tenant, id, "ana@santos.test", &seen, &NoExtraChecks)
        .await
}

#[sqlx::test]
async fn d7_approval_is_refused_for_two_links_or_an_overlong_body(pool: PgPool) {
    let two_links = Email {
        body_text: "Upload here: https://getkompleto.test/u/a or https://getkompleto.test/u/b".to_owned(),
        ..sample_email()
    };
    let too_long = Email {
        body_text: "x".repeat(2_001),
        ..sample_email()
    };

    for (case, email) in [("two links", two_links), ("over 2,000 characters", too_long)] {
        let refused = approve(&pool, &email).await;
        assert!(
            matches!(refused, Err(OutboxError::CheckFailed(_))),
            "{case}: {refused:?}"
        );
    }
    approve(
        &pool,
        &Email {
            body_text: "x".repeat(2_000),
            ..sample_email()
        },
    )
    .await
    .unwrap();
}

#[sqlx::test]
async fn d7_a_product_rule_refuses_approval_with_its_reason(pool: PgPool) {
    let outbox = Outbox::new(pool.clone(), clock());
    let tenant = TenantId::new();
    let id = outbox.create_draft(tenant, &sample_email()).await.unwrap();
    let seen = outbox.get(tenant, id).await.unwrap().content_hash();

    let refused = outbox.approve(tenant, id, "ana@santos.test", &seen, &RefuseAll).await;

    assert!(
        matches!(&refused, Err(OutboxError::CheckFailed(reason)) if reason.contains("stored email")),
        "{refused:?}"
    );
    assert_eq!(outbox.get(tenant, id).await.unwrap().status, MessageStatus::Draft);
}

#[sqlx::test]
async fn d7_the_check_runs_again_just_before_sending(pool: PgPool) {
    let clock = clock();
    let outbox = Outbox::new(pool.clone(), clock.clone());
    let tenant = TenantId::new();
    let id = outbox.create_draft(tenant, &sample_email()).await.unwrap();
    let seen = outbox.get(tenant, id).await.unwrap().content_hash();
    outbox
        .approve(tenant, id, "ana@santos.test", &seen, &NoExtraChecks)
        .await
        .unwrap();
    let mailer = RecordingMailer::idempotent();

    // Something changed between approval and sending.
    let report = Sender::new(pool, mailer.clone(), clock, Arc::new(RefuseAll))
        .drain_once(10)
        .await
        .unwrap();

    assert_eq!(report.blocked, 1);
    assert!(mailer.calls().is_empty(), "a message failing its check is never sent");
    let message = outbox.get(tenant, id).await.unwrap();
    assert_eq!(message.status, MessageStatus::Blocked);
    assert!(message.last_error.unwrap().contains("stored email"));
}
