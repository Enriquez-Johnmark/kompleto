//! The BIR calendar (B3): every form's Due date in the next 12 months.

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use time::Duration;

use super::ApiFirm;
use crate::kompleto::schedule::MANILA;
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/calendar", get(calendar))
}

/// One form's Due date.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CalendarEntry {
    /// `YYYY-MM-DD`.
    pub due: String,
    /// e.g. "2551Q".
    pub form: String,
    /// e.g. "Q3 2026".
    pub period: String,
    /// A nationwide BIR extension, e.g. "RMC 12-2027".
    pub extension: Option<String>,
    /// The statutory date, when a weekend or holiday moved it, `YYYY-MM-DD`.
    pub moved_from: Option<String>,
}

async fn calendar(State(state): State<AppState>, _firm: ApiFirm) -> Json<Vec<CalendarEntry>> {
    let today = state.clock.now().to_offset(MANILA).date();
    Json(
        state
            .calendar
            .due_dates_between(today, today + Duration::days(365))
            .into_iter()
            .map(|(form, period, due)| CalendarEntry {
                moved_from: (due.extension.is_none() && due.date != due.statutory).then(|| due.statutory.to_string()),
                due: due.date.to_string(),
                form: form.code().to_owned(),
                period: period.to_string(),
                extension: due.extension,
            })
            .collect(),
    )
}
