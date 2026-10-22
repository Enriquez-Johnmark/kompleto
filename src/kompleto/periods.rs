//! Collection Periods (B4): a Client is asked once a month for everything
//! its forms need for the month that just ended. Its Cut-off is the earliest
//! Due date among those forms, minus the Firm's cut-off offset, but never
//! before the day the Period opens.

use std::collections::HashMap;

use time::{Date, Duration, Month};

use super::calendar::{BirForm, Calendar, TaxPeriod};
use super::payroll::PayrollKey;

/// Due dates the bookkeeper set by hand for one Client, replacing the
/// calendar's date for that form and Period only (B3).
pub type DueOverrides = HashMap<(BirForm, TaxPeriod), Date>;

/// One form a Client files for one Period.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filing {
    /// The form.
    pub form: BirForm,
    /// The month, quarter or year it covers.
    pub period: TaxPeriod,
    /// When it's due for this Client.
    pub due: Date,
    /// True if the bookkeeper overrode the calendar's date.
    pub overridden: bool,
    /// The BIR issuance behind an extension of the calendar's date.
    pub extension: Option<String>,
}

/// Everything a Client is asked for at once: the forms whose Periods end in
/// one month.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionPeriod {
    /// The month the forms' Periods end in.
    pub year: i32,
    /// The month the forms' Periods end in.
    pub month: Month,
    /// What the Client is told it covers: the month, or the quarter (or
    /// year) when the Client files nothing monthly.
    pub label: TaxPeriod,
    /// The forms due, earliest first.
    pub filings: Vec<Filing>,
    /// The date the Firm wants the documents by.
    pub cut_off: Date,
}

impl CollectionPeriod {
    /// The storage key of the month, e.g. `2026-11`.
    pub fn key(&self) -> String {
        TaxPeriod::Month {
            year: self.year,
            month: self.month,
        }
        .key()
    }

    /// The day documents become due: the day after the month ends.
    pub fn opens_on(&self) -> Date {
        TaxPeriod::Month {
            year: self.year,
            month: self.month,
        }
        .last_day()
            + Duration::days(1)
    }
}

/// One Client's view of the calendar.
#[derive(Debug, Clone, Copy)]
pub struct ClientCalendar<'a> {
    /// The built-in calendar.
    pub calendar: &'a Calendar,
    /// The forms the Client files.
    pub forms: &'a [BirForm],
    /// The Client's own due dates.
    pub overrides: &'a DueOverrides,
    /// How many days before the earliest Due date the Cut-off falls.
    pub cut_off_offset_days: u8,
}

impl ClientCalendar<'_> {
    /// The collection Period for the month `month` of `year`, or `None` if
    /// none of the Client's forms has a Period ending that month.
    pub fn period(&self, year: i32, month: Month) -> Option<CollectionPeriod> {
        let mut ending = vec![TaxPeriod::Month { year, month }];
        if u8::from(month) % 3 == 0 {
            ending.push(TaxPeriod::Quarter {
                year,
                quarter: u8::from(month) / 3,
            });
        }
        if month == Month::December {
            ending.push(TaxPeriod::Year(year));
        }
        let mut filings: Vec<Filing> = ending
            .iter()
            .flat_map(|&period| self.forms.iter().map(move |&form| (form, period)))
            .filter_map(|(form, period)| {
                let calendar = self.calendar.due_date(form, period)?;
                let overridden = self.overrides.get(&(form, period)).copied();
                Some(Filing {
                    form,
                    period,
                    due: overridden.unwrap_or(calendar.date),
                    overridden: overridden.is_some(),
                    extension: calendar.extension.filter(|_| overridden.is_none()),
                })
            })
            .collect();
        filings.sort_by_key(|filing| (filing.due, filing.form));
        let earliest = filings.first()?.due;
        // The narrowest Period among the forms names what's being collected.
        let label = ending
            .into_iter()
            .find(|period| filings.iter().any(|f| f.period == *period))?;
        Some(CollectionPeriod {
            year,
            month,
            label,
            filings,
            // Never before the documents exist, or no Draft could be sent.
            cut_off: (earliest - Duration::days(i64::from(self.cut_off_offset_days)))
                .max(TaxPeriod::Month { year, month }.last_day() + Duration::days(1)),
        })
    }

    /// The Client's collection Periods whose Cut-off is on or after `today`,
    /// up to a year ahead.
    pub fn upcoming(&self, today: Date) -> Vec<CollectionPeriod> {
        let until = today + Duration::days(365);
        // A month's documents are due at most about four months after it ends.
        let first = i32::from(u8::from(today.month())) - 5;
        (first..first + 17)
            .filter_map(|n| self.period(today.year() + n.div_euclid(12), month_from_index(n.rem_euclid(12))))
            .filter(|period| (today..=until).contains(&period.cut_off))
            .collect()
    }
}

/// How people see a Period's storage key: "September 2026", "Q3 2026",
/// "payroll on Thu 15 Oct 2026"; an unknown key as it is.
pub fn period_label(key: &str) -> String {
    if let Some(payroll) = PayrollKey::parse(key) {
        return payroll.label();
    }
    TaxPeriod::parse_key(key).map_or_else(|| key.to_owned(), |period| period.to_string())
}

