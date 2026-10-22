//! Seam 3 (M6): the Firm's data export in-process (I1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::io::Read;

use axum::http::StatusCode;
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::clients::{Clients, NewClient};
use kompleto::kompleto::payors::Payors;
use kompleto::kompleto::uploads::{Label, Uploads};
use sqlx::PgPool;
use time::macros::date;

const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0 a photo of a 2307";

#[sqlx::test]
async fn i1_the_firm_downloads_all_its_data_as_one_zip(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let (reyes, _) = app.logged_in_firm("Reyes CPA").await;
    let clients = Clients::new(app.pool.clone(), app.clock.clone());
    let nena_id = clients.create(santos, &nena()).await.unwrap();
    let other = NewClient {
        business_name: "Reyes Client Store".to_owned(),
        ..nena()
    };
    clients.create(reyes, &other).await.unwrap();
    Payors::new(app.pool.clone(), app.clock.clone())
        .add(santos, nena_id, "Jollibee Foods Corp")
        .await
        .unwrap();
    let links = common::upload_links(&app.pool, app.clock.clone());
    let url = links
        .for_period(santos, nena_id, "2026-09", date!(2026 - 10 - 26))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    let document = "BIR Form 2307 from Jollibee Foods Corp".to_owned();
    Uploads::new(app.pool.clone(), app.clock.clone(), app.files.clone())
        .add(
            &link,
            std::slice::from_ref(&document),
            Label::Owed {
                document: document.clone(),
                payor: Some("Jollibee Foods Corp".to_owned()),
            },
            JPEG,
        )
        .await
        .unwrap();
    // A Contact's text that a spreadsheet would run as a formula.
    sqlx::query(
        "insert into replies (id, firm_id, client_id, period, from_address, body_text, automatic, received_at)
         values ($1, $2, $3, '2026-09', 'nena@example.test', '=1+1 po', false, now())",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(santos.as_uuid())
    .bind(nena_id.as_uuid())
    .execute(&app.pool)
    .await
    .unwrap();

    let export = app.get("/api/settings/export", &cookie).await;

    assert_eq!(export.status, StatusCode::OK);
    assert_eq!(export.headers["content-type"], "application/zip");
    assert_eq!(
        export.headers["content-disposition"],
        r#"attachment; filename="kompleto-export-2026-10-21.zip""#
    );
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(export.bytes.clone())).unwrap();
    let mut read = |name: &str| {
        let mut bytes = Vec::new();
        zip.by_name(name)
            .unwrap_or_else(|_| panic!("no {name}"))
            .read_to_end(&mut bytes)
            .unwrap();
        bytes
    };
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).unwrap();
    let clients_csv = text(read("clients.csv"));
    assert!(clients_csv.contains("Nena's Sari-Sari Store"), "{clients_csv}");
    assert!(!clients_csv.contains("Reyes Client Store"), "another Firm's");
    assert!(text(read("payors.csv")).contains("Jollibee Foods Corp"));
    assert!(text(read("uploads.csv")).contains(&document));
    assert!(text(read("replies.csv")).contains("'=1+1 po"), "never a formula");
    assert!(text(read("activity.csv")).contains("client.created"));
    for name in ["checklist.csv", "due_dates.csv", "messages.csv", "bounces.csv"] {
        read(name);
    }
    let photo = zip
        .file_names()
        .filter_map(Result::ok)
        .find(|name| name.starts_with("files/"))
        .unwrap()
        .into_owned();
    assert!(photo.ends_with(".jpg"), "{photo}");
    let mut bytes = Vec::new();
    zip.by_name(&photo).unwrap().read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes, JPEG);
    let logged: i64 =
        sqlx::query_scalar("select count(*) from audit_events where tenant_id = $1 and action = 'firm.exported'")
            .bind(santos.as_uuid())
            .fetch_one(&app.pool)
            .await
            .unwrap();
    assert_eq!(logged, 1);
}

/// Removes the stored file named `id` somewhere under `dir`.
fn remove_stored(dir: &std::path::Path, id: &str) -> bool {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() && remove_stored(&path, id) {
            return true;
        }
        if path.file_name().is_some_and(|name| name == id) {
            std::fs::remove_file(&path).unwrap();
            return true;
        }
    }
    false
}

#[sqlx::test]
async fn i1_an_export_survives_clients_whose_files_share_a_name_and_a_missing_file(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let clients = Clients::new(app.pool.clone(), app.clock.clone());
    let links = common::upload_links(&app.pool, app.clock.clone());
    let uploads = Uploads::new(app.pool.clone(), app.clock.clone(), app.files.clone());
    // Both names make the same file name: apostrophes are dropped.
    let mut ids = Vec::new();
    for name in ["Nena's Store", "Nenas Store", "Lost File Store"] {
        let client = NewClient {
            business_name: name.to_owned(),
            ..nena()
        };
        let id = clients.create(santos, &client).await.unwrap();
        let url = links
            .for_period(santos, id, "2026-09", date!(2026 - 10 - 26))
            .await
            .unwrap();
        let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
        let label = Label::SomethingElse {
            note: "resibo".to_owned(),
        };
        ids.push(uploads.add(&link, &[], label, JPEG).await.unwrap());
    }
    // The deletion sweep removed this one's file but hasn't marked it yet.
    remove_stored(
        &std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")),
        &ids[2].to_string(),
    );

    let export = app.get("/api/settings/export", &cookie).await;

    assert_eq!(export.status, StatusCode::OK);
    let zip = zip::ZipArchive::new(std::io::Cursor::new(export.bytes)).unwrap();
    let files: Vec<String> = zip
        .file_names()
        .filter_map(Result::ok)
        .filter(|name| name.starts_with("files/"))
        .map(|name| name.into_owned())
        .collect();
    assert_eq!(files.len(), 2, "{files:?}");
    assert_ne!(files[0], files[1]);
}
