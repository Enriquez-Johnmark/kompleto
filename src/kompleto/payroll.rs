//! Payroll chasing (P1, P2): before each payday, the documents payroll
//! needs; after each month's contribution due dates, the proof that SSS,
//! PhilHealth and Pag-IBIG were paid. Kompleto never computes pay,
//! contributions or tax: these Periods only ask for documents.

use std::collections::BTreeMap;
use std::fmt;

use time::{Date, Duration, Month};

use super::calendar::{Calendar, day_in_month};
use super::dates::show_date;

/// The documents asked for before each payday (P1).
pub const PAYROLL_INPUTS: [&str; 3] = [
    "Timesheets or DTRs",
    "Overtime and leave approvals",
    "New hires and resignations",
];
/// How many days before payday the first request comes.
const INPUTS_OPEN_DAYS_BEFORE: i64 = 8;
/// How many days before payday the Cut-off is.
const INPUTS_CUT_OFF_DAYS_BEFORE: i64 = 3;
/// How many days after the last contribution due date the Cut-off is.
const RECEIPTS_CUT_OFF_DAYS_AFTER: i64 = 7;
/// The default PhilHealth due day of the following month: the latest
/// window for regular employers (P2).
pub const PHILHEALTH_DEFAULT_DAY: u8 = 20;
/// The default Pag-IBIG due day: the month's last day.
pub const PAGIBIG_DEFAULT_DAY: u8 = 31;

/// When a Client's employees are paid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayCycle {
    /// On the 15th and the last day of each month.
    TwiceMonthly,
    /// On the last day of each month.
    Monthly,
}

impl PayCycle {
    /// Its stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TwiceMonthly => "twice-monthly",
            Self::Monthly => "monthly",
        }
    }

    /// The cycle a stored name names.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "twice-monthly" => Some(Self::TwiceMonthly),
            "monthly" => Some(Self::Monthly),
            _ => None,
        }
    }

    /// The paydays in a month.
    fn paydays(self, year: i32, month: Month) -> Vec<Date> {
        let last = day_in_month(year, month, 31);
        match self {
            Self::TwiceMonthly => vec![day_in_month(year, month, 15), last],
            Self::Monthly => vec![last],
        }
    }
}

/// A Client's payroll chasing, when it's on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayrollSettings {
    /// When employees are paid.
    pub cycle: PayCycle,
    /// PhilHealth's due day of the following month, 1–31 (31: the last day).
    pub philhealth_due_day: u8,
    /// Pag-IBIG's due day of the following month, 1–31.
    pub pagibig_due_day: u8,
}

/// A Client's payroll chasing as stored: on or off, with the due days
/// kept while it's off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayrollChoice {
    /// When employees are paid; `None` turns payroll chasing off.
    pub cycle: Option<PayCycle>,
    /// PhilHealth's due day of the following month, 1–31.
    pub philhealth_due_day: u8,
    /// Pag-IBIG's due day of the following month, 1–31.
    pub pagibig_due_day: u8,
}

impl Default for PayrollChoice {
    /// Off, with the default due days.
    fn default() -> Self {
        Self {
            cycle: None,
            philhealth_due_day: PHILHEALTH_DEFAULT_DAY,
            pagibig_due_day: PAGIBIG_DEFAULT_DAY,
        }
    }
}

impl PayrollChoice {
    /// The settings to chase with, or `None` when it's off.
    pub fn settings(self) -> Option<PayrollSettings> {
        self.cycle.map(|cycle| PayrollSettings {
            cycle,
            philhealth_due_day: self.philhealth_due_day,
            pagibig_due_day: self.pagibig_due_day,
        })
    }
}

/// An agency whose contributions an employer remits each month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Agency {
    Sss,
    PhilHealth,
    PagIbig,
}

impl Agency {
    const ALL: [Self; 3] = [Self::Sss, Self::PhilHealth, Self::PagIbig];

    /// The Owed document: proof that the month's contributions were paid.
    pub fn receipt(self) -> &'static str {
        match self {
            Self::Sss => "SSS payment receipt (with the PRN)",
            Self::PhilHealth => "PhilHealth payment receipt (SPA, ePAR or OR)",
            Self::PagIbig => "Pag-IBIG payment receipt (MCRF or OR)",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Sss => "SSS",
            Self::PhilHealth => "PhilHealth",
            Self::PagIbig => "Pag-IBIG",
        }
    }

    /// Its part of a Period key.
    fn code(self) -> &'static str {
        match self {
            Self::Sss => "sss",
            Self::PhilHealth => "philhealth",
            Self::PagIbig => "pagibig",
        }
    }
}

/// One payroll Period: what it asks for, when it opens and its Cut-off.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayrollPeriod {
    /// What it asks for, which is also its storage key.
    pub key: PayrollKey,
    /// The day of the first request.
    pub opens_on: Date,
    /// The day the Firm wants the documents by.
    pub cut_off: Date,
}

