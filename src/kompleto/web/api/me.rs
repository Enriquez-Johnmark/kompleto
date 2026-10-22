//! Who is logged in, for the app's header.

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use super::{ApiError, ApiFirm};
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/me", get(me))
}

/// The logged-in Firm.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Me {
    /// The name Clients see as the sender.
    pub firm_name: String,
    /// The one address that can log in.
    pub login_email: String,
}

async fn me(State(state): State<AppState>, ApiFirm(firm): ApiFirm) -> Result<Json<Me>, ApiError> {
    let firm = state.firms.get(firm).await?;
    Ok(Json(Me {
        firm_name: firm.name,
        login_email: firm.login_email,
    }))
}
