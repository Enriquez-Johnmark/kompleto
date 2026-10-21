//! The Client repository. Every function takes the Firm's ID, and every
//! query filters on it (A2).

use std::sync::Arc;

use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

use super::{Language, NewClient};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::kompleto::calendar::BirForm;
use crate::kompleto::firms::FirmId;
use crate::kompleto::schedule::ReminderDays;

id_type!(
    /// One of a Firm's Clients.
    ClientId
);

/// A Client as stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    /// Its ID.
    pub id: ClientId,
    /// Name, Contact, language and forms.
    pub details: NewClient,
    /// The Client's own cut-off offset; `None` uses the Firm's.
    pub cut_off_offset_days: Option<u8>,
    /// Days before the Cut-off the Client gets reminders.
    pub reminder_days: ReminderDays,
    /// No Drafts are created while paused.
    pub paused: bool,
}

/// Client errors.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// No such Client for this Firm, including another Firm's (A2).
    #[error("client not found")]
    NotFound,
    /// The Firm already has a Client with this business name.
    #[error("you already have a client named {0:?}")]
    DuplicateName(String),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// The Client repository.
#[derive(Clone)]
pub struct Clients {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Clients {
    /// A repository over `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Adds one Client.
    pub async fn create(&self, firm: FirmId, client: &NewClient) -> Result<ClientId, ClientError> {
        self.insert(&self.pool, firm, client).await
    }

    /// The Client `id`, if it's this Firm's.
    pub async fn get(&self, firm: FirmId, id: ClientId) -> Result<Client, ClientError> {
        let row = sqlx::query_as!(
            ClientRow,
            "select id, business_name, contact_first_name, email, language, forms,
                    cut_off_offset_days, reminder_days, paused
             from clients where firm_id = $1 and id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(ClientError::NotFound)?;
        Ok(row.into())
    }

    /// All of the Firm's Clients, by business name.
    pub async fn list(&self, firm: FirmId) -> Result<Vec<Client>, ClientError> {
        let rows = sqlx::query_as!(
            ClientRow,
            "select id, business_name, contact_first_name, email, language, forms,
                    cut_off_offset_days, reminder_days, paused
             from clients where firm_id = $1 order by lower(business_name)",
            firm.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Client::from).collect())
    }

    /// Replaces a Client's name, Contact, language and forms.
    pub async fn update(&self, firm: FirmId, id: ClientId, client: &NewClient) -> Result<(), ClientError> {
        let forms = form_codes(&client.forms);
        let updated = sqlx::query!(
            "update clients set business_name = $3, contact_first_name = $4, email = $5,
                    language = $6, forms = $7, updated_at = $8
             where firm_id = $1 and id = $2",
            firm.as_uuid(),
            id.as_uuid(),
            client.business_name,
            client.contact_first_name,
            client.email,
            client.language.as_str(),
            &forms,
            self.clock.now(),
        )
        .execute(&self.pool)
        .await
        .map_err(|err| duplicate_name(err, client))?;
        if updated.rows_affected() == 0 {
            return Err(ClientError::NotFound);
        }
        Ok(())
    }

    async fn insert(&self, db: impl PgExecutor<'_>, firm: FirmId, client: &NewClient) -> Result<ClientId, ClientError> {
        let id = ClientId::new();
        let now = self.clock.now();
        sqlx::query!(
            "insert into clients (id, firm_id, business_name, contact_first_name, email, language,
                                  forms, created_at, updated_at)
             values ($1, $2, $3, $4, $5, $6, $7, $8, $8)",
            id.as_uuid(),
            firm.as_uuid(),
            client.business_name,
            client.contact_first_name,
            client.email,
            client.language.as_str(),
            &form_codes(&client.forms),
            now,
        )
        .execute(db)
        .await
        .map_err(|err| duplicate_name(err, client))?;
        Ok(id)
    }
}

fn form_codes(forms: &[BirForm]) -> Vec<String> {
    forms.iter().map(|form| form.code().to_owned()).collect()
}

fn duplicate_name(err: sqlx::Error, client: &NewClient) -> ClientError {
    match err.as_database_error().and_then(|db| db.constraint()) {
        Some("clients_business_name") => ClientError::DuplicateName(client.business_name.clone()),
        _ => ClientError::Database(err),
    }
}

struct ClientRow {
    id: Uuid,
    business_name: String,
    contact_first_name: String,
    email: String,
    language: String,
    forms: Vec<String>,
    cut_off_offset_days: Option<i16>,
    reminder_days: Vec<i16>,
    paused: bool,
}

impl From<ClientRow> for Client {
    fn from(row: ClientRow) -> Self {
        let reminder_days: Vec<u8> = row.reminder_days.iter().filter_map(|&d| u8::try_from(d).ok()).collect();
        Self {
            id: row.id.into(),
            details: NewClient {
                business_name: row.business_name,
                contact_first_name: row.contact_first_name,
                email: row.email,
                // The column's check constraint allows only the two names.
                language: Language::parse(&row.language).unwrap_or(Language::English),
                forms: row.forms.iter().filter_map(|code| BirForm::parse(code)).collect(),
            },
            cut_off_offset_days: row.cut_off_offset_days.and_then(|d| u8::try_from(d).ok()),
            reminder_days: ReminderDays::new(&reminder_days).unwrap_or_default(),
            paused: row.paused,
        }
    }
}