impl PayrollSettings {
    /// The Client's payroll Periods whose Cut-off is on or after `today`
    /// and that open within 60 days.
    pub fn upcoming(&self, calendar: &Calendar, today: Date) -> Vec<PayrollPeriod> {
        let months: Vec<(i32, Month)> = (-2..=2).map(|n| month_offset(today, n)).collect();
        let inputs = months.iter().flat_map(|&(year, month)| {
            self.cycle.paydays(year, month).into_iter().map(|payday| PayrollPeriod {
                key: PayrollKey::Inputs(payday),
                opens_on: payday - Duration::days(INPUTS_OPEN_DAYS_BEFORE),
                cut_off: payday - Duration::days(INPUTS_CUT_OFF_DAYS_BEFORE),
            })
        });
        let receipts = months
            .iter()
            .flat_map(|&(year, month)| self.receipts(calendar, year, month));
        let mut periods: Vec<PayrollPeriod> = inputs
            .chain(receipts)
            .filter(|p| p.cut_off >= today && p.opens_on <= today + Duration::days(60))
            .collect();
        periods.sort_by_key(|p| (p.opens_on, p.cut_off));
        periods
    }

    /// Whether `key` is still one of this Client's payroll Periods, after
    /// its settings changed.
    pub fn still_has(&self, calendar: &Calendar, key: &PayrollKey) -> bool {
        match key {
            PayrollKey::Inputs(payday) => self.cycle.paydays(payday.year(), payday.month()).contains(payday),
            PayrollKey::Receipts { year, month, .. } => self
                .receipts(calendar, *year, *month)
                .iter()
                .any(|period| period.key == *key),
        }
    }

    /// The receipts Periods for one month's contributions: one per due
    /// date, so each is asked for once it's due, and a Reply to one
    /// request never stops the requests for receipts due later.
    fn receipts(&self, calendar: &Calendar, year: i32, month: Month) -> Vec<PayrollPeriod> {
        let (due_year, due_month) = next_month(year, month);
        let due = |agency| {
            let day = match agency {
                Agency::Sss => 31,
                Agency::PhilHealth => self.philhealth_due_day,
                Agency::PagIbig => self.pagibig_due_day,
            };
            calendar.next_working_day(day_in_month(due_year, due_month, day))
        };
        let mut by_due: BTreeMap<Date, Vec<Agency>> = BTreeMap::new();
        for agency in Agency::ALL {
            by_due.entry(due(agency)).or_default().push(agency);
        }
        by_due
            .into_iter()
            .map(|(due, agencies)| PayrollPeriod {
                key: PayrollKey::Receipts { year, month, agencies },
                opens_on: due + Duration::days(1),
                cut_off: due + Duration::days(RECEIPTS_CUT_OFF_DAYS_AFTER),
            })
            .collect()
    }
}

/// What a payroll Period asks for; written out, its storage key:
/// `payroll-2026-10-15` or `remit-2026-09-sss-pagibig`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayrollKey {
    /// A payday's documents (P1).
    Inputs(Date),
    /// Receipts for one month's contributions to `agencies`, all due on
    /// the same day (P2).
    Receipts {
        year: i32,
        month: Month,
        agencies: Vec<Agency>,
    },
}

impl fmt::Display for PayrollKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inputs(payday) => write!(f, "payroll-{payday}"),
            Self::Receipts { year, month, agencies } => {
                write!(f, "remit-{year}-{:02}", u8::from(*month))?;
                agencies.iter().try_for_each(|agency| write!(f, "-{}", agency.code()))
            }
        }
    }
}

impl PayrollKey {
    /// The key's meaning, or `None` if it isn't a payroll key.
    pub fn parse(key: &str) -> Option<Self> {
        if let Some(date) = key.strip_prefix("payroll-") {
            let format = time::macros::format_description!("[year]-[month]-[day]");
            return Date::parse(date, &format).ok().map(Self::Inputs);
        }
        let mut parts = key.strip_prefix("remit-")?.split('-');
        let year = parts.next()?.parse().ok()?;
        let month = Month::try_from(parts.next()?.parse::<u8>().ok()?).ok()?;
        let agencies: Vec<Agency> = parts
            .map(|code| Agency::ALL.into_iter().find(|agency| agency.code() == code))
            .collect::<Option<_>>()?;
        let parsed = Self::Receipts { year, month, agencies };
        // Only the spelling `Display` writes: one key per Period.
        (!matches!(&parsed, Self::Receipts { agencies, .. } if agencies.is_empty()) && parsed.to_string() == key)
            .then_some(parsed)
    }

    /// How people see it, e.g. "payroll on Thu 15 Oct 2026".
    pub fn label(&self) -> String {
        match self {
            Self::Inputs(payday) => format!("payroll on {}", show_date(*payday)),
            Self::Receipts { year, month, agencies } => {
                let names: Vec<&str> = agencies.iter().map(|agency| agency.name()).collect();
                let names = match names.as_slice() {
                    [.., last] if names.len() > 1 => format!("{} and {last}", names[..names.len() - 1].join(", ")),
                    _ => names.concat(),
                };
                format!("{month} {year} {names} contributions")
            }
        }
    }

