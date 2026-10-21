//! Shared setup for integration tests: a controllable clock, a fake mailer
//! and a sample message.

#![allow(dead_code)] // Each test binary uses a different subset of these helpers.
#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

pub mod web;

use std::sync::{Arc, Mutex};

use kompleto::engine::clock::FixedClock;
use kompleto::engine::mail::{Email, MailError, Mailer, SendReceipt};
use time::macros::datetime;

pub fn clock() -> Arc<FixedClock> {
    Arc::new(FixedClock::new(datetime!(2026-10-21 09:00 UTC)))
}

pub fn sample_email() -> Email {
    Email {
        to_address: "nena@example.test".to_owned(),
        from_name: "Santos Bookkeeping".to_owned(),
        reply_to: Some("reply@getkompleto.test".to_owned()),
        subject: "Kulang pa po ang September receipts".to_owned(),
        body_text: "Hi Nena, paki-upload po ang September receipts bago Oct 20.".to_owned(),
    }
}

/// A mailer that records every send instead of delivering it.
#[derive(Clone, Default)]
pub struct RecordingMailer {
    idempotent: bool,
    sent: Arc<Mutex<Vec<(Email, String)>>>,
    failures: Arc<Mutex<Vec<MailError>>>,
}

impl RecordingMailer {
    /// A mailer whose provider blocks duplicate sends with the same key, like Resend.
    pub fn idempotent() -> Self {
        Self {
            idempotent: true,
            ..Self::default()
        }
    }

    /// A mailer whose provider can't block duplicates, like Mailpit or SES.
    pub fn non_idempotent() -> Self {
        Self::default()
    }

    /// Fails the next call with `err`; calls after the queued failures succeed.
    pub fn then_fail(self, err: MailError) -> Self {
        self.failures.lock().unwrap().push(err);
        self
    }

    pub fn calls(&self) -> Vec<(Email, String)> {
        self.sent.lock().unwrap().clone()
    }
}

impl Mailer for RecordingMailer {
    fn blocks_duplicate_keys(&self) -> bool {
        self.idempotent
    }

    async fn send(&self, email: &Email, idempotency_key: &str) -> Result<SendReceipt, MailError> {
        self.sent
            .lock()
            .unwrap()
            .push((email.clone(), idempotency_key.to_owned()));
        let mut failures = self.failures.lock().unwrap();
        if !failures.is_empty() {
            return Err(failures.remove(0));
        }
        Ok(SendReceipt {
            provider_message_id: format!("fake-{idempotency_key}"),
        })
    }
}

/// Creates a Firm named `name` with a login email derived from it.
pub async fn firm(pool: &sqlx::PgPool, name: &str) -> kompleto::kompleto::firms::FirmId {
    let email = format!("{}@firm.test", name.to_lowercase().replace(' ', "."));
    kompleto::kompleto::firms::Firms::new(pool.clone(), clock())
        .create(name, &email)
        .await
        .unwrap()
}

/// The file master key tests use.
pub fn master_key() -> kompleto::engine::files::MasterKey {
    kompleto::engine::files::MasterKey::from_hex(&"ab".repeat(32)).unwrap()
}

/// Upload links signed with the test master key, for the test base URL.
pub fn upload_links(pool: &sqlx::PgPool, clock: Arc<FixedClock>) -> kompleto::kompleto::upload_links::UploadLinks {
    kompleto::kompleto::upload_links::UploadLinks::new(pool.clone(), clock, &master_key(), "http://kompleto.test")
}

/// Kompleto's pre-send rules, for the test base URL.
pub fn kompleto_checks(pool: &sqlx::PgPool, clock: Arc<FixedClock>) -> kompleto::kompleto::presend::KompletoChecks {
    kompleto::kompleto::presend::KompletoChecks::new(
        pool.clone(),
        clock.clone(),
        "http://kompleto.test",
        upload_links(pool, clock),
    )
}

/// Encrypted files kept in a fresh folder under the test target directory.
pub fn files(pool: &sqlx::PgPool, clock: Arc<FixedClock>) -> kompleto::engine::files::Files {
    use kompleto::engine::files::{FileCipher, Files, LocalDisk};
    let root = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(uuid::Uuid::now_v7().to_string());
    Files::new(
        pool.clone(),
        clock,
        Arc::new(LocalDisk::new(root)),
        Arc::new(FileCipher::new(&master_key())),
    )
}

/// A sample Client: a non-VAT sole proprietor reminded in Taglish.
pub fn nena() -> kompleto::kompleto::clients::NewClient {
    use kompleto::kompleto::calendar::BirForm;
    kompleto::kompleto::clients::NewClient {
        business_name: "Nena's Sari-Sari Store".to_owned(),
        contact_first_name: "Nena".to_owned(),
        email: "nena@example.test".to_owned(),
        language: kompleto::kompleto::clients::Language::Taglish,
        forms: vec![BirForm::F2551Q, BirForm::F1701Q],
    }
}

/// `#[sqlx::test]` gives each test a fresh database on the server named by
/// `DATABASE_URL`; this is that database's URL, for a child process.
pub async fn database_url(pool: &sqlx::PgPool) -> String {
    let name: String = sqlx::query_scalar("select current_database()")
        .fetch_one(pool)
        .await
        .unwrap();
    let base = std::env::var("DATABASE_URL").unwrap();
    let (server, _) = base.rsplit_once('/').unwrap();
    format!("{server}/{name}")
}
