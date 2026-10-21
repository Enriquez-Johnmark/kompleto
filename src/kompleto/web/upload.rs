//! The upload page (F1, F2): what a Contact opens from a Reminder, on a
//! phone, without logging in. The link itself is the credential.
//!
//! - Fixed text in the Client's reminder language; the model writes none of it.
//! - It lists the Period's Owed documents and whether each has arrived. The
//!   Contact can't see, download or delete what was uploaded, so a
//!   forwarded link gives nothing away.
//! - One file per request (the page's script sends several in turn), at
//!   most [`UPLOAD_SLOTS`] processed at once, each request at most 10 MB.
//! - Pages carry `noindex`, and a `same-origin` referrer policy so the link
//!   never reaches another site. Not `no-referrer`: with it, browsers send
//!   `Origin: null` on the page's own uploads, and the same-origin check
//!   refuses them.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::multipart::MultipartError;
use axum::extract::{DefaultBodyLimit, Multipart, Path, Query, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use serde::Deserialize;

use super::{AppState, NotFoundPage, WebError, show_date};
use crate::kompleto::calendar::TaxPeriod;
use crate::kompleto::checklist::{ANOTHER_PAYOR_2307, owed_documents, payor_2307};
use crate::kompleto::clients::{Client, Language};
use crate::kompleto::upload_links::{LinkError, OpenLink};
use crate::kompleto::uploads::{Label, LabelProblem, MAX_FILE_BYTES, UploadError, is_2307, uploaded_documents};

/// How many uploads are processed at once.
pub const UPLOAD_SLOTS: usize = 4;
/// How long one upload may take to arrive: 10 MB at about 0.7 Mbit/s.
const UPLOAD_TIME_LIMIT: std::time::Duration = std::time::Duration::from_secs(120);

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/u/{token}", get(page).post(upload))
        .layer(DefaultBodyLimit::max(MAX_FILE_BYTES + 64 * 1024))
        .layer(middleware::from_fn(private))
}

/// Keeps upload pages out of search engines, caches and other sites' logs.
async fn private(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-robots-tag", HeaderValue::from_static("noindex, nofollow"));
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

/// The page's fixed wording, in one language.
struct Words {
    lang: &'static str,
    title: &'static str,
    uploaded: &'static str,
    not_yet: &'static str,
    what: &'static str,
    something_else: &'static str,
    note: &'static str,
    payor: &'static str,
    file: &'static str,
    button: &'static str,
    sending: &'static str,
    sent: &'static str,
}

const TAGLISH: Words = Words {
    lang: "fil",
    title: "Mag-upload ng documents",
    uploaded: "Na-upload na",
    not_yet: "Wala pa",
    what: "Ano po ang file na ito?",
    something_else: "Iba pa",
    note: "Kung iba pa, ano po ito?",
    payor: "Kung 2307, kanino pong business galing?",
    file: "Piliin ang photo o PDF (hanggang 10 MB bawat isa)",
    button: "I-upload",
    sending: "Ina-upload…",
    sent: "Salamat po! Natanggap na namin ang file ninyo.",
};

const ENGLISH: Words = Words {
    lang: "en",
    title: "Upload your documents",
    uploaded: "Uploaded",
    not_yet: "Not yet",
    what: "What is this file?",
    something_else: "Something else",
    note: "If something else, what is it?",
    payor: "If a 2307, which business is it from?",
    file: "Choose photos or PDFs (up to 10 MB each)",
    button: "Upload",
    sending: "Uploading…",
    sent: "Thank you! We've received your file.",
};

fn words(language: Language) -> &'static Words {
    match language {
        Language::Taglish => &TAGLISH,
        Language::English => &ENGLISH,
    }
}

/// One Owed document on the page.
struct Item {
    index: usize,
    label: String,
    uploaded: bool,
}

#[derive(Template, WebTemplate)]
#[template(path = "upload.html")]
struct UploadPage {
    w: &'static Words,
    intro: String,
    items: Vec<Item>,
    /// The choice for a 2307 from a business not on the Payor list.
    another: Option<Item>,
    asks_payor: bool,
    error: Option<String>,
    sent: bool,
}

#[derive(Template, WebTemplate)]
#[template(path = "upload_expired.html")]
struct ExpiredPage {
    w: &'static Words,
    message: String,
}

#[derive(Deserialize)]
struct Sent {
    sent: Option<u8>,
}

/// What a link opens: whose it is, and what they owe.
struct Opened {
    link: OpenLink,
    firm_name: String,
    client: Client,
    owed: Vec<String>,
    /// The Client's Payors (H2).
    payors: Vec<String>,
    /// What an upload can be labelled: the Owed documents, then, when
    /// Payors' 2307s are owed, one from another business.
    choices: Vec<String>,
}

impl Opened {
    /// The Payor of a listed Payor's 2307, which the Contact needn't type.
    fn listed_payor(&self, document: &str) -> Option<&String> {
        self.payors.iter().find(|payor| payor_2307(payor) == document)
    }
}

/// Why a link didn't open.
enum Closed {
    NotFound,
    Expired(ExpiredPage),
    Failed(WebError),
}

