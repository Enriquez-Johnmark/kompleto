//! Client pages: the list, adding and editing a Client (B1), and one
//! Client's Periods with their Due dates, Cut-off and Owed documents (B4).

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, get, post};
use axum::{Form, Router};
use serde::Deserialize;
use time::Date;
use uuid::Uuid;

use super::{AppState, CurrentFirm, ScopedRoute, WebError, show_date};
use crate::kompleto::calendar::{BirForm, TaxPeriod};
use crate::kompleto::checklist::{DEFAULT_ITEMS, ExtraItem, Recurs, owed_documents};
use crate::kompleto::clients::{Client, ClientError, ClientId, ClientInput, ExtraItemId, NewClient, ScheduleSettings};
use crate::kompleto::firms::FirmId;
use crate::kompleto::periods::ClientCalendar;
use crate::kompleto::schedule::{ReminderDays, planned_drafts};

pub(super) fn routes() -> Router<AppState> {
    super::scoped_router(client_pages())
        .route("/clients", get(list).post(create))
        .route("/clients/new", get(new_form))
}

const SAMPLE_CLIENT: &str =
    "business_name=Sample+Store&contact_first_name=Ana&email=ana%40example.test&language=english&forms=2551Q";

/// Every page about one Client (A2).
pub(super) fn client_pages() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    let page = |method, path, sample_form| ScopedRoute {
        method,
        path,
        sample_form,
    };
    vec![
        (page(Method::GET, "/clients/{id}", ""), get(show)),
        (page(Method::GET, "/clients/{id}/edit", ""), get(edit_form)),
        (page(Method::POST, "/clients/{id}", SAMPLE_CLIENT), post(update)),
        (
            page(
                Method::POST,
                "/clients/{id}/schedule",
                "cut_off_offset_days=&reminder_days=7%2C3%2C1",
            ),
            post(set_schedule),
        ),
        (
            page(
                Method::POST,
                "/clients/{id}/overrides",
                "form=2551Q&period=2026-Q4&due=2027-01-29",
            ),
            post(set_override),
        ),
        (
            page(Method::POST, "/clients/{id}/checklist/remove", "key=bank-statements"),
            post(remove_item),
        ),
        (
            page(Method::POST, "/clients/{id}/checklist/restore", "key=bank-statements"),
            post(restore_item),
        ),
        (
            page(
                Method::POST,
                "/clients/{id}/checklist/add",
                "label=Lazada+payouts&recurs=every-period",
            ),
            post(add_item),
        ),
        (
            page(Method::POST, "/clients/{id}/checklist/extra/{item}/delete", ""),
            post(delete_item),
        ),
    ]
}

#[derive(Template, WebTemplate)]
#[template(path = "clients.html")]
struct ClientList {
    clients: Vec<Client>,
}

