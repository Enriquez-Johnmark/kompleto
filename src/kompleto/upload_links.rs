//! Upload links (F2). One per Client per collection Period, created with the
//! Period's first Draft and used in all its Reminders.
//!
//! A link is the record's ID followed by a 128-bit HMAC signature, made with
//! a key derived from the master key. The database holds no usable link, and
//! a guessed or altered link is simply not found. A link works until 30 days
//! after its Period's Cut-off.

use std::sync::Arc;

use hmac::{Hmac, KeyInit, Mac};
use secrecy::{ExposeSecret, SecretBox};
use sha2::Sha256;
use sqlx::PgPool;
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

use super::clients::ClientId;
use super::firms::FirmId;
use super::schedule::MANILA;
use crate::engine::clock::Clock;
use crate::engine::files::MasterKey;
use crate::engine::ids::id_type;

id_type!(
    /// One Upload link.
    UploadLinkId
);

/// How long a link works after its Period's Cut-off (F2).
pub const WORKS_FOR_DAYS_AFTER_CUT_OFF: i64 = 30;
/// Signature length in bytes: 128 bits.
const SIGNATURE_LEN: usize = 16;

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

/// Creates and checks Upload links.
#[derive(Clone)]
pub struct UploadLinks {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    key: Arc<SecretBox<[u8; 32]>>,
    base_url: String,
}

impl UploadLinks {
    /// Links to an app reached at `base_url`, signed with a key from `master`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, master: &MasterKey, base_url: &str) -> Self {
        Self {
            pool,
            clock,
            key: Arc::new(master.derive("kompleto upload link v1")),
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
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
        Ok(self.url(id))
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
        let id = self.verify(token).ok_or(LinkError::NotFound)?;
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

    fn url(&self, id: Uuid) -> String {
        let signature: String = self.sign(id)[..SIGNATURE_LEN]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        format!("{}/u/{}{signature}", self.base_url, id.simple())
    }

    fn mac(&self, id: Uuid) -> Hmac<Sha256> {
        let mut padded = [0_u8; 64];
        padded[..32].copy_from_slice(self.key.expose_secret());
        <Hmac<Sha256> as KeyInit>::new(&padded.into()).chain_update(id.as_bytes())
    }

    fn sign(&self, id: Uuid) -> [u8; 32] {
        self.mac(id).finalize().into_bytes().into()
    }

    /// The ID in `token`, if its signature is right (compared in constant time).
    fn verify(&self, token: &str) -> Option<Uuid> {
        let bytes = from_hex(token)?;
        let (id, signature) = bytes.split_at_checked(16)?;
        if signature.len() != SIGNATURE_LEN {
            return None;
        }
        let id = Uuid::from_slice(id).ok()?;
        self.mac(id).verify_truncated_left(signature).ok()?;
        Some(id)
    }
}

/// Decodes lowercase or uppercase hex; `None` if it isn't hex.
fn from_hex(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) || !text.is_ascii() {
        return None;
    }
    text.as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok())
        .collect()
}

/// When a link stops working: the end of the 30th day after the Cut-off,
/// in Manila.
fn expires_at(cut_off: Date) -> OffsetDateTime {
    (cut_off + Duration::days(WORKS_FOR_DAYS_AFTER_CUT_OFF + 1))
        .midnight()
        .assume_offset(MANILA)
}
