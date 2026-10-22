//! Seam 5: every page and API endpoint about one of a Firm's records
//! answers "not found" to another Firm (A2). The router is built from the
//! same list, so a new endpoint can't skip this check.

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
use kompleto::kompleto::uploads::{Label, Uploads};
use kompleto::kompleto::web::scoped_routes;
use sqlx::PgPool;
use time::Duration;
use uuid::Uuid;

#[sqlx::test]
async fn a2_every_client_draft_and_upload_page_is_not_found_for_another_firm(pool: PgPool) {
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
    // An Upload from the Client's Contact.
    let links = common::upload_links(&pool, app.clock.clone());
    let url = links
        .for_period(santos, client, "2026-09", time::macros::date!(2026 - 10 - 26))
        .await
        .unwrap();
    let link = links.open(url.rsplit('/').next().unwrap()).await.unwrap();
    let owed = vec!["Bank statements".to_owned()];
    let label = Label::Owed {
        document: owed[0].clone(),
        payor: None,
    };
    let upload = Uploads::new(pool.clone(), app.clock.clone(), app.files.clone())
        .add(&link, &owed, label, b"%PDF-1.7 statement")
        .await
        .unwrap();
    // A Payor of the Client.
    let payor = kompleto::kompleto::payors::Payors::new(pool.clone(), app.clock.clone())
        .add(santos, client, "Jollibee Foods Corp")
        .await
        .unwrap();
    // A Reply from the Client's Contact.
    let reply = uuid::Uuid::now_v7();
    sqlx::query(
        "insert into replies (id, firm_id, client_id, period, from_address, subject, body_text, automatic, received_at)
         values ($1, $2, $3, '2026-09', 'nena@example.test', 'Re: documents', 'Ipapadala ko po.', false, now())",
    )
    .bind(reply)
    .bind(santos.as_uuid())
    .bind(client.as_uuid())
    .execute(&pool)
    .await
    .unwrap();
    // A reminder Draft: a monthly client, on 1 Nov 2026 when October's is due.
    let employer = NewClient {
        business_name: "Lito Hardware".to_owned(),
        forms: vec![BirForm::F1601C],
        ..nena()
    };
    clients.create(santos, &employer).await.unwrap();
    app.clock.advance(Duration::days(11));
    Scheduler::new(
        pool.clone(),
        app.clock.clone(),
        common::upload_links(&pool, app.clock.clone()),
    )
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
            .replace("{draft}", &draft.to_string())
            .replace("{upload}", &upload.to_string())
            .replace("{reply}", &reply.to_string())
            .replace("{payor}", &payor.to_string());
        let hash = outbox.get(santos, draft.into()).await.unwrap().content_hash().to_hex();
        let sample = route.sample_form.replace("{hash}", &hash);
        let api = uri.starts_with("/api/");
        let send = |cookie: String| {
            let (uri, sample, method) = (uri.clone(), sample.clone(), route.method.clone());
            let app = &app;
            async move {
                if api {
                    let json = (!sample.is_empty()).then(|| serde_json::from_str(&sample).unwrap());
                    app.api(method, &uri, Some(&cookie), json).await
                } else {
                    let form = (method != Method::GET).then_some(sample.as_str());
                    app.request(method, &uri, Some(&cookie), form).await
                }
            }
        };

        let foreign = send(reyes_cookie.clone()).await;
        assert_eq!(
            foreign.status,
            StatusCode::NOT_FOUND,
            "{} {uri} as another firm",
            route.method
        );

        let own = send(santos_cookie.clone()).await;
        // A conflict or a refusal means the page ran but an earlier page
        // already changed the Draft (discarded it, or edited out its link).
        assert!(
            own.status.is_success()
                || own.status.is_redirection()
                || [StatusCode::CONFLICT, StatusCode::UNPROCESSABLE_ENTITY].contains(&own.status),
            "{} {uri}: {}",
            route.method,
            own.status
        );
    }
}
