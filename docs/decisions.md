# Decision log

Every owner-approved decision, newest first, with the date it was made. Decisions that a later one replaced stay here, marked **superseded**, so the reasoning trail survives. `AGENTS.md` lists only the decisions in force.

## 2026-10-22

- **M6 round.** The owner accepted the recommendations ("go ahead"); whether they hold a bank-issued card (question 1) is still to be answered before deployment:
  - **hosting at ₱0 for the pilots:** Oracle Cloud Always Free in Singapore if the owner has a bank-issued card, otherwise the owner's Mac behind a free Cloudflare Tunnel; the paid Singapore VPS (≈ ₱410/month) once the first Firm pays. Supersedes "paid hosting starts at M6" (`docs/research/free-hosting.md`);
  - **backups:** nightly, encrypted, to the owner's Google Drive (free 15 GB), kept 30 days;
  - **domain:** getkompleto.com at Cloudflare Registrar, paid through PayPal with a Maya virtual card (tested with a small amount first), bought just before pilots;
  - **sending:** Resend Free with the daily budget of 90, nearest Cut-offs first; the Amazon SES plan is dropped (its free tier closed to new accounts on 21 Jul 2026); Resend Pro from the first payment if 90 a day isn't enough (`docs/research/domain-and-email-at-zero.md`);
  - **receiving:** Cloudflare Email Routing and a free Email Worker POST each reply to the signed endpoint; replies stay off Resend's quota;
  - **export (I1):** one "Download all data" zip from Settings: CSVs of Clients, Payors, checklist items, messages, Uploads, Replies and the activity log, plus the files still held; each export is logged;
  - **closing (I2):** "Close account" with the Firm's name typed to confirm; sending stops, waiting Drafts are cancelled and sessions end at once; everything is deleted after 7 days; backups age out by day 30;
  - **activity log (J2):** also records Replies, logins and Client, Payor and checklist changes; an "Activity" page shows the Firm its last 90 days;
  - **hardening:** rate limits on login links and uploads, security headers, a health check for a free uptime monitor, a non-admin service account, Postgres reachable only locally;
  - **order:** export, closing, activity log, send budget and hardening first, locally at ₱0; hosting, domain and email last, once the domain is bought;
  - **each Firm sending from its own Gmail:** not now; kept in the research note.

## 2026-10-21

- **After the M5 code review.** The owner accepted four recommendations ("go ahead"):
  - **only a signed reply stops Reminders:** mail matched by the sender's address alone (no reply code) is still kept, shown and announced to the Firm, but doesn't stop Reminders, because a sender address can be forged. Refines the M5 round's "a human Reply cancels the Period's waiting Reminders";
  - **firm-isolation exceptions** gain the inbound-email and delivery-event handlers, which find the Firm from a signed code or the provider's message ID before any Firm is known;
  - **M5 is done** once the first item is built;
  - **M6 starts with a grilling round**, at ₱0 until the agreed domain purchase.
- **M5 round.** The owner accepted all 14 recommendations ("go ahead"):
  - the app gets a secured endpoint for raw inbound email now; which provider delivers it (Cloudflare Email Routing or Resend inbound) is chosen at M6;
  - each Reminder's reply address carries a signed code naming its Client and Period; mail without a code matches only when exactly one Client has the sender's address; unmatched mail keeps no content;
  - generous auto-reply detection (headers and subject prefixes); auto-replies never stop Reminders;
  - a human Reply cancels the Period's waiting Reminders and shows on the dashboard and Client page;
  - the Firm gets a notification email with one link to read the Reply in Kompleto, not a copy of the Contact's text (changes "forwards a copy");
  - photo and PDF attachments in Replies are kept as Uploads labelled "Sent by email", under the upload rules;
  - Reply text is deleted after 90 days;
  - Resend's signed webhook reports Bounces and complaints, matched to messages we sent; a permanent Bounce or a complaint stops all Reminders to that address until the Firm changes it;
  - the home page becomes the dashboard (H1);
  - a Payor list per Client, picked on the upload page; one Owed document per missing Payor; a quarterly tracker page (H2);
  - the M5 test seams; `mail-parser` 0.11 and `base64` 0.22.

  Recorded in full in `docs/product/p1-spec.md` and `docs/architecture.md`. Sources: `docs/research/inbound-email.md`.
