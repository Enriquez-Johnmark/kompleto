//! The Firm's settings: its default cut-off, its AI cost this month (J1),
//! and closing the account (I2).

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use super::{ApiError, ApiFirm, ApiJson};
use crate::engine::cost_log;
use crate::kompleto::closing::{self, ClosingError};
use crate::kompleto::dates::manila_month;
use crate::kompleto::schedule::MANILA;
use crate::kompleto::web::{AppState, SESSION_COOKIE, actor};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/settings", get(settings).put(save))
        .route("/api/settings/close", post(close_account))
}

/// The Firm's settings.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Settings {
    pub firm_name: String,
    pub login_email: String,
    /// Default days between a Period's earliest Due date and its Cut-off.
    pub cut_off_offset_days: u8,
    /// The model's cost for this Firm this month, Manila time (J1).
    pub ai_cost: AiCost,
}

/// What wording reminders with the AI cost this month.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct AiCost {
    pub cost_usd: f64,
    /// AI-worded Drafts.
    pub drafts: i64,
}

async fn settings(State(state): State<AppState>, ApiFirm(firm): ApiFirm) -> Result<Json<Settings>, ApiError> {
    let today = state.clock.now().to_offset(MANILA).date();
    let (from, to) = manila_month(today.year(), today.month());
    let totals = cost_log::totals_for_tenant(&state.pool, firm, from, to)
        .await
        .map_err(ApiError::internal)?;
    let details = state.firms.get(firm).await?;
    Ok(Json(Settings {
        firm_name: details.name,
        login_email: details.login_email,
        cut_off_offset_days: details.cut_off_offset_days,
        ai_cost: AiCost {
            cost_usd: totals.cost_usd(),
            drafts: totals.calls,
        },
    }))
}

/// A change to the Firm's settings.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SettingsChange {
    /// 0 to 30; Clients with their own setting keep it.
    pub cut_off_offset_days: u8,
}

async fn save(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    ApiJson(change): ApiJson<SettingsChange>,
) -> Result<StatusCode, ApiError> {
    if change.cut_off_offset_days > 30 {
        return Err(ApiError::invalid("The cut-off offset is 0 to 30 days."));
    }
    state.firms.set_cut_off_offset(firm, change.cut_off_offset_days).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Closing the account, confirmed by typing the Firm's name.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CloseAccount {
    pub confirm: String,
}

/// A closed account.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Closed {
    /// The day everything is deleted, `YYYY-MM-DD`, Manila time.
    pub erase_on: String,
    /// Where to write to undo it before then.
    pub support_email: Option<String>,
}

/// Closes the account once the Firm's name is typed (I2), and logs out.
async fn close_account(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    ApiJson(close): ApiJson<CloseAccount>,
) -> Result<Response, ApiError> {
    let actor = actor(&state, firm).await?;
    match closing::close(&state.pool, &state.clock, firm, &close.confirm, &actor).await {
        Ok(erase_after) => {
            let closed = Closed {
                erase_on: erase_after.to_offset(MANILA).date().to_string(),
                support_email: state.support_email.clone(),
            };
            let cookie = format!("{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
            Ok(([(header::SET_COOKIE, cookie)], Json(closed)).into_response())
        }
        Err(ClosingError::NameMismatch) => {
            Err(ApiError::invalid("Type your firm's name exactly to close the account."))
        }
        Err(err) => Err(ApiError::internal(err)),
    }
}
