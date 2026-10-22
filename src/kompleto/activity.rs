//! The Firm's activity log (J2): the engine's audit entries for one Firm,
//! in words.

use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::firms::FirmId;
use crate::engine::audit;

/// How far back the Activity page looks.
pub const SHOWN_DAYS: i64 = 90;

/// One entry, as the Firm sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// When it happened.
    pub at: OffsetDateTime,
    /// Who did it: the Firm's login address, or `"system"` and the like.
    pub actor: String,
    /// What happened, e.g. `"outbox.approved"`.
    pub action: String,
    /// The kind of record.
    pub subject_type: String,
    /// The record's ID.
    pub subject_id: Uuid,
}

/// A Firm's entries since `since`, newest first.
pub async fn since(pool: &PgPool, firm: FirmId, since: OffsetDateTime) -> Result<Vec<Entry>, sqlx::Error> {
    let rows = sqlx::query!(
        "select at, actor, action, subject_type, subject_id from audit_events
         where tenant_id = $1 and at >= $2 order by at desc, id desc",
        firm.as_uuid(),
        since,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| Entry {
            at: row.at,
            actor: row.actor,
            action: row.action,
            subject_type: row.subject_type,
            subject_id: row.subject_id,
        })
        .collect())
}

/// `action` in words.
pub fn describe(action: &str) -> &str {
    match action {
        "outbox.draft_created" => "Drafted a message",
        "outbox.edited" => "Edited a message",
        "outbox.approved" => "Approved a message",
        "outbox.refused" => "A message failed the pre-send checks",
        "outbox.cancelled" => "Cancelled a message",
        "outbox.expired" => "A message expired unapproved",
        "outbox.send_started" => "Started sending a message",
        "outbox.sent" => "Sent a message",
        "outbox.send_failed" => "Sending a message failed",
        "outbox.send_interrupted" => "Sending a message was interrupted",
        "upload.received" => "Marked an upload received",
        "file.deleted" => "Deleted an uploaded file",
        other => other,
    }
}

/// `actor` in words: Kompleto itself, or the person.
pub fn who(actor: &str) -> &str {
    if actor == audit::SYSTEM || actor.starts_with("system:") {
        "Kompleto"
    } else {
        actor
    }
}
