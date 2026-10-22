//! Seam 1 (M5): Replies from sample messages, against Postgres (G1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use common::clamd::{Behaviour, FakeClamd, nobody_listening};
use common::{clock, firm, nena};
use kompleto::engine::clock::FixedClock;
use kompleto::engine::inbound::parse;
use kompleto::engine::scan::Clamd;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::reminders::Scheduler;
use kompleto::kompleto::replies::{Received, Replies, ReplyError};
use kompleto::kompleto::uploads::{Label, Uploads};
use sqlx::PgPool;
use time::Duration;

fn employer() -> NewClient {
    NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    }
}

/// A Firm whose employer Client has October's first Reminder waiting (1 Nov
/// 2026); returns its reply address.
async fn reminded(pool: &PgPool) -> (FirmId, Arc<FixedClock>, String) {
    let clock = clock();
    clock.advance(Duration::days(11));
    let santos = firm(pool, "Santos Bookkeeping").await;
    Clients::new(pool.clone(), clock.clone())
        .create(santos, &employer())
        .await
        .unwrap();
    Scheduler::new(pool.clone(), clock.clone(), common::upload_links(pool, clock.clone()))
        .unwrap()
        .run_once()
        .await
        .unwrap();
    let reply_to: String = sqlx::query_scalar("select reply_to from outbox_messages")
        .fetch_one(pool)
        .await
        .unwrap();
    (santos, clock, reply_to)
}

fn replies(pool: &PgPool, clock: Arc<FixedClock>) -> Replies {
    replies_scanned_by(pool, clock, None)
}

fn replies_scanned_by(pool: &PgPool, clock: Arc<FixedClock>, scanner: Option<Clamd>) -> Replies {
    let checks = common::kompleto_checks(pool, clock.clone());
    let uploads = Uploads::new(pool.clone(), clock.clone(), common::files(pool, clock.clone()));
    let uploads = match scanner {
        Some(clamd) => uploads.scanned_by(clamd),
        None => uploads,
    };
    Replies::new(
        pool.clone(),
        clock.clone(),
        common::upload_links(pool, clock),
        checks,
        "http://kompleto.test",
    )
    .with_uploads(uploads)
}

async fn status(pool: &PgPool) -> String {
    sqlx::query_scalar("select m.status from outbox_messages m join scheduled_drafts s on s.message_id = m.id")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test]
async fn g1_a_human_reply_stops_the_periods_reminders(pool: PgPool) {
    let (santos, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("human_reply", &reply_to)).unwrap();

    let received = replies(&pool, clock.clone()).receive(&email).await.unwrap();

    let Received::Human(id) = received else {
        panic!("{received:?}");
    };
    assert_eq!(status(&pool).await, "cancelled");
    let reply = replies(&pool, clock).get(santos, id).await.unwrap();
    assert_eq!(reply.period, "2026-10");
    assert!(reply.text.unwrap().starts_with("Hi po! Ipapadala ko po bukas"));
}

#[sqlx::test]
async fn g1_after_a_human_reply_no_later_reminder_is_drafted_for_the_period(pool: PgPool) {
    let (_, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("human_reply", &reply_to)).unwrap();
    replies(&pool, clock.clone()).receive(&email).await.unwrap();

    clock.advance(Duration::days(3)); // 4 Nov: past the 3- and 1-day reminders' days
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone())).unwrap();
    scheduler.run_once().await.unwrap();

    let drafts: i64 = sqlx::query_scalar("select count(*) from scheduled_drafts")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(drafts, 1, "only the request sent before the reply");
}

#[sqlx::test]
async fn j2_a_reply_is_logged_once(pool: PgPool) {
    let (santos, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("human_reply", &reply_to)).unwrap();

    replies(&pool, clock.clone()).receive(&email).await.unwrap();
    replies(&pool, clock).receive(&email).await.unwrap();

    let logged: Vec<String> =
        sqlx::query_scalar("select actor from audit_events where tenant_id = $1 and action = 'reply.received'")
            .bind(santos.as_uuid())
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(logged, ["system:replies"]);
}

#[sqlx::test]
async fn g1_an_automatic_reply_is_noted_but_reminders_continue(pool: PgPool) {
    let (_, clock, reply_to) = reminded(&pool).await;

    for name in ["outlook_ooo", "gmail_vacation"] {
        let email = parse(&common::inbound(name, &reply_to)).unwrap();
        let received = replies(&pool, clock.clone()).receive(&email).await.unwrap();
        assert!(matches!(received, Received::Automatic(_)), "{name}: {received:?}");
    }

    assert_eq!(status(&pool).await, "draft");
}

