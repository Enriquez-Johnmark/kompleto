//! Upload links (F2). One per Client per collection Period, created with the
//! Period's first Draft and used in all its Reminders.
//!
//! A link is the record's ID followed by a 128-bit HMAC signature, made with
//! a key derived from the master key. The database holds no usable link, and
//! a guessed or altered link is simply not found. A link works until 30 days
//! after its Period's Cut-off.
//!
//! Each link also has a reply address, `reply+CODE@domain`, whose code is
//! the link's ID signed with a separate key. A Contact's Reply to a Reminder
//! comes back to it, which names the Client and Period (G1).

use std::sync::Arc;

use sqlx::PgPool;
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

use super::clients::ClientId;
use super::firms::FirmId;
use super::schedule::MANILA;
use crate::engine::clock::Clock;
use crate::engine::files::MasterKey;
use crate::engine::ids::id_type;
use crate::engine::signed::Signer;

id_type!(
    /// One Upload link.
    UploadLinkId
);

/// How long a link works after its Period's Cut-off (F2).
pub const WORKS_FOR_DAYS_AFTER_CUT_OFF: i64 = 30;

/// Why a link doesn't open.
#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    /// No such link, or its signature is wrong.
    #[error("upload link not found")]
    NotFound,
    /// The link is genuine but past its last day.
    #[error("upload link expired")]
    Expired(OpenLink),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// A genuine link, and whose it is.
#[derive(Debug, Clone)]
pub struct OpenLink {
    /// The link.
    pub id: UploadLinkId,
    /// The Firm it belongs to.
    pub firm: FirmId,
    /// The Client it belongs to.
    pub client: ClientId,
    /// The collection Period's key, e.g. `2026-10`.
    pub period: String,
    /// The Period's Cut-off.
    pub cut_off: Date,
}

/// A link as issued for a Reminder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssuedLink {
    /// The upload page's address.
    pub url: String,
    /// Where the Contact's Reply goes, when replies are set up.
    pub reply_to: Option<String>,
}

/// Creates and checks Upload links and their reply addresses.
#[derive(Clone)]
pub struct UploadLinks {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    signer: Arc<Signer>,
    reply_signer: Arc<Signer>,
    reply_domain: Option<String>,
    base_url: String,
}

impl UploadLinks {
    /// Links to an app reached at `base_url`, signed with a key from `master`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, master: &MasterKey, base_url: &str) -> Self {
        Self {
            pool,
            clock,
            signer: Arc::new(Signer::new(master, "kompleto upload link v1")),
            reply_signer: Arc::new(Signer::new(master, "kompleto reply address v1")),
            reply_domain: None,
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// Gives each link a reply address at `domain`.
    #[must_use]
    pub fn with_replies(mut self, domain: &str) -> Self {
        self.reply_domain = Some(domain.trim().to_ascii_lowercase());
        self
    }

    /// The Client's link for `period`, created if it doesn't exist yet. A
    /// changed Cut-off (a new offset or due date) moves the link's last day
    /// with it.
    pub async fn for_period(
        &self,
        firm: FirmId,
        client: ClientId,
        period: &str,
        cut_off: Date,
    ) -> Result<String, sqlx::Error> {
        Ok(self.issue(firm, client, period, cut_off).await?.url)
    }

    /// As [`UploadLinks::for_period`], with the link's reply address.
    pub async fn issue(
        &self,
        firm: FirmId,
        client: ClientId,
        period: &str,
        cut_off: Date,
    ) -> Result<IssuedLink, sqlx::Error> {
        sqlx::query!(
            "insert into upload_links (id, firm_id, client_id, period, cut_off, expires_at, created_at)
             values ($1, $2, $3, $4, $5, $6, $7)
             on conflict (client_id, period) do update
                 set cut_off = excluded.cut_off, expires_at = excluded.expires_at
                 where upload_links.cut_off <> excluded.cut_off",
            Uuid::now_v7(),
            firm.as_uuid(),
            client.as_uuid(),
            period,
            cut_off,
            expires_at(cut_off),
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        let id = sqlx::query_scalar!(
            "select id from upload_links where firm_id = $1 and client_id = $2 and period = $3",
            firm.as_uuid(),
            client.as_uuid(),
            period,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(IssuedLink {
            url: self.url(id),
            reply_to: self
                .reply_domain
                .as_ref()
                .map(|domain| format!("reply+{}@{domain}", self.reply_signer.token(id))),
        })
    }

    /// The Client's link for `period` as [`UploadLinks::open`] gives it, if
    /// it has one (expired or not).
    pub async fn find(&self, firm: FirmId, client: ClientId, period: &str) -> Result<Option<OpenLink>, sqlx::Error> {
        let row = sqlx::query!(
            "select id, cut_off from upload_links where firm_id = $1 and client_id = $2 and period = $3",
            firm.as_uuid(),
            client.as_uuid(),
            period,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|row| OpenLink {
            id: row.id.into(),
            firm,
            client,
            period: period.to_owned(),
            cut_off: row.cut_off,
        }))
    }

    /// The Client's link for `period`, if it has one.
    pub async fn existing(&self, firm: FirmId, client: ClientId, period: &str) -> Result<Option<String>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            "select id from upload_links where firm_id = $1 and client_id = $2 and period = $3",
            firm.as_uuid(),
            client.as_uuid(),
            period,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(id.map(|id| self.url(id)))
    }

    /// The link that `token` (the last part of its URL) names, if it's
    /// genuine and still works. The token itself is the credential, so this
    /// is the one lookup not scoped by a Firm.
    pub async fn open(&self, token: &str) -> Result<OpenLink, LinkError> {
        let id = self.signer.verify(token).ok_or(LinkError::NotFound)?;
        let row = sqlx::query!(
            "select firm_id, client_id, period, cut_off, expires_at from upload_links where id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(LinkError::NotFound)?;
        let link = OpenLink {
            id: id.into(),
            firm: row.firm_id.into(),
            client: row.client_id.into(),
            period: row.period,
            cut_off: row.cut_off,
        };
        if self.clock.now() >= row.expires_at {
            return Err(LinkError::Expired(link));
        }
        Ok(link)
    }

    /// The link whose reply address is among `recipients`, if any; a
    /// forged or altered code names none. The signed code is the credential,
    /// as in [`UploadLinks::open`], so this lookup isn't scoped by a Firm. An
    /// expired link still takes Replies.
    pub async fn by_reply_address(&self, recipients: &[String]) -> Result<Option<OpenLink>, sqlx::Error> {
        let Some(domain) = &self.reply_domain else {
            return Ok(None);
        };
        let id = recipients.iter().find_map(|address| {
            let (local, at) = address.rsplit_once('@')?;
            let code = local.strip_prefix("reply+")?;
            (at == domain).then(|| self.reply_signer.verify(code)).flatten()
        });
        let Some(id) = id else {
            return Ok(None);
        };
        let row = sqlx::query!(
            "select firm_id, client_id, period, cut_off from upload_links where id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|row| OpenLink {
            id: id.into(),
            firm: row.firm_id.into(),
            client: row.client_id.into(),
            period: row.period,
            cut_off: row.cut_off,
        }))
    }

    fn url(&self, id: Uuid) -> String {
        format!("{}/u/{}", self.base_url, self.signer.token(id))
    }
}

/// When a link stops working: the end of the 30th day after the Cut-off,
/// in Manila.
fn expires_at(cut_off: Date) -> OffsetDateTime {
    (cut_off + Duration::days(WORKS_FOR_DAYS_AFTER_CUT_OFF + 1))
        .midnight()
        .assume_offset(MANILA)
}
