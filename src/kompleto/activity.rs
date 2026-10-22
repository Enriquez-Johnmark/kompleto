//! The Firm's activity log (J2): the engine's audit entries for one Firm,
//! in words, and how Kompleto records the Firm's own changes in them.

use sqlx::{PgConnection, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use super::firms::FirmId;
use crate::engine::audit::{self, AuditEvent};

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

/// Records a change the Firm made, in the caller's transaction, naming the
/// Firm's login address as the actor (a Firm has one login).
pub(crate) async fn record_by_firm(
    tx: &mut PgConnection,
    firm: FirmId,
    at: OffsetDateTime,
    action: &str,
    subject: (&str, Uuid),
    details: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let actor = sqlx::query_scalar!("select login_email from firms where id = $1", firm.as_uuid())
        .fetch_one(&mut *tx)
        .await?;
    let event = AuditEvent {
        tenant: firm,
        at,
        actor: &actor,
        action,
        subject_type: subject.0,
        subject_id: subject.1,
        details,
    };
    audit::record(&mut *tx, &event).await
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
        "client.created" => "Added a client",
        "client.updated" => "Changed a client's details",
        "client.schedule_changed" => "Changed a client's reminder schedule",
        "client.due_date_set" => "Changed a client's due date",
        "checklist.item_removed" => "Removed a checklist item",
        "checklist.item_restored" => "Restored a checklist item",
        "checklist.item_added" => "Added a checklist item",
        "checklist.item_deleted" => "Deleted a checklist item",
        "payor.added" => "Added a Payor",
        "payor.removed" => "Removed a Payor",
        "reply.received" => "A reply arrived",
        "firm.logged_in" => "Logged in",
        "firm.exported" => "Downloaded all data",
        "firm.cut_off_changed" => "Changed the default cut-off",
        "firm.closed" => "Closed the account",
        "firm.reopened" => "Reopened the account",
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
