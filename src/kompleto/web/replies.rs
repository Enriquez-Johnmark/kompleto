//! Reading a Reply (G1): the page a Reply notice links to. The Contact's
//! text is shown escaped, as plain text, and only to its own Firm.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::Method;
use axum::routing::{MethodRouter, get};
use uuid::Uuid;

use super::{AppState, CurrentFirm, ScopedRoute, WebError, show_date};
use crate::kompleto::calendar::TaxPeriod;
use crate::kompleto::replies::{ReplyError, ReplyId};
use crate::kompleto::schedule::MANILA;

pub(super) fn routes() -> Router<AppState> {
    super::scoped_router(reply_pages())
}

/// Every page about one Reply (A2).
pub(super) fn reply_pages() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    vec![(
        ScopedRoute {
            method: Method::GET,
            path: "/replies/{reply}",
            sample_form: "",
        },
        get(show),
    )]
}

impl From<ReplyError> for WebError {
    fn from(err: ReplyError) -> Self {
        match err {
            ReplyError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "reply.html")]
struct ReplyPage {
    client_id: String,
    business: String,
    period: String,
    from: String,
    received: String,
    automatic: bool,
    subject: Option<String>,
    text: Option<String>,
    attachments: i32,
}

async fn show(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
) -> Result<ReplyPage, WebError> {
    let reply = state.replies.get(firm, ReplyId::from(id)).await?;
    let client = state.clients.get(firm, reply.client).await?;
    Ok(ReplyPage {
        client_id: client.id.to_string(),
        business: client.details.business_name,
        period: TaxPeriod::parse_key(&reply.period).map_or(reply.period.clone(), |period| period.to_string()),
        from: reply.from,
        received: show_date(reply.received_at.to_offset(MANILA).date()),
        automatic: reply.automatic,
        subject: reply.subject,
        text: reply.text,
        attachments: reply.attachments,
    })
}
