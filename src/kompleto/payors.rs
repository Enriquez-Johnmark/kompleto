//! Payors (H2): for each Client, the businesses expected to send it a BIR
//! Form 2307 each quarter. Names only, never TINs. Every query is scoped by
//! the Firm (A2).

use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use super::clients::{ClientError, ClientId};
use super::firms::FirmId;
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::outbox::links;

id_type!(
    /// One expected Payor of one Client.
    PayorId
);

/// The longest Payor name.
pub const MAX_NAME_CHARS: usize = 100;

/// One expected Payor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payor {
    /// Its ID.
    pub id: PayorId,
    /// The business's name, as the Firm wrote it.
    pub name: String,
}

/// The Payor lists of a Firm's Clients.
#[derive(Clone)]
pub struct Payors {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Payors {
    /// Payor lists in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// A Client's Payors, by name.
    pub async fn list(&self, firm: FirmId, client: ClientId) -> Result<Vec<Payor>, ClientError> {
        let rows = sqlx::query!(
            "select id, name from payors where firm_id = $1 and client_id = $2 order by lower(name)",
            firm.as_uuid(),
            client.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Payor {
                id: row.id.into(),
                name: row.name,
            })
            .collect())
    }

    /// Adds `name` to a Client's Payors.
    pub async fn add(&self, firm: FirmId, client: ClientId, name: &str) -> Result<PayorId, ClientError> {
        let name = name.trim();
        if name.is_empty() || name.chars().count() > MAX_NAME_CHARS || name.chars().any(char::is_control) {
            return Err(ClientError::Invalid(format!(
                "a Payor's name is 1 to {MAX_NAME_CHARS} characters"
            )));
        }
        // It appears in Reminders, which carry no link but the upload link.
        if !links(name).is_empty() {
            return Err(ClientError::Invalid(
                "a Payor's name can't look like a web address".to_owned(),
            ));
        }
        let id = PayorId::new();
        let added = sqlx::query!(
            "insert into payors (id, firm_id, client_id, name, created_at)
             select $1, $2, $3, $4, $5
             where exists (select 1 from clients where firm_id = $2 and id = $3)
             on conflict do nothing",
            id.as_uuid(),
            firm.as_uuid(),
            client.as_uuid(),
            name,
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        if added.rows_affected() == 1 {
            return Ok(id);
        }
        // Either the Client isn't this Firm's, or the name is listed already.
        let listed = self.list(firm, client).await?;
        if listed.iter().any(|payor| payor.name.eq_ignore_ascii_case(name)) {
            Err(ClientError::DuplicateName(name.to_owned()))
        } else {
            Err(ClientError::NotFound)
        }
    }

    /// Removes a Payor from a Client's list.
    pub async fn remove(&self, firm: FirmId, client: ClientId, payor: PayorId) -> Result<(), ClientError> {
        let removed = sqlx::query!(
            "delete from payors where firm_id = $1 and client_id = $2 and id = $3",
            firm.as_uuid(),
            client.as_uuid(),
            payor.as_uuid(),
        )
        .execute(&self.pool)
        .await?;
        if removed.rows_affected() == 0 {
            return Err(ClientError::NotFound);
        }
        Ok(())
    }
}