- **After M4.** The owner accepted eight recommendations ("go ahead"):
  - **budget ₱0 until pilots** (the owner has no money to spend for now). Supersedes "~₱600/month until a business pays". The first spend is the domain, just before pilots; the first ₱1,500 payment from a Firm covers running costs after that;
  - **the domain waits:** getkompleto.com is bought just before M6 (hosting and real email), not before 1 Nov 2026, accepting the .com price rise and the small risk of losing the name (renaming is a configuration change plus docs). Supersedes the "buy before 1 Nov" plan in the P1 spec round 3;
  - **no subscription for the app's AI:** a Claude subscription doesn't include API use, and the consumer terms forbid automated access without an API key (Consumer Terms §3(7), effective 8 Oct 2025). The API key stays optional and Drafts stay templates until there is one;
  - **the Taglish test runs by hand** in claude.ai on the owner's existing subscription, with fictional clients only; Claude prepares the kit and shuffles the answers for a blind rating (`prototypes/taglish-test/`);
  - **keep building M5 and M6 at no cost** (template Drafts, local Mailpit); before pilots, research the cheapest domain, email and hosting set-up, free tiers first;
  - **`same-origin` referrer policy** on the upload page instead of `no-referrer` (found in the M4 code review: browsers send `Origin: null` under `no-referrer`, so every upload was refused);
  - **M4 is done;**
  - **a development command** that runs the scheduler as if it were a chosen date, so the whole loop can be tried before 1 Nov; then **M5 starts with a grilling round**.
- **M4 round.** The owner accepted all 15 recommendations ("go ahead"):
  - one Upload link per Client per Period, created with the Period's first Draft; the link is a record ID plus a 128-bit HMAC signature, so the database holds no usable link; it works until 30 days after the Cut-off;
  - Kompleto inserts the link into every Reminder and the Viber text; the pre-send check requires exactly that link;
  - the upload page: fixed text in the Client's language, Owed documents with their status, no viewing or deleting uploads; each Upload labelled as one Owed document (or "Something else" with a note), with a typed Payor name for 2307s until M5;
  - JPEG, PNG, HEIC/HEIF, WebP and PDF up to 10 MB, detected by content and never decoded; one file per request, at most 4 at a time; at most 60 files and 200 MB per link; `noindex` and `no-referrer`;
  - a per-file key wrapped by `FILE_MASTER_KEY` (`kompleto gen-key`), bound to the file and Firm; files on local disk behind an engine `FileStore` until R2 at M6, never in Postgres;
  - Download and Mark received on the Client page; an hourly sweep deletes files 30 days after received or 90 days after upload, audited; generated download names;
  - G2 brought into M4: Reminders list only Owed documents not yet uploaded and stop when none are left;
  - six M4 test seams; no virus scanning in M4 (revisit before paid pilots).

  Recorded in full in `docs/product/p1-spec.md` and `docs/architecture.md`.
- **After M3.** The owner accepted four recommendations ("go ahead"):
  - keep the rules in `AGENTS.md`, with `CLAUDE.md` importing it (one copy, readable by other coding tools too);
  - `kompleto llm-costs` may read every Firm's AI costs: it is an operator command, never reachable from a Firm's pages. Added to the Firm isolation rule in `AGENTS.md`;
  - M3 is done;
  - M4 starts with a grilling round.
