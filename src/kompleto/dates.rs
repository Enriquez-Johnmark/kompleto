//! How dates are written for people: on pages and in messages.

use time::Date;
use time::macros::format_description;

/// A date as Kompleto writes it: "Mon 26 Oct 2026".
pub fn show_date(date: Date) -> String {
    let format = format_description!("[weekday repr:short] [day padding:none] [month repr:short] [year]");
    date.format(&format).unwrap_or_else(|_| date.to_string())
}
