//! The scheduler (D1): each run creates the Drafts that are due, as fixed
//! templates in the Client's language. Every Draft waits for the
//! bookkeeper's approval; nothing here approves or sends. M3 replaces the
//! template text with AI drafts, keeping the template as the fallback.
//!
//! - Only the latest Draft due is created; a missed earlier one is skipped,
//!   so a Client never gets a burst of catch-up messages.
//! - A newly added Client is chased from the first Period whose Cut-off is
//!   after the day it was added.
//! - Each Draft is recorded in `scheduled_drafts` in the same transaction
//!   that creates it; that table's constraints allow at most four per
//!   Client per Period.
//!
//! This is a system-wide worker: it reads every Firm's Clients, and names
//! the Firm in every row it writes.

use std::sync::Arc;

use sqlx::PgPool;

use super::calendar::{Calendar, CalendarError};
use super::checklist::owed_documents;
use super::clients::{Client, ClientError, ClientId, Clients, Language};
use super::dates::show_date;
use super::firms::FirmId;
use super::periods::{ClientCalendar, CollectionPeriod};
use super::schedule::{MANILA, MAX_DRAFTS_PER_PERIOD, PlannedDraft, Slot, appears_at, planned_drafts};
use crate::engine::clock::Clock;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};

/// Scheduler errors.
#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Reading a Client failed.
    #[error(transparent)]
    Client(#[from] ClientError),
    /// The outbox refused a Draft.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// Creates the Drafts that are due.
pub struct Scheduler {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    calendar: Calendar,
    clients: Clients,
    outbox: Outbox,
}

impl Scheduler {
    /// A scheduler using the built-in calendar.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Result<Self, CalendarError> {
        Ok(Self {
            calendar: Calendar::built_in()?,
            clients: Clients::new(pool.clone(), clock.clone()),
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
        })
    }

    /// Creates every Draft due now; returns how many. Safe to run again.
    /// One Client's failure is logged and doesn't stop the others.
    pub async fn run_once(&self) -> Result<usize, SchedulerError> {
        let rows = sqlx::query_as!(
            SweepRow,
            "select c.id, c.firm_id, c.created_at, f.name as firm_name, f.cut_off_offset_days as firm_offset
             from clients c join firms f on f.id = c.firm_id
             where not c.paused
             order by c.created_at, c.id"
        )
        .fetch_all(&self.pool)
        .await?;
        let mut created = 0;
        for row in rows {
            match self.schedule_client(&row).await {
                Ok(n) => created += n,
                Err(err) => {
                    tracing::error!(firm = %row.firm_id, client = %row.id, %err, "creating a reminder draft failed")
                }
            }
        }
        Ok(created)
    }

    /// Creates the Drafts due now for one Client.
    async fn schedule_client(&self, row: &SweepRow) -> Result<usize, SchedulerError> {
        let (firm, id) = (FirmId::from(row.firm_id), ClientId::from(row.id));
        let client = self.clients.get(firm, id).await?;
        let overrides = self.clients.due_overrides(firm, id).await?;
        let calendar = ClientCalendar {
            calendar: &self.calendar,
            forms: &client.details.forms,
            overrides: &overrides,
            cut_off_offset_days: client
                .cut_off_offset_days
                .unwrap_or_else(|| u8::try_from(row.firm_offset).unwrap_or(5)),
        };
        let now = self.clock.now();
        let added_on = row.created_at.to_offset(MANILA).date();
        let mut created = 0;
        for period in calendar.upcoming(now.to_offset(MANILA).date()) {
            if period.cut_off <= added_on {
                continue;
            }
            let latest_due = planned_drafts(&period, &client.reminder_days)
                .into_iter()
                .rfind(|draft| appears_at(draft.on) <= now);
            if let Some(draft) = latest_due
                && self.create(&row.firm_name, &client, firm, &period, draft).await?
            {
                created += 1;
            }
        }
        Ok(created)
    }

    /// Creates the slot's Draft and records it, in one transaction; false if
    /// the slot already has one or the Period has had its four.
    async fn create(
        &self,
        firm_name: &str,
        client: &Client,
        firm: FirmId,
        period: &CollectionPeriod,
        draft: PlannedDraft,
    ) -> Result<bool, SchedulerError> {
        let (key, slot) = (period.key(), slot_key(draft.slot));
        let exists = sqlx::query_scalar!(
            r#"select exists (select 1 from scheduled_drafts
                              where firm_id = $1 and client_id = $2 and period = $3 and slot = $4) as "exists!""#,
            firm.as_uuid(),
            client.id.as_uuid(),
            key,
            slot,
        )
        .fetch_one(&self.pool)
        .await?;
        if exists {
            return Ok(false);
        }
        let edits = self.clients.checklist_edits(firm, client.id).await?;
        let owed = owed_documents(&client.details.forms, period.month, &edits);
        let mut tx = self.pool.begin().await?;
        let message = self
            .outbox
            .create_draft_in(&mut tx, firm, &template(firm_name, client, period, draft.slot, &owed))
            .await?;
        // A concurrent run that recorded this slot first, or a full Period,
        // makes this insert a no-op, and the Draft rolls back with it.
        let recorded = sqlx::query_scalar!(
            r#"insert into scheduled_drafts (firm_id, client_id, period, slot, ordinal, message_id, created_at)
               select $1, $2, $3, $4, count(*)::smallint, $5, $6
               from scheduled_drafts where client_id = $2 and period = $3
               having count(*) < $7
               on conflict do nothing
               returning 1 as "recorded!""#,
            firm.as_uuid(),
            client.id.as_uuid(),
            key,
            slot,
            message.as_uuid(),
            self.clock.now(),
            i64::try_from(MAX_DRAFTS_PER_PERIOD).unwrap_or(4),
        )
        .fetch_optional(&mut *tx)
        .await?;
        if recorded.is_none() {
            tx.rollback().await?;
            return Ok(false);
        }
        tx.commit().await?;
        Ok(true)
    }
}

