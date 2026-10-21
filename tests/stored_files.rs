//! Seam 1 and 4 (M4): encrypted files kept in a file store, and deleted at
//! their deadline (F3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::clock;
use kompleto::engine::clock::FixedClock;
use kompleto::engine::files::{FileCipher, FileError, Files, LocalDisk, MasterKey};
use kompleto::engine::ids::TenantId;
use sqlx::PgPool;
use time::Duration;

fn files(pool: &PgPool, clock: Arc<FixedClock>) -> (Files, PathBuf) {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(uuid::Uuid::now_v7().to_string());
    let cipher = FileCipher::new(&MasterKey::from_hex(&"ab".repeat(32)).unwrap());
    let files = Files::new(pool.clone(), clock, Arc::new(LocalDisk::new(&root)), Arc::new(cipher));
    (files, root)
}

fn on_disk(root: &PathBuf) -> Vec<Vec<u8>> {
    let mut found = Vec::new();
    for tenant in std::fs::read_dir(root).into_iter().flatten().flatten() {
        for file in std::fs::read_dir(tenant.path()).unwrap().flatten() {
            found.push(std::fs::read(file.path()).unwrap());
        }
    }
    found
}

#[sqlx::test]
async fn f3_a_saved_file_is_encrypted_on_disk_and_reads_back_for_its_firm_only(pool: PgPool) {
    let clock = clock();
    let (files, root) = files(&pool, clock.clone());
    let (santos, reyes) = (TenantId::new(), TenantId::new());
    let photo = b"JPEG of a September sales invoice".to_vec();

    let id = files
        .save(santos, &photo, clock.now_plus(Duration::days(90)))
        .await
        .unwrap();

    assert_eq!(files.read(santos, id).await.unwrap(), photo);
    assert!(matches!(files.read(reyes, id).await, Err(FileError::NotFound)));
    let stored = on_disk(&root);
    assert_eq!(stored.len(), 1);
    assert!(!stored[0].windows(photo.len()).any(|w| w == photo.as_slice()));
}
