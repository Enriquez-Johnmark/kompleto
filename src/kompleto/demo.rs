//! A demo Firm with a realistic history, for showing Kompleto to
//! bookkeepers (M8). Development only: the command refuses to run unless
//! email goes to Mailpit, and every Contact has a `.test` address, which
//! can never be delivered.
//!
//! The history is not made up row by row: the days before `as_of` are
//! replayed through the real scheduler, outbox, Upload and Reply code, so
//! the demo shows exactly what Kompleto would have done.

use std::sync::Arc;

use sqlx::PgPool;
use time::{Date, Duration, Time};

use super::calendar::{CalendarError, TaxPeriod};
use super::checklist::{ExtraItem, Recurs, is_received_2307, owed_documents, payor_2307};
use super::clients::{ClientError, ClientId, ClientInput, Clients, NewClient, ScheduleSettings};
use super::drafts::Drafts;
use super::firms::{FirmError, FirmId, Firms};
use super::payors::Payors;
use super::presend::KompletoChecks;
use super::reminders::{Scheduler, SchedulerError};
use super::replies::{Replies, ReplyError};
use super::schedule::{MANILA, ReminderDays, ScheduleError};
use super::upload_links::UploadLinks;
use super::uploads::{Label, UploadError, Uploads};
use crate::engine::clock::{Clock, FixedClock};
use crate::engine::files::Files;
use crate::engine::inbound::InboundEmail;
use crate::engine::outbox::{DrainReport, Outbox, OutboxError};
use crate::engine::webhooks::DeliveryEvent;
use crate::kompleto::bounces::{BounceError, Bounces};

/// How many days of history the demo replays before `as_of`.
pub const HISTORY_DAYS: i64 = 45;

/// The last days before `as_of` whose Drafts are left waiting.
const UNAPPROVED_DAYS: i64 = 3;

