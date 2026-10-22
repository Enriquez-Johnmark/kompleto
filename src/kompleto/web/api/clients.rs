//! The Client list, adding a Client and editing its details (B1), and the
//! BIR forms a Client can file.

use axum::extract::State;
use axum::http::{Method, StatusCode};
use axum::routing::{MethodRouter, get, put};
use axum::{Json, Router};
use serde::Serialize;
use uuid::Uuid;

use super::{ApiError, ApiFirm, ApiJson, ApiMember, ApiPath};
use crate::kompleto::calendar::BirForm;
use crate::kompleto::clients::{Client, ClientError, ClientId, ClientInput, Language, NewClient};
use crate::kompleto::web::{AppState, ScopedRoute, scoped_router};

pub(super) fn routes() -> Router<AppState> {
    scoped_router(client_endpoints())
        .route("/api/clients", get(list).post(create))
        .route("/api/bir-forms", get(bir_forms))
}

const SAMPLE_CLIENT: &str = r#"{"business_name": "Sample Store", "contact_first_name": "Ana",
    "email": "ana@example.test", "language": "english", "forms": ["2551Q"]}"#;

/// Every endpoint about one Client (A2).
pub(in crate::kompleto::web) fn client_endpoints() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    vec![(
        ScopedRoute {
            method: Method::PUT,
            path: "/api/clients/{id}",
            sample_form: SAMPLE_CLIENT,
        },
        put(update),
    )]
}

/// One Client in the list.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ClientSummary {
    pub id: Uuid,
    pub business_name: String,
    pub contact_first_name: String,
    pub email: String,
    /// `taglish` or `english`: the reminders' language.
    pub language: String,
    /// Form numbers, e.g. `["2551Q", "1701Q"]`.
    pub forms: Vec<String>,
    /// No reminders are drafted while paused.
    pub paused: bool,
}

impl From<Client> for ClientSummary {
    fn from(client: Client) -> Self {
        Self {
            id: client.id.as_uuid(),
            business_name: client.details.business_name,
            contact_first_name: client.details.contact_first_name,
            email: client.details.email,
            language: language(client.details.language).to_owned(),
            forms: client.details.forms.iter().map(|form| form.code().to_owned()).collect(),
            paused: client.paused,
        }
    }
}

/// A language as the API writes it.
pub(super) fn language(language: Language) -> &'static str {
    match language {
        Language::Taglish => "taglish",
        Language::English => "english",
    }
}

async fn list(State(state): State<AppState>, ApiFirm(firm): ApiFirm) -> Result<Json<Vec<ClientSummary>>, ApiError> {
    let clients = state.clients.list(firm).await?;
    Ok(Json(clients.into_iter().map(ClientSummary::from).collect()))
}

/// A Client added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Created {
    pub id: Uuid,
}

/// Checks every field of `input` and reports all problems at once.
fn parse(input: &ClientInput) -> Result<NewClient, ApiError> {
    NewClient::parse(input).map_err(|errors| ApiError::Refused {
        status: StatusCode::UNPROCESSABLE_ENTITY,
        message: "Not saved: please fix the problems below.".to_owned(),
        problems: errors.iter().map(ToString::to_string).collect(),
    })
}

fn duplicate(err: ClientError) -> ApiError {
    match err {
        ClientError::DuplicateName(_) => ApiError::invalid(format!("Not saved: {err}.")),
        other => other.into(),
    }
}

async fn create(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiJson(input): ApiJson<ClientInput>,
) -> Result<(StatusCode, Json<Created>), ApiError> {
    let client = parse(&input)?;
    let id = state
        .clients
        .by(&member.email)
        .create(member.firm, &client)
        .await
        .map_err(duplicate)?;
    Ok((StatusCode::CREATED, Json(Created { id: id.as_uuid() })))
}

async fn update(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(input): ApiJson<ClientInput>,
) -> Result<StatusCode, ApiError> {
    let firm = member.firm;
    let id = ClientId::from(id);
    // Another Firm's Client is "not found" before any field is checked.
    state.clients.get(firm, id).await?;
    let client = parse(&input)?;
    state
        .clients
        .by(&member.email)
        .update(firm, id, &client)
        .await
        .map_err(duplicate)?;
    Ok(StatusCode::NO_CONTENT)
}

/// A BIR form a Client can file.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BirFormOption {
    /// e.g. "2551Q".
    pub code: String,
    /// e.g. "Quarterly Percentage Tax Return".
    pub name: String,
}

async fn bir_forms(State(state): State<AppState>, _firm: ApiFirm) -> Json<Vec<BirFormOption>> {
    Json(
        BirForm::ALL
            .into_iter()
            .map(|form| BirFormOption {
                code: form.code().to_owned(),
                name: state.calendar.form_name(form).to_owned(),
            })
            .collect(),
    )
}