impl IntoResponse for Closed {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, NotFoundPage).into_response(),
            Self::Expired(page) => (StatusCode::GONE, page).into_response(),
            Self::Failed(err) => err.into_response(),
        }
    }
}

async fn open(state: &AppState, token: &str) -> Result<Opened, Closed> {
    let link = match state.links.open(token).await {
        Ok(link) => link,
        Err(LinkError::NotFound) => return Err(Closed::NotFound),
        Err(LinkError::Expired(link)) => {
            let opened = details(state, link).await.map_err(Closed::Failed)?;
            let message = match opened.client.details.language {
                Language::Taglish => format!(
                    "Expired na po ang link na ito. Pakihingi po ng bago sa {}.",
                    opened.firm_name
                ),
                Language::English => format!("This link has expired. Please ask {} for a new one.", opened.firm_name),
            };
            return Err(Closed::Expired(ExpiredPage {
                w: words(opened.client.details.language),
                message,
            }));
        }
        Err(LinkError::Database(err)) => return Err(Closed::Failed(WebError::internal(err))),
    };
    details(state, link).await.map_err(Closed::Failed)
}

async fn details(state: &AppState, link: OpenLink) -> Result<Opened, WebError> {
    let firm_name = state.firms.get(link.firm).await?.name;
    let client = state.clients.get(link.firm, link.client).await?;
    let edits = state.clients.checklist_edits(link.firm, link.client).await?;
    let (_, month) = TaxPeriod::parse_key(&link.period)
        .ok_or_else(|| WebError::internal(format!("bad period key {:?}", link.period)))?
        .last_month();
    let owed = owed_documents(&client.details.forms, month, &edits);
    let mut choices = owed.clone();
    if edits.payors.iter().any(|payor| owed.contains(&payor_2307(payor))) {
        choices.push(ANOTHER_PAYOR_2307.to_owned());
    }
    Ok(Opened {
        link,
        firm_name,
        client,
        owed,
        payors: edits.payors,
        choices,
    })
}

async fn render(state: &AppState, opened: &Opened, error: Option<String>, sent: bool) -> Result<UploadPage, WebError> {
    let uploads = state
        .uploads
        .for_period(opened.link.firm, opened.link.client, &opened.link.period)
        .await
        .map_err(WebError::internal)?;
    let uploaded = |document: &str| {
        uploads
            .iter()
            .any(|upload| matches!(&upload.label, Label::Owed { document: d, .. } if d == document))
    };
    let cut_off = show_date(opened.link.cut_off);
    let intro = match opened.client.details.language {
        Language::Taglish => format!("Kailangan po ng {} ang mga ito bago {cut_off}:", opened.firm_name),
        Language::English => format!("{} needs these by {cut_off}:", opened.firm_name),
    };
    Ok(UploadPage {
        w: words(opened.client.details.language),
        intro,
        items: opened
            .owed
            .iter()
            .enumerate()
            .map(|(index, label)| Item {
                index,
                label: label.clone(),
                uploaded: uploaded(label),
            })
            .collect(),
        another: opened.choices.get(opened.owed.len()).map(|label| Item {
            index: opened.owed.len(),
            label: label.clone(),
            uploaded: false,
        }),
        asks_payor: opened
            .choices
            .iter()
            .any(|document| is_2307(document) && opened.listed_payor(document).is_none()),
        error,
        sent,
    })
}

async fn page(State(state): State<AppState>, Path(token): Path<String>, Query(query): Query<Sent>) -> Response {
    let opened = match open(&state, &token).await {
        Ok(opened) => opened,
        Err(closed) => return closed.into_response(),
    };
    render(&state, &opened, None, query.sent.is_some())
        .await
        .into_response()
}

async fn upload(State(state): State<AppState>, Path(token): Path<String>, multipart: Multipart) -> Response {
    // The link is checked before taking a slot, so bad links can't take them.
    let opened = match open(&state, &token).await {
        Ok(opened) => opened,
        Err(closed) => return closed.into_response(),
    };
    let Ok(_slot) = state.upload_slots.acquire().await else {
        return WebError::Internal.into_response();
    };
    // A sender that trickles its file can't keep a slot for long.
    let form = match tokio::time::timeout(UPLOAD_TIME_LIMIT, read_form(multipart)).await {
        Ok(Ok(form)) => form,
        Ok(Err(err)) => return refusal(&state, &opened, &err).await,
        Err(_) => return refusal(&state, &opened, &UploadError::Incomplete).await,
    };
    let label = match form.document.as_str() {
        "other" => Label::SomethingElse { note: form.note },
        index => match index.parse::<usize>().ok().and_then(|i| opened.choices.get(i)) {
            Some(document) => Label::Owed {
                document: document.clone(),
                payor: match opened.listed_payor(document) {
                    Some(payor) => Some(payor.clone()),
                    None => is_2307(document).then_some(form.payor),
                },
            },
            None => Label::Owed {
                document: String::new(),
                payor: None,
            },
        },
    };
    match state
        .uploads
        .add(&opened.link, &opened.choices, label, &form.file)
        .await
    {
        Ok(_) => {
            stop_if_complete(&state, &opened).await;
            Redirect::to(&format!("/u/{token}?sent=1")).into_response()
        }
        Err(UploadError::File(err)) => WebError::internal(err).into_response(),
        Err(UploadError::Database(err)) => WebError::internal(err).into_response(),
        Err(err) => refusal(&state, &opened, &err).await,
    }
}

