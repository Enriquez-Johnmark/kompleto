//! The Drafts endpoints: every waiting Reminder with its full text and
//! label, approval bound to the exact text shown (D2), batch approval
//! after a full review (D3), editing, discarding, and the text for "copy
//! for Viber" (E3). Approval runs the pre-send checks (D7) and refuses an
//! expired Draft (D4).

use axum::extract::{Path, State};
use axum::http::{Method, StatusCode};
use axum::routing::{MethodRouter, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ApiError, ApiFirm};
use crate::engine::outbox::{ContentHash, MessageStatus, OutboxError};
use crate::kompleto::drafting::DraftSource;
use crate::kompleto::drafts::Reminder;
use crate::kompleto::firms::FirmId;
use crate::kompleto::web::{AppState, ScopedRoute, actor, scoped_router};

pub(super) fn routes() -> Router<AppState> {
    scoped_router(draft_endpoints())
        .route("/api/drafts", get(list))
        .route("/api/drafts/approve", post(approve_batch))
}

/// Every endpoint about one Draft (A2).
pub(in crate::kompleto::web) fn draft_endpoints() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    let endpoint = |method, path, sample_form| ScopedRoute {
        method,
        path,
        sample_form,
    };
    vec![
        (endpoint(Method::GET, "/api/drafts/{draft}", ""), get(show)),
        (
            endpoint(
                Method::PUT,
                "/api/drafts/{draft}",
                r#"{"subject": "Kulang pa po", "body": "Paki-send po."}"#,
            ),
            axum::routing::put(save_edit),
        ),
        (
            endpoint(Method::POST, "/api/drafts/{draft}/approve", r#"{"hash": "{hash}"}"#),
            post(approve),
        ),
        (endpoint(Method::POST, "/api/drafts/{draft}/discard", ""), post(discard)),
    ]
}

/// One Reminder.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DraftView {
    pub id: Uuid,
    /// The Client's business name.
    pub client: String,
    /// The recipient: always the Client's stored email.
    pub to: String,
    pub subject: String,
    /// The full text, with the Client's upload link: also what "copy for
    /// Viber" copies (E3).
    pub body: String,
    /// Who wrote the wording.
    pub source: DraftSourceView,
    /// draft, approved, sending, sent, failed, blocked, unconfirmed…
    pub status: String,
    /// The Period's Cut-off, `YYYY-MM-DD`.
    pub cut_off: Option<String>,
    /// The fingerprint an approval sends back, binding it to this text (D2).
    pub hash: String,
    /// Why a message was held back after approval.
    pub problem: Option<String>,
}

/// Who wrote a Draft's wording.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum DraftSourceView {
    Ai,
    Template,
}

impl From<Reminder> for DraftView {
    fn from(reminder: Reminder) -> Self {
        Self {
            hash: reminder.content_hash().to_hex(),
            id: reminder.id.as_uuid(),
            client: reminder.client,
            to: reminder.email.to_address,
            subject: reminder.email.subject,
            body: reminder.email.body_text,
            source: match reminder.source {
                DraftSource::Ai => DraftSourceView::Ai,
                DraftSource::Template => DraftSourceView::Template,
            },
            status: reminder.status.as_str().to_owned(),
            cut_off: reminder.cut_off.map(|date| date.to_string()),
            problem: reminder.last_error.filter(|_| {
                matches!(
                    reminder.status,
                    MessageStatus::Failed | MessageStatus::Blocked | MessageStatus::Unconfirmed
                )
            }),
        }
    }
}

/// The Drafts page's data.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DraftList {
    /// Waiting for approval, nearest Cut-off first.
    pub waiting: Vec<DraftView>,
    /// Approved in the last week, with how each is going.
    pub approved: Vec<DraftView>,
}

async fn list(State(state): State<AppState>, ApiFirm(firm): ApiFirm) -> Result<Json<DraftList>, ApiError> {
    let waiting = state.drafts.waiting(firm).await.map_err(ApiError::internal)?;
    let approved = state.drafts.recently_approved(firm).await.map_err(ApiError::internal)?;
    Ok(Json(DraftList {
        waiting: waiting.into_iter().map(DraftView::from).collect(),
        approved: approved.into_iter().map(DraftView::from).collect(),
    }))
}

/// The Firm's Reminder `id`, or "not found" for anything else (A2).
async fn reminder(state: &AppState, firm: FirmId, id: Uuid) -> Result<Reminder, ApiError> {
    state
        .drafts
        .get(firm, id.into())
        .await
        .map_err(ApiError::internal)?
        .ok_or(ApiError::NotFound)
}

async fn show(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    Path(id): Path<Uuid>,
) -> Result<Json<DraftView>, ApiError> {
    Ok(Json(reminder(&state, firm, id).await?.into()))
}

/// Approving one Draft as shown.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Approval {
    /// The [`DraftView::hash`] of the text the bookkeeper saw.
    pub hash: String,
}

