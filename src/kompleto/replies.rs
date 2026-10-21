//! Replies (G1): what Contacts send back to their Reminders. A human Reply
//! stops the Period's Reminders and tells the Firm, by an email that links
//! to the Reply in Kompleto rather than copying it, so nothing a Contact
//! writes is ever relayed from our domain. An automatic reply (out of
//! office) is only noted. Mail that matches no Client keeps no content.
//!
//! Receiving is a system-wide worker: the message itself names the Firm,
//! through its signed reply address or a sender address only one Client
//! has, so those two lookups aren't scoped by a Firm. Everything after them
//! names the Firm.

use std::sync::Arc;

use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::calendar::TaxPeriod;
use super::clients::{ClientId, Clients};
use super::drafts::Drafts;
use super::firms::{FirmId, Firms};
use super::presend::KompletoChecks;
use super::upload_links::{OpenLink, UploadLinks};
use super::uploads::{Label, UploadError, Uploads};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::inbound::InboundEmail;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};

/// The approver recorded for Reply notices, sent without a person approving.
pub const NOTICE_APPROVER: &str = "system:replies";

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
    /// Cancelling the Period's Reminders or sending a notice failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
    /// The notice's Firm or Client couldn't be read.
    #[error("reply notice: {0}")]
    Notice(String),
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
    /// How many attachments were kept as Uploads.
    pub attachments: i32,
    /// It came through its signed reply address, not just from the
    /// Contact's address, which anyone can put in From.
    pub signed: bool,
}

/// Receives and lists Replies.
#[derive(Clone)]
pub struct Replies {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    links: UploadLinks,
    drafts: Drafts,
    firms: Firms,
    clients: Clients,
    outbox: Outbox,
    checks: KompletoChecks,
    base_url: String,
    uploads: Option<Uploads>,
}

/// The label a Reply's attachments get as Uploads.
pub const ATTACHMENT_NOTE: &str = "Sent by email";

impl Replies {
    /// Replies matched through `links`' reply addresses; notices link to
    /// pages at `base_url` and pass `checks` like every other message.
    pub fn new(
        pool: PgPool,
        clock: Arc<dyn Clock>,
        links: UploadLinks,
        checks: KompletoChecks,
        base_url: &str,
    ) -> Self {
        Self {
            drafts: Drafts::new(pool.clone(), clock.clone()),
            firms: Firms::new(pool.clone(), clock.clone()),
            clients: Clients::new(pool.clone(), clock.clone()),
            outbox: Outbox::new(pool.clone(), clock.clone()),
            checks,
            base_url: base_url.trim_end_matches('/').to_owned(),
            pool,
            clock,
            links,
            uploads: None,
        }
    }

    /// Matches Replies through `links` instead, e.g. ones with reply
    /// addresses.
    #[must_use]
    pub fn matching(mut self, links: UploadLinks) -> Self {
        self.links = links;
        self
    }

    /// Keeps a human Reply's photo and PDF attachments as Uploads.
    #[must_use]
    pub fn with_uploads(mut self, uploads: Uploads) -> Self {
        self.uploads = Some(uploads);
        self
    }