#[sqlx::test]
async fn g1_mail_with_a_forged_code_from_an_unknown_sender_keeps_nothing(pool: PgPool) {
    let (_, clock, reply_to) = reminded(&pool).await;
    let forged = reply_to.replacen("reply+", "reply+0", 1);
    let raw = String::from_utf8(common::inbound("human_reply", &forged))
        .unwrap()
        .replace("Nena@Example.TEST", "stranger@elsewhere.test");

    let received = replies(&pool, clock)
        .receive(&parse(raw.as_bytes()).unwrap())
        .await
        .unwrap();

    assert_eq!(received, Received::Unmatched);
    let kept: i64 = sqlx::query_scalar("select count(*) from replies")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(kept, 0);
    assert_eq!(status(&pool).await, "draft");
}

#[sqlx::test]
async fn g1_mail_without_a_code_matches_the_only_client_with_that_address(pool: PgPool) {
    let (santos, clock, _) = reminded(&pool).await;
    // The Reminder went out; Nena then writes to the plain sending address.
    sqlx::query("update outbox_messages set status = 'sent', approved_hash = '\\x00', sent_at = $1")
        .bind(clock.now_plus(Duration::hours(1)))
        .execute(&pool)
        .await
        .unwrap();
    let email = parse(&common::inbound("human_reply", "reminders@getkompleto.test")).unwrap();

    let first = replies(&pool, clock.clone()).receive(&email).await.unwrap();
    // A second Client with the same address makes the sender ambiguous.
    Clients::new(pool.clone(), clock.clone())
        .create(
            santos,
            &NewClient {
                business_name: "Nena's Other Store".to_owned(),
                ..employer()
            },
        )
        .await
        .unwrap();
    let second = replies(&pool, clock).receive(&email).await.unwrap();

    assert!(matches!(first, Received::Human(_)), "{first:?}");
    assert_eq!(second, Received::Unmatched);
}

#[sqlx::test]
async fn g1_mail_matched_only_by_its_sender_is_shown_but_reminders_continue(pool: PgPool) {
    let (santos, clock, _) = reminded(&pool).await;
    sqlx::query("update outbox_messages set status = 'sent', approved_hash = '\\x00', sent_at = $1")
        .bind(clock.now_plus(Duration::hours(1)))
        .execute(&pool)
        .await
        .unwrap();
    // Anyone can put the Contact's address in From, so it proves nothing.
    let email = parse(&common::inbound("with_attachments", "reminders@getkompleto.test")).unwrap();

    let Received::Human(id) = replies(&pool, clock.clone()).receive(&email).await.unwrap() else {
        panic!("not kept");
    };
    clock.advance(Duration::days(3)); // 4 Nov: the next Reminder's day
    Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone()))
        .unwrap()
        .run_once()
        .await
        .unwrap();

    let reply = replies(&pool, clock).get(santos, id).await.unwrap();
    assert!(!reply.signed);
    assert_eq!(reply.attachments, 0, "nothing kept on an unproven sender's word");
    let drafts: i64 = sqlx::query_scalar("select count(*) from scheduled_drafts")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(drafts, 2, "Reminders continue");
    let notice: String = sqlx::query_scalar("select body_text from outbox_messages where reply_to is null")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(notice.contains("reminders continue"), "{notice}");
}

