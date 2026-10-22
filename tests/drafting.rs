//! Seam 2 (M3): drafting with a fake model (D5, D6, J1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::time::Duration;

use common::clock;
use kompleto::engine::ids::TenantId;
use kompleto::engine::llm::AnthropicClient;
use kompleto::kompleto::clients::Language;
use kompleto::kompleto::drafting::{DraftBrief, DraftSource, Drafter, Purpose, Tone, template};
use secrecy::SecretString;
use serde_json::{Value, json};
use sqlx::PgPool;
use time::macros::date;
use wiremock::{Mock, MockServer, ResponseTemplate};

fn brief() -> DraftBrief {
    DraftBrief {
        business_name: "Nena's Store".to_owned(),
        contact_first_name: "Nena".to_owned(),
        owed: vec![
            "Sales invoices issued, including cancelled ones".to_owned(),
            "2307s received from customers".to_owned(),
        ],
        period: "Q3 2026".to_owned(),
        purpose: Purpose::BirFilings,
        cut_off: date!(2026 - 10 - 21),
        language: Language::Taglish,
        tone: Tone::Firmer,
        upload_link: "http://kompleto.test/u/0123abcd".to_owned(),
    }
}

fn answer(text: Value) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .insert_header("request-id", "req_42")
        .set_body_json(json!({
            "id": "msg_1", "type": "message", "role": "assistant", "model": "claude-haiku-5-5",
            "content": [{"type": "text", "text": text.to_string()}],
            "stop_reason": "end_turn", "usage": {"input_tokens": 300, "output_tokens": 80}
        }))
}

async fn drafter_answering(pool: &PgPool, response: ResponseTemplate) -> (Drafter<AnthropicClient>, MockServer) {
    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::method("POST"))
        .respond_with(response)
        .mount(&server)
        .await;
    let model = AnthropicClient::new(
        &server.uri(),
        SecretString::from("k"),
        "claude-haiku-5-5",
        Duration::from_millis(500),
    )
    .unwrap();
    (Drafter::new(pool.clone(), clock(), Some(model)), server)
}

fn good_wording() -> Value {
    json!({
        "subject": "Paalala po: kulang pa ang Q3 documents",
        "opening": "Hi Nena! Paalala lang po, kailangan pa namin ang mga ito para sa Q3 filing ninyo:",
        "closing": "Salamat po sa tulong!"
    })
}

#[sqlx::test]
async fn d5_valid_model_wording_gets_the_exact_facts_inserted_by_kompleto(pool: PgPool) {
    let (drafter, _server) = drafter_answering(&pool, answer(good_wording())).await;

    let draft = drafter.draft(TenantId::new(), "Santos Bookkeeping", &brief()).await;

    assert_eq!(draft.source, DraftSource::Ai);
    assert_eq!(draft.subject, "Paalala po: kulang pa ang Q3 documents");
    for expected in [
        "Hi Nena! Paalala lang po",
        "- Sales invoices issued, including cancelled ones\n- 2307s received from customers",
        "Wed 21 Oct 2026",
        "I-upload po dito: http://kompleto.test/u/0123abcd",
        "Salamat po sa tulong!",
        "Santos Bookkeeping",
    ] {
        assert!(draft.body.contains(expected), "missing {expected:?} in {}", draft.body);
    }
}

#[sqlx::test]
async fn d5_anything_but_valid_wording_gives_a_template_labelled_as_one(pool: PgPool) {
    let mut with_link = good_wording();
    with_link["closing"] = json!("I-upload po dito: https://evil.example/upload");
    let mut with_amount = good_wording();
    with_amount["opening"] = json!("Hi Nena, may balance po kayong ₱5,000.");
    let mut with_tin = good_wording();
    with_tin["opening"] = json!("Hi Nena, TIN 123-456-789-000 po ba ito?");
    let mut bare_domain = good_wording();
    bare_domain["closing"] = json!("I-send po sa gcash-pay.ph/upload, salamat!");
    let mut long_subject = good_wording();
    long_subject["subject"] = json!("x".repeat(121));
    let cases = [
        ("not JSON", ResponseTemplate::new(200).set_body_json(json!({"type": "message", "model": "m", "content": [{"type": "text", "text": "Sure! Here you go"}], "stop_reason": "end_turn", "usage": {"input_tokens": 1, "output_tokens": 1}}))),
        ("missing field", answer(json!({"subject": "Hi"}))),
        ("a link", answer(with_link)),
        ("a bare domain", answer(bare_domain)),
        ("a peso amount", answer(with_amount)),
        ("a TIN", answer(with_tin)),
        ("a long subject", answer(long_subject)),
        ("a server error", ResponseTemplate::new(500)),
        ("a timeout", answer(good_wording()).set_delay(Duration::from_secs(2))),
    ];
    for (case, response) in cases {
        let (drafter, _server) = drafter_answering(&pool, response).await;

        let draft = drafter.draft(TenantId::new(), "Santos Bookkeeping", &brief()).await;

        assert_eq!(draft.source, DraftSource::Template, "{case}");
        assert!(
            draft
                .body
                .contains("- 2307s received from customers\n\nI-upload po dito: http://kompleto.test/u/0123abcd"),
            "{case}: {}",
            draft.body
        );
    }
}

