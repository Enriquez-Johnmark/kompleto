//! The 2307 tracker (H2): one quarter, every Client's Payors.

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ApiError, ApiFirm};
use crate::kompleto::calendar::TaxPeriod;
use crate::kompleto::schedule::MANILA;
use crate::kompleto::tracker::{PayorStatus, quarter};
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/tracker", get(tracker))
}

/// One quarter's 2307s.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Tracker {
    /// e.g. "Q3 2026".
    pub quarter: String,
    /// e.g. "2026-Q3".
    pub key: String,
    /// The quarter before, e.g. "2026-Q2".
    pub earlier: String,
    /// The quarter after, unless it hasn't ended yet.
    pub later: Option<String>,
    pub clients: Vec<TrackedClient>,
}

/// One Client's quarter.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct TrackedClient {
    pub id: Uuid,
    pub business_name: String,
    pub payors: Vec<TrackedPayor>,
    /// Payors of uploaded 2307s that aren't listed, as the Contact typed them.
    pub unexpected: Vec<String>,
}

/// One listed Payor's 2307.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct TrackedPayor {
    pub name: String,
    /// `received` (the Firm marked it), `uploaded` (waiting to be checked)
    /// or `missing`.
    pub status: String,
}

#[derive(Debug, Deserialize)]
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
    ApiFirm(firm): ApiFirm,
    Query(shown): Query<Shown>,
) -> Result<Json<Tracker>, ApiError> {
    let today = state.clock.now().to_offset(MANILA).date();
    let current = (today.year(), (u8::from(today.month()) - 1) / 3 + 1);
    let last_ended = around(current.0, current.1).0;
    let (year, number) = match shown.quarter.as_deref().map(TaxPeriod::parse_key) {
        None => last_ended,
        Some(Some(TaxPeriod::Quarter { year, quarter })) => (year, quarter),
        Some(_) => return Err(ApiError::NotFound),
    };
    let clients = quarter(&state.pool, firm, year, number)
        .await
        .map_err(ApiError::internal)?;
    let (earlier, later) = around(year, number);
    Ok(Json(Tracker {
        quarter: TaxPeriod::Quarter { year, quarter: number }.to_string(),
        key: key((year, number)),
        earlier: key(earlier),
        later: (later <= last_ended).then(|| key(later)),
        clients: clients
            .into_iter()
            .map(|client| TrackedClient {
                id: client.id.as_uuid(),
                business_name: client.business_name,
                payors: client
                    .payors
                    .into_iter()
                    .map(|payor| TrackedPayor {
                        name: payor.name,
                        status: match payor.status {
                            PayorStatus::Received => "received",
                            PayorStatus::Uploaded => "uploaded",
                            PayorStatus::Missing => "missing",
                        }
                        .to_owned(),
                    })
                    .collect(),
                unexpected: client.unexpected,
            })
            .collect(),
    }))
}
