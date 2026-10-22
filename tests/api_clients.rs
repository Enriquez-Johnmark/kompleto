//! Adding and editing Clients through the API (B1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, Language};
use serde_json::{Value, json};
use sqlx::PgPool;

fn nena() -> Value {
    json!({
        "business_name": "Nena's Store",
        "contact_first_name": "Nena",
        "email": "nena@example.test",
        "language": "taglish",
        "forms": ["2551Q", "1701Q"],
    })
}

#[sqlx::test]
async fn b1_a_client_is_added_and_listed(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let added = app.api(Method::POST, "/api/clients", Some(&cookie), Some(nena())).await;

    assert_eq!(added.status, StatusCode::CREATED, "{}", added.body);
    let clients = Clients::new(pool, app.clock.clone()).list(firm).await.unwrap();
    assert_eq!(clients.len(), 1);
    assert_eq!(added.json()["id"], clients[0].id.to_string());
    assert_eq!(clients[0].details.business_name, "Nena's Store");
    assert_eq!(clients[0].details.language, Language::Taglish);
    assert_eq!(clients[0].details.forms, [BirForm::F2551Q, BirForm::F1701Q]);
    let list = app.api(Method::GET, "/api/clients", Some(&cookie), None).await.json();
    assert_eq!(list[0]["business_name"], "Nena's Store");
    assert_eq!(list[0]["forms"], json!(["2551Q", "1701Q"]));
    assert_eq!(list[0]["paused"], false);
}

#[sqlx::test]
async fn b1_a_client_with_problems_is_refused_with_every_reason(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let input = json!({
        "business_name": "Shop", "contact_first_name": "", "email": "nope",
        "language": "taglish", "forms": [],
    });

    let refused = app.api(Method::POST, "/api/clients", Some(&cookie), Some(input)).await;

    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    let problems = refused.json()["problems"].to_string();
    for reason in [
        "Contact first name: is required",
        "is not an email address",
        "Forms: tick at least one",
    ] {
        assert!(problems.contains(reason), "missing {reason:?} in {problems}");
    }
    assert!(
        Clients::new(pool, app.clock.clone())
            .list(firm)
            .await
            .unwrap()
            .is_empty()
    );
}

#[sqlx::test]
async fn b1_a_second_client_with_the_same_name_is_refused(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    app.api(Method::POST, "/api/clients", Some(&cookie), Some(nena())).await;

    let again = app.api(Method::POST, "/api/clients", Some(&cookie), Some(nena())).await;

    assert_eq!(again.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        again.json()["message"].as_str().unwrap().contains("Nena's Store"),
        "{}",
        again.body
    );
}

#[sqlx::test]
async fn b1_a_clients_details_can_be_edited(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let id = app
        .api(Method::POST, "/api/clients", Some(&cookie), Some(nena()))
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let mut english = nena();
    english["language"] = json!("english");

    let saved = app
        .api(Method::PUT, &format!("/api/clients/{id}"), Some(&cookie), Some(english))
        .await;

    assert_eq!(saved.status, StatusCode::NO_CONTENT, "{}", saved.body);
    let client = &Clients::new(pool, app.clock.clone()).list(firm).await.unwrap()[0];
    assert_eq!(client.details.language, Language::English);
}

#[sqlx::test]
async fn the_bir_forms_are_listed_with_their_names(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let forms = app.api(Method::GET, "/api/bir-forms", Some(&cookie), None).await.json();

    let codes: Vec<&str> = forms
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["code"].as_str().unwrap())
        .collect();
    assert!(codes.contains(&"2550Q") && codes.contains(&"0619-E"), "{codes:?}");
    assert!(forms[0]["name"].as_str().is_some_and(|name| !name.is_empty()));
}
