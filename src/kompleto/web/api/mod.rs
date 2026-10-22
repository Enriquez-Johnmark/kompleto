//! The JSON API under `/api/` that the React app calls
//! (`docs/adr/0005-react-app-served-by-the-rust-binary.md`).
//!
//! - Every Firm endpoint takes an [`ApiFirm`] from the session cookie and
//!   passes its ID to the repositories, so another Firm's IDs are "not
//!   found" (A2).
//! - Changes must come from our own origin, as for the pages, and carry a
//!   JSON body, which a cross-site form can't send.
//! - Errors are JSON ([`ErrorBody`]) without internal details.

mod login;
mod me;

use axum::Json;
use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use serde::Serialize;

use super::{AppState, WebError, session_cookie};
use crate::kompleto::clients::ClientError;
use crate::kompleto::firms::{FirmError, FirmId};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .merge(login::routes())
        .merge(me::routes())
        .route("/api/{*rest}", any(not_found))
}

async fn not_found() -> ApiError {
    ApiError::NotFound
}

/// What every API error answers.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ErrorBody {
    /// One sentence for the bookkeeper.
    pub message: String,
    /// Each problem found, e.g. one per invalid field; may be empty.
    pub problems: Vec<String>,
}

/// An API failure.
#[derive(Debug)]
pub enum ApiError {
    /// Missing, or another Firm's (A2).
    NotFound,
    /// No live session.
    NotLoggedIn,
    /// The request was understood but refused, with the reason and
    /// any problems found.
    Refused {
        /// 409, 422 or 503.
        status: StatusCode,
        /// One sentence for the bookkeeper.
        message: String,
        /// Each problem, e.g. one per invalid field.
        problems: Vec<String>,
    },
    /// Logged; the bookkeeper sees a generic error.
    Internal,
}

impl ApiError {
    /// Logs `err` and answers with a generic error.
    pub fn internal(err: impl std::fmt::Display) -> Self {
        tracing::error!(%err, "API request failed");
        Self::Internal
    }

    /// Refused as unprocessable, with `message`.
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::Refused {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: message.into(),
            problems: Vec::new(),
        }
    }
}

impl From<WebError> for ApiError {
    fn from(err: WebError) -> Self {
        match err {
            WebError::NotFound => Self::NotFound,
            WebError::Internal => Self::Internal,
        }
    }
}

impl From<ClientError> for ApiError {
    fn from(err: ClientError) -> Self {
        WebError::from(err).into()
    }
}

impl From<FirmError> for ApiError {
    fn from(err: FirmError) -> Self {
        WebError::from(err).into()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message, problems) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found.".to_owned(), Vec::new()),
            Self::NotLoggedIn => (StatusCode::UNAUTHORIZED, "Log in first.".to_owned(), Vec::new()),
            Self::Refused {
                status,
                message,
                problems,
            } => (status, message, problems),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.".to_owned(),
                Vec::new(),
            ),
        };
        (status, Json(ErrorBody { message, problems })).into_response()
    }
}

/// The logged-in Firm, for API endpoints; without a live session the
/// answer is 401 rather than a redirect.
#[derive(Debug, Clone, Copy)]
pub struct ApiFirm(pub FirmId);

impl FromRequestParts<AppState> for ApiFirm {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = session_cookie(parts).ok_or(ApiError::NotLoggedIn)?;
        match state.auth.firm_for_session(&session).await {
            Ok(Some(firm)) => Ok(Self(firm)),
            Ok(None) => Err(ApiError::NotLoggedIn),
            Err(err) => Err(ApiError::internal(err)),
        }
    }
}
