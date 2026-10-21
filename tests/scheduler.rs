//! Seam 6: the scheduler with the test clock (D1). The clock steps a day at
//! a time; the scheduler runs (twice, to show reruns create nothing) and
//! every Draft it creates is recorded.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::collections::BTreeMap;

use common::{clock, firm, nena};
use kompleto::engine::clock::Clock;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{ClientId, Clients, Language, NewClient, ScheduleSettings};
use kompleto::kompleto::reminders::Scheduler;
use kompleto::kompleto::schedule::ReminderDays;
use sqlx::PgPool;
use time::macros::date;
use time::{Date, Duration};

fn employer() -> NewClient {
    NewClient {
        forms: vec![BirForm::F1601C],
        language: Language::English,
        ..nena()
    }
}

/// Runs the scheduler every day at 07:30 Manila until `until`, returning
/// each Draft's (period, Manila date it appeared).
async fn run_daily(
    pool: &PgPool,
    clock: &std::sync::Arc<kompleto::engine::clock::FixedClock>,
    until: Date,
) -> Vec<(String, Date)> {
    let scheduler = Scheduler::new(pool.clone(), clock.clone()).unwrap();
    while clock.now().date() < until {
        scheduler.run_once().await.unwrap();
        scheduler.run_once().await.unwrap();
        clock.advance(Duration::days(1));
    }
    sqlx::query_as::<_, (String, time::OffsetDateTime)>(
        "select s.period, m.created_at from scheduled_drafts s join outbox_messages m on m.id = s.message_id
         order by m.created_at",
    )
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .map(|(period, at)| (period, at.to_offset(time::macros::offset!(+8)).date()))
    .collect()
}

#[sqlx::test]
async fn d1_drafts_appear_on_the_schedule_and_never_more_than_four_per_period(pool: PgPool) {
    let clock = clock(); // 21 Oct 2026, 09:00 UTC
    clock.advance(Duration::hours(14) + Duration::minutes(30)); // 22 Oct, 07:30 Manila
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();

    let drafts = run_daily(&pool, &clock, date!(2027 - 01 - 31)).await;

    // 1601-C is due the 10th (15 Jan for December): cut-offs 5 Nov, 5 Dec, 10 Jan.
    let expected = [
        ("2026-10", date!(2026 - 11 - 01)),
        ("2026-10", date!(2026 - 11 - 02)),
        ("2026-10", date!(2026 - 11 - 04)),
        ("2026-11", date!(2026 - 12 - 01)),
        ("2026-11", date!(2026 - 12 - 02)),
        ("2026-11", date!(2026 - 12 - 04)),
        ("2026-12", date!(2027 - 01 - 01)),
        ("2026-12", date!(2027 - 01 - 03)),
        ("2026-12", date!(2027 - 01 - 07)),
        ("2026-12", date!(2027 - 01 - 09)),
    ];
    let drafts: Vec<_> = drafts.iter().map(|(p, d)| (p.as_str(), *d)).collect();
    assert_eq!(drafts, expected);
    let mut per_period = BTreeMap::new();
    for (period, _) in &drafts {
        *per_period.entry(*period).or_insert(0) += 1;
    }
    assert!(per_period.values().all(|&n| n <= 4), "{per_period:?}");
    let statuses: Vec<String> = sqlx::query_scalar("select distinct status from outbox_messages")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(statuses, ["draft"], "the scheduler never approves anything");
}

#[sqlx::test]
async fn d1_a_client_added_late_gets_only_the_latest_due_draft(pool: PgPool) {
    let clock = clock();
    clock.advance(Duration::days(43) + Duration::hours(1)); // 3 Dec 2026, 18:00 Manila
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();

    let drafts = run_daily(&pool, &clock, date!(2026 - 12 - 06)).await;

    // November's first request (1 Dec) and 3-day reminder (2 Dec) were
    // missed; only the latest due is sent, then the 1-day reminder.
    assert_eq!(
        drafts,
        [
            ("2026-11".to_owned(), date!(2026 - 12 - 03)),
            ("2026-11".to_owned(), date!(2026 - 12 - 04))
        ]
    );
}

#[sqlx::test]
async fn d1_paused_clients_get_no_drafts(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock.clone());
    let id: ClientId = clients.create(santos, &employer()).await.unwrap();
    let paused = ScheduleSettings {
        cut_off_offset_days: None,
        reminder_days: ReminderDays::default(),
        paused: true,
    };
    clients.set_schedule(santos, id, &paused).await.unwrap();

    assert!(run_daily(&pool, &clock, date!(2026 - 12 - 10)).await.is_empty());
}

#[sqlx::test]
async fn d1_a_draft_lists_the_owed_documents_and_cut_off_in_the_clients_language(pool: PgPool) {
    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov 2026, 17:00 Manila
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(
            santos,
            &NewClient {
                language: Language::Taglish,
                ..employer()
            },
        )
        .await
        .unwrap();

    Scheduler::new(pool.clone(), clock.clone())
        .unwrap()
        .run_once()
        .await
        .unwrap();

    let (to, from, subject, body): (String, String, String, String) =
        sqlx::query_as("select to_address, from_name, subject, body_text from outbox_messages")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        (to.as_str(), from.as_str()),
        ("nena@example.test", "Santos Bookkeeping")
    );
    assert!(subject.contains("October 2026"), "{subject}");
    for expected in [
        "Hi Nena",
        "bago Thu 5 Nov 2026",
        "- Payroll register and SSS, PhilHealth and Pag-IBIG reports",
        "Salamat po",
    ] {
        assert!(body.contains(expected), "missing {expected:?} in {body}");
    }
}
