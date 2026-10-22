//! Payroll chasing (P1, P2): before each payday, the documents payroll
//! needs; after each month's contribution due dates, the proof that SSS,
//! PhilHealth and Pag-IBIG were paid. Kompleto never computes pay,
//! contributions or tax: these Periods only ask for documents.

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

/// An agency whose contributions an employer remits each month.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Agency {
    Sss,
    PhilHealth,
    PagIbig,
}

impl Agency {
    /// The Owed document: proof that the month's contributions were paid.
    pub fn receipt(self) -> &'static str {
        match self {
            Self::Sss => "SSS payment receipt (with the PRN)",
            Self::PhilHealth => "PhilHealth payment receipt (SPA, ePAR or OR)",
            Self::PagIbig => "Pag-IBIG payment receipt (MCRF or OR)",
        }
    }
}

/// What a payroll Period asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayrollKind {
    /// The documents for one payday (P1).
    Inputs {
        /// The payday.
        payday: Date,
    },
    /// The receipts for one month's contributions (P2).
    Receipts {
        /// The month the contributions are for.
        year: i32,
        /// The month the contributions are for.
        month: Month,
        /// Each agency's due date.
        dues: [(Agency, Date); 3],
    },
}

/// One payroll Period: its documents, when it opens and its Cut-off.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayrollPeriod {
    /// What it asks for.
    pub kind: PayrollKind,
    /// The day of the first request.
    pub opens_on: Date,
    /// The day the Firm wants the documents by.
    pub cut_off: Date,
}

impl PayrollPeriod {
    /// The storage key: `payroll-2026-10-15` or `remit-2026-09`.
    pub fn key(&self) -> String {
        match &self.kind {
            PayrollKind::Inputs { payday } => format!("payroll-{payday}"),
            PayrollKind::Receipts { year, month, .. } => format!("remit-{year}-{:02}", u8::from(*month)),
        }
    }

    /// The Owed documents on `today`: receipts only once their due date
    /// has passed.
    pub fn owed_on(&self, today: Date) -> Vec<String> {
        match &self.kind {
            PayrollKind::Inputs { .. } => PAYROLL_INPUTS.iter().map(|&d| d.to_owned()).collect(),
            PayrollKind::Receipts { dues, .. } => dues
                .iter()
                .filter(|(_, due)| *due < today)
                .map(|(agency, _)| agency.receipt().to_owned())
                .collect(),
        }
    }
}

impl PayrollSettings {
    /// The Client's payroll Periods whose Cut-off is on or after `today`
    /// and that open within 60 days.
    pub fn upcoming(&self, calendar: &Calendar, today: Date) -> Vec<PayrollPeriod> {
        let months: Vec<(i32, Month)> = (-3..=2).map(|n| month_offset(today, n)).collect();
        let inputs = months.iter().flat_map(|&(year, month)| {
            self.cycle.paydays(year, month).into_iter().map(|payday| PayrollPeriod {
                kind: PayrollKind::Inputs { payday },
                opens_on: payday - Duration::days(INPUTS_OPEN_DAYS_BEFORE),
                cut_off: payday - Duration::days(INPUTS_CUT_OFF_DAYS_BEFORE),
            })
        });
        let receipts = months.iter().map(|&(year, month)| self.receipts(calendar, year, month));
        let mut periods: Vec<PayrollPeriod> = inputs
            .chain(receipts)
            .filter(|p| p.cut_off >= today && p.opens_on <= today + Duration::days(60))
            .collect();
        periods.sort_by_key(|p| (p.opens_on, p.cut_off));
        periods
    }

    /// The receipts Period for one month's contributions.
    fn receipts(&self, calendar: &Calendar, year: i32, month: Month) -> PayrollPeriod {
        let (due_year, due_month) = next_month(year, month);
        let due = |day| calendar.next_working_day(day_in_month(due_year, due_month, day));
        let dues = [
            (Agency::Sss, due(31)),
            (Agency::PhilHealth, due(self.philhealth_due_day)),
            (Agency::PagIbig, due(self.pagibig_due_day)),
        ];
        let earliest = dues.iter().map(|(_, d)| *d).min().unwrap_or(Date::MIN);
        let latest = dues.iter().map(|(_, d)| *d).max().unwrap_or(Date::MIN);
        PayrollPeriod {
            kind: PayrollKind::Receipts { year, month, dues },
            opens_on: earliest + Duration::days(1),
            cut_off: latest + Duration::days(RECEIPTS_CUT_OFF_DAYS_AFTER),
        }
    }
}

