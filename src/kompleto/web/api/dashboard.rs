//! The dashboard (H1): every Client with what's still missing, the next
//! Cut-off and the last message, nearest Cut-off first. Paused Clients
//! come last.

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use super::{ApiError, ApiFirm};
use crate::kompleto::checklist::owed_documents;
use crate::kompleto::dashboard::activity;
use crate::kompleto::periods::ClientCalendar;
use crate::kompleto::schedule::MANILA;
use crate::kompleto::uploads::uploaded_documents;
use crate::kompleto::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/dashboard", get(dashboard))
}

/// The dashboard.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Dashboard {
    /// Approved Reminders still unsent after [`HELD_AFTER`]: held by the
    /// daily send budget, or the sender is down.
    pub held: i64,
    /// Every Client, nearest Cut-off first, paused ones last.
    pub clients: Vec<DashboardClient>,
}

/// One Client's row.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DashboardClient {
    pub id: Uuid,
    pub business_name: String,
    /// The next Period with something due, e.g. "October 2026".
    pub period: Option<String>,
    /// Its Cut-off, `YYYY-MM-DD`.
    pub cut_off: Option<String>,
    /// Owed documents not yet uploaded for that Period.
    pub missing: Vec<String>,
    /// The latest Reminder.
    pub last_message: Option<LastMessage>,
    /// Drafts waiting for approval.
    pub drafts_waiting: i64,
    /// A person replied about the next Period.
    pub replied: bool,
    /// Only an automatic reply came about the next Period.
    pub auto_replied: bool,
    /// The Client's email bounced permanently or complained.
    pub bouncing: bool,
    /// A temporary bounce was reported.
    pub soft_bounced: bool,
    pub paused: bool,
}

/// A Client's latest Reminder.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct LastMessage {
    /// draft, approved, sent…
    pub status: String,
    /// When it was sent or last changed, RFC 3339.
    pub at: String,
}

/// How long an approved Reminder normally takes to go out, at most.
const HELD_AFTER: time::Duration = time::Duration::minutes(15);

async fn dashboard(State(state): State<AppState>, ApiFirm(firm): ApiFirm) -> Result<Json<Dashboard>, ApiError> {
    let clients = state.clients.list(firm).await?;
    let firm_offset = state.firms.get(firm).await?.cut_off_offset_days;
    let activity = activity(&state.pool, firm).await.map_err(ApiError::internal)?;
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
        let missing = match &next {
            Some(period) => {
                let edits = state.clients.checklist_edits(firm, client.id).await?;
                let uploaded = uploaded_documents(&state.pool, firm, client.id, &period.key())
                    .await
                    .map_err(ApiError::internal)?;
                owed_documents(&client.details.forms, period.month, &edits)
                    .into_iter()
                    .filter(|document| !uploaded.contains(document))
                    .collect()
            }
            None => Vec::new(),
        };
        let seen = activity.get(&client.id).cloned().unwrap_or_default();
        let key = next.as_ref().map(|period| period.key()).unwrap_or_default();
        let sort_date = next.as_ref().map(|period| period.cut_off);
        rows.push((
            (client.paused, sort_date.is_none(), sort_date),
            DashboardClient {
                id: client.id.as_uuid(),
                business_name: client.details.business_name,
                period: next.as_ref().map(|period| period.label.to_string()),
                cut_off: sort_date.map(|date| date.to_string()),
                missing,
                last_message: seen.last_message.map(|last| LastMessage {
                    status: last.status,
                    at: last.at.format(&Rfc3339).unwrap_or_default(),
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
    let held = sqlx::query_scalar!(
        r#"select count(*) as "count!" from outbox_messages
           where tenant_id = $1 and status = 'approved' and expires_at is not null and approved_at < $2"#,
        firm.as_uuid(),
        state.clock.now() - HELD_AFTER,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(ApiError::internal)?;
    Ok(Json(Dashboard {
        held,
        clients: rows.into_iter().map(|(_, row)| row).collect(),
    }))
}
