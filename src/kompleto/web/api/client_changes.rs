//! Changes made from a Client's page: its reminder schedule (D1), its own
//! Due dates (B3), the documents asked for (B4) and its Payors (H2).

use axum::extract::State;
use axum::http::{Method, StatusCode};
use axum::routing::{MethodRouter, delete, post, put};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use super::{ApiError, ApiJson, ApiMember, ApiPath};
use crate::kompleto::calendar::{BirForm, TaxPeriod};
use crate::kompleto::checklist::{ExtraItem, Recurs};
use crate::kompleto::clients::{ClientError, ClientId, ExtraItemId, ScheduleSettings};
use crate::kompleto::import::payors::MAX_NAMES;
use crate::kompleto::payors::PayorId;
use crate::kompleto::payroll::{PAGIBIG_DEFAULT_DAY, PHILHEALTH_DEFAULT_DAY, PayCycle, PayrollSettings};
use crate::kompleto::schedule::ReminderDays;
use crate::kompleto::web::{AppState, ScopedRoute, scoped_router};

pub(super) fn routes() -> Router<AppState> {
    scoped_router(change_endpoints())
}

/// Every endpoint changing one Client (A2).
pub(in crate::kompleto::web) fn change_endpoints() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    let endpoint = |method, path, sample_form| ScopedRoute {
        method,
        path,
        sample_form,
    };
    vec![
        (
            endpoint(
                Method::PUT,
                "/api/clients/{id}/schedule",
                r#"{"cut_off_offset_days": null, "reminder_days": [7, 3, 1], "paused": false}"#,
            ),
            put(set_schedule),
        ),
        (
            endpoint(
                Method::PUT,
                "/api/clients/{id}/due-dates",
                r#"{"form": "2551Q", "period": "2026-Q4", "due": "2027-01-29"}"#,
            ),
            put(set_due_date),
        ),
        (
            endpoint(
                Method::POST,
                "/api/clients/{id}/checklist/remove",
                r#"{"key": "bank-statements"}"#,
            ),
            post(remove_item),
        ),
        (
            endpoint(
                Method::POST,
                "/api/clients/{id}/checklist/restore",
                r#"{"key": "bank-statements"}"#,
            ),
            post(restore_item),
        ),
        (
            endpoint(
                Method::POST,
                "/api/clients/{id}/checklist/extras",
                r#"{"label": "Lazada payouts", "recurs": "every-period"}"#,
            ),
            post(add_item),
        ),
        (
            endpoint(Method::DELETE, "/api/clients/{id}/checklist/extras/{item}", ""),
            delete(delete_item),
        ),
        (
            endpoint(
                Method::PUT,
                "/api/clients/{id}/payroll",
                r#"{"pay_cycle": "monthly", "philhealth_due_day": 20, "pagibig_due_day": 31}"#,
            ),
            put(set_payroll),
        ),
        (
            endpoint(
                Method::POST,
                "/api/clients/{id}/payors",
                r#"{"name": "Puregold Price Club"}"#,
            ),
            post(add_payor),
        ),
        (
            endpoint(
                Method::POST,
                "/api/clients/{id}/payors/batch",
                r#"{"names": ["Puregold Price Club"]}"#,
            ),
            post(add_payors),
        ),
        (
            endpoint(Method::DELETE, "/api/clients/{id}/payors/{payor}", ""),
            delete(remove_payor),
        ),
    ]
}

/// A refusal in the bookkeeper's words.
fn refused(reason: impl Into<String>) -> ApiError {
    ApiError::invalid(reason)
}

fn client_refusal(err: ClientError) -> ApiError {
    match err {
        ClientError::Invalid(reason) => refused(reason),
        err @ ClientError::DuplicateName(_) => refused(format!("{err}.")),
        other => other.into(),
    }
}

/// How a Client's reminders are scheduled.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ScheduleChange {
    /// 0 to 30 days; `null` uses the Firm's default.
    pub cut_off_offset_days: Option<u8>,
    /// Up to 3 days, each 1 to 14 days before the Cut-off.
    pub reminder_days: Vec<u8>,
    pub paused: bool,
}

async fn set_schedule(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(change): ApiJson<ScheduleChange>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    let id = ClientId::from(id);
    state.clients.get(firm, id).await?;
    if change.cut_off_offset_days.is_some_and(|days| days > 30) {
        return Err(refused("The cut-off offset is 0 to 30 days."));
    }
    let reminder_days = ReminderDays::new(&change.reminder_days).map_err(|err| refused(err.to_string()))?;
    let settings = ScheduleSettings {
        cut_off_offset_days: change.cut_off_offset_days,
        reminder_days,
        paused: change.paused,
    };
    state
        .clients
        .by(&member.email)
        .set_schedule(firm, id, &settings)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// One filing's Due date for this Client only.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DueDateChange {
    /// e.g. "2551Q".
    pub form: String,
    /// The filing's period key, e.g. "2026-Q4".
    pub period: String,
    /// `YYYY-MM-DD`; `null` goes back to the calendar's date.
    pub due: Option<String>,
}

