//! Payors from a sales export (H3): upload it and see the names of the
//! customers who withheld tax. The bookkeeper adds the ones they tick
//! through a Client's Payor batch endpoint; nothing here is stored.

use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::routing::post;
use axum::{Json, Router};
use serde::Serialize;

use super::import::{Upload, refused};
use super::{ApiError, ApiFirm};
use crate::kompleto::import::MAX_BYTES;
use crate::kompleto::import::payors::withholding_customers;
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

async fn names(
    State(state): State<AppState>,
    _firm: ApiFirm,
    multipart: Multipart,
) -> Result<Json<PayorNames>, ApiError> {
    let upload = Upload::read(multipart).await?;
    let every_row = upload
        .fields
        .iter()
        .any(|(name, value)| name == "every_row" && value == "true");
    let names = withholding_customers(upload.records(&state).await?, every_row).map_err(refused)?;
    Ok(Json(PayorNames { names }))
}
