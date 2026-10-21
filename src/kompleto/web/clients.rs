//! Client pages.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use super::{AppState, CurrentFirm, WebError};
use crate::kompleto::clients::Client;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/clients", get(list))
}

#[derive(Template, WebTemplate)]
#[template(path = "clients.html")]
struct ClientList {
    clients: Vec<Client>,
}

async fn list(State(state): State<AppState>, CurrentFirm(firm): CurrentFirm) -> Result<ClientList, WebError> {
    Ok(ClientList {
        clients: state.clients.list(firm).await?,
    })
}
