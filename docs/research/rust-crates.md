# Rust crates for Kompleto (P1)

Status: research note, 2026-10-21. **Nothing here is an owner-approved decision.** It feeds the Phase 7 (Rust architecture) gate. Spec: `docs/product/p1-spec.md`; technical direction: `docs/research/README.md` and `agent-architecture-comparison.md` §4.

Accessed: 2026-10-21 for every source unless stated.

Evidence labels:
- **[REG]** crates.io registry API data (version, date, licence, MSRV, downloads);
- **[GH]** GitHub repository or API data (stars, open issues and PRs, last push);
- **[OD]** official docs, README or changelog of the crate or service;
- **[PRB]** our own throwaway probe, run locally on 2026-10-21 (see "Probe" below);
- **[VC]** vendor claim.

Confidence: **H/M/L**. Downloads are written as "all-time / last 90 days". "Open" on GitHub counts issues plus PRs. Numbers in brackets such as [3] point to Sources.

**Probe [PRB].** A throwaway manifest in the session scratchpad (not in the repo) listed the recommended stack below. `cargo generate-lockfile` resolved it to 334 packages. `SQLX_OFFLINE=true cargo check` passed on the local rustc 1.98.1 (aarch64-apple-darwin). Nothing was built for production and no project file was created. This shows only that the versions resolve and type-check together; it is not a test of behaviour.

## Recommended stack

These are recommendations, not decisions.

| Need | Crate | Version (release date) | Licence | Runner-up |
|---|---|---|---|---|
| Async runtime | `tokio` | 1.54.0 (2026-10-19) | MIT | none realistic |
| Web framework | `axum` | 0.8.9 (2026-04-14) | MIT | none (stay on axum) |
| HTTP middleware | `tower-http` | 0.7.1 (2026-08-31) | MIT | hand-written tower layers |
| Cookies | `axum-extra` (`cookie`) | 0.12.6 (2026-04-14) | MIT | `tower-cookies` |
| Database | `sqlx` (postgres, macros, migrate) | 0.9.0 (2026-05-21) | MIT OR Apache-2.0 | `diesel` 2.3.14 + `diesel-async` 0.9.2 |
| Job queue | **own `jobs` table** (`FOR UPDATE SKIP LOCKED` on sqlx) | — | — | `apalis` + `apalis-postgres` 1.0.0 (2026-10-12) |
| HTML templates | `askama` + `askama_web` (`axum-0.8`) | 0.16.1 (2026-09-04) / 0.16.0 (2026-04-29) | MIT OR Apache-2.0 | `maud` 0.27.0 |
| htmx (vendored JS file) | htmx | 2.0.11 (npm `latest`, 2026-09-22) | 0BSD | none needed |
| HTTP client | `reqwest` (`rustls`, `json`) | 0.13.5 (2026-09-08) | MIT OR Apache-2.0 | none needed |
| Email sending | **Resend HTTP API via our own reqwest call** | — | — | `resend-rs` 0.35.0 (official SDK); SMTP via `lettre` 0.11.23 |
| Inbound MIME parsing | `mail-parser` | 0.11.9 (2026-09-09) | Apache-2.0 OR MIT | `mailparse` 0.18.0 |
| CSV | `csv` | 1.4.0 (2025-10-17) | Unlicense OR MIT | none needed |
| Excel (xlsx) | `calamine`, **in an isolated child process** | 0.37.0 (2026-10-18) | MIT | none in pure Rust with equal coverage |
| Export zip (I1) | `zip` (write only) | 9.0.5 (2026-10-17) | MIT | `async_zip` (not checked) |
| File encryption | `chacha20poly1305` (XChaCha20-Poly1305) | 0.11.0 (2026-06-28) | Apache-2.0 OR MIT | `aes-gcm` 0.11.1 |
| Secret handling | `secrecy` + `zeroize` | 0.10.3 (2024-10-09) / 1.9.1 (2026-10-06) | Apache-2.0 OR MIT | plain `zeroize` |
| Random tokens | `getrandom` (or `rand` 0.10 `SysRng`) | 0.4.3 (2026-06-17) | MIT OR Apache-2.0 | `rand` 0.10.3 |
| Token hashing / MACs | `sha2` + `hmac` + `subtle` | 0.11.1 (2026-10-12) / 0.13.0 (2026-03-29) / 2.6.1 (2024-06-24) | MIT OR Apache-2.0; `subtle` BSD-3-Clause | `blake3` 1.8.7 |
| Sessions | **own `sessions` table** + `axum-extra` cookie | — | — | `tower-sessions` 0.15.0 (store crate needs a git dependency, see §8) |
| Logging | `tracing` + `tracing-subscriber` (`json`, `env-filter`) | 0.1.44 (2025-12-18) / 0.3.23 (2026-03-13) | MIT | add `tracing-opentelemetry` 0.34.0 later |
| DB tests | `#[sqlx::test]` on a Postgres service container | (sqlx 0.9.0) | — | `testcontainers` 0.28.0 / `testcontainers-modules` 0.15.0 |
| Snapshots | `insta` | 1.49.0 (2026-10-03) | Apache-2.0 | none needed |
| HTTP mocks | `wiremock` | 0.6.5 (2025-08-24) | MIT/Apache-2.0 | `httpmock` 0.9.0 (2026-10-20) |
| Fake clock | **own `Clock` trait**, plus tokio `test-util` for timers | — | — | `mock_instant` 0.6.1 |
| Errors | `thiserror` (modules) + `anyhow` (startup and `main` only) | 2.0.21 (2026-09-23) / 1.0.104 (2026-07-18) | MIT OR Apache-2.0 | — |
| Config | **typed struct read from env**, `dotenvy` in dev only | dotenvy 0.15.7 (2023-03-22) | MIT | `figment` 0.10.19 / `config` 0.15.27 |
| Dates | `time` (sqlx `time` feature) | 0.3.55 (2026-08-01) | MIT OR Apache-2.0 | `jiff` 0.2.38 + `jiff-sqlx` 0.2.0 |
| IDs | `uuid` (`v7`) | 1.28.0 (2026-10-10) | Apache-2.0 OR MIT | — |
| Supply chain | `cargo-deny` | 0.20.2 (2026-07-09) | MIT OR Apache-2.0 | `cargo-audit` 0.22.2 |
| Toolchain | Rust stable **1.99.1** (2026-10-16), edition **2024** | — | — | — |

