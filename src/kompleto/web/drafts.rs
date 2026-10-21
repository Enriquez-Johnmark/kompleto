//! The Drafts pages: every waiting reminder with its full text and label,
//! approve (bound to the exact text shown, D2), edit, discard, and "copy
//! for Viber" (E3). Approval runs the pre-send checks (D7) and refuses an
//! expired Draft (D4).

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, get, post};
use axum::{Form, Router};
use serde::Deserialize;
use uuid::Uuid;

use super::{AppState, CurrentFirm, ScopedRoute, WebError, show_date};
use crate::engine::ids::MessageId;
use crate::engine::outbox::{ContentHash, OutboxError};
use crate::kompleto::drafting::DraftSource;
use crate::kompleto::drafts::Reminder;
use crate::kompleto::firms::FirmId;

pub(super) fn routes() -> Router<AppState> {
    super::scoped_router(draft_pages())
        .route("/drafts", get(list))
        .route("/drafts/review", post(review))
        .route("/drafts/approve", post(approve_batch))
}

/// Every page about one Draft (A2).
pub(super) fn draft_pages() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    let page = |method, path, sample_form| ScopedRoute {
        method,
        path,
        sample_form,
    };
    vec![
        (page(Method::GET, "/drafts/{draft}/edit", ""), get(edit_form)),
        (
            page(
                Method::POST,
                "/drafts/{draft}",
                "subject=Kulang+pa+po&body=Paki-send+po.",
            ),
            post(save_edit),
        ),
        (
            page(Method::POST, "/drafts/{draft}/approve", "hash={hash}"),
            post(approve),
        ),
        (page(Method::POST, "/drafts/{draft}/discard", ""), post(discard)),
    ]
}

/// One reminder, ready to show.
struct ReminderView {
    id: MessageId,
    client: String,
    to: String,
    subject: String,
    body: String,
    label: &'static str,
    status: &'static str,
    cut_off: Option<String>,
    hash: String,
}

