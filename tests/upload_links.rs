//! Seam 2 (M4): Upload links (F2), with the test clock.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, master_key, nena};
use kompleto::engine::files::MasterKey;
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::upload_links::{LinkError, UploadLinks};
use sqlx::PgPool;
use time::macros::{date, datetime};

const BASE_URL: &str = "http://kompleto.test";

fn token(url: &str) -> &str {
    url.rsplit('/').next().unwrap()
}

#[sqlx::test]
async fn f2_each_client_and_period_has_one_link_that_names_only_them(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock.clone());
    let nena_id = clients.create(santos, &nena()).await.unwrap();
    let links = UploadLinks::new(pool.clone(), clock, &master_key(), BASE_URL);

    let october = links
        .for_period(santos, nena_id, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();
    let again = links
        .for_period(santos, nena_id, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();
    let november = links
        .for_period(santos, nena_id, "2026-11", date!(2026 - 12 - 05))
        .await
        .unwrap();

    assert_eq!(october, again);
    assert_ne!(october, november);
    assert!(october.starts_with("http://kompleto.test/u/"), "{october}");
    assert_eq!(token(&october).len(), 64, "a 128-bit ID and a 128-bit signature");
    let open = links.open(token(&october)).await.unwrap();
    assert_eq!(
        (open.firm, open.client, open.period.as_str()),
        (santos, nena_id, "2026-10")
    );
}

#[sqlx::test]
async fn f2_a_guessed_or_altered_link_is_not_found(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let nena_id = Clients::new(pool.clone(), clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let links = UploadLinks::new(pool.clone(), clock.clone(), &master_key(), BASE_URL);
    let url = links
        .for_period(santos, nena_id, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();
    let token = token(&url);
    let mut altered = token.to_owned();
    let last = if altered.ends_with('0') { "1" } else { "0" };
    altered.replace_range(63.., last);
    let other_key = UploadLinks::new(
        pool.clone(),
        clock,
        &MasterKey::from_hex(&"cd".repeat(32)).unwrap(),
        BASE_URL,
    );

    for (case, result) in [
        ("altered signature", links.open(&altered).await),
        ("ID alone", links.open(&token[..32]).await),
        ("not hex", links.open("../../etc/passwd").await),
        ("signed with another key", other_key.open(token).await),
    ] {
        assert!(matches!(result, Err(LinkError::NotFound)), "{case}: {result:?}");
    }
    let stored: Vec<String> = sqlx::query_scalar("select row_to_json(l)::text from upload_links l")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert!(
        !stored.concat().contains(&token[32..]),
        "the database holds no signature"
    );
}

#[sqlx::test]
async fn f2_a_link_stops_working_30_days_after_the_cut_off(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let nena_id = Clients::new(pool.clone(), clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let links = UploadLinks::new(pool.clone(), clock.clone(), &master_key(), BASE_URL);
    let url = links
        .for_period(santos, nena_id, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();

    // The last minute of 5 Dec 2026 in Manila, then the first of 6 Dec.
    clock.advance(datetime!(2026-12-05 23:59 +8) - datetime!(2026-10-21 09:00 UTC));
    links.open(token(&url)).await.unwrap();
    clock.advance(time::Duration::minutes(1));
    assert!(matches!(links.open(token(&url)).await, Err(LinkError::Expired(_))));
}

#[sqlx::test]
async fn f2_a_changed_cut_off_moves_the_links_last_day(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let nena_id = Clients::new(pool.clone(), clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let links = UploadLinks::new(pool.clone(), clock.clone(), &master_key(), BASE_URL);
    let url = links
        .for_period(santos, nena_id, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();

    // The Firm moves the Cut-off to 9 Nov before the next reminder.
    let again = links
        .for_period(santos, nena_id, "2026-10", date!(2026 - 11 - 09))
        .await
        .unwrap();

    assert_eq!(again, url, "the same link");
    assert_eq!(links.open(token(&url)).await.unwrap().cut_off, date!(2026 - 11 - 09));
    clock.advance(datetime!(2026-12-09 23:59 +8) - datetime!(2026-10-21 09:00 UTC));
    links.open(token(&url)).await.unwrap();
    clock.advance(time::Duration::minutes(1));
    assert!(matches!(links.open(token(&url)).await, Err(LinkError::Expired(_))));
}

#[sqlx::test]
async fn g1_each_link_has_a_signed_reply_address_naming_its_client_and_period(pool: PgPool) {
    let clock = clock();
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let nena_id = Clients::new(pool.clone(), clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let links = common::upload_links(&pool, clock);
    let issued = links
        .issue(santos, nena_id, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();
    let reply = issued.reply_to.unwrap();

    let found = links
        .by_reply_address(&["someone@else.test".to_owned(), reply.clone()])
        .await
        .unwrap()
        .unwrap();

    assert!(
        reply.starts_with("reply+") && reply.ends_with("@getkompleto.test"),
        "{reply}"
    );
    assert_eq!(
        (found.firm, found.client, found.period.as_str()),
        (santos, nena_id, "2026-10")
    );
    let mut altered = reply.clone();
    altered.replace_range(10..11, if &reply[10..11] == "0" { "1" } else { "0" });
    let upload_code = format!("reply+{}@getkompleto.test", token(&issued.url));
    let elsewhere = reply.replace("getkompleto.test", "evil.test");
    for (case, address) in [
        ("altered", altered),
        ("the upload code", upload_code),
        ("another domain", elsewhere),
    ] {
        assert!(links.by_reply_address(&[address]).await.unwrap().is_none(), "{case}");
    }
}