The stack's effective MSRV is **1.94.0** (set by sqlx 0.9) [REG]. Pin the toolchain in `rust-toolchain.toml` rather than tracking MSRV.

## 1. Async runtime and web

**tokio 1.54.0** (2026-10-19), MIT, MSRV 1.85; 1.09B / 266M downloads [REG][1]. It is the foundation for everything else here. **H.**

**axum 0.8.9** (2026-04-14), MIT, MSRV 1.80; 532M / 137M downloads [REG][2]. The repo has 27.5k stars and 110 open, and was pushed 2026-10-20 [GH][3].
- **0.8.x is current; 0.9 is not released.** The changelog's "Unreleased" section lists breaking changes:
  - `tower-log` is no longer a default feature;
  - nested-router fallbacks merge differently;
  - `axum::serve` applies hyper's `header_read_timeout`, and its output type changes [OD][4].
  
  Expect a 0.9 migration during or after the pilots. **H.**
- **Pitfall: request body limit.** axum's default body limit for extractors such as `Multipart` and `Bytes` is 2 MB (`DefaultBodyLimit`). F1 needs 10 MB per file, so raise it on the upload route only. Pair it with `tower_http::limit::RequestBodyLimitLayer` (axum docs; not re-fetched this pass, **M**).

**tower-http 0.7.1** (2026-08-31), MIT, MSRV 1.65 [REG][5].
- 0.7.0 shipped on 2026-06-15 [REG]. It added a **`csrf` middleware**, a port of Go 1.25's cross-origin protection using `Sec-Fetch-Site` and `Origin`, with no token state [OD][6]. This suits server-rendered forms.
- Breaking changes in 0.7.0 [OD][6]:
  - the implicit `tokio` and `async-compression` features were removed (enable `compression-gzip`, `fs` and so on directly);
  - `SizeAbove` became `u64`;
  - file paths with a trailing slash now return 404.
- Features to enable: `compression-gzip`, `compression-br`, `trace`, `limit`, `fs` (static files and vendored htmx), `csrf`, `request-id`, `timeout`, `set-header` (security headers).
- **Duplicate version:** reqwest 0.13.5 still depends on tower-http ^0.6.8, so both 0.6.11 and 0.7.1 end up in the tree. This is harmless but shows in `cargo deny check bans` [PRB].

**axum-extra 0.12.6** (2026-04-14), MIT [REG][7]. Its features are `cookie`, `cookie-signed` and `cookie-private` (`CookieJar`, `SignedCookieJar`, `PrivateCookieJar`). It requires axum ^0.8.9 [REG]. We need only plain `CookieJar` because the session cookie holds an opaque random token (§8). **H.**

## 2. Database and job queue

**sqlx 0.9.0**, MIT OR Apache-2.0 [REG][8].
- Release dates: crates.io says 2026-05-21; the changelog heading says 2026-05-06 [OD][9]. **MSRV 1.94.0** [REG][OD]. 169M / 48M downloads.
- **The repository moved** from `launchbadge/sqlx` to **`transact-rs/sqlx`** (the old URL redirects). The changelog explains that LaunchBadge no longer owns it and that the principal authors hold it collectively [OD][9]. The repo has 17.6k stars and 788 open, and was pushed 2026-10-16 [GH]. This is not an impostor; the crate name and owners are unchanged.

Changes in 0.9 that matter to us [OD][9]:
- **`SqlSafeStr`:** `query*()` now takes only `&'static str` or `AssertSqlSafe(..)`. Dynamic SQL built with `format!` has to be wrapped visibly, which is a useful review speed bump.
- **Postgres nullability inference changed** (generic plan), so `query!` output types can differ from 0.8.
- **`sqlx.toml`** is behind the non-default `sqlx-toml` feature. It can rename `DATABASE_URL` and the `_sqlx_migrations` table, and set global type overrides.
- **`Cargo.lock` was removed** from the sqlx repo: `cargo install --locked sqlx-cli` no longer works.
- Migration ordering is now deterministic, and the advisory lock is cancel-safe.
- **Offline macros:**
  - `query!` checks SQL against a live database at compile time (`DATABASE_URL`).
  - Otherwise it reads `.sqlx/` query data written by `cargo sqlx prepare`.
  - With `SQLX_OFFLINE=true` it errors if the cache is missing: "`SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare`…".
  - `SQLX_OFFLINE_DIR` takes precedence [OD, sqlx-macros-core 0.9.0 source].
  - Commit `.sqlx/` and run `cargo sqlx prepare --check` in CI. **H.**
- **Migrations:** `sqlx::migrate!()` embeds `migrations/` at compile time, so editing an applied migration causes a checksum mismatch at startup. Treat applied migrations as immutable. **H.**

**TLS provider (from the probe).** sqlx's `tls-rustls` uses **ring**, while reqwest 0.13's default `rustls` uses **aws-lc-rs**. Together they compile both providers into the binary [PRB].
- Use sqlx's **`tls-rustls-aws-lc-rs`** feature. With it, ring left the tree entirely [PRB].
- If both providers stay, rustls cannot pick a default and needs `CryptoProvider::install_default()` at startup. That is from rustls behaviour as known; **not re-verified this pass, M.**

**diesel 2.3.14** (2026-10-07) and **diesel-async 0.9.2** (2026-06-19), MIT OR Apache-2.0, MSRV 1.86 / 1.84 [REG][10]. The repo moved to `diesel-rs/diesel_async` (828 stars, 10 open) [GH]. It is mature and type-safe, but async support lives in a separate crate and its DSL adds indirection. The architecture note already prefers raw SQL for the event and journal tables. Runner-up. **M.**

**Job queue.**
- **apalis 1.0.0 is out** (2026-10-12), MIT OR Apache-2.0; 1.34M / 397k downloads; 1.46k stars, 4 open [REG][GH][11].
- **apalis-postgres 1.0.0** (2026-10-12) is the Postgres backend [REG][12]:
  - it depends on **sqlx ^0.9**, so it is compatible;
  - crates.io shows its licence as "non-standard" because it uses `license-file`; the file is **MIT** [GH];
  - the repo is young (8 stars, 2 open);
  - features: polling and trigger-based storages, heartbeats and re-enqueueing of orphaned jobs [OD/VC][12].
- **Don't use `apalis-sql`.** Its stable release is 0.7.4 (2025-11-18) and its newest is `1.0.0-rc.9`; it was superseded by the per-database crates [REG].
- **Recommendation:** keep the hand-rolled `jobs` table (about 150 lines). Kompleto's correctness hinges on the send journal (E1), not on the queue, and owning the schema keeps job rows in the same transaction as state changes. apalis-postgres is the fallback, as `README.md` already states. **M.**

