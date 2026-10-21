# Kompleto architecture

Status: **Phase 7, complete (owner-approved 2026-10-21).** Built round by round through grilling. Everything under "Decided" is owner-approved, with its date. It implements `docs/product/p1-spec.md` and its acceptance criteria (referenced by ID, e.g. D2, E1). Research behind it: `docs/research/rust-crates.md`, `docs/research/hosting-and-email-infra.md`, `docs/research/llm-options.md`, `docs/research/agent-architecture-comparison.md`.

## Decided

### One program, two parts (2026-10-21)

One Rust crate, split into two top-level modules:

```
src/
├── engine/      reusable: jobs & scheduler, outbox & approval, pre-send checks,
│                LLM client, email send/receive, audit & cost log
└── kompleto/    product-only: firms & clients, BIR calendar, document checklist,
                 2307 tracker, upload page, dashboard pages
```

**Rule:** `engine` never depends on `kompleto`; only `kompleto` calls into `engine`. An automated check fails the build if the rule is broken, so the engine stays reusable for later products (S1, add-ons).

### Outbox lifecycle (2026-10-21)

Every outgoing message follows one state machine, enforced in code:

```
draft ──approve──► approved ──► sending ──► sent
  │                   │            └──► failed (retry with the SAME key, never a new send)
  ├─ edit → new draft (old approval void)
  └─ deadline passes → expired          cancelled ◄── client replied / everything uploaded
```

- **Approval binds content.** Approval stores a hash of the exact message. Sending re-checks the hash; on a mismatch, nothing goes out (D2).
- **Send at most once.** Each send carries a one-time idempotency key recorded before the provider call. After a crash, the outbox reconciles with the provider rather than sending again (E1).
- **Implementation details (M1):**
  - a `blocked` state for a message whose content changed behind the outbox;
  - a `may_have_sent` flag carried across attempts;
  - retries after 1 min, 5 min, 30 min, then 2 h, at most 5 attempts.

  The `expired` and `cancelled` transitions arrive with D4 (M3) and G1–G2 (M5).
- **Unconfirmed (2026-10-21).** Resend can safely retry with the same key within its 24-hour window. Mailpit and SES can't block duplicates. If a crash or an unknown outcome hits a send through one of those, or a Resend send older than 23 hours, the message becomes **unconfirmed**. It is shown to the Firm as "may have been sent, check before resending" and never retried automatically.

### Test seams for M1 (2026-10-21)

1. **Outbox API** (`engine::outbox`) with a fake mailer: D2.
2. **The real binary, crashed on purpose** between the provider call and recording the send, then restarted, against a fake Resend: E1.
3. **Email adapters** (Resend, Mailpit) against a fake HTTP server: request shape, idempotency header.
4. **Job queue** (enqueue → claim → finish, concurrent claimers).
5. **Engine boundary**: a test fails if `src/engine` uses `kompleto` (ADR 0001).

### Test seams for M4 (2026-10-21)

1. **File encryption** (`engine::files`): a round trip, and a changed byte, a swapped key or a file moved to another Firm all fail to decrypt.
2. **Upload links** with the test clock: created with the first Draft, one per Client and Period, a wrong or altered signature is "not found", and the link stops working 30 days after the Cut-off (F2).
3. **Upload page in-process:** file types by content, the 10 MB limit, labels and the Payor name, and the per-link limits (F1).
4. **Deletion** by advancing the clock: 30 days after received, 90 days after upload, with audit entries (F3).
5. **A2 sweep** extended to Upload lists, downloads and Mark received.
6. **Pre-send checks** accept only the Client's own Upload link.

Implementation notes (M4):
- `engine::files` owns encryption (XChaCha20-Poly1305: a random key per file, wrapped by a key derived from `FILE_MASTER_KEY`, both bound to the file ID and tenant), the `FileStore` interface (local disk until R2) and the `stored_files` table. A file's row is written before the file itself, so every stored file has a deadline the hourly sweep acts on; deleting removes the file, then its key, and is audited. A deadline only ever moves earlier. One file that can't be deleted doesn't stop the sweep.
- Upload links are the link's ID plus the first 128 bits of an HMAC-SHA256 over it, with a key derived from the master key, checked in constant time. Looking a link up by its token is the one query not scoped by a Firm: the token is the credential. A changed Cut-off moves the link's last day with it.
- An Upload's ID is its stored file's ID. Labels name an Owed document by its text; a 2307 also carries a typed Payor name until M5's Payor list.
- The upload page sends `Referrer-Policy: same-origin`, not `no-referrer`: with `no-referrer`, browsers send `Origin: null` with the page's own uploads, and the same-origin check refuses them. The link still never reaches another site.
- The link is checked before an upload takes one of the 4 slots, and an upload has 2 minutes to arrive.
- G2: the scheduler lists only Owed documents without an Upload and, on every run, cancels the Period's waiting reminders once all have one; the upload page does the same at once after the last upload.

### Test seams for M3 (2026-10-21)

