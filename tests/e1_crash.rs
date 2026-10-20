//! Seam 2: E1 as the spec words it. The real `kompleto` binary is killed
//! (`abort()`) after the provider accepted an email but before the send was
//! recorded, then restarted. Needs `--features failpoints`.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::collections::HashSet;
use std::process::{Command, ExitStatus};

use common::{clock, sample_message};
use kompleto::engine::ids::{MessageId, TenantId};
use kompleto::engine::outbox::{MessageStatus, Outbox};
use serde_json::json;
use sqlx::PgPool;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const FAILPOINT: &str = "after_send_before_record";

#[sqlx::test]
async fn e1_crash_mid_send_with_resend_retries_under_one_key_so_one_email_goes_out(pool: PgPool) {
    let provider = fake_provider("/emails", json!({ "id": "re_1" })).await;
    let (tenant, id) = approved(&pool).await;

    let crashed = drain(&pool, &resend_env(&provider), Some(FAILPOINT)).await;
    assert!(!crashed.success(), "the failpoint should have killed the process");
    drain(&pool, &resend_env(&provider), None).await;

    let keys: HashSet<String> = provider
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter_map(|r| {
            r.headers
                .get("Idempotency-Key")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        })
        .collect();
    assert_eq!(
        keys,
        HashSet::from([id.to_string()]),
        "every attempt must carry the message's one key"
    );
    assert_eq!(status(&pool, tenant, id).await, MessageStatus::Sent);
}

#[sqlx::test]
async fn e1_crash_mid_send_without_duplicate_blocking_never_resends(pool: PgPool) {
    let provider = fake_provider("/api/v1/send", json!({ "ID": "mp_1" })).await;
    let (tenant, id) = approved(&pool).await;

    drain(&pool, &mailpit_env(&provider), Some(FAILPOINT)).await;
    drain(&pool, &mailpit_env(&provider), None).await;

    assert_eq!(
        provider.received_requests().await.unwrap().len(),
        1,
        "the email was sent twice"
    );
    assert_eq!(status(&pool, tenant, id).await, MessageStatus::Unconfirmed);
}

async fn fake_provider(route: &str, body: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(route))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    server
}

async fn approved(pool: &PgPool) -> (TenantId, MessageId) {
    let outbox = Outbox::new(pool.clone(), clock());
    let tenant = TenantId::new();
    let id = outbox.create_draft(tenant, &sample_message()).await.unwrap();
    let seen = outbox.get(tenant, id).await.unwrap().content_hash();
    outbox
        .approve(tenant, id, "bookkeeper@santos.test", &seen)
        .await
        .unwrap();
    (tenant, id)
}

async fn status(pool: &PgPool, tenant: TenantId, id: MessageId) -> MessageStatus {
    Outbox::new(pool.clone(), clock()).get(tenant, id).await.unwrap().status
}

fn resend_env(provider: &MockServer) -> Vec<(&'static str, String)> {
    vec![
        ("MAIL_PROVIDER", "resend".to_owned()),
        ("RESEND_BASE_URL", provider.uri()),
        ("RESEND_API_KEY", "re_test".to_owned()),
    ]
}

fn mailpit_env(provider: &MockServer) -> Vec<(&'static str, String)> {
    vec![("MAIL_PROVIDER", "mailpit".to_owned()), ("MAILPIT_URL", provider.uri())]
}

/// Runs `kompleto outbox-drain` against this test's own database.
async fn drain(pool: &PgPool, env: &[(&'static str, String)], failpoint: Option<&str>) -> ExitStatus {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kompleto"));
    cmd.arg("outbox-drain")
        .env("DATABASE_URL", database_url(pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@getkompleto.test")
        .env("RUST_LOG", "warn")
        .envs(env.iter().map(|(k, v)| (*k, v.as_str())));
    match failpoint {
        Some(name) => cmd.env("KOMPLETO_FAILPOINT", name),
        None => cmd.env_remove("KOMPLETO_FAILPOINT"),
    };
    tokio::task::spawn_blocking(move || cmd.status().unwrap())
        .await
        .unwrap()
}

/// `#[sqlx::test]` gives each test a fresh database on the server named by
/// `DATABASE_URL`; point the child process at that same database.
async fn database_url(pool: &PgPool) -> String {
    let name: String = sqlx::query_scalar("select current_database()")
        .fetch_one(pool)
        .await
        .unwrap();
    let base = std::env::var("DATABASE_URL").unwrap();
    let (server, _) = base.rsplit_once('/').unwrap();
    format!("{server}/{name}")
}
