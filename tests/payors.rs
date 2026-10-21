//! Seam 4 (M5): each Client's Payor list (H2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::clients::{ClientError, Clients};
use kompleto::kompleto::payors::Payors;
use kompleto::kompleto::reminders::Scheduler;
use sqlx::PgPool;

#[sqlx::test]
async fn h2_a_client_lists_its_expected_payors_by_name(pool: PgPool) {
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let client = Clients::new(pool.clone(), clock())
        .create(santos, &nena())
        .await
        .unwrap();
    let payors = Payors::new(pool.clone(), clock());

    let sm = payors.add(santos, client, "  SM Supermalls ").await.unwrap();
    payors.add(santos, client, "Jollibee Foods Corp").await.unwrap();

    let names: Vec<String> = payors
        .list(santos, client)
        .await
        .unwrap()
        .into_iter()
        .map(|payor| payor.name)
        .collect();
    assert_eq!(names, ["Jollibee Foods Corp", "SM Supermalls"]);
    for (case, name) in [
        ("empty", " "),
        ("a link", "www.pay-here.ph"),
        ("too long", &"x".repeat(101)),
    ] {
        let refused = payors.add(santos, client, name).await;
        assert!(matches!(refused, Err(ClientError::Invalid(_))), "{case}: {refused:?}");
    }
    payors.add(santos, client, "Peñafrancia Trading").await.unwrap();
    for duplicate in ["sm supermalls", "PEÑAFRANCIA TRADING"] {
        let refused = payors.add(santos, client, duplicate).await;
        assert!(
            matches!(refused, Err(ClientError::DuplicateName(_))),
            "{duplicate}: {refused:?}"
        );
    }
    let reserved = payors.add(santos, client, "Another Business").await;
    assert!(
        matches!(reserved, Err(ClientError::Invalid(_))),
        "the upload page's own choice"
    );
    assert!(matches!(
        payors.add(reyes, client, "Puregold").await,
        Err(ClientError::NotFound)
    ));
    assert!(payors.list(reyes, client).await.unwrap().is_empty());
    assert!(matches!(
        payors.remove(reyes, client, sm).await,
        Err(ClientError::NotFound)
    ));
    payors.remove(santos, client, sm).await.unwrap();
    assert_eq!(payors.list(santos, client).await.unwrap().len(), 2);
}

#[sqlx::test]
async fn h2_a_quarter_end_reminder_asks_for_each_payors_2307_by_name(pool: PgPool) {
    let clock = clock();
    clock.advance(time::Duration::days(84)); // 13 Jan 2027: Q4's 2551Q reminders start
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let payors = Payors::new(pool.clone(), clock.clone());
    payors.add(santos, client, "Jollibee Foods Corp").await.unwrap();
    payors.add(santos, client, "SM Supermalls").await.unwrap();

    Scheduler::new(pool.clone(), clock.clone(), common::upload_links(&pool, clock.clone()))
        .unwrap()
        .run_once()
        .await
        .unwrap();

    let body: String = sqlx::query_scalar("select body_text from outbox_messages")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(body.contains("- BIR Form 2307 from Jollibee Foods Corp"), "{body}");
    assert!(body.contains("- BIR Form 2307 from SM Supermalls"), "{body}");
    assert!(!body.contains("2307s received from customers"), "{body}");
}
