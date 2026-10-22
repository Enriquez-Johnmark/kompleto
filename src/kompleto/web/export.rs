//! The Firm's data export download (I1).

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

/// The zip is written to a temporary file, not memory, and streamed from
/// it; the file is unlinked once open, so nothing is left behind.
async fn export(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<Response, WebError> {
    let path = std::env::temp_dir().join(format!("kompleto-export-{}.zip", uuid::Uuid::now_v7()));
    let file = std::fs::File::create(&path).map_err(WebError::internal)?;
    let written = write_zip(&state.pool, &state.clients, &state.uploads, firm, file).await;
    let opened = match written {
        Ok(_) => tokio::fs::File::open(&path).await.map_err(WebError::internal),
        Err(err) => Err(WebError::internal(err)),
    };
    if let Err(err) = tokio::fs::remove_file(&path).await {
        tracing::error!(%firm, %err, "removing an export's temporary file failed");
    }
    let opened = opened?;
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
