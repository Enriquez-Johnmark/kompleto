//! Seam 3 (M4): what the upload page keeps (F1), against Postgres.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::upload_links::OpenLink;
use kompleto::kompleto::uploads::{FileKind, Label, MAX_FILE_BYTES, UploadError, Uploads};
use sqlx::PgPool;
use time::macros::date;

const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0 a photo of a receipt";

fn owed() -> Vec<String> {
    vec![
        "Sales invoices issued, including cancelled ones".to_owned(),
        "2307s received from customers".to_owned(),
    ]
}

fn invoices() -> Label {
    Label::Owed {
        document: owed()[0].clone(),
        payor: None,
    }
}

async fn setup(pool: &PgPool) -> (Uploads, OpenLink) {
    let clock = clock();
    let santos = firm(pool, "Santos Bookkeeping").await;
    let client = Clients::new(pool.clone(), clock.clone())
        .create(santos, &nena())
        .await
        .unwrap();
    let links = common::upload_links(pool, clock.clone());
    let url = links
        .for_period(santos, client, "2026-10", date!(2026 - 11 - 05))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    let uploads = Uploads::new(pool.clone(), clock.clone(), common::files(pool, clock));
    (uploads, link)
}

#[sqlx::test]
async fn f1_photos_and_pdfs_are_kept_whatever_their_names(pool: PgPool) {
    let (uploads, link) = setup(&pool).await;
    let heic = b"\0\0\0\x18ftypheic\0\0\0\0mif1heic".as_slice();
    let png = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".as_slice();
    let webp = b"RIFF\0\0\0\0WEBPVP8 ".as_slice();
    let pdf = b"%PDF-1.7\n%".as_slice();

    for (bytes, kind) in [
        (JPEG, FileKind::Jpeg),
        (heic, FileKind::Heic),
        (png, FileKind::Png),
        (webp, FileKind::Webp),
        (pdf, FileKind::Pdf),
    ] {
        uploads.add(&link, &owed(), invoices(), bytes).await.unwrap();
        assert_eq!(FileKind::sniff(bytes), Some(kind));
    }
    let kept = uploads.for_period(link.firm, link.client, "2026-10").await.unwrap();
    assert_eq!(kept.len(), 5);
    assert_eq!(kept[0].label, invoices());
}

#[sqlx::test]
async fn f1_anything_else_or_over_10_mb_is_refused(pool: PgPool) {
    let (uploads, link) = setup(&pool).await;
    let mut big = JPEG.to_vec();
    big.resize(MAX_FILE_BYTES + 1, 0);

    for (case, bytes) in [
        ("a program", b"MZ\x90\0 an .exe renamed receipt.jpg".as_slice()),
        ("a web page", b"<html><script>alert(1)</script>".as_slice()),
        ("an SVG", b"<svg xmlns=\"http://www.w3.org/2000/svg\">".as_slice()),
    ] {
        let refused = uploads.add(&link, &owed(), invoices(), bytes).await;
        assert!(
            matches!(refused, Err(UploadError::NotAllowedType | UploadError::TooBig)),
            "{case}: {refused:?}"
        );
    }
    assert!(matches!(
        uploads.add(&link, &owed(), invoices(), &big).await,
        Err(UploadError::TooBig)
    ));
    assert!(matches!(
        uploads.add(&link, &owed(), invoices(), b"").await,
        Err(UploadError::NoFile)
    ));
    let mut largest = JPEG.to_vec();
    largest.resize(MAX_FILE_BYTES, 0);
    uploads.add(&link, &owed(), invoices(), &largest).await.unwrap();
}

#[sqlx::test]
async fn f1_each_file_is_labelled_and_a_2307_names_its_payor(pool: PgPool) {
    let (uploads, link) = setup(&pool).await;
    let label = |document: &str, payor: Option<&str>| Label::Owed {
        document: document.to_owned(),
        payor: payor.map(str::to_owned),
    };

    for (case, bad) in [
        ("not owed", label("Bank statements", None)),
        ("2307 without a payor", label("2307s received from customers", None)),
        (
            "2307 with a blank payor",
            label("2307s received from customers", Some("  ")),
        ),
        ("note missing", Label::SomethingElse { note: String::new() }),
        (
            "note over two lines",
            Label::SomethingElse {
                note: "a\nb".to_owned(),
            },
        ),
    ] {
        let refused = uploads.add(&link, &owed(), bad, JPEG).await;
        assert!(matches!(refused, Err(UploadError::Invalid(_))), "{case}: {refused:?}");
    }
    uploads
        .add(
            &link,
            &owed(),
            label("2307s received from customers", Some(" Jollibee Foods Corp ")),
            JPEG,
        )
        .await
        .unwrap();
    uploads
        .add(
            &link,
            &owed(),
            Label::SomethingElse {
                note: "Lazada payout report".to_owned(),
            },
            JPEG,
        )
        .await
        .unwrap();

    let kept = uploads.for_period(link.firm, link.client, "2026-10").await.unwrap();
    assert_eq!(
        kept[0].label,
        label("2307s received from customers", Some("Jollibee Foods Corp"))
    );
    assert_eq!(
        kept[1].label,
        Label::SomethingElse {
            note: "Lazada payout report".to_owned()
        }
    );
}

#[sqlx::test]
async fn f1_a_link_takes_at_most_60_files(pool: PgPool) {
    let (uploads, link) = setup(&pool).await;

    for _ in 0..60 {
        uploads.add(&link, &owed(), invoices(), JPEG).await.unwrap();
    }

    assert!(matches!(
        uploads.add(&link, &owed(), invoices(), JPEG).await,
        Err(UploadError::LinkFull)
    ));
}

#[sqlx::test]
async fn f3_marking_an_upload_received_twice_at_once_records_it_once(pool: PgPool) {
    let (uploads, link) = setup(&pool).await;
    let id = uploads.add(&link, &owed(), invoices(), JPEG).await.unwrap();

    let (one, two) = tokio::join!(
        uploads.mark_received(link.firm, link.client, id, "ana@santos.test"),
        uploads.mark_received(link.firm, link.client, id, "ana@santos.test"),
    );

    one.unwrap();
    two.unwrap();
    let recorded: i64 = sqlx::query_scalar("select count(*) from audit_events where action = 'upload.received'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(recorded, 1);
}
