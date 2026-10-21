//! Seam 5: every page about one Client answers "not found" to another Firm
//! (A2). The router is built from the same list, so a new page can't skip
//! this check.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::nena;
use common::web::TestApp;
use kompleto::engine::outbox::Outbox;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::checklist::{ExtraItem, Recurs};
use kompleto::kompleto::clients::Clients;
use kompleto::kompleto::clients::NewClient;
use kompleto::kompleto::reminders::Scheduler;
use kompleto::kompleto::web::scoped_routes;
use sqlx::PgPool;
use time::Duration;
use uuid::Uuid;

#[sqlx::test]
async fn a2_every_client_and_draft_page_is_not_found_for_another_firm(pool: PgPool) {
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
    // A reminder Draft: a monthly client, on 1 Nov 2026 when October's is due.
    let employer = NewClient {
        business_name: "Lito Hardware".to_owned(),
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    clients.create(santos, &employer).await.unwrap();
    app.clock.advance(Duration::days(11));
    Scheduler::new(pool.clone(), app.clock.clone())
        .unwrap()
        .run_once()
        .await
        .unwrap();
    let draft: Uuid = sqlx::query_scalar("select message_id from scheduled_drafts")
        .fetch_one(&pool)
        .await
        .unwrap();
    let outbox = Outbox::new(pool.clone(), app.clock.clone());
    let routes = scoped_routes();
    assert!(routes.len() >= 3, "{routes:?}");

    for route in routes {
        let uri = route
            .path
            .replace("{id}", &client.to_string())
            .replace("{item}", &item.to_string())
            .replace("{draft}", &draft.to_string());
        let hash = outbox.get(santos, draft.into()).await.unwrap().content_hash().to_hex();
        let sample = route.sample_form.replace("{hash}", &hash);
        let form = (route.method == Method::POST).then_some(sample.as_str());

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
        // A conflict means the page ran but an earlier page (say, discard)
        // already changed the Draft.
        assert!(
            own.status.is_success() || own.status.is_redirection() || own.status == StatusCode::CONFLICT,
            "{} {uri}: {}",
            route.method,
            own.status
        );
    }
}
