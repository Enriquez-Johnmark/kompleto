//! Login pages (A1). Opening a link shows a "Log in" button; only pressing
//! it uses the link, so email link scanners can't use it up.

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;

use super::{AppState, CurrentFirm, SESSION_COOKIE, WebError, session_cookie};
use crate::kompleto::auth::{LoginOutcome, SESSION_LIFETIME};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_page).post(request_link))
        .route("/login/{token}", get(link_page).post(use_link))
        .route("/logout", post(log_out))
}

#[derive(Template, WebTemplate)]
#[template(path = "login.html")]
struct LoginPage;

#[derive(Template, WebTemplate)]
#[template(path = "login_sent.html")]
struct LinkSentPage;

#[derive(Template, WebTemplate)]
#[template(path = "login_link.html")]
struct LinkPage {
    token: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "login_failed.html")]
struct LinkFailedPage;

async fn login_page() -> LoginPage {
    LoginPage
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
}

async fn request_link(State(state): State<AppState>, Form(form): Form<LoginForm>) -> Result<LinkSentPage, WebError> {
    let outcome = state.auth.request_link(&form.email).await.map_err(WebError::internal)?;
    if outcome == LoginOutcome::Sent {
        state.wake_sender.notify_one();
    }
    // The same page either way, so it never reveals who has an account.
    Ok(LinkSentPage)
}

async fn link_page(Path(token): Path<String>) -> LinkPage {
    // Only hex digits ever appear in a token; drop anything else.
    LinkPage {
        token: token.chars().filter(char::is_ascii_hexdigit).collect(),
    }
}

async fn use_link(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, WebError> {
    let Some(session) = state.auth.log_in(&token).await.map_err(WebError::internal)? else {
        return Ok(LinkFailedPage.into_response());
    };
    let secure = if state.secure_cookies { "; Secure" } else { "" };
    let cookie = format!(
        "{SESSION_COOKIE}={session}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax{secure}",
        SESSION_LIFETIME.whole_seconds()
    );
    Ok(([(header::SET_COOKIE, cookie)], Redirect::to("/clients")).into_response())
}

async fn log_out(
    State(state): State<AppState>,
    _firm: CurrentFirm,
    request: axum::extract::Request,
) -> Result<Response, WebError> {
    let (parts, _) = request.into_parts();
    if let Some(session) = session_cookie(&parts) {
        state.auth.log_out(&session).await.map_err(WebError::internal)?;
    }
    let cookie = format!("{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
    Ok(([(header::SET_COOKIE, cookie)], Redirect::to("/login")).into_response())
}