## 3. HTML templating

| | askama 0.16.1 | maud 0.27.0 | minijinja 3.0.1 |
|---|---|---|---|
| Released | 2026-09-04 | 2025-02-02 (20 months ago) | 2026-10-15 (3.0 major just shipped) |
| Licence / MSRV | MIT OR Apache-2.0 / 1.88 | MIT OR Apache-2.0 / not stated | Apache-2.0 / 1.70 |
| Repo | askama-rs/askama: 1.2k stars, 26 open, pushed 2026-10-20 | lambda-fairy/maud: 2.6k stars, 66 open, pushed 2026-05-25 | mitsuhiko/minijinja: 2.8k stars, 22 open, pushed 2026-10-18 |
| Compile-time checks | **Yes**: template syntax and the fields and methods used are checked by the derive | **Yes**: HTML is a Rust macro | **No**: errors at render time; hot reload via `minijinja-autoreload` |
| Escaping | Automatic HTML escaping for `.html` templates | Automatic | Automatic when configured |
| axum | `askama_web` 0.16.0 with feature `axum-0.8` (`#[derive(Template, WebTemplate)]`) | `axum-core` 0.5 feature built in | manual `Html(..)` |

Sources [REG][GH][13][14][15].

- **Recommendation: askama + askama_web.** Compile-time checking catches broken pages in `cargo check`, and the Jinja-like syntax stays readable. AI drafts are untrusted text, and auto-escaping covers them on the dashboard. **H.**
- **Pitfall: `askama_axum` is deprecated** (`0.5.0+deprecated`, 2025-04-18). The integration crates were removed from askama in 0.13; use `askama_web` [REG]. Note that `askama-axum` resolves to the same deprecated crate.
- **htmx:** npm `htmx.org` `latest` is **2.0.11** (2026-09-22), 0BSD. **4.0.0** is published under the `next` tag (2026-08-28) [OD][16].
  - Vendor 2.0.11 as a static file served by `tower-http` `ServeDir`; no CDN, and the CSP stays simple.
  - `axum-htmx` 0.8.1 (2025-06-05) offers typed `HX-*` headers, but reading two headers by hand is trivial. Optional.
  - No template engine conflicts with htmx; it is plain HTML attributes. **H.**

## 4. HTTP client

**reqwest 0.13.5** (2026-09-08), MIT OR Apache-2.0, MSRV 1.85; 819M / 224M downloads; 11.9k stars, 478 open [REG][GH][17].
- **rustls is the default TLS backend since 0.13.0** (previously native-tls), with the **aws-lc-rs** crypto provider and **`rustls-platform-verifier`** for roots [OD][18].
  - `rustls-tls` was renamed `rustls`.
  - `query` and `form` became opt-in features.
  - The `default-tls` feature maps to `rustls` [REG].
- **Features:** `default-features = false, features = ["rustls", "json", "http2"]`.
- **Pitfalls:**
  - Set `timeout` and `connect_timeout` on the `Client`; reqwest applies no total request timeout by default (reqwest docs; **M**, not re-fetched).
  - `rustls-platform-verifier` reads the OS trust store, so a Linux container image needs `ca-certificates`.
  - `aws-lc-sys` compiles C code, so the build image needs a C toolchain (possibly CMake on some targets; the `cmake` crate appears in the lockfile) [PRB]. Verify in the first Docker build.

## 5. Email

### Sending

**The provider choice matters more than the crate, because of E1.** E1 says an approved email is sent at most once, even across a crash.
- **Resend:** `POST /emails` accepts an **`Idempotency-Key`** header: "Resend checks whether an email with the same idempotency key has already been sent in the last 24 hours". Keys are at most 256 characters and expire after 24 hours [OD][19]. Over SMTP the equivalent is the `Resend-Idempotency-Key` header [OD][19].
  - Reconciling a send that has `started` but not `completed` becomes "retry with the same key", which is safe within 24 hours.
- **Amazon SES v2:** the `SendEmail` API reference has **no idempotency token**; the page contains neither "idempotency" nor "ClientToken" [OD][20]. With SES, a crash between the call and recording the result can only be reconciled by asking SES or the event stream whether the message went, never by retrying.

| Option | Version | Notes |
|---|---|---|
| **Own reqwest call to Resend** | — | One endpoint and about 5 types, the same pattern as the Anthropic client, testable with wiremock, with the idempotency header explicit. **Recommended.** |
| `resend-rs` | 0.35.0 (2026-10-18), MIT | The **official** SDK: repo `resend/resend-rust`, linked from resend.com/docs/send-with-rust. 817k / 386k downloads; 82 stars, 5 open [REG][GH][OD][21]. It defaults to **native-tls**; use `default-features = false, features = ["rustls-tls"]`. It pulls in `governor`, `mailparse` and `maybe-async`. The crates.io owner is an individual account (`AntoniosBarotsis`), not an org. Runner-up. |
| `aws-sdk-sesv2` + `aws-config` | 1.141.0 / 1.13.0 (both 2026-10-20), Apache-2.0, **MSRV 1.94.1** | Resolves to **225 crates** on its own [PRB]; releases almost daily. Heavy for one API call. If SES is chosen, `aws-sigv4` 1.6.2 (62 crates alone) plus reqwest is the lighter path [REG][PRB]. |
| `lettre` (SMTP) | 0.11.23 (2026-08-03), MIT, MSRV 1.85 | 21M / 7.8M downloads; 2.3k stars, 92 open [REG][GH]. It defaults to **native-tls**; use `tokio1-rustls` + `builder`. RUSTSEC-2026-0141 affects only the Boring TLS backend [RS]. Choose it only if the provider is reached over SMTP. |

**Typosquats and lookalikes (avoid):**
- `resend` 0.1.4 (2024-10, owner `WalterYongtaoWang`, 149 recent downloads): unrelated to Resend.
- `resend-rust` 0.1.3 (2024-08, owner `afteralec`, no repository link): its name copies the official *repo* name, but the official *crate* is `resend-rs` [REG].

### Inbound replies (raw MIME)