/// Once every Owed document has arrived, cancels the Period's waiting
/// reminders (G2). A failure is logged; the scheduler's next run retries.
async fn stop_if_complete(state: &AppState, opened: &Opened) {
    let link = &opened.link;
    let uploaded = match uploaded_documents(&state.pool, link.firm, link.client, &link.period).await {
        Ok(uploaded) => uploaded,
        Err(err) => {
            tracing::error!(firm = %link.firm, %err, "checking what's still owed failed");
            return;
        }
    };
    if !opened.owed.iter().all(|document| uploaded.contains(document)) {
        return;
    }
    match state.drafts.cancel_waiting(link.firm, link.client, &link.period).await {
        Ok(0) => {}
        Ok(cancelled) => tracing::info!(firm = %link.firm, cancelled, "everything uploaded; reminders stopped"),
        Err(err) => tracing::error!(firm = %link.firm, %err, "stopping reminders after an upload failed"),
    }
}

/// The page again, with what went wrong.
async fn refusal(state: &AppState, opened: &Opened, err: &UploadError) -> Response {
    let text = message(opened.client.details.language, err, &opened.firm_name);
    match render(state, opened, Some(text), false).await {
        Ok(page) => (err.status(), page).into_response(),
        Err(err) => err.into_response(),
    }
}

/// The upload form's fields.
#[derive(Default)]
struct UploadForm {
    document: String,
    payor: String,
    note: String,
    file: Vec<u8>,
}

async fn read_form(mut multipart: Multipart) -> Result<UploadForm, UploadError> {
    let mut form = UploadForm::default();
    // Only the size limit means "too big"; any other broken body is incomplete.
    let broken = |err: MultipartError| {
        if err.status() == StatusCode::PAYLOAD_TOO_LARGE {
            UploadError::TooBig
        } else {
            UploadError::Incomplete
        }
    };
    while let Some(field) = multipart.next_field().await.map_err(broken)? {
        let name = field.name().unwrap_or_default().to_owned();
        let bytes = field.bytes().await.map_err(broken)?;
        let text = || String::from_utf8_lossy(&bytes).into_owned();
        match name.as_str() {
            "document" => form.document = text(),
            "payor" => form.payor = text(),
            "note" => form.note = text(),
            "file" => form.file = bytes.to_vec(),
            _ => {}
        }
    }
    Ok(form)
}

impl UploadError {
    fn status(&self) -> StatusCode {
        match self {
            Self::NotAllowedType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::TooBig => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Incomplete => StatusCode::BAD_REQUEST,
            Self::LinkFull => StatusCode::TOO_MANY_REQUESTS,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::NoFile | Self::Invalid(_) | Self::File(_) | Self::Database(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

/// What went wrong, in the Contact's language.
fn message(language: Language, err: &UploadError, firm_name: &str) -> String {
    let taglish = language == Language::Taglish;
    let text = match err {
        UploadError::NotAllowedType if taglish => "Photo o PDF lang po ang puwedeng i-upload.",
        UploadError::NotAllowedType => "Only photos and PDF files can be uploaded.",
        UploadError::TooBig if taglish => "Hanggang 10 MB lang po bawat file.",
        UploadError::TooBig => "Each file must be 10 MB or less.",
        UploadError::NoFile if taglish => "Pumili po muna ng file.",
        UploadError::NoFile => "Please choose a file first.",
        UploadError::Incomplete if taglish => "Hindi po buo ang dumating na file. Pakisubukan po ulit.",
        UploadError::Incomplete => "The file didn't arrive complete. Please try again.",
        UploadError::LinkFull if taglish => {
            return format!("Puno na po ang link na ito. Pakisabihan po ang {firm_name}.");
        }
        UploadError::LinkFull => return format!("This link can't take any more files. Please tell {firm_name}."),
        UploadError::Invalid(problem) => match (problem, taglish) {
            (LabelProblem::NotOwed, true) => "Pakipili po kung ano ang file na ito.",
            (LabelProblem::NotOwed, false) => "Please choose what this file is.",
            (LabelProblem::NoPayor, true) => "Pakisulat po kung aling business ang nagbigay ng 2307.",
            (LabelProblem::NoPayor, false) => "Please say which business the 2307 is from.",
            (LabelProblem::NoNote, true) => "Pakisulat po kung ano ang file na ito.",
            (LabelProblem::NoNote, false) => "Please say what this file is.",
            (LabelProblem::TooLong, true) => "Paikliin po sa isang linya.",
            (LabelProblem::TooLong, false) => "Please keep it to one short line.",
        },
        UploadError::NotFound | UploadError::File(_) | UploadError::Database(_) => {
            "Something went wrong. Please try again."
        }
    };
    text.to_owned()
}
