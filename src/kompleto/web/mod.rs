//! Kompleto's server-rendered pages.
//!
//! - Every Firm page takes a [`CurrentFirm`] from the session cookie and
//!   passes its ID to the repositories, so another Firm's IDs are "not
//!   found" (A2).
//! - Form posts must come from our own pages: the `Origin` header has to
//!   match the configured base URL. Together with `SameSite=Lax` cookies
//!   this stops cross-site form posts.

mod activity;
mod clients;
mod dashboard;
mod drafts;
mod export;
mod hooks;
mod import;
mod login;
mod payors;
mod replies;
mod settings;
mod tracker;
mod upload;
mod uploads;

use std::collections::BTreeMap;
use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, get};
use secrecy::SecretString;
use sqlx::PgPool;
use tokio::sync::{Notify, Semaphore};
use tower_http::trace::TraceLayer;

use super::auth::Auth;
use super::bounces::Bounces;
use super::calendar::{Calendar, CalendarError};
use super::clients::{ClientError, Clients};
use super::dates::show_date;
use super::drafts::Drafts;
use super::firms::{FirmError, FirmId, Firms};
use super::import::xlsx::XlsxReader;
use super::payors::Payors;
use super::presend::KompletoChecks;
use super::replies::Replies;
use super::upload_links::UploadLinks;
use super::uploads::Uploads;
use crate::engine::clock::Clock;
use crate::engine::files::{Files, MasterKey};
use crate::engine::outbox::{Outbox, OutboxError};

/// The session cookie's name.
const SESSION_COOKIE: &str = "kompleto_session";

/// What every handler can reach.
#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    calendar: Arc<Calendar>,
    auth: Auth,
    firms: Firms,
    clients: Clients,
    drafts: Drafts,
    outbox: Outbox,
    checks: KompletoChecks,
    links: UploadLinks,
    uploads: Uploads,
    replies: Replies,
    payors: Payors,
    /// Uploads processed at once.
    upload_slots: Arc<Semaphore>,
    xlsx: XlsxReader,
    /// Wakes the outbox sender so a login email goes out at once.
    wake_sender: Arc<Notify>,
    /// `scheme://host[:port]` of the base URL; form posts must come from it.
    origin: String,
    /// Mark cookies `Secure` (when served over HTTPS).
    secure_cookies: bool,
    /// Inbound email, when `INBOUND_SECRET` is set.
    inbound: Option<Arc<Inbound>>,
    /// The provider's delivery events, when `RESEND_WEBHOOK_SECRET` is set.
    delivery_events: Option<Arc<DeliveryEvents>>,
}

/// Taking in Bounces and complaints.
struct DeliveryEvents {
    secret: SecretString,
    bounces: Bounces,
}

/// Taking in Replies.
struct Inbound {
    secret: SecretString,
    replies: Replies,
}

impl AppState {
    /// The app's state; links in emails point at `base_url`, Upload links
    /// are signed with a key from `master`, and Uploads kept in `files`.
    pub fn new(
        pool: PgPool,
        clock: Arc<dyn Clock>,
        base_url: &str,
        xlsx: XlsxReader,
        master: &MasterKey,
        files: Files,
    ) -> Result<Self, CalendarError> {
        let base_url = base_url.trim_end_matches('/');
        let origin = match base_url.split_once("://") {
            Some((scheme, rest)) => format!("{scheme}://{}", rest.split('/').next().unwrap_or(rest)),
            None => base_url.to_owned(),
        };
        let links = UploadLinks::new(pool.clone(), clock.clone(), master, base_url);
        let checks = KompletoChecks::new(pool.clone(), clock.clone(), base_url, links.clone());
        let replies = Replies::new(pool.clone(), clock.clone(), links.clone(), checks.clone(), base_url);
        Ok(Self {
            calendar: Arc::new(Calendar::built_in()?),
            auth: Auth::new(pool.clone(), clock.clone(), base_url, checks.clone()),
            firms: Firms::new(pool.clone(), clock.clone()),
            clients: Clients::new(pool.clone(), clock.clone()),
            drafts: Drafts::new(pool.clone(), clock.clone()),
            outbox: Outbox::new(pool.clone(), clock.clone()),
            checks,
            links,
            uploads: Uploads::new(pool.clone(), clock.clone(), files),
            replies,
            payors: Payors::new(pool.clone(), clock.clone()),
            upload_slots: Arc::new(Semaphore::new(upload::UPLOAD_SLOTS)),
            pool,
            clock,
            xlsx,
            wake_sender: Arc::new(Notify::new()),
            secure_cookies: base_url.starts_with("https://"),
            origin,
            inbound: None,
            delivery_events: None,
        })
    }

