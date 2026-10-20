# Kompleto architecture

Status: **Phase 7, decisions complete; awaiting the owner's approval to close the phase.** Built round by round through grilling. Everything under "Decided" is owner-approved, with its date. It implements `docs/product/p1-spec.md` and its acceptance criteria (referenced by ID, e.g. D2, E1). Research behind it: `docs/research/rust-crates.md`, `docs/research/hosting-and-email-infra.md`, `docs/research/llm-options.md`, `docs/research/agent-architecture-comparison.md`.

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

### Build order (2026-10-21)

Each milestone ends with its acceptance tests passing; the owner can demo after any of them.

| Milestone | Delivers | Proves |
|---|---|---|
| **M1** | Project skeleton, database, outbox, sending to a local test mailbox, audit log | E1, D2 |
| **M2** | Firms, login, clients (form + CSV/Excel), BIR calendar, document checklist, schedule | A1, A2, B1–B4, D1 |
| **M3** | AI drafts with template fallback, approval screens, "copy for Viber" | D3–D7, E3, J1 |
| **M4** | Phone upload page, encrypted files, auto-deletion | F1–F3 → **core loop demo-ready (~week 4)** |
| **M5** | Replies and bounces, dashboard, 2307 tracker | G1–G3, H1–H2 |
| **M6** | Data export, account closure, real email domain, hosting, hardening | I1–I2, E2, J2 → pilot-ready |

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

## Open

- Owner approval of this document closes Phase 7.
