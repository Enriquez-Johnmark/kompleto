//! The clamd client: files are streamed to ClamAV and its verdict is read
//! strictly; anything but a clear answer is an error, never "clean".

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::time::Duration;

use common::clamd::{Behaviour, FakeClamd, nobody_listening};
use kompleto::engine::scan::{Clamd, ScanError, Verdict};

#[tokio::test]
async fn f1_a_clean_file_is_streamed_whole_and_passes() {
    let fake = FakeClamd::start(Behaviour::Scans).await;
    // Larger than one chunk, so it arrives in several.
    let file: Vec<u8> = (0..200_000u32).map(|n| (n % 251) as u8).collect();

    let verdict = Clamd::new(fake.addr()).scan(&file).await.unwrap();

    assert_eq!(verdict, Verdict::Clean);
    assert_eq!(fake.streams.lock().unwrap().as_slice(), [file]);
}

#[tokio::test]
async fn f1_an_infected_file_is_reported_with_the_virus_name() {
    let fake = FakeClamd::start(Behaviour::Scans).await;

    let verdict = Clamd::new(fake.addr()).scan(b"%PDF-1.4 EICAR").await.unwrap();

    assert_eq!(verdict, Verdict::Infected("Eicar-Test-Signature".to_owned()));
}

#[tokio::test]
async fn f1_an_error_answer_unreachable_or_silent_clamd_is_an_error() {
    let erroring = FakeClamd::start(Behaviour::Errors).await;
    let hanging = FakeClamd::start(Behaviour::Hangs).await;

    let errored = Clamd::new(erroring.addr()).scan(b"%PDF-1.4").await;
    let unreachable = Clamd::new(nobody_listening().await).scan(b"%PDF-1.4").await;
    let timed_out = Clamd::new(hanging.addr())
        .with_timeout(Duration::from_millis(200))
        .scan(b"%PDF-1.4")
        .await;

    assert!(matches!(errored, Err(ScanError::Unexpected(answer)) if answer.contains("size limit")));
    assert!(matches!(unreachable, Err(ScanError::Unreachable(_))));
    assert!(matches!(timed_out, Err(ScanError::TimedOut)));
}
