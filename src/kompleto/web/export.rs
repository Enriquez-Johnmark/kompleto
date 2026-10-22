//! The Firm's data export download (I1).

use std::io::{BufWriter, Seek};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde_json::json;
use time::macros::format_description;

use super::{AppState, CurrentFirm, WebError};
use crate::kompleto::activity::record_by_firm;
use crate::kompleto::export::write_zip;
use crate::kompleto::schedule::MANILA;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/settings/export", get(export))
}

/// A temporary file holding decrypted documents: only Kompleto's user can
/// read it, and it's removed however the request ends, even cancelled
/// half way, since dropping this removes it.
struct TempFile(PathBuf);

impl TempFile {
    fn create() -> std::io::Result<(Self, std::fs::File)> {
        let path = std::env::temp_dir().join(format!("kompleto-export-{}.zip", uuid::Uuid::now_v7()));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)?;
        Ok((Self(path), file))
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_file(&self.0) {
            tracing::error!(%err, "removing an export's temporary file failed");
        }
    }
}

/// The zip is written to a temporary file, not memory, and streamed from
/// it; the file is removed once open, so nothing is left behind.
async fn export(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<Response, WebError> {
    let (temp, file) = TempFile::create().map_err(WebError::internal)?;
    let written = write_zip(&state.pool, &state.clients, &state.uploads, firm, BufWriter::new(file))
        .await
        .map_err(WebError::internal)?;
    let mut file = written
        .into_inner()
        .map_err(|err| WebError::internal(err.into_error()))?;
    file.rewind().map_err(WebError::internal)?;
    let opened = tokio::fs::File::from_std(file);
    drop(temp);
    let now = state.clock.now();
    let mut tx = state.pool.begin().await.map_err(WebError::internal)?;
    record_by_firm(&mut tx, firm, now, "firm.exported", ("firm", firm.as_uuid()), json!({}))
        .await
        .map_err(WebError::internal)?;
    tx.commit().await.map_err(WebError::internal)?;
    let date = now
        .to_offset(MANILA)
        .date()
        .format(format_description!("[year]-[month]-[day]"))
        .map_err(WebError::internal)?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/zip".to_owned()),
            (
                header::CONTENT_DISPOSITION,
                format!(r#"attachment; filename="kompleto-export-{date}.zip""#),
            ),
        ],
        Body::from_stream(tokio_util::io::ReaderStream::new(opened)),
    )
        .into_response())
}
