//! Payors from a sales export (H3): upload it and see the names of the
//! customers who withheld tax. The bookkeeper adds the ones they tick
//! through a Client's Payor batch endpoint; nothing here is stored.

use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::routing::post;
use axum::{Json, Router};
use serde::Serialize;

use super::{ApiError, ApiFirm};
use crate::kompleto::import::payors::withholding_customers;
use crate::kompleto::import::{self, ImportError, MAX_BYTES, numbered};
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/payors/import", post(names))
        // Room for a 2 MB file plus the request's own bytes.
        .layer(DefaultBodyLimit::max(MAX_BYTES + 64 * 1024))
}

/// The customers who withheld tax, as the file spells them.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PayorNames {
    pub names: Vec<String>,
}

fn refused(reason: impl ToString) -> ApiError {
    ApiError::invalid(reason.to_string())
}

async fn names(
    State(state): State<AppState>,
    _firm: ApiFirm,
    mut multipart: Multipart,
) -> Result<Json<PayorNames>, ApiError> {
    let (mut file, mut every_row) = (None, false);
    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("file") => {
                let name = field.file_name().unwrap_or("").to_ascii_lowercase();
                let bytes = field.bytes().await.map_err(|_| refused(ImportError::TooLarge))?;
                file = Some((name, bytes));
            }
            Some("every_row") => every_row = true,
            _ => {}
        }
    }
    let (name, bytes) = file.ok_or_else(|| refused("Choose a file to upload."))?;
    let records = if name.ends_with(".csv") {
        import::csv_records(&bytes)
    } else if name.ends_with(".xlsx") {
        state.xlsx.cells(&bytes).await.map(numbered)
    } else {
        return Err(refused("Upload a .csv or .xlsx file."));
    };
    let names = records
        .map_err(Into::into)
        .and_then(|records| withholding_customers(records, every_row))
        .map_err(refused)?;
    Ok(Json(PayorNames { names }))
}