async fn approve(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    Path(id): Path<Uuid>,
    Json(approval): Json<Approval>,
) -> Result<StatusCode, ApiError> {
    let reminder = reminder(&state, firm, id).await?;
    let seen = ContentHash::from_hex(&approval.hash).ok_or_else(damaged)?;
    let approver = actor(&state, firm).await?;
    state
        .outbox
        .approve(firm, reminder.id, &approver, &seen, &state.checks)
        .await
        .map_err(refusal)?;
    state.wake_sender.notify_one();
    Ok(StatusCode::NO_CONTENT)
}

fn damaged() -> ApiError {
    ApiError::Refused {
        status: StatusCode::BAD_REQUEST,
        message: "That approval was damaged; reload the page and try again.".to_owned(),
        problems: Vec::new(),
    }
}

/// Why the outbox refused an approval, edit or discard, in the
/// bookkeeper's words.
fn refusal(err: OutboxError) -> ApiError {
    let (status, message) = match err {
        OutboxError::StaleApproval => (
            StatusCode::CONFLICT,
            "This draft changed since you saw it; check it again.".to_owned(),
        ),
        OutboxError::WrongStatus(_) => (
            StatusCode::CONFLICT,
            "This draft was already approved or discarded.".to_owned(),
        ),
        OutboxError::Expired => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "This draft expired at its cut-off and can't be sent.".to_owned(),
        ),
        OutboxError::CheckFailed(reason) => (StatusCode::UNPROCESSABLE_ENTITY, format!("Not approved: {reason}.")),
        OutboxError::InvalidMessage(reason) => (StatusCode::UNPROCESSABLE_ENTITY, format!("Not saved: {reason}.")),
        OutboxError::CheckUnavailable(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Couldn't check this draft just now; try again in a moment.".to_owned(),
        ),
        OutboxError::NotFound => return ApiError::NotFound,
        other => return ApiError::internal(other),
    };
    ApiError::Refused {
        status,
        message,
        problems: Vec::new(),
    }
}

async fn discard(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let reminder = reminder(&state, firm, id).await?;
    let actor = actor(&state, firm).await?;
    state.outbox.cancel(firm, reminder.id, &actor).await.map_err(refusal)?;
    Ok(StatusCode::NO_CONTENT)
}

/// New wording for a Draft. The recipient can't be changed.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DraftEdit {
    pub subject: String,
    pub body: String,
}

/// Saves new text. Any earlier approval is void (D2), and the new text is
/// checked again when approved (D7).
async fn save_edit(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    Path(id): Path<Uuid>,
    Json(edit): Json<DraftEdit>,
) -> Result<Json<DraftView>, ApiError> {
    let reminder = reminder(&state, firm, id).await?;
    let mut email = reminder.email.clone();
    email.subject = edit.subject.trim().to_owned();
    email.body_text = edit.body.replace("\r\n", "\n");
    let editor = actor(&state, firm).await?;
    state
        .outbox
        .edit(firm, reminder.id, &email, &editor)
        .await
        .map_err(refusal)?;
    Ok(Json(reminder_again(&state, firm, id).await?))
}

async fn reminder_again(state: &AppState, firm: FirmId, id: Uuid) -> Result<DraftView, ApiError> {
    Ok(reminder(state, firm, id).await?.into())
}

/// Approving several reviewed Drafts, each bound to the text shown (D3).
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BatchApproval {
    pub approvals: Vec<BatchItem>,
}

/// One reviewed Draft.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BatchItem {
    pub id: Uuid,
    /// The [`DraftView::hash`] the review showed.
    pub hash: String,
}

/// What happened to a batch.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BatchResult {
    pub approved: u32,
    /// Drafts not approved, each with the reason.
    pub refused: Vec<BatchRefusal>,
}

/// A Draft a batch didn't approve.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BatchRefusal {
    pub id: Uuid,
    pub client: String,
    pub reason: String,
}

/// Approves each reviewed Draft against the text the review showed; one
/// that changed meanwhile, or fails a check, is reported, not approved.
/// Every ID is checked before anything is approved (A2).
async fn approve_batch(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    Json(batch): Json<BatchApproval>,
) -> Result<Json<BatchResult>, ApiError> {
    let mut reviewed = Vec::with_capacity(batch.approvals.len());
    for item in batch.approvals {
        reviewed.push((reminder(&state, firm, item.id).await?, item.hash));
    }
    let approver = actor(&state, firm).await?;
    let mut result = BatchResult {
        approved: 0,
        refused: Vec::new(),
    };
    for (reminder, hash) in reviewed {
        let outcome = match ContentHash::from_hex(&hash) {
            Some(seen) => state
                .outbox
                .approve(firm, reminder.id, &approver, &seen, &state.checks)
                .await
                .map_err(refusal),
            None => Err(damaged()),
        };
        match outcome {
            Ok(()) => result.approved += 1,
            Err(ApiError::Refused { message, .. }) => result.refused.push(BatchRefusal {
                id: reminder.id.as_uuid(),
                client: reminder.client,
                reason: message,
            }),
            Err(other) => return Err(other),
        }
    }
    if result.approved > 0 {
        state.wake_sender.notify_one();
    }
    Ok(Json(result))
}