- **M3 round.** The owner accepted all 14 recommendations ("go ahead"):
  - six M3 test seams (`docs/architecture.md`);
  - the API key is optional (no key: every Draft is a template); the blind Taglish test runs once Anthropic credits are bought;
  - Haiku 5.5, thinking off, JSON-schema output, 15-second timeout, no retry; the model writes only a subject, opening and closing, while Kompleto inserts the Owed documents, Cut-off, firm name and (M4) upload link;
  - tone by reminder, and the rules for invalid model output;
  - a Drafts page with approve, edit and discard; batch approval after reviewing every Draft in full; expiry at the end of the Cut-off day;
  - pre-send checks at approval and at sending, supplied by the product through an engine interface;
  - "copy for Viber" on every Draft and approved message; a cost record for every AI call with a monthly cost report; drafting inside the hourly run.

  Recorded in full in `docs/product/p1-spec.md` and `docs/architecture.md`.
- **Cut-off never before the Period opens.** If the cut-off offset would put a Period's Cut-off before the day its documents become due, the Cut-off is that opening day, so the first request still goes out. Found in the M2 code review. Recorded in `docs/product/p1-spec.md`.
- **M2 round.** The owner accepted all 22 recommendations ("go ahead"):
  - a session-start hook that starts Docker, falls back to `mirror.gcr.io`, migrates and installs sqlx-cli; and removing the stale duplicate sections from `AGENTS.md`;
  - six M2 test seams (`docs/architecture.md`);
  - firms created by a command; login emails through the outbox, auto-approved as `system:login` (Reminders never); 15-minute single-use hashed tokens; 30-day sessions; at most 5 login emails per firm per hour;
  - client fields, CSV/XLSX import rules and a two-step preview-then-confirm import;
  - the built-in calendar's forms, eBIRForms dates, weekend and holiday roll-forward, one calendar data file with nationwide extensions, and one-off per-client due-date overrides;
  - one collection Period per month (per quarter for quarterly-only clients), the Cut-off rule, the default Owed documents and where chasing starts;
  - the reminder timing, template drafts until M3, and per-client schedule control;
  - the M2 pages and crates.

  Recorded in full in `docs/product/p1-spec.md` and `docs/architecture.md`. Sources for the calendar: `docs/research/bir-calendar.md`.
