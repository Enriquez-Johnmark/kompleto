//! The Firm's side of Uploads (F3): download one, and mark it received,
//! which starts its 30 days before deletion.

use axum::Router;
use axum::extract::{Path, State};
use axum::http::{Method, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, get, post};
use uuid::Uuid;

use super::{AppState, CurrentFirm, ScopedRoute, WebError, actor};
use crate::engine::ids::FileId;
use crate::kompleto::clients::ClientId;
use crate::kompleto::uploads::UploadError;

pub(super) fn routes() -> Router<AppState> {
    super::scoped_router(upload_pages())
}

/// Every page about one Upload (A2).
pub(super) fn upload_pages() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    let page = |method, path| ScopedRoute {
        method,
        path,
        sample_form: "",
    };
    vec![
        (page(Method::GET, "/clients/{id}/uploads/{upload}"), get(download)),
        (
            page(Method::POST, "/clients/{id}/uploads/{upload}/received"),
            post(received),
        ),
    ]
}

impl From<UploadError> for WebError {
    fn from(err: UploadError) -> Self {
        match err {
            UploadError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

/// The file, as an attachment with a generated name, so nothing a Contact
/// sent is ever shown in the browser or named by them.
async fn download(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path((id, upload)): Path<(Uuid, Uuid)>,
) -> Result<Response, WebError> {
    let client = state.clients.get(firm, ClientId::from(id)).await?;
    let file = state
        .uploads
        .download(firm, client.id, FileId::from(upload), &client.details.business_name)
        .await?;
    Ok((
        [
            (header::CONTENT_TYPE, file.kind.media_type().to_owned()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", file.name),
            ),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff".to_owned()),
            (header::CACHE_CONTROL, "no-store".to_owned()),
        ],
        file.bytes,
    )
        .into_response())
}

async fn received(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path((id, upload)): Path<(Uuid, Uuid)>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    let by = actor(&state, firm).await?;
    state.uploads.mark_received(firm, id, FileId::from(upload), &by).await?;
    Ok(Redirect::to(&format!("/clients/{id}#uploads")).into_response())
}
