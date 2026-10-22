//! Login by one-time email link, and sessions (A1).
//!
//! - A link works once, for 15 minutes. Only the SHA-256 hash of its token
//!   is stored.
//! - The email goes through the outbox like every other message, approved
//!   automatically as `system:login`, the one kind of message that may be.
//! - An address with no account gets no email; callers show the same
//!   screen either way.
//! - Each person logs in with their own address (M9): the owner's or a
//!   staff member's. At most five login emails per person per hour.
//! - A session lasts 30 days, or until logout.

use std::sync::Arc;

use sha2::{Digest, Sha256};
use sqlx::PgPool;
use time::Duration;

use serde_json::json;

use super::activity::record_by;
use super::firms::{FirmError, FirmId};
use super::members::{MemberId, Role};
use super::presend::KompletoChecks;
use crate::engine::clock::Clock;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};

/// How long a login link works.
pub const LINK_LIFETIME: Duration = Duration::minutes(15);
/// How long a session lasts.
pub const SESSION_LIFETIME: Duration = Duration::days(30);
/// Login emails per person per hour.
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

/// Who a live session belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    /// Their Firm.
    pub firm: FirmId,
    /// The person.
    pub member: MemberId,
    /// Their address, which names them in the activity log.
    pub email: String,
    /// What they may do.
    pub role: Role,
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
    outbox: Outbox,
    checks: KompletoChecks,
    base_url: String,
}

impl Auth {
    /// Logins whose links point at `base_url`, e.g. `https://app.getkompleto.com`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, base_url: &str, checks: KompletoChecks) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            checks,
            pool,
            clock,
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// Emails a login link to `email` if it's the address of a member of an
    /// open Firm.
    pub async fn request_link(&self, email: &str) -> Result<LoginOutcome, AuthError> {
        self.send_link(email, None).await
    }

    /// Emails a new staff member at `email` a login link saying that
    /// `invited_by` added them (M9).
    pub async fn invite(&self, email: &str, invited_by: &str) -> Result<LoginOutcome, AuthError> {
        self.send_link(email, Some(invited_by)).await
    }

    async fn send_link(&self, email: &str, invited_by: Option<&str>) -> Result<LoginOutcome, AuthError> {
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        // Locking the member serialises their requests, so parallel ones
        // can't all pass the hourly limit before any of them is counted.
        let Some(member) = sqlx::query!(
            "select m.id, m.firm_id, m.email, f.name as firm_name
             from firm_members m join firms f on f.id = m.firm_id and f.closed_at is null
             where lower(m.email) = lower($1)
             for update of m",
            email.trim(),
        )
        .fetch_optional(&mut *tx)
        .await?
        else {
            return Ok(LoginOutcome::NotSent);
        };
        let firm = FirmId::from(member.firm_id);
        let recent = sqlx::query_scalar!(
            r#"select count(*) as "count!" from login_tokens where member_id = $1 and created_at > $2"#,
            member.id,
            now - Duration::hours(1),
        )
        .fetch_one(&mut *tx)
        .await?;
        if recent >= MAX_LINKS_PER_HOUR {
            tracing::warn!(%firm, "login emails limited this hour");
            return Ok(LoginOutcome::NotSent);
        }
        let token = random_token()?;
        sqlx::query!(
            "insert into login_tokens (token_hash, firm_id, member_id, created_at, expires_at)
             values ($1, $2, $3, $4, $5)",
            &hash(&token)[..],
            firm.as_uuid(),
            member.id,
            now,
            now + LINK_LIFETIME,
        )
        .execute(&mut *tx)
        .await?;
        let link = format!("{}/login/{token}", self.base_url);
        let (subject, opening) = match invited_by {
            None => (
                "Your Kompleto login link".to_owned(),
                format!("Hi {},\n\nOpen this link to log in to Kompleto:", member.firm_name),
            ),
            Some(owner) => (
                format!("You've been added to {} on Kompleto", member.firm_name),
                format!(
                    "Hi,\n\n{owner} added you to {} on Kompleto, which chases clients for their \
                     BIR documents. Open this link to log in:",
                    member.firm_name
                ),
            ),
        };
        let email = Email {
            to_address: member.email,
            from_name: "Kompleto".to_owned(),
            reply_to: None,
            subject,
            body_text: format!(
                "{opening}\n\n{link}\n\nIt works once, for the next 15 minutes. After that, ask for a new one \
                 on Kompleto's login page. If you didn't expect this email, ignore it.\n"
            ),
        };
        let id = self.outbox.create_draft_in(&mut tx, firm, &email, None).await?;
        tx.commit().await?;
        let hash = self.outbox.get(firm, id).await?.content_hash();
        self.outbox
            .approve(firm, id, LOGIN_APPROVER, &hash, &self.checks)
            .await?;
        Ok(LoginOutcome::Sent)
    }

    /// Uses up a login link and starts a session, returning the session's
    /// secret for the cookie. `None` if the link is unknown, used or expired.
    pub async fn log_in(&self, token: &str) -> Result<Option<String>, AuthError> {
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let used = sqlx::query!(
            "update login_tokens t set used_at = $2
             from firms f, firm_members m
             where t.token_hash = $1 and t.used_at is null and t.expires_at > $2
               and f.id = t.firm_id and f.closed_at is null
               and m.id = t.member_id and m.firm_id = t.firm_id
             returning t.firm_id, t.member_id, m.email",
            &hash(token)[..],
            now,
        )
        .fetch_optional(&mut *tx)
        .await?;
        let Some(used) = used else {
            return Ok(None);
        };
        let session = random_token()?;
        sqlx::query!(
            "insert into sessions (session_hash, firm_id, member_id, created_at, expires_at)
             values ($1, $2, $3, $4, $5)",
            &hash(&session)[..],
            used.firm_id,
            used.member_id,
            now,
            now + SESSION_LIFETIME,
        )
        .execute(&mut *tx)
        .await?;
        let firm = FirmId::from(used.firm_id);
        record_by(
            &mut tx,
            firm,
            &used.email,
            now,
            "firm.logged_in",
            ("firm", firm.as_uuid()),
            json!({}),
        )
        .await?;
        tx.commit().await?;
        Ok(Some(session))
    }

    /// Who a live session belongs to.
    pub async fn session(&self, session: &str) -> Result<Option<Session>, AuthError> {
        let row = sqlx::query!(
            "select s.firm_id, s.member_id, m.email, m.role
             from sessions s
             join firms f on f.id = s.firm_id and f.closed_at is null
             join firm_members m on m.id = s.member_id and m.firm_id = s.firm_id
             where s.session_hash = $1 and s.expires_at > $2",
            &hash(session)[..],
            self.clock.now(),
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|row| Session {
            firm: row.firm_id.into(),
            member: row.member_id.into(),
            email: row.email,
            role: Role::parse(&row.role),
        }))
    }

    /// The Firm a live session belongs to.
    pub async fn firm_for_session(&self, session: &str) -> Result<Option<FirmId>, AuthError> {
        Ok(self.session(session).await?.map(|session| session.firm))
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
