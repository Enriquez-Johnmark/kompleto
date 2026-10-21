//! Writing a Draft's text (D5, D6). The model writes only the wording: a
//! subject, an opening and a closing. Kompleto inserts the facts itself
//! (the exact Owed documents, the Cut-off, the Upload link and the firm's
//! name), so the model can't garble or invent them. Without a model, or when its answer isn't
//! valid, the Draft is a fixed template and labelled as one.
//!
//! The request carries only what ADR 0003 allows, in [`DraftBrief`], and is
//! never sent if any of it looks like a TIN or a peso amount.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use time::Date;

use super::clients::Language;
use super::dates::show_date;
use crate::engine::clock::Clock;
use crate::engine::cost_log::{self, LlmCall, Outcome};
use crate::engine::ids::TenantId;
use crate::engine::llm::{LanguageModel, LlmError, LlmRequest, Usage};
use crate::engine::outbox::links;

/// How firm a message is (M3 round: friendlier first, firmer near the Cut-off).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tone {
    /// The first request.
    Friendly,
    /// A gentle reminder, a week out.
    FriendlyReminder,
    /// A few days out.
    Firmer,
    /// The day before the Cut-off.
    UrgentButPolite,
}

impl Tone {
    /// The tone for a message `days_before` the Cut-off; `None` is the first request.
    pub fn for_reminder(days_before: Option<u8>) -> Self {
        match days_before {
            None => Self::Friendly,
            Some(days) if days >= 7 => Self::FriendlyReminder,
            Some(days) if days >= 3 => Self::Firmer,
            Some(_) => Self::UrgentButPolite,
        }
    }
}

/// Everything a Draft is written from. Its fields are exactly those ADR 0003
/// lets the model see (plus the firm name, which only the template uses).
#[derive(Debug, Clone)]
pub struct DraftBrief {
    /// The Client's business name.
    pub business_name: String,
    /// The Contact's first name.
    pub contact_first_name: String,
    /// The Owed documents, as listed.
    pub owed: Vec<String>,
    /// What the documents are for, e.g. "Q3 2026".
    pub period: String,
    /// When they're wanted.
    pub cut_off: Date,
    /// The Client's reminder language.
    pub language: Language,
    /// How firm to be.
    pub tone: Tone,
    /// The Client's Upload link for the Period. Inserted by Kompleto; never
    /// sent to the model.
    pub upload_link: String,
}

/// Who wrote a Draft's wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftSource {
    /// The model.
    Ai,
    /// The fixed template.
    Template,
}

impl DraftSource {
    /// The stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ai => "ai",
            Self::Template => "template",
        }
    }
}

/// A Draft's subject and body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftText {
    /// Subject line.
    pub subject: String,
    /// Plain-text body.
    pub body: String,
    /// Who wrote the wording.
    pub source: DraftSource,
}

/// Writes Drafts, with the model when there is one.
pub struct Drafter<M> {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    model: Option<M>,
}

const SYSTEM: &str = "You write the wording of a short email from a Philippine bookkeeper to a small-business client \
who owes documents for a BIR filing. The email itself will list the documents and the deadline; you write only:
- subject: one line, at most 100 characters;
- opening: one to three sentences that greet the contact by first name and say documents are still needed for the period;
- closing: one short sentence of thanks.
Write in the requested language. Taglish means natural Tagalog-English as Filipino bookkeepers write to clients, \
with \"po\" where respectful. Match the requested tone; later reminders are firmer but stay polite.
Never include links, email addresses, amounts, account or tax numbers, or any document names.";

/// The shape the model must answer in.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Wording {
    subject: String,
    opening: String,
    closing: String,
}

impl<M: LanguageModel> Drafter<M> {
    /// A drafter; `model` is `None` when no API key is configured.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, model: Option<M>) -> Self {
        Self { pool, clock, model }
    }

    /// The Draft for `brief`, written by the model if it can be, else the template.
    pub async fn draft(&self, tenant: TenantId, firm_name: &str, brief: &DraftBrief) -> DraftText {
        if let Some(model) = &self.model {
            match self.ask(model, tenant, brief).await {
                Some(wording) => return assemble(firm_name, brief, &wording),
                None => tracing::info!(%tenant, "using the template for a draft"),
            }
        }
        template(firm_name, brief)
    }

    /// The model's wording, or `None` if it wasn't asked, failed or answered
    /// with anything invalid. Every call it makes is recorded (J1).
    async fn ask(&self, model: &M, tenant: TenantId, brief: &DraftBrief) -> Option<Wording> {
        let fields = [&brief.business_name, &brief.contact_first_name, &brief.period];
        if fields.into_iter().chain(&brief.owed).any(|text| looks_sensitive(text)) {
            tracing::warn!(%tenant, "a draft's details look like a TIN or an amount; not sent to the model");
            return None;
        }
        let user = json!({
            "business_name": brief.business_name,
            "contact_first_name": brief.contact_first_name,
            "owed_documents": brief.owed,
            "period": brief.period,
            "cut_off": show_date(brief.cut_off),
            "language": match brief.language { Language::Taglish => "Taglish", Language::English => "English" },
            "tone": brief.tone,
        });
        let request = LlmRequest {
            system: SYSTEM.to_owned(),
            user: user.to_string(),
            schema: json!({
                "type": "object",
                "properties": {"subject": {"type": "string"}, "opening": {"type": "string"}, "closing": {"type": "string"}},
                "required": ["subject", "opening", "closing"],
                "additionalProperties": false,
            }),
            max_tokens: 1_000,
        };
        let (wording, usage, model_name, request_id, outcome) = match model.complete(&request).await {
            Ok(reply) => {
                let usage = Usage {
                    input_tokens: reply.input_tokens,
                    output_tokens: reply.output_tokens,
                };
                let wording = serde_json::from_str::<Wording>(&reply.text).ok().filter(is_valid);
                let outcome = if wording.is_some() {
                    Outcome::Ok
                } else {
                    Outcome::Invalid
                };
                (wording, usage, reply.model, reply.request_id, outcome)
            }
            Err(err) => {
                tracing::warn!(%tenant, %err, "the model gave no draft");
                let (usage, request_id) = match err {
                    LlmError::Incomplete { usage, request_id, .. } => (usage, request_id),
                    LlmError::Api { request_id, .. } => (Usage::default(), request_id),
                    LlmError::Unavailable(_) | LlmError::Malformed(_) => (Usage::default(), None),
                };
                (None, usage, model.model().to_owned(), request_id, Outcome::Failed)
            }
        };
        let call = LlmCall {
            tenant,
            at: self.clock.now(),
            model: model_name,
            usage,
            request_id,
            outcome,
        };
        if let Err(err) = cost_log::record(&self.pool, &call).await {
            tracing::error!(%tenant, %err, "recording a model call failed");
        }
        wording
    }
}

