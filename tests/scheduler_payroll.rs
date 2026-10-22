//! The scheduler's payroll Periods (P1, P2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::engine::clock::Clock;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, Language, NewClient};
use kompleto::kompleto::payroll::{PayCycle, PayrollChoice};
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
    let monthly = PayrollChoice {
        cycle: Some(PayCycle::Monthly),
        ..PayrollChoice::default()
    };
    clients.set_payroll(santos, client, monthly).await.unwrap();
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
async fn p2_contribution_receipts_are_asked_for_after_each_due_date(pool: PgPool) {
    let drafts = payroll_drafts(&pool, date!(2026 - 11 - 08)).await;

    // PhilHealth was due Tue 20 Oct; SSS and Pag-IBIG on Sat 31 Oct, moved
    // to Tue 3 Nov past All Saints' and All Souls' Days.
    let receipts: Vec<(&str, Date)> = drafts
        .iter()
        .filter(|(p, ..)| p.starts_with("remit-2026-09"))
        .map(|(p, on, _)| (p.as_str(), *on))
        .collect();
    assert_eq!(
        receipts,
        [
            ("remit-2026-09-philhealth", date!(2026 - 10 - 22)),
            ("remit-2026-09-philhealth", date!(2026 - 10 - 24)),
            ("remit-2026-09-philhealth", date!(2026 - 10 - 26)),
            ("remit-2026-09-sss-pagibig", date!(2026 - 11 - 04)),
            ("remit-2026-09-sss-pagibig", date!(2026 - 11 - 07)),
        ]
    );
    let body = |period: &str| &drafts.iter().find(|(p, ..)| p == period).unwrap().2;
    let philhealth = body("remit-2026-09-philhealth");
    assert!(philhealth.contains("PhilHealth payment receipt"), "{philhealth}");
    assert!(!philhealth.contains("SSS payment receipt"), "{philhealth}");
    let later = body("remit-2026-09-sss-pagibig");
    assert!(
        later.contains("SSS payment receipt") && later.contains("Pag-IBIG payment receipt"),
        "{later}"
    );
}