**mail-parser 0.11.9** (2026-09-09), Apache-2.0 OR MIT; 4.5M / 2.0M downloads; 466 stars, 3 open; pushed 2026-10-19 [REG][GH][22].
- Stalwart's parser. Its README claims it is "fuzzed" and "tested with MIRI", covers RFC 5322 and MIME with 41 charsets, and is zero-copy [OD/VC].
- The crate root carries `forbid(unsafe_code)` [PRB, source grep].
- It returns the text body, the HTML body and attachments per RFC 8621. That is what G1 needs to show the reply on the dashboard and forward it.
- The `main` branch now declares `rust-version = "1.98"`; the published 0.11.9 states none [GH]. The next release may raise our floor.
- `mail_parser` (underscore) resolves to the same crate on crates.io; **`mailparser` is an empty 0.0.0 placeholder** with no repository (owner `JayceFayne`) [REG]. Avoid it.

**mailparse 0.18.0** (2026-10-05), **0BSD**, MSRV 1.85; 15.5M / 4.4M downloads; 226 stars, 1 open [REG][GH]. Simpler and older. Runner-up.

### Bounces (G3)

- **No maintained Rust crate parses DSNs** (RFC 3464 `message/delivery-status`). crates.io searches for "dsn", "bounce" and "delivery status" found only unrelated or tiny crates [REG].
- mail-parser has no DSN-specific handling; a `delivery-status` part comes back as an ordinary part [PRB, source grep].
- `mail-auth` (Stalwart, 0.13.3) covers DKIM/SPF/DMARC/ARC and reports, and "DSN authentication" for DKIM2, but it does **not** parse bounce reasons [OD][23].
- **Recommendation:** use the provider's bounce events, not DSN email.
  - Resend sends `email.bounced` and `email.received` webhooks [OD][24], signed with Svix headers (`svix-id`, `svix-timestamp`, `svix-signature`).
  - Verification needs the **raw request body**; re-serialised JSON breaks the signature [OD][25].
  - Retries keep the same `svix-id`, so deduplicate on it [OD][24].
  - Match the bounce to our own send record by the provider's email ID. That satisfies "only bounces that match a message we actually sent" in G3.
  - Verify the HMAC with `hmac` + `sha2` and compare with `subtle`, or use the provider SDK.
- A hand-written parser for `message/delivery-status` blocks (header-like fields) stays a fallback, needed only if bounces arrive as email.

## 6. CSV and Excel import

**csv 1.4.0** (2025-10-17), Unlicense OR MIT, MSRV 1.73; 271M / 58M downloads [REG][26]. Stable; there is little to release. Use it with `serde` and `flexible(true)`, and validate each row into a typed struct. B2 needs per-row reasons. **H.**

**calamine 0.37.0** (2026-10-18), MIT, MSRV 1.88; 15.8M / 5.7M downloads; 2.5k stars, 44 open, pushed 2026-10-18 [REG][GH][27]. It is the only mature pure-Rust xlsx/xls/ods reader.

**Is it safe on untrusted files? Not on its own (H).** Recent releases fixed several out-of-memory and panic bugs [OD][28]:
- XLS out-of-memory in 0.29 and 0.32;
- an ODS "7 KB file → 17 billion cells" exhaustion in 0.33;
- unchecked CFB sector counts in 0.37.

But these are **still open** [GH][29]:
- **#693** (open, updated 2026-10-06): "`Range::from_sparse` densifies without bound: a **2 KB well-formed xlsx aborts the process**". The `<dimension>` / cell corners `A1` and `XFD1048576` make it allocate rows × columns. The failure is an **abort, not a panic**, so `catch_unwind` cannot contain it.
- **#684** (open): `read_shared_strings` reserves capacity from the attacker-controlled `uniqueCount` inside `Xlsx::new()`.
- **#694, #692** (open): unchecked u32 arithmetic on cell references and dimensions.

**Design implication:**
- Parse Excel files **in a short-lived child process** (the same binary with a hidden subcommand), with a memory rlimit and a wall-clock timeout. Return the rows as JSON over stdout.
- Cap the upload size (a 60-row client list is a few KB; 1 MB is generous).
- Read only the first sheet, stop after a row cap (e.g. 500), and use the cell-reader API rather than materialising a full `Range` where possible (**M**; check against the 0.37 API at implementation).
- CSV needs none of this.
- A failed or aborted parse shows "couldn't read this file; save it as CSV". B2's "a failed import changes nothing" then holds by construction.

**zip 9.0.5** (2026-10-17), MIT, MSRV 1.88 [REG]. calamine depends on it (^9.0) [REG], and we use it to *write* the I1 export.
- RUSTSEC-2026-0367/0368/0369 cover `extract()` (symlinks, SUID bits) and are patched in ≥ 9.0.0 [RS].
- We never call `extract()`. **H.**

## 7. Encryption at rest

Uploaded files are photos and PDFs of at most 10 MB (F1); F3 requires them to be encrypted and deleted on schedule.

| | chacha20poly1305 0.11.0 | aes-gcm 0.11.1 | age 0.12.1 |
|---|---|---|---|
| Released | 2026-06-28 | 2026-08-21 | 2026-07-14 |
| Licence / MSRV | Apache-2.0 OR MIT / 1.85 | Apache-2.0 OR MIT / 1.85 | MIT OR Apache-2.0 / 1.74 |
| Downloads | 98M / 30M | 184M / 58M | 6.8M / 3.9M |
| Notes | Includes **XChaCha20-Poly1305**: 192-bit nonces are safe to pick at random even when a key is reused; constant-time in software | Fast with AES-NI or ARMv8; a 96-bit nonce must never repeat under one key | A file format with streaming (STREAM) and recipients. Pulls **old RustCrypto generations** (chacha20poly1305 0.10, hkdf/hmac 0.12, rand 0.8) plus `i18n-embed` and `rust-embed`, which means duplicate crypto stacks. RUSTSEC-2024-0433 (plugin binary execution) affected old versions |

Sources [REG][GH][30][31][RS].

- Both RustCrypto AEADs moved to **aead 0.6** (hybrid-array, a `Generate` trait behind `getrandom`) [PRB, source]. Many online examples still show the 0.10 API; follow docs.rs for 0.11.
- RustCrypto/AEADs: 968 stars, 37 open, pushed 2026-10-13 [GH].

