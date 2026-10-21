//! A Firm's Clients (B1, B2): who gets asked for documents, in which
//! language, for which BIR forms.

mod repo;

use std::fmt;

pub use repo::{Client, ClientError, ClientId, Clients, ExtraItemId, ScheduleSettings};

use super::calendar::BirForm;

/// The language a Client's reminders are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// Tagalog-English mix, as most Filipino clients text.
    Taglish,
    /// English.
    English,
}

impl Language {
    /// The stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Taglish => "taglish",
            Self::English => "english",
        }
    }

    /// Reads a language name, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "taglish" => Some(Self::Taglish),
            "english" => Some(Self::English),
            _ => None,
        }
    }
}

/// A Client's details as typed into the form or a row of an import file.
/// Untrusted until [`NewClient::parse`] accepts it.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ClientInput {
    /// Business name.
    pub business_name: String,
    /// The Contact's first name.
    pub contact_first_name: String,
    /// The Contact's email.
    pub email: String,
    /// `Taglish` or `English`.
    pub language: String,
    /// Form numbers, e.g. `["2550Q", "0619-E"]`.
    pub forms: Vec<String>,
}

/// A Client's details, checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewClient {
    /// Business name, trimmed.
    pub business_name: String,
    /// The Contact's first name, trimmed.
    pub contact_first_name: String,
    /// The Contact's email, trimmed.
    pub email: String,
    /// Reminder language.
    pub language: Language,
    /// The forms the Client files, without repeats, in calendar order.
    pub forms: Vec<BirForm>,
}

/// Why a field was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    /// The field, as labelled on the form and in the import file.
    pub field: &'static str,
    /// What's wrong, in words a bookkeeper understands.
    pub reason: String,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.reason)
    }
}

impl NewClient {
    /// Checks every field, returning all problems at once.
    pub fn parse(input: &ClientInput) -> Result<Self, Vec<FieldError>> {
        let mut errors = Vec::new();
        let mut check = |field: &'static str, result: Result<(), String>| {
            if let Err(reason) = result {
                errors.push(FieldError { field, reason });
            }
        };
        let business_name = input.business_name.trim();
        check("Business name", check_name(business_name, 120));
        let contact_first_name = input.contact_first_name.trim();
        check("Contact first name", check_name(contact_first_name, 60));
        let email = input.email.trim();
        check("Email", check_email(email));
        let language = Language::parse(&input.language);
        check(
            "Language",
            language
                .map(|_| ())
                .ok_or_else(|| "must be Taglish or English".to_owned()),
        );
        let mut forms = Vec::new();
        for text in input.forms.iter().map(|f| f.trim()).filter(|f| !f.is_empty()) {
            match BirForm::parse(text) {
                Some(form) => forms.push(form),
                None => check("Forms", Err(format!("unknown form {text:?}"))),
            }
        }
        forms.sort_unstable();
        forms.dedup();
        if forms.is_empty() && input.forms.iter().all(|f| f.trim().is_empty()) {
            check("Forms", Err("tick at least one".to_owned()));
        }
        match language {
            Some(language) if errors.is_empty() => Ok(Self {
                business_name: business_name.to_owned(),
                contact_first_name: contact_first_name.to_owned(),
                email: email.to_owned(),
                language,
                forms,
            }),
            _ => Err(errors),
        }
    }
}

fn check_name(name: &str, max_chars: usize) -> Result<(), String> {
    if name.is_empty() {
        Err("is required".to_owned())
    } else if name.chars().count() > max_chars {
        Err(format!("is longer than {max_chars} characters"))
    } else if name.chars().any(char::is_control) {
        Err("has a control character".to_owned())
    } else {
        Ok(())
    }
}

/// A deliberately plain check: one `@`, something before it, a dotted
/// domain after it, no spaces. Delivery is the real test.
fn check_email(email: &str) -> Result<(), String> {
    let valid = email.len() <= 254
        && !email.chars().any(|c| c.is_whitespace() || c.is_control())
        && email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty()
                && !domain.contains('@')
                && domain.split('.').count() >= 2
                && domain.split('.').all(|part| !part.is_empty())
        });
    if email.is_empty() {
        Err("is required".to_owned())
    } else if valid {
        Ok(())
    } else {
        Err(format!("{email:?} is not an email address"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(business: &str, contact: &str, email: &str, language: &str, forms: &[&str]) -> ClientInput {
        ClientInput {
            business_name: business.to_owned(),
            contact_first_name: contact.to_owned(),
            email: email.to_owned(),
            language: language.to_owned(),
            forms: forms.iter().map(|&f| f.to_owned()).collect(),
        }
    }

    fn reasons(input: &ClientInput) -> Vec<String> {
        NewClient::parse(input)
            .unwrap_err()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    fn b1_a_client_has_business_contact_email_language_and_forms() {
        let client = NewClient::parse(&input(
            " Nena's Sari-Sari Store ",
            "Nena",
            "nena@example.test",
            "taglish",
            &["2551Q", "1701q", "2551Q"],
        ))
        .unwrap();

        assert_eq!(client.business_name, "Nena's Sari-Sari Store");
        assert_eq!(client.language, Language::Taglish);
        assert_eq!(client.forms, [BirForm::F2551Q, BirForm::F1701Q]);
    }

    #[test]
    fn b1_every_problem_is_reported_at_once() {
        assert_eq!(
            reasons(&input("", "Nena", "nena-at-example", "Bisaya", &["2551Q", "2550M"])),
            [
                "Business name: is required",
                "Email: \"nena-at-example\" is not an email address",
                "Language: must be Taglish or English",
                "Forms: unknown form \"2550M\"",
            ]
        );
        assert_eq!(
            reasons(&input("Store", "Nena", "nena@example.test", "English", &[])),
            ["Forms: tick at least one"]
        );
    }

    #[test]
    fn b1_names_refuse_control_characters_and_overlong_text() {
        let long = "x".repeat(121);
        assert_eq!(
            reasons(&input(&long, "Ne\u{7}na", "nena@example.test", "English", &["2551Q"])),
            [
                "Business name: is longer than 120 characters",
                "Contact first name: has a control character"
            ]
        );
    }
}
