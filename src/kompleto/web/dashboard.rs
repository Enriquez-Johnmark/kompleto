//! The dashboard (H1), the home page: every Client with what's still
//! missing, the next Cut-off and the last message, nearest Cut-off first.
//! Paused Clients come last.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use super::{AppState, CurrentFirm, WebError, show_date};
use crate::kompleto::checklist::owed_documents;
use crate::kompleto::dashboard::activity;
use crate::kompleto::periods::ClientCalendar;
use crate::kompleto::schedule::MANILA;
use crate::kompleto::uploads::uploaded_documents;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/", get(dashboard))
}

/// One Client's row.
struct Row {
    id: String,
    business: String,
    period: String,
    cut_off: String,
    missing: Vec<String>,
    last_message: String,
    drafts_waiting: i64,
    replied: bool,
    auto_replied: bool,
    bouncing: bool,
    soft_bounced: bool,
    paused: bool,
}

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
struct DashboardPage {
    rows: Vec<Row>,
}

async fn dashboard(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<DashboardPage, WebError> {
    let clients = state.clients.list(firm).await?;
    let firm_offset = state.firms.get(firm).await?.cut_off_offset_days;
    let activity = activity(&state.pool, firm).await.map_err(WebError::internal)?;
    let today = state.clock.now().to_offset(MANILA).date();
    let mut rows = Vec::with_capacity(clients.len());
    for client in clients {
        let overrides = state.clients.due_overrides(firm, client.id).await?;
        let calendar = ClientCalendar {
            calendar: &state.calendar,
            forms: &client.details.forms,
            overrides: &overrides,
            cut_off_offset_days: client.cut_off_offset_days.unwrap_or(firm_offset),
        };
        let next = calendar.upcoming(today).into_iter().next();
        let (missing, period, cut_off, sort_date) = match &next {
            Some(period) => {
                let edits = state.clients.checklist_edits(firm, client.id).await?;
                let uploaded = uploaded_documents(&state.pool, firm, client.id, &period.key())
                    .await
                    .map_err(WebError::internal)?;
                let missing = owed_documents(&client.details.forms, period.month, &edits)
                    .into_iter()
                    .filter(|document| !uploaded.contains(document))
                    .collect();
                (
                    missing,
                    period.label.to_string(),
                    show_date(period.cut_off),
                    Some(period.cut_off),
                )
            }
            None => (Vec::new(), String::new(), String::new(), None),
        };
        let seen = activity.get(&client.id).cloned().unwrap_or_default();
        let key = next.as_ref().map(|period| period.key()).unwrap_or_default();
        rows.push((
            (client.paused, sort_date.is_none(), sort_date),
            Row {
                id: client.id.to_string(),
                business: client.details.business_name,
                period,
                cut_off,
                missing,
                last_message: seen.last_message.map_or_else(String::new, |last| {
                    format!("{} {}", last.status, show_date(last.at.to_offset(MANILA).date()))
                }),
                drafts_waiting: seen.drafts_waiting,
                replied: seen.replied.contains(&key),
                auto_replied: seen.auto_replied.contains(&key),
                bouncing: seen.bouncing,
                soft_bounced: seen.soft_bounced,
                paused: client.paused,
            },
        ));
    }
    rows.sort_by_key(|(order, _)| *order);
    Ok(DashboardPage {
        rows: rows.into_iter().map(|(_, row)| row).collect(),
    })
}
