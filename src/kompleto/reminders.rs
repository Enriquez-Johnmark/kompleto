//! The scheduler (D1): each run creates the Drafts that are due, written by
//! the model or, failing that, the template (`super::drafting`). Every
//! Draft waits for the bookkeeper's approval; nothing here approves or sends.
//!
//! - Only the latest Draft due is created; a missed earlier one is skipped,
//!   so a Client never gets a burst of catch-up messages.
//! - A newly added Client is chased from the first Period whose Cut-off is
//!   after the day it was added.
//! - An unapproved Draft expires at the end of its Period's Cut-off day
//!   (D4); each run also sweeps overdue Drafts to `expired`.
//! - A Draft lists only the Owed documents not yet uploaded; once all
//!   have been, no more are created for the Period and any waiting one is
//!   cancelled (G2).
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
use super::clients::{Client, ClientError, ClientId, Clients};
use super::drafting::{DraftBrief, Drafter, Tone};
use super::drafts::Drafts;
use super::firms::FirmId;
use super::periods::{ClientCalendar, CollectionPeriod};
use super::schedule::{MANILA, MAX_DRAFTS_PER_PERIOD, PlannedDraft, Slot, appears_at, planned_drafts};
use super::upload_links::UploadLinks;
use super::uploads::uploaded_documents;
use crate::engine::clock::Clock;
use crate::engine::llm::AnthropicClient;
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
    drafter: Drafter<AnthropicClient>,
    links: UploadLinks,
    drafts: Drafts,
}

impl Scheduler {
    /// A scheduler that writes every Draft from the template, with links
    /// from `links`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, links: UploadLinks) -> Result<Self, CalendarError> {
        Self::with_model(pool, clock, links, None)
    }

    /// A scheduler that asks `model` for each Draft's wording, when there is one.
    pub fn with_model(
        pool: PgPool,
        clock: Arc<dyn Clock>,
        links: UploadLinks,
        model: Option<AnthropicClient>,
    ) -> Result<Self, CalendarError> {
        Ok(Self {
            links,
            drafts: Drafts::new(pool.clone(), clock.clone()),
            drafter: Drafter::new(pool.clone(), clock.clone(), model),
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
        let expired = self.outbox.expire_overdue().await?;
        if expired > 0 {
            tracing::info!(expired, "unapproved drafts expired at their cut-off");
        }
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
        let edits = self.clients.checklist_edits(firm, client.id).await?;
        let uploaded = uploaded_documents(&self.pool, firm, client.id, &key).await?;
        let owed: Vec<String> = owed_documents(&client.details.forms, period.month, &edits)
            .into_iter()
            .filter(|document| !uploaded.contains(document))
            .collect();
        // Checked on every run, even when this slot already has its Draft, so
        // a Draft left waiting after the last upload (say, one drafted while
        // it arrived) is cancelled by the next run.
        if owed.is_empty() {
            self.drafts.cancel_waiting(firm, client.id, &key).await?;
            return Ok(false);
        }
        // Checked before drafting, so a slot that can't be recorded never
        // costs a model call.
        let taken = sqlx::query!(
            r#"select count(*) as "drafts!", coalesce(bool_or(slot = $4), false) as "this_slot!"
               from scheduled_drafts where firm_id = $1 and client_id = $2 and period = $3"#,
            firm.as_uuid(),
            client.id.as_uuid(),
            key,
            slot,
        )
        .fetch_one(&self.pool)
        .await?;
        if taken.this_slot || taken.drafts >= i64::try_from(MAX_DRAFTS_PER_PERIOD).unwrap_or(4) {
            return Ok(false);
        }
        let link = self.links.issue(firm, client.id, &key, period.cut_off).await?;
        let brief = DraftBrief {
            business_name: client.details.business_name.clone(),
            contact_first_name: client.details.contact_first_name.clone(),
            owed,
            period: period.label.to_string(),
            cut_off: period.cut_off,
            language: client.details.language,
            tone: Tone::for_reminder(match draft.slot {
                Slot::FirstRequest => None,
                Slot::Reminder { days_before } => Some(days_before),
            }),
            upload_link: link.url,
        };
        // Drafted before the transaction, so a slow model holds no locks.
        let text = self.drafter.draft(firm, firm_name, &brief).await;
        let email = Email {
            to_address: client.details.email.clone(),
            from_name: firm_name.to_owned(),
            reply_to: link.reply_to,
            subject: text.subject,
            body_text: text.body,
        };
        let mut tx = self.pool.begin().await?;
        // D4: unapproved, it expires at the end of the Cut-off day (Manila).
        let expires_at = (period.cut_off + time::Duration::days(1))
            .midnight()
            .assume_offset(MANILA);
        let message = self
            .outbox
            .create_draft_in(&mut tx, firm, &email, Some(expires_at))
            .await?;
        // A concurrent run that recorded this slot first, or a full Period,
        // makes this insert a no-op, and the Draft rolls back with it.
        let recorded = sqlx::query_scalar!(
            r#"insert into scheduled_drafts
                   (firm_id, client_id, period, slot, ordinal, message_id, created_at, source, cut_off)
               select $1, $2, $3, $4, count(*)::smallint, $5, $6, $8, $9
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
            text.source.as_str(),
            period.cut_off,
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