1. **AI client** (`engine::llm`) against a fake Anthropic server: request shape, and an inspection of every outgoing request for TIN-shaped numbers, peso amounts and file contents (D6); timeouts and bad replies.
2. **Drafting** with a fake model: valid output gives an "AI" Draft, anything else a "Template" Draft (D5); every call recorded with model, tokens, cost and request ID (J1).
3. **Pre-send checks** against Postgres (D7), at approval and at sending. The engine defines the check as an interface the product supplies, so the engine still never depends on Kompleto.
4. **Approval pages in-process** with the test clock: batch review (D3), expiry (D4), edit voiding approval (D2), the Viber text (E3).
5. **A2 sweep** extended to every Draft page.
6. **Cost report** per Firm per month (J1).

Implementation notes (M3):
- A pre-send check either refuses a message (it is blocked, with the reason) or couldn't run (say, a database timeout); a message whose check couldn't run is retried, not blocked. A message that may already have gone out stays "unconfirmed" whatever its check says.
- Approval runs the checks before its own transaction, so a check's queries never wait on a connection the approval holds; the content hash then confirms the approved text is the text checked.
- Links are words with a scheme, starting with `www.`, or bare domains on a common top-level domain (`gcash-pay.ph/upload`), after stripping surrounding punctuation. Model output uses the same rule. Firm names that read as links are refused, since the name signs every email.
- The scheduler checks a Period's four-Draft limit before drafting, so a slot that can't be recorded never costs a model call.
- `kompleto llm-costs` reads every tenant's costs; like the system-wide workers it is an operator tool, never reachable from a Firm's pages.

### Test seams for M2 (2026-10-21)

1. **Calendar and schedule logic**, pure functions with no database or clock: due dates, roll-forward, extensions, overrides, Cut-off, reminder dates and the cap. Table-driven: B3, B4, D1's dates.
2. **Repositories** against real Postgres (`#[sqlx::test]`), every function taking a `FirmId`: B1, B2's all-or-nothing import, A2 at the data layer.
3. **Import parser**: bytes in, valid rows and per-row errors out; `.xlsx` parsed in a child process with time and memory limits, tested with a hostile file: B2.
4. **The web app in-process** (`Router` + `oneshot`, no port) with the test clock and a recording mailer, following cookies through login: A1, B1, B4's pages.
5. **A2 route sweep**: the router is built from one list of Firm-owned routes; a test requests each with another firm's IDs and must get "not found".
6. **Scheduler with the test clock**: step day by day through a quarter, run the scheduler each day, check Drafts appear on the expected days, never more than 4 per client per Period, and a rerun creates nothing: D1.

Implementation notes (M2):
- Login emails go through the outbox, auto-approved as `system:login`; only that kind of message may be auto-approved.
- The BIR calendar is one data file, `data/bir-calendar.toml`, built into the binary: each form's rule with its source, the holidays and nationwide extensions. The program warns at startup if the holidays don't cover the next 12 months.
- An hourly scheduler sweeps all firms and creates the Drafts that are due; a unique (client, Period, slot) constraint enforces the cap.
- Manila time is a fixed UTC+8 offset (no daylight saving since 1978), so no time-zone database is needed.
- A Cut-off is never earlier than the day its Period opens: an offset that would put it earlier (e.g. 10 days before a 10th-of-the-month due date) gives a Cut-off on the opening day, so the first request still goes out. (Found in the M2 code review; owner-approved 2026-10-21.)
- The scheduler creates each Draft and its `scheduled_drafts` record in one transaction, and one Client's failure is logged without stopping the sweep.
- Login emails are prepared after the login page replies, so response time doesn't reveal which addresses have accounts.
- The Excel parser child runs with an empty environment, and the server reads at most 16 MB of output from it.

The Engine scopes data by **Tenant** (`GLOSSARY.md`); Kompleto's `FirmId` maps to it.

### Firm isolation (2026-10-21)

- Every record belongs to a firm.
- Every repository function takes a distinct `FirmId` type, so a query that forgets the firm doesn't compile.
- A test per page and per download tries another firm's IDs and must get "not found" (A2).
- Postgres row-level security is a possible later second lock; it is not in the first version.

### Time (2026-10-21)

- All times are stored in UTC and shown and scheduled in **Asia/Manila**.
- Code never reads the system clock directly; it uses an injected `Clock`, so tests can jump time for reminders, expiry and deletion (D1, D4, F2, F3, I2).

### Background work (2026-10-21)

- Scheduling, drafting, sending, reading replies and deleting files run as jobs in a Postgres table, claimed with `FOR UPDATE SKIP LOCKED` by workers inside the same program.
- No Redis and no separate worker service. `apalis` + `apalis-postgres` 1.0 is the fallback if the hand-rolled queue gets costly.
- **Sending uses the outbox table itself as its queue** (claimed with `FOR UPDATE SKIP LOCKED`, one sender at a time under an advisory lock) rather than a separate job per email, so a message's send state lives in one row. Other work uses the `jobs` table. (Implementation note, M1.)