async fn list(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<ClientList, WebError> {
    Ok(ClientList {
        clients: state.clients.list(firm).await?,
    })
}

/// One checkbox on the client form.
struct FormChoice {
    code: &'static str,
    name: String,
    checked: bool,
}

#[derive(Template, WebTemplate)]
#[template(path = "client_form.html")]
struct ClientForm {
    editing: bool,
    action: String,
    input: ClientInput,
    forms: Vec<FormChoice>,
    errors: Vec<String>,
}

impl ClientForm {
    fn new(state: &AppState, action: String, input: ClientInput, errors: Vec<String>) -> Self {
        let forms = BirForm::ALL
            .into_iter()
            .map(|form| FormChoice {
                code: form.code(),
                name: state.calendar.form_name(form).to_owned(),
                checked: input.forms.iter().any(|f| BirForm::parse(f) == Some(form)),
            })
            .collect();
        Self {
            editing: false,
            action,
            input,
            forms,
            errors,
        }
    }

    fn editing(state: &AppState, id: ClientId, input: ClientInput, errors: Vec<String>) -> Self {
        Self {
            editing: true,
            ..Self::new(state, format!("/clients/{id}"), input, errors)
        }
    }
}

async fn new_form(State(state): State<AppState>, _firm: CurrentFirm) -> ClientForm {
    let input = ClientInput {
        language: "taglish".to_owned(),
        ..ClientInput::default()
    };
    ClientForm::new(&state, "/clients".to_owned(), input, Vec::new())
}

/// The client form's fields. Read as pairs because `forms` repeats.
fn client_input(pairs: Vec<(String, String)>) -> ClientInput {
    let mut input = ClientInput::default();
    for (name, value) in pairs {
        match name.as_str() {
            "business_name" => input.business_name = value,
            "contact_first_name" => input.contact_first_name = value,
            "email" => input.email = value,
            "language" => input.language = value,
            "forms" => input.forms.push(value),
            _ => {}
        }
    }
    input
}

async fn create(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Form(pairs): Form<Vec<(String, String)>>,
) -> Result<Response, WebError> {
    let input = client_input(pairs);
    let refused = |errors: Vec<String>| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientForm::new(&state, "/clients".to_owned(), input.clone(), errors),
        )
            .into_response()
    };
    let client = match NewClient::parse(&input) {
        Ok(client) => client,
        Err(errors) => return Ok(refused(errors.iter().map(ToString::to_string).collect())),
    };
    match state.clients.create(firm, &client).await {
        Ok(id) => Ok(Redirect::to(&format!("/clients/{id}")).into_response()),
        Err(err @ ClientError::DuplicateName(_)) => Ok(refused(vec![err.to_string()])),
        Err(err) => Err(err.into()),
    }
}

async fn edit_form(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
) -> Result<ClientForm, WebError> {
    let id = ClientId::from(id);
    let details = state.clients.get(firm, id).await?.details;
    let input = ClientInput {
        business_name: details.business_name,
        contact_first_name: details.contact_first_name,
        email: details.email,
        language: details.language.as_str().to_owned(),
        forms: details.forms.iter().map(|form| form.code().to_owned()).collect(),
    };
    Ok(ClientForm::editing(&state, id, input, Vec::new()))
}

async fn update(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(pairs): Form<Vec<(String, String)>>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    // Another Firm's Client is "not found" before any field is checked.
    state.clients.get(firm, id).await?;
    let input = client_input(pairs);
    let refused = |errors: Vec<String>| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientForm::editing(&state, id, input.clone(), errors),
        )
            .into_response()
    };
    let client = match NewClient::parse(&input) {
        Ok(client) => client,
        Err(errors) => return Ok(refused(errors.iter().map(ToString::to_string).collect())),
    };
    match state.clients.update(firm, id, &client).await {
        Ok(()) => Ok(Redirect::to(&format!("/clients/{id}")).into_response()),
        Err(err @ ClientError::DuplicateName(_)) => Ok(refused(vec![err.to_string()])),
        Err(err) => Err(err.into()),
    }
}

/// One form's filing, ready to show.
struct FilingView {
    form: BirForm,
    period: String,
    period_key: String,
    due_iso: String,
    due: String,
    overridden: bool,
    extension: Option<String>,
}

/// One collection Period, ready to show.
struct PeriodView {
    label: String,
    cut_off: String,
    drafts: Vec<String>,
    filings: Vec<FilingView>,
    owed: Vec<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "client.html")]
struct ClientPage {
    client: Client,
    language: &'static str,
    periods: Vec<PeriodView>,
    firm_offset: u8,
    offset_value: String,
    reminder_days: String,
    default_items: Vec<ChecklistItemView>,
    extra_items: Vec<ChecklistItemView>,
    uploads: Vec<UploadView>,
    error: Option<String>,
}

/// One Upload, ready to show.
struct UploadView {
    id: String,
    period: String,
    document: String,
    detail: String,
    uploaded: String,
    size: String,
    received: Option<String>,
    deleted: bool,
    delete_by: String,
}

/// One line of a Client's checklist, ready to show.
struct ChecklistItemView {
    /// A default item's key (empty for an added item).
    key: String,
    /// An added item's ID (empty for a default item).
    id: String,
    label: String,
    when: &'static str,
    removed: bool,
}

