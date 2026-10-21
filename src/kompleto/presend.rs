//! Kompleto's own pre-send rules (D7), run by the outbox at approval and
//! again just before sending, on top of the engine's own rules (length, at
//! most one link).
//!
//! - A reminder goes only to its Client's stored email, and carries no link
//!   but the Client's own upload link. Upload links arrive with M4; until
//!   then a reminder may carry no link at all.
//! - A login email goes only to its Firm's login address, with exactly one
//!   link, to our own login page.
//! - Anything else isn't recognised, and never goes out.

use sqlx::PgPool;

use super::clients::{ClientId, Clients};
use super::firms::FirmId;
use crate::engine::clock::Clock;
use crate::engine::ids::{MessageId, TenantId};
use crate::engine::mail::Email;
use crate::engine::outbox::{BoxFuture, CheckFailed, PreSendCheck, links};

/// Kompleto's pre-send rules.
#[derive(Clone)]
pub struct KompletoChecks {
    pool: PgPool,
    clients: Clients,
    login_link_prefix: String,
}

impl KompletoChecks {
    /// Rules for an app reached at `base_url`, whose login links start there.
    pub fn new(pool: PgPool, clock: std::sync::Arc<dyn Clock>, base_url: &str) -> Self {
        Self {
            clients: Clients::new(pool.clone(), clock),
            pool,
            login_link_prefix: format!("{}/login/", base_url.trim_end_matches('/')),
        }
    }

    async fn check_message(&self, firm: FirmId, id: MessageId, email: &Email) -> Result<(), CheckFailed> {
        let failed = |err: sqlx::Error| CheckFailed(format!("couldn't check the message: {err}"));
        let client = sqlx::query_scalar!(
            "select client_id from scheduled_drafts where firm_id = $1 and message_id = $2",
            firm.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(failed)?;
        if let Some(client) = client {
            let client = self
                .clients
                .get(firm, ClientId::from(client))
                .await
                .map_err(|err| CheckFailed(format!("couldn't check the message: {err}")))?;
            if !same_address(&email.to_address, &client.details.email) {
                return Err(CheckFailed("the recipient isn't the client's stored email".to_owned()));
            }
            if !links(&email.body_text).is_empty() {
                return Err(CheckFailed(
                    "it has a link that isn't the client's own upload link".to_owned(),
                ));
            }
            return Ok(());
        }
        let login_email = sqlx::query_scalar!("select login_email from firms where id = $1", firm.as_uuid())
            .fetch_optional(&self.pool)
            .await
            .map_err(failed)?;
        let body_links = links(&email.body_text);
        let is_login = login_email.is_some_and(|login| same_address(&email.to_address, &login))
            && body_links.len() == 1
            && body_links.iter().all(|link| link.starts_with(&self.login_link_prefix));
        if is_login {
            Ok(())
        } else {
            Err(CheckFailed(
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
