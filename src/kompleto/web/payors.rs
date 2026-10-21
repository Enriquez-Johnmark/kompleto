//! A Client's Payor list (H2), edited on the Client page.

use axum::extract::{Path, State};
use axum::http::Method;
use axum::response::Response;
use axum::routing::{MethodRouter, post};
use axum::{Form, Router};
use serde::Deserialize;
use uuid::Uuid;

use super::clients::{back_to, refuse};
use super::{AppState, CurrentFirm, ScopedRoute, WebError};
use crate::kompleto::clients::{ClientError, ClientId};
use crate::kompleto::payors::PayorId;

pub(super) fn routes() -> Router<AppState> {
    super::scoped_router(payor_pages())
}

/// Every page about one Client's Payors (A2).
pub(super) fn payor_pages() -> Vec<(ScopedRoute, MethodRouter<AppState>)> {
    vec![
        (
            ScopedRoute {
                method: Method::POST,
                path: "/clients/{id}/payors",
                sample_form: "name=Puregold+Price+Club",
            },
            post(add),
        ),
        (
            ScopedRoute {
                method: Method::POST,
                path: "/clients/{id}/payors/{payor}/delete",
                sample_form: "",
            },
            post(remove),
        ),
    ]
}

#[derive(Deserialize)]
struct PayorForm {
    name: String,
}

async fn add(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path(id): Path<Uuid>,
    Form(form): Form<PayorForm>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    match state.payors.add(firm, id, &form.name).await {
        Ok(_) => Ok(back_to(id)),
        Err(err @ (ClientError::Invalid(_) | ClientError::DuplicateName(_))) => {
            refuse(&state, firm, id, format!("{err}.")).await
        }
        Err(err) => Err(err.into()),
    }
}

async fn remove(
    State(state): State<AppState>,
    CurrentFirm(firm): CurrentFirm,
    Path((id, payor)): Path<(Uuid, Uuid)>,
) -> Result<Response, WebError> {
    let id = ClientId::from(id);
    state.payors.remove(firm, id, PayorId::from(payor)).await?;
    Ok(back_to(id))
}
