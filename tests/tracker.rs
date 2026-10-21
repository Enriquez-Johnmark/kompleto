//! Seam 4 (M5): the quarterly 2307 tracker (H2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::checklist::{ANOTHER_PAYOR_2307, payor_2307};
use kompleto::kompleto::clients::{ClientId, Clients};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::payors::Payors;
use kompleto::kompleto::tracker::{PayorStatus, quarter};
use kompleto::kompleto::uploads::{Label, Uploads};
use sqlx::PgPool;
use time::macros::date;

/// `firm`'s Client Nena uploads a 2307 for `period`, labelled `document`
/// from `payor`; returns the Upload's ID.
async fn upload_2307(
    pool: &PgPool,
    firm: FirmId,
    client: ClientId,
    period: &str,
    document: &str,
    payor: &str,
) -> kompleto::engine::ids::FileId {
    let clock = clock();
    let links = common::upload_links(pool, clock.clone());
    let url = links
        .for_period(firm, client, period, date!(2026 - 10 - 26))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    Uploads::new(pool.clone(), clock.clone(), common::files(pool, clock))
        .add(
            &link,
            &[document.to_owned()],
            Label::Owed {
                document: document.to_owned(),
                payor: Some(payor.to_owned()),
            },
            b"%PDF-1.7",
        )
        .await
        .unwrap()
}

#[sqlx::test]
async fn h2_the_tracker_shows_each_payor_received_uploaded_or_missing_and_unlisted_ones_as_unexpected(pool: PgPool) {
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let clients = Clients::new(pool.clone(), clock());
    let client = clients.create(santos, &nena()).await.unwrap();
    let other = clients.create(reyes, &nena()).await.unwrap();
    let payors = Payors::new(pool.clone(), clock());
    for name in ["Jollibee Foods Corp", "SM Supermalls", "Puregold Price Club"] {
        payors.add(santos, client, name).await.unwrap();
    }
    let jollibee = payor_2307("Jollibee Foods Corp");
    let received = upload_2307(&pool, santos, client, "2026-09", &jollibee, "Jollibee Foods Corp").await;
    Uploads::new(pool.clone(), clock(), common::files(&pool, clock()))
        .mark_received(santos, client, received, "ana@santos.test")
        .await
        .unwrap();
    upload_2307(&pool, santos, client, "2026-09", ANOTHER_PAYOR_2307, "sm supermalls").await;
    upload_2307(&pool, santos, client, "2026-09", ANOTHER_PAYOR_2307, "Robinsons Land").await;
    // Another quarter's, and another Firm's, don't count.
    upload_2307(
        &pool,
        santos,
        client,
        "2026-06",
        &payor_2307("Puregold Price Club"),
        "Puregold Price Club",
    )
    .await;
    upload_2307(&pool, reyes, other, "2026-09", ANOTHER_PAYOR_2307, "Ayala Land").await;

    let tracked = quarter(&pool, santos, 2026, 3).await.unwrap();

    assert_eq!(tracked.len(), 1, "{tracked:?}");
    let row = &tracked[0];
    assert_eq!(row.id, client);
    let statuses: Vec<(&str, PayorStatus)> = row
        .payors
        .iter()
        .map(|payor| (payor.name.as_str(), payor.status))
        .collect();
    assert_eq!(
        statuses,
        [
            ("Jollibee Foods Corp", PayorStatus::Received),
            ("Puregold Price Club", PayorStatus::Missing),
            ("SM Supermalls", PayorStatus::Uploaded),
        ]
    );
    assert_eq!(row.unexpected, ["Robinsons Land"]);
    assert!(quarter(&pool, reyes, 2026, 3).await.unwrap()[0].payors.is_empty());
}
