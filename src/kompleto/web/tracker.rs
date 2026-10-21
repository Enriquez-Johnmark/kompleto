//! The 2307 tracker page (H2): one quarter, every Client's Payors.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use serde::Deserialize;

use super::{AppState, CurrentFirm, WebError};
use crate::kompleto::calendar::TaxPeriod;
use crate::kompleto::schedule::MANILA;
use crate::kompleto::tracker::{PayorStatus, TrackedClient, quarter};

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/tracker", get(tracker))
}

#[derive(Template, WebTemplate)]
#[template(path = "tracker.html")]
struct TrackerPage {
    quarter: String,
    key: String,
    earlier: String,
    later: Option<String>,
    clients: Vec<TrackedClient>,
}

impl TrackerPage {
    fn status(status: &PayorStatus) -> &'static str {
        match status {
            PayorStatus::Received => "received",
            PayorStatus::Uploaded => "uploaded",
            PayorStatus::Missing => "missing",
        }
    }
}

#[derive(Deserialize)]
struct Shown {
    quarter: Option<String>,
}

/// The quarter before `year`'s `quarter`, and the one after.
fn around(year: i32, quarter: u8) -> ((i32, u8), (i32, u8)) {
    let earlier = if quarter == 1 {
        (year - 1, 4)
    } else {
        (year, quarter - 1)
    };
    let later = if quarter == 4 {
        (year + 1, 1)
    } else {
        (year, quarter + 1)
    };
    (earlier, later)
}

fn key((year, quarter): (i32, u8)) -> String {
    TaxPeriod::Quarter { year, quarter }.key()
}

async fn tracker(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Query(shown): Query<Shown>,
) -> Result<TrackerPage, WebError> {
    let today = state.clock.now().to_offset(MANILA).date();
    let current = (today.year(), (u8::from(today.month()) - 1) / 3 + 1);
    let last_ended = around(current.0, current.1).0;
    let (year, number) = match shown.quarter.as_deref().map(TaxPeriod::parse_key) {
        None => last_ended,
        Some(Some(TaxPeriod::Quarter { year, quarter })) => (year, quarter),
        Some(_) => return Err(WebError::NotFound),
    };
    let clients = quarter(&state.pool, firm, year, number)
        .await
        .map_err(WebError::internal)?;
    let (earlier, later) = around(year, number);
    Ok(TrackerPage {
        quarter: TaxPeriod::Quarter { year, quarter: number }.to_string(),
        key: key((year, number)),
        earlier: key(earlier),
        later: (later <= last_ended).then(|| key(later)),
        clients,
    })
}
