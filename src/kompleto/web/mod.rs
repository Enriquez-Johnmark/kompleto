//! Kompleto's web layer: the JSON API the React app calls (`api`), the
//! built React app itself (`spa`), the Client's server-rendered upload
//! page (`upload`) and the signed provider hooks (`hooks`).
//!
//! - Every Firm endpoint takes an [`api::ApiFirm`] from the session cookie
//!   and passes its ID to the repositories, so another Firm's IDs are "not
//!   found" (A2).
//! - Changes must come from our own pages: the `Origin` header has to
//!   match the configured base URL. Together with `SameSite=Lax` cookies
//!   this stops cross-site posts.

mod api;
mod hooks;
mod spa;
mod upload;

use std::collections::BTreeMap;
use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
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
use super::firms::{FirmError, Firms};
use super::import::xlsx::XlsxReader;
use super::members::Members;
use super::payors::Payors;
use super::presend::KompletoChecks;
use super::replies::Replies;
use super::upload_links::UploadLinks;
use super::uploads::Uploads;
use crate::engine::clock::Clock;
use crate::engine::files::{Files, MasterKey};
use crate::engine::outbox::{Outbox, OutboxError};
use crate::engine::scan::Clamd;

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
    members: Members,
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
    /// Where Firms write for help (`SUPPORT_EMAIL`), if set.
    support_email: Option<String>,
    /// Inbound email, when `INBOUND_SECRET` is set.
    inbound: Option<Arc<Inbound>>,
    /// The provider's delivery events, when `RESEND_WEBHOOK_SECRET` is set.
    delivery_events: Option<Arc<DeliveryEvents>>,
    /// The built React app (`WEB_DIR`), when served by this server.
    web_dir: Option<Arc<std::path::PathBuf>>,
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
    /// are signed with a key from `master`, and Uploads kept in `files`
    /// after `scanner`, if any, finds them clean.
    pub fn new(
        pool: PgPool,
        clock: Arc<dyn Clock>,
        base_url: &str,
        xlsx: XlsxReader,
        master: &MasterKey,
        files: Files,
        scanner: Option<Clamd>,
    ) -> Result<Self, CalendarError> {
        let base_url = base_url.trim_end_matches('/');
        let origin = match base_url.split_once("://") {
            Some((scheme, rest)) => format!("{scheme}://{}", rest.split('/').next().unwrap_or(rest)),
            None => base_url.to_owned(),
        };
        let links = UploadLinks::new(pool.clone(), clock.clone(), master, base_url);
        let checks = KompletoChecks::new(pool.clone(), clock.clone(), base_url, links.clone());
        let replies = Replies::new(pool.clone(), clock.clone(), links.clone(), checks.clone(), base_url);
        let uploads = Uploads::new(pool.clone(), clock.clone(), files);
        let uploads = match scanner {
            Some(clamd) => uploads.scanned_by(clamd),
            None => uploads,
        };
        Ok(Self {
            calendar: Arc::new(Calendar::built_in()?),
            auth: Auth::new(pool.clone(), clock.clone(), base_url, checks.clone()),
            firms: Firms::new(pool.clone(), clock.clone()),
            members: Members::new(pool.clone(), clock.clone()),
            clients: Clients::new(pool.clone(), clock.clone()),
            drafts: Drafts::new(pool.clone(), clock.clone()),
            outbox: Outbox::new(pool.clone(), clock.clone()),
            checks,
            links,
            uploads,
            replies,
            payors: Payors::new(pool.clone(), clock.clone()),
            upload_slots: Arc::new(Semaphore::new(upload::UPLOAD_SLOTS)),
            pool,
            clock,
            xlsx,
            wake_sender: Arc::new(Notify::new()),
            secure_cookies: base_url.starts_with("https://"),
            support_email: None,
            origin,
            inbound: None,
            delivery_events: None,
            web_dir: None,
        })
    }

    /// Serves the built React app from `dir` (`web/dist`).
    #[must_use]
    pub fn with_web_dir(mut self, dir: &std::path::Path) -> Self {
        if !spa::is_built(dir) {
            tracing::warn!(dir = %dir.display(), "the React app isn't built there; run `pnpm --dir web build`");
        }
        self.web_dir = Some(Arc::new(dir.to_owned()));
        self
    }

    /// Where Firms write for help, e.g. to undo closing an account.
    #[must_use]
    pub fn with_support_email(mut self, address: &str) -> Self {
        self.support_email = Some(address.trim().to_owned());
        self
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
        .merge(api::routes())
        .merge(upload::routes())
        .merge(hooks::routes())
        .route("/static/kompleto.js", get(script))
        .route("/static/kompleto.css", get(stylesheet))
        .route("/health", get(health))
        .fallback(spa::app)
        .layer(middleware::from_fn_with_state(state.clone(), same_origin_posts))
        .layer(middleware::from_fn_with_state(state.clone(), security_headers))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// What a page may load: our own scripts, styles and fonts, and nothing
/// from anywhere else. Scripts are files, never inline, so no injected
/// script can run; inline styles stay allowed for the server-rendered
/// pages and the components' positioning.
const CONTENT_SECURITY_POLICY: &str = "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; \
     img-src 'self'; font-src 'self'; connect-src 'self'; manifest-src 'self'; form-action 'self'; \
     frame-ancestors 'none'; base-uri 'none'";

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

/// "ok" while the database and virus scanner answer, for an uptime monitor;
/// the reason for a failure goes to the log, not to whoever asked.
async fn health(State(state): State<AppState>) -> Response {
    let unavailable = (StatusCode::SERVICE_UNAVAILABLE, "unavailable").into_response();
    if let Err(err) = sqlx::query_scalar!("select 1 as \"one!\"").fetch_one(&state.pool).await {
        tracing::error!(%err, "health check: the database didn't answer");
        return unavailable;
    }
    // Without the scanner, every Upload is refused.
    if let Err(err) = state.uploads.scanner_answers().await {
        tracing::error!(%err, "health check: the virus scanner didn't answer");
        return unavailable;
    }
    "ok".into_response()
}

/// The upload pages' stylesheet (`static/kompleto.css`), built into the binary.
async fn stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../../static/kompleto.css"),
    )
}

/// The upload page's script (`static/kompleto.js`), built into the binary.
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
    /// GET, POST or PUT.
    pub method: Method,
    /// The path: `{id}` is a Client, `{item}` one of its extra checklist
    /// items, `{draft}` a reminder Draft, `{upload}` one of its Uploads,
    /// `{reply}` one of its Replies, `{payor}` one of its Payors.
    pub path: &'static str,
    /// A body the page accepts, for tests: a form, or JSON for `/api/`
    /// paths; `{hash}` stands for the Draft's content hash.
    pub sample_form: &'static str,
}

/// Every page about one of a Firm's records.
pub fn scoped_routes() -> Vec<ScopedRoute> {
    api::scoped_endpoints().into_iter().map(|(route, _)| route).collect()
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