fn recurs_text(recurs: Recurs) -> &'static str {
    match recurs {
        Recurs::EveryPeriod => "every period",
        Recurs::QuarterEnd => "at quarter end",
        Recurs::YearEnd => "in December",
    }
}

async fn show(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
) -> Result<ClientPage, WebError> {
    client_page(&state, firm, ClientId::from(id), None).await
}

async fn client_page(
    state: &AppState,
    firm: FirmId,
    id: ClientId,
    error: Option<String>,
) -> Result<ClientPage, WebError> {
    let client = state.clients.get(firm, id).await?;
    let overrides = state.clients.due_overrides(firm, id).await?;
    let manila = crate::kompleto::schedule::MANILA;
    let uploads = state
        .uploads
        .for_client(firm, id)
        .await?
        .into_iter()
        .rev()
        .map(|upload| UploadView {
            id: upload.id.to_string(),
            period: upload.period.clone(),
            document: upload.label.document().to_owned(),
            detail: upload.label.detail().unwrap_or_default().to_owned(),
            uploaded: show_date(upload.uploaded_at.to_offset(manila).date()),
            size: format!("{} KB", u32::try_from(upload.size_bytes).unwrap_or(0).div_ceil(1024)),
            received: upload.received_at.map(|at| show_date(at.to_offset(manila).date())),
            deleted: upload.deleted,
            delete_by: show_date(upload.delete_by.to_offset(manila).date()),
        })
        .collect();
    let edits = state.clients.checklist_edits(firm, id).await?;
    let firm_offset = state.firms.get(firm).await?.cut_off_offset_days;
    let calendar = ClientCalendar {
        calendar: &state.calendar,
        forms: &client.details.forms,
        overrides: &overrides,
        cut_off_offset_days: client.cut_off_offset_days.unwrap_or(firm_offset),
    };
    let today = state.clock.now().to_offset(crate::kompleto::schedule::MANILA).date();
    let periods = calendar
        .upcoming(today)
        .into_iter()
        .map(|period| PeriodView {
            label: period.label.to_string(),
            cut_off: show_date(period.cut_off),
            drafts: planned_drafts(&period, &client.reminder_days)
                .iter()
                .map(|d| show_date(d.on))
                .collect(),
            owed: owed_documents(&client.details.forms, period.month, &edits),
            filings: period
                .filings
                .into_iter()
                .map(|filing| FilingView {
                    form: filing.form,
                    period: filing.period.to_string(),
                    period_key: filing.period.key(),
                    due_iso: filing.due.to_string(),
                    due: show_date(filing.due),
                    overridden: filing.overridden,
                    extension: filing.extension,
                })
                .collect(),
        })
        .collect();
    let default_items = DEFAULT_ITEMS
        .iter()
        .filter(|item| item.when_filing.is_empty() || item.when_filing.iter().any(|f| client.details.forms.contains(f)))
        .map(|item| ChecklistItemView {
            key: item.key.to_owned(),
            id: String::new(),
            label: item.label.to_owned(),
            when: recurs_text(item.recurs),
            removed: edits.removed.contains(item.key),
        })
        .collect();
    let extra_items = state
        .clients
        .extra_items(firm, id)
        .await?
        .into_iter()
        .map(|(item_id, item)| ChecklistItemView {
            key: String::new(),
            id: item_id.to_string(),
            label: item.label,
            when: recurs_text(item.recurs),
            removed: false,
        })
        .collect();
    let language = match client.details.language {
        crate::kompleto::clients::Language::Taglish => "Taglish",
        crate::kompleto::clients::Language::English => "English",
    };
    Ok(ClientPage {
        offset_value: client.cut_off_offset_days.map(|d| d.to_string()).unwrap_or_default(),
        reminder_days: client
            .reminder_days
            .days()
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        client,
        language,
        periods,
        firm_offset,
        default_items,
        extra_items,
        uploads,
        error,
    })
}

#[derive(Deserialize)]
struct ItemKeyForm {
    key: String,
}

