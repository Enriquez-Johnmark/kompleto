//! Start-up configuration, read once from environment variables. A missing
//! or invalid value stops the program at start-up, not mid-request.

use std::net::SocketAddr;

use secrecy::SecretString;

use crate::engine::mail::{ConfiguredMailer, MailpitMailer, ResendMailer};

/// Configuration errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// A required variable is missing.
    #[error("environment variable {0} is required")]
    Missing(&'static str),
    /// A variable has an unusable value.
    #[error("environment variable {name} is invalid: {reason}")]
    Invalid {
        /// The variable.
        name: &'static str,
        /// What's wrong with it.
        reason: String,
    },
}

/// Which email provider to use.
#[derive(Debug)]
pub enum MailConfig {
    /// Local Mailpit at this base URL.
    Mailpit {
        /// e.g. `http://localhost:8025`.
        base_url: String,
    },
    /// Resend.
    Resend {
        /// Normally `https://api.resend.com`.
        base_url: String,
        /// API key.
        api_key: SecretString,
    },
}

/// Everything the program needs to start.
#[derive(Debug)]
pub struct Config {
    /// Postgres connection string.
    pub database_url: String,
    /// Address the web server listens on.
    pub bind_addr: SocketAddr,
    /// The email provider.
    pub mail: MailConfig,
    /// Address emails are sent from, on our own domain.
    pub mail_from_address: String,
}

impl Config {
    /// Reads the configuration from the environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        let mail = match optional("MAIL_PROVIDER").as_deref().unwrap_or("mailpit") {
            "mailpit" => MailConfig::Mailpit {
                base_url: optional("MAILPIT_URL").unwrap_or_else(|| "http://localhost:8025".to_owned()),
            },
            "resend" => MailConfig::Resend {
                base_url: optional("RESEND_BASE_URL").unwrap_or_else(|| "https://api.resend.com".to_owned()),
                api_key: SecretString::from(required("RESEND_API_KEY")?),
            },
            other => {
                return Err(ConfigError::Invalid {
                    name: "MAIL_PROVIDER",
                    reason: format!("{other:?} is not one of: mailpit, resend"),
                });
            }
        };
        let bind_addr = optional("BIND_ADDR")
            .unwrap_or_else(|| "127.0.0.1:3000".to_owned())
            .parse()
            .map_err(|err: std::net::AddrParseError| ConfigError::Invalid {
                name: "BIND_ADDR",
                reason: err.to_string(),
            })?;
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            bind_addr,
            mail,
            mail_from_address: required("MAIL_FROM_ADDRESS")?,
        })
    }

    /// Builds the configured mailer.
    pub fn mailer(&self) -> Result<ConfiguredMailer, ConfigError> {
        let invalid = |err: reqwest::Error| ConfigError::Invalid {
            name: "MAIL_PROVIDER",
            reason: err.to_string(),
        };
        Ok(match &self.mail {
            MailConfig::Mailpit { base_url } => {
                ConfiguredMailer::Mailpit(MailpitMailer::new(base_url, &self.mail_from_address).map_err(invalid)?)
            }
            MailConfig::Resend { base_url, api_key } => ConfiguredMailer::Resend(
                ResendMailer::new(base_url, api_key.clone(), &self.mail_from_address).map_err(invalid)?,
            ),
        })
    }
}

fn optional(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    optional(name).ok_or(ConfigError::Missing(name))
}