/// What a payroll Period key is about, for labels and the upload page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayrollKey {
    /// A payday's documents.
    Inputs(Date),
    /// A month's contribution receipts.
    Receipts(i32, Month),
}

impl PayrollKey {
    /// The key's meaning, or `None` if it isn't a payroll key.
    pub fn parse(key: &str) -> Option<Self> {
        if let Some(date) = key.strip_prefix("payroll-") {
            let format = time::macros::format_description!("[year]-[month]-[day]");
            return Date::parse(date, &format).ok().map(Self::Inputs);
        }
        let (year, month) = key.strip_prefix("remit-")?.split_once('-')?;
        let month = Month::try_from(month.parse::<u8>().ok()?).ok()?;
        Some(Self::Receipts(year.parse().ok()?, month))
    }

    /// How people see it, e.g. "payroll on Thu 15 Oct 2026".
    pub fn label(self) -> String {
        match self {
            Self::Inputs(payday) => format!("payroll on {}", show_date(payday)),
            Self::Receipts(year, month) => format!("{month} {year} SSS, PhilHealth and Pag-IBIG contributions"),
        }
    }

    /// Everything the upload page offers: every receipt, even before its
    /// due date, since a Contact may have paid early.
    pub fn documents(self) -> Vec<String> {
        match self {
            Self::Inputs(_) => PAYROLL_INPUTS.iter().map(|&d| d.to_owned()).collect(),
            Self::Receipts(..) => [Agency::Sss, Agency::PhilHealth, Agency::PagIbig]
                .iter()
                .map(|agency| agency.receipt().to_owned())
                .collect(),
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
            .filter_map(|p| match p.kind {
                PayrollKind::Inputs { payday } => Some((payday, p.opens_on, p.cut_off)),
                PayrollKind::Receipts { .. } => None,
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
        assert_eq!(
            periods
                .iter()
                .find(|p| p.key() == "payroll-2026-10-31")
                .unwrap()
                .owed_on(date!(2026 - 10 - 23)),
            PAYROLL_INPUTS
        );
    }

    #[test]
    fn p1_monthly_pays_on_the_last_day_only() {
        let calendar = Calendar::built_in().unwrap();
        let periods = settings(PayCycle::Monthly).upcoming(&calendar, date!(2027 - 02 - 01));

        let paydays: Vec<Date> = inputs(&periods).into_iter().map(|(payday, ..)| payday).collect();
        assert_eq!(paydays[..2], [date!(2027 - 02 - 28), date!(2027 - 03 - 31)]);
    }

    #[test]
    fn p2_each_receipt_is_owed_once_its_due_date_has_passed() {
        let calendar = Calendar::built_in().unwrap();
        let settings = PayrollSettings {
            philhealth_due_day: 15,
            ..settings(PayCycle::Monthly)
        };
        let september = settings
            .upcoming(&calendar, date!(2026 - 10 - 01))
            .into_iter()
            .find(|p| p.key() == "remit-2026-09")
            .unwrap();

        // October 2026: PhilHealth on Thu the 15th; SSS and Pag-IBIG on
        // Sat the 31st, moved to Tue 3 November past All Saints' and All
        // Souls' Days.
        assert_eq!(september.opens_on, date!(2026 - 10 - 16));
        assert_eq!(september.cut_off, date!(2026 - 11 - 10));
        assert!(september.owed_on(date!(2026 - 10 - 15)).is_empty());
        assert_eq!(september.owed_on(date!(2026 - 10 - 16)), [Agency::PhilHealth.receipt()]);
        assert_eq!(september.owed_on(date!(2026 - 11 - 03)).len(), 1);
        assert_eq!(september.owed_on(date!(2026 - 11 - 04)).len(), 3);
    }

    #[test]
    fn p2_keys_name_the_payday_or_the_month_and_read_back() {
        assert_eq!(
            PayrollKey::parse("payroll-2026-10-15"),
            Some(PayrollKey::Inputs(date!(2026 - 10 - 15)))
        );
        assert_eq!(
            PayrollKey::parse("remit-2026-09"),
            Some(PayrollKey::Receipts(2026, Month::September))
        );
        assert_eq!(PayrollKey::parse("2026-09"), None);
        assert_eq!(PayrollKey::parse("remit-2026-13"), None);
        assert_eq!(
            PayrollKey::Inputs(date!(2026 - 10 - 15)).label(),
            "payroll on Thu 15 Oct 2026"
        );
        assert_eq!(
            PayrollKey::Receipts(2026, Month::September).label(),
            "September 2026 SSS, PhilHealth and Pag-IBIG contributions"
        );
    }
}
