//! Seam 3: what the Excel parser child can reach. A stand-in shell script
//! plays a parser that a crafted file has taken over.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use kompleto::kompleto::import::ImportError;
use kompleto::kompleto::import::xlsx::XlsxReader;

/// An executable script in a fresh directory.
fn script(dir: &std::path::Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

// One test, so the scripts are written before any child process starts.
#[tokio::test]
async fn b2_a_hijacked_excel_parser_gets_no_secrets_and_cant_flood_the_server() {
    let dir = std::env::temp_dir().join(format!("kompleto-sandbox-{}", uuid::Uuid::now_v7()));
    std::fs::create_dir_all(&dir).unwrap();
    let env_file = dir.join("child.env");
    let snoop = script(&dir, "snoop", &format!("env > '{}'; echo '[]'", env_file.display()));
    let flood = script(&dir, "flood", "head -c 100000000 /dev/zero");
    assert!(
        std::env::var("DATABASE_URL").is_ok(),
        "the test itself has secrets to leak"
    );

    XlsxReader::new(snoop).read(b"PK").await.unwrap_err();
    let flooded = XlsxReader::new(flood).read(b"PK").await;

    let child_env = std::fs::read_to_string(&env_file).unwrap();
    assert!(!child_env.contains("DATABASE_URL"), "{child_env}");
    assert!(matches!(flooded, Err(ImportError::Unreadable(_))), "{flooded:?}");
    std::fs::remove_dir_all(&dir).unwrap();
}
