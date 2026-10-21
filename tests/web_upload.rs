//! Seam 3 (M4): the upload page in-process (F1, F2), with the test clock.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::checklist::owed_documents;
use kompleto::kompleto::clients::{ClientId, Clients};
use kompleto::kompleto::firms::FirmId;
use kompleto::kompleto::uploads::MAX_FILE_BYTES;
use sqlx::PgPool;
use time::Month;
use time::macros::date;

const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0 a photo of a receipt";

/// Nena's Upload link for September 2026 (Cut-off 26 Oct), as a path.
async fn nena_link(app: &TestApp) -> String {
    let santos = common::firm(&app.pool, "Santos Bookkeeping").await;
    nena_link_of(app, santos).await.1
}

/// Nena, a Client of `firm`, and her Upload link for September 2026.
async fn nena_link_of(app: &TestApp, firm: FirmId) -> (ClientId, String) {
    let client = Clients::new(app.pool.clone(), app.clock.clone())
        .create(firm, &nena())
        .await
        .unwrap();
    let url = common::upload_links(&app.pool, app.clock.clone())
        .for_period(firm, client, "2026-09", date!(2026 - 10 - 26))
        .await
        .unwrap();
    (client, url.trim_start_matches(common::web::BASE_URL).to_owned())
}

/// Nena uploads a photo of her first Owed document; returns its ID.
async fn upload_photo(app: &TestApp, link: &str) -> uuid::Uuid {
    let sent = app.post_multipart(link, &[("document", b"0"), ("file", JPEG)]).await;
    assert_eq!(sent.status, StatusCode::SEE_OTHER, "{}", sent.body);
    sqlx::query_scalar("select id from uploads order by uploaded_at desc limit 1")
        .fetch_one(&app.pool)
        .await
        .unwrap()
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
    // Not no-referrer: browsers would then send `Origin: null` with the
    // page's own uploads, and the same-origin check would refuse them all.
    assert_eq!(page.headers["referrer-policy"], "same-origin");
    assert!(!page.body.contains("no-referrer"));
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

#[sqlx::test]
async fn f3_the_firm_downloads_an_upload_as_an_attachment_with_a_generated_name(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let (client, link) = nena_link_of(&app, santos).await;
    let upload = upload_photo(&app, &link).await;

    let page = app.get(&format!("/clients/{client}"), &cookie).await;
    let file = app.get(&format!("/clients/{client}/uploads/{upload}"), &cookie).await;

    assert!(page.body.contains("Mark received"), "{}", page.body);
    assert_eq!(file.status, StatusCode::OK);
    assert_eq!(file.bytes, JPEG, "decrypted as sent");
    assert_eq!(file.headers["content-type"], "image/jpeg");
    assert_eq!(file.headers["x-content-type-options"], "nosniff");
    let disposition = file.headers["content-disposition"].to_str().unwrap();
    assert!(
        disposition.starts_with(r#"attachment; filename="Nenas-Sari-Sari-Store_2026-09_"#)
            && disposition.ends_with(r#"_1.jpg""#),
        "{disposition}"
    );
}

#[sqlx::test]
async fn f3_an_upload_marked_received_is_deleted_30_days_later(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let (client, link) = nena_link_of(&app, santos).await;
    let upload = upload_photo(&app, &link).await;

    let marked = app
        .post(&format!("/clients/{client}/uploads/{upload}/received"), &cookie, "")
        .await;

    assert_eq!(marked.status, StatusCode::SEE_OTHER);
    app.clock.advance(time::Duration::days(30) - time::Duration::minutes(1));
    assert_eq!(app.files.sweep().await.unwrap(), 0);
    app.clock.advance(time::Duration::minutes(1));
    assert_eq!(app.files.sweep().await.unwrap(), 1);
    let cookie = app.log_in("santos.bookkeeping@firm.test").await; // the old session has ended
    let gone = app.get(&format!("/clients/{client}/uploads/{upload}"), &cookie).await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
    let page = app.get(&format!("/clients/{client}"), &cookie).await;
    assert!(page.body.contains("Deleted"), "{}", page.body);
    let actor: String = sqlx::query_scalar("select actor from audit_events where action = 'upload.received'")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(actor, "santos.bookkeeping@firm.test");
}

#[sqlx::test]
async fn f3_an_upload_never_marked_received_is_deleted_after_90_days(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, _) = app.logged_in_firm("Santos Bookkeeping").await;
    let (client, link) = nena_link_of(&app, santos).await;
    let upload = upload_photo(&app, &link).await;

    app.clock.advance(time::Duration::days(90) - time::Duration::minutes(1));
    assert_eq!(app.files.sweep().await.unwrap(), 0);
    app.clock.advance(time::Duration::minutes(1));
    assert_eq!(app.files.sweep().await.unwrap(), 1);

    let cookie = app.log_in("santos.bookkeeping@firm.test").await; // the old session has ended
    let gone = app.get(&format!("/clients/{client}/uploads/{upload}"), &cookie).await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn g2_the_last_missing_document_cancels_the_periods_waiting_reminders(pool: PgPool) {
    use kompleto::kompleto::calendar::BirForm;
    use kompleto::kompleto::reminders::Scheduler;

    let app = TestApp::new(pool);
    let santos = common::firm(&app.pool, "Santos Bookkeeping").await;
    let employer = kompleto::kompleto::clients::NewClient {
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    Clients::new(app.pool.clone(), app.clock.clone())
        .create(santos, &employer)
        .await
        .unwrap();
    app.clock.advance(time::Duration::days(11)); // 1 Nov: October's first request
    Scheduler::new(
        app.pool.clone(),
        app.clock.clone(),
        common::upload_links(&app.pool, app.clock.clone()),
    )
    .unwrap()
    .run_once()
    .await
    .unwrap();
    let body: String = sqlx::query_scalar("select body_text from outbox_messages")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    let link = body
        .split_whitespace()
        .find(|word| word.starts_with("http://kompleto.test/u/"))
        .unwrap()
        .trim_start_matches(common::web::BASE_URL)
        .to_owned();
    let owed = owed_documents(&employer.forms, Month::October, &Default::default());

    for index in 0..owed.len() {
        let status = || async {
            sqlx::query_scalar::<_, String>("select status from outbox_messages")
                .fetch_one(&app.pool)
                .await
                .unwrap()
        };
        assert_eq!(status().await, "draft", "still waiting before upload {index}");
        let index = index.to_string();
        let sent = app
            .post_multipart(
                &link,
                &[
                    ("document", index.as_bytes()),
                    ("payor", b"Jollibee Foods Corp"),
                    ("file", JPEG),
                ],
            )
            .await;
        assert_eq!(sent.status, StatusCode::SEE_OTHER, "{}", sent.body);
    }

    let status: String = sqlx::query_scalar("select status from outbox_messages")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(status, "cancelled");
}

#[sqlx::test]
async fn f1_a_missing_or_broken_file_isnt_called_too_big(pool: PgPool) {
    use axum::body::Body;
    use axum::http::{Method, Request, header};

    let app = TestApp::new(pool);
    let link = nena_link(&app).await;

    let no_file = app.post_multipart(&link, &[("document", b"0")]).await;
    let broken = app
        .send(
            Request::builder()
                .method(Method::POST)
                .uri(&link)
                .header(header::ORIGIN, common::web::BASE_URL)
                .header(header::CONTENT_TYPE, "multipart/form-data; boundary=cut")
                .body(Body::from(
                    b"--cut\r\nContent-Disposition: form-data; name=\"file\"; filename=\"a.jpg\"\r\n\r\n\xFF\xD8"
                        .as_slice(),
                ))
                .unwrap(),
        )
        .await;

    assert_eq!(no_file.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(no_file.body.contains("Pumili po muna ng file"), "{}", no_file.body);
    assert_eq!(broken.status, StatusCode::BAD_REQUEST);
    assert!(broken.body.contains("Hindi po buo"), "{}", broken.body);
}
