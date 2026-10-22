//! Reading a Reply (G1): what a Reply notice links to. The Contact's text
//! is data for the app to show as plain text, and only to its own Firm.

use axum::extract::{Path, State};
use axum::http::Method;
use axum::routing::{MethodRouter, get};
use axum::{Json, Router};
use serde::Serialize;
use uuid::Uuid;

use super::client_page::{period_label, rfc3339};
use super::{ApiError, ApiFirm};
use crate::kompleto::replies::ReplyId;
use crate::kompleto::web::{AppState, ScopedRoute, scoped_router};

pub(super) fn routes() -> Router<AppState> {
    scoped_router(reply_endpoints())
}

/// Every endpoint about one Reply (A2).
pub(in crate::kompleto::web) fn reply_endpoints() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    vec![(
        ScopedRoute {
            method: Method::GET,
            path: "/api/replies/{reply}",
            sample_form: "",
        },
        get(show),
    )]
}

/// One Reply.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReplyDetail {
    pub id: Uuid,
    pub client_id: Uuid,
    pub business_name: String,
    /// The Period's label.
    pub period: String,
    /// The sender's address.
    pub from: String,
    /// RFC 3339.
    pub received_at: String,
    /// An out-of-office or similar.
    pub automatic: bool,
    /// It came through its signed reply address. If not, it was matched by
    /// the sender's address only, which anyone can forge.
    pub signed: bool,
    /// Until it's deleted with the text after 90 days.
    pub subject: Option<String>,
    /// The Contact's text, untrusted: shown as plain text, never as HTML.
    pub text: Option<String>,
    /// Photos and PDFs kept as Uploads.
    pub attachments: i32,
}

async fn show(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    Path(id): Path<Uuid>,
) -> Result<Json<ReplyDetail>, ApiError> {
    let reply = state.replies.get(firm, ReplyId::from(id)).await?;
    let client = state.clients.get(firm, reply.client).await?;
    Ok(Json(ReplyDetail {
        id: reply.id.as_uuid(),
        client_id: client.id.as_uuid(),
        business_name: client.details.business_name,
        period: period_label(&reply.period),
        from: reply.from,
        received_at: rfc3339(reply.received_at),
        automatic: reply.automatic,
        signed: reply.signed,
        subject: reply.subject,
        text: reply.text,
        attachments: reply.attachments,
    }))
}