**Recommendation: envelope encryption with XChaCha20-Poly1305** (**M**, design judgement on verified facts):
1. **Master key (KEK).** 32 random bytes, base64 in an env var (`KOMPLETO_MASTER_KEY`). Load it into `secrecy::SecretBox<[u8; 32]>`, which is zeroised on drop. Store a `key_version` beside every wrapped key so the master key can rotate.
2. **Data key per file (DEK).** 32 bytes from `getrandom::fill`. Encrypt the file in one shot (10 MB fits in memory) with a random 24-byte nonce.
   - **AAD = `firm_id ‖ client_id ‖ file_id`.** A ciphertext moved to another firm's row then fails to decrypt, which gives A2 a second line of defence.
3. **Wrap the DEK** with the KEK (XChaCha20-Poly1305, its own nonce and AAD) and store it in Postgres. The ciphertext goes on disk or in object storage.
4. **Crypto-shredding for F3 and I2.** Deleting a file deletes its wrapped-DEK row. Copies of the ciphertext left in backups become unreadable once backups older than the deletion expire. Without this, "never kept beyond 90 days" leaks through database or disk backups.
5. `aes-gcm` is an equally valid runner-up, since each DEK encrypts exactly one message. XChaCha wins on misuse resistance.

**secrecy 0.10.3** (2024-10-09) is stable and complete (182M downloads) [REG]. Wrap the Anthropic and Resend API keys and the master key in it. **zeroize 1.9.1** (2026-10-06) [REG].

Also set reqwest's `HeaderValue::set_sensitive(true)` on `x-api-key`, so the key is redacted from `Debug` output.

## 8. Auth helpers

