//! Importing Clients from a file (B2): upload it, see every row checked,
//! then confirm. The confirmed rows come back from the app, so they're
//! checked again before anything is added, and added in one transaction.

use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use super::{ApiError, ApiFirm, ApiJson, ApiMember};
use crate::kompleto::clients::{ClientError, ClientInput, NewClient};
use crate::kompleto::firms::FirmId;
use crate::kompleto::import::{self, COLUMNS, ImportError, ImportRow, MAX_BYTES, MAX_ROWS};
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/import", post(check_upload))
        .route("/api/import/confirm", post(confirm))
        .route("/api/import/template.csv", get(template))
        // Room for a 2 MB file plus the request's own bytes.
        .layer(DefaultBodyLimit::max(MAX_BYTES + 64 * 1024))
}

/// Every row of an uploaded file, checked.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ImportPreview {
    /// Rows that will be added.
    pub valid: Vec<ValidRow>,
    /// Rows that won't, each with every reason.
    pub invalid: Vec<InvalidRow>,
    /// The valid rows as typed, to send back with the confirmation.
    pub rows: Vec<ClientInput>,
}

/// A row that will be added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ValidRow {
    /// Its row number in the file.
    pub row: u32,
    pub business_name: String,
    pub contact_first_name: String,
    pub email: String,
    pub forms: Vec<String>,
}

/// A row that won't be added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct InvalidRow {
    pub row: u32,
    pub reasons: Vec<String>,
}

fn refused(reason: impl ToString) -> ApiError {
    ApiError::invalid(reason.to_string())
}

fn row_number(row: usize) -> u32 {
    u32::try_from(row).unwrap_or(u32::MAX)
}

async fn check_upload(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    mut multipart: Multipart,
) -> Result<Json<ImportPreview>, ApiError> {
    let mut file = None;
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            let name = field.file_name().unwrap_or("").to_ascii_lowercase();
            let bytes = field.bytes().await.map_err(|_| refused(ImportError::TooLarge))?;
            file = Some((name, bytes));
        }
    }
    let (name, bytes) = file.ok_or_else(|| refused("Choose a file to upload."))?;
    let rows = if name.ends_with(".csv") {
        import::read_csv(&bytes)
    } else if name.ends_with(".xlsx") {
        state.xlsx.read(&bytes).await
    } else {
        return Err(refused("Upload a .csv or .xlsx file."));
    };
    let rows = rows.map_err(refused)?;
    Ok(Json(preview(&state, firm, rows).await?))
}

async fn preview(state: &AppState, firm: FirmId, rows: Vec<ImportRow>) -> Result<ImportPreview, ApiError> {
    let existing: Vec<String> = state
        .clients
        .list(firm)
        .await?
        .into_iter()
        .map(|c| c.details.business_name)
        .collect();
    let checked = import::preview(&rows, &existing);
    let valid_rows = rows
        .into_iter()
        .filter(|row| checked.valid.iter().any(|(n, _)| *n == row.row))
        .map(|row| row.input)
        .collect();
    Ok(ImportPreview {
        valid: checked
            .valid
            .into_iter()
            .map(|(row, client)| ValidRow {
                row: row_number(row),
                forms: client.forms.iter().map(|form| form.code().to_owned()).collect(),
                business_name: client.business_name,
                contact_first_name: client.contact_first_name,
                email: client.email,
            })
            .collect(),
        invalid: checked
            .invalid
            .into_iter()
            .map(|(row, reasons)| InvalidRow {
                row: row_number(row),
                reasons,
            })
            .collect(),
        rows: valid_rows,
    })
}

/// The previewed rows, confirmed.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ImportConfirmation {
    pub rows: Vec<ClientInput>,
}

/// How many Clients were added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Imported {
    pub added: u32,
}

async fn confirm(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiJson(confirmation): ApiJson<ImportConfirmation>,
) -> Result<Json<Imported>, ApiError> {
    let firm = member.firm;
    if confirmation.rows.len() > MAX_ROWS {
        return Err(refused(ImportError::TooManyRows));
    }
    // Checked again: the confirmation could be stale or tampered with.
    let rows: Vec<ImportRow> = confirmation
        .rows
        .into_iter()
        .enumerate()
        .map(|(i, input)| ImportRow { row: i + 2, input })
        .collect();
    let existing: Vec<String> = state
        .clients
        .list(firm)
        .await?
        .into_iter()
        .map(|c| c.details.business_name)
        .collect();
    let checked = import::preview(&rows, &existing);
    if !checked.invalid.is_empty() {
        return Err(ApiError::Refused {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: "Nothing was added: some rows changed since the preview. Upload the file again.".to_owned(),
            problems: checked
                .invalid
                .into_iter()
                .map(|(row, reasons)| format!("Row {row}: {}", reasons.join("; ")))
                .collect(),
        });
    }
    let clients: Vec<NewClient> = checked.valid.into_iter().map(|(_, client)| client).collect();
    let added = state
        .clients
        .by(&member.email)
        .create_all(firm, &clients)
        .await
        .map_err(|err| match err {
            err @ ClientError::DuplicateName(_) => refused(format!("Nothing was added: {err}.")),
            other => other.into(),
        })?;
    Ok(Json(Imported {
        added: u32::try_from(added.len()).unwrap_or(u32::MAX),
    }))
}

async fn template(_firm: ApiFirm) -> impl IntoResponse {
    let csv = format!(
        "{}\nNena's Sari-Sari Store,Nena,nena@example.com,Taglish,\"2551Q, 1701Q\"\n",
        COLUMNS.join(",")
    );
    (
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"kompleto-clients.csv\"",
            ),
        ],
        csv,
    )
}
