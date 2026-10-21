//! Seam 3 (M4): the upload page in-process (F1, F2), with the test clock.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::checklist::owed_documents;
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::uploads::MAX_FILE_BYTES;
use sqlx::PgPool;
use time::Month;
use time::macros::date;

const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0 a photo of a receipt";

/// Nena's Upload link for September 2026 (Cut-off 26 Oct), as a path.
async fn nena_link(app: &TestApp) -> String {
    let santos = common::firm(&app.pool, "Santos Bookkeeping").await;
    let client = Clients::new(app.pool.clone(), app.clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let url = common::upload_links(&app.pool, app.clock.clone())
        .for_period(santos, client, "2026-09", date!(2026 - 10 - 26))
        .await
        .unwrap();
    url.trim_start_matches(common::web::BASE_URL).to_owned()
}

fn owed() -> Vec<String> {
    owed_documents(&nena().forms, Month::September, &Default::default())
}

fn index_of_2307() -> String {
    owed()
        .iter()
        .position(|item| item.contains("2307"))
        .unwrap()
        .to_string()
}

#[sqlx::test]
async fn f1_the_upload_page_opens_without_login_and_lists_what_is_owed(pool: PgPool) {
    let app = TestApp::new(pool);
    let link = nena_link(&app).await;

    let page = app.request(axum::http::Method::GET, &link, None, None).await;

    assert_eq!(page.status, StatusCode::OK);
    assert!(
        page.body
            .contains("Kailangan po ng Santos Bookkeeping ang mga ito bago Mon 26 Oct 2026"),
        "{}",
        page.body
    );
    for item in owed() {
        assert!(page.body.contains(&item.replace('\'', "&#39;")), "missing {item:?}");
    }
    assert!(page.body.contains("Wala pa"));
    assert!(page.body.contains(r#"name="viewport""#), "made for phones");
    assert_eq!(page.headers["x-robots-tag"], "noindex, nofollow");
    assert_eq!(page.headers["referrer-policy"], "no-referrer");
}

#[sqlx::test]
async fn f1_a_photo_sent_from_the_page_is_kept_and_shown_as_uploaded(pool: PgPool) {
    let app = TestApp::new(pool);
    let link = nena_link(&app).await;

    let sent = app.post_multipart(&link, &[("document", b"0"), ("file", JPEG)]).await;

    assert_eq!(sent.status, StatusCode::SEE_OTHER, "{}", sent.body);
    let page = app.request(axum::http::Method::GET, sent.location(), None, None).await;
    assert!(page.body.contains("Salamat po! Natanggap na namin"), "{}", page.body);
    assert!(page.body.contains("Na-upload na"));
    let stored: i64 = sqlx::query_scalar("select count(*) from uploads")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(stored, 1);
}

#[sqlx::test]
async fn f1_a_refused_file_gets_a_clear_message_in_the_clients_language(pool: PgPool) {
    let app = TestApp::new(pool);
    let link = nena_link(&app).await;
    let mut big = JPEG.to_vec();
    big.resize(MAX_FILE_BYTES + 1, 0);
    let payorless = index_of_2307();

    for (case, fields, status, message) in [
        (
            "a program",
            vec![("document", b"0".as_slice()), ("file", b"MZ\x90\0".as_slice())],
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Photo o PDF lang po",
        ),
        (
            "over 10 MB",
            vec![("document", b"0".as_slice()), ("file", big.as_slice())],
            StatusCode::PAYLOAD_TOO_LARGE,
            "Hanggang 10 MB lang po",
        ),
        (
            "a 2307 without its payor",
            vec![("document", payorless.as_bytes()), ("file", JPEG)],
            StatusCode::UNPROCESSABLE_ENTITY,
            "kung aling business",
        ),
    ] {
        let refused = app.post_multipart(&link, &fields).await;

        assert_eq!(refused.status, status, "{case}");
        assert!(refused.body.contains(message), "{case}: {}", refused.body);
    }
    let stored: i64 = sqlx::query_scalar("select count(*) from uploads")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(stored, 0);
}

#[sqlx::test]
async fn f2_an_altered_link_is_not_found_and_an_expired_one_says_whom_to_ask(pool: PgPool) {
    let app = TestApp::new(pool);
    let link = nena_link(&app).await;
    let mut altered = link.clone();
    let last = if altered.ends_with('0') { "1" } else { "0" };
    altered.replace_range(altered.len() - 1.., last);

    let wrong = app.request(axum::http::Method::GET, &altered, None, None).await;
    let wrong_post = app
        .post_multipart(&altered, &[("document", b"0"), ("file", JPEG)])
        .await;
    app.clock.advance(time::Duration::days(36)); // 26 Nov 2026: past 30 days after the Cut-off
    let expired = app.request(axum::http::Method::GET, &link, None, None).await;
    let expired_post = app.post_multipart(&link, &[("document", b"0"), ("file", JPEG)]).await;

    assert_eq!(
        (wrong.status, wrong_post.status),
        (StatusCode::NOT_FOUND, StatusCode::NOT_FOUND)
    );
    assert_eq!(
        (expired.status, expired_post.status),
        (StatusCode::GONE, StatusCode::GONE)
    );
    assert!(
        expired.body.contains("Pakihingi po ng bago sa Santos Bookkeeping"),
        "{}",
        expired.body
    );
}