#[sqlx::test]
async fn d5_no_api_key_means_every_draft_is_a_template(pool: PgPool) {
    let drafter: Drafter<AnthropicClient> = Drafter::new(pool, clock(), None);

    assert_eq!(
        drafter
            .draft(TenantId::new(), "Santos Bookkeeping", &brief())
            .await
            .source,
        DraftSource::Template
    );
}

#[sqlx::test]
async fn d6_requests_carry_only_the_allowed_fields_and_nothing_sensitive(pool: PgPool) {
    let (drafter, server) = drafter_answering(&pool, answer(good_wording())).await;
    let mut tin_in_name = brief();
    tin_in_name.business_name = "Store 123456789000".to_owned();
    let mut amount_in_item = brief();
    amount_in_item.owed.push("Receipt for ₱12,500 rent".to_owned());

    drafter
        .draft(TenantId::new(), "Santos Bookkeeping 987654321", &brief())
        .await;
    let blocked_name = drafter.draft(TenantId::new(), "Santos Bookkeeping", &tin_in_name).await;
    let blocked_item = drafter
        .draft(TenantId::new(), "Santos Bookkeeping", &amount_in_item)
        .await;

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1, "requests with sensitive-looking data are never sent");
    assert_eq!(
        (blocked_name.source, blocked_item.source),
        (DraftSource::Template, DraftSource::Template)
    );
    for request in requests {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        let text = body.to_string();
        assert!(!text.contains("Santos"), "the firm name stays out of the request");
        assert!(!text.contains('₱') && !text.contains("987654321"), "{text}");
        assert!(!text.contains("/u/"), "the upload link stays out of the request");
        let user: Value = serde_json::from_str(body["messages"][0]["content"].as_str().unwrap()).unwrap();
        let mut fields: Vec<_> = user.as_object().unwrap().keys().cloned().collect();
        fields.sort();
        assert_eq!(
            fields,
            [
                "business_name",
                "contact_first_name",
                "cut_off",
                "language",
                "owed_documents",
                "period",
                "tone"
            ]
        );
    }
}

#[sqlx::test]
async fn j1_each_model_call_is_recorded_with_model_tokens_cost_and_request_id(pool: PgPool) {
    let (drafter, _server) = drafter_answering(&pool, answer(good_wording())).await;
    let tenant = TenantId::new();

    drafter.draft(tenant, "Santos Bookkeeping", &brief()).await;

    let (model, input, output, cost, request_id, outcome): (String, i32, i32, Option<i64>, Option<String>, String) =
        sqlx::query_as("select model, input_tokens, output_tokens, cost_nano_usd, request_id, outcome from llm_calls where tenant_id = $1")
            .bind(tenant.as_uuid())
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((model.as_str(), input, output), ("claude-haiku-5-5", 300, 80));
    assert_eq!(cost, Some(300 * 100 + 80 * 500));
    assert_eq!((request_id.as_deref(), outcome.as_str()), (Some("req_42"), "ok"));
}

#[test]
fn p1_p2_payroll_templates_say_what_the_documents_are_for() {
    let payroll = DraftBrief {
        owed: vec!["Timesheets or DTRs".to_owned()],
        period: "payroll on Thu 15 Oct 2026".to_owned(),
        purpose: Purpose::Payroll,
        language: Language::English,
        ..brief()
    };
    let receipts = DraftBrief {
        period: "September 2026 SSS, PhilHealth and Pag-IBIG contributions".to_owned(),
        purpose: Purpose::Contributions,
        language: Language::Taglish,
        ..brief()
    };

    let payroll = template("Santos Bookkeeping", &payroll);
    let receipts = template("Santos Bookkeeping", &receipts);

    assert!(
        payroll
            .subject
            .starts_with("Documents for the payroll on Thu 15 Oct 2026"),
        "{}",
        payroll.subject
    );
    assert!(
        payroll
            .body
            .contains("For the payroll on Thu 15 Oct 2026, please send us these"),
        "{}",
        payroll.body
    );
    assert!(!payroll.body.contains("BIR"), "{}", payroll.body);
    assert!(
        receipts.body.contains(
            "Para sa September 2026 SSS, PhilHealth and Pag-IBIG contributions ninyo, paki-send po ang mga resibo"
        ),
        "{}",
        receipts.body
    );
}