#[sqlx::test]
async fn g1_the_firm_is_told_of_a_reply_with_a_link_not_a_copy(pool: PgPool) {
    let (_, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("human_reply", &reply_to)).unwrap();

    let Received::Human(id) = replies(&pool, clock).receive(&email).await.unwrap() else {
        panic!("not a human reply");
    };

    let (to, subject, body, status): (String, String, String, String) = sqlx::query_as(
        "select to_address, subject, body_text, status from outbox_messages where to_address <> 'nena@example.test'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(to, "santos.bookkeeping@firm.test");
    assert_eq!(subject, "Nena (Nena's Sari-Sari Store) replied about October 2026");
    assert!(body.contains(&format!("http://kompleto.test/replies/{id}")), "{body}");
    assert!(
        !body.contains("Ipapadala"),
        "the Contact's text is never copied: {body}"
    );
    assert_eq!(status, "approved", "on its way within the sender's next drain");
}

#[sqlx::test]
async fn g1_photo_and_pdf_attachments_are_kept_as_uploads_and_the_rest_dropped(pool: PgPool) {
    let (santos, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("with_attachments", &reply_to)).unwrap();

    let Received::Human(id) = replies(&pool, clock.clone()).receive(&email).await.unwrap() else {
        panic!("not a human reply");
    };

    let reply = replies(&pool, clock.clone()).get(santos, id).await.unwrap();
    assert_eq!(reply.attachments, 1, "the JPEG kept, the .exe dropped");
    let client: uuid::Uuid = sqlx::query_scalar("select id from clients")
        .fetch_one(&pool)
        .await
        .unwrap();
    let kept = Uploads::new(pool.clone(), clock.clone(), common::files(&pool, clock))
        .for_client(santos, client.into())
        .await
        .unwrap();
    assert_eq!(kept.len(), 1);
    assert_eq!(
        kept[0].label,
        Label::SomethingElse {
            note: "Sent by email".to_owned()
        }
    );
}

#[sqlx::test]
async fn g1_an_attachment_with_a_virus_is_not_kept(pool: PgPool) {
    let clamd = FakeClamd::start(Behaviour::Scans).await;
    let (santos, clock, reply_to) = reminded(&pool).await;
    // The photo, now carrying the fake clamd's test virus.
    let infected = String::from_utf8(common::inbound("with_attachments", &reply_to))
        .unwrap()
        .replace("/9j/4CBhIHBob3RvIG9mIGEgcmVjZWlwdA==", "/9j/4CBFSUNBUg==");
    let email = parse(infected.as_bytes()).unwrap();
    let replies = replies_scanned_by(&pool, clock, Some(Clamd::new(clamd.addr())));

    let Received::Human(id) = replies.receive(&email).await.unwrap() else {
        panic!("not a human reply");
    };

    assert_eq!(replies.get(santos, id).await.unwrap().attachments, 0);
    assert_eq!(
        clamd.streams.lock().unwrap().len(),
        1,
        "only the photo is scanned; the program was refused before"
    );
}

#[sqlx::test]
async fn g1_a_reply_with_attachments_waits_while_they_cant_be_scanned(pool: PgPool) {
    let (santos, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("with_attachments", &reply_to)).unwrap();
    let down = replies_scanned_by(&pool, clock.clone(), Some(Clamd::new(nobody_listening().await)));
    let clamd = FakeClamd::start(Behaviour::Scans).await;
    let up = replies_scanned_by(&pool, clock, Some(Clamd::new(clamd.addr())));

    // Refused whole, so the provider delivers it again later.
    let refused = down.receive(&email).await;
    let kept: i64 = sqlx::query_scalar("select count(*) from replies")
        .fetch_one(&pool)
        .await
        .unwrap();
    let Received::Human(id) = up.receive(&email).await.unwrap() else {
        panic!("not a human reply");
    };

    assert!(matches!(refused, Err(ReplyError::NotScanned)), "{refused:?}");
    assert_eq!(kept, 0);
    assert_eq!(up.get(santos, id).await.unwrap().attachments, 1);
}

#[sqlx::test]
async fn g1_the_same_email_delivered_twice_is_kept_and_announced_once(pool: PgPool) {
    let (_, clock, reply_to) = reminded(&pool).await;

    for name in ["with_attachments", "human_reply"] {
        let email = parse(&common::inbound(name, &reply_to)).unwrap();
        let first = replies(&pool, clock.clone()).receive(&email).await.unwrap();
        let again = replies(&pool, clock.clone()).receive(&email).await.unwrap();
        assert_eq!(first, again, "{name}");
    }

    let count = |sql: &'static str| {
        let pool = pool.clone();
        async move { sqlx::query_scalar::<_, i64>(sql).fetch_one(&pool).await.unwrap() }
    };
    assert_eq!(count("select count(*) from replies").await, 2);
    assert_eq!(count("select count(*) from uploads").await, 1);
    assert_eq!(
        count("select count(*) from outbox_messages where reply_to is null").await,
        2,
        "one notice each"
    );
}

#[sqlx::test]
async fn g1_a_replys_text_is_deleted_after_90_days_and_the_reply_kept(pool: PgPool) {
    let (santos, clock, reply_to) = reminded(&pool).await;
    let email = parse(&common::inbound("human_reply", &reply_to)).unwrap();
    let Received::Human(id) = replies(&pool, clock.clone()).receive(&email).await.unwrap() else {
        panic!("not a human reply");
    };

    let early =
        kompleto::kompleto::replies::forget_old_text(&pool, clock.now_plus(Duration::days(90) - Duration::minutes(1)))
            .await
            .unwrap();
    let due = kompleto::kompleto::replies::forget_old_text(&pool, clock.now_plus(Duration::days(90)))
        .await
        .unwrap();

    assert_eq!((early, due), (0, 1));
    let reply = replies(&pool, clock).get(santos, id).await.unwrap();
    assert_eq!((reply.subject, reply.text), (None, None));
    assert_eq!(reply.period, "2026-10", "the reply itself stays");
}
