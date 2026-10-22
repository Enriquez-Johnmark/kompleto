//! Requests for an account from the public sign-up page (M10, L1). A request
//! creates nothing and sends nothing: the operator approves it from the
//! command line, which creates the Firm. The answer to a request is the same
//! whether or not the address already has an account.
//!
//! Requests belong to no Firm, so these queries aren't scoped by one; only
//! the operator's command reads them.

use std::sync::Arc;

use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use super::firms::{FirmError, FirmId, check_name, create_in};
use super::members::is_email_address;
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;

id_type!(
    /// One request for an account.
    RequestId
);

/// The most requests waiting at once; beyond it, new ones are refused until
/// the operator catches up.
pub const MAX_PENDING: i64 = 200;
/// The most new requests an hour, so one script can't fill the queue at once.
pub const MAX_PER_HOUR: i64 = 20;
/// Days a request waits before it is deleted unanswered.
pub const EXPIRES_AFTER_DAYS: i64 = 30;

/// One request waiting for the operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRequest {
    /// Its ID.
    pub id: RequestId,
    /// The firm's name, as its Clients will see it.
    pub firm_name: String,
    /// The owner's address.
    pub email: String,
    /// When it was made.
    pub requested_at: OffsetDateTime,
}

/// Sign-up errors.
#[derive(Debug, thiserror::Error)]
pub enum SignupError {
    /// What's wrong with the request, one problem each.
    #[error("{}", .0.join("; "))]
    Invalid(Vec<String>),
    /// Too many requests are waiting.
    #[error("too many requests are waiting")]
    TooMany,
    /// No such request.
    #[error("request not found")]
    NotFound,
    /// Creating the Firm failed.
    #[error(transparent)]
    Firm(#[from] FirmError),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// Account requests.
#[derive(Clone)]
pub struct Signups {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Signups {
    /// Requests in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Records a request for an account for `firm_name`, owned by `email`.
    /// An address that already logs in, or already asked, gets the same
    /// `Ok` and nothing is recorded.
    pub async fn request(&self, firm_name: &str, email: &str, accepted_terms: bool) -> Result<(), SignupError> {
        let (firm_name, email) = (firm_name.trim(), email.trim());
        let mut problems = Vec::new();
        if let Err(problem) = check_name(firm_name) {
            problems.push(format!("Firm name: {problem}"));
        }
        if !is_email_address(email) {
            problems.push(format!("Email: {email:?} is not an email address"));
        }
        if !accepted_terms {
            problems.push("Accept the terms and the data processing agreement".to_owned());
        }
        if !problems.is_empty() {
            return Err(SignupError::Invalid(problems));
        }
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        // One request at a time, so the limits hold under parallel requests.
        sqlx::query!("lock table account_requests in share row exclusive mode")
            .execute(&mut *tx)
            .await?;
        sqlx::query!(
            "delete from account_requests where created_at < $1",
            now - Duration::days(EXPIRES_AFTER_DAYS)
        )
        .execute(&mut *tx)
        .await?;
        let counts = sqlx::query!(
            r#"select count(*) as "pending!", count(*) filter (where created_at > $1) as "last_hour!"
               from account_requests"#,
            now - Duration::hours(1),
        )
        .fetch_one(&mut *tx)
        .await?;
        // Checked before anything about the address, so a full queue answers
        // members and strangers alike.
        if counts.pending >= MAX_PENDING || counts.last_hour >= MAX_PER_HOUR {
            return Err(SignupError::TooMany);
        }
        let member = sqlx::query_scalar!(
            r#"select exists (
                   select 1 from firm_members m join firms f on f.id = m.firm_id and f.closed_at is null
                   where lower(m.email) = lower($1)
               ) as "member!""#,
            email
        )
        .fetch_one(&mut *tx)
        .await?;
        if member {
            return Ok(());
        }
        let now = self.clock.now();
        sqlx::query!(
            "insert into account_requests (id, firm_name, email, accepted_at, created_at)
             values ($1, $2, $3, $4, $4)
             on conflict (lower(email)) do nothing",
            RequestId::new().as_uuid(),
            firm_name,
            email,
            now,
        )
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Requests waiting for the operator, oldest first.
    pub async fn pending(&self) -> Result<Vec<AccountRequest>, SignupError> {
        let rows =
            sqlx::query!("select id, firm_name, email, created_at from account_requests order by created_at, id")
                .fetch_all(&self.pool)
                .await?;
        Ok(rows
            .into_iter()
            .map(|row| AccountRequest {
                id: row.id.into(),
                firm_name: row.firm_name,
                email: row.email,
                requested_at: row.created_at,
            })
            .collect())
    }

    /// Opens the account `id` asked for and deletes the request, together;
    /// returns the new Firm and what the request said.
    pub async fn approve(&self, id: RequestId) -> Result<(FirmId, AccountRequest), SignupError> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query!(
            "select firm_name, email, created_at from account_requests where id = $1 for update",
            id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(SignupError::NotFound)?;
        let request = AccountRequest {
            id,
            firm_name: row.firm_name,
            email: row.email,
            requested_at: row.created_at,
        };
        let firm = create_in(&mut tx, self.clock.now(), &request.firm_name, &request.email).await?;
        sqlx::query!("delete from account_requests where id = $1", id.as_uuid())
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok((firm, request))
    }

    /// Turns down request `id`, deleting it.
    pub async fn decline(&self, id: RequestId) -> Result<(), SignupError> {
        if self.delete(id.as_uuid()).await? {
            Ok(())
        } else {
            Err(SignupError::NotFound)
        }
    }

    /// Turns down every waiting request, e.g. after a flood of junk ones;
    /// returns how many.
    pub async fn decline_all(&self) -> Result<u64, SignupError> {
        let deleted = sqlx::query!("delete from account_requests").execute(&self.pool).await?;
        Ok(deleted.rows_affected())
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let deleted = sqlx::query!("delete from account_requests where id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(deleted.rows_affected() == 1)
    }
}
