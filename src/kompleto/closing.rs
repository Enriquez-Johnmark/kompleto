//! Closing a Firm's account (I2). At once: sending stops, waiting messages
//! are cancelled, sessions end, logins and Upload links stop working. After
//! [`GRACE_DAYS`] all its data is deleted; until then an operator can undo
//! it ([`reopen`]). Cancelled messages stay cancelled.
//!
//! The engine's accounts do the closing, reopening and erasing
//! (`engine::accounts`); Kompleto's rows point at the account, so erasing it
//! removes them too.

use std::sync::Arc;

use sqlx::PgPool;
use time::OffsetDateTime;

use super::firms::FirmId;
use crate::engine::accounts::{Accounts, CloseError};
use crate::engine::clock::Clock;
use crate::engine::files::{FileError, Files};
use crate::engine::outbox::OutboxError;

pub use crate::engine::accounts::GRACE_DAYS;

/// Why an account wasn't closed.
#[derive(Debug, thiserror::Error)]
pub enum ClosingError {
    /// The typed name isn't the Firm's.
    #[error("type your firm's name exactly to close the account")]
    NameMismatch,
    /// Cancelling a message failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl From<CloseError> for ClosingError {
    fn from(err: CloseError) -> Self {
        match err {
            CloseError::NameMismatch => Self::NameMismatch,
            // Only a logged-in member closes, so their Firm exists.
            CloseError::NotFound => Self::Database(sqlx::Error::RowNotFound),
            CloseError::Outbox(err) => Self::Outbox(err),
            CloseError::Database(err) => Self::Database(err),
        }
    }
}

/// Closes `firm`'s account if `typed` is its name (ignoring case and
/// surrounding spaces); returns when everything will be deleted. Closing
/// again keeps the date and only retries cancelling waiting messages.
pub async fn close(
    pool: &PgPool,
    clock: &Arc<dyn Clock>,
    firm: FirmId,
    typed: &str,
    actor: &str,
) -> Result<OffsetDateTime, ClosingError> {
    Ok(Accounts::new(pool.clone(), clock.clone())
        .close(firm, typed, actor)
        .await?)
}

/// Reopens the closed account that logs in as `login_email`, before it's
/// erased; an operator does this when a Firm asks. False if there's no such
/// closed account.
pub async fn reopen(pool: &PgPool, clock: &Arc<dyn Clock>, login_email: &str) -> Result<bool, sqlx::Error> {
    Accounts::new(pool.clone(), clock.clone()).reopen(login_email).await
}

/// Deletes all data and files of every Firm closed [`GRACE_DAYS`] ago;
/// returns how many Firms. A system-wide worker. One Firm that fails is
/// logged and tried again next time; the rest go on.
pub async fn erase_due(pool: &PgPool, clock: &Arc<dyn Clock>, files: &Files) -> Result<usize, FileError> {
    Accounts::new(pool.clone(), clock.clone()).erase_due(files).await
}
