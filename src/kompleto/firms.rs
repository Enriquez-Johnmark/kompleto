//! Firms: the bookkeepers who use Kompleto. Each is an engine account
//! (`engine::accounts`), whose tenant is the Firm: the account holds its
//! name, owner, staff and closing; the `firms` table holds only Kompleto's
//! own settings, and a Firm with no row there has the defaults.

use std::sync::Arc;

use sqlx::PgPool;

use super::activity::record_as;
use crate::engine::accounts::{AccountError, Accounts};
use crate::engine::clock::Clock;
use crate::engine::ids::TenantId;

pub use crate::engine::accounts::check_name;

/// A Firm's default days between a Period's earliest Due date and its Cut-off.
pub const DEFAULT_CUT_OFF_OFFSET_DAYS: u8 = 5;

/// A Firm's ID; the same value is its engine `TenantId`.
pub type FirmId = TenantId;

/// A Firm as stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Firm {
    /// Its ID.
    pub id: FirmId,
    /// The name Clients see as the sender, e.g. "Santos Bookkeeping".
    pub name: String,
    /// The one address that can log in.
    pub login_email: String,
    /// Default days between a Period's earliest Due date and its Cut-off.
    pub cut_off_offset_days: u8,
}

/// Firm errors.
#[derive(Debug, thiserror::Error)]
pub enum FirmError {
    /// No such Firm.
    #[error("firm not found")]
    NotFound,
    /// Another Firm already logs in with this address.
    #[error("another firm already uses that login email")]
    LoginEmailTaken,
    /// A name or email that can't be used.
    #[error("{0}")]
    Invalid(String),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// The Firm repository.
#[derive(Clone)]
pub struct Firms {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    /// Who the activity log names; `None` is the Firm's login address.
    actor: Option<String>,
}

impl Firms {
    /// A repository over `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            pool,
            clock,
            actor: None,
        }
    }

    /// The same repository, naming `actor` (a member's address) in the
    /// activity log (K3).
    #[must_use]
    pub fn by(&self, actor: &str) -> Self {
        Self {
            actor: Some(actor.to_owned()),
            ..self.clone()
        }
    }

    /// Creates a Firm: an open account owned by `login_email`.
    pub async fn create(&self, name: &str, login_email: &str) -> Result<FirmId, FirmError> {
        Accounts::new(self.pool.clone(), self.clock.clone())
            .create(name, login_email)
            .await
            .map_err(FirmError::from)
    }

    /// The Firm `id`.
    pub async fn get(&self, id: FirmId) -> Result<Firm, FirmError> {
        let row = sqlx::query!(
            r#"select a.name, m.email, f.cut_off_offset_days as "cut_off_offset_days?"
               from accounts a
               join account_members m on m.tenant_id = a.tenant_id and m.role = 'owner'
               left join firms f on f.id = a.tenant_id
               where a.tenant_id = $1"#,
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(FirmError::NotFound)?;
        Ok(Firm {
            id,
            name: row.name,
            login_email: row.email,
            cut_off_offset_days: row
                .cut_off_offset_days
                .and_then(|days| u8::try_from(days).ok())
                .unwrap_or(DEFAULT_CUT_OFF_OFFSET_DAYS),
        })
    }

    /// Sets the Firm's default cut-off offset (0 to 30 days).
    pub async fn set_cut_off_offset(&self, id: FirmId, days: u8) -> Result<(), FirmError> {
        if days > 30 {
            return Err(FirmError::Invalid("the cut-off offset is 0 to 30 days".to_owned()));
        }
        let mut tx = self.pool.begin().await?;
        // Kompleto's row is made the first time a setting changes.
        let updated = sqlx::query!(
            "insert into firms (id, cut_off_offset_days)
             select tenant_id, $2 from accounts where tenant_id = $1
             on conflict (id) do update set cut_off_offset_days = excluded.cut_off_offset_days",
            id.as_uuid(),
            i16::from(days)
        )
        .execute(&mut *tx)
        .await?;
        if updated.rows_affected() == 0 {
            return Err(FirmError::NotFound);
        }
        let details = serde_json::json!({ "days": days });
        let subject = ("firm", id.as_uuid());
        record_as(
            &mut tx,
            id,
            self.actor.as_deref(),
            self.clock.now(),
            "firm.cut_off_changed",
            subject,
            details,
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// The Firm that logs in with `email`, ignoring case and spaces. Login
    /// has no Firm yet, so this is the one lookup not scoped by a `FirmId`.
    pub async fn find_by_login_email(&self, email: &str) -> Result<Option<Firm>, FirmError> {
        let id = sqlx::query_scalar!(
            "select a.tenant_id from accounts a
             join account_members m on m.tenant_id = a.tenant_id and m.role = 'owner'
             where lower(m.email) = lower($1) and a.closed_at is null",
            email.trim()
        )
        .fetch_optional(&self.pool)
        .await?;
        match id {
            Some(id) => self.get(id.into()).await.map(Some),
            None => Ok(None),
        }
    }
}

impl From<AccountError> for FirmError {
    fn from(err: AccountError) -> Self {
        match err {
            AccountError::Invalid(reason) => Self::Invalid(reason),
            AccountError::EmailTaken => Self::LoginEmailTaken,
            AccountError::NotFound => Self::NotFound,
            AccountError::Database(err) => Self::Database(err),
        }
    }
}
