//! The built-in BIR calendar (B3): which Periods each BIR form is filed for
//! and when each is due. The rules live in one data file,
//! `data/bir-calendar.toml`, so a change to them is one edit.

use std::collections::{BTreeSet, HashMap};
use std::fmt;

use serde::Deserialize;
use time::macros::format_description;
use time::{Date, Duration, Month, Weekday};

/// The calendar's rules, as written in `data/bir-calendar.toml`.
const BUILT_IN: &str = include_str!("../../data/bir-calendar.toml");

/// A BIR form in the built-in calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BirForm {
    /// Monthly remittance of expanded withholding tax.
    F0619E,
    /// Monthly withholding tax on compensation.
    F1601C,
    /// Quarterly expanded withholding tax return.
    F1601EQ,
    /// Quarterly VAT return.
    F2550Q,
    /// Quarterly percentage tax return.
    F2551Q,
    /// Quarterly income tax return, individuals.
    F1701Q,
    /// Quarterly income tax return, corporations.
    F1702Q,
    /// Annual income tax return, individuals (1701 and 1701A).
    F1701,
    /// Annual income tax return, corporations (1702-RT/EX/MX).
    F1702,
    /// Annual information return, compensation.
    F1604C,
    /// Annual information return, expanded withholding.
    F1604E,
}

impl BirForm {
    /// Every form, in display order.
    pub const ALL: [Self; 11] = [
        Self::F0619E,
        Self::F1601C,
        Self::F1601EQ,
        Self::F2550Q,
        Self::F2551Q,
        Self::F1701Q,
        Self::F1702Q,
        Self::F1701,
        Self::F1702,
        Self::F1604C,
        Self::F1604E,
    ];

    /// The form's BIR number, e.g. `"1601-EQ"`.
    pub fn code(self) -> &'static str {
        match self {
            Self::F0619E => "0619-E",
            Self::F1601C => "1601-C",
            Self::F1601EQ => "1601-EQ",
            Self::F2550Q => "2550Q",
            Self::F2551Q => "2551Q",
            Self::F1701Q => "1701Q",
            Self::F1702Q => "1702Q",
            Self::F1701 => "1701",
            Self::F1702 => "1702",
            Self::F1604C => "1604-C",
            Self::F1604E => "1604-E",
        }
    }

    /// Reads a form number as people write it: `"1601EQ"`, `"1601-eq"`,
    /// `"1701A"` (filed in place of 1701) or `"1702-RT"`.
    pub fn parse(text: &str) -> Option<Self> {
        let wanted: String = text
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_uppercase())
            .collect();
        let wanted = match wanted.as_str() {
            "1701A" => "1701",
            "1702RT" | "1702EX" | "1702MX" => "1702",
            other => other,
        };
        Self::ALL
            .into_iter()
            .find(|form| form.code().replace('-', "") == wanted)
    }
}

impl fmt::Display for BirForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.code())
    }
}

/// The month, quarter or year a BIR form covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TaxPeriod {
    /// One calendar month.
    Month {
        /// The year.
        year: i32,
        /// The month.
        month: Month,
    },
    /// One calendar quarter, 1 to 4.
    Quarter {
        /// The year.
        year: i32,
        /// 1 to 4.
        quarter: u8,
    },
    /// One calendar year.
    Year(i32),
}

impl TaxPeriod {
    /// The year and month the Period ends in.
    pub fn last_month(self) -> (i32, Month) {
        match self {
            Self::Month { year, month } => (year, month),
            Self::Quarter { year, quarter } => (year, month_number(quarter.saturating_mul(3))),
            Self::Year(year) => (year, Month::December),
        }
    }

    /// The Period's last day.
    pub fn last_day(self) -> Date {
        let (year, month) = self.last_month();
        day_in_month(year, month, 31)
    }

