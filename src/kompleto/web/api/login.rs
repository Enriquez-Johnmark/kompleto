//! Logging in by email link (A1). The React app shows the link's page
//! with a "Log in" button; only pressing it posts here and uses the link,
//! so email link scanners can't use it up.

use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;

use super::{ApiError, ApiFirm};
use crate::kompleto::auth::{LoginOutcome, SESSION_LIFETIME};
use crate::kompleto::web::{AppState, SESSION_COOKIE, session_cookie};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/login", post(request_link))
        .route("/api/login/{token}", post(use_link))
        .route("/api/logout", post(log_out))
}

/// Asking for a login link.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct LoginRequest {
    /// The address to send it to.
    pub email: String,
}

async fn request_link(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> StatusCode {
    // The lookup and the email happen after the answer, so neither the
    // answer nor how long it takes reveals who has an account (A1).
    let (auth, wake_sender) = (state.auth, state.wake_sender);
    tokio::spawn(async move {
        match auth.request_link(&request.email).await {
            Ok(LoginOutcome::Sent) => wake_sender.notify_one(),
            Ok(LoginOutcome::NotSent) => {}
            Err(err) => tracing::error!(%err, "sending a login link failed"),
        }
    });
    StatusCode::ACCEPTED
}

async fn use_link(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, ApiError> {
    let Some(session) = state.auth.log_in(&token).await.map_err(ApiError::internal)? else {
        return Err(ApiError::invalid(
            "This login link has expired or was already used. Ask for a new one.",
        ));
    };
    let secure = if state.secure_cookies { "; Secure" } else { "" };
    let cookie = format!(
        "{SESSION_COOKIE}={session}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax{secure}",
        SESSION_LIFETIME.whole_seconds()
    );
    Ok(([(header::SET_COOKIE, cookie)], Json(serde_json::json!({}))).into_response())
}

async fn log_out(
    State(state): State<AppState>,
    _firm: ApiFirm,
    request: axum::extract::Request,
) -> Result<Response, ApiError> {
    let (parts, _) = request.into_parts();
    if let Some(session) = session_cookie(&parts) {
        state.auth.log_out(&session).await.map_err(ApiError::internal)?;
    }
    let cookie = format!("{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
    Ok(([(header::SET_COOKIE, cookie)], StatusCode::NO_CONTENT).into_response())
}
