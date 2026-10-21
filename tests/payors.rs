//! Seam 4 (M5): each Client's Payor list (H2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::clients::{ClientError, Clients};
use kompleto::kompleto::payors::Payors;
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
    assert!(matches!(
        payors.add(santos, client, "sm supermalls").await,
        Err(ClientError::DuplicateName(_))
    ));
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
    assert_eq!(payors.list(santos, client).await.unwrap().len(), 1);
}
