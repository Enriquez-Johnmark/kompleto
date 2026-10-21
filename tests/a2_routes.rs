//! Seam 5: every page about one Client answers "not found" to another Firm
//! (A2). The router is built from the same list, so a new page can't skip
//! this check.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::nena;
use common::web::TestApp;
use kompleto::kompleto::checklist::{ExtraItem, Recurs};
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::web::client_routes;
use sqlx::PgPool;

#[sqlx::test]
async fn a2_every_client_page_is_not_found_for_another_firm(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (santos, santos_cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let (_, reyes_cookie) = app.logged_in_firm("Reyes CPA").await;
    let clients = Clients::new(pool.clone(), app.clock.clone());
    let client = clients.create(santos, &nena()).await.unwrap();
    let extra = ExtraItem {
        label: "Lazada payout reports".to_owned(),
        recurs: Recurs::EveryPeriod,
    };
    let item = clients.add_extra_item(santos, client, &extra).await.unwrap();
    let routes = client_routes();
    assert!(routes.len() >= 3, "{routes:?}");

    for route in routes {
        let uri = route
            .path
            .replace("{id}", &client.to_string())
            .replace("{item}", &item.to_string());
        let form = (route.method == Method::POST).then_some(route.sample_form);

        let foreign = app.request(route.method.clone(), &uri, Some(&reyes_cookie), form).await;
        assert_eq!(
            foreign.status,
            StatusCode::NOT_FOUND,
            "{} {uri} as another firm",
            route.method
        );

        let own = app
            .request(route.method.clone(), &uri, Some(&santos_cookie), form)
            .await;
        assert!(
            own.status.is_success() || own.status.is_redirection(),
            "{} {uri}: {}",
            route.method,
            own.status
        );
    }
}
