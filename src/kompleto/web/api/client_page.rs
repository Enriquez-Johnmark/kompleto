//! One Client's page (B4): its details, reminder schedule, the next 12
//! months' Periods with their Due dates, Cut-off and Owed documents, its
//! checklist, Payors, Uploads and Replies.

use axum::extract::State;
use axum::http::Method;
use axum::routing::{MethodRouter, get};
use axum::{Json, Router};
use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use super::clients::language;
use super::{ApiError, ApiFirm, ApiPath};
use crate::kompleto::checklist::{DEFAULT_ITEMS, Recurs, owed_documents};
use crate::kompleto::clients::ClientId;
use crate::kompleto::periods::{ClientCalendar, period_label};
use crate::kompleto::schedule::{MANILA, planned_drafts};
use crate::kompleto::web::{AppState, ScopedRoute};

/// Every endpoint about one Client's page (A2).
pub(in crate::kompleto::web) fn page_endpoints() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    vec![(
        ScopedRoute {
            method: Method::GET,
            path: "/api/clients/{id}",
            sample_form: "",
        },
        get(page),
    )]
}

pub(super) fn routes() -> Router<AppState> {
    crate::kompleto::web::scoped_router(page_endpoints())
}

/// A Client's page.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ClientDetail {
    pub id: Uuid,
    pub business_name: String,
    pub contact_first_name: String,
    pub email: String,
    /// `taglish` or `english`.
    pub language: String,
    pub forms: Vec<String>,
    pub paused: bool,
    pub schedule: Schedule,
    /// The next 12 months, earliest first.
    pub periods: Vec<PeriodView>,
    pub checklist: Checklist,
    pub payors: Vec<PayorView>,
    /// Newest first.
    pub uploads: Vec<UploadView>,
    /// Newest first.
    pub replies: Vec<ReplySummary>,
}

/// How the Client's reminders are scheduled (D1).
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Schedule {
    /// The Client's own cut-off offset; `null` uses the Firm's.
    pub cut_off_offset_days: Option<u8>,
    /// The Firm's default offset.
    pub firm_offset_days: u8,
    /// Days before the Cut-off that reminders are drafted.
    pub reminder_days: Vec<u8>,
}

/// One collection Period.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PeriodView {
    /// e.g. "October 2026" or "Q4 2026".
    pub label: String,
    /// `YYYY-MM-DD`.
    pub cut_off: String,
    /// The days Drafts are created, `YYYY-MM-DD`.
    pub draft_dates: Vec<String>,
    pub owed: Vec<String>,
    pub filings: Vec<FilingView>,
}

/// One form's filing in a Period.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct FilingView {
    /// e.g. "2551Q".
    pub form: String,
    /// e.g. "Q3 2026".
    pub period: String,
    /// e.g. "2026-Q3", to set a date for this filing.
    pub period_key: String,
    /// `YYYY-MM-DD`.
    pub due: String,
    /// The Firm set this Client's date.
    pub overridden: bool,
    /// A nationwide BIR extension, e.g. "RMC 12-2027".
    pub extension: Option<String>,
}

/// The documents asked for.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Checklist {
    /// Built-in items for the Client's forms.
    pub defaults: Vec<DefaultItem>,
    /// Items the Firm added.
    pub extras: Vec<ExtraItemView>,
}

/// A built-in checklist item.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DefaultItem {
    pub key: String,
    pub label: String,
    /// `every-period`, `quarter-end` or `year-end`.
    pub recurs: String,
    /// The Firm stopped asking for it.
    pub removed: bool,
}

/// An item the Firm added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ExtraItemView {
    pub id: Uuid,
    pub label: String,
    /// `every-period`, `quarter-end` or `year-end`.
    pub recurs: String,
}

/// A business that owes the Client a 2307.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PayorView {
    pub id: Uuid,
    pub name: String,
}

/// A file the Contact uploaded.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct UploadView {
    pub id: Uuid,
    /// The Period's label.
    pub period: String,
    /// The Owed document, or "Something else".
    pub document: String,
    /// The Payor or the Contact's note.
    pub detail: Option<String>,
    pub size_bytes: i32,
    /// RFC 3339.
    pub uploaded_at: String,
    /// RFC 3339, once marked received.
    pub received_at: Option<String>,
    /// The file is gone; only this record remains.
    pub deleted: bool,
    /// When the file is deleted at the latest, RFC 3339.
    pub delete_by: String,
}

/// A Reply in the Client's list.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReplySummary {
    pub id: Uuid,
    /// RFC 3339.
    pub received_at: String,
    /// The Period's label.
    pub period: String,
    /// An out-of-office or similar.
    pub automatic: bool,
    pub subject: Option<String>,
}