/// One Client as the sweep sees it.
struct SweepRow {
    id: uuid::Uuid,
    firm_id: uuid::Uuid,
    created_at: time::OffsetDateTime,
    firm_name: String,
    firm_offset: i16,
}

fn slot_key(slot: Slot) -> String {
    match slot {
        Slot::FirstRequest => "first".to_owned(),
        Slot::Reminder { days_before } => format!("before-{days_before}"),
    }
}

/// The fixed-template Draft for one slot.
fn template(firm_name: &str, client: &Client, period: &CollectionPeriod, slot: Slot, owed: &[String]) -> Email {
    let (name, label, cut_off) = (
        &client.details.contact_first_name,
        period.label,
        show_date(period.cut_off),
    );
    let list: String = owed.iter().map(|item| format!("- {item}\n")).collect();
    let reminder = matches!(slot, Slot::Reminder { .. });
    let (subject, body) = match client.details.language {
        Language::Taglish => (
            format!("Mga dokumento para sa {label}, bago {cut_off}"),
            format!(
                "Hi {name},\n\n{}Para sa {label} BIR filings ninyo, paki-send po ang mga ito bago {cut_off}:\n\n\
                 {list}\nSalamat po!\n{firm_name}\n",
                if reminder { "Paalala lang po. " } else { "" }
            ),
        ),
        Language::English => (
            format!("Documents for {label}, by {cut_off}"),
            format!(
                "Hi {name},\n\n{}For your {label} BIR filings, please send us these by {cut_off}:\n\n\
                 {list}\nThank you!\n{firm_name}\n",
                if reminder { "A quick reminder. " } else { "" }
            ),
        ),
    };
    Email {
        to_address: client.details.email.clone(),
        from_name: firm_name.to_owned(),
        reply_to: None,
        subject,
        body_text: body,
    }
}
