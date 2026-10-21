//! The quarterly 2307 tracker (H2): each Client's Payors, and whether
//! their 2307 for the quarter has been received, uploaded or is missing.
//! Scoped by the Firm (A2).

use std::collections::HashMap;

use sqlx::PgPool;

use super::calendar::TaxPeriod;
use super::checklist::is_received_2307;
use super::clients::ClientId;
use super::firms::FirmId;

/// Where one Payor's 2307 stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayorStatus {
    /// The Firm marked an Upload of it received.
    Received,
    /// Uploaded, waiting to be checked.
    Uploaded,
    /// Nothing yet.
    Missing,
}

/// One listed Payor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedPayor {
    /// The name, as the Firm listed it.
    pub name: String,
    /// Where its 2307 stands.
    pub status: PayorStatus,
}

/// One Client's quarter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedClient {
    /// The Client.
    pub id: ClientId,
    /// Its business name.
    pub business_name: String,
    /// Its listed Payors, by name.
    pub payors: Vec<TrackedPayor>,
    /// Payors of uploaded 2307s that aren't listed, as the Contact typed them.
    pub unexpected: Vec<String>,
}

/// Every Client of `firm`, with its 2307s for `quarter` (1 to 4) of `year`.
pub async fn quarter(pool: &PgPool, firm: FirmId, year: i32, quarter: u8) -> Result<Vec<TrackedClient>, sqlx::Error> {
    let (year, month) = TaxPeriod::Quarter { year, quarter }.last_month();
    // Uploads are kept under the Collection Period of the quarter's last month.
    let period = TaxPeriod::Month { year, month }.key();
    let mut clients: Vec<TrackedClient> = sqlx::query!(
        "select id, business_name from clients where firm_id = $1 order by lower(business_name), id",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| TrackedClient {
        id: row.id.into(),
        business_name: row.business_name,
        payors: Vec::new(),
        unexpected: Vec::new(),
    })
    .collect();
    let mut listed: HashMap<ClientId, Vec<String>> = HashMap::new();
    for row in sqlx::query!(
        "select client_id, name from payors where firm_id = $1 order by lower(name)",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    {
        listed.entry(row.client_id.into()).or_default().push(row.name);
    }
    // Each Client's uploaded 2307s: (Payor, received).
    let mut uploaded: HashMap<ClientId, Vec<(String, bool)>> = HashMap::new();
    for row in sqlx::query!(
        r#"select client_id, owed_document as "document!", payor as "payor!", received_at is not null as "received!"
           from uploads
           where firm_id = $1 and period = $2 and owed_document is not null and payor is not null
           order by uploaded_at, id"#,
        firm.as_uuid(),
        period,
    )
    .fetch_all(pool)
    .await?
    {
        if is_received_2307(&row.document) {
            uploaded
                .entry(row.client_id.into())
                .or_default()
                .push((row.payor, row.received));
        }
    }
    for client in &mut clients {
        let names = listed.remove(&client.id).unwrap_or_default();
        let uploads = uploaded.remove(&client.id).unwrap_or_default();
        let same = |a: &str, b: &str| a.trim().to_lowercase() == b.trim().to_lowercase();
        for (payor, _) in &uploads {
            let known = names.iter().chain(&client.unexpected).any(|name| same(name, payor));
            if !known {
                client.unexpected.push(payor.trim().to_owned());
            }
        }
        client.payors = names
            .into_iter()
            .map(|name| {
                let theirs = uploads.iter().filter(|(payor, _)| same(payor, &name));
                let status = theirs.fold(PayorStatus::Missing, |status, (_, received)| match (status, received) {
                    (PayorStatus::Received, _) | (_, true) => PayorStatus::Received,
                    _ => PayorStatus::Uploaded,
                });
                TrackedPayor { name, status }
            })
            .collect();
    }
    Ok(clients)
}
