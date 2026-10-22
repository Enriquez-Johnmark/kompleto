//! The scheduler's payroll Periods (P1, P2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::engine::clock::Clock;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, Language, NewClient};
use kompleto::kompleto::payroll::{PAGIBIG_DEFAULT_DAY, PHILHEALTH_DEFAULT_DAY, PayCycle, PayrollSettings};
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;
use time::macros::date;
use time::{Date, Duration};

/// Runs the scheduler every day at 07:30 Manila from 22 Oct 2026 until
/// `until` for one employer with monthly payroll; returns each payroll
/// Draft's (period, Manila date it appeared, body).
async fn payroll_drafts(pool: &PgPool, until: Date) -> Vec<(String, Date, String)> {
    let clock = clock(); // 21 Oct 2026, 09:00 UTC
    clock.advance(Duration::hours(14) + Duration::minutes(30)); // 22 Oct, 07:30 Manila
    let santos = firm(pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock.clone());
    let employer = NewClient {
        forms: vec![BirForm::F1601C],
        language: Language::English,
        ..nena()
    };
    let client = clients.create(santos, &employer).await.unwrap();
    let settings = PayrollSettings {
        cycle: PayCycle::Monthly,
        philhealth_due_day: PHILHEALTH_DEFAULT_DAY,
        pagibig_due_day: PAGIBIG_DEFAULT_DAY,
    };
    clients.set_payroll(santos, client, Some(settings)).await.unwrap();
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(pool, clock.clone())).unwrap();
    while clock.now().date() < until {
        scheduler.run_once().await.unwrap();
        clock.advance(Duration::days(1));
    }
    sqlx::query_as::<_, (String, time::OffsetDateTime, String)>(
        "select s.period, m.created_at, m.body_text from scheduled_drafts s
         join outbox_messages m on m.id = s.message_id
         where s.period like 'payroll-%' or s.period like 'remit-%'
         order by m.created_at",
    )
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .map(|(period, at, body)| (period, at.to_offset(time::macros::offset!(+8)).date(), body))
    .collect()
}

#[sqlx::test]
async fn p1_payroll_documents_are_asked_for_before_payday(pool: PgPool) {
    let drafts = payroll_drafts(&pool, date!(2026 - 10 - 30)).await;

    // Payday 31 Oct: first request on the 23rd, Cut-off the 28th, so
    // reminders 3 and 1 days before it.
    let inputs: Vec<(&str, Date)> = drafts
        .iter()
        .filter(|(period, ..)| period.starts_with("payroll-"))
        .map(|(period, on, _)| (period.as_str(), *on))
        .collect();
    assert_eq!(
        inputs,
        [
            ("payroll-2026-10-31", date!(2026 - 10 - 23)),
            ("payroll-2026-10-31", date!(2026 - 10 - 25)),
            ("payroll-2026-10-31", date!(2026 - 10 - 27)),
        ]
    );
    let body = &drafts.iter().find(|(p, ..)| p.starts_with("payroll-")).unwrap().2;
    assert!(body.contains("Timesheets or DTRs"), "{body}");
    assert!(body.contains("payroll on Sat 31 Oct 2026"), "{body}");
    assert!(!body.contains("BIR"), "{body}");
}

#[sqlx::test]
async fn p2_contribution_receipts_are_asked_for_once_each_due_date_has_passed(pool: PgPool) {
    let drafts = payroll_drafts(&pool, date!(2026 - 11 - 08)).await;

    let receipts: Vec<&(String, Date, String)> = drafts.iter().filter(|(p, ..)| p == "remit-2026-09").collect();
    // PhilHealth was due 20 Oct; SSS and Pag-IBIG on 31 Oct, moved to 3 Nov.
    let dates: Vec<Date> = receipts.iter().map(|(_, on, _)| *on).collect();
    assert_eq!(
        dates,
        [date!(2026 - 10 - 22), date!(2026 - 11 - 03), date!(2026 - 11 - 07)]
    );
    let first = &receipts[0].2;
    assert!(first.contains("PhilHealth payment receipt"), "{first}");
    assert!(!first.contains("SSS payment receipt"), "{first}");
    let last = &receipts[2].2;
    assert!(
        last.contains("SSS payment receipt") && last.contains("Pag-IBIG payment receipt"),
        "{last}"
    );
}
