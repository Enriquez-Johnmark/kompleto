//! The Activity log (J2): what happened to the Firm's records lately.

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use time::Duration;

use super::client_page::rfc3339;
use super::{ApiError, ApiFirm};
use crate::kompleto::activity::{self, SHOWN_DAYS, describe, who};
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/activity", get(page))
}

/// The Firm's recent activity.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ActivityLog {
    /// How many days back it goes.
    pub days: i64,
    /// Newest first.
    pub entries: Vec<ActivityEntry>,
}

/// One thing that happened.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ActivityEntry {
    /// RFC 3339.
    pub at: String,
    /// The login address, or "Kompleto" for the system.
    pub who: String,
    /// e.g. "Marked an upload received".
    pub what: String,
}

async fn page(State(state): State<AppState>, ApiFirm(firm): ApiFirm) -> Result<Json<ActivityLog>, ApiError> {
    let since = state.clock.now() - Duration::days(SHOWN_DAYS);
    let entries = activity::since(&state.pool, firm, since)
        .await
        .map_err(ApiError::internal)?;
    Ok(Json(ActivityLog {
        days: SHOWN_DAYS,
        entries: entries
            .into_iter()
            .map(|entry| ActivityEntry {
                at: rfc3339(entry.at),
                who: who(&entry.actor).to_owned(),
                what: describe(&entry.action).to_owned(),
            })
            .collect(),
    }))
}
