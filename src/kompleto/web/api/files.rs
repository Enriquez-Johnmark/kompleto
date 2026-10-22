//! The Firm's side of Uploads (F3): download one, and mark it received,
//! which starts its 30 days before deletion.

use axum::Router;
use axum::extract::State;
use axum::http::{Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{MethodRouter, get, post};
use uuid::Uuid;

use super::{ApiError, ApiFirm, ApiPath};
use crate::engine::ids::FileId;
use crate::kompleto::clients::ClientId;
use crate::kompleto::web::{AppState, ScopedRoute, actor, scoped_router};

pub(super) fn routes() -> Router<AppState> {
    scoped_router(file_endpoints())
}

/// Every endpoint about one Upload (A2).
pub(in crate::kompleto::web) fn file_endpoints() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    let endpoint = |method, path| ScopedRoute {
        method,
        path,
        sample_form: "",
    };
    vec![
        (
            endpoint(Method::GET, "/api/clients/{id}/uploads/{upload}/file"),
            get(download),
        ),
        (
            endpoint(Method::POST, "/api/clients/{id}/uploads/{upload}/received"),
            post(received),
        ),
    ]
}

/// The file, as an attachment with a generated name, so nothing a Contact
/// sent is ever shown in the browser or named by them.
async fn download(
    State(state): State<AppState>,
    ApiFirm(firm): ApiFirm,
    ApiPath((id, upload)): ApiPath<(Uuid, Uuid)>,
) -> Result<Response, ApiError> {
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
    ApiFirm(firm): ApiFirm,
    ApiPath((id, upload)): ApiPath<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let by = actor(&state, firm).await?;
    state
        .uploads
        .mark_received(firm, ClientId::from(id), FileId::from(upload), &by)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