    /// A stable text key for storage: `2026-11`, `2026-Q3` or `2026`.
    pub fn key(self) -> String {
        match self {
            Self::Month { year, month } => format!("{year}-{:02}", u8::from(month)),
            Self::Quarter { year, quarter } => format!("{year}-Q{quarter}"),
            Self::Year(year) => year.to_string(),
        }
    }

    /// Reads a key written by [`TaxPeriod::key`].
    pub fn parse_key(key: &str) -> Option<Self> {
        let Some((year, rest)) = key.split_once('-') else {
            return key.parse().ok().map(Self::Year);
        };
        let year = year.parse().ok()?;
        if let Some(quarter) = rest.strip_prefix('Q') {
            let quarter = quarter.parse().ok().filter(|q| (1..=4).contains(q))?;
            return Some(Self::Quarter { year, quarter });
        }
        let month = Month::try_from(rest.parse::<u8>().ok()?).ok()?;
        Some(Self::Month { year, month })
    }
}

impl fmt::Display for TaxPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Month { year, month } => format!("{month} {year}"),
            Self::Quarter { year, quarter } => format!("Q{quarter} {year}"),
            Self::Year(year) => year.to_string(),
        };
        f.pad(&text)
    }
}

/// When one form is due for one Period.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueDate {
    /// The date the law sets.
    pub statutory: Date,
    /// The date filing is actually due: the statutory date, moved past
    /// weekends and holidays, or as a BIR extension sets it.
    pub date: Date,
    /// The issuance behind an extension, e.g. `"RMC 30-2026"`.
    pub extension: Option<String>,
}

/// The built-in calendar is broken; a test keeps this from shipping.
#[derive(Debug, thiserror::Error)]
pub enum CalendarError {
    /// The file isn't valid TOML or doesn't match the expected shape.
    #[error("bir-calendar.toml: {0}")]
    Parse(#[from] toml::de::Error),
    /// The file and the code disagree.
    #[error("bir-calendar.toml: {0}")]
    Invalid(String),
}

/// The BIR calendar: every form's rule and the national holidays.
#[derive(Debug)]
pub struct Calendar {
    rules: HashMap<BirForm, FormRule>,
    holidays: BTreeSet<Date>,
    holidays_known_from: Date,
    holidays_known_until: Date,
    extensions: HashMap<(BirForm, TaxPeriod), (Date, String)>,
}

impl Calendar {
    /// The calendar built into the program.
    pub fn built_in() -> Result<Self, CalendarError> {
        Self::from_toml(BUILT_IN)
    }

    /// Reads a calendar in the format of `data/bir-calendar.toml`.
    pub fn from_toml(text: &str) -> Result<Self, CalendarError> {
        let file: CalendarFile = toml::from_str(text)?;
        let mut rules = HashMap::new();
        for rule in file.form {
            let form = BirForm::parse(&rule.code)
                .filter(|form| form.code() == rule.code)
                .ok_or_else(|| CalendarError::Invalid(format!("unknown form {:?}", rule.code)))?;
            if rules.insert(form, rule).is_some() {
                return Err(CalendarError::Invalid(format!("{form} has two rules")));
            }
        }
        if let Some(missing) = BirForm::ALL.into_iter().find(|form| !rules.contains_key(form)) {
            return Err(CalendarError::Invalid(format!("{missing} has no rule")));
        }
        let holidays = file
            .holidays
            .dates
            .iter()
            .map(|holiday| parse_date(&holiday.date))
            .collect::<Result<_, _>>()?;
        let mut extensions = HashMap::new();
        for extension in file.extension {
            let period = TaxPeriod::parse_key(&extension.period)
                .ok_or_else(|| CalendarError::Invalid(format!("bad period {:?}", extension.period)))?;
            let due = parse_date(&extension.due)?;
            for code in &extension.forms {
                let form = BirForm::parse(code)
                    .ok_or_else(|| CalendarError::Invalid(format!("unknown form {code:?} in an extension")))?;
                extensions.insert((form, period), (due, extension.source.clone()));
            }
        }
        Ok(Self {
            rules,
            extensions,
            holidays,
            holidays_known_from: parse_date(&file.holidays.known_from)?,
            holidays_known_until: parse_date(&file.holidays.known_until)?,
        })
    }

