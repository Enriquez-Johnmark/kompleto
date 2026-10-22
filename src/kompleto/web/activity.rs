//! The Activity page (J2): what happened to the Firm's records lately.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use time::Duration;
use time::macros::format_description;

use super::{AppState, CurrentFirm, WebError};
use crate::kompleto::activity::{self, SHOWN_DAYS, describe, who};
use crate::kompleto::schedule::MANILA;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/activity", get(page))
}

struct Row {
    at: String,
    who: String,
    what: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "activity.html")]
struct ActivityPage {
    days: i64,
    rows: Vec<Row>,
}

async fn page(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<ActivityPage, WebError> {
    let since = state.clock.now() - Duration::days(SHOWN_DAYS);
    let entries = activity::since(&state.pool, firm, since)
        .await
        .map_err(WebError::internal)?;
    let format =
        format_description!("[weekday repr:short] [day padding:none] [month repr:short] [year] [hour]:[minute]");
    Ok(ActivityPage {
        days: SHOWN_DAYS,
        rows: entries
            .into_iter()
            .map(|entry| Row {
                at: entry.at.to_offset(MANILA).format(&format).unwrap_or_default(),
                who: who(&entry.actor).to_owned(),
                what: describe(&entry.action).to_owned(),
            })
            .collect(),
    })
}