pub(super) fn rfc3339(at: OffsetDateTime) -> String {
    at.format(&Rfc3339).unwrap_or_default()
}

fn recurs(recurs: Recurs) -> &'static str {
    match recurs {
        Recurs::EveryPeriod => "every-period",
        Recurs::QuarterEnd => "quarter-end",
        Recurs::YearEnd => "year-end",
    }
}

async fn page(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<Json<ClientDetail>, ApiError> {
    let id = ClientId::from(id);
    let client = state.clients.get(firm, id).await?;
    // The rest are independent: one round trip's wait, not eight.
    let (overrides, edits, firm_details, extra_items, payor_list, upload_list, reply_list) = tokio::try_join!(
        async { Ok::<_, ApiError>(state.clients.due_overrides(firm, id).await?) },
        async { Ok(state.clients.checklist_edits(firm, id).await?) },
        async { Ok(state.firms.get(firm).await?) },
        async { Ok(state.clients.extra_items(firm, id).await?) },
        async { Ok(state.payors.list(firm, id).await?) },
        async { Ok(state.uploads.for_client(firm, id).await?) },
        async { Ok(state.replies.for_client(firm, id).await?) },
    )?;
    let firm_offset = firm_details.cut_off_offset_days;
    let calendar = ClientCalendar {
        calendar: &state.calendar,
        forms: &client.details.forms,
        overrides: &overrides,
        cut_off_offset_days: client.cut_off_offset_days.unwrap_or(firm_offset),
    };
    let today = state.clock.now().to_offset(MANILA).date();
    let periods = calendar
        .upcoming(today)
        .into_iter()
        .map(|period| PeriodView {
            label: period.label.to_string(),
            cut_off: period.cut_off.to_string(),
            draft_dates: planned_drafts(&period, &client.reminder_days)
                .iter()
                .map(|draft| draft.on.to_string())
                .collect(),
            owed: owed_documents(&client.details.forms, period.month, &edits),
            filings: period
                .filings
                .into_iter()
                .map(|filing| FilingView {
                    form: filing.form.code().to_owned(),
                    period: filing.period.to_string(),
                    period_key: filing.period.key(),
                    due: filing.due.to_string(),
                    overridden: filing.overridden,
                    extension: filing.extension,
                })
                .collect(),
        })
        .collect();
    let defaults = DEFAULT_ITEMS
        .iter()
        .filter(|item| item.when_filing.is_empty() || item.when_filing.iter().any(|f| client.details.forms.contains(f)))
        .map(|item| DefaultItem {
            key: item.key.to_owned(),
            label: item.label.to_owned(),
            recurs: recurs(item.recurs).to_owned(),
            removed: edits.removed.contains(item.key),
        })
        .collect();
    let extras = extra_items
        .into_iter()
        .map(|(item_id, item)| ExtraItemView {
            id: item_id.as_uuid(),
            label: item.label,
            recurs: recurs(item.recurs).to_owned(),
        })
        .collect();
    let payors = payor_list
        .into_iter()
        .map(|payor| PayorView {
            id: payor.id.as_uuid(),
            name: payor.name,
        })
        .collect();
    let uploads = upload_list
        .into_iter()
        .rev()
        .map(|upload| UploadView {
            id: upload.id.as_uuid(),
            period: period_label(&upload.period),
            document: upload.label.document().to_owned(),
            detail: upload.label.detail().map(str::to_owned),
            size_bytes: upload.size_bytes,
            uploaded_at: rfc3339(upload.uploaded_at),
            received_at: upload.received_at.map(rfc3339),
            deleted: upload.deleted,
            delete_by: rfc3339(upload.delete_by),
        })
        .collect();
    let replies = reply_list
        .into_iter()
        .map(|reply| ReplySummary {
            id: reply.id.as_uuid(),
            received_at: rfc3339(reply.received_at),
            period: period_label(&reply.period),
            automatic: reply.automatic,
            subject: reply.subject,
        })
        .collect();
    Ok(Json(ClientDetail {
        id: client.id.as_uuid(),
        business_name: client.details.business_name,
        contact_first_name: client.details.contact_first_name,
        email: client.details.email,
        language: language(client.details.language).to_owned(),
        forms: client.details.forms.iter().map(|form| form.code().to_owned()).collect(),
        paused: client.paused,
        schedule: Schedule {
            cut_off_offset_days: client.cut_off_offset_days,
            firm_offset_days: firm_offset,
            reminder_days: client.reminder_days.days().to_vec(),
        },
        periods,
        checklist: Checklist { defaults, extras },
        payors,
        uploads,
        replies,
    }))
}
