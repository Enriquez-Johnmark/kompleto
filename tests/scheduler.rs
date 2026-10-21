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
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(pool, clock.clone())).unwrap();
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
    // Old Drafts have expired by now (D4); none was ever approved.
    assert!(
        statuses.iter().all(|s| s == "draft" || s == "expired"),
        "the scheduler never approves anything: {statuses:?}"
    );
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

    Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone()))
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

#[sqlx::test]
async fn d1_one_clients_failed_draft_neither_stops_the_others_nor_uses_up_its_slot(pool: PgPool) {
    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov 2026, 17:00 Manila: October's first request is due
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock.clone());
    let broken = clients
        .create(
            santos,
            &NewClient {
                business_name: "A Broken".to_owned(),
                ..employer()
            },
        )
        .await
        .unwrap();
    clients
        .create(
            santos,
            &NewClient {
                business_name: "B Fine".to_owned(),
                ..employer()
            },
        )
        .await
        .unwrap();
    // An address the outbox refuses, as if the row had been damaged.
    sqlx::query("update clients set email = 'no-at-sign' where id = $1")
        .bind(broken.as_uuid())
        .execute(&pool)
        .await
        .unwrap();
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone())).unwrap();

    assert_eq!(scheduler.run_once().await.unwrap(), 1);
    let claims: i64 = sqlx::query_scalar("select count(*) from scheduled_drafts where client_id = $1")
        .bind(broken.as_uuid())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(claims, 0, "a failed draft leaves no claim behind");

    sqlx::query("update clients set email = 'nena@example.test' where id = $1")
        .bind(broken.as_uuid())
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(scheduler.run_once().await.unwrap(), 1, "the next run creates it");
}

#[sqlx::test]
async fn d5_the_scheduler_drafts_with_the_model_and_labels_each_draft(pool: PgPool) {
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let server = MockServer::start().await;
    let wording =
        json!({"subject": "Paalala po", "opening": "Hi Nena! Kailangan pa po namin ito:", "closing": "Salamat po!"});
    Mock::given(wiremock::matchers::method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "type": "message", "model": "claude-haiku-5-5",
            "content": [{"type": "text", "text": wording.to_string()}],
            "stop_reason": "end_turn", "usage": {"input_tokens": 10, "output_tokens": 10}
        })))
        .mount(&server)
        .await;
    let model = kompleto::engine::llm::AnthropicClient::new(
        &server.uri(),
        secrecy::SecretString::from("k"),
        "claude-haiku-5-5",
        std::time::Duration::from_secs(2),
    )
    .unwrap();
    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov 2026: October's first request is due
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();

    Scheduler::with_model(
        pool.clone(),
        clock.clone(),
        common::upload_links(&pool, clock.clone()),
        Some(model),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();

    let (source, body): (String, String) = sqlx::query_as(
        "select s.source, m.body_text from scheduled_drafts s join outbox_messages m on m.id = s.message_id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(source, "ai");
    assert!(
        body.starts_with("Hi Nena! Kailangan pa po namin ito:") && body.contains("Santos Bookkeeping"),
        "{body}"
    );
}

#[sqlx::test]
async fn d4_a_reminder_draft_expires_at_the_end_of_its_cut_off_day(pool: PgPool) {
    use kompleto::engine::outbox::{MessageStatus, Outbox, OutboxError};

    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov 2026, 17:00 Manila: October's first request
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone())).unwrap();
    scheduler.run_once().await.unwrap();
    let id: uuid::Uuid = sqlx::query_scalar("select message_id from scheduled_drafts")
        .fetch_one(&pool)
        .await
        .unwrap();
    let outbox = Outbox::new(pool.clone(), clock.clone());
    let seen = outbox.get(santos, id.into()).await.unwrap().content_hash();
    let checks = common::kompleto_checks(&pool, clock.clone());

    // October's cut-off is Thu 5 Nov; 23:59 Manila is 15:59 UTC.
    clock.advance(Duration::days(4) + Duration::hours(6) + Duration::minutes(59)); // 5 Nov, 23:59 Manila
    scheduler.run_once().await.unwrap();
    assert_eq!(
        outbox.get(santos, id.into()).await.unwrap().status,
        MessageStatus::Draft
    );

    clock.advance(Duration::minutes(1)); // 6 Nov, 00:00 Manila
    let late = outbox
        .approve(santos, id.into(), "ana@santos.test", &seen, &checks)
        .await;
    assert!(matches!(late, Err(OutboxError::Expired)), "{late:?}");
}