async fn remove_item(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<ItemKeyForm>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    match state.clients.remove_default_item(firm, id, &form.key).await {
        Ok(()) => Ok(back_to(id)),
        Err(ClientError::Invalid(reason)) => refuse(&state, firm, id, reason).await,
        Err(err) => Err(err.into()),
    }
}

async fn restore_item(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<ItemKeyForm>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    state.clients.restore_default_item(firm, id, &form.key).await?;
    Ok(back_to(id))
}

#[derive(Deserialize)]
struct AddItemForm {
    label: String,
    recurs: String,
}

async fn add_item(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<AddItemForm>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    state.clients.get(firm, id).await?;
    let Some(recurs) = Recurs::parse(&form.recurs) else {
        return refuse(&state, firm, id, "Choose when the document is owed.".to_owned()).await;
    };
    match state
        .clients
        .add_extra_item(
            firm,
            id,
            &ExtraItem {
                label: form.label,
                recurs,
            },
        )
        .await
    {
        Ok(_) => Ok(back_to(id)),
        Err(ClientError::Invalid(reason)) => refuse(&state, firm, id, reason).await,
        Err(err) => Err(err.into()),
    }
}

async fn delete_item(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path((id, item)): Path<(Uuid, Uuid)>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    state
        .clients
        .delete_extra_item(firm, id, ExtraItemId::from(item))
        .await?;
    Ok(back_to(id))
}

/// Shows the client page again with `error`, as the answer to a refused form.
async fn refuse(state: &AppState, firm: FirmId, id: ClientId, error: String) -> Result<Response, WebError> {
    let page = client_page(state, firm, id, Some(error)).await?;
    Ok((StatusCode::UNPROCESSABLE_ENTITY, page).into_response())
}

fn back_to(id: ClientId) -> Response {
    Redirect::to(&format!("/clients/{id}")).into_response()
}

#[derive(Deserialize)]
struct ScheduleForm {
    cut_off_offset_days: String,
    reminder_days: String,
    paused: Option<String>,
}

async fn set_schedule(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<ScheduleForm>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    state.clients.get(firm, id).await?;
    let offset = form.cut_off_offset_days.trim();
    let offset = match offset.parse::<u8>() {
        _ if offset.is_empty() => None,
        Ok(days) if days <= 30 => Some(days),
        _ => return refuse(&state, firm, id, "The cut-off offset is 0 to 30 days.".to_owned()).await,
    };
    let days: Result<Vec<u8>, _> = form
        .reminder_days
        .split([',', ' '])
        .filter(|day| !day.is_empty())
        .map(str::parse)
        .collect();
    let Ok(days) = days else {
        return refuse(&state, firm, id, "Reminder days are numbers, e.g. 7, 3, 1.".to_owned()).await;
    };
    let reminder_days = match ReminderDays::new(&days) {
        Ok(days) => days,
        Err(err) => return refuse(&state, firm, id, err.to_string()).await,
    };
    let settings = ScheduleSettings {
        cut_off_offset_days: offset,
        reminder_days,
        paused: form.paused.is_some(),
    };
    state.clients.set_schedule(firm, id, &settings).await?;
    Ok(back_to(id))
}

#[derive(Deserialize)]
struct OverrideForm {
    form: String,
    period: String,
    due: String,
}

async fn set_override(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<OverrideForm>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    state.clients.get(firm, id).await?;
    let filing = BirForm::parse(&form.form)
        .zip(TaxPeriod::parse_key(&form.period))
        .filter(|(bir_form, period)| state.calendar.due_date(*bir_form, *period).is_some());
    let Some((bir_form, period)) = filing else {
        return refuse(&state, firm, id, "That form isn't filed for that period.".to_owned()).await;
    };
    let due = match form.due.trim() {
        "" => None,
        text => match Date::parse(text, time::macros::format_description!("[year]-[month]-[day]")) {
            Ok(date) => Some(date),
            Err(_) => return refuse(&state, firm, id, format!("{text:?} isn't a date.")).await,
        },
    };
    state.clients.set_due_override(firm, id, bir_form, period, due).await?;
    Ok(back_to(id))
}