    /// Every due date from `from` to `until`, inclusive, by date then form.
    pub fn due_dates_between(&self, from: Date, until: Date) -> Vec<(BirForm, TaxPeriod, DueDate)> {
        let mut periods = Vec::new();
        // A Period is due at most four months after it ends.
        for year in from.year() - 1..=until.year() {
            periods.push(TaxPeriod::Year(year));
            periods.extend((1..=4).map(|quarter| TaxPeriod::Quarter { year, quarter }));
            periods.extend((1..=12).map(|n| TaxPeriod::Month {
                year,
                month: month_number(n),
            }));
        }
        let mut due: Vec<_> = periods
            .into_iter()
            .flat_map(|period| BirForm::ALL.map(|form| (form, period)))
            .filter_map(|(form, period)| self.due_date(form, period).map(|due| (form, period, due)))
            .filter(|(_, _, due)| (from..=until).contains(&due.date))
            .collect();
        due.sort_by_key(|(form, period, due)| (due.date, *form, *period));
        due
    }

    /// True if the national holidays are known for every day from `from`
    /// to `until`, so due dates in that range move correctly.
    pub fn holidays_cover(&self, from: Date, until: Date) -> bool {
        self.holidays_known_from <= from && until <= self.holidays_known_until
    }

    /// `date`, or the next working day if it's a weekend or national holiday.
    pub fn next_working_day(&self, mut date: Date) -> Date {
        while matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday) || self.holidays.contains(&date) {
            date = date.next_day().unwrap_or(date);
        }
        date
    }

    /// The form's name, e.g. "Quarterly VAT return".
    pub fn form_name(&self, form: BirForm) -> &str {
        self.rules.get(&form).map_or("", |rule| rule.name.as_str())
    }