#[sqlx::test]
async fn d4_the_hourly_run_marks_overdue_drafts_expired(pool: PgPool) {
    let clock = clock();
    clock.advance(Duration::days(11));
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone())).unwrap();
    scheduler.run_once().await.unwrap();

    clock.advance(Duration::days(5)); // 6 Nov, past the cut-off day
    scheduler.run_once().await.unwrap();

    let statuses: Vec<String> = sqlx::query_scalar("select status from outbox_messages order by created_at")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(statuses[0], "expired");
}

#[sqlx::test]
async fn d1_a_period_with_its_four_drafts_costs_no_more_model_calls(pool: PgPool) {
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;
    let model = kompleto::engine::llm::AnthropicClient::new(
        &server.uri(),
        secrecy::SecretString::from("k"),
        "claude-haiku-5-5",
        std::time::Duration::from_secs(2),
    )
    .unwrap();
    let clock = clock();
    clock.advance(Duration::days(11)); // 1 Nov 2026: October's first request is due
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    // As if the reminder days changed after October already had four Drafts.
    for (ordinal, slot) in ["before-9", "before-8", "before-6", "before-5"].into_iter().enumerate() {
        sqlx::query(
            "insert into scheduled_drafts (firm_id, client_id, period, slot, ordinal, message_id, created_at)
             values ($1, $2, '2026-10', $3, $4, $5, now())",
        )
        .bind(santos.as_uuid())
        .bind(client.as_uuid())
        .bind(slot)
        .bind(i16::try_from(ordinal).unwrap())
        .bind(uuid::Uuid::now_v7())
        .execute(&pool)
        .await
        .unwrap();
    }

    Scheduler::with_model(
        pool.clone(),
        clock.clone(),
        common::upload_links(&pool, clock.clone()),
        Some(model),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();

    assert!(
        server.received_requests().await.unwrap().is_empty(),
        "no model call for a full Period"
    );
}

#[sqlx::test]
async fn f2_every_reminder_for_a_period_carries_the_clients_one_upload_link(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();

    run_daily(&pool, &clock, date!(2026 - 11 - 06)).await;

    let bodies: Vec<String> = sqlx::query_scalar(
        "select m.body_text from scheduled_drafts s join outbox_messages m on m.id = s.message_id
         where s.period = '2026-10' order by m.created_at",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let link = |body: &str| {
        body.split_whitespace()
            .find(|word| word.starts_with("http://kompleto.test/u/"))
            .map(str::to_owned)
    };
    assert!(bodies.len() > 1, "{bodies:?}");
    let first = link(&bodies[0]).unwrap();
    assert!(bodies.iter().all(|body| link(body).as_deref() == Some(first.as_str())));
    let open = common::upload_links(&pool, clock.clone())
        .open(first.rsplit('/').next().unwrap())
        .await
        .unwrap();
    assert_eq!((open.firm, open.period.as_str()), (santos, "2026-10"));
    let reply_to: Vec<Option<String>> = sqlx::query_scalar(
        "select m.reply_to from scheduled_drafts s join outbox_messages m on m.id = s.message_id where s.period = '2026-10'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert!(
        reply_to.iter().all(|r| r
            .as_deref()
            .is_some_and(|r| r.starts_with("reply+") && Some(r) == reply_to[0].as_deref())),
        "every reminder for the Period replies to the same signed address: {reply_to:?}"
    );
}

#[sqlx::test]
async fn g2_reminders_ask_only_for_what_is_missing_and_stop_once_all_is_uploaded(pool: PgPool) {
    use kompleto::kompleto::checklist::owed_documents;
    use kompleto::kompleto::uploads::{Label, Uploads};

    let clock = clock(); // 21 Oct 2026, 17:00 Manila
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    let owed = owed_documents(&[BirForm::F1601C], time::Month::October, &Default::default());
    assert!(owed.len() >= 2, "{owed:?}");
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone())).unwrap();
    let drafts = || async {
        sqlx::query_as::<_, (String, String)>(
            "select m.body_text, m.status from scheduled_drafts s join outbox_messages m on m.id = s.message_id
             order by m.created_at",
        )
        .fetch_all(&pool)
        .await
        .unwrap()
    };
    clock.advance(Duration::days(11)); // 1 Nov: October's first request
    scheduler.run_once().await.unwrap();
    let first = drafts().await;
    let token = first[0]
        .0
        .split_whitespace()
        .find_map(|word| word.strip_prefix("http://kompleto.test/u/"))
        .unwrap()
        .to_owned();
    let link = common::upload_links(&pool, clock.clone()).open(&token).await.unwrap();
    let uploads = Uploads::new(pool.clone(), clock.clone(), common::files(&pool, clock.clone()));
    let upload = |document: &String| Label::Owed {
        document: document.clone(),
        payor: Some("Jollibee Foods Corp".to_owned()),
    };
    uploads.add(&link, &owed, upload(&owed[0]), b"%PDF-1.7").await.unwrap();

    clock.advance(Duration::days(1)); // 2 Nov: 3 days before the Cut-off
    scheduler.run_once().await.unwrap();
    let second = &drafts().await[1].0;
    assert!(!second.contains(&owed[0]), "already uploaded: {second}");
    assert!(second.contains(&owed[1]), "{second}");

    for document in &owed[1..] {
        uploads.add(&link, &owed, upload(document), b"%PDF-1.7").await.unwrap();
    }
    clock.advance(Duration::days(2)); // 4 Nov: the day before the Cut-off
    scheduler.run_once().await.unwrap();

    let after = drafts().await;
    assert_eq!(after.len(), 2, "no reminder once everything has arrived");
    assert!(after.iter().all(|(_, status)| status == "cancelled"), "{after:?}");
}