    /// The Owed documents.
    pub fn documents(&self) -> Vec<String> {
        match self {
            Self::Inputs(_) => PAYROLL_INPUTS.iter().map(|&d| d.to_owned()).collect(),
            Self::Receipts { agencies, .. } => agencies.iter().map(|agency| agency.receipt().to_owned()).collect(),
        }
    }
}

fn next_month(year: i32, month: Month) -> (i32, Month) {
    if month == Month::December {
        (year + 1, Month::January)
    } else {
        (year, month.next())
    }
}

/// The month `n` months from `today`'s.
fn month_offset(today: Date, n: i32) -> (i32, Month) {
    let index = today.year() * 12 + i32::from(u8::from(today.month())) - 1 + n;
    let month = u8::try_from(index.rem_euclid(12) + 1).unwrap_or(1);
    (index.div_euclid(12), Month::try_from(month).unwrap_or(Month::January))
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;

    fn settings(cycle: PayCycle) -> PayrollSettings {
        PayrollSettings {
            cycle,
            philhealth_due_day: PHILHEALTH_DEFAULT_DAY,
            pagibig_due_day: PAGIBIG_DEFAULT_DAY,
        }
    }

    fn inputs(periods: &[PayrollPeriod]) -> Vec<(Date, Date, Date)> {
        periods
            .iter()
            .filter_map(|p| match p.key {
                PayrollKey::Inputs(payday) => Some((payday, p.opens_on, p.cut_off)),
                PayrollKey::Receipts { .. } => None,
            })
            .collect()
    }

    #[test]
    fn p1_twice_a_month_asks_8_days_before_each_payday_with_a_cut_off_3_days_before() {
        let calendar = Calendar::built_in().unwrap();
        let periods = settings(PayCycle::TwiceMonthly).upcoming(&calendar, date!(2026 - 10 - 22));

        assert_eq!(
            inputs(&periods)[..3],
            [
                (date!(2026 - 10 - 31), date!(2026 - 10 - 23), date!(2026 - 10 - 28)),
                (date!(2026 - 11 - 15), date!(2026 - 11 - 07), date!(2026 - 11 - 12)),
                (date!(2026 - 11 - 30), date!(2026 - 11 - 22), date!(2026 - 11 - 27)),
            ]
        );
        assert_eq!(PayrollKey::Inputs(date!(2026 - 10 - 31)).documents(), PAYROLL_INPUTS);
    }

    #[test]
    fn p1_monthly_pays_on_the_last_day_only() {
        let calendar = Calendar::built_in().unwrap();
        let periods = settings(PayCycle::Monthly).upcoming(&calendar, date!(2027 - 02 - 01));

        let paydays: Vec<Date> = inputs(&periods).into_iter().map(|(payday, ..)| payday).collect();
        assert_eq!(paydays[..2], [date!(2027 - 02 - 28), date!(2027 - 03 - 31)]);
    }

    #[test]
    fn p2_each_due_date_gets_its_own_receipts_period() {
        let calendar = Calendar::built_in().unwrap();
        let settings = PayrollSettings {
            philhealth_due_day: 15,
            ..settings(PayCycle::Monthly)
        };
        let september: Vec<(String, Date, Date)> = settings
            .upcoming(&calendar, date!(2026 - 10 - 01))
            .into_iter()
            .filter(|p| p.key.to_string().starts_with("remit-2026-09"))
            .map(|p| (p.key.to_string(), p.opens_on, p.cut_off))
            .collect();

        // October 2026: PhilHealth on Thu the 15th; SSS and Pag-IBIG on
        // Sat the 31st, moved to Tue 3 November past All Saints' and All
        // Souls' Days.
        assert_eq!(
            september,
            [
                (
                    "remit-2026-09-philhealth".to_owned(),
                    date!(2026 - 10 - 16),
                    date!(2026 - 10 - 22)
                ),
                (
                    "remit-2026-09-sss-pagibig".to_owned(),
                    date!(2026 - 11 - 04),
                    date!(2026 - 11 - 10)
                ),
            ]
        );
    }

    #[test]
    fn p2_keys_name_the_payday_or_the_month_and_agencies_and_read_back() {
        let receipts = PayrollKey::Receipts {
            year: 2026,
            month: Month::September,
            agencies: vec![Agency::Sss, Agency::PagIbig],
        };
        assert_eq!(
            PayrollKey::parse("payroll-2026-10-15"),
            Some(PayrollKey::Inputs(date!(2026 - 10 - 15)))
        );
        assert_eq!(PayrollKey::parse("remit-2026-09-sss-pagibig"), Some(receipts.clone()));
        for other in [
            "2026-09",
            "remit-2026-09",
            "remit-2026-13-sss",
            "remit-2026-9-sss",
            "remit-2026-09-gsis",
        ] {
            assert_eq!(PayrollKey::parse(other), None, "{other}");
        }
        assert_eq!(
            PayrollKey::Inputs(date!(2026 - 10 - 15)).label(),
            "payroll on Thu 15 Oct 2026"
        );
        assert_eq!(receipts.label(), "September 2026 SSS and Pag-IBIG contributions");
        assert_eq!(receipts.documents().len(), 2);
    }
}
