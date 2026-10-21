//! Kompleto's own pre-send rules (D7), run by the outbox at approval and
//! again just before sending, on top of the engine's own rules (length, at
//! most one link).
//!
//! - A reminder goes only to its Client's stored email, and carries exactly
//!   one link: the Client's own Upload link for the reminder's Period.
//! - A login email or a Reply notice goes only to its Firm's login address,
//!   with exactly one link, to our own login page or Reply page.
//! - Anything else isn't recognised, and never goes out.

use sqlx::PgPool;

use super::clients::{ClientId, Clients};
use super::firms::FirmId;
use super::upload_links::UploadLinks;
use crate::engine::clock::Clock;
use crate::engine::ids::{MessageId, TenantId};
use crate::engine::mail::Email;
use crate::engine::outbox::{BoxFuture, CheckFailed, PreSendCheck, links};

/// Kompleto's pre-send rules.
#[derive(Clone)]
pub struct KompletoChecks {
    pool: PgPool,
    clients: Clients,
    links: UploadLinks,
    /// Pages a message to the Firm's login address may link to.
    firm_link_prefixes: [String; 2],
}

impl KompletoChecks {
    /// Rules for an app reached at `base_url`, whose login links start
    /// there, and whose Upload links come from `links`.
    pub fn new(pool: PgPool, clock: std::sync::Arc<dyn Clock>, base_url: &str, links: UploadLinks) -> Self {
        Self {
            links,
            clients: Clients::new(pool.clone(), clock),
            pool,
            firm_link_prefixes: ["login", "replies"].map(|page| format!("{}/{page}/", base_url.trim_end_matches('/'))),
        }
    }

    async fn check_message(&self, firm: FirmId, id: MessageId, email: &Email) -> Result<(), CheckFailed> {
        let failed = |err: sqlx::Error| CheckFailed::Unavailable(err.to_string());
        let reminder = sqlx::query!(
            "select client_id, period from scheduled_drafts where firm_id = $1 and message_id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(failed)?;
        if let Some(reminder) = reminder {
            let client = self
                .clients
                .get(firm, ClientId::from(reminder.client_id))
                .await
                .map_err(|err| CheckFailed::Unavailable(err.to_string()))?;
            if !same_address(&email.to_address, &client.details.email) {
                return Err(CheckFailed::Refused(
                    "the recipient isn't the client's stored email".to_owned(),
                ));
            }
            let own = self
                .links
                .existing(firm, client.id, &reminder.period)
                .await
                .map_err(failed)?;
            return match (links(&email.body_text).as_slice(), own) {
                ([link], Some(own)) if *link == own => Ok(()),
                ([], _) => Err(CheckFailed::Refused(
                    "it doesn't have the client's upload link".to_owned(),
                )),
                _ => Err(CheckFailed::Refused(
                    "it has a link that isn't the client's own upload link".to_owned(),
                )),
            };
        }
        let login_email = sqlx::query_scalar!("select login_email from firms where id = $1", firm.as_uuid())
            .fetch_optional(&self.pool)
            .await
            .map_err(failed)?;
        let body_links = links(&email.body_text);
        let is_login = login_email.is_some_and(|login| same_address(&email.to_address, &login))
            && body_links.len() == 1
            && body_links.iter().all(|link| {
                self.firm_link_prefixes
                    .iter()
                    .any(|prefix| link.starts_with(prefix.as_str()))
            });
        if is_login {
            Ok(())
        } else {
            Err(CheckFailed::Refused(
                "Kompleto doesn't recognise this message, so it won't send it".to_owned(),
            ))
        }
    }
}

fn same_address(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

impl PreSendCheck for KompletoChecks {
    fn check<'a>(
        &'a self,
        tenant: TenantId,
        id: MessageId,
        email: &'a Email,
    ) -> BoxFuture<'a, Result<(), CheckFailed>> {
        Box::pin(self.check_message(tenant, id, email))
    }
}
