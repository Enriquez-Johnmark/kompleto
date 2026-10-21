//! Drives the web app in-process (no port), with the test clock.

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode, header};
use kompleto::engine::clock::FixedClock;
use kompleto::kompleto::firms::{FirmId, Firms};
use kompleto::kompleto::web::{AppState, router};
use sqlx::PgPool;
use tower::ServiceExt;

pub const BASE_URL: &str = "http://kompleto.test";

pub struct TestApp {
    pub router: Router,
    pub pool: PgPool,
    pub clock: Arc<FixedClock>,
}

pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl Response {
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
        let clock = super::clock();
        let xlsx = kompleto::kompleto::import::xlsx::XlsxReader::new(env!("CARGO_BIN_EXE_kompleto").into());
        let state = AppState::new(pool.clone(), clock.clone(), BASE_URL, xlsx).unwrap();
        Self {
            router: router(state),
            pool,
            clock,
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
        }
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
        self.request(Method::POST, "/login", None, Some(&format!("email={email}")))
            .await;
        let token = self.latest_login_token().await;
        let response = self.request(Method::POST, &format!("/login/{token}"), None, None).await;
        (firm, response.session_cookie().unwrap())
    }

    /// The token in the newest login email.
    pub async fn latest_login_token(&self) -> String {
        let body: String =
            sqlx::query_scalar("select body_text from outbox_messages order by created_at desc, id desc limit 1")
                .fetch_one(&self.pool)
                .await
                .unwrap();
        let marker = format!("{BASE_URL}/login/");
        let start = body.find(&marker).unwrap() + marker.len();
        body[start..].chars().take_while(char::is_ascii_alphanumeric).collect()
    }
}
