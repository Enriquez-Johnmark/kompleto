//! The Client repository. Every function takes the Firm's ID, and every
//! query filters on it (A2).

use std::sync::Arc;

use serde_json::json;
use sqlx::{PgConnection, PgPool};
use time::Date;
use uuid::Uuid;

use super::{Language, NewClient};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::kompleto::activity::record_by_firm;
use crate::kompleto::calendar::{BirForm, TaxPeriod};
use crate::kompleto::checklist::{ChecklistEdits, DEFAULT_ITEMS, ExtraItem, Recurs};
use crate::kompleto::firms::FirmId;
use crate::kompleto::periods::DueOverrides;
use crate::kompleto::schedule::ReminderDays;

id_type!(
    /// One of a Firm's Clients.
    ClientId
);
id_type!(
    /// A checklist item added for one Client.
    ExtraItemId
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

/// How a Client's reminders are scheduled (D1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleSettings {
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
    /// A value that can't be stored, with the reason.
    #[error("{0}")]
    Invalid(String),
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
        let mut tx = self.pool.begin().await?;
        let id = self.insert(&mut tx, firm, client).await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Adds every Client in `clients` in one transaction: all of them, or
    /// none if any fails (B2).
    pub async fn create_all(&self, firm: FirmId, clients: &[NewClient]) -> Result<Vec<ClientId>, ClientError> {
        let mut tx = self.pool.begin().await?;
        let mut ids = Vec::with_capacity(clients.len());
        for client in clients {
            ids.push(self.insert(&mut tx, firm, client).await?);
        }
        tx.commit().await?;
        Ok(ids)
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
        let mut tx = self.pool.begin().await?;
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
        .execute(&mut *tx)
        .await
        .map_err(|err| duplicate_name(err, client))?;
        if updated.rows_affected() == 0 {
            return Err(ClientError::NotFound);
        }
        let subject = ("client", id.as_uuid());
        record_by_firm(&mut tx, firm, self.clock.now(), "client.updated", subject, json!({})).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Changes how a Client's reminders are scheduled. Drafts already
    /// created stay as they are.
    pub async fn set_schedule(
        &self,
        firm: FirmId,
        id: ClientId,
        settings: &ScheduleSettings,
    ) -> Result<(), ClientError> {
        let days: Vec<i16> = settings.reminder_days.days().iter().map(|&d| i16::from(d)).collect();
        let mut tx = self.pool.begin().await?;
        let updated = sqlx::query!(
            "update clients set cut_off_offset_days = $3, reminder_days = $4, paused = $5, updated_at = $6
             where firm_id = $1 and id = $2",
            firm.as_uuid(),
            id.as_uuid(),
            settings.cut_off_offset_days.map(i16::from),
            &days,
            settings.paused,
            self.clock.now(),
        )
        .execute(&mut *tx)
        .await?;
        if updated.rows_affected() == 0 {
            return Err(ClientError::NotFound);
        }
        let details = json!({ "paused": settings.paused });
        let subject = ("client", id.as_uuid());
        record_by_firm(
            &mut tx,
            firm,
            self.clock.now(),
            "client.schedule_changed",
            subject,
            details,
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Sets (or with `None`, removes) the Client's own Due date for one
    /// form and Period.
    pub async fn set_due_override(
        &self,
        firm: FirmId,
        id: ClientId,
        form: BirForm,
        period: TaxPeriod,
        due: Option<Date>,
    ) -> Result<(), ClientError> {
        self.ensure_exists(firm, id).await?;
        let mut tx = self.pool.begin().await?;
        match due {
            Some(due) => {
                sqlx::query!(
                    "insert into due_date_overrides (firm_id, client_id, form, period, due_date)
                 values ($1, $2, $3, $4, $5)
                 on conflict (client_id, form, period) do update set due_date = excluded.due_date",
                    firm.as_uuid(),
                    id.as_uuid(),
                    form.code(),
                    period.key(),
                    due,
                )
                .execute(&mut *tx)
                .await?
            }
            None => sqlx::query!(
                "delete from due_date_overrides where firm_id = $1 and client_id = $2 and form = $3 and period = $4",
                firm.as_uuid(),
                id.as_uuid(),
                form.code(),
                period.key(),
            )
            .execute(&mut *tx)
            .await?,
        };
        let details = json!({ "form": form.code(), "period": period.key(), "due": due.map(|d| d.to_string()) });
        let subject = ("client", id.as_uuid());
        record_by_firm(&mut tx, firm, self.clock.now(), "client.due_date_set", subject, details).await?;
        tx.commit().await?;
        Ok(())
    }

    /// The Client's own Due dates.
    pub async fn due_overrides(&self, firm: FirmId, id: ClientId) -> Result<DueOverrides, ClientError> {
        self.ensure_exists(firm, id).await?;
        let rows = sqlx::query!(
            "select form, period, due_date from due_date_overrides where firm_id = $1 and client_id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some((
                    (BirForm::parse(&row.form)?, TaxPeriod::parse_key(&row.period)?),
                    row.due_date,
                ))
            })
            .collect())
    }

    /// Stops asking this Client for a default checklist item.
    pub async fn remove_default_item(&self, firm: FirmId, id: ClientId, key: &str) -> Result<(), ClientError> {
        if !DEFAULT_ITEMS.iter().any(|item| item.key == key) {
            return Err(ClientError::Invalid(format!("no checklist item {key:?}")));
        }
        self.ensure_exists(firm, id).await?;
        sqlx::query!(
            "insert into client_removed_items (firm_id, client_id, item_key) values ($1, $2, $3)
             on conflict do nothing",
            firm.as_uuid(),
            id.as_uuid(),
            key,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Asks this Client for a removed default item again.
    pub async fn restore_default_item(&self, firm: FirmId, id: ClientId, key: &str) -> Result<(), ClientError> {
        self.ensure_exists(firm, id).await?;
        sqlx::query!(
            "delete from client_removed_items where firm_id = $1 and client_id = $2 and item_key = $3",
            firm.as_uuid(),
            id.as_uuid(),
            key,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Adds an item to this Client's checklist.
    pub async fn add_extra_item(
        &self,
        firm: FirmId,
        id: ClientId,
        item: &ExtraItem,
    ) -> Result<ExtraItemId, ClientError> {
        let label = item.label.trim();
        if label.is_empty() || label.chars().count() > 120 || label.chars().any(char::is_control) {
            return Err(ClientError::Invalid("an item is 1 to 120 characters".to_owned()));
        }
        self.ensure_exists(firm, id).await?;
        let item_id = ExtraItemId::new();
        sqlx::query!(
            "insert into client_extra_items (id, firm_id, client_id, label, recurs, created_at)
             values ($1, $2, $3, $4, $5, $6)",
            item_id.as_uuid(),
            firm.as_uuid(),
            id.as_uuid(),
            label,
            item.recurs.as_str(),
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        Ok(item_id)
    }

    /// Removes an item added for this Client.
    pub async fn delete_extra_item(&self, firm: FirmId, id: ClientId, item: ExtraItemId) -> Result<(), ClientError> {
        let deleted = sqlx::query!(
            "delete from client_extra_items where firm_id = $1 and client_id = $2 and id = $3",
            firm.as_uuid(),
            id.as_uuid(),
            item.as_uuid(),
        )
        .execute(&self.pool)
        .await?;
        if deleted.rows_affected() == 0 {
            return Err(ClientError::NotFound);
        }
        Ok(())
    }

    /// The items added for this Client, oldest first.
    pub async fn extra_items(&self, firm: FirmId, id: ClientId) -> Result<Vec<(ExtraItemId, ExtraItem)>, ClientError> {
        self.ensure_exists(firm, id).await?;
        let rows = sqlx::query!(
            "select id, label, recurs from client_extra_items
             where firm_id = $1 and client_id = $2 order by created_at, id",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let recurs = Recurs::parse(&row.recurs)?;
                Some((
                    row.id.into(),
                    ExtraItem {
                        label: row.label,
                        recurs,
                    },
                ))
            })
            .collect())
    }

    /// How this Client's checklist differs from the default.
    pub async fn checklist_edits(&self, firm: FirmId, id: ClientId) -> Result<ChecklistEdits, ClientError> {
        let extra = self
            .extra_items(firm, id)
            .await?
            .into_iter()
            .map(|(_, item)| item)
            .collect();
        let removed = sqlx::query_scalar!(
            "select item_key from client_removed_items where firm_id = $1 and client_id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .collect();
        let payors = sqlx::query_scalar!(
            "select name from payors where firm_id = $1 and client_id = $2 order by lower(name)",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ChecklistEdits { removed, extra, payors })
    }

    /// `NotFound` unless the Client exists and is this Firm's.
    async fn ensure_exists(&self, firm: FirmId, id: ClientId) -> Result<(), ClientError> {
        sqlx::query_scalar!(
            "select 1 as \"found!\" from clients where firm_id = $1 and id = $2",
            firm.as_uuid(),
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|_| ())
        .ok_or(ClientError::NotFound)
    }

    async fn insert(&self, tx: &mut PgConnection, firm: FirmId, client: &NewClient) -> Result<ClientId, ClientError> {
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
        .execute(&mut *tx)
        .await
        .map_err(|err| duplicate_name(err, client))?;
        record_by_firm(tx, firm, now, "client.created", ("client", id.as_uuid()), json!({})).await?;
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
