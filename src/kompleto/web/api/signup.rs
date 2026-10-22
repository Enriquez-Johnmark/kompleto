//! The public "Request an account" endpoint (M10, L1). It answers the same
//! whether or not the address already has an account.

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;

use super::{ApiError, ApiJson};
use crate::kompleto::signups::SignupError;
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/signup", post(signup))
}

/// A request for an account.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SignupRequest {
    /// The firm's name, as its Clients will see it.
    pub firm_name: String,
    /// The owner's email address.
    pub email: String,
    /// Whether they accepted the terms and the data processing agreement.
    pub accepted_terms: bool,
}

async fn signup(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<SignupRequest>,
) -> Result<StatusCode, ApiError> {
    match state
        .signups
        .request(&request.firm_name, &request.email, request.accepted_terms)
        .await
    {
        Ok(()) => Ok(StatusCode::ACCEPTED),
        Err(SignupError::Invalid(problems)) => Err(ApiError::Refused {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: "Check the form and try again.".to_owned(),
            problems,
        }),
        Err(SignupError::TooMany) => Err(ApiError::Refused {
            status: StatusCode::SERVICE_UNAVAILABLE,
            message: "Kompleto can't take new requests just now. Please try again in a few days.".to_owned(),
            problems: Vec::new(),
        }),
        Err(err) => Err(ApiError::internal(err)),
    }
}
