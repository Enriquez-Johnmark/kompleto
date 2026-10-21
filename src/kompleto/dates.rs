//! How dates are written for people: on pages and in messages.

use time::Date;
use time::macros::format_description;

/// A date as Kompleto writes it: "Mon 26 Oct 2026".
pub fn show_date(date: Date) -> String {
    let format = format_description!("[weekday repr:short] [day padding:none] [month repr:short] [year]");
    date.format(&format).unwrap_or_else(|_| date.to_string())
}

/// The start of `month` and of the month after, in Manila time: the range
/// for monthly totals.
pub fn manila_month(year: i32, month: time::Month) -> (time::OffsetDateTime, time::OffsetDateTime) {
    let start = |year, month| {
        Date::from_calendar_date(year, month, 1)
            .unwrap_or(Date::MIN)
            .midnight()
            .assume_offset(super::schedule::MANILA)
    };
    let (next_year, next_month) = if month == time::Month::December {
        (year + 1, time::Month::January)
    } else {
        (year, month.next())
    };
    (start(year, month), start(next_year, next_month))
}