impl From<Reminder> for ReminderView {
    fn from(reminder: Reminder) -> Self {
        Self {
            hash: reminder.content_hash().to_hex(),
            id: reminder.id,
            client: reminder.client,
            to: reminder.email.to_address,
            subject: reminder.email.subject,
            body: reminder.email.body_text,
            label: match reminder.source {
                DraftSource::Ai => "AI",
                DraftSource::Template => "Template",
            },
            status: reminder.status.as_str(),
            cut_off: reminder.cut_off.map(show_date),
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "drafts.html")]
struct DraftsPage {
    drafts: Vec<ReminderView>,
    approved: Vec<ReminderView>,
    error: Option<String>,
}

async fn drafts_page(state: &AppState, firm: FirmId, error: Option<String>) -> Result<DraftsPage, WebError> {
    let waiting = state.drafts.waiting(firm).await.map_err(WebError::internal)?;
    let approved = state.drafts.recently_approved(firm).await.map_err(WebError::internal)?;
    Ok(DraftsPage {
        drafts: waiting.into_iter().map(ReminderView::from).collect(),
        approved: approved.into_iter().map(ReminderView::from).collect(),
        error,
    })
}

async fn list(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<DraftsPage, WebError> {
    drafts_page(&state, firm, None).await
}

/// The Firm's reminder `id`, or "not found" for anything else (A2).
async fn reminder(state: &AppState, firm: FirmId, id: Uuid) -> Result<Reminder, WebError> {
    state
        .drafts
        .get(firm, id.into())
        .await
        .map_err(WebError::internal)?
        .ok_or(WebError::NotFound)
}

/// Who is acting: the Firm's one login (J2).
async fn actor(state: &AppState, firm: FirmId) -> Result<String, WebError> {
    Ok(state.firms.get(firm).await?.login_email)
}

#[derive(Deserialize)]
struct ApproveForm {
    hash: String,
}

async fn approve(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<ApproveForm>,
) -> Result<Response, WebError> {
    let reminder = reminder(&state, firm, id).await?;
    let Some(seen) = ContentHash::from_hex(&form.hash) else {
        return refuse(
            &state,
            firm,
            StatusCode::BAD_REQUEST,
            "That approval form was damaged; try again.",
        )
        .await;
    };
    let approver = actor(&state, firm).await?;
    match state
        .outbox
        .approve(firm, reminder.id, &approver, &seen, &state.checks)
        .await
    {
        Ok(()) => {
            state.wake_sender.notify_one();
            Ok(Redirect::to("/drafts").into_response())
        }
        Err(err) => refused(&state, firm, err).await,
    }
}

/// The Drafts page again, explaining why an approval didn't go through.
pub(super) async fn refused(state: &AppState, firm: FirmId, err: OutboxError) -> Result<Response, WebError> {
    let (status, message) = approval_refusal(err)?;
    refuse(state, firm, status, &message).await
}

/// The status and words for an approval the outbox refused.
pub(super) fn approval_refusal(err: OutboxError) -> Result<(StatusCode, String), WebError> {
    Ok(match err {
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
        other => return Err(other.into()),
    })
}

async fn refuse(state: &AppState, firm: FirmId, status: StatusCode, message: &str) -> Result<Response, WebError> {
    let page = drafts_page(state, firm, Some(message.to_owned())).await?;
    Ok((status, page).into_response())
}

async fn discard(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
) -> Result<Response, WebError> {
    let reminder = reminder(&state, firm, id).await?;
    let actor = actor(&state, firm).await?;
    match state.outbox.cancel(firm, reminder.id, &actor).await {
        Ok(()) => Ok(Redirect::to("/drafts").into_response()),
        Err(err @ OutboxError::WrongStatus(_)) => refused(&state, firm, err).await,
        Err(err) => Err(err.into()),
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "draft_edit.html")]
struct EditPage {
    draft: ReminderView,
    error: Option<String>,
}

async fn edit_form(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
) -> Result<EditPage, WebError> {
    Ok(EditPage {
        draft: reminder(&state, firm, id).await?.into(),
        error: None,
    })
}

#[derive(Deserialize)]
struct EditForm {
    subject: String,
    body: String,
}

/// Saves new text. Only the subject and body change; any earlier approval
/// is void (D2), and the new text is checked again when approved (D7).
async fn save_edit(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<EditForm>,
) -> Result<Response, WebError> {
    let reminder = reminder(&state, firm, id).await?;
    let mut email = reminder.email.clone();
    email.subject = form.subject.trim().to_owned();
    // Browsers send CRLF line endings; store plain newlines.
    email.body_text = form.body.replace("\r\n", "\n");
    let editor = actor(&state, firm).await?;
    let (status, error) = match state.outbox.edit(firm, reminder.id, &email, &editor).await {
        Ok(()) => return Ok(Redirect::to("/drafts").into_response()),
        Err(OutboxError::InvalidMessage(reason)) => (StatusCode::UNPROCESSABLE_ENTITY, format!("Not saved: {reason}.")),
        Err(err) => approval_refusal(err)?,
    };
    let draft = ReminderView {
        subject: email.subject,
        body: email.body_text,
        ..reminder.into()
    };
    Ok((
        status,
        EditPage {
            draft,
            error: Some(error),
        },
    )
        .into_response())
}

#[derive(Template, WebTemplate)]
#[template(path = "drafts_review.html")]
struct ReviewPage {
    drafts: Vec<ReminderView>,
}

/// D3: the ticked Drafts, each in full, before any is approved. Another
/// Firm's ID makes the whole page "not found" (A2).
async fn review(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Form(pairs): Form<Vec<(String, String)>>,
) -> Result<Response, WebError> {
    let mut drafts = Vec::new();
    for (name, value) in pairs {
        if name != "ids" {
            continue;
        }
        let id = Uuid::parse_str(&value).map_err(|_| WebError::NotFound)?;
        drafts.push(ReminderView::from(reminder(&state, firm, id).await?));
    }
    if drafts.is_empty() {
        return refuse(
            &state,
            firm,
            StatusCode::UNPROCESSABLE_ENTITY,
            "Tick the drafts to review first.",
        )
        .await;
    }
    Ok(ReviewPage { drafts }.into_response())
}

#[derive(Template, WebTemplate)]
#[template(path = "drafts_result.html")]
struct ResultPage {
    approved: usize,
    refused: Vec<(String, String)>,
}

/// Approves each reviewed Draft against the text the review page showed;
/// one that changed meanwhile, or fails a check, is reported, not approved.
async fn approve_batch(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Form(pairs): Form<Vec<(String, String)>>,
) -> Result<Response, WebError> {
    let mut reviewed = Vec::new();
    for (name, value) in pairs {
        if name != "approve" {
            continue;
        }
        let (id, hash) = value.split_once(':').ok_or(WebError::NotFound)?;
        let id = Uuid::parse_str(id).map_err(|_| WebError::NotFound)?;
        // Every ID is checked before anything is approved (A2).
        reviewed.push((reminder(&state, firm, id).await?, ContentHash::from_hex(hash)));
    }
    let approver = actor(&state, firm).await?;
    let mut page = ResultPage {
        approved: 0,
        refused: Vec::new(),
    };
    for (reminder, seen) in reviewed {
        let Some(seen) = seen else {
            page.refused
                .push((reminder.client, "the approval form was damaged".to_owned()));
            continue;
        };
        match state
            .outbox
            .approve(firm, reminder.id, &approver, &seen, &state.checks)
            .await
        {
            Ok(()) => page.approved += 1,
            Err(err) => page.refused.push((reminder.client, approval_refusal(err)?.1)),
        }
    }
    if page.approved > 0 {
        state.wake_sender.notify_one();
    }
    Ok(page.into_response())
}
