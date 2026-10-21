//! Client pages: the list, adding and editing a Client (B1), and one
//! Client's Periods with their Due dates, Cut-off and Owed documents (B4).

use askama::Template;
use askama_web::WebTemplate;
use std::collections::BTreeMap;

use axum::extract::{Path, State};
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, get, post};
use axum::{Form, Router};
use uuid::Uuid;

use super::{AppState, ClientRoute, CurrentFirm, WebError, show_date};
use crate::kompleto::calendar::BirForm;
use crate::kompleto::checklist::owed_documents;
use crate::kompleto::clients::{Client, ClientError, ClientId, ClientInput, NewClient};
use crate::kompleto::periods::ClientCalendar;
use crate::kompleto::schedule::planned_drafts;

pub(super) fn routes() -> Router<AppState> {
    let mut by_path: BTreeMap<&str, MethodRouter<AppState>> = BTreeMap::new();
    for (route, handler) in client_pages() {
        let merged = match by_path.remove(route.path) {
            Some(existing) => existing.merge(handler),
            None => handler,
        };
        by_path.insert(route.path, merged);
    }
    by_path.into_iter().fold(
        Router::new()
            .route("/clients", get(list).post(create))
            .route("/clients/new", get(new_form)),
        |router, (path, handler)| router.route(path, handler),
    )
}

const SAMPLE_CLIENT: &str =
    "business_name=Sample+Store&contact_first_name=Ana&email=ana%40example.test&language=english&forms=2551Q";

/// Every page about one Client (A2).
pub(super) fn client_pages() -> Vec<(ClientRoute, MethodRouter<AppState>)> {
    let page = |method, path, sample_form| ClientRoute {
        method,
        path,
        sample_form,
    };
    vec![
        (page(Method::GET, "/clients/{id}", ""), get(show)),
        (page(Method::GET, "/clients/{id}/edit", ""), get(edit_form)),
        (page(Method::POST, "/clients/{id}", SAMPLE_CLIENT), post(update)),
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
}

async fn show(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
) -> Result<ClientPage, WebError> {
    let id = ClientId::from(id);
    let client = state.clients.get(firm, id).await?;
    let overrides = state.clients.due_overrides(firm, id).await?;
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
                    due: show_date(filing.due),
                    overridden: filing.overridden,
                    extension: filing.extension,
                })
                .collect(),
        })
        .collect();
    let language = match client.details.language {
        crate::kompleto::clients::Language::Taglish => "Taglish",
        crate::kompleto::clients::Language::English => "English",
    };
    Ok(ClientPage {
        client,
        language,
        periods,
    })
}
