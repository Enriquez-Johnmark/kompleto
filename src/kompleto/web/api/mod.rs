//! The JSON API under `/api/` that the React app calls
//! (`docs/adr/0005-react-app-served-by-the-rust-binary.md`).
//!
//! - Every Firm endpoint takes an [`ApiFirm`] from the session cookie and
//!   passes its ID to the repositories, so another Firm's IDs are "not
//!   found" (A2).
//! - Changes must come from our own origin, as for the pages, and carry a
//!   JSON body, which a cross-site form can't send.
//! - Errors are JSON ([`ErrorBody`]) without internal details.

mod activity;
mod calendar;
mod client_changes;
mod client_page;
mod clients;
mod dashboard;
mod drafts;
mod export;
mod files;
mod import;
mod login;
mod me;
mod replies;
mod settings;
mod signup;
mod staff;
mod tracker;

use axum::Json;
use axum::Router;
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{AppState, WebError, session_cookie};
use crate::kompleto::auth::Session;
use crate::kompleto::clients::ClientError;
use crate::kompleto::firms::{FirmError, FirmId};
use crate::kompleto::members::Role;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .merge(activity::routes())
        .merge(calendar::routes())
        .merge(client_changes::routes())
        .merge(client_page::routes())
        .merge(clients::routes())
        .merge(dashboard::routes())
        .merge(drafts::routes())
        .merge(export::routes())
        .merge(files::routes())
        .merge(import::routes())
        .merge(replies::routes())
        .merge(settings::routes())
        .merge(signup::routes())
        .merge(staff::routes())
        .merge(tracker::routes())
        .merge(login::routes())
        .merge(me::routes())
        .route("/api/{*rest}", any(not_found))
}

/// Every endpoint about one of a Firm's records (A2).
pub(super) fn scoped_endpoints() -> Vec<(super::ScopedRoute, axum::routing::MethodRouter<AppState>)> {
    drafts::draft_endpoints()
        .into_iter()
        .chain(clients::client_endpoints())
        .chain(client_page::page_endpoints())
        .chain(client_changes::change_endpoints())
        .chain(files::file_endpoints())
        .chain(replies::reply_endpoints())
        .collect()
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
        /// 403, 409, 422 or 503.
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

impl From<crate::kompleto::uploads::UploadError> for ApiError {
    fn from(err: crate::kompleto::uploads::UploadError) -> Self {
        match err {
            crate::kompleto::uploads::UploadError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

impl From<crate::kompleto::replies::ReplyError> for ApiError {
    fn from(err: crate::kompleto::replies::ReplyError) -> Self {
        match err {
            crate::kompleto::replies::ReplyError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
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

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::Refused {
            status: rejection.status(),
            message: "That wasn't understood; check what you typed and try again.".to_owned(),
            problems: vec![rejection.body_text()],
        }
    }
}

impl From<PathRejection> for ApiError {
    /// A malformed ID in the address names nothing: "not found" (A2).
    fn from(_: PathRejection) -> Self {
        Self::NotFound
    }
}

/// A JSON body; one that can't be read is refused as an [`ErrorBody`],
/// like every other API error.
#[derive(Debug)]
pub struct ApiJson<T>(pub T);

impl<T: DeserializeOwned, S: Send + Sync> FromRequest<S> for ApiJson<T> {
    type Rejection = ApiError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state).await?;
        Ok(Self(value))
    }
}

/// Values from the path; a malformed one is "not found" (A2).
#[derive(Debug)]
pub struct ApiPath<T>(pub T);

impl<T: DeserializeOwned + Send, S: Send + Sync> FromRequestParts<S> for ApiPath<T> {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Path(value) = axum::extract::Path::<T>::from_request_parts(parts, state).await?;
        Ok(Self(value))
    }
}

/// The logged-in Firm, for API endpoints; without a live session the
/// answer is 401 rather than a redirect.
#[derive(Debug, Clone, Copy)]
pub struct ApiFirm(pub FirmId);

impl FromRequestParts<AppState> for ApiFirm {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let ApiMember(member) = ApiMember::from_request_parts(parts, state).await?;
        Ok(Self(member.firm))
    }
}

/// The logged-in person (M9), owner or staff; answers 401 when there's no
/// live session.
pub struct ApiMember(pub Session);

impl FromRequestParts<AppState> for ApiMember {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = session_cookie(parts).ok_or(ApiError::NotLoggedIn)?;
        match state.auth.session(&session).await {
            Ok(Some(member)) => Ok(Self(member)),
            Ok(None) => Err(ApiError::NotLoggedIn),
            Err(err) => Err(ApiError::internal(err)),
        }
    }
}

/// The logged-in Firm's owner; staff are refused with 403 (K2).
pub struct ApiOwner(pub Session);

impl FromRequestParts<AppState> for ApiOwner {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let ApiMember(member) = ApiMember::from_request_parts(parts, state).await?;
        if member.role != Role::Owner {
            return Err(ApiError::Refused {
                status: StatusCode::FORBIDDEN,
                message: "Only the firm's owner can do this.".to_owned(),
                problems: Vec::new(),
            });
        }
        Ok(Self(member))
    }
}
