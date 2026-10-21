//! Seam 4: client pages (B1, B4).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::web::TestApp;
use kompleto::kompleto::calendar::BirForm;
use kompleto::kompleto::clients::{Clients, Language};
use sqlx::PgPool;

const NENA_FORM: &str = "business_name=Nena%27s+Store&contact_first_name=Nena&email=nena%40example.test\
                         &language=taglish&forms=2551Q&forms=1701Q";

#[sqlx::test]
async fn b1_a_client_is_added_through_the_form(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let form_page = app.get("/clients/new", &cookie).await;
    assert!(form_page.body.contains(r#"value="2550Q""#), "{}", form_page.body);

    let added = app.post("/clients", &cookie, NENA_FORM).await;

    assert_eq!(added.status, StatusCode::SEE_OTHER, "{}", added.body);
    let clients = Clients::new(pool, app.clock.clone()).list(firm).await.unwrap();
    assert_eq!(clients.len(), 1);
    assert_eq!(added.location(), format!("/clients/{}", clients[0].id));
    assert_eq!(clients[0].details.business_name, "Nena's Store");
    assert_eq!(clients[0].details.language, Language::Taglish);
    assert_eq!(clients[0].details.forms, [BirForm::F2551Q, BirForm::F1701Q]);
}

#[sqlx::test]
async fn b1_a_form_with_problems_is_shown_again_with_every_reason(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;

    let refused = app
        .post(
            "/clients",
            &cookie,
            "business_name=Shop&contact_first_name=&email=nope&language=taglish",
        )
        .await;

    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    for reason in [
        "Contact first name: is required",
        "is not an email address",
        "Forms: tick at least one",
    ] {
        assert!(refused.body.contains(reason), "missing {reason:?} in {}", refused.body);
    }
    assert!(refused.body.contains(r#"value="Shop""#), "the typed values are kept");
    assert!(
        Clients::new(pool, app.clock.clone())
            .list(firm)
            .await
            .unwrap()
            .is_empty()
    );
}

#[sqlx::test]
async fn b4_the_client_page_lists_each_periods_owed_documents_and_cut_off(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let added = app.post("/clients", &cookie, NENA_FORM).await;

    let page = app.get(added.location(), &cookie).await;

    assert_eq!(page.status, StatusCode::OK);
    // Today is 21 Oct 2026: Q3's cut-off is today (2551Q due 26 Oct, minus 5 days).
    for expected in [
        "Q3 2026",
        "Cut-off: Wed 21 Oct 2026",
        "2551Q",
        "Mon 26 Oct 2026",
        "1701Q",
        "Mon 16 Nov 2026",
        "Sales invoices issued, including cancelled ones",
        "2307s received from customers",
        "Q4 2026",
        "Cut-off: Wed 20 Jan 2027",
    ] {
        assert!(page.body.contains(expected), "missing {expected:?} in {}", page.body);
    }
}

#[sqlx::test]
async fn b1_a_clients_details_can_be_edited(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let page = app.post("/clients", &cookie, NENA_FORM).await.location().to_owned();
    let edit = app.get(&format!("{page}/edit"), &cookie).await;
    assert!(
        edit.body.contains(r#"value="Nena&#39;s Store""#) || edit.body.contains(r#"value="Nena&#x27;s Store""#),
        "{}",
        edit.body
    );

    let saved = app
        .post(
            &page,
            &cookie,
            &NENA_FORM.replace("language=taglish", "language=english"),
        )
        .await;

    assert_eq!(saved.status, StatusCode::SEE_OTHER, "{}", saved.body);
    let client = &Clients::new(pool, app.clock.clone()).list(firm).await.unwrap()[0];
    assert_eq!(client.details.language, Language::English);
}

#[sqlx::test]
async fn d1_the_schedule_form_changes_cut_off_reminders_and_pause(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let page = app.post("/clients", &cookie, NENA_FORM).await.location().to_owned();

    let saved = app
        .post(
            &format!("{page}/schedule"),
            &cookie,
            "cut_off_offset_days=8&reminder_days=10%2C+2&paused=on",
        )
        .await;
    assert_eq!(saved.status, StatusCode::SEE_OTHER, "{}", saved.body);

    let client = &Clients::new(pool, app.clock.clone()).list(firm).await.unwrap()[0];
    assert_eq!(
        (client.cut_off_offset_days, client.reminder_days.days(), client.paused),
        (Some(8), &[10, 2][..], true)
    );
    // Q4 2026: 2551Q due 25 Jan 2027, minus 8 days.
    assert!(app.get(&page, &cookie).await.body.contains("Cut-off: Sun 17 Jan 2027"));

    let refused = app
        .post(
            &format!("{page}/schedule"),
            &cookie,
            "cut_off_offset_days=5&reminder_days=7%2C3%2C2%2C1",
        )
        .await;
    assert_eq!(refused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        refused.body.contains("choose at most 3 reminder days"),
        "{}",
        refused.body
    );
}

#[sqlx::test]
async fn b3_a_due_date_set_on_the_client_page_moves_only_that_clients_dates(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (_, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let page = app.post("/clients", &cookie, NENA_FORM).await.location().to_owned();
    let other = app
        .post("/clients", &cookie, &NENA_FORM.replace("Nena%27s", "Other"))
        .await
        .location()
        .to_owned();

    let saved = app
        .post(
            &format!("{page}/overrides"),
            &cookie,
            "form=2551Q&period=2026-Q4&due=2027-01-29",
        )
        .await;
    assert_eq!(saved.status, StatusCode::SEE_OTHER, "{}", saved.body);

    let body = app.get(&page, &cookie).await.body;
    assert!(
        body.contains("Fri 29 Jan 2027 <span class=\"muted\">(your date)</span>"),
        "{body}"
    );
    assert!(body.contains("Cut-off: Sun 24 Jan 2027"), "{body}");
    assert!(app.get(&other, &cookie).await.body.contains("Cut-off: Wed 20 Jan 2027"));

    app.post(&format!("{page}/overrides"), &cookie, "form=2551Q&period=2026-Q4&due=")
        .await;
    assert!(app.get(&page, &cookie).await.body.contains("Cut-off: Wed 20 Jan 2027"));
}

#[sqlx::test]
async fn b4_the_checklist_can_be_edited_from_the_client_page(pool: PgPool) {
    let app = TestApp::new(pool.clone());
    let (firm, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let page = app.post("/clients", &cookie, NENA_FORM).await.location().to_owned();
    let bank = "<li>Bank and e-wallet (GCash, Maya) statements</li>";
    let lazada = "<li>Lazada payout reports</li>";
    assert!(app.get(&page, &cookie).await.body.contains(bank));

    app.post(&format!("{page}/checklist/remove"), &cookie, "key=bank-statements")
        .await;
    app.post(
        &format!("{page}/checklist/add"),
        &cookie,
        "label=Lazada+payout+reports&recurs=every-period",
    )
    .await;

    let body = app.get(&page, &cookie).await.body;
    assert!(!body.contains(bank) && body.contains(lazada), "{body}");

    let clients = Clients::new(pool, app.clock.clone());
    let id = clients.list(firm).await.unwrap()[0].id;
    let (item, _) = clients.extra_items(firm, id).await.unwrap().remove(0);
    app.post(&format!("{page}/checklist/extra/{item}/delete"), &cookie, "")
        .await;
    app.post(&format!("{page}/checklist/restore"), &cookie, "key=bank-statements")
        .await;

    let body = app.get(&page, &cookie).await.body;
    assert!(body.contains(bank) && !body.contains(lazada), "{body}");
}

#[sqlx::test]
async fn h2_the_firm_lists_a_clients_payors_on_its_page(pool: PgPool) {
    let app = TestApp::new(pool);
    let (santos, cookie) = app.logged_in_firm("Santos Bookkeeping").await;
    let client = kompleto::kompleto::clients::Clients::new(app.pool.clone(), app.clock.clone())
        .create(santos, &common::nena())
        .await
        .unwrap();

    let added = app
        .post(
            &format!("/clients/{client}/payors"),
            &cookie,
            "name=Jollibee+Foods+Corp",
        )
        .await;
    let refused = app
        .post(&format!("/clients/{client}/payors"), &cookie, "name=www.pay-here.ph")
        .await;
    let page = app.get(&format!("/clients/{client}"), &cookie).await;

    assert_eq!(added.status, axum::http::StatusCode::SEE_OTHER);
    assert_eq!(refused.status, axum::http::StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        refused.body.contains("can&#39;t look like a web address")
            || refused.body.contains("can't look like a web address"),
        "{}",
        refused.body
    );
    assert!(page.body.contains("Jollibee Foods Corp"));
}
