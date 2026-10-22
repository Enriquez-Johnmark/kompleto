//! Start-up configuration, read once from environment variables. A missing
//! or invalid value stops the program at start-up, not mid-request.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use secrecy::SecretString;

use crate::engine::files::MasterKey;
use crate::engine::llm::AnthropicClient;
use crate::engine::mail::{ConfiguredMailer, MailpitMailer, ResendMailer};
use crate::engine::outbox::SendBudget;

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
    /// The URL people reach the app at, used in emailed links and to check
    /// that form posts come from our own pages.
    pub public_base_url: String,
    /// The email provider.
    pub mail: MailConfig,
    /// Address emails are sent from, on our own domain.
    pub mail_from_address: String,
    /// The model for drafting; `None` without `ANTHROPIC_API_KEY`, and every
    /// Draft is then a template.
    pub model: Option<ModelConfig>,
    /// The key that protects every stored file (`FILE_MASTER_KEY`). Only
    /// the server needs it; it refuses to start without it.
    pub file_master_key: Option<SecretString>,
    /// Where encrypted Uploads are kept (`UPLOAD_DIR`, default `uploads`).
    pub upload_dir: PathBuf,
    /// Proves inbound email comes from our provider (`INBOUND_SECRET`);
    /// without it, no Replies are taken in.
    pub inbound_secret: Option<SecretString>,
    /// Verifies Resend's delivery events (`RESEND_WEBHOOK_SECRET`,
    /// `whsec_…`); without it, Bounces aren't taken in.
    pub resend_webhook_secret: Option<SecretString>,
    /// Where Firms write for help (`SUPPORT_EMAIL`), shown when an account
    /// closes. Route it to a person, not to the reply Worker.
    pub support_email: Option<String>,
}

/// Where and which model drafts.
#[derive(Debug)]
pub struct ModelConfig {
    /// Normally `https://api.anthropic.com`.
    pub base_url: String,
    /// API key.
    pub api_key: SecretString,
    /// Model ID, e.g. `claude-haiku-5-5`.
    pub model: String,
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
        let public_base_url = public_base_url(optional("PUBLIC_BASE_URL").as_deref(), bind_addr)?;
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            bind_addr,
            public_base_url,
            mail,
            mail_from_address: required("MAIL_FROM_ADDRESS")?,
            model: optional("ANTHROPIC_API_KEY").map(|key| ModelConfig {
                base_url: optional("ANTHROPIC_BASE_URL").unwrap_or_else(|| "https://api.anthropic.com".to_owned()),
                api_key: SecretString::from(key),
                model: optional("LLM_MODEL").unwrap_or_else(|| "claude-haiku-5-5".to_owned()),
            }),
            file_master_key: optional("FILE_MASTER_KEY").map(SecretString::from),
            upload_dir: optional("UPLOAD_DIR").map_or_else(|| PathBuf::from("uploads"), PathBuf::from),
            inbound_secret: optional("INBOUND_SECRET").map(SecretString::from),
            resend_webhook_secret: optional("RESEND_WEBHOOK_SECRET").map(SecretString::from),
            support_email: optional("SUPPORT_EMAIL"),
        })
    }

    /// Builds the drafting model's client, if one is configured. It gives up
    /// after 15 seconds; the Draft is then a template.
    pub fn language_model(&self) -> Result<Option<AnthropicClient>, ConfigError> {
        self.model
            .as_ref()
            .map(|model| {
                AnthropicClient::new(
                    &model.base_url,
                    model.api_key.clone(),
                    &model.model,
                    Duration::from_secs(15),
                )
                .map_err(|err| ConfigError::Invalid {
                    name: "ANTHROPIC_BASE_URL",
                    reason: err.to_string(),
                })
            })
            .transpose()
    }

    /// The domain Replies come back to: the sending address's own.
    pub fn reply_domain(&self) -> &str {
        self.mail_from_address
            .rsplit_once('@')
            .map_or(self.mail_from_address.as_str(), |(_, domain)| domain)
    }

    /// The file master key; an error if it's missing or malformed.
    pub fn master_key(&self) -> Result<MasterKey, ConfigError> {
        use secrecy::ExposeSecret;
        let key = self
            .file_master_key
            .as_ref()
            .ok_or(ConfigError::Missing("FILE_MASTER_KEY"))?;
        MasterKey::from_hex(key.expose_secret()).map_err(|err| ConfigError::Invalid {
            name: "FILE_MASTER_KEY",
            reason: err.to_string(),
        })
    }

    /// The daily send budget for the configured provider: Resend Free's
    /// cap, or none for Mailpit, which delivers nowhere.
    pub fn send_budget(&self) -> Option<SendBudget> {
        match self.mail {
            MailConfig::Mailpit { .. } => None,
            MailConfig::Resend { .. } => Some(SendBudget::RESEND_FREE),
        }
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

/// The URL people open, as `scheme://host[:port]` in lowercase without a
/// default port, which is exactly the `Origin` browsers send with form
/// posts. Without `PUBLIC_BASE_URL` it's the bind address, unless that is
/// "all interfaces", which no browser can open.
fn public_base_url(setting: Option<&str>, bind_addr: SocketAddr) -> Result<String, ConfigError> {
    let invalid = |reason: &str| ConfigError::Invalid {
        name: "PUBLIC_BASE_URL",
        reason: reason.to_owned(),
    };
    let Some(setting) = setting else {
        if bind_addr.ip().is_unspecified() {
            return Err(ConfigError::Missing("PUBLIC_BASE_URL"));
        }
        return Ok(format!("http://{bind_addr}"));
    };
    let lower = setting.trim().trim_end_matches('/').to_ascii_lowercase();
    let (scheme, host) = lower
        .split_once("://")
        .filter(|(scheme, _)| matches!(*scheme, "http" | "https"))
        .ok_or_else(|| invalid("must start with http:// or https://"))?;
    if host.is_empty() || host.contains('/') {
        return Err(invalid("must be just scheme://host[:port], with no path"));
    }
    let default_port = if scheme == "https" { ":443" } else { ":80" };
    Ok(format!(
        "{scheme}://{}",
        host.strip_suffix(default_port).unwrap_or(host)
    ))
}

fn optional(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    optional(name).ok_or(ConfigError::Missing(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(text: &str) -> SocketAddr {
        text.parse().unwrap()
    }

    #[test]
    fn the_base_url_is_normalised_so_browser_origins_match_it() {
        let local = addr("127.0.0.1:3000");
        assert_eq!(
            public_base_url(Some("HTTPS://App.Getkompleto.com:443/"), local).unwrap(),
            "https://app.getkompleto.com"
        );
        assert_eq!(
            public_base_url(Some("http://localhost:80"), local).unwrap(),
            "http://localhost"
        );
        assert_eq!(
            public_base_url(Some("http://127.0.0.1:3000"), local).unwrap(),
            "http://127.0.0.1:3000"
        );
        assert!(public_base_url(Some("app.getkompleto.com"), local).is_err());
        assert!(public_base_url(Some("https://app.getkompleto.com/kompleto"), local).is_err());
    }

    #[test]
    fn the_base_url_defaults_to_the_bind_address_only_when_browsers_can_use_it() {
        assert_eq!(
            public_base_url(None, addr("127.0.0.1:3000")).unwrap(),
            "http://127.0.0.1:3000"
        );
        assert!(matches!(
            public_base_url(None, addr("0.0.0.0:3000")),
            Err(ConfigError::Missing("PUBLIC_BASE_URL"))
        ));
    }
}