    /// Takes in the provider's delivery events signed with `secret`
    /// (`whsec_…`).
    #[must_use]
    pub fn with_delivery_events(mut self, secret: SecretString) -> Self {
        self.delivery_events = Some(Arc::new(DeliveryEvents {
            secret,
            bounces: Bounces::new(self.pool.clone(), self.clock.clone()),
        }));
        self
    }

    /// Takes in Replies signed with `secret`, at reply addresses on
    /// `reply_domain`.
    #[must_use]
    pub fn with_inbound(mut self, secret: SecretString, reply_domain: &str) -> Self {
        let links = self.links.clone().with_replies(reply_domain);
        self.inbound = Some(Arc::new(Inbound {
            secret,
            replies: self.replies.clone().matching(links).with_uploads(self.uploads.clone()),
        }));
        self
    }

    /// Notified whenever an email is approved and should go out now.
    pub fn sender_wake_up(&self) -> Arc<Notify> {
        self.wake_sender.clone()
    }
}

/// The web app.
pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(dashboard::routes())
        .merge(login::routes())
        .merge(clients::routes())
        .merge(drafts::routes())
        .merge(import::routes())
        .merge(settings::routes())
        .merge(upload::routes())
        .merge(uploads::routes())
        .merge(hooks::routes())
        .merge(replies::routes())
        .merge(payors::routes())
        .merge(tracker::routes())
        .merge(activity::routes())
        .merge(export::routes())
        .route("/static/kompleto.js", get(script))
        .layer(middleware::from_fn_with_state(state.clone(), same_origin_posts))
        .layer(middleware::from_fn_with_state(state.clone(), security_headers))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// What a page may load: our own script and nothing else from anywhere.
/// Styles are inline in each page. The script is a file, not inline, so no
/// injected script can run.
const CONTENT_SECURITY_POLICY: &str = "default-src 'none'; script-src 'self'; style-src 'unsafe-inline'; \
     img-src 'self'; connect-src 'self'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'";

/// Security headers on every response; HSTS only when served over HTTPS.
async fn security_headers(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CONTENT_SECURITY_POLICY),
    );
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
    if state.secure_cookies {
        headers.insert(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000"),
        );
    }
    response
}

/// Kompleto's one script (`static/kompleto.js`), built into the binary.
async fn script() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        include_str!("../../../static/kompleto.js"),
    )
}

/// Refuses form posts whose `Origin` isn't our own.
async fn same_origin_posts(State(state): State<AppState>, request: Request, next: Next) -> Response {
    // Machine endpoints prove themselves with signatures instead.
    let safe = matches!(*request.method(), Method::GET | Method::HEAD) || request.uri().path().starts_with("/hooks/");
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

/// A page about one of a Firm's records. The router is built from these,
/// and a test requests each with another Firm's IDs, expecting "not found"
/// (A2).
#[derive(Debug, Clone)]
pub struct ScopedRoute {
    /// GET or POST.
    pub method: Method,
    /// The path: `{id}` is a Client, `{item}` one of its extra checklist
    /// items, `{draft}` a reminder Draft, `{upload}` one of its Uploads,
    /// `{reply}` one of its Replies, `{payor}` one of its Payors.
    pub path: &'static str,
    /// A form body the page accepts, for tests; `{hash}` stands for the
    /// Draft's content hash.
    pub sample_form: &'static str,
}

/// Every page about one of a Firm's records.
pub fn scoped_routes() -> Vec<ScopedRoute> {
    clients::client_pages()
        .into_iter()
        .chain(drafts::draft_pages())
        .chain(uploads::upload_pages())
        .chain(replies::reply_pages())
        .chain(payors::payor_pages())
        .map(|(route, _)| route)
        .collect()
}

/// A router for `pages`, merging methods that share a path.
fn scoped_router(pages: Vec<(ScopedRoute, MethodRouter<AppState>)>) -> Router<AppState> {
    let mut by_path: BTreeMap<&str, MethodRouter<AppState>> = BTreeMap::new();
    for (route, handler) in pages {
        let merged = match by_path.remove(route.path) {
            Some(existing) => existing.merge(handler),
            None => handler,
        };
        by_path.insert(route.path, merged);
    }
    by_path
        .into_iter()
        .fold(Router::new(), |router, (path, handler)| router.route(path, handler))
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

/// Who is acting, for the audit log: the Firm's login email.
async fn actor(state: &AppState, firm: FirmId) -> Result<String, WebError> {
    Ok(state.firms.get(firm).await?.login_email)
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

impl From<OutboxError> for WebError {
    fn from(err: OutboxError) -> Self {
        match err {
            OutboxError::NotFound => Self::NotFound,
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
