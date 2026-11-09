//! Login by one-time email link, and sessions (A1).
//!
//! - A link works once, for 15 minutes. Only the SHA-256 hash of its token
//!   is stored.
//! - The email goes through the outbox like every other message, approved
//!   automatically as `system:login`, the one kind of message that may be.
//! - An address with no account gets no email; callers show the same
//!   screen either way.
//! - Each person logs in with their own address (M9): the owner's or a
//!   staff member's. At most five login emails per address, and 30 per
//!   Firm, an hour.
//! - A session lasts 30 days, or until logout.

use std::sync::Arc;

use sqlx::PgPool;

use super::firms::{FirmError, FirmId};
use super::members::{MemberId, Role};
use super::presend::KompletoChecks;
use crate::engine::accounts::{LoginError, LoginLink, Logins};
use crate::engine::clock::Clock;
use crate::engine::mail::Email;
use crate::engine::outbox::OutboxError;

pub use crate::engine::accounts::{
    LINK_LIFETIME, LOGIN_APPROVER, LoginOutcome, MAX_ACCOUNT_LINKS_PER_HOUR as MAX_FIRM_LINKS_PER_HOUR,
    MAX_LINKS_PER_HOUR, SESSION_LIFETIME,
};

/// Why a login link is emailed.
#[derive(Debug, Clone, Copy)]
enum LinkKind<'a> {
    /// The person asked for one.
    Login,
    /// The owner added them as staff.
    Invitation { invited_by: &'a str },
    /// The operator approved their account request.
    Welcome,
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

impl From<LoginError> for AuthError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::Database(err) => Self::Database(err),
            LoginError::Outbox(err) => Self::Outbox(err),
            LoginError::Random(reason) => Self::Random(reason),
        }
    }
}

/// Logins and sessions: the engine's (`engine::accounts`), with Kompleto's
/// login, invitation and welcome emails.
#[derive(Clone)]
pub struct Auth {
    logins: Logins,
    checks: KompletoChecks,
    base_url: String,
}

impl Auth {
    /// Logins whose links point at `base_url`, e.g. `https://app.getkompleto.com`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, base_url: &str, checks: KompletoChecks) -> Self {
        Self {
            logins: Logins::new(pool, clock),
            checks,
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// Emails a login link to `email` if it's the address of a member of an
    /// open Firm.
    pub async fn request_link(&self, email: &str) -> Result<LoginOutcome, AuthError> {
        self.send_link(email, LinkKind::Login).await
    }

    /// Emails a new staff member at `email` a login link saying that
    /// `invited_by` added them (M9).
    pub async fn invite(&self, email: &str, invited_by: &str) -> Result<LoginOutcome, AuthError> {
        self.send_link(email, LinkKind::Invitation { invited_by }).await
    }

    /// Emails the owner of a newly approved Firm (M10) a login link that
    /// welcomes them.
    pub async fn welcome(&self, email: &str) -> Result<LoginOutcome, AuthError> {
        self.send_link(email, LinkKind::Welcome).await
    }

    async fn send_link(&self, email: &str, kind: LinkKind<'_>) -> Result<LoginOutcome, AuthError> {
        let compose = |link: &LoginLink<'_>| {
            let firm_name = link.account_name;
            let (subject, opening) = match kind {
                LinkKind::Login => (
                    "Your Kompleto login link".to_owned(),
                    format!("Hi,\n\nOpen this link to log in to {firm_name} on Kompleto:"),
                ),
                LinkKind::Invitation { invited_by: owner } => (
                    format!("You've been added to {firm_name} on Kompleto"),
                    format!(
                        "Hi,\n\n{owner} added you to {firm_name} on Kompleto, which chases clients for their \
                         BIR documents. Open this link to log in:"
                    ),
                ),
                LinkKind::Welcome => (
                    "Your Kompleto account is ready".to_owned(),
                    format!(
                        "Hi,\n\nWelcome to Kompleto! The account for {firm_name} is ready. Open this link to log in:"
                    ),
                ),
            };
            let link = format!("{}/login/{}", self.base_url, link.token);
            Email {
                // The engine sends it to the member's own address.
                to_address: String::new(),
                from_name: "Kompleto".to_owned(),
                reply_to: None,
                subject,
                body_text: format!(
                    "{opening}\n\n{link}\n\nIt works once, for the next 15 minutes. After that, ask for a new one \
                     on Kompleto's login page. If you didn't expect this email, ignore it.\n"
                ),
            }
        };
        Ok(self.logins.send_link(email, compose, &self.checks).await?)
    }

    /// Uses up a login link and starts a session, returning the session's
    /// secret for the cookie. `None` if the link is unknown, used or expired.
    pub async fn log_in(&self, token: &str) -> Result<Option<String>, AuthError> {
        Ok(self.logins.log_in(token).await?)
    }

    /// Who a live session belongs to.
    pub async fn session(&self, session: &str) -> Result<Option<Session>, AuthError> {
        Ok(self.logins.session(session).await?.map(|session| Session {
            firm: session.tenant,
            member: session.member,
            email: session.email,
            role: session.role,
        }))
    }

    /// The Firm a live session belongs to.
    pub async fn firm_for_session(&self, session: &str) -> Result<Option<FirmId>, AuthError> {
        Ok(self.session(session).await?.map(|session| session.firm))
    }

    /// Ends a session.
    pub async fn log_out(&self, session: &str) -> Result<(), AuthError> {
        Ok(self.logins.log_out(session).await?)
    }
}