**Magic-link tokens (A1) and upload links (F2):**
- **Generation.** 32 bytes from `getrandom::fill` (getrandom 0.4.3), or `rand::rngs::SysRng` (rand 0.10 re-exports getrandom's `SysRng`) [PRB, source]. Encode as base64url without padding (`base64` 0.23.1) [REG].
- **Storage.** Store only **SHA-256(token)** (`sha2` 0.11.1, 2026-10-12, MSRV 1.88) [REG]. Look up by hash.
  - A timing-safe compare is unnecessary for an indexed hash lookup.
  - Use `subtle::ConstantTimeEq` where we compare secrets in memory: webhook HMACs and any CSRF token.
  - `subtle` 2.6.1 (2024-06-24, BSD-3-Clause) is stable; its last repo push was 2024-08 [REG][GH].
- **Single use and 15-minute expiry (A1).** One statement:

  ```sql
  UPDATE login_tokens SET used_at = $now
  WHERE token_hash = $1 AND used_at IS NULL AND expires_at > $now
  RETURNING firm_id
  ```

  It is atomic under concurrency.
- **`blake3` 1.8.7** (CC0-1.0 OR Apache-2.0 variants) is faster, but `sha2` is already in the tree through sqlx and hmac [PRB]. Prefer one hash.

**Sessions:**
- **`tower-sessions` 0.15.0** (2026-02-01), MIT; 4.3M / 1.2M downloads; 424 stars, 4 open [REG][GH].
- **Pitfall:** the published **`tower-sessions-sqlx-store` 0.15.0 (2025-01-01) requires `sqlx ^0.8` and `tower-sessions-core ^0.14`** [REG]. It is **incompatible** with both sqlx 0.9 and tower-sessions 0.15.
  - The fix is on `main` (commit "Update sqlx to 0.9.0", 2026-07-12; Cargo.toml now `sqlx 0.9.0`, `tower-sessions-core 0.15.0`), but it is **unpublished** [GH].
  - Issues #83 and #86 report the core mismatch; PR #87 is open [GH].
  - Using it today means a git dependency.
- **Recommendation: a hand-rolled `sessions` table:**
  - an opaque 32-byte token in a `Secure; HttpOnly; SameSite=Lax` cookie via `axum-extra` `CookieJar`;
  - SHA-256 of the token in the table, with `firm_id`, `created_at`, `expires_at` and `last_seen_at`;
  - an extractor that resolves the cookie to `FirmId`.
  
  That is about 100 lines. It removes a dependency and puts the firm-scoping (A2) in our own code. tower-sessions is the runner-up once the store is published. **M.**
- **Pitfall: link scanners consume one-time links.** Email security scanners fetch links (with GET) before the user clicks.
  - The GET on a magic link should render a page with a "Log in" button, and a **POST consumes the token**.
  - Upload links (F2) are not single-use, so they are unaffected. But their GET must never trigger a side effect.
  - This is a known industry behaviour; no source was fetched this pass. **M.**
- **Rate limiting the login form:** `tower_governor` 0.8.0 (2025-08-14) / `governor` 0.10.4 (2025-12-16) [REG]. Optional for an invite-only pilot.

## 9. Observability

- **tracing 0.1.44** (2025-12-18) and **tracing-subscriber 0.3.23** (2026-03-13), MIT, MSRV 1.65 [REG][32].
  - Use features `env-filter` and `json`: JSON lines to stdout in production, pretty output in development.
  - **RUSTSEC-2025-0055** (ANSI escape sequences from logged user input poisoning terminals) is patched in ≥ 0.3.20 [RS]. Still disable ANSI in production.
- **tracing-opentelemetry 0.34.0** (2026-09-23) needs `opentelemetry` ^0.33 (0.33.1, 2026-10-09). All are 0.x and must be bumped together [REG][33].
  - **Not worth it at pilot scale (M).** It needs one service, 3–5 firms and a collector or backend to run and pay for.
  - J1 (cost per firm) and J2 (audit log) are **database records**, not traces.
  - Add OTel when there is a second process or a latency question that logs can't answer.
- **Pitfall: privacy in logs.** The spec keeps TINs, amounts and file contents away from the AI, and logs deserve the same care.
  - Use `#[instrument(skip_all, fields(firm_id, job_id))]`.
  - Never log email bodies, client names or addresses, or request bodies.
  - `tower-http` `trace` with default settings logs URIs. Upload-link tokens in paths would be logged, so redact them, or carry them in a path segment that a custom `make_span` drops.

## 10. Testing

**Postgres:** `#[sqlx::test]` (sqlx 0.9.0, needs the `migrate` feature) [OD][34].
- It creates a **fresh database per test** and applies `migrations/` automatically (or `migrator = "crate::MIGRATOR"`). It hands the test a `PgPool` or connection, and supports fixtures.
- It needs `DATABASE_URL` pointing at a server where it can `CREATE DATABASE`.
- Databases from failed tests stay in place for inspection and are deleted the next time a test binary runs [OD][34].
- **Recommendation:** run Postgres once (`docker compose` locally, a `services: postgres` container in CI) and use `#[sqlx::test]`.
- **testcontainers 0.28.0** (2026-08-06) / **testcontainers-modules 0.15.0** (2026-02-21), MSRV 1.88, are the runner-up [REG][35].
  - **Pitfall:** modules 0.15.0 depends on `testcontainers ^0.27`, not 0.28 [REG]. Use the re-export `testcontainers_modules::testcontainers` rather than a separate `testcontainers` dependency.

**Snapshots: insta 1.49.0** (2026-10-03), Apache-2.0; 2.97k stars [REG][GH]. Snapshot:
- rendered HTML pages;
- the exact Anthropic request JSON (D6: a test that inspects every outgoing request for TIN-shaped numbers, amounts or file contents);
- email bodies, including the footer and sender format (E2);
- import error reports (B2).

Use the `json` and `redactions` features for IDs and timestamps. **H.**

**HTTP mocks: wiremock 0.6.5** (2025-08-24), MIT/Apache-2.0; 92M / 28M downloads [REG][36].
- **Its last commit is 2025-08-24** (14 months ago; 805 stars, 33 open) [GH]. The API is stable and test-only, so the stall is a low risk.
- Mount the mock with `.expect(1)` to assert "sent exactly once" for E1, and use request matchers to check that the `Idempotency-Key` header is present.
- **Runner-up: httpmock 0.9.0** (2026-10-20, MIT, MSRV 1.88; 675 stars, actively maintained) [REG][GH]. `mockito` 1.7.2 (2026-02-02) is a third option.

**Fake clock (D4, F2, F3, I2, A1):**
- **Recommended: an own `trait Clock { fn now(&self) -> OffsetDateTime; }`** with `SystemClock` and `FixedClock`. **Every SQL time comparison takes `$now` as a bind parameter**, never `now()`. Then "advance the clock 31 days and run the deletion job" (F3) is a plain test.
- `tokio::time::pause()` / `advance()` (tokio `test-util` feature) control **tokio timers only**, not `SystemTime` and not Postgres. Use it only for the job-poll loop.
- `mock_instant` 0.6.1 mocks `std` `Instant`/`SystemTime` but not Postgres, so it is not enough on its own [REG].

**E1 crash test:**
- Build a test-only failpoint behind a cargo feature, for example `KOMPLETO_FAILPOINT=after_send_before_record`, which calls `std::process::abort()`.
- Run the real binary as a child process against wiremock, restart it, and assert the mock saw one request (or two with the same `Idempotency-Key`, if the provider deduplicates).

**Runner:** `cargo-nextest` is already installed locally (version not checked). It runs each test in its own process, which suits failpoints.

## 11. Errors and config

- **thiserror 2.0.21** (2026-09-23) for per-module error enums. **anyhow 1.0.104** (2026-07-18) only in `main` and startup wiring. This matches the house style ("thiserror for library errors, anyhow for binaries only", `.claude/skills/rust-best-practices`) [REG].
  - Handlers return an `AppError` enum that implements `IntoResponse`.
  - It maps "not found" and "belongs to another firm" to the **same 404** (A2) and logs the cause, never showing it to the user.
- **Config: a plain typed `Config::from_env()`**, about 60 lines, failing fast at startup. Call `dotenvy::dotenv()` only under `cfg(debug_assertions)` or a dev flag.
  - `dotenvy` 0.15.7 was last released **2023-03-22**, but the repo is active (pushed 2026-08-18) and the crate is feature-complete [REG][GH].
  - `figment` 0.10.19 was last released **2024-05-17**, though the repo was pushed 2026-10-13 [REG][GH].
  - `config` 0.15.27 (2026-09-30) is active but has 138 open, and its layered-file model is more than one service with ~10 settings needs [REG][GH].
  - **Recommendation:** env only; figment is the runner-up if layered files ever appear. **M.**
- `envy` 0.4.2 (2021-01-04) is stale; avoid it [REG].

## 12. Tooling

- **Rust stable is 1.99.1** (tag published 2026-10-16; 1.99.0 on 2026-10-01). The `static.rust-lang.org` stable channel manifest lists `rustc 1.99.1 (58cf8f9c3 2026-10-13)` [GH][OD][37].
  - The local toolchain is **1.98.1**, so run `rustup update`.
  - On the six-week cadence, 1.100 should follow around 2026-11-12 (inferred, not announced).
- **Edition 2024** is the current edition; `cargo new` on cargo 1.98.1 writes `edition = "2024"` [PRB]. Use it.
- **cargo-deny 0.20.2** (2026-07-09), MIT OR Apache-2.0, MSRV 1.88; 2.5k stars, pushed 2026-10-20 [REG][GH][38]. One tool covers `advisories` (RustSec), `licenses`, `bans` (duplicates, e.g. tower-http 0.6/0.7 and sha2 0.10/0.11) and `sources` (forbid unknown git sources).
  - **cargo-audit 0.22.2** (2026-06-05) duplicates the advisories check. Optional.
- **Licence allow-list** from the probe's 334 packages [PRB]:
  - MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-3-Clause, ISC, Unicode-3.0, Zlib, CDLA-Permissive-2.0 (webpki-roots), Unlicense, 0BSD, MIT-0, BSL-1.0, CC0-1.0.
  - No copyleft is required; LGPL appears only as an OR alternative in `r-efi`.
  - `aws-lc-sys` has a compound ISC/Apache/MIT/BSD expression; all of its terms are permissive.
- **Advisory check:** checked against the RustSec advisory-db (last commit 2026-10-20) [RS], every advisory touching the probe's resolved versions is already patched:
  - rustls 0.23.46 (RUSTSEC-2026-0347/0348/0349, fixed in ≥ 0.23.46, which is exactly the locked version);
  - zip 9.0.5, h2 0.4.20, quick-xml 0.42.0, tracing-subscriber 0.3.23, sqlx 0.9.0.
  
  This is a spot check, not a `cargo deny` run. Keep **rustls ≥ 0.23.46** in the lockfile.
- **Clippy:** in `Cargo.toml` (single crate, so no workspace table needed):

  ```toml
  [lints.rust]
  unsafe_code = "forbid"

  [lints.clippy]
  pedantic = { level = "warn", priority = -1 }
  unwrap_used = "deny"
  expect_used = "deny"   # allow in tests via clippy.toml: allow-expect-in-tests / allow-unwrap-in-tests
  panic = "deny"
  dbg_macro = "deny"
  todo = "warn"
  module_name_repetitions = "allow"  # noisy with one crate and many modules
  missing_errors_doc = "allow"       # binary crate, not a public library
  ```

  CI runs `cargo clippy --all-targets --locked -- -D warnings`, as the house skill says [house style].

## Pitfalls to design around

1. **calamine can abort the process on a 2 KB crafted xlsx** (open #693; also #684). Parse Excel in a child process with a memory and time limit; CSV is safe in process.
2. **E1 depends on the email provider.** Resend has a 24-hour `Idempotency-Key`; SES `SendEmail` has none. With SES, a crash between "sent" and "recorded" must be reconciled from provider events, never retried. Persist the key before the call.
3. **`tower-sessions-sqlx-store` 0.15.0 on crates.io is incompatible with sqlx 0.9 and tower-sessions 0.15.** Hand-roll sessions, or take a git dependency.
4. **Two rustls crypto providers.** sqlx's `tls-rustls` (ring) plus reqwest 0.13 (aws-lc-rs). Use sqlx `tls-rustls-aws-lc-rs`. aws-lc-sys needs a C toolchain at build time, and the container needs `ca-certificates`.
5. **Email link scanners consume one-time login links.** Consume the magic-link token on POST from a confirmation page, not on GET (A1).
6. **Time in SQL.** Bind `$now` from an injected clock in every query; `now()` makes D4, F2, F3 and I2 untestable. tokio's paused clock does not reach Postgres.
7. **Backups defeat deletion.** Use per-file DEKs and delete the wrapped key (crypto-shredding), so the 90-day promise (F3) and account closure (I2) hold for backups too. Bind firm and file IDs as AEAD associated data.
8. **axum's 2 MB default body limit** rejects 10 MB uploads (F1); raise it on the upload route only.
9. **Webhook signatures need the raw body.** Extract `Bytes`, verify the Svix HMAC, then parse. Deduplicate on `svix-id`.
10. **sqlx offline builds.** Commit `.sqlx/`, set `SQLX_OFFLINE=true` in CI, and check with `cargo sqlx prepare --check`. 0.9's `SqlSafeStr` rejects `format!`-built SQL unless wrapped in `AssertSqlSafe`.
11. **Secrets and PII in logs.** Mark the API-key header sensitive, wrap keys in `secrecy`, use `skip_all` on spans, and keep upload tokens out of traced URIs.
12. **Churn ahead:** axum 0.9 (unreleased, breaking), the next mail-parser release (main declares MSRV 1.98) and the OTel 0.x crates. Pin with `Cargo.lock` and upgrade on purpose.

## Unresolved questions

1. **Email provider: Resend or SES?** It decides E1's reconciliation design and the bounce and inbound path. Resend has idempotency keys, signed webhooks and inbound receiving. **Not found** this pass:
   - whether Resend inbound exposes the **raw MIME** of a received email (the spec assumes raw MIME; `docs/dashboard/receiving/email-content` returned 404);
   - Resend's pricing and PH availability;
   - SES's inbound (receipt-rule) region availability for our account.
   
   An owner decision, after a short provider check.
2. **Where uploaded ciphertext lives:** local disk on one VPS, or S3-compatible object storage. This affects whether an S3 client crate is needed (not researched here).
3. **calamine child-process isolation:** whether `setrlimit` via `libc` or `rustix` behaves the same on the chosen Linux host as on macOS (needs a test on the deploy target). Also whether a cheaper pre-check exists (scanning `<dimension>` and `uniqueCount` with `quick-xml` before calling calamine).
4. **Whether reqwest has a default request timeout.** Stated here from memory as "none"; check docs.rs at implementation.
5. **`time` vs `jiff`** for dates. The Philippines uses UTC+8 with no DST, so `time` with a fixed offset suffices. `jiff` 0.2.38 + `jiff-sqlx` 0.2.0 (supports sqlx 0.9) is nicer if time-zone rules ever matter [REG].
6. **htmx 2.0.11 vs 4.0.0:** 4.0.0 is tagged `next`, not `latest`. Revisit only if 4 becomes `latest` before implementation.

## Sources

All accessed 2026-10-21. crates.io data comes from `https://crates.io/api/v1/crates/<name>` (and `/<version>/dependencies`, `/owners`); GitHub data from `https://api.github.com/repos/<owner>/<repo>`.

1. tokio: crates.io/crates/tokio (1.54.0, 2026-10-19). [REG] · H.
2. axum: crates.io/crates/axum (0.8.9, 2026-04-14). [REG] · H.
3. github.com/tokio-rs/axum (27.5k stars, 110 open, pushed 2026-10-20). [GH] · H.
4. axum CHANGELOG, raw.githubusercontent.com/tokio-rs/axum/main/axum/CHANGELOG.md ("Unreleased" breaking list). [OD] · H.
5. tower-http: crates.io/crates/tower-http (0.7.1, 2026-08-31; 0.7.0, 2026-06-15). [REG] · H.
6. tower-http CHANGELOG, raw.githubusercontent.com/tower-rs/tower-http/main/tower-http/CHANGELOG.md (0.7.0 `csrf`, breaking changes). [OD] · H.
7. axum-extra: crates.io/crates/axum-extra (0.12.6, 2026-04-14; features). [REG] · H.
8. sqlx: crates.io/crates/sqlx (0.9.0, 2026-05-21, MSRV 1.94.0). [REG] · H.
9. sqlx CHANGELOG, raw.githubusercontent.com/transact-rs/sqlx/main/CHANGELOG.md (0.9.0 section dated 2026-05-06; repo move to transact-rs). [OD] · H.
10. diesel 2.3.14 / diesel-async 0.9.2: crates.io/crates/diesel, crates.io/crates/diesel-async; github.com/diesel-rs/diesel_async. [REG][GH] · H.
11. apalis: crates.io/crates/apalis (1.0.0, 2026-10-12); github.com/apalis-dev/apalis. [REG][GH] · H.
12. apalis-postgres: crates.io/crates/apalis-postgres (1.0.0, 2026-10-12, depends on sqlx ^0.9); github.com/apalis-dev/apalis-postgres (README, Cargo.toml, LICENSE = MIT). [REG][GH][OD] · H.
13. askama 0.16.1, askama_web 0.16.0 (feature `axum-0.8`), askama_axum 0.5.0+deprecated: crates.io; github.com/askama-rs/askama. [REG][GH] · H.
14. maud 0.27.0: crates.io/crates/maud; github.com/lambda-fairy/maud. [REG][GH] · H.
15. minijinja 3.0.1: crates.io/crates/minijinja; github.com/mitsuhiko/minijinja. [REG][GH] · H.
16. htmx: registry.npmjs.org/htmx.org (dist-tags `latest` 2.0.11, 2026-09-22; `next` 4.0.0, 2026-08-28; 0BSD). [OD] · H.
17. reqwest: crates.io/crates/reqwest (0.13.5, 2026-09-08; `default-tls` → `rustls`); github.com/seanmonstar/reqwest. [REG][GH] · H.
18. reqwest CHANGELOG, raw.githubusercontent.com/seanmonstar/reqwest/master/CHANGELOG.md (v0.13.0: rustls default, aws-lc provider). [OD] · H.
19. Resend, "Idempotency keys", resend.com/docs/dashboard/emails/idempotency-keys; "Send email", resend.com/docs/api-reference/emails/send-email. [OD] · H.
20. AWS, "SendEmail" (SES API v2), docs.aws.amazon.com/ses/latest/APIReference-V2/API_SendEmail.html (no idempotency token found). [OD] · M (absence of evidence on one page).
21. resend-rs 0.35.0: crates.io/crates/resend-rs; github.com/resend/resend-rust (README: "The official Resend client"); resend.com/docs/send-with-rust. [REG][GH][OD] · H.
22. mail-parser 0.11.9: crates.io/crates/mail-parser; github.com/stalwartlabs/mail-parser (README, main Cargo.toml `rust-version = "1.98"`); crate source 0.11.9 grep. [REG][GH][OD][PRB] · H.
23. mail-auth 0.13.3: crates.io/crates/mail-auth; github.com/stalwartlabs/mail-auth README. [REG][OD] · H.
24. Resend, "Event types", resend.com/docs/dashboard/webhooks/event-types (`email.bounced`, `email.received`; retries keep `svix-id`). [OD] · H.
25. Resend, "Verify webhook requests", resend.com/docs/dashboard/webhooks/verify-webhooks-requests (raw body; Svix headers). [OD] · H.
26. csv 1.4.0: crates.io/crates/csv; github.com/BurntSushi/rust-csv. [REG][GH] · H.
27. calamine 0.37.0: crates.io/crates/calamine; github.com/tafia/calamine. [REG][GH] · H.
28. calamine Changelog, raw.githubusercontent.com/tafia/calamine/master/Changelog.md (0.29–0.37 OOM and panic fixes). [OD] · H.
29. calamine issues #693, #684, #694, #692 (open), github.com/tafia/calamine/issues. [GH] · H.
30. chacha20poly1305 0.11.0 / aes-gcm 0.11.1: crates.io; github.com/RustCrypto/AEADs; aead 0.6.1 source. [REG][GH][PRB] · H.
31. age 0.12.1: crates.io/crates/age (dependencies); github.com/str4d/rage. [REG][GH] · H.
32. tracing 0.1.44 / tracing-subscriber 0.3.23: crates.io. [REG] · H.
33. tracing-opentelemetry 0.34.0 / opentelemetry 0.33.1: crates.io; github.com/tokio-rs/tracing-opentelemetry. [REG][GH] · H.
34. sqlx 0.9.0 `src/macros/test.md` (the `#[sqlx::test]` docs, also on docs.rs/sqlx/0.9.0). [OD] · H.
35. testcontainers 0.28.0 / testcontainers-modules 0.15.0 (depends on testcontainers ^0.27): crates.io; github.com/testcontainers/testcontainers-rs. [REG][GH] · H.
36. wiremock 0.6.5: crates.io; github.com/LukeMathWalker/wiremock-rs (last commit 2025-08-24). httpmock 0.9.0, mockito 1.7.2: crates.io; github.com/httpmock/httpmock. [REG][GH] · H.
37. Rust releases: api.github.com/repos/rust-lang/rust/releases (1.99.1, 2026-10-16); static.rust-lang.org/dist/channel-rust-stable.toml. [GH][OD] · H.
38. cargo-deny 0.20.2 / cargo-audit 0.22.2: crates.io; github.com/EmbarkStudios/cargo-deny. [REG][GH] · H.
39. tower-sessions 0.15.0 / tower-sessions-sqlx-store 0.15.0 (deps sqlx ^0.8, tower-sessions-core ^0.14): crates.io; github.com/maxcountryman/tower-sessions-stores (main Cargo.toml, commits, issues #83 #86, PR #87). [REG][GH] · H.
40. Lookalike checks: crates.io/crates/resend (0.1.4), resend-rust (0.1.3), mailparser (0.0.0), askama-axum, mail_parser, tower_sessions (owners via `/owners`). [REG] · H.
41. Other registry data (versions, licences, MSRV): getrandom 0.4.3, rand 0.10.3, sha2 0.11.1, hmac 0.13.0, subtle 2.6.1, blake3 1.8.7, base64 0.23.1, secrecy 0.10.3, zeroize 1.9.1, zip 9.0.5, thiserror 2.0.21, anyhow 1.0.104, figment 0.10.19, config 0.15.27, dotenvy 0.15.7, envy 0.4.2, time 0.3.55, jiff 0.2.38, jiff-sqlx 0.2.0, uuid 1.28.0, insta 1.49.0, mock_instant 0.6.1, aws-sdk-sesv2 1.141.0, aws-config 1.13.0, aws-sigv4 1.6.2, lettre 0.11.23, mailparse 0.18.0, axum-htmx 0.8.1, tower_governor 0.8.0, governor 0.10.4. [REG] · H.
42. [RS] RustSec advisory-db, github.com/rustsec/advisory-db (shallow clone, last commit 2026-10-20): RUSTSEC-2026-0347/0348/0349 (rustls), 2026-0367/0368/0369 (zip), 2026-0258 (h2), 2026-0194/0195 (quick-xml), 2026-0141 (lettre, Boring TLS), 2025-0055 (tracing-subscriber), 2024-0433 (age), 2024-0363 (sqlx). [OD] · H.
43. [PRB] Local probe, 2026-10-21: `cargo generate-lockfile`, `cargo tree -d`, `cargo metadata` licence census and `SQLX_OFFLINE=true cargo check` on rustc 1.98.1, in the session scratchpad; a separate resolution of `aws-config` + `aws-sdk-sesv2` (225 crates) and `aws-sigv4` (62 crates). · H for "resolves and type-checks"; says nothing about runtime behaviour.