    /// Takes in one inbound email.
    pub async fn receive(&self, email: &InboundEmail) -> Result<Received, ReplyError> {
        let Some((link, signed)) = self.matching_link(email).await? else {
            let domain = email.from.rsplit_once('@').map_or("", |(_, domain)| domain);
            tracing::info!(sender_domain = domain, "inbound email matched no client; nothing kept");
            return Ok(Received::Unmatched);
        };
        let new = ReplyId::new();
        let inserted = sqlx::query_scalar!(
            "insert into replies
                 (id, firm_id, client_id, period, from_address, subject, body_text, automatic, received_at, message_key, signed)
             values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             on conflict (firm_id, message_key) do nothing
             returning id",
            new.as_uuid(),
            link.firm.as_uuid(),
            link.client.as_uuid(),
            link.period,
            email.from,
            email.subject,
            email.text,
            email.automatic,
            self.clock.now(),
            email.key,
            signed,
        )
        .fetch_optional(&self.pool)
        .await?;
        // A redelivery: the provider retries when an earlier attempt failed
        // part way, so finish what's left, but never keep anything twice.
        let (id, handled) = match inserted {
            Some(_) => (new, false),
            None => {
                let row = sqlx::query!(
                    "select id, handled_at is not null as \"handled!\" from replies
                     where firm_id = $1 and message_key = $2",
                    link.firm.as_uuid(),
                    email.key,
                )
                .fetch_one(&self.pool)
                .await?;
                (ReplyId::from(row.id), row.handled)
            }
        };
        if email.automatic {
            return Ok(Received::Automatic(id));
        }
        if handled {
            return Ok(Received::Human(id));
        }
        // Attachments are kept on the first attempt only; one a failure
        // dropped is lost rather than risk keeping one twice.
        // Only a signed Reply changes anything; one matched by its sender
        // alone is shown and announced, nothing more.
        if inserted.is_some() && signed {
            let kept = self.keep_attachments(&link, &email.attachments).await;
            if kept > 0 {
                sqlx::query!(
                    "update replies set attachments = $3 where id = $1 and firm_id = $2",
                    id.as_uuid(),
                    link.firm.as_uuid(),
                    kept,
                )
                .execute(&self.pool)
                .await?;
            }
        }
        if signed {
            self.drafts.cancel_waiting(link.firm, link.client, &link.period).await?;
        }
        // The Reply is kept either way; a notice that fails is logged.
        if let Err(err) = self.notify(&link, id, signed).await {
            tracing::error!(firm = %link.firm, reply = %id, %err, "notifying the firm of a reply failed");
        }
        sqlx::query!(
            "update replies set handled_at = $3 where id = $1 and firm_id = $2",
            id.as_uuid(),
            link.firm.as_uuid(),
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        Ok(Received::Human(id))
    }

    /// Keeps each attachment the upload rules accept (photos and PDFs, the
    /// size and per-link limits) as an Upload; returns how many.
    async fn keep_attachments(&self, link: &OpenLink, attachments: &[Vec<u8>]) -> i32 {
        let Some(uploads) = &self.uploads else {
            return 0;
        };
        let mut kept = 0;
        for bytes in attachments {
            let label = Label::SomethingElse {
                note: ATTACHMENT_NOTE.to_owned(),
            };
            match uploads.add(link, &[], label, bytes).await {
                Ok(_) => kept += 1,
                Err(err @ (UploadError::File(_) | UploadError::Database(_))) => {
                    tracing::error!(firm = %link.firm, %err, "keeping a reply's attachment failed");
                }
                // Not a photo or PDF, too big, or the link is full: dropped.
                Err(_) => {}
            }
        }
        kept
    }

    /// Emails the Firm that Reply `id` arrived, with a link to read it.
    async fn notify(&self, link: &OpenLink, id: ReplyId, signed: bool) -> Result<(), ReplyError> {
        let firm = self
            .firms
            .get(link.firm)
            .await
            .map_err(|err| ReplyError::Notice(err.to_string()))?;
        let client = self
            .clients
            .get(link.firm, link.client)
            .await
            .map_err(|err| ReplyError::Notice(err.to_string()))?;
        let period =
            TaxPeriod::parse_key(&link.period).map_or_else(|| link.period.clone(), |period| period.to_string());
        let (contact, business) = (&client.details.contact_first_name, &client.details.business_name);
        let email = Email {
            to_address: firm.login_email,
            from_name: "Kompleto".to_owned(),
            reply_to: None,
            subject: format!("{contact} ({business}) replied about {period}"),
            body_text: format!(
                "{contact} at {business} replied to your reminder about {period}. {}\n\n\
                 Read the reply in Kompleto:\n{}/replies/{id}\n",
                if signed {
                    format!("Their reminders for {period} have stopped.")
                } else {
                    "It came from their address but not through the reminder's reply address, \
                     so their reminders continue; stop them yourself if the reply is genuine."
                        .to_owned()
                },
                self.base_url
            ),
        };
        let message = self.outbox.create_draft(link.firm, &email).await?;
        let hash = self.outbox.get(link.firm, message).await?.content_hash();
        self.outbox
            .approve(link.firm, message, NOTICE_APPROVER, &hash, &self.checks)
            .await?;
        Ok(())
    }

    /// The link the email answers, and whether it came through the signed
    /// reply address; else the latest Reminder sent to the one Client with
    /// the sender's address.
    async fn matching_link(&self, email: &InboundEmail) -> Result<Option<(OpenLink, bool)>, ReplyError> {
        if let Some(link) = self.links.by_reply_address(&email.recipients).await? {
            return Ok(Some((link, true)));
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
            .await?
            .map(|link| (link, false)))
    }

    /// A Client's Replies, newest first.
    pub async fn for_client(&self, firm: FirmId, client: ClientId) -> Result<Vec<Reply>, ReplyError> {
        let rows = sqlx::query!(
            "select id, client_id, period, from_address, subject, body_text, automatic, received_at, attachments, signed
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
                attachments: row.attachments,
                signed: row.signed,
            })
            .collect())
    }

    /// Reply `id` of `firm`.
    pub async fn get(&self, firm: FirmId, id: ReplyId) -> Result<Reply, ReplyError> {
        let row = sqlx::query!(
            "select id, client_id, period, from_address, subject, body_text, automatic, received_at, attachments, signed
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
            attachments: row.attachments,
            signed: row.signed,
        })
    }
}

/// How long a Reply's text is kept.
pub const TEXT_KEPT_DAYS: i64 = 90;

/// Deletes the subject and text of every Reply older than 90 days at `now`;
/// returns how many. A system-wide sweep: the row stays, as "replied on
/// <date>", for every Firm.
pub async fn forget_old_text(pool: &PgPool, now: OffsetDateTime) -> Result<u64, sqlx::Error> {
    let forgotten = sqlx::query!(
        "update replies r set subject = null, body_text = null
         from (select id, firm_id from replies where body_text is not null and received_at <= $1) old
         where r.id = old.id and r.firm_id = old.firm_id",
        now - time::Duration::days(TEXT_KEPT_DAYS),
    )
    .execute(pool)
    .await?;
    Ok(forgotten.rows_affected())
}
