//! The outbox: every outgoing message moves through one state machine, and
//! only an approved message whose content still matches what was approved
//! can be sent (D2). Sending lives in [`Sender`].
//!
//! ```text
//! draft ──approve──► approved ──► sending ──► sent
//!   │                   │            └──► failed (retry with the SAME key) / unconfirmed
//!   ├─ edit → draft again (old approval void)
//!   └─ deadline passes → expired          cancelled ◄── client replied / everything uploaded
//! ```

mod sender;

use std::sync::Arc;

use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use time::OffsetDateTime;

use super::audit::{self, AuditEvent};
use super::clock::Clock;
use super::ids::{MessageId, TenantId};

pub use sender::{DrainReport, Sender};

/// A message to put in the outbox.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewMessage {
    /// Recipient address.
    pub to_address: String,
    /// Sender display name, e.g. the Firm's name.
    pub from_name: String,
    /// Where replies go.
    pub reply_to: Option<String>,
    /// Subject line.
    pub subject: String,
    /// Plain-text body.
    pub body_text: String,
}

/// Where a message is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    /// Waiting for approval.
    Draft,
    /// Approved; will be sent.
    Approved,
    /// Handed to the provider; outcome not yet recorded.
    Sending,
    /// The provider accepted it.
    Sent,
    /// The last attempt failed; see `retry_at` for whether it will retry.
    Failed,
    /// Refused at send time because its content no longer matched the approval.
    Blocked,
    /// May or may not have been sent; a person must check before resending.
    Unconfirmed,
    /// Its deadline passed before it was approved or sent.
    Expired,
    /// No longer needed (e.g. the client replied).
    Cancelled,
}

impl MessageStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Approved => "approved",
            Self::Sending => "sending",
            Self::Sent => "sent",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
            Self::Unconfirmed => "unconfirmed",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
        }
    }

    fn parse(s: &str) -> Result<Self, OutboxError> {
        Ok(match s {
            "draft" => Self::Draft,
            "approved" => Self::Approved,
            "sending" => Self::Sending,
            "sent" => Self::Sent,
            "failed" => Self::Failed,
            "blocked" => Self::Blocked,
            "unconfirmed" => Self::Unconfirmed,
            "expired" => Self::Expired,
            "cancelled" => Self::Cancelled,
            other => return Err(OutboxError::CorruptRow(format!("unknown status {other:?}"))),
        })
    }
}

/// A fingerprint of a message's exact content. An approval binds to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentHash([u8; 32]);

impl ContentHash {
    fn of(m: &NewMessage) -> Self {
        let mut hasher = Sha256::new();
        // Length-prefix every field so no two different messages can share
        // the same byte stream.
        for field in [
            m.to_address.as_str(),
            m.from_name.as_str(),
            m.reply_to.as_deref().unwrap_or(""),
            m.subject.as_str(),
            m.body_text.as_str(),
        ] {
            hasher.update((field.len() as u64).to_be_bytes());
            hasher.update(field.as_bytes());
        }
        hasher.update([u8::from(m.reply_to.is_some())]);
        Self(hasher.finalize().into())
    }

    fn from_db(bytes: &[u8]) -> Result<Self, OutboxError> {
        <[u8; 32]>::try_from(bytes)
            .map(Self)
            .map_err(|_| OutboxError::CorruptRow("approved_hash is not 32 bytes".to_owned()))
    }

