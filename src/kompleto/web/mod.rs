//! Kompleto's server-rendered pages.
//!
//! - Every Firm page takes a [`CurrentFirm`] from the session cookie and
//!   passes its ID to the repositories, so another Firm's IDs are "not
//!   found" (A2).
//! - Form posts must come from our own pages: the `Origin` header has to
//!   match the configured base URL. Together with `SameSite=Lax` cookies
//!   this stops cross-site form posts.

mod clients;
mod import;
mod login;
mod settings;

use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::http::{Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use sqlx::PgPool;
use tokio::sync::Notify;
use tower_http::trace::TraceLayer;

use super::auth::Auth;
use super::calendar::{Calendar, CalendarError};
use super::clients::{ClientError, Clients};
use super::firms::{FirmError, FirmId, Firms};
use super::import::xlsx::XlsxReader;
use crate::engine::clock::Clock;

/// The session cookie's name.
const SESSION_COOKIE: &str = "kompleto_session";

/// What every handler can reach.
#[derive(Clone)]
pub struct AppState {
    clock: Arc<dyn Clock>,
    calendar: Arc<Calendar>,
    auth: Auth,
    firms: Firms,
    clients: Clients,
    xlsx: XlsxReader,
    /// Wakes the outbox sender so a login email goes out at once.
    wake_sender: Arc<Notify>,
    /// `scheme://host[:port]` of the base URL; form posts must come from it.
    origin: String,
    /// Mark cookies `Secure` (when served over HTTPS).
    secure_cookies: bool,
}

impl AppState {
    /// The app's state; links in emails point at `base_url`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, base_url: &str, xlsx: XlsxReader) -> Result<Self, CalendarError> {
        let base_url = base_url.trim_end_matches('/');
        let origin = match base_url.split_once("://") {
            Some((scheme, rest)) => format!("{scheme}://{}", rest.split('/').next().unwrap_or(rest)),
            None => base_url.to_owned(),
        };
        Ok(Self {
            calendar: Arc::new(Calendar::built_in()?),
            auth: Auth::new(pool.clone(), clock.clone(), base_url),
            firms: Firms::new(pool.clone(), clock.clone()),
            clients: Clients::new(pool, clock.clone()),
            clock,
            xlsx,
            wake_sender: Arc::new(Notify::new()),
            secure_cookies: base_url.starts_with("https://"),
            origin,
        })
    }

    /// Notified whenever an email is approved and should go out now.
    pub fn sender_wake_up(&self) -> Arc<Notify> {
        self.wake_sender.clone()
    }
}

/// The web app.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::to("/clients") }))
        .merge(login::routes())
        .merge(clients::routes())
        .merge(import::routes())
        .merge(settings::routes())
        .layer(middleware::from_fn_with_state(state.clone(), same_origin_posts))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Refuses form posts whose `Origin` isn't our own.
async fn same_origin_posts(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let safe = matches!(*request.method(), Method::GET | Method::HEAD);
    let ours = request
        .headers()
        .get(header::ORIGIN)
        .is_some_and(|origin| origin.as_bytes() == state.origin.as_bytes());
    if safe || ours {
        next.run(request).await
    } else {
        (StatusCode::FORBIDDEN, "Forbidden").into_response()
    }
}

/// A page about one Client. The router is built from these, and a test
/// requests each with another Firm's IDs, expecting "not found" (A2).
#[derive(Debug, Clone)]
pub struct ClientRoute {
    /// GET or POST.
    pub method: Method,
    /// The path; `{id}` is the Client, `{item}` one of its extra checklist items.
    pub path: &'static str,
    /// A form body the page accepts, for tests.
    pub sample_form: &'static str,
}

/// Every page about one Client.
pub fn client_routes() -> Vec<ClientRoute> {
    clients::client_pages().into_iter().map(|(route, _)| route).collect()
}

/// The logged-in Firm. Pages that take it send visitors without a live
/// session to the login page.
#[derive(Debug, Clone, Copy)]
pub struct CurrentFirm(pub FirmId);

impl FromRequestParts<AppState> for CurrentFirm {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let Some(session) = session_cookie(parts) else {
            return Err(Redirect::to("/login").into_response());
        };
        match state.auth.firm_for_session(&session).await {
            Ok(Some(firm)) => Ok(Self(firm)),
            Ok(None) => Err(Redirect::to("/login").into_response()),
            Err(err) => Err(WebError::internal(err).into_response()),
        }
    }
}

/// The session secret from the request's cookies.
fn session_cookie(parts: &Parts) -> Option<String> {
    parts
        .headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == SESSION_COOKIE)
        .map(|(_, value)| value.to_owned())
}

/// A date as pages show it: "Mon 26 Oct 2026".
fn show_date(date: time::Date) -> String {
    let format = time::macros::format_description!("[weekday repr:short] [day padding:none] [month repr:short] [year]");
    date.format(&format).unwrap_or_else(|_| date.to_string())
}

/// A handler failure, shown without internal details.
#[derive(Debug)]
pub enum WebError {
    /// Missing, or another Firm's (A2).
    NotFound,
    /// Logged; the visitor sees a generic error.
    Internal,
}

impl WebError {
    fn internal(err: impl std::fmt::Display) -> Self {
        tracing::error!(%err, "request failed");
        Self::Internal
    }
}

impl From<ClientError> for WebError {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

impl From<FirmError> for WebError {
    fn from(err: FirmError) -> Self {
        match err {
            FirmError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "not_found.html")]
struct NotFoundPage;

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, NotFoundPage).into_response(),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.",
            )
                .into_response(),
        }
    }
}