/// The month at 0-based index `index` (0 = January).
fn month_from_index(index: i32) -> Month {
    u8::try_from(index + 1)
        .ok()
        .and_then(|n| Month::try_from(n).ok())
        .unwrap_or(Month::January)
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;
    use crate::kompleto::calendar::BirForm::*;

    fn period(forms: &[BirForm], overrides: &DueOverrides, year: i32, month: Month) -> Option<CollectionPeriod> {
        let calendar = Calendar::built_in().unwrap();
        ClientCalendar {
            calendar: &calendar,
            forms,
            overrides,
            cut_off_offset_days: 5,
        }
        .period(year, month)
    }

    #[test]
    fn p1_p2_payroll_periods_are_named_in_words() {
        assert_eq!(period_label("2026-09"), "September 2026");
        assert_eq!(period_label("payroll-2026-10-15"), "payroll on Thu 15 Oct 2026");
        assert_eq!(
            period_label("remit-2026-09"),
            "September 2026 SSS, PhilHealth and Pag-IBIG contributions"
        );
    }

    #[test]
    fn b4_cut_off_is_the_earliest_due_date_minus_the_offset() {
        let vat_employer = [F1601C, F0619E, F1601EQ, F2550Q, F1701Q];

        let november = period(&vat_employer, &DueOverrides::new(), 2026, Month::November).unwrap();
        assert_eq!(
            november.label,
            TaxPeriod::Month {
                year: 2026,
                month: Month::November
            }
        );
        assert_eq!(
            november.filings.iter().map(|f| f.form).collect::<Vec<_>>(),
            [F0619E, F1601C]
        );
        assert_eq!(november.cut_off, date!(2026 - 12 - 05));

        let december = period(&vat_employer, &DueOverrides::new(), 2026, Month::December).unwrap();
        let forms: Vec<_> = december.filings.iter().map(|f| f.form).collect();
        assert_eq!(forms, [F1601C, F2550Q, F1601EQ]);
        assert_eq!(december.cut_off, date!(2027 - 01 - 10));
    }

    #[test]
    fn b4_a_client_filing_only_quarterly_is_asked_once_a_quarter() {
        let non_vat_individual = [F2551Q, F1701Q, F1701];
        let none = DueOverrides::new();

        assert_eq!(period(&non_vat_individual, &none, 2027, Month::January), None);
        assert_eq!(period(&non_vat_individual, &none, 2027, Month::February), None);
        let march = period(&non_vat_individual, &none, 2027, Month::March).unwrap();
        assert_eq!(march.label, TaxPeriod::Quarter { year: 2027, quarter: 1 });
        assert_eq!(march.cut_off, date!(2027 - 04 - 21));
    }

    #[test]
    fn b3_a_per_client_override_changes_only_that_clients_dates() {
        let forms = [F1601C];
        let november = TaxPeriod::Month {
            year: 2026,
            month: Month::November,
        };
        let overrides = DueOverrides::from([((F1601C, november), date!(2026 - 12 - 14))]);

        let overridden = period(&forms, &overrides, 2026, Month::November).unwrap();
        assert_eq!(overridden.filings[0].due, date!(2026 - 12 - 14));
        assert!(overridden.filings[0].overridden);
        assert_eq!(overridden.cut_off, date!(2026 - 12 - 09));

        let other_client = period(&forms, &DueOverrides::new(), 2026, Month::November).unwrap();
        assert_eq!(other_client.filings[0].due, date!(2026 - 12 - 10));
        let other_month = period(&forms, &overrides, 2026, Month::October).unwrap();
        assert_eq!(other_month.filings[0].due, date!(2026 - 11 - 10));
    }

    #[test]
    fn b4_the_cut_off_is_never_before_the_period_opens() {
        let calendar = Calendar::built_in().unwrap();
        let forms = [F1601C];
        let client = ClientCalendar {
            calendar: &calendar,
            forms: &forms,
            overrides: &DueOverrides::new(),
            cut_off_offset_days: 10,
        };

        // 1601-C for November is due 10 Dec; 10 days earlier is 30 Nov,
        // before November's documents can exist.
        let november = client.period(2026, Month::November).unwrap();

        assert_eq!(november.cut_off, date!(2026 - 12 - 01));
        assert_eq!(november.cut_off, november.opens_on());
    }

    #[test]
    fn b4_upcoming_lists_periods_whose_cut_off_is_still_ahead() {
        let calendar = Calendar::built_in().unwrap();
        let forms = [F2550Q];
        let client = ClientCalendar {
            calendar: &calendar,
            forms: &forms,
            overrides: &DueOverrides::new(),
            cut_off_offset_days: 5,
        };

        let upcoming = client.upcoming(date!(2026 - 10 - 21));
        let cut_offs: Vec<_> = upcoming.iter().map(|p| p.cut_off).collect();
        // Q3 2026's cut-off (21 Oct) is today, so it's still listed.
        assert_eq!(
            cut_offs,
            [
                date!(2026 - 10 - 21),
                date!(2027 - 01 - 20),
                date!(2027 - 04 - 21),
                date!(2027 - 07 - 21),
                date!(2027 - 10 - 20)
            ]
        );
    }
}
