//! Import pages (B2): upload a file, see every row checked, then confirm.
//! The confirmed rows come back from the page, so they're checked again
//! before anything is added, and added in one transaction.

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;

use super::{AppState, CurrentFirm, WebError};
use crate::kompleto::clients::{ClientError, ClientInput, NewClient};
use crate::kompleto::firms::FirmId;
use crate::kompleto::import::{self, COLUMNS, ImportError, ImportRow, MAX_BYTES};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/import", get(upload_page).post(check_upload))
        .route("/import/confirm", post(confirm))
        .route("/import/template.csv", get(template))
        // Room for a 2 MB file plus the form's own bytes.
        .layer(DefaultBodyLimit::max(MAX_BYTES + 64 * 1024))
}

#[derive(Template, WebTemplate)]
#[template(path = "import.html")]
struct UploadPage {
    error: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "import_preview.html")]
struct PreviewPage {
    valid: Vec<(usize, NewClient)>,
    invalid: Vec<(usize, Vec<String>)>,
    /// The valid rows as typed, to send back with the confirmation.
    rows_json: String,
}

async fn upload_page(_firm: CurrentFirm) -> UploadPage {
    UploadPage { error: None }
}

fn refused(error: impl ToString) -> Response {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        UploadPage {
            error: Some(error.to_string()),
        },
    )
        .into_response()
}

async fn check_upload(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    mut multipart: Multipart,
) -> Result<Response, WebError> {
    let mut file = None;
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            let name = field.file_name().unwrap_or("").to_ascii_lowercase();
            let Ok(bytes) = field.bytes().await else {
                return Ok(refused(ImportError::TooLarge));
            };
            file = Some((name, bytes));
        }
    }
    let Some((name, bytes)) = file else {
        return Ok(refused("choose a file to upload"));
    };
    let rows = if name.ends_with(".csv") {
        import::read_csv(&bytes)
    } else if name.ends_with(".xlsx") {
        state.xlsx.read(&bytes).await
    } else {
        return Ok(refused("Upload a .csv or .xlsx file."));
    };
    match rows {
        Ok(rows) => Ok(preview_page(&state, firm, &rows).await?.into_response()),
        Err(err) => Ok(refused(err)),
    }
}

async fn preview_page(state: &AppState, firm: FirmId, rows: &[ImportRow]) -> Result<PreviewPage, WebError> {
    let existing: Vec<String> = state
        .clients
        .list(firm)
        .await?
        .into_iter()
        .map(|c| c.details.business_name)
        .collect();
    let preview = import::preview(rows, &existing);
    let valid_inputs: Vec<&ClientInput> = rows
        .iter()
        .filter(|row| preview.valid.iter().any(|(n, _)| *n == row.row))
        .map(|row| &row.input)
        .collect();
    Ok(PreviewPage {
        rows_json: serde_json::to_string(&valid_inputs).map_err(WebError::internal)?,
        valid: preview.valid,
        invalid: preview.invalid,
    })
}

#[derive(Deserialize)]
struct ConfirmForm {
    rows: String,
}

async fn confirm(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Form(form): Form<ConfirmForm>,
) -> Result<Response, WebError> {
    let Ok(inputs) = serde_json::from_str::<Vec<ClientInput>>(&form.rows) else {
        return Ok(refused("the confirmation was damaged; please upload the file again"));
    };
    // Checked again: the page could be stale or tampered with.
    let rows: Vec<ImportRow> = inputs
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
    let preview = import::preview(&rows, &existing);
    if !preview.invalid.is_empty() {
        return Ok(preview_page(&state, firm, &rows).await?.into_response());
    }
    let clients: Vec<NewClient> = preview.valid.into_iter().map(|(_, client)| client).collect();
    match state.clients.create_all(firm, &clients).await {
        Ok(_) => Ok(Redirect::to("/clients").into_response()),
        Err(err @ ClientError::DuplicateName(_)) => Ok(refused(err)),
        Err(err) => Err(err.into()),
    }
}

async fn template(_firm: CurrentFirm) -> impl IntoResponse {
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
