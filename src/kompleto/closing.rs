//! Closing a Firm's account (I2). At once: sending stops, waiting messages
//! are cancelled, sessions end, logins and Upload links stop working. After
//! [`GRACE_DAYS`] all its data is deleted; until then an operator can undo
//! it.

use std::sync::Arc;

use serde_json::json;
use sqlx::{PgConnection, PgPool};
use time::{Duration, OffsetDateTime};

use super::activity::record_by_firm;
use super::firms::FirmId;
use crate::engine::clock::Clock;
use crate::engine::files::{FileError, Files};
use crate::engine::outbox::{Outbox, OutboxError};
use crate::engine::tenants;

/// Days between closing and deleting everything.
pub const GRACE_DAYS: i64 = 7;

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
    let now = clock.now();
    let mut tx = pool.begin().await?;
    let row = sqlx::query!(
        "select name, erase_after from firms where id = $1 for update",
        firm.as_uuid()
    )
    .fetch_one(&mut *tx)
    .await?;
    if row.name.trim().to_lowercase() != typed.trim().to_lowercase() {
        return Err(ClosingError::NameMismatch);
    }
    // Closing again only retries the cancelling below.
    let erase_after = if let Some(erase_after) = row.erase_after {
        tx.rollback().await?;
        erase_after
    } else {
        mark_closed(&mut tx, firm, now).await?;
        tx.commit().await?;
        now + Duration::days(GRACE_DAYS)
    };

    // Closed first, so nothing new is drafted while these are cancelled.
    let waiting = sqlx::query_scalar!(
        "select id from outbox_messages where tenant_id = $1 and status in ('draft', 'approved')",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    let outbox = Outbox::new(pool.clone(), clock.clone());
    for id in waiting {
        match outbox.cancel(firm, id.into(), actor).await {
            // Already being sent: too late to stop.
            Ok(()) | Err(OutboxError::WrongStatus(_)) => {}
            Err(err) => return Err(err.into()),
        }
    }
    Ok(erase_after)
}

/// Deletes all data and files of every Firm closed [`GRACE_DAYS`] ago by
/// `now`; returns how many Firms. A system-wide worker: it names each Firm
/// in every delete. The engine's rows go first and the Firm's row last, so
/// a crash part way leaves the Firm to be found and finished next time.
pub async fn erase_due(pool: &PgPool, files: &Files, now: OffsetDateTime) -> Result<usize, FileError> {
    let due = sqlx::query_scalar!("select id from firms where erase_after <= $1", now)
        .fetch_all(pool)
        .await?;
    for &id in &due {
        let firm = FirmId::from(id);
        tenants::erase(pool, files, firm).await?;
        // Its Clients and everything about them go with it (on delete cascade).
        sqlx::query!("delete from firms where id = $1 and erase_after <= $2", id, now)
            .execute(pool)
            .await?;
        tracing::info!(%firm, "closed account erased");
    }
    Ok(due.len())
}

/// Marks `firm` closed, ends its sessions and logs it.
async fn mark_closed(tx: &mut PgConnection, firm: FirmId, now: OffsetDateTime) -> Result<(), sqlx::Error> {
    let erase_after = now + Duration::days(GRACE_DAYS);
    sqlx::query!(
        "update firms set closed_at = $2, erase_after = $3 where id = $1",
        firm.as_uuid(),
        now,
        erase_after,
    )
    .execute(&mut *tx)
    .await?;
    sqlx::query!("delete from sessions where firm_id = $1", firm.as_uuid())
        .execute(&mut *tx)
        .await?;
    sqlx::query!("delete from login_tokens where firm_id = $1", firm.as_uuid())
        .execute(&mut *tx)
        .await?;
    record_by_firm(tx, firm, now, "firm.closed", ("firm", firm.as_uuid()), json!({})).await?;
    Ok(())
}
