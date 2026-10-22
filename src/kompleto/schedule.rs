//! When Drafts appear for one collection Period (D1): a first request the
//! day after the Period ends, then reminders a set number of days before the
//! Cut-off, never more than four in all.

use time::macros::{offset, time};
use time::{Date, Duration, OffsetDateTime, Time, UtcOffset};

use super::periods::CollectionPeriod;

/// The most Drafts one Client gets for one Period (D1).
pub const MAX_DRAFTS_PER_PERIOD: usize = 4;

/// Manila time: UTC+8 all year (no daylight saving since 1978).
pub const MANILA: UtcOffset = offset!(+8);

/// Drafts appear at 07:00 Manila time, ready for the bookkeeper's morning.
const DRAFTS_APPEAR: Time = time!(07:00);

/// Which message in a Period's sequence a Draft is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Slot {
    /// The first request, when the Period's documents become due.
    FirstRequest,
    /// A reminder this many days before the Cut-off.
    Reminder {
        /// Days before the Cut-off.
        days_before: u8,
    },
}

/// The days before the Cut-off a Client gets reminders: up to three, each
/// 1 to 14 days.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReminderDays(Vec<u8>);

/// A schedule the rules don't allow.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ScheduleError {
    /// More reminders than the limit of four messages allows.
    #[error("choose at most {} reminder days", MAX_DRAFTS_PER_PERIOD - 1)]
    TooManyReminders,
    /// A reminder day outside 1–14.
    #[error("reminder days must be between 1 and 14 days before the cut-off")]
    DayOutOfRange,
}

impl ReminderDays {
    /// Checks and normalises a choice of reminder days.
    pub fn new(days: &[u8]) -> Result<Self, ScheduleError> {
        let mut days = days.to_vec();
        days.sort_unstable_by(|a, b| b.cmp(a));
        days.dedup();
        if days.len() > MAX_DRAFTS_PER_PERIOD - 1 {
            return Err(ScheduleError::TooManyReminders);
        }
        if days.iter().any(|day| !(1..=14).contains(day)) {
            return Err(ScheduleError::DayOutOfRange);
        }
        Ok(Self(days))
    }

    /// The days, largest first.
    pub fn days(&self) -> &[u8] {
        &self.0
    }
}

impl Default for ReminderDays {
    /// 7, 3 and 1 days before the Cut-off.
    fn default() -> Self {
        Self(vec![7, 3, 1])
    }
}

/// A Draft due to appear on a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedDraft {
    /// Which message it is.
    pub slot: Slot,
    /// The Manila date it appears on.
    pub on: Date,
}

/// The Drafts for `period`, in date order.
pub fn planned_drafts(period: &CollectionPeriod, reminders: &ReminderDays) -> Vec<PlannedDraft> {
    planned_between(period.opens_on(), period.cut_off, reminders)
}

/// The Drafts for any Period that opens on `opens_on`, in date order.
pub fn planned_between(opens_on: Date, cut_off: Date, reminders: &ReminderDays) -> Vec<PlannedDraft> {
    let first = PlannedDraft {
        slot: Slot::FirstRequest,
        on: opens_on,
    };
    let reminders = reminders.days().iter().map(|&days_before| PlannedDraft {
        slot: Slot::Reminder { days_before },
        on: cut_off - Duration::days(i64::from(days_before)),
    });
    std::iter::once(first)
        .chain(reminders.filter(|reminder| reminder.on > first.on))
        .collect()
}

/// The moment Drafts for `on` appear: 07:00 Manila time.
pub fn appears_at(on: Date) -> OffsetDateTime {
    on.with_time(DRAFTS_APPEAR).assume_offset(MANILA)
}

#[cfg(test)]
mod tests {
    use time::Month;
    use time::macros::{date, datetime};

    use super::*;
    use crate::kompleto::calendar::{BirForm, Calendar};
    use crate::kompleto::periods::{ClientCalendar, DueOverrides};

    fn period(forms: &[BirForm], month: Month) -> CollectionPeriod {
        let calendar = Calendar::built_in().unwrap();
        let overrides = DueOverrides::new();
        let client = ClientCalendar {
            calendar: &calendar,
            forms,
            overrides: &overrides,
            cut_off_offset_days: 5,
        };
        client.period(2026, month).unwrap()
    }

    fn dates(drafts: &[PlannedDraft]) -> Vec<Date> {
        drafts.iter().map(|draft| draft.on).collect()
    }

    #[test]
    fn d1_first_request_then_reminders_7_3_and_1_days_before_cut_off() {
        // Cut-off 10 Jan 2027 (1601-C for December is due 15 Jan).
        let drafts = planned_drafts(&period(&[BirForm::F1601C], Month::December), &ReminderDays::default());

        assert_eq!(
            dates(&drafts),
            [
                date!(2027 - 01 - 01),
                date!(2027 - 01 - 03),
                date!(2027 - 01 - 07),
                date!(2027 - 01 - 09)
            ]
        );
        assert_eq!(drafts[0].slot, Slot::FirstRequest);
        assert_eq!(drafts[1].slot, Slot::Reminder { days_before: 7 });
    }

    #[test]
    fn d1_reminders_on_or_before_the_first_request_are_skipped() {
        // Cut-off 5 Dec 2026: the 7-day reminder would land on 28 Nov.
        let drafts = planned_drafts(&period(&[BirForm::F0619E], Month::November), &ReminderDays::default());

        assert_eq!(
            dates(&drafts),
            [date!(2026 - 12 - 01), date!(2026 - 12 - 02), date!(2026 - 12 - 04)]
        );
    }

    #[test]
    fn d1_never_more_than_four_drafts_per_period() {
        let calendar = Calendar::built_in().unwrap();
        let overrides = DueOverrides::new();
        let reminders = ReminderDays::new(&[14, 2, 1]).unwrap();
        for subset in 1_u16..(1 << BirForm::ALL.len()) {
            let forms: Vec<_> = BirForm::ALL
                .into_iter()
                .enumerate()
                .filter(|(i, _)| subset & (1 << i) != 0)
                .map(|(_, f)| f)
                .collect();
            let client = ClientCalendar {
                calendar: &calendar,
                forms: &forms,
                overrides: &overrides,
                cut_off_offset_days: 0,
            };
            for period in client.upcoming(date!(2026 - 10 - 21)) {
                assert!(planned_drafts(&period, &reminders).len() <= MAX_DRAFTS_PER_PERIOD);
            }
        }
    }

    #[test]
    fn d1_a_schedule_allows_at_most_three_reminders_of_1_to_14_days() {
        assert_eq!(ReminderDays::new(&[7, 3, 2, 1]), Err(ScheduleError::TooManyReminders));
        assert_eq!(ReminderDays::new(&[15]), Err(ScheduleError::DayOutOfRange));
        assert_eq!(ReminderDays::new(&[0]), Err(ScheduleError::DayOutOfRange));
        assert_eq!(ReminderDays::new(&[1, 7, 7]).unwrap().days(), [7, 1]);
    }

    #[test]
    fn drafts_appear_at_7am_manila_time() {
        assert_eq!(appears_at(date!(2027 - 01 - 01)), datetime!(2026-12-31 23:00 UTC));
    }
}