#[sqlx::test]
async fn g2_a_draft_still_waiting_after_everything_arrived_is_cancelled_by_the_next_run(pool: PgPool) {
    use kompleto::kompleto::checklist::owed_documents;
    use kompleto::kompleto::uploads::{Label, Uploads};

    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone())).unwrap();
    clock.advance(Duration::days(11)); // 1 Nov: October's first request
    scheduler.run_once().await.unwrap();
    let body: String = sqlx::query_scalar("select body_text from outbox_messages")
        .fetch_one(&pool)
        .await
        .unwrap();
    let token = body
        .split_whitespace()
        .find_map(|word| word.strip_prefix("http://kompleto.test/u/"))
        .unwrap();
    let link = common::upload_links(&pool, clock.clone()).open(token).await.unwrap();
    let owed = owed_documents(&[BirForm::F1601C], time::Month::October, &Default::default());
    // Everything arrives, but the cancellation that should follow never ran.
    let uploads = Uploads::new(pool.clone(), clock.clone(), common::files(&pool, clock.clone()));
    for document in &owed {
        let label = Label::Owed {
            document: document.clone(),
            payor: None,
        };
        uploads.add(&link, &owed, label, b"%PDF-1.7").await.unwrap();
    }

    scheduler.run_once().await.unwrap(); // the same day: this slot already has its Draft

    let status: String = sqlx::query_scalar("select status from outbox_messages")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "cancelled");
}
