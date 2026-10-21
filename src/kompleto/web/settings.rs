//! The Firm's settings and the BIR calendar page (B3).

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Form, Router};
use serde::Deserialize;
use time::Duration;

use super::{AppState, CurrentFirm, WebError, show_date};
use crate::kompleto::firms::Firm;
use crate::kompleto::schedule::MANILA;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/settings", get(settings_page).post(save_settings))
        .route("/calendar", get(calendar_page))
}

#[derive(Template, WebTemplate)]
#[template(path = "settings.html")]
struct SettingsPage {
    firm: Firm,
    error: Option<String>,
}

async fn settings_page(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
) -> Result<SettingsPage, WebError> {
    Ok(SettingsPage {
        firm: state.firms.get(firm).await?,
        error: None,
    })
}

#[derive(Deserialize)]
struct SettingsForm {
    cut_off_offset_days: String,
}

async fn save_settings(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Form(form): Form<SettingsForm>,
) -> Result<Response, WebError> {
    match form.cut_off_offset_days.trim().parse::<u8>() {
        Ok(days) if days <= 30 => {
            state.firms.set_cut_off_offset(firm, days).await?;
            Ok(Redirect::to("/settings").into_response())
        }
        _ => {
            let page = SettingsPage {
                firm: state.firms.get(firm).await?,
                error: Some("The cut-off offset is 0 to 30 days.".to_owned()),
            };
            Ok((StatusCode::UNPROCESSABLE_ENTITY, page).into_response())
        }
    }
}

struct CalendarRow {
    due: String,
    form: &'static str,
    period: String,
    note: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "calendar.html")]
struct CalendarPage {
    rows: Vec<CalendarRow>,
}

async fn calendar_page(State(state): State<AppState>, _firm: CurrentFirm) -> CalendarPage {
    let today = state.clock.now().to_offset(MANILA).date();
    let rows = state
        .calendar
        .due_dates_between(today, today + Duration::days(365))
        .into_iter()
        .map(|(form, period, due)| CalendarRow {
            note: match (&due.extension, due.date == due.statutory) {
                (Some(extension), _) => format!("extended, {extension}"),
                (None, false) => format!("moved from {}", show_date(due.statutory)),
                (None, true) => String::new(),
            },
            due: show_date(due.date),
            form: form.code(),
            period: period.to_string(),
        })
        .collect();
    CalendarPage { rows }
}