    /// When `form` is due for `period`, or `None` if the form isn't filed
    /// for that kind of Period.
    pub fn due_date(&self, form: BirForm, period: TaxPeriod) -> Option<DueDate> {
        let rule = self.rules.get(&form)?;
        if !rule.covers.includes(period) {
            return None;
        }
        let due = match (period.last_month().1, rule.december_due) {
            (Month::December, Some(december)) => december,
            _ => rule.due,
        };
        let statutory = due.after(period.last_day());
        let (date, extension) = match self.extensions.get(&(form, period)) {
            Some((extended, source)) => (*extended, Some(source.clone())),
            None => (statutory, None),
        };
        Some(DueDate {
            statutory,
            date: self.next_working_day(date),
            extension,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CalendarFile {
    form: Vec<FormRule>,
    holidays: Holidays,
    #[serde(default)]
    extension: Vec<Extension>,
}

/// A nationwide BIR extension: new due date for some forms for one Period.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Extension {
    forms: Vec<String>,
    period: String,
    due: String,
    source: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Holidays {
    known_from: String,
    known_until: String,
    dates: Vec<Holiday>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Holiday {
    date: String,
    #[expect(dead_code, reason = "documents the holiday in the data file")]
    name: String,
}

/// Reads a `YYYY-MM-DD` date from the data file.
fn parse_date(text: &str) -> Result<Date, CalendarError> {
    Date::parse(text, format_description!("[year]-[month]-[day]"))
        .map_err(|err| CalendarError::Invalid(format!("bad date {text:?}: {err}")))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FormRule {
    code: String,
    name: String,
    covers: Covers,
    due: DueRule,
    december_due: Option<DueRule>,
    #[expect(dead_code, reason = "documents the legal basis in the data file")]
    source: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Covers {
    Monthly,
    MonthlyExceptQuarterEnd,
    Quarterly,
    QuarterlyExceptQ4,
    Annual,
}

impl Covers {
    fn includes(self, period: TaxPeriod) -> bool {
        match (self, period) {
            (Self::Monthly, TaxPeriod::Month { .. }) | (Self::Quarterly, TaxPeriod::Quarter { .. }) => true,
            (Self::MonthlyExceptQuarterEnd, TaxPeriod::Month { month, .. }) => u8::from(month) % 3 != 0,
            (Self::QuarterlyExceptQ4, TaxPeriod::Quarter { quarter, .. }) => quarter < 4,
            (Self::Annual, TaxPeriod::Year(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum DueRule {
    DaysAfter { days_after: u16 },
    DayOfMonthAfter { months_after: u8, day: u8 },
}

impl DueRule {
    /// The statutory due date for a Period ending on `period_end`.
    fn after(self, period_end: Date) -> Date {
        match self {
            Self::DaysAfter { days_after } => period_end + Duration::days(i64::from(days_after)),
            Self::DayOfMonthAfter { months_after, day } => {
                let months = i32::from(u8::from(period_end.month())) - 1 + i32::from(months_after);
                let year = period_end.year() + months / 12;
                day_in_month(year, month_number(u8::try_from(months % 12 + 1).unwrap_or(1)), day)
            }
        }
    }
}

/// The month numbered `n` (1 to 12); out-of-range numbers give January,
/// which callers never pass.
fn month_number(n: u8) -> Month {
    Month::try_from(n).unwrap_or(Month::January)
}

/// Day `day` of a month, or the month's last day if `day` is past it.
pub(crate) fn day_in_month(year: i32, month: Month, day: u8) -> Date {
    let day = day.clamp(1, month.length(year));
    Date::from_calendar_date(year, month, day).unwrap_or(Date::MIN)
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;

    fn calendar() -> Calendar {
        Calendar::built_in().unwrap()
    }

    fn statutory(form: BirForm, period: TaxPeriod) -> Option<Date> {
        calendar().due_date(form, period).map(|due| due.statutory)
    }

    const fn q(year: i32, quarter: u8) -> TaxPeriod {
        TaxPeriod::Quarter { year, quarter }
    }

    const fn m(year: i32, month: Month) -> TaxPeriod {
        TaxPeriod::Month { year, month }
    }

    #[test]
    fn b3_statutory_due_dates_follow_each_forms_rule() {
        let cases = [
            (BirForm::F2550Q, q(2026, 3), date!(2026 - 10 - 25)),
            (BirForm::F2551Q, q(2026, 4), date!(2027 - 01 - 25)),
            (BirForm::F1601EQ, q(2027, 1), date!(2027 - 04 - 30)),
            (BirForm::F1601EQ, q(2026, 4), date!(2027 - 01 - 31)),
            (BirForm::F0619E, m(2027, Month::January), date!(2027 - 02 - 10)),
            (BirForm::F1601C, m(2026, Month::November), date!(2026 - 12 - 10)),
            (BirForm::F1601C, m(2026, Month::December), date!(2027 - 01 - 15)),
            (BirForm::F1701Q, q(2027, 1), date!(2027 - 05 - 15)),
            (BirForm::F1702Q, q(2026, 3), date!(2026 - 11 - 29)),
            (BirForm::F1701, TaxPeriod::Year(2026), date!(2027 - 04 - 15)),
            (BirForm::F1702, TaxPeriod::Year(2026), date!(2027 - 04 - 15)),
            (BirForm::F1604C, TaxPeriod::Year(2026), date!(2027 - 01 - 31)),
            (BirForm::F1604E, TaxPeriod::Year(2026), date!(2027 - 03 - 01)),
        ];
        for (form, period, expected) in cases {
            assert_eq!(statutory(form, period), Some(expected), "{form} {period}");
        }
    }

    #[test]
    fn b3_due_dates_on_weekends_and_holidays_move_to_the_next_working_day() {
        let cases = [
            (BirForm::F1601C, m(2026, Month::November), date!(2026 - 12 - 10)),
            (BirForm::F2550Q, q(2026, 3), date!(2026 - 10 - 26)),
            (BirForm::F1601EQ, q(2026, 3), date!(2026 - 11 - 03)),
            (BirForm::F1702Q, q(2026, 3), date!(2026 - 12 - 01)),
            (BirForm::F1601EQ, q(2026, 4), date!(2027 - 02 - 01)),
            (BirForm::F1604C, TaxPeriod::Year(2026), date!(2027 - 02 - 01)),
            (BirForm::F1701Q, q(2027, 1), date!(2027 - 05 - 17)),
            (BirForm::F1702Q, q(2027, 2), date!(2027 - 08 - 31)),
        ];
        for (form, period, expected) in cases {
            let due = calendar().due_date(form, period).map(|due| due.date);
            assert_eq!(due, Some(expected), "{form} {period}");
        }
    }

    #[test]
    fn b3_a_bir_extension_is_applied_in_one_place() {
        let extended = format!(
            "{BUILT_IN}\n[[extension]]\nforms = [\"2550Q\"]\nperiod = \"2026-Q3\"\n\
             due = \"2026-11-06\"\nsource = \"RMC 999-2026\"\n"
        );
        let calendar = Calendar::from_toml(&extended).unwrap();

        let due = calendar.due_date(BirForm::F2550Q, q(2026, 3)).unwrap();
        assert_eq!(
            (due.date, due.extension.as_deref()),
            (date!(2026 - 11 - 06), Some("RMC 999-2026"))
        );
        assert_eq!(due.statutory, date!(2026 - 10 - 25));
        let other_form = calendar.due_date(BirForm::F2551Q, q(2026, 3)).unwrap();
        assert_eq!((other_form.date, other_form.extension), (date!(2026 - 10 - 26), None));
        let other_period = calendar.due_date(BirForm::F2550Q, q(2026, 4)).unwrap();
        assert_eq!(other_period.date, date!(2027 - 01 - 25));
    }

    #[test]
    fn tax_periods_round_trip_through_their_keys() {
        for period in [m(2026, Month::November), q(2026, 3), TaxPeriod::Year(2026)] {
            assert_eq!(TaxPeriod::parse_key(&period.key()), Some(period));
        }
        assert_eq!(q(2026, 3).key(), "2026-Q3");
        assert_eq!(TaxPeriod::parse_key("2026-Q5"), None);
    }

    #[test]
    fn b3_built_in_calendar_covers_the_next_12_months() {
        let (from, until) = (date!(2026 - 10 - 21), date!(2027 - 10 - 21));
        let calendar = calendar();
        assert!(calendar.holidays_cover(from, until));

        let listing: String = calendar
            .due_dates_between(from, until)
            .iter()
            .map(|(form, period, due)| format!("{} {form:<8} {period:<15} {}\n", due.date, due.statutory))
            .collect();
        insta::assert_snapshot!(listing);
    }

    #[test]
    fn b3_forms_are_only_due_for_the_periods_they_cover() {
        assert_eq!(statutory(BirForm::F0619E, m(2026, Month::December)), None);
        assert_eq!(statutory(BirForm::F1701Q, q(2026, 4)), None);
        assert_eq!(statutory(BirForm::F1702Q, q(2026, 4)), None);
        assert_eq!(statutory(BirForm::F2550Q, m(2026, Month::September)), None);
        assert_eq!(statutory(BirForm::F1701, q(2026, 4)), None);
    }

    #[test]
    fn form_numbers_parse_as_people_write_them() {
        assert_eq!(BirForm::parse("1601eq"), Some(BirForm::F1601EQ));
        assert_eq!(BirForm::parse(" 0619-E "), Some(BirForm::F0619E));
        assert_eq!(BirForm::parse("1701A"), Some(BirForm::F1701));
        assert_eq!(BirForm::parse("1702-RT"), Some(BirForm::F1702));
        assert_eq!(BirForm::parse("2550M"), None);
    }
}