/// What could stop the demo being built.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum DemoError {
    Client(#[from] ClientError),
    Firm(#[from] FirmError),
    Scheduler(#[from] SchedulerError),
    Calendar(#[from] CalendarError),
    Outbox(#[from] OutboxError),
    Upload(#[from] UploadError),
    Reply(#[from] ReplyError),
    Bounce(#[from] BounceError),
    Schedule(#[from] ScheduleError),
    Database(#[from] sqlx::Error),
    /// A built-in demo Client didn't pass the client checks.
    #[error("demo client {0:?} was refused")]
    BadClient(String),
}

/// What the demo made.
#[derive(Debug, Default)]
pub struct DemoSummary {
    /// The demo Firm.
    pub firm: Option<FirmId>,
    pub clients: usize,
    /// Reminders approved and sent in the replayed history.
    pub sent: usize,
    /// Drafts waiting for approval on `as_of`.
    pub waiting: usize,
    pub uploads: usize,
    pub replies: usize,
}

/// Kompleto's services, all on the demo's own clock.
pub struct DemoServices {
    pub pool: PgPool,
    /// Moved forward through the replayed days.
    pub clock: Arc<FixedClock>,
    /// With reply addresses, so Replies match.
    pub links: UploadLinks,
    /// On the same clock.
    pub files: Files,
    /// Where links point, e.g. `http://127.0.0.1:3000`.
    pub base_url: String,
}

/// One demo Client and how its Contact behaves.
struct DemoClient {
    name: &'static str,
    contact: &'static str,
    email: &'static str,
    language: &'static str,
    forms: &'static [&'static str],
    payors: &'static [&'static str],
    habit: Habit,
}

/// What a Contact does a couple of days after a first request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Habit {
    /// Uploads everything, so reminders stop (G2).
    UploadsAll,
    /// Uploads the first two documents, which the Firm marks received.
    UploadsSome,
    /// Writes back, so reminders stop (G1).
    Replies,
    /// Away: an out-of-office answer, which doesn't stop reminders.
    AutoReplies,
    /// Never answers; their Drafts are left to expire now and then.
    Silent,
    /// Their address bounces (G3).
    Bounces,
}

const CLIENTS: &[DemoClient] = &[
    DemoClient {
        name: "Aling Nena's Sari-Sari Store",
        contact: "Nena",
        email: "nena.sarisari@example.test",
        language: "taglish",
        forms: &["2551Q", "1701Q"],
        payors: &[],
        habit: Habit::UploadsAll,
    },
    DemoClient {
        name: "Dr. Reyes Dental Clinic",
        contact: "Mike",
        email: "frontdesk@reyesdental.example.test",
        language: "english",
        forms: &["0619-E", "1601-EQ", "1601-C", "2551Q", "1701Q", "1604-C", "1604-E"],
        payors: &[],
        habit: Habit::UploadsSome,
    },
    DemoClient {
        name: "Lito Hardware & Construction Supply",
        contact: "Lito",
        email: "lito@litohardware.example.test",
        language: "taglish",
        forms: &[
            "0619-E", "1601-EQ", "1601-C", "2550Q", "1702Q", "1604-C", "1604-E", "1702",
        ],
        payors: &["Ayala Land Inc", "Robinsons Land Corp"],
        habit: Habit::Replies,
    },
    DemoClient {
        name: "Bautista Bakeshop",
        contact: "Carla",
        email: "carla@bautistabakeshop.example.test",
        language: "taglish",
        forms: &["1601-C", "2551Q", "1701Q", "1604-C"],
        payors: &[],
        habit: Habit::UploadsAll,
    },
    DemoClient {
        name: "Garcia Auto Supply",
        contact: "Ben",
        email: "ben.garcia@example.test",
        language: "english",
        forms: &["0619-E", "1601-EQ", "2550Q", "1702Q", "1702"],
        payors: &[],
        habit: Habit::Silent,
    },
    DemoClient {
        name: "Mendoza Beauty Salon",
        contact: "Joy",
        email: "joy@mendozasalon.example.test",
        language: "taglish",
        forms: &["2551Q", "1701Q", "1701"],
        payors: &[],
        habit: Habit::UploadsSome,
    },
    DemoClient {
        name: "Villanueva Travel & Tours",
        contact: "Rina",
        email: "rina@villanuevatravel.example.test",
        language: "english",
        forms: &["0619-E", "1601-EQ", "1601-C", "2550Q", "1702Q"],
        payors: &["SM Supermalls"],
        habit: Habit::AutoReplies,
    },
    DemoClient {
        name: "Santos Carinderia",
        contact: "Rosa",
        email: "rosa.carinderia@example.test",
        language: "taglish",
        forms: &["2551Q", "1701Q"],
        payors: &[],
        habit: Habit::Replies,
    },
    DemoClient {
        name: "Cruz Printing Services",
        contact: "Paolo",
        email: "paolo@cruzprinting.example.test",
        language: "english",
        forms: &["1601-C", "2551Q", "1701Q", "1604-C"],
        payors: &["Jollibee Foods Corp", "Globe Telecom"],
        habit: Habit::UploadsAll,
    },
    DemoClient {
        name: "Tan Electronics Trading",
        contact: "Kevin",
        email: "kevin@tanelectronics.example.test",
        language: "english",
        forms: &["0619-E", "1601-EQ", "1601-C", "2550Q", "1702Q", "1604-C", "1604-E"],
        payors: &["Meralco"],
        habit: Habit::UploadsSome,
    },
    DemoClient {
        name: "Dela Cruz Pharmacy",
        contact: "Mylene",
        email: "mylene@delacruzpharmacy.example.test",
        language: "taglish",
        forms: &["1601-C", "2550Q", "1701Q"],
        payors: &[],
        habit: Habit::Silent,
    },
    DemoClient {
        name: "Ramos Design Studio",
        contact: "Gab",
        email: "gab@ramosdesign.example.test",
        language: "english",
        forms: &["2551Q", "1701Q", "1701"],
        payors: &["Globe Telecom", "Puregold Price Club"],
        habit: Habit::UploadsSome,
    },
    DemoClient {
        name: "Aquino Rice Dealer",
        contact: "Jun",
        email: "jun.aquino@example.test",
        language: "taglish",
        forms: &["2551Q", "1701Q"],
        payors: &[],
        habit: Habit::UploadsAll,
    },
    DemoClient {
        name: "Lim Hardware Depot",
        contact: "Arnold",
        email: "arnold@limhardware.example.test",
        language: "english",
        forms: &["1601-C", "2550Q", "1702Q"],
        payors: &[],
        habit: Habit::Bounces,
    },
    DemoClient {
        name: "Fernandez Event Catering",
        contact: "Liza",
        email: "liza@fernandezcatering.example.test",
        language: "taglish",
        forms: &["1601-C", "2551Q", "1701Q"],
        payors: &[],
        habit: Habit::Silent,
    },
];

/// A one-page PDF reading "Kompleto demo document", standing in for a
/// Contact's receipts and statements.
const SAMPLE_PDF: &[u8] = b"%PDF-1.4
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/MediaBox[0 0 300 144]/Contents 4 0 R/Resources<</Font<</F1 5 0 R>>>>>>endobj
4 0 obj<</Length 54>>stream
BT /F1 18 Tf 20 70 Td (Kompleto demo document) Tj ET
endstream endobj
5 0 obj<</Type/Font/Subtype/Type1/BaseFont/Helvetica>>endobj
trailer<</Root 1 0 R>>
%%EOF
";

/// Builds the demo Firm `name`, logging in as `login_email`, with its
/// history up to `as_of` (Manila). `send` drains the outbox, as the server's
/// sender would.
pub async fn seed(
    services: &DemoServices,
    name: &str,
    login_email: &str,
    as_of: Date,
    send: impl AsyncFn() -> Result<DrainReport, OutboxError>,
) -> Result<DemoSummary, DemoError> {
    let DemoServices {
        pool,
        clock,
        links,
        files,
        base_url,
    } = services;
    let shared: Arc<dyn Clock> = clock.clone();
    let firms = Firms::new(pool.clone(), shared.clone());
    let clients = Clients::new(pool.clone(), shared.clone());
    let payors = Payors::new(pool.clone(), shared.clone());
    let outbox = Outbox::new(pool.clone(), shared.clone());
    let drafts = Drafts::new(pool.clone(), shared.clone());
    let checks = KompletoChecks::new(pool.clone(), shared.clone(), base_url, links.clone());
    let uploads = Uploads::new(pool.clone(), shared.clone(), files.clone());
    let replies = Replies::new(pool.clone(), shared.clone(), links.clone(), checks.clone(), base_url)
        .with_uploads(uploads.clone());
    let bounces = Bounces::new(pool.clone(), shared.clone());
    let scheduler = Scheduler::new(pool.clone(), shared.clone(), links.clone())?;

    // The Firm and its Clients are added on the history's first day, since a
    // Client is chased only for Periods that end after it was added.
    let start = as_of - Duration::days(HISTORY_DAYS);
    clock.advance(start.midnight().assume_offset(MANILA) - clock.now());
    let mut summary = DemoSummary::default();
    let firm = firms.create(name, login_email).await?;
    summary.firm = Some(firm);
    let mut added = Vec::with_capacity(CLIENTS.len());
    for demo in CLIENTS {
        let input = ClientInput {
            business_name: demo.name.to_owned(),
            contact_first_name: demo.contact.to_owned(),
            email: demo.email.to_owned(),
            language: demo.language.to_owned(),
            forms: demo.forms.iter().map(|form| (*form).to_owned()).collect(),
        };
        let client = NewClient::parse(&input).map_err(|_| DemoError::BadClient(demo.name.to_owned()))?;
        let id = clients.create(firm, &client).await?;
        for payor in demo.payors {
            payors.add(firm, id, payor).await?;
        }
        added.push((id, demo));
    }
    summary.clients = added.len();
    // A few Firm choices the pages show.
    let bakeshop = added[3].0;
    let item = ExtraItem {
        label: "Foodpanda and GrabFood payout reports".to_owned(),
        recurs: Recurs::EveryPeriod,
    };
    clients.add_extra_item(firm, bakeshop, &item).await?;
    let caterer = added[14].0;
    let paused = ScheduleSettings {
        cut_off_offset_days: None,
        reminder_days: ReminderDays::new(&[7, 3, 1])?,
        paused: true,
    };
    clients.set_schedule(firm, caterer, &paused).await?;

    // Replay each day: Drafts at 07:30, the Firm approves at 09:00, and the
    // Contacts act on the first requests sent two days before.
    let morning = Time::from_hms(7, 30, 0).unwrap_or(Time::MIDNIGHT);
    let mut day = start;
    while day <= as_of {
        let at = day.with_time(morning).assume_offset(MANILA);
        clock.advance(at - clock.now());
        scheduler.run_once().await?;
        if day == as_of {
            break;
        }
        clock.advance(Duration::minutes(90));
        // The last few days' Drafts wait for the bookkeeper, as they would
        // after a weekend; before that, a busy bookkeeper misses one now and
        // then, and it expires.
        let catching_up = as_of - day <= Duration::days(UNAPPROVED_DAYS);
        for (index, draft) in drafts.waiting(firm).await?.into_iter().enumerate() {
            if catching_up || index % 7 == 6 {
                continue;
            }
            outbox
                .approve(firm, draft.id, login_email, &draft.content_hash(), &checks)
                .await?;
        }
        summary.sent += send().await?.sent;
        clock.advance(Duration::hours(3));
        for (client, demo) in &added {
            let Some(period) = first_request_on(pool, firm, *client, day - Duration::days(2)).await? else {
                continue;
            };
            match demo.habit {
                Habit::UploadsAll | Habit::UploadsSome => {
                    summary.uploads += upload(
                        &clients,
                        &uploads,
                        links,
                        firm,
                        *client,
                        &period,
                        demo.habit,
                        // Checked at once, except while catching up.
                        (!catching_up).then_some(login_email),
                    )
                    .await?;
                }
                Habit::Replies | Habit::AutoReplies => {
                    let automatic = demo.habit == Habit::AutoReplies;
                    if reply(pool, &replies, firm, *client, demo, automatic).await? {
                        summary.replies += 1;
                    }
                }
                Habit::Bounces => bounce(pool, &bounces, firm, *client).await?,
                Habit::Silent => {}
            }
        }
        // Notices and anything else waiting go out too.
        summary.sent += send().await?.sent;
        day += Duration::days(1);
    }
    summary.waiting = drafts.waiting(firm).await?.len();
    Ok(summary)
}

/// The Period whose first request went to `client` on `day`, if one did.
async fn first_request_on(
    pool: &PgPool,
    firm: FirmId,
    client: ClientId,
    day: Date,
) -> Result<Option<String>, sqlx::Error> {
    let from = day.midnight().assume_offset(MANILA);
    sqlx::query_scalar!(
        "select period from upload_links
         where firm_id = $1 and client_id = $2 and created_at >= $3 and created_at < $4
         order by created_at limit 1",
        firm.as_uuid(),
        client.as_uuid(),
        from,
        from + Duration::days(1),
    )
    .fetch_optional(pool)
    .await
}

/// A customer whose 2307 a Client without listed Payors uploads.
fn customer(business_name: &str) -> &'static str {
    const CUSTOMERS: [&str; 3] = ["Puregold Price Club", "Mercury Drug Corp", "Robinsons Supermarket"];
    CUSTOMERS[business_name.len() % CUSTOMERS.len()]
}

/// The Contact uploads every Owed document, or the first two; `received_by`,
/// if given, marks them received at once. Returns how many were uploaded.
#[expect(clippy::too_many_arguments, reason = "one call site, in the demo's replay")]
async fn upload(
    clients: &Clients,
    uploads: &Uploads,
    links: &UploadLinks,
    firm: FirmId,
    client: ClientId,
    period: &str,
    habit: Habit,
    received_by: Option<&str>,
) -> Result<usize, DemoError> {
    let Some(link) = links.find(firm, client, period).await? else {
        return Ok(0);
    };
    let Some(month) = TaxPeriod::parse_key(period).map(|p| p.last_month().1) else {
        return Ok(0);
    };
    let details = clients.get(firm, client).await?.details;
    let edits = clients.checklist_edits(firm, client).await?;
    let owed = owed_documents(&details.forms, month, &edits);
    let count = if habit == Habit::UploadsAll {
        owed.len()
    } else {
        2.min(owed.len())
    };
    for document in owed.iter().take(count) {
        // A listed Payor's 2307 names it; the general one names a customer.
        let payor = is_received_2307(document).then(|| {
            edits
                .payors
                .iter()
                .find(|payor| payor_2307(payor) == *document)
                .cloned()
                .unwrap_or_else(|| customer(&details.business_name).to_owned())
        });
        let label = Label::Owed {
            document: document.clone(),
            payor,
        };
        let id = uploads.add(&link, &owed, label, SAMPLE_PDF).await?;
        if let Some(by) = received_by {
            uploads.mark_received(firm, client, id, by).await?;
        }
    }
    Ok(count)
}

/// The Contact writes back to the latest Reminder; true if it was taken in.
async fn reply(
    pool: &PgPool,
    replies: &Replies,
    firm: FirmId,
    client: ClientId,
    demo: &DemoClient,
    automatic: bool,
) -> Result<bool, DemoError> {
    let Some(reply_to) = latest_sent(pool, firm, client)
        .await?
        .and_then(|(reply_to, _)| reply_to)
    else {
        return Ok(false);
    };
    let (subject, text) = if automatic {
        (
            "Out of office",
            format!(
                "Hi! I'm away until next week with limited access to email. — {}",
                demo.contact
            ),
        )
    } else if demo.language == "taglish" {
        (
            "Re: Mga dokumento",
            "Hello po! Pasensya na po, medyo busy sa store. Ipapadala ko po bukas ang mga resibo at bank statement. Salamat po!".to_owned(),
        )
    } else {
        (
            "Re: Documents",
            "Hi! Sorry for the delay. I'll send the receipts and the bank statement tomorrow. Thanks!".to_owned(),
        )
    };
    let email = InboundEmail {
        key: format!("demo-{}-{reply_to}", client.as_uuid()),
        from: demo.email.to_owned(),
        recipients: vec![reply_to],
        subject: subject.to_owned(),
        text,
        automatic,
        attachments: Vec::new(),
    };
    replies.receive(&email).await?;
    Ok(true)
}

/// The Client's address bounces: the provider reports its latest Reminder.
async fn bounce(pool: &PgPool, bounces: &Bounces, firm: FirmId, client: ClientId) -> Result<(), DemoError> {
    if let Some((_, Some(provider_message_id))) = latest_sent(pool, firm, client).await? {
        let event = DeliveryEvent::Bounced {
            provider_message_id,
            permanent: true,
            reason: "550 5.1.1 The email account that you tried to reach does not exist".to_owned(),
        };
        bounces.record(&event).await?;
    }
    Ok(())
}

/// The reply address and provider ID of the latest Reminder sent to `client`.
async fn latest_sent(
    pool: &PgPool,
    firm: FirmId,
    client: ClientId,
) -> Result<Option<(Option<String>, Option<String>)>, sqlx::Error> {
    let row = sqlx::query!(
        "select m.reply_to, m.provider_message_id from outbox_messages m
         join scheduled_drafts s on s.message_id = m.id and s.firm_id = m.tenant_id
         where s.firm_id = $1 and s.client_id = $2 and m.status = 'sent'
         order by m.sent_at desc limit 1",
        firm.as_uuid(),
        client.as_uuid(),
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| (row.reply_to, row.provider_message_id)))
}
