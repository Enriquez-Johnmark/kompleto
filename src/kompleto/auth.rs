//! Login by one-time email link, and sessions (A1).
//!
//! - A link works once, for 15 minutes. Only the SHA-256 hash of its token
//!   is stored.
//! - The email goes through the outbox like every other message, approved
//!   automatically as `system:login`, the one kind of message that may be.
//! - An address with no account gets no email; callers show the same
//!   screen either way.
//! - At most five login emails per Firm per hour.
//! - A session lasts 30 days, or until logout.

use std::sync::Arc;

use sha2::{Digest, Sha256};
use sqlx::PgPool;
use time::Duration;

use super::firms::{FirmError, FirmId, Firms};
use crate::engine::clock::Clock;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};

/// How long a login link works.
pub const LINK_LIFETIME: Duration = Duration::minutes(15);
/// How long a session lasts.
pub const SESSION_LIFETIME: Duration = Duration::days(30);
/// Login emails per Firm per hour.
pub const MAX_LINKS_PER_HOUR: i64 = 5;
/// The approver recorded for login emails.
pub const LOGIN_APPROVER: &str = "system:login";

/// Whether a login email was sent. Callers show the same screen either way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginOutcome {
    /// A link is on its way.
    Sent,
    /// No account, or too many links this hour.
    NotSent,
}

/// Login errors: only infrastructure failures; a bad link is `Ok(None)`.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// The outbox refused the login email.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
    /// Looking up the Firm failed.
    #[error(transparent)]
    Firm(#[from] FirmError),
    /// The system's random number source failed.
    #[error("no randomness available: {0}")]
    Random(String),
}

/// Logins and sessions.
#[derive(Clone)]
pub struct Auth {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    firms: Firms,
    outbox: Outbox,
    base_url: String,
}

impl Auth {
    /// Logins whose links point at `base_url`, e.g. `https://app.getkompleto.com`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, base_url: &str) -> Self {
        Self {
            firms: Firms::new(pool.clone(), clock.clone()),
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// Emails a login link to `email` if it's a Firm's login address.
    pub async fn request_link(&self, email: &str) -> Result<LoginOutcome, AuthError> {
        let Some(firm) = self.firms.find_by_login_email(email).await? else {
            return Ok(LoginOutcome::NotSent);
        };
        let now = self.clock.now();
        let recent = sqlx::query_scalar!(
            r#"select count(*) as "count!" from login_tokens where firm_id = $1 and created_at > $2"#,
            firm.id.as_uuid(),
            now - Duration::hours(1),
        )
        .fetch_one(&self.pool)
        .await?;
        if recent >= MAX_LINKS_PER_HOUR {
            tracing::warn!(firm = %firm.id, "login emails limited this hour");
            return Ok(LoginOutcome::NotSent);
        }
        let token = random_token()?;
        sqlx::query!(
            "insert into login_tokens (token_hash, firm_id, created_at, expires_at) values ($1, $2, $3, $4)",
            &hash(&token)[..],
            firm.id.as_uuid(),
            now,
            now + LINK_LIFETIME,
        )
        .execute(&self.pool)
        .await?;
        let email = Email {
            to_address: firm.login_email.clone(),
            from_name: "Kompleto".to_owned(),
            reply_to: None,
            subject: "Your Kompleto login link".to_owned(),
            body_text: format!(
                "Hi {},\n\nOpen this link to log in to Kompleto:\n\n{}/login/{token}\n\n\
                 It works once, for the next 15 minutes. If you didn't ask for it, ignore this email.\n",
                firm.name, self.base_url
            ),
        };
        let id = self.outbox.create_draft(firm.id, &email).await?;
        let hash = self.outbox.get(firm.id, id).await?.content_hash();
        self.outbox.approve(firm.id, id, LOGIN_APPROVER, &hash).await?;
        Ok(LoginOutcome::Sent)
    }

    /// Uses up a login link and starts a session, returning the session's
    /// secret for the cookie. `None` if the link is unknown, used or expired.
    pub async fn log_in(&self, token: &str) -> Result<Option<String>, AuthError> {
        let now = self.clock.now();
        let firm = sqlx::query_scalar!(
            "update login_tokens set used_at = $2
             where token_hash = $1 and used_at is null and expires_at > $2
             returning firm_id",
            &hash(token)[..],
            now,
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(firm) = firm else {
            return Ok(None);
        };
        let session = random_token()?;
        sqlx::query!(
            "insert into sessions (session_hash, firm_id, created_at, expires_at) values ($1, $2, $3, $4)",
            &hash(&session)[..],
            firm,
            now,
            now + SESSION_LIFETIME,
        )
        .execute(&self.pool)
        .await?;
        Ok(Some(session))
    }

    /// The Firm a live session belongs to.
    pub async fn firm_for_session(&self, session: &str) -> Result<Option<FirmId>, AuthError> {
        let firm = sqlx::query_scalar!(
            "select firm_id from sessions where session_hash = $1 and expires_at > $2",
            &hash(session)[..],
            self.clock.now(),
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(firm.map(FirmId::from))
    }

    /// Ends a session.
    pub async fn log_out(&self, session: &str) -> Result<(), AuthError> {
        sqlx::query!("delete from sessions where session_hash = $1", &hash(session)[..])
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

/// 32 random bytes as 64 lowercase hex digits.
fn random_token() -> Result<String, AuthError> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes).map_err(|err| AuthError::Random(err.to_string()))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

fn hash(secret: &str) -> [u8; 32] {
    Sha256::digest(secret.as_bytes()).into()
}
