//! Replies (G1): what Contacts send back to their Reminders. A human Reply
//! stops the Period's Reminders; an automatic one (out of office) is only
//! noted. Mail that matches no Client keeps no content.
//!
//! Receiving is a system-wide worker: the message itself names the Firm,
//! through its signed reply address or a sender address only one Client
//! has, so those two lookups aren't scoped by a Firm. Everything after them
//! names the Firm.

use std::sync::Arc;

use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::clients::ClientId;
use super::drafts::Drafts;
use super::firms::FirmId;
use super::upload_links::{OpenLink, UploadLinks};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::inbound::InboundEmail;
use crate::engine::outbox::OutboxError;

id_type!(
    /// One Reply.
    ReplyId
);

/// Reply errors.
#[derive(Debug, thiserror::Error)]
pub enum ReplyError {
    /// No such Reply for this Firm.
    #[error("reply not found")]
    NotFound,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Cancelling the Period's Reminders failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// What became of an inbound email.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Received {
    /// It matched no Client; nothing of it was kept.
    Unmatched,
    /// An automatic reply, noted; Reminders continue.
    Automatic(ReplyId),
    /// A human Reply; the Period's waiting Reminders were cancelled.
    Human(ReplyId),
}

/// One Reply, as the Firm sees it.
#[derive(Debug, Clone)]
pub struct Reply {
    /// Its ID.
    pub id: ReplyId,
    /// The Client it came from.
    pub client: ClientId,
    /// The collection Period's key.
    pub period: String,
    /// The sender's address.
    pub from: String,
    /// The subject, until it's deleted with the text.
    pub subject: Option<String>,
    /// The text, kept 90 days.
    pub text: Option<String>,
    /// Sent automatically (out of office).
    pub automatic: bool,
    /// When it arrived.
    pub received_at: OffsetDateTime,
}

/// Receives and lists Replies.
#[derive(Clone)]
pub struct Replies {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    links: UploadLinks,
    drafts: Drafts,
}

impl Replies {
    /// Replies matched through `links`' reply addresses.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, links: UploadLinks) -> Self {
        Self {
            drafts: Drafts::new(pool.clone(), clock.clone()),
            pool,
            clock,
            links,
        }
    }

    /// Takes in one inbound email.
    pub async fn receive(&self, email: &InboundEmail) -> Result<Received, ReplyError> {
        let Some(link) = self.matching_link(email).await? else {
            let domain = email.from.rsplit_once('@').map_or("", |(_, domain)| domain);
            tracing::info!(sender_domain = domain, "inbound email matched no client; nothing kept");
            return Ok(Received::Unmatched);
        };
        let id = ReplyId::new();
        sqlx::query!(
            "insert into replies (id, firm_id, client_id, period, from_address, subject, body_text, automatic, received_at)
             values ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            id.as_uuid(),
            link.firm.as_uuid(),
            link.client.as_uuid(),
            link.period,
            email.from,
            email.subject,
            email.text,
            email.automatic,
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        if email.automatic {
            return Ok(Received::Automatic(id));
        }
        self.drafts.cancel_waiting(link.firm, link.client, &link.period).await?;
        Ok(Received::Human(id))
    }

    /// The link the email answers: through its reply address, or else the
    /// latest Reminder sent to the one Client with the sender's address.
    async fn matching_link(&self, email: &InboundEmail) -> Result<Option<OpenLink>, ReplyError> {
        if let Some(link) = self.links.by_reply_address(&email.recipients).await? {
            return Ok(Some(link));
        }
        let clients = sqlx::query!(
            "select id, firm_id from clients where lower(email) = $1 limit 2",
            email.from,
        )
        .fetch_all(&self.pool)
        .await?;
        let [client] = clients.as_slice() else {
            return Ok(None);
        };
        let period = sqlx::query_scalar!(
            "select s.period from scheduled_drafts s
             join outbox_messages m on m.id = s.message_id and m.tenant_id = s.firm_id
             where s.firm_id = $1 and s.client_id = $2 and m.status = 'sent'
             order by m.sent_at desc limit 1",
            client.firm_id,
            client.id,
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(period) = period else {
            return Ok(None);
        };
        Ok(self
            .links
            .find(client.firm_id.into(), client.id.into(), &period)
            .await?)
    }

    /// A Client's Replies, newest first.
    pub async fn for_client(&self, firm: FirmId, client: ClientId) -> Result<Vec<Reply>, ReplyError> {
        let rows = sqlx::query!(
            "select id, client_id, period, from_address, subject, body_text, automatic, received_at
             from replies where firm_id = $1 and client_id = $2 order by received_at desc, id desc",
            firm.as_uuid(),
            client.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Reply {
                id: row.id.into(),
                client: row.client_id.into(),
                period: row.period,
                from: row.from_address,
                subject: row.subject,
                text: row.body_text,
                automatic: row.automatic,
                received_at: row.received_at,
            })
            .collect())
    }

    /// Reply `id` of `firm`.
    pub async fn get(&self, firm: FirmId, id: ReplyId) -> Result<Reply, ReplyError> {
        let row = sqlx::query!(
            "select id, client_id, period, from_address, subject, body_text, automatic, received_at
             from replies where firm_id = $1 and id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(ReplyError::NotFound)?;
        Ok(Reply {
            id: row.id.into(),
            client: row.client_id.into(),
            period: row.period,
            from: row.from_address,
            subject: row.subject,
            text: row.body_text,
            automatic: row.automatic,
            received_at: row.received_at,
        })
    }
}