    /// The hash as bytes, e.g. to put in an approval form.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// A message as stored.
#[derive(Debug, Clone)]
pub struct Message {
    /// Its ID.
    pub id: MessageId,
    /// Whose it is.
    pub tenant: TenantId,
    /// Where it is in its life.
    pub status: MessageStatus,
    /// Its content.
    pub content: NewMessage,
    /// When it was sent, if it was.
    pub sent_at: Option<OffsetDateTime>,
    /// The last error, if any.
    pub last_error: Option<String>,
}

impl Message {
    /// The fingerprint of the content as it is now. Show this alongside the
    /// content when asking for approval, and pass it back to [`Outbox::approve`].
    pub fn content_hash(&self) -> ContentHash {
        ContentHash::of(&self.content)
    }
}

/// Outbox errors.
#[derive(Debug, thiserror::Error)]
pub enum OutboxError {
    /// No such message for this tenant. Another tenant's message is reported
    /// the same way (A2).
    #[error("message not found")]
    NotFound,
    /// The message content isn't acceptable.
    #[error("invalid message: {0}")]
    InvalidMessage(String),
    /// The content changed after the approver looked at it.
    #[error("the message changed since it was shown; review it again")]
    StaleApproval,
    /// The message isn't in a state that allows this.
    #[error("not allowed while the message is {0:?}")]
    WrongStatus(MessageStatus),
    /// A stored row doesn't make sense.
    #[error("corrupt outbox row: {0}")]
    CorruptRow(String),
    /// Database failure.
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

/// Creating, editing and approving outbox messages.
#[derive(Clone)]
pub struct Outbox {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Outbox {
    /// An outbox on `pool`, reading time from `clock`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Stores `message` as a draft awaiting approval.
    pub async fn create_draft(&self, tenant: TenantId, message: &NewMessage) -> Result<MessageId, OutboxError> {
        validate(message)?;
        let id = MessageId::new();
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "insert into outbox_messages
                (id, tenant_id, status, to_address, from_name, reply_to, subject, body_text, created_at, updated_at)
             values ($1, $2, 'draft', $3, $4, $5, $6, $7, $8, $8)",
            id.as_uuid(),
            tenant.as_uuid(),
            message.to_address,
            message.from_name,
            message.reply_to,
            message.subject,
            message.body_text,
            now,
        )
        .execute(&mut *tx)
        .await?;
        audit::record(
            &mut *tx,
            &event(tenant, now, "system", "outbox.draft_created", id, json!({})),
        )
        .await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Fetches one of `tenant`'s messages.
    pub async fn get(&self, tenant: TenantId, id: MessageId) -> Result<Message, OutboxError> {
        let row = sqlx::query!(
            "select id, tenant_id, status, to_address, from_name, reply_to, subject, body_text, sent_at, last_error
             from outbox_messages where id = $1 and tenant_id = $2",
            id.as_uuid(),
            tenant.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(OutboxError::NotFound)?;
        Ok(Message {
            id: row.id.into(),
            tenant: row.tenant_id.into(),
            status: MessageStatus::parse(&row.status)?,
            content: NewMessage {
                to_address: row.to_address,
                from_name: row.from_name,
                reply_to: row.reply_to,
                subject: row.subject,
                body_text: row.body_text,
            },
            sent_at: row.sent_at,
            last_error: row.last_error,
        })
    }

    /// Replaces a draft's or an approved message's content. Any approval is
    /// void: the message is a draft again.
    pub async fn edit(&self, tenant: TenantId, id: MessageId, message: &NewMessage) -> Result<(), OutboxError> {
        validate(message)?;
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let status = lock_status(&mut tx, tenant, id).await?;
        if !matches!(status, MessageStatus::Draft | MessageStatus::Approved) {
            return Err(OutboxError::WrongStatus(status));
        }
        sqlx::query!(
            "update outbox_messages
             set to_address = $3, from_name = $4, reply_to = $5, subject = $6, body_text = $7,
                 status = 'draft', approved_hash = null, approved_at = null, approved_by = null, updated_at = $8
             where id = $1 and tenant_id = $2",
            id.as_uuid(),
            tenant.as_uuid(),
            message.to_address,
            message.from_name,
            message.reply_to,
            message.subject,
            message.body_text,
            now,
        )
        .execute(&mut *tx)
        .await?;
        let voided = status == MessageStatus::Approved;
        audit::record(
            &mut *tx,
            &event(
                tenant,
                now,
                "system",
                "outbox.edited",
                id,
                json!({ "approval_voided": voided }),
            ),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Approves a draft. `seen` is the [`ContentHash`] of the content the
    /// approver was shown; if the content has changed since, nothing is
    /// approved (D2).
    pub async fn approve(
        &self,
        tenant: TenantId,
        id: MessageId,
        approver: &str,
        seen: &ContentHash,
    ) -> Result<(), OutboxError> {
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let status = lock_status(&mut tx, tenant, id).await?;
        if status != MessageStatus::Draft {
            return Err(OutboxError::WrongStatus(status));
        }
        let current = self.get_in(&mut tx, tenant, id).await?;
        if ContentHash::of(&current) != *seen {
            return Err(OutboxError::StaleApproval);
        }
        sqlx::query!(
            "update outbox_messages
             set status = 'approved', approved_hash = $3, approved_at = $4, approved_by = $5, updated_at = $4
             where id = $1 and tenant_id = $2",
            id.as_uuid(),
            tenant.as_uuid(),
            seen.as_bytes().as_slice(),
            now,
            approver,
        )
        .execute(&mut *tx)
        .await?;
        audit::record(
            &mut *tx,
            &event(tenant, now, approver, "outbox.approved", id, json!({})),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn get_in(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant: TenantId,
        id: MessageId,
    ) -> Result<NewMessage, OutboxError> {
        let row = sqlx::query!(
            "select to_address, from_name, reply_to, subject, body_text
             from outbox_messages where id = $1 and tenant_id = $2",
            id.as_uuid(),
            tenant.as_uuid(),
        )
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(OutboxError::NotFound)?;
        Ok(NewMessage {
            to_address: row.to_address,
            from_name: row.from_name,
            reply_to: row.reply_to,
            subject: row.subject,
            body_text: row.body_text,
        })
    }
}

async fn lock_status(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant: TenantId,
    id: MessageId,
) -> Result<MessageStatus, OutboxError> {
    let status = sqlx::query_scalar!(
        "select status from outbox_messages where id = $1 and tenant_id = $2 for update",
        id.as_uuid(),
        tenant.as_uuid(),
    )
    .fetch_optional(&mut **tx)
    .await?
    .ok_or(OutboxError::NotFound)?;
    MessageStatus::parse(&status)
}

fn event<'a>(
    tenant: TenantId,
    at: OffsetDateTime,
    actor: &'a str,
    action: &'static str,
    id: MessageId,
    details: serde_json::Value,
) -> AuditEvent<'a> {
    AuditEvent {
        tenant,
        at,
        actor,
        action,
        subject_type: "outbox_message",
        subject_id: id.as_uuid(),
        details,
    }
}

/// Header fields must be single lines, so a crafted value can't add headers
/// or recipients.
fn validate(m: &NewMessage) -> Result<(), OutboxError> {
    let single_line = [
        ("recipient", m.to_address.as_str()),
        ("sender name", m.from_name.as_str()),
        ("reply-to", m.reply_to.as_deref().unwrap_or("")),
        ("subject", m.subject.as_str()),
    ];
    if let Some((field, _)) = single_line.iter().find(|(_, v)| v.contains(['\r', '\n'])) {
        return Err(OutboxError::InvalidMessage(format!("{field} must be a single line")));
    }
    if !m.to_address.contains('@') {
        return Err(OutboxError::InvalidMessage(
            "recipient is not an email address".to_owned(),
        ));
    }
    if m.body_text.trim().is_empty() {
        return Err(OutboxError::InvalidMessage("body is empty".to_owned()));
    }
    Ok(())
}
