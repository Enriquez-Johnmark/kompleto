//! Firms: the bookkeepers who use Kompleto. Each is an engine Tenant. There
//! is no sign-up; we create Firms with `kompleto firm add`.

use std::sync::Arc;

use sqlx::PgPool;

use crate::engine::clock::Clock;
use crate::engine::ids::TenantId;
use crate::engine::outbox::links;

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
}

impl Firms {
    /// A repository over `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Creates a Firm.
    pub async fn create(&self, name: &str, login_email: &str) -> Result<FirmId, FirmError> {
        let (name, login_email) = (name.trim(), login_email.trim());
        if name.is_empty() || name.chars().count() > 100 || name.chars().any(char::is_control) {
            return Err(FirmError::Invalid("a firm name is 1 to 100 characters".to_owned()));
        }
        // The name signs every email; one that reads as a link would make
        // the pre-send checks (D7) refuse them all.
        if !links(name).is_empty() {
            return Err(FirmError::Invalid(format!(
                "{name:?} looks like a web address; use the business name"
            )));
        }
        if !login_email.contains('@') || login_email.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(FirmError::Invalid(format!("{login_email:?} is not an email address")));
        }
        let id = FirmId::new();
        sqlx::query!(
            "insert into firms (id, name, login_email, created_at) values ($1, $2, $3, $4)",
            id.as_uuid(),
            name,
            login_email,
            self.clock.now(),
        )
        .execute(&self.pool)
        .await
        .map_err(|err| match err.as_database_error().and_then(|db| db.constraint()) {
            Some("firms_login_email") => FirmError::LoginEmailTaken,
            _ => FirmError::Database(err),
        })?;
        Ok(id)
    }

    /// The Firm `id`.
    pub async fn get(&self, id: FirmId) -> Result<Firm, FirmError> {
        let row = sqlx::query!(
            "select id, name, login_email, cut_off_offset_days from firms where id = $1",
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(FirmError::NotFound)?;
        Ok(Firm {
            id: row.id.into(),
            name: row.name,
            login_email: row.login_email,
            cut_off_offset_days: u8::try_from(row.cut_off_offset_days).unwrap_or(5),
        })
    }

    /// Sets the Firm's default cut-off offset (0 to 30 days).
    pub async fn set_cut_off_offset(&self, id: FirmId, days: u8) -> Result<(), FirmError> {
        if days > 30 {
            return Err(FirmError::Invalid("the cut-off offset is 0 to 30 days".to_owned()));
        }
        let updated = sqlx::query!(
            "update firms set cut_off_offset_days = $2 where id = $1",
            id.as_uuid(),
            i16::from(days)
        )
        .execute(&self.pool)
        .await?;
        if updated.rows_affected() == 0 {
            return Err(FirmError::NotFound);
        }
        Ok(())
    }

    /// The Firm that logs in with `email`, ignoring case and spaces. Login
    /// has no Firm yet, so this is the one lookup not scoped by a `FirmId`.
    pub async fn find_by_login_email(&self, email: &str) -> Result<Option<Firm>, FirmError> {
        let id = sqlx::query_scalar!(
            "select id from firms where lower(login_email) = lower($1)",
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
