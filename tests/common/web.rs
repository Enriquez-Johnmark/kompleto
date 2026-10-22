//! Drives the web app in-process (no port), with the test clock.

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode, header};
use kompleto::engine::clock::FixedClock;
use kompleto::engine::files::Files;
use kompleto::kompleto::firms::{FirmId, Firms};
use kompleto::kompleto::web::{AppState, router};
use sqlx::PgPool;
use tower::ServiceExt;

pub const BASE_URL: &str = "http://kompleto.test";
/// The secret the test app's delivery events are signed with.
/// Where the closed-account page tells a Firm to write.
pub const SUPPORT_EMAIL: &str = "help@getkompleto.test";

pub const WEBHOOK_SECRET: &str = "whsec_c2VjcmV0LWtleS1mb3ItdGVzdHM=";
/// The secret the test app's inbound email is signed with.
pub const INBOUND_SECRET: &str = "test-inbound-secret";

pub struct TestApp {
    pub router: Router,
    pub pool: PgPool,
    pub clock: Arc<FixedClock>,
    /// The app's own encrypted files, e.g. to run their deletion sweep.
    pub files: Files,
}

pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
    pub bytes: Vec<u8>,
}

impl Response {
    /// The body as JSON.
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_slice(&self.bytes).unwrap_or_else(|_| panic!("not JSON: {}", self.body))
    }

    pub fn location(&self) -> &str {
        self.headers
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
    }

    /// The `name=value` part of the session cookie this response sets.
    pub fn session_cookie(&self) -> Option<String> {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find(|v| v.starts_with("kompleto_session=") && !v.starts_with("kompleto_session=;"))
            .map(|v| v.split(';').next().unwrap_or("").to_owned())
    }
}

impl TestApp {
    pub fn new(pool: PgPool) -> Self {
        Self::at(pool, BASE_URL)
    }

    /// The app serving the built React app from `web_dir`.
    pub fn with_web_dir(pool: PgPool, web_dir: &std::path::Path) -> Self {
        Self::build(pool, BASE_URL, Some(web_dir))
    }

    /// The app served at `base_url`, which may have a path.
    pub fn at(pool: PgPool, base_url: &str) -> Self {
        Self::build(pool, base_url, None)
    }

    fn build(pool: PgPool, base_url: &str, web_dir: Option<&std::path::Path>) -> Self {
        let clock = super::clock();
        let xlsx = kompleto::kompleto::import::xlsx::XlsxReader::new(env!("CARGO_BIN_EXE_kompleto").into());
        let files = super::files(&pool, clock.clone());
        let state = AppState::new(
            pool.clone(),
            clock.clone(),
            base_url,
            xlsx,
            &super::master_key(),
            files.clone(),
        )
        .unwrap()
        .with_inbound(INBOUND_SECRET.to_owned().into(), "getkompleto.test")
        .with_delivery_events(WEBHOOK_SECRET.to_owned().into())
        .with_support_email(SUPPORT_EMAIL);
        let state = match web_dir {
            Some(dir) => state.with_web_dir(dir),
            None => state,
        };
        Self {
            router: router(state),
            pool,
            clock,
            files,
        }
    }

    pub async fn request(&self, method: Method, uri: &str, cookie: Option<&str>, form: Option<&str>) -> Response {
        let mut request = Request::builder().method(method.clone()).uri(uri);
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        if method != Method::GET {
            request = request.header(header::ORIGIN, BASE_URL);
        }
        let body = match form {
            Some(form) => {
                request = request.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
                Body::from(form.to_owned())
            }
            None => Body::empty(),
        };
        self.send(request.body(body).unwrap()).await
    }

    pub async fn send(&self, request: Request<Body>) -> Response {
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        Response {
            status,
            headers,
            body: String::from_utf8_lossy(&bytes).into_owned(),
            bytes: bytes.to_vec(),
        }
    }

    /// Posts a multipart form, as the upload page does; a field named
    /// `file` is sent as a file.
    pub async fn post_multipart(&self, uri: &str, fields: &[(&str, &[u8])]) -> Response {
        let boundary = "kompleto-test-boundary";
        let mut body = Vec::new();
        for (name, value) in fields {
            let filename = if *name == "file" {
                r#"; filename="photo.jpg""#
            } else {
                ""
            };
            body.extend_from_slice(
                format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"{filename}\r\n\r\n").as_bytes(),
            );
            body.extend_from_slice(value);
            body.extend_from_slice(b"\r\n");
        }
        body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
        let request = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(header::ORIGIN, BASE_URL)
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))
            .unwrap();
        self.send(request).await
    }

    /// Sends `json` as an API request, from our own origin unless it's a GET.
    pub async fn api(
        &self,
        method: Method,
        uri: &str,
        cookie: Option<&str>,
        json: Option<serde_json::Value>,
    ) -> Response {
        let mut request = Request::builder().method(method.clone()).uri(uri);
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        if method != Method::GET {
            request = request.header(header::ORIGIN, BASE_URL);
        }
        let body = match json {
            Some(json) => {
                request = request.header(header::CONTENT_TYPE, "application/json");
                Body::from(json.to_string())
            }
            None => Body::empty(),
        };
        self.send(request.body(body).unwrap()).await
    }

    pub async fn get(&self, uri: &str, cookie: &str) -> Response {
        self.request(Method::GET, uri, Some(cookie), None).await
    }

    pub async fn post(&self, uri: &str, cookie: &str, form: &str) -> Response {
        self.request(Method::POST, uri, Some(cookie), Some(form)).await
    }

    /// Creates a Firm and logs it in, returning its ID and session cookie.
    pub async fn logged_in_firm(&self, name: &str) -> (FirmId, String) {
        let email = format!("{}@firm.test", name.to_lowercase().replace(' ', "."));
        let firm = Firms::new(self.pool.clone(), self.clock.clone())
            .create(name, &email)
            .await
            .unwrap();
        (firm, self.log_in(&email).await)
    }

    /// Logs in as the Firm whose login address is `email`; returns the
    /// session cookie.
    pub async fn log_in(&self, email: &str) -> String {
        let earlier: i64 = sqlx::query_scalar("select count(*) from outbox_messages where to_address = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .unwrap();
        self.request(Method::POST, "/login", None, Some(&format!("email={email}")))
            .await;
        let token = self.login_token_after(email, earlier).await;
        let response = self.request(Method::POST, &format!("/login/{token}"), None, None).await;
        response.session_cookie().unwrap()
    }

    /// The token in the newest login email to `email`. The email is made
    /// after the login page replies, so this waits for it.
    pub async fn login_token_for(&self, email: &str) -> String {
        self.login_token_after(email, 0).await
    }

    /// The token in the newest login email to `email`, once more than
    /// `earlier` have been made.
    async fn login_token_after(&self, email: &str, earlier: i64) -> String {
        let mut body = None;
        for _ in 0..100 {
            body = sqlx::query_scalar(
                "select body_text from outbox_messages where to_address = $1
                 and (select count(*) from outbox_messages where to_address = $1) > $2
                 order by created_at desc, id desc limit 1",
            )
            .bind(email)
            .bind(earlier)
            .fetch_optional(&self.pool)
            .await
            .unwrap();
            if body.is_some() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        let Some(body): Option<String> = body else {
            panic!("no login email to {email} arrived");
        };
        let marker = format!("{BASE_URL}/login/");
        let start = body.find(&marker).unwrap() + marker.len();
        body[start..].chars().take_while(char::is_ascii_alphanumeric).collect()
    }
}