/// D5: the model's wording is used only if every part is plain, short text.
fn is_valid(wording: &Wording) -> bool {
    let subject = wording.subject.trim();
    let plain = |text: &str| !looks_sensitive(text) && !has_link_or_address(text);
    !subject.is_empty()
        && subject.chars().count() <= 120
        && !subject.contains(['\n', '\r'])
        && [&wording.opening, &wording.closing]
            .iter()
            .all(|part| !part.trim().is_empty() && part.chars().count() <= 600)
        && [subject, &wording.opening, &wording.closing].into_iter().all(plain)
}

/// The same link rule the pre-send check applies (D7), plus any address.
fn has_link_or_address(text: &str) -> bool {
    !links(text).is_empty() || text.contains('@')
}

/// True if `text` has a peso amount or a run of nine or more digits (a TIN
/// is twelve, often written with dashes or spaces).
pub fn looks_sensitive(text: &str) -> bool {
    let lower = text.to_lowercase();
    let php_amount = lower
        .split("php")
        .skip(1)
        .any(|rest| rest.trim_start().starts_with(|c: char| c.is_ascii_digit()));
    if lower.contains('₱') || lower.contains("pesos") || php_amount {
        return true;
    }
    // Digits separated only by dashes, spaces, dots or commas count as one run.
    let mut digits_in_run = 0;
    let mut chars = lower.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            digits_in_run += 1;
            if digits_in_run >= 9 {
                return true;
            }
        } else if !(matches!(c, '-' | ' ' | '.' | ',') && chars.peek().is_some_and(char::is_ascii_digit)) {
            digits_in_run = 0;
        }
    }
    false
}

fn owed_list(brief: &DraftBrief) -> String {
    brief.owed.iter().map(|item| format!("- {item}\n")).collect()
}

fn upload_line(brief: &DraftBrief) -> String {
    match brief.language {
        Language::Taglish => format!("I-upload po dito: {}", brief.upload_link),
        Language::English => format!("Upload them here: {}", brief.upload_link),
    }
}

/// The model's wording around the facts Kompleto inserts.
fn assemble(firm_name: &str, brief: &DraftBrief, wording: &Wording) -> DraftText {
    let cut_off = show_date(brief.cut_off);
    let deadline = match brief.language {
        Language::Taglish => format!("Paki-send po bago {cut_off}."),
        Language::English => format!("Please send these by {cut_off}."),
    };
    DraftText {
        subject: wording.subject.trim().to_owned(),
        body: format!(
            "{}\n\n{}\n{}\n{deadline}\n\n{}\n{firm_name}\n",
            wording.opening.trim(),
            owed_list(brief),
            upload_line(brief),
            wording.closing.trim()
        ),
        source: DraftSource::Ai,
    }
}

/// The fixed template (D5 fallback).
pub fn template(firm_name: &str, brief: &DraftBrief) -> DraftText {
    let (name, label, cut_off, list, upload) = (
        &brief.contact_first_name,
        &brief.period,
        show_date(brief.cut_off),
        owed_list(brief),
        upload_line(brief),
    );
    let reminder = brief.tone != Tone::Friendly;
    let (subject, body) = match brief.language {
        Language::Taglish => (
            format!("Mga dokumento para sa {label}, bago {cut_off}"),
            format!(
                "Hi {name},\n\n{}Para sa {label} BIR filings ninyo, paki-send po ang mga ito bago {cut_off}:\n\n\
                 {list}\n{upload}\n\nSalamat po!\n{firm_name}\n",
                if reminder { "Paalala lang po. " } else { "" }
            ),
        ),
        Language::English => (
            format!("Documents for {label}, by {cut_off}"),
            format!(
                "Hi {name},\n\n{}For your {label} BIR filings, please send us these by {cut_off}:\n\n\
                 {list}\n{upload}\n\nThank you!\n{firm_name}\n",
                if reminder { "A quick reminder. " } else { "" }
            ),
        ),
    };
    DraftText {
        subject,
        body,
        source: DraftSource::Template,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6_tins_and_peso_amounts_are_recognised() {
        for sensitive in [
            "123-456-789-000",
            "TIN 123 456 789",
            "₱5,000",
            "PHP 12,000",
            "php5000",
            "5,000 pesos",
        ] {
            assert!(looks_sensitive(sensitive), "{sensitive}");
        }
        for plain in [
            "Q3 2026",
            "Wed 21 Oct 2026",
            "2307s received from customers",
            "1601-EQ",
            "Philippines",
            "Store 12345678",
        ] {
            assert!(!looks_sensitive(plain), "{plain}");
        }
    }
}
