//! What the dashboard (H1) shows besides the calendar: each Client's latest
//! Reminder, Drafts waiting, Replies and Bounces, read for one Firm at a
//! time.

use std::collections::{BTreeSet, HashMap};

use sqlx::PgPool;
use time::OffsetDateTime;

use super::clients::ClientId;
use super::firms::FirmId;

/// A Client's latest Reminder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastMessage {
    /// Where it is: draft, approved, sent…
    pub status: String,
    /// When it was sent, or last changed.
    pub at: OffsetDateTime,
}

/// One Client's activity.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Activity {
    /// The latest Reminder, if any.
    pub last_message: Option<LastMessage>,
    /// Drafts waiting for approval.
    pub drafts_waiting: i64,
    /// Periods with a human Reply.
    pub replied: BTreeSet<String>,
    /// Periods with only an automatic reply.
    pub auto_replied: BTreeSet<String>,
    /// The Client's current email bounced permanently or complained.
    pub bouncing: bool,
    /// A temporary Bounce was reported.
    pub soft_bounced: bool,
}

/// Every Client's activity in `firm`, by Client.
pub async fn activity(pool: &PgPool, firm: FirmId) -> Result<HashMap<ClientId, Activity>, sqlx::Error> {
    let mut by_client: HashMap<ClientId, Activity> = HashMap::new();
    let last = sqlx::query!(
        r#"select distinct on (s.client_id) s.client_id, m.status,
                  coalesce(m.sent_at, m.updated_at) as "at!"
           from scheduled_drafts s join outbox_messages m on m.id = s.message_id and m.tenant_id = s.firm_id
           where s.firm_id = $1
           order by s.client_id, m.created_at desc"#,
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    for row in last {
        by_client.entry(row.client_id.into()).or_default().last_message = Some(LastMessage {
            status: row.status,
            at: row.at,
        });
    }
    let waiting = sqlx::query!(
        r#"select s.client_id, count(*) as "drafts!"
           from scheduled_drafts s join outbox_messages m on m.id = s.message_id and m.tenant_id = s.firm_id
           where s.firm_id = $1 and m.status = 'draft'
           group by s.client_id"#,
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    for row in waiting {
        by_client.entry(row.client_id.into()).or_default().drafts_waiting = row.drafts;
    }
    let replies = sqlx::query!(
        "select client_id, period, automatic from replies where firm_id = $1",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    for row in replies {
        let activity = by_client.entry(row.client_id.into()).or_default();
        if row.automatic {
            activity.auto_replied.insert(row.period);
        } else {
            activity.replied.insert(row.period);
        }
    }
    let bounces = sqlx::query!(
        r#"select b.client_id,
                  bool_or(b.kind in ('bounce', 'complaint') and b.address = lower(trim(c.email))) as "bouncing!",
                  bool_or(b.kind = 'soft_bounce') as "soft!"
           from bounces b join clients c on c.id = b.client_id and c.firm_id = b.firm_id
           where b.firm_id = $1
           group by b.client_id"#,
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    for row in bounces {
        let activity = by_client.entry(row.client_id.into()).or_default();
        activity.bouncing = row.bouncing;
        activity.soft_bounced = row.soft;
    }
    Ok(by_client)
}
