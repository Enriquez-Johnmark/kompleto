//! The people who log in to a Firm (M9): one owner, whose address is the
//! Firm's login address, and up to 5 staff. Every query is scoped by the
//! Firm (A2).

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;
use time::OffsetDateTime;

use super::activity::record_by;
use super::firms::FirmId;
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;

id_type!(
    /// One person who logs in to a Firm.
    MemberId
);

/// What a member may do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Everything, including exporting data, closing the account and
    /// managing staff.
    Owner,
    /// The daily work: Clients, Drafts, Uploads and Replies.
    Staff,
}

impl Role {
    /// `owner` or `staff`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Staff => "staff",
        }
    }

    pub(crate) fn parse(text: &str) -> Self {
        if text == "owner" { Self::Owner } else { Self::Staff }
    }
}

/// One person who logs in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// Their ID.
    pub id: MemberId,
    /// The address their login links go to.
    pub email: String,
    /// What they may do.
    pub role: Role,
    /// When they were added.
    pub added_at: OffsetDateTime,
}

/// The most staff a Firm can have, besides its owner.
pub const MAX_STAFF: i64 = 5;

/// Member errors.
#[derive(Debug, thiserror::Error)]
pub enum MemberError {
    /// Not an email address.
    #[error("{0}")]
    Invalid(String),
    /// The address already logs in to a Firm.
    #[error("that address already logs in to Kompleto")]
    EmailTaken,
    /// The Firm already has its 5 staff.
    #[error("a firm can have at most 5 staff")]
    TooManyStaff,
    /// No such member of this Firm, or the Firm is closed.
    #[error("member not found")]
    NotFound,
    /// The owner can't be removed.
    #[error("the owner can't be removed")]
    IsOwner,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// The members of each Firm.
#[derive(Clone)]
pub struct Members {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Members {
    /// Members in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// `firm`'s members, owner first, then staff by when they were added.
    pub async fn list(&self, firm: FirmId) -> Result<Vec<Member>, MemberError> {
        let rows = sqlx::query!(
            "select id, email, role, created_at from firm_members where firm_id = $1
             order by role = 'owner' desc, created_at, id",
            firm.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Member {
                id: row.id.into(),
                email: row.email,
                role: Role::parse(&row.role),
                added_at: row.created_at,
            })
            .collect())
    }

    /// Adds `email` to `firm`'s staff; `by` is the owner's address, for the
    /// activity log.
    pub async fn add_staff(&self, firm: FirmId, email: &str, by: &str) -> Result<Member, MemberError> {
        let email = email.trim();
        if !is_email_address(email) {
            return Err(MemberError::Invalid(format!("{email:?} is not an email address")));
        }
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        // One change per Firm at a time, so two can't both squeeze under the limit.
        sqlx::query!(
            "select id from firms where id = $1 and closed_at is null for update",
            firm.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(MemberError::NotFound)?;
        let staff = sqlx::query_scalar!(
            r#"select count(*) as "count!" from firm_members where firm_id = $1 and role = 'staff'"#,
            firm.as_uuid()
        )
        .fetch_one(&mut *tx)
        .await?;
        if staff >= MAX_STAFF {
            return Err(MemberError::TooManyStaff);
        }
        let id = MemberId::new();
        sqlx::query!(
            "insert into firm_members (id, firm_id, email, role, created_at) values ($1, $2, $3, 'staff', $4)",
            id.as_uuid(),
            firm.as_uuid(),
            email,
            now,
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| match err.as_database_error().and_then(|db| db.constraint()) {
            Some("firm_members_email") => MemberError::EmailTaken,
            _ => MemberError::Database(err),
        })?;
        record_by(
            &mut tx,
            firm,
            by,
            now,
            "member.added",
            ("member", id.as_uuid()),
            json!({ "email": email }),
        )
        .await?;
        tx.commit().await?;
        Ok(Member {
            id,
            email: email.to_owned(),
            role: Role::Staff,
            added_at: now,
        })
    }

    /// Removes a staff member from `firm`; their sessions and login links
    /// go with them. `by` is the owner's address, for the activity log.
    pub async fn remove_staff(&self, firm: FirmId, member: MemberId, by: &str) -> Result<(), MemberError> {
        let mut tx = self.pool.begin().await?;
        let role = sqlx::query_scalar!(
            "select role from firm_members where firm_id = $1 and id = $2 for update",
            firm.as_uuid(),
            member.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(MemberError::NotFound)?;
        if Role::parse(&role) == Role::Owner {
            return Err(MemberError::IsOwner);
        }
        let email = sqlx::query_scalar!(
            "delete from firm_members where firm_id = $1 and id = $2 returning email",
            firm.as_uuid(),
            member.as_uuid(),
        )
        .fetch_one(&mut *tx)
        .await?;
        record_by(
            &mut tx,
            firm,
            by,
            self.clock.now(),
            "member.removed",
            ("member", member.as_uuid()),
            json!({ "email": email }),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

/// Whether `email` looks like an address a login link can go to.
pub(crate) fn is_email_address(email: &str) -> bool {
    email.len() <= 254 && email.contains('@') && !email.chars().any(|c| c.is_whitespace() || c.is_control())
}
