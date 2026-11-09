//! Requests for an account from the public sign-up page (M10, L1). A request
//! creates nothing and sends nothing: the operator approves it from the
//! command line, which creates the Firm. The answer to a request is the same
//! whether or not the address already has an account.
//!
//! The requests are the engine's sign-up requests (`engine::accounts`); this
//! module words their problems for a bookkeeper. Requests belong to no Firm,
//! so they aren't scoped by one; only the operator's command reads them.

use std::sync::Arc;

use sqlx::PgPool;
use time::OffsetDateTime;

use super::firms::{FirmError, FirmId};
use crate::engine::accounts;
use crate::engine::clock::Clock;

pub use crate::engine::accounts::{EXPIRES_AFTER_DAYS, MAX_PENDING, MAX_PER_HOUR, RequestId};

/// One request waiting for the operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRequest {
    /// Its ID.
    pub id: RequestId,
    /// The firm's name, as its Clients will see it.
    pub firm_name: String,
    /// The owner's address.
    pub email: String,
    /// When it was made.
    pub requested_at: OffsetDateTime,
}

impl From<accounts::AccountRequest> for AccountRequest {
    fn from(request: accounts::AccountRequest) -> Self {
        Self {
            id: request.id,
            firm_name: request.name,
            email: request.email,
            requested_at: request.requested_at,
        }
    }
}

/// Sign-up errors.
#[derive(Debug, thiserror::Error)]
pub enum SignupError {
    /// What's wrong with the request, one problem each.
    #[error("{}", .0.join("; "))]
    Invalid(Vec<String>),
    /// Too many requests are waiting.
    #[error("too many requests are waiting")]
    TooMany,
    /// No such request.
    #[error("request not found")]
    NotFound,
    /// Creating the Firm failed.
    #[error(transparent)]
    Firm(#[from] FirmError),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl From<accounts::SignupError> for SignupError {
    fn from(err: accounts::SignupError) -> Self {
        match err {
            accounts::SignupError::Invalid(problems) => Self::Invalid(problems.into_iter().map(reworded).collect()),
            accounts::SignupError::TooMany => Self::TooMany,
            accounts::SignupError::NotFound => Self::NotFound,
            accounts::SignupError::Account(err) => Self::Firm(err.into()),
            accounts::SignupError::Database(err) => Self::Database(err),
        }
    }
}

/// A problem as a bookkeeper reads it: the account is their firm, and the
/// terms they accept include the data processing agreement.
fn reworded(problem: String) -> String {
    if let Some(rest) = problem.strip_prefix("Name: ") {
        format!("Firm name: {rest}")
    } else if problem == "Accept the terms" {
        "Accept the terms and the data processing agreement".to_owned()
    } else {
        problem
    }
}

/// Account requests.
#[derive(Clone)]
pub struct Signups {
    inner: accounts::Signups,
}

impl Signups {
    /// Requests in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            inner: accounts::Signups::new(pool, clock),
        }
    }

    /// Records a request for an account named `firm_name`, owned by `email`.
    /// An address that already logs in gets the same `Ok` and nothing is
    /// recorded.
    pub async fn request(&self, firm_name: &str, email: &str, accepted_terms: bool) -> Result<(), SignupError> {
        Ok(self.inner.request(firm_name, email, accepted_terms).await?)
    }

    /// The requests waiting, oldest first.
    pub async fn pending(&self) -> Result<Vec<AccountRequest>, SignupError> {
        Ok(self
            .inner
            .pending()
            .await?
            .into_iter()
            .map(AccountRequest::from)
            .collect())
    }

    /// Creates the Firm a request asks for and removes the request.
    pub async fn approve(&self, id: RequestId) -> Result<(FirmId, AccountRequest), SignupError> {
        let (firm, request) = self.inner.approve(id).await?;
        Ok((firm, request.into()))
    }

    /// Removes a request without creating anything.
    pub async fn decline(&self, id: RequestId) -> Result<(), SignupError> {
        Ok(self.inner.decline(id).await?)
    }

    /// Removes every waiting request; returns how many.
    pub async fn decline_all(&self) -> Result<u64, SignupError> {
        Ok(self.inner.decline_all().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l1_problems_are_worded_for_a_bookkeeper() {
        assert_eq!(
            reworded("Name: a name is 1 to 100 characters".to_owned()),
            "Firm name: a name is 1 to 100 characters"
        );
        assert_eq!(
            reworded("Accept the terms".to_owned()),
            "Accept the terms and the data processing agreement"
        );
        assert_eq!(reworded("Email: not an address".to_owned()), "Email: not an address");
    }
}
