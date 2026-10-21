//! A Firm's reminder Drafts as the approval pages see them: the outbox
//! message, which Client it's for, its Cut-off and who wrote it. Every
//! query is scoped by the Firm (A2), and only scheduled reminders count;
//! the Firm's own login emails are never listed or approvable here.

use std::sync::Arc;

use sqlx::PgPool;
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

use super::drafting::DraftSource;
use super::firms::FirmId;
use super::schedule::MANILA;
use crate::engine::clock::Clock;
use crate::engine::ids::MessageId;
use crate::engine::mail::Email;
use crate::engine::outbox::{ContentHash, MessageStatus};

/// One reminder Draft, or a reminder already approved.
#[derive(Debug, Clone)]
pub struct Reminder {
    /// The outbox message.
    pub id: MessageId,
    /// The Client's business name.
    pub client: String,
    /// Its content.
    pub email: Email,
    /// Where it is in its life.
    pub status: MessageStatus,
    /// Who wrote the wording.
    pub source: DraftSource,
    /// The Period's Cut-off.
    pub cut_off: Option<Date>,
    /// Why it was held back or its last attempt failed.
    pub last_error: Option<String>,
}

impl Reminder {
    /// The fingerprint an approval must match (D2).
    pub fn content_hash(&self) -> ContentHash {
        ContentHash::of(&self.email)
    }
}

/// Reading reminders.
#[derive(Clone)]
pub struct Drafts {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

struct ReminderRow {
    id: Uuid,
    client: String,
    to_address: String,
    from_name: String,
    reply_to: Option<String>,
    subject: String,
    body_text: String,
    status: String,
    source: String,
    expires_at: Option<OffsetDateTime>,
    last_error: Option<String>,
}

impl ReminderRow {
    fn into_reminder(self) -> Option<Reminder> {
        Some(Reminder {
            id: self.id.into(),
            client: self.client,
            email: Email {
                to_address: self.to_address,
                from_name: self.from_name,
                reply_to: self.reply_to,
                subject: self.subject,
                body_text: self.body_text,
            },
            status: MessageStatus::parse_name(&self.status)?,
            source: if self.source == "ai" {
                DraftSource::Ai
            } else {
                DraftSource::Template
            },
            // A Draft expires as its Cut-off day ends (D4).
            cut_off: self
                .expires_at
                .map(|at| at.to_offset(MANILA).date() - Duration::days(1)),
            last_error: self.last_error,
        })
    }
}

impl Drafts {
    /// Reminders of `pool`'s Firms.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Drafts waiting for approval, nearest Cut-off first.
    pub async fn waiting(&self, firm: FirmId) -> Result<Vec<Reminder>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ReminderRow,
            "select m.id, c.business_name as client, m.to_address, m.from_name, m.reply_to, m.subject,
                    m.body_text, m.status, s.source, m.expires_at, m.last_error
             from scheduled_drafts s
             join outbox_messages m on m.id = s.message_id and m.tenant_id = s.firm_id
             join clients c on c.id = s.client_id and c.firm_id = s.firm_id
             where s.firm_id = $1 and m.status = 'draft' and (m.expires_at is null or m.expires_at > $2)
             order by m.expires_at, c.business_name, m.created_at",
            firm.as_uuid(),
            self.clock.now(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().filter_map(ReminderRow::into_reminder).collect())
    }

    /// Reminders approved in the last week, newest first: sent, on their
    /// way, or held back (failed, blocked, unconfirmed) with the reason.
    pub async fn recently_approved(&self, firm: FirmId) -> Result<Vec<Reminder>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ReminderRow,
            "select m.id, c.business_name as client, m.to_address, m.from_name, m.reply_to, m.subject,
                    m.body_text, m.status, s.source, m.expires_at, m.last_error
             from scheduled_drafts s
             join outbox_messages m on m.id = s.message_id and m.tenant_id = s.firm_id
             join clients c on c.id = s.client_id and c.firm_id = s.firm_id
             where s.firm_id = $1 and m.approved_at > $2
               and m.status in ('approved', 'sending', 'sent', 'failed', 'blocked', 'unconfirmed')
             order by m.approved_at desc",
            firm.as_uuid(),
            self.clock.now() - Duration::days(7),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().filter_map(ReminderRow::into_reminder).collect())
    }

    /// One of the Firm's reminders; `None` for anything else, including
    /// another Firm's message or a login email.
    pub async fn get(&self, firm: FirmId, id: MessageId) -> Result<Option<Reminder>, sqlx::Error> {
        let row = sqlx::query_as!(
            ReminderRow,
            "select m.id, c.business_name as client, m.to_address, m.from_name, m.reply_to, m.subject,
                    m.body_text, m.status, s.source, m.expires_at, m.last_error
             from scheduled_drafts s
             join outbox_messages m on m.id = s.message_id and m.tenant_id = s.firm_id
             join clients c on c.id = s.client_id and c.firm_id = s.firm_id
             where s.firm_id = $1 and m.id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.and_then(ReminderRow::into_reminder))
    }
}
