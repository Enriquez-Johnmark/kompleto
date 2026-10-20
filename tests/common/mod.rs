//! Shared setup for integration tests: a controllable clock, a fake mailer
//! and a sample message.

#![allow(dead_code)] // Each test binary uses a different subset of these helpers.
#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::sync::{Arc, Mutex};

use kompleto::engine::clock::FixedClock;
use kompleto::engine::mail::{MailError, Mailer, OutgoingEmail, SendReceipt};
use kompleto::engine::outbox::NewMessage;
use time::macros::datetime;

pub fn clock() -> Arc<FixedClock> {
    Arc::new(FixedClock::new(datetime!(2026-10-21 09:00 UTC)))
}

pub fn sample_message() -> NewMessage {
    NewMessage {
        to_address: "nena@example.test".to_owned(),
        from_name: "Santos Bookkeeping".to_owned(),
        reply_to: Some("reply@getkompleto.test".to_owned()),
        subject: "Kulang pa po ang September receipts".to_owned(),
        body_text: "Hi Nena, paki-upload po ang September receipts bago Oct 20.".to_owned(),
    }
}

/// What the fake mailer does on each call, in order; after the script runs
/// out it succeeds.
#[derive(Clone, Debug)]
pub enum Scripted {
    Fail(MailError),
}

/// A mailer that records every send instead of delivering it.
#[derive(Clone, Default)]
pub struct RecordingMailer {
    idempotent: bool,
    sent: Arc<Mutex<Vec<(OutgoingEmail, String)>>>,
    script: Arc<Mutex<Vec<Scripted>>>,
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

    pub fn then(self, step: Scripted) -> Self {
        self.script.lock().unwrap().push(step);
        self
    }

    pub fn calls(&self) -> Vec<(OutgoingEmail, String)> {
        self.sent.lock().unwrap().clone()
    }
}

impl Mailer for RecordingMailer {
    fn blocks_duplicate_keys(&self) -> bool {
        self.idempotent
    }

    async fn send(&self, email: &OutgoingEmail, idempotency_key: &str) -> Result<SendReceipt, MailError> {
        self.sent
            .lock()
            .unwrap()
            .push((email.clone(), idempotency_key.to_owned()));
        let next = {
            let mut script = self.script.lock().unwrap();
            if script.is_empty() {
                None
            } else {
                Some(script.remove(0))
            }
        };
        match next {
            Some(Scripted::Fail(err)) => Err(err),
            None => Ok(SendReceipt {
                provider_message_id: format!("fake-{idempotency_key}"),
            }),
        }
    }
}