### Build order (2026-10-21)

Each milestone ends with its acceptance tests passing; the owner can demo after any of them.

| Milestone | Delivers | Proves |
|---|---|---|
| **M1** | Project skeleton, database, outbox, sending to a local test mailbox, audit log | E1, D2 |
| **M2** | Firms, login, clients (form + CSV/Excel), BIR calendar, document checklist, schedule | A1, A2, B1–B4, D1 |
| **M3** | AI drafts with template fallback, approval screens, "copy for Viber" | D3–D7, E3, J1 |
| **M4** | Phone upload page, encrypted files, auto-deletion | F1–F3 → **core loop demo-ready (~week 4)** |
| **M5** | Replies and bounces, dashboard, 2307 tracker | G1–G3, H1–H2 |
| **M6** | Data export, account closure, real email domain, daily send budget, hosting, hardening | I1–I2, E2, J2 → pilot-ready |

### Hosting (2026-10-21)

- **Development:** on the owner's Mac (Docker via OrbStack for Postgres), at no cost.
- **Paid hosting starts at M6:**
  - a Vultr 1 GB server in Singapore, with automatic backups, running the binary, Postgres and Caddy (TLS);
  - Cloudflare R2 for encrypted files and nightly encrypted `pg_dump` backups (kept 30 days);
  - Cloudflare Email Routing plus an Email Worker that POSTs each reply's raw MIME to the app;
  - Better Stack / Healthchecks.io for uptime, Sentry Free for errors.
- About US$6.72/month in the worst case. Fallbacks: Linode Nanode or a DigitalOcean Droplet; SES receiving via S3 + SNS. Details: `docs/research/hosting-and-email-infra.md`.

### Email sending (2026-10-21)

- Pilots send through **Resend's free tier**, with an `Idempotency-Key` per send (E1).
- The owner applies for **Amazon SES** (Singapore) in pilot week 1. If it's refused, we stay on Resend.
- The app enforces a **daily send budget of 90**, under Resend's 100/day cap. When more reminders are due, the nearest deadlines go first, the rest wait until the next day, and the dashboard shows what's waiting.

### Crate stack (2026-10-21)

Pinned versions from `docs/research/rust-crates.md`:

| Need | Crates |
|---|---|
| Runtime and web | tokio 1.54, axum 0.8.9, tower-http 0.7, axum-extra 0.12 |
| Database | sqlx 0.9 (Postgres, migrations, offline macros) |
| Pages | askama 0.16 + askama_web, with htmx 2.0.11 vendored |
| HTTP client | reqwest 0.13 (rustls) |
| Inbound mail | mail-parser 0.11 |
| Import | csv 1.4, calamine 0.37 |
| File encryption | chacha20poly1305 0.11, secrecy 0.10 |
| Login tokens | getrandom 0.4, sha2 0.11, subtle 2.6 |
| Errors | thiserror 2, anyhow 1 (app edge only) |
| Logs | tracing 0.1 + tracing-subscriber 0.3 |
| Tests | `#[sqlx::test]`, insta, wiremock |
| Dependency checks | cargo-deny on every build |
| Added at M2 (2026-10-21) | toml (calendar file), encoding_rs (Windows-1252 CSV), tower (tests), rustix (limits on the Excel child process) |
| Added at M4 (2026-10-21) | hmac (Upload link signatures) |

Lookalike crates to avoid: `resend`, `resend-rust`, `mailparser`.

Design rules:
- **Excel files are parsed in a separate child process** with time and memory limits (calamine can abort on crafted files).
- **Sessions use our own table**; the published sqlx store for tower-sessions doesn't work with sqlx 0.9.
- **Login links open a page with a "Log in" button**; the token is consumed only by that POST, so link scanners can't use it up (A1).

### Auto-replies (2026-10-21)

- Out-of-office and other automatic replies (recognised by `Auto-Submitted`, `X-Autoreply` and similar headers) **do not stop reminders**. They show on the dashboard as "auto-reply".
- Only a human reply stops reminders. This refines G1.

### File-encryption keys (2026-10-21)

- Each file gets its own data key, wrapped by one **master key**. The master key is held only in the server environment and in the owner's password manager.
- Losing the master key loses the stored files. That's acceptable: files are temporary (≤ 90 days) and clients can re-upload.
- Database backups are encrypted.

### Code home and checks (2026-10-21)

- A **private GitHub repository** under the owner's account.
- GitHub Actions on every push: `cargo fmt --check`, clippy, all tests against a real Postgres service, and cargo-deny. The workflow is added at M1, when there is code to check.
- Rust pinned via `rust-toolchain.toml` (1.99.1, edition 2024).
- **Development runs in Claude Code on the web (2026-10-21)**, not on the owner's 8 GB Mac. A SessionStart hook starts Postgres and Mailpit with `docker compose` and installs the pinned toolchain. Fallback: build locally with lighter settings.

