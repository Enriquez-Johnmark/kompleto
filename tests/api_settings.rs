//! The Firm's settings through the API (and J1's monthly AI cost).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::{Method, StatusCode};
use common::web::TestApp;
use sqlx::PgPool;

#[sqlx::test]
async fn the_firms_default_cut_off_offset_applies_to_clients_without_their_own(pool: PgPool) {
    let app = TestApp::new(pool);
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let shop = serde_json::json!({
        "business_name": "Shop", "contact_first_name": "Ana", "email": "a@b.test",
        "language": "english", "forms": ["2551Q"],
    });
    let client = app
        .api(Method::POST, "/api/clients", Some(&cookie), Some(shop))
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();

    let save = |days: u8| {
        let change = serde_json::json!({ "cut_off_offset_days": days });
        app.api(Method::PUT, "/api/settings", Some(&cookie), Some(change))
    };
    let saved = save(10).await;

    assert_eq!(saved.status, StatusCode::NO_CONTENT, "{}", saved.body);
    let settings = app.api(Method::GET, "/api/settings", Some(&cookie), None).await.json();
    assert_eq!(settings["cut_off_offset_days"], 10);
    // Q4 2026: 2551Q due Mon 25 Jan 2027, minus 10 days.
    let page = app
        .api(Method::GET, &format!("/api/clients/{client}"), Some(&cookie), None)
        .await
        .json();
    assert_eq!(page["periods"][0]["cut_off"], "2027-01-15");
    assert_eq!(save(31).await.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test]
async fn j1_settings_show_the_firms_ai_cost_this_month(pool: PgPool) {
    use kompleto::engine::cost_log::{self, LlmCall, Outcome};
    use kompleto::engine::llm::Usage;
    use time::macros::datetime;

    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let call = |at, input| LlmCall {
        tenant: firm,
        at,
        model: "claude-haiku-5-5".to_owned(),
        usage: Usage {
            input_tokens: input,
            output_tokens: 1_000,
        },
        request_id: None,
        outcome: Outcome::Ok,
    };
    // Today is 21 Oct 2026; September's call doesn't count.
    for record in [
        call(datetime!(2026-10-02 01:00 UTC), 20_000),
        call(datetime!(2026-10-20 01:00 UTC), 30_000),
        call(datetime!(2026-09-30 01:00 UTC), 1_000_000),
    ] {
        cost_log::record(&pool, &record).await.unwrap();
    }

    let settings = app.api(Method::GET, "/api/settings", Some(&cookie), None).await.json();

    // 50,000 × $0.10/M + 2,000 × $0.50/M = $0.006.
    let cost = settings["ai_cost"]["cost_usd"].as_f64().unwrap();
    assert!((cost - 0.006).abs() < 1e-9, "{settings}");
    assert_eq!(settings["ai_cost"]["drafts"], 2);
}