- **Commit straight to main.** The owner is the only developer: no branches or pull requests from M2 on. Small commits (one behaviour each), every commit green on fmt, clippy and tests, pushed to `main`, with CI fixed immediately if it goes red. This supersedes the Phase 8 "branch per milestone, PR, merge on the owner's word" workflow.
- **Build in the cloud.** From M2 on, coding happens in Claude Code on the web (4 vCPU / 16 GB VMs), with a start-up hook for Postgres, Mailpit and the pinned Rust. If that fails, the fallback is local builds with lighter settings.
- **M1 test seams, Tenant, unconfirmed sends.** Five test seams for M1. "Tenant" is the Engine's word for the owner of data (a Firm in Kompleto). A send whose outcome is unknown, through a provider that can't block duplicates (or past Resend's 24-hour window), becomes "unconfirmed" and is never retried automatically. Recorded in `docs/architecture.md`.
- **Phase 8.** Five decisions:
  - the milestone workflow: a branch each, test first, code review, and a pull request merged only on the owner's "merge";
  - the Kompleto code rules in `AGENTS.md`;
  - `GLOSSARY.md`, including "Payor" for the business that owes a Client a 2307;
  - ADRs 0001–0004;
  - `docker compose` with Postgres 17 and Mailpit for development.
- **Phase 7 closed.** `docs/architecture.md` approved as Kompleto's architecture.
- **Architecture, round 2.** Six decisions:
  - hosting from M6: Vultr Singapore, Cloudflare R2 and Email Routing, about $6.72/month;
  - Resend with idempotency keys, an SES application in pilot week 1, and a 90/day send budget;
  - the pinned crate stack, with isolated Excel parsing, our own sessions table and POST-to-log-in;
  - auto-replies don't stop reminders;
  - per-file keys wrapped by a master key;
  - a private GitHub repository with CI.

  Recorded in full in `docs/architecture.md`.
- **Architecture, round 1.** Six decisions: one crate split into `engine/` and `kompleto/` (the engine never depends on the product), the outbox state machine with hash-bound approval and send-once keys, typed firm isolation, UTC storage with an injected clock, a Postgres job table, and the build order M1–M6. Recorded in full in `docs/architecture.md`.
- **Acceptance criteria.** The 25 criteria in `docs/product/p1-spec.md` are Kompleto's definition of done (login links 15 min; upload links 30 days after cut-off; files ≤ 10 MB; replies forwarded within 5 min).
- **P1 spec, round 3.** Seven decisions:
  - the name Kompleto (getkompleto.com);
  - adding clients by form or CSV/Excel;
  - the sender identity;
  - the pilot success measures, replacing the old B thresholds;
  - leaving and deletion;
  - registration timing;
  - Claude Haiku 5.5, with Sonnet 5.5 as the fallback, confirmed by a blind Taglish test.

  Recorded in full in `docs/product/p1-spec.md` → "Decided".
- **P1 spec, round 2.** Six decisions: accounts and login, the reminder schedule, approval rules, client replies, the 2307 tracker, and what the AI sees. Recorded in full in `docs/product/p1-spec.md` → "Decided".
- **P1 spec, round 1.** Seven decisions about the first version: who it's for, its scope, channels, the BIR calendar, uploaded files, AI provider and web technology. Recorded in full in `docs/product/p1-spec.md` → "Decided".
- **First product: P1.** P1 is the first product built on the engine: a document and BIR-deadline chaser for Philippine bookkeepers. It is sold on three things together: a BIR deadline calendar per client, a tracker for received 2307s, and Taglish/English reminders the bookkeeper approves before they go out. Payroll-input chasing comes later as an add-on. S1, the seller tax helper, is the fallback and gets its own kill-checks before it is ever built. Basis: `docs/research/opportunity-scorecard.md` §9–12 and `docs/research/p1-kill-checks.md`.

## 2026-10-20

- **Engine design.** We use our own engine design, built for a hosted product with many customers (Postgres, our own LLM API client). It borrows Harvey's proven ideas:
  - code decides the next step;
  - the model only writes text and gets no tools;
  - every send passes one approval gate and one pre-send check;
  - follow-ups stop on reply;
  - every model call is costed.

  It avoids Harvey's known bugs (`docs/research/harvey-teardown.md`). No Harvey code is copied; any adapted prompt keeps its MIT notice.
- **Engine built with the first product.** There is no engine-only phase. The engine is built inside the first product, with engine modules kept separate from product modules, so the second product reuses them.
- **Build first.** Build the whole first product, core loop first, before showing it to customers; the owner chooses when to demo. The spec, architecture and readiness gates still come before code. This supersedes the 2026-10-18 build gate.

## 2026-10-18

- **Ambition.** Profitable small business first. The reusable foundation is built from that revenue, and international expansion is earned later.
- **Outreach method.** Free written outreach: public Facebook groups and communities, and at most 15–20 personalised messages a week. Claude drafts, the owner approves and sends. Since 2026-10-20 it finds customers for the built product.
- **Pilots.** 3–5 free 4-week pilots, then offer ₱1,500 for the next month via GCash. Since 2026-10-20 they run on the built product, not by hand.
- **Budget.** Up to ~₱600/month (LLM API + domain) until a business pays. No customer data goes to free LLM tiers that train on inputs.
- **Lead hypothesis: B**, a lead and quote follow-up agent for PH service businesses. **Superseded** 2026-10-21 by "First product: P1".
- **Build gate.** Product code waited until hand-run pilots hit a bar: ≥ 3 pilots, ≥ 70% of drafts approved, ≥ 1 recovered reply or booking per business per week, ≥ 1 paying. **Superseded** 2026-10-20 by "Build first".

## Owner constraints

These are stated facts, not preferences to argue with.

- **2026-10-18:** no business network, and no customer interviews or sales calls. Validation and distribution have to work through self-serve and written channels.
- **2026-10-18:** no money for paid ads. Written, asynchronous contact with people who sign up (chat or email) is fine.
