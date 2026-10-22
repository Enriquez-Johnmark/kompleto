//! Who is logged in, for the app's header.

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use super::{ApiError, ApiMember};
use crate::kompleto::members::Role;
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/me", get(me))
}

/// The logged-in person and their Firm.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Me {
    /// The name Clients see as the sender.
    pub firm_name: String,
    /// The person's address.
    pub email: String,
    /// `owner` or `staff` (K2).
    pub role: String,
}

async fn me(State(state): State<AppState>, ApiMember(member): ApiMember) -> Result<Json<Me>, ApiError> {
    let firm = state.firms.get(member.firm).await?;
    Ok(Json(Me {
        firm_name: firm.name,
        email: member.email,
        role: match member.role {
            Role::Owner => "owner",
            Role::Staff => "staff",
        }
        .to_owned(),
    }))
}