async fn set_due_date(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(change): ApiJson<DueDateChange>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    let id = ClientId::from(id);
    state.clients.get(firm, id).await?;
    let (form, period) = BirForm::parse(&change.form)
        .zip(TaxPeriod::parse_key(&change.period))
        .filter(|(form, period)| state.calendar.due_date(*form, *period).is_some())
        .ok_or_else(|| refused("That form isn't filed for that period."))?;
    let due = match change.due.as_deref().map(str::trim) {
        None | Some("") => None,
        Some(text) => Some(
            Date::parse(text, time::macros::format_description!("[year]-[month]-[day]"))
                .map_err(|_| refused(format!("{text:?} isn't a date.")))?,
        ),
    };
    state
        .clients
        .by(&member.email)
        .set_due_override(firm, id, form, period, due)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// A built-in checklist item, by its key.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ItemKey {
    pub key: String,
}

async fn remove_item(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(item): ApiJson<ItemKey>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    state
        .clients
        .by(&member.email)
        .remove_default_item(firm, ClientId::from(id), &item.key)
        .await
        .map_err(client_refusal)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn restore_item(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(item): ApiJson<ItemKey>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    state
        .clients
        .by(&member.email)
        .restore_default_item(firm, ClientId::from(id), &item.key)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// A document to ask for besides the built-in ones.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewItem {
    pub label: String,
    /// `every-period`, `quarter-end` or `year-end`.
    pub recurs: String,
}

async fn add_item(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(item): ApiJson<NewItem>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    let id = ClientId::from(id);
    state.clients.get(firm, id).await?;
    let recurs = Recurs::parse(&item.recurs).ok_or_else(|| refused("Choose when the document is owed."))?;
    let item = ExtraItem {
        label: item.label,
        recurs,
    };
    state
        .clients
        .by(&member.email)
        .add_extra_item(firm, id, &item)
        .await
        .map_err(client_refusal)?;
    Ok(StatusCode::CREATED)
}

async fn delete_item(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath((id, item)): ApiPath<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    state
        .clients
        .by(&member.email)
        .delete_extra_item(firm, ClientId::from(id), ExtraItemId::from(item))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// A business that owes the Client a 2307 each quarter. Names only.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewPayor {
    pub name: String,
}

async fn add_payor(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(payor): ApiJson<NewPayor>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    state
        .payors
        .by(&member.email)
        .add(firm, ClientId::from(id), &payor.name)
        .await
        .map_err(client_refusal)?;
    Ok(StatusCode::CREATED)
}

/// A Client's payroll chasing (P1, P2), as shown and as changed.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Payroll {
    /// `twice-monthly`, `monthly`, or `null` when it's off.
    pub pay_cycle: Option<String>,
    /// PhilHealth's due day of the following month, 1 to 31.
    pub philhealth_due_day: u8,
    /// Pag-IBIG's due day of the following month, 1 to 31 (31: the last day).
    pub pagibig_due_day: u8,
}

impl From<Option<PayrollSettings>> for Payroll {
    fn from(settings: Option<PayrollSettings>) -> Self {
        Self {
            pay_cycle: settings.map(|s| s.cycle.as_str().to_owned()),
            philhealth_due_day: settings.map_or(PHILHEALTH_DEFAULT_DAY, |s| s.philhealth_due_day),
            pagibig_due_day: settings.map_or(PAGIBIG_DEFAULT_DAY, |s| s.pagibig_due_day),
        }
    }
}

async fn set_payroll(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(change): ApiJson<Payroll>,
) -> Result<StatusCode, ApiError> {
    let settings = match change.pay_cycle.as_deref() {
        None => None,
        Some(cycle) => Some(PayrollSettings {
            cycle: PayCycle::parse(cycle).ok_or_else(|| refused("Choose twice a month or monthly."))?,
            philhealth_due_day: change.philhealth_due_day,
            pagibig_due_day: change.pagibig_due_day,
        }),
    };
    state
        .clients
        .by(&member.email)
        .set_payroll(member.firm, ClientId::from(id), settings)
        .await
        .map_err(client_refusal)?;
    Ok(StatusCode::NO_CONTENT)
}

/// Names ticked from a sales export (H3).
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewPayors {
    pub names: Vec<String>,
}

/// What became of each name.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PayorsAdded {
    pub added: u32,
    pub already_listed: u32,
    /// Each refused name with why, e.g. "Puregold: a Payor's name is 1 to 100 characters".
    pub refused: Vec<String>,
}

async fn add_payors(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(batch): ApiJson<NewPayors>,
) -> Result<Json<PayorsAdded>, ApiError> {
    if batch.names.len() > MAX_NAMES {
        return Err(refused(format!("Add at most {MAX_NAMES} Payors at once.")));
    }
    let outcome = state
        .payors
        .by(&member.email)
        .add_all(member.firm, ClientId::from(id), &batch.names)
        .await?;
    let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
    let outcome = PayorsAdded {
        added: count(outcome.added.len()),
        already_listed: count(outcome.already_listed),
        refused: outcome
            .refused
            .into_iter()
            .map(|(name, reason)| format!("{name}: {reason}"))
            .collect(),
    };
    Ok(Json(outcome))
}

async fn remove_payor(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath((id, payor)): ApiPath<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    state
        .payors
        .by(&member.email)
        .remove(firm, ClientId::from(id), PayorId::from(payor))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
