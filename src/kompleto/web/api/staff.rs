//! The Firm's staff (M9, K1, K2): only the owner lists, invites and removes
//! them.

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::client_page::rfc3339;
use super::{ApiError, ApiJson, ApiOwner, ApiPath};
use crate::kompleto::auth::LoginOutcome;
use crate::kompleto::members::{MAX_STAFF, Member, MemberError};
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/staff", get(list).post(invite))
        .route("/api/staff/{member}", delete(remove))
}

/// The Firm's members, owner first.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct StaffList {
    /// The owner, then staff by when they were added.
    pub members: Vec<MemberView>,
    /// The most staff the Firm can have.
    pub max_staff: i64,
}

/// One person who logs in.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemberView {
    /// Their ID.
    pub id: String,
    /// Their address.
    pub email: String,
    /// `owner` or `staff`.
    pub role: String,
    /// When they were added (RFC 3339).
    pub added_at: String,
}

impl From<Member> for MemberView {
    fn from(member: Member) -> Self {
        Self {
            id: member.id.to_string(),
            email: member.email,
            role: member.role.as_str().to_owned(),
            added_at: rfc3339(member.added_at),
        }
    }
}

/// A staff member just added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Invited {
    /// Who was added.
    pub member: MemberView,
    /// Whether their invitation with a login link was emailed; if not, they
    /// can ask for a link on the login page.
    pub email_sent: bool,
}

/// A new staff member's address.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Invite {
    /// Their email address.
    pub email: String,
}

impl From<MemberError> for ApiError {
    fn from(err: MemberError) -> Self {
        let conflict = |message: &str| Self::Refused {
            status: StatusCode::CONFLICT,
            message: message.to_owned(),
            problems: Vec::new(),
        };
        match err {
            MemberError::Invalid(_) => Self::invalid("Enter their email address."),
            MemberError::EmailTaken => conflict("That address already logs in to Kompleto."),
            MemberError::TooManyStaff => conflict("A firm can have at most 5 staff. Remove one first."),
            MemberError::IsOwner => conflict("The owner can't be removed."),
            MemberError::NotFound => Self::NotFound,
            MemberError::Database(err) => Self::internal(err),
        }
    }
}

async fn list(State(state): State<AppState>, ApiOwner(owner): ApiOwner) -> Result<Json<StaffList>, ApiError> {
    let members = state.members.list(owner.firm).await?;
    Ok(Json(StaffList {
        members: members.into_iter().map(MemberView::from).collect(),
        max_staff: MAX_STAFF,
    }))
}

async fn invite(
    State(state): State<AppState>,
    ApiOwner(owner): ApiOwner,
    ApiJson(invite): ApiJson<Invite>,
) -> Result<(StatusCode, Json<Invited>), ApiError> {
    let added = state.members.add_staff(owner.firm, &invite.email, &owner.email).await?;
    // They're added either way; without the email they can still ask for
    // a link on the login page, and the owner is told.
    let email_sent = match state.auth.invite(&added.email, &owner.email).await {
        Ok(outcome) => outcome == LoginOutcome::Sent,
        Err(err) => {
            tracing::error!(firm = %owner.firm, %err, "emailing a new staff member failed");
            false
        }
    };
    state.wake_sender.notify_one();
    Ok((
        StatusCode::CREATED,
        Json(Invited {
            member: added.into(),
            email_sent,
        }),
    ))
}

async fn remove(
    State(state): State<AppState>,
    ApiOwner(owner): ApiOwner,
    ApiPath(member): ApiPath<Uuid>,
) -> Result<StatusCode, ApiError> {
    state
        .members
        .remove_staff(owner.firm, member.into(), &owner.email)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
