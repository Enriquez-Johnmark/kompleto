//! Bounces (G3): delivery failures and spam complaints the provider reports
//! for Reminders we sent. Only events matching one of our messages count.
//!
//! A permanent Bounce or a complaint stops the Client's Reminders, in every
//! Period, for as long as the Client's email is the address that failed:
//! changing it starts them again. A temporary Bounce is only noted.
//!
//! Recording is a system-wide worker: the provider's message ID names the
//! Firm, so that one lookup isn't scoped by a Firm. Everything after it is.

use std::sync::Arc;

use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::clients::ClientId;
use super::drafts::Drafts;
use super::firms::FirmId;
use crate::engine::clock::Clock;
use crate::engine::outbox::{Outbox, OutboxError};
use crate::engine::webhooks::DeliveryEvent;

/// Bounce errors.
#[derive(Debug, thiserror::Error)]
pub enum BounceError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Cancelling the Client's Reminders failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// What became of a delivery event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recorded {
    /// It matched no Reminder we sent, or wasn't a failure: ignored.
    Ignored,
    /// A temporary Bounce, noted.
    Noted,
    /// A permanent Bounce or complaint: the Client's Reminders stopped.
    Stopped,
}

/// One Bounce or complaint, as the Firm sees it.
#[derive(Debug, Clone)]
pub struct Bounce {
    /// The address that failed.
    pub address: String,
    /// `bounce`, `soft_bounce` or `complaint`.
    pub kind: String,
    /// The provider's explanation.
    pub reason: Option<String>,
    /// When it was reported.
    pub at: OffsetDateTime,
}

/// Records delivery failures.
#[derive(Clone)]
pub struct Bounces {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
    drafts: Drafts,
}

impl Bounces {
    /// Bounces of `pool`'s Firms.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            drafts: Drafts::new(pool.clone(), clock.clone()),
            pool,
            clock,
        }
    }

    /// Takes in one verified delivery event.
    pub async fn record(&self, event: &DeliveryEvent) -> Result<Recorded, BounceError> {
        let (provider_message_id, kind, reason) = match event {
            DeliveryEvent::Bounced {
                provider_message_id,
                permanent,
                reason,
            } => (
                provider_message_id,
                if *permanent { "bounce" } else { "soft_bounce" },
                Some(reason.as_str()),
            ),
            DeliveryEvent::Complained { provider_message_id } => (provider_message_id, "complaint", None),
            DeliveryEvent::Other => return Ok(Recorded::Ignored),
        };
        let Some(sent) = self.outbox.find_sent(provider_message_id).await? else {
            tracing::info!(provider_message_id, "a delivery event for no message we sent; ignored");
            return Ok(Recorded::Ignored);
        };
        let firm: FirmId = sent.tenant;
        let client = sqlx::query_scalar!(
            "select client_id from scheduled_drafts where firm_id = $1 and message_id = $2",
            firm.as_uuid(),
            sent.id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(client) = client.map(ClientId::from) else {
            // A login email or notice to the Firm itself: not a Reminder.
            tracing::warn!(%firm, kind, "a message to the firm itself bounced");
            return Ok(Recorded::Ignored);
        };
        let address = sent.to_address.trim().to_ascii_lowercase();
        sqlx::query!(
            "insert into bounces (id, firm_id, client_id, message_id, address, kind, reason, at)
             values ($1, $2, $3, $4, $5, $6, $7, $8)",
            Uuid::now_v7(),
            firm.as_uuid(),
            client.as_uuid(),
            sent.id.as_uuid(),
            address,
            kind,
            reason,
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        if kind == "soft_bounce" {
            return Ok(Recorded::Noted);
        }
        self.drafts.cancel_waiting_to(firm, client, &address).await?;
        // A late Bounce for an address since replaced stops nothing more.
        let current = sqlx::query_scalar!(
            "select lower(trim(email)) as \"email!\" from clients where firm_id = $1 and id = $2",
            firm.as_uuid(),
            client.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?;
        if current.as_deref() == Some(address.as_str()) {
            Ok(Recorded::Stopped)
        } else {
            Ok(Recorded::Noted)
        }
    }

    /// A Client's Bounces and complaints, newest first.
    pub async fn for_client(&self, firm: FirmId, client: ClientId) -> Result<Vec<Bounce>, BounceError> {
        let rows = sqlx::query!(
            "select address, kind, reason, at from bounces
             where firm_id = $1 and client_id = $2 order by at desc",
            firm.as_uuid(),
            client.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Bounce {
                address: row.address,
                kind: row.kind,
                reason: row.reason,
                at: row.at,
            })
            .collect())
    }
}
