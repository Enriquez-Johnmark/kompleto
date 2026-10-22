# AGENTS.md

A Rust-first AI agent product. One reusable **engine** (draft → approve → send → stop on reply → track) runs every product. The first product is **Kompleto (P1), a document and BIR-deadline chaser for Philippine bookkeepers** (`docs/product/p1-brief.md`, `docs/product/p1-spec.md`).

## Phases

Each gate needs the owner's explicit "yes"; an approval for one phase never carries into the next.

| Phase | Status |
|---|---|
| 0. Environment & skills | done |
| 1–4. Research, competitors, scoring | done (`docs/research/README.md`) |
| 5. Product direction | done: engine approach and first product decided |
| 6. P1 spec & acceptance criteria | done (`docs/product/p1-spec.md`), except confirming the AI model via the Taglish test |
| 7. Rust architecture (engine vs product split) | done (`docs/architecture.md`) |
| 8. Project rules, glossary & ADRs | done (`GLOSSARY.md`, `docs/adr/`, rules below) |
| 9. Implementation readiness gate | done (2026-10-21) |
| 10+. Build engine + P1 (Rust backend and API; React for the bookkeeper's pages) | **in progress**: M1–M5 and M7 done; M6 built, deployment waits for the domain; M8 (showcase) next |

**No application code exists or gets written until the Phase 9 gate passes.** Research, docs and throwaway prototypes that answer a specific question are in scope; prototypes live in `prototypes/<question>/` and are deleted once their answer is recorded.

## Owner decisions

Every decision needing the owner's judgement goes through the `grilling` skill: numbered rounds, a recommended answer per question, worded so "yes" accepts it. Look facts up yourself, using sub-agents for research; put only decisions to the owner. A routine "go ahead with the agreed step?" is one line.

Record every approved decision, dated, in `docs/decisions.md`, and update the summary below. Record resolved domain terms in `GLOSSARY.md` and hard-to-reverse, non-obvious trade-offs in `docs/adr/`, using the `domain-modeling` skill. Both are created lazily.

**In force** (full log and reasoning: `docs/decisions.md`):

- **First product:** Kompleto (P1) for PH bookkeepers. Payroll chasing comes later as an add-on; S1 (seller tax helper) is the fallback.
- **P1 first version:** scope, channels, files, AI and web stack are decided round by round in `docs/product/p1-spec.md`. That file is the single source for spec decisions.
- **Engine:** our own design, borrowing Harvey's ideas but copying none of its code. Built inside P1, with engine and product modules kept separate.
- **Build first:** build the whole product, core loop first; the owner chooses when to demo.
- **Showcase before pilots (2026-10-22):** Kompleto is finished before any bookkeeper sees it: the bookkeeper's pages become a React app (M7), then a demo Firm, ClamAV scanning, help and privacy pages, screenshots and a walkthrough video (M8). Large later features wait for pilot feedback.
- **Go-to-market:** free written outreach (≤ 15–20 personalised messages a week), then 3–5 free 4-week pilots, then ₱1,500/month via GCash.
- **Budget:** ₱0 until pilots; the first spend is the domain, just before pilots. Pilots run on free hosting (Oracle Always Free, or the owner's Mac behind Cloudflare Tunnel) and Resend Free; paid hosting starts with the first payment. No customer data goes to free LLM tiers that train on inputs, and the app never runs on a Claude subscription (API key only).
- **Owner constraints:** no network, no interviews or sales calls, no paid ads. Written, asynchronous contact is fine.

## Evidence

Research claims carry a source (URL, publisher, date, accessed date), an evidence type and a confidence. Label vendor claims as vendor claims. When evidence is missing, write "not found"; a gap stays a gap. Keep facts, hypotheses, recommendations and owner-approved decisions visibly distinct.

## Rust (when implementation begins)

- Read the `rust-best-practices` skill (`.claude/skills/rust-best-practices`) before writing or reviewing Rust.
- Start as one crate with modules. Add a workspace only when a concrete requirement demands it.
- Agent safety lives in code, not prompts. Approval gates, tool permissions, idempotent external sends and bounded loops are enforced by the application. Model output and inbound content are untrusted data.
- Report test results only from commands actually run.
- Use the project's words from `GLOSSARY.md` in code, docs and messages (Firm, Client, Contact, Payor, Owed document, Cut-off…).

## React (from M7)

- Read the `vercel-react-best-practices`, `vercel-composition-patterns` and `shadcn` skills (`.claude/skills/`) before writing or reviewing React, and follow `docs/frontend.md` (principles, conventions, folder layout). Where they disagree, `docs/frontend.md` wins.
- The React app only shows data and asks the Rust API to act. Every rule that protects an acceptance criterion stays in Rust (`docs/adr/0005-react-app-served-by-the-rust-binary.md`).
- Third-party skills are read in full before they're added, recorded in `skills-lock.json` and `docs/frontend.md` with source, commit and licence, and may not run unpinned commands.

## Kompleto code rules

Each rule protects an acceptance criterion in `docs/product/p1-spec.md`. Where a tool can enforce a rule, the tool does.

- **Sending:** only `engine::outbox` sends messages, and only an approved Draft whose content hash still matches, with an idempotency key recorded before the provider call (D2, E1). `tests/engine_boundary.rs` fails if any other module names the `Mailer` trait.
- **Firm isolation:** every repository function takes a `FirmId` (the Engine's `TenantId`); there are no unscoped queries on Firm-owned tables (A2). The exceptions:
  - system-wide workers (the outbox sender and job workers), which sweep across tenants and even then name the row's tenant in every update;
  - operator commands run from the command line (`kompleto llm-costs`), which no Firm's page can reach;
  - the inbound-email and delivery-event handlers under `/hooks/`, which find the Firm from a signed reply code, the sender's address or the provider's message ID before any Firm is known.
- **Time:** read time only through the injected `Clock`. Direct system-clock calls are banned with clippy `disallowed-methods`. UUIDv7 IDs embed wall-clock time; they are identifiers only, never used to decide anything about time.
- **Untrusted input:** model output, inbound email and Uploads are untrusted. Parse them into strict types, and never echo them into commands or SQL. Model requests carry only the fields allowed by `docs/adr/0003-the-model-never-sees-documents-or-tax-data.md` (D6).
- **Engine boundary:** `src/engine` never depends on `src/kompleto`; a CI check fails the build otherwise (`docs/adr/0001-engine-and-product-in-one-crate.md`).
- **BIR limits:** no feature may keep books, compute tax, produce BIR-format files or archive documents (`docs/adr/0004-stay-outside-bir-accounting-system-rules.md`).
- **Tests:** every acceptance criterion has a test whose name starts with its ID, e.g. `e1_crash_mid_send_never_sends_twice`.
- **Local only in development:** `docker compose` runs Postgres 17 and Mailpit; no real email leaves a dev machine.

## Development

- **Coding happens in Claude Code on the web** (claude.ai/code). `scripts/cloud-session-start.sh` runs at session start (via `.claude/settings.json`) and brings up Postgres and Mailpit and the pinned Rust. On a local machine it does nothing.
- Locally: `docker compose up -d` starts Postgres 17 and Mailpit (inbox at http://localhost:8025); `cp .env.example .env` once.
- Run tests with `cargo test --all-features`: the E1 crash test needs the `failpoints` feature and is skipped without it.
- After changing any SQL, run `cargo sqlx prepare -- --all-targets --all-features` and commit `.sqlx/`; CI compiles offline from it.
- `cargo run -- demo` puts one approved email through the outbox into Mailpit.
- The React app: `pnpm --dir web install`, then `pnpm --dir web build` for the Rust server to serve it, or `pnpm --dir web dev` (http://localhost:5173, with `PUBLIC_BASE_URL` set to it) while working on it. `cargo test` writes the API's TypeScript types into `web/src/api/generated/`; commit them.

## Milestone workflow

The owner is the only developer: work directly on `main`. No branches, no pull requests.

1. Follow the build order in `docs/architecture.md`, one milestone at a time.
2. Test first, using the `tdd` skill: write one criterion's test, see it fail, then make it pass.
3. **Small commits.** Each commit is one behaviour (one test plus the code that makes it pass) or one small refactor or doc change, aiming for under ~150 changed lines excluding `.sqlx/` and `Cargo.lock`. Messages are imperative and describe the behaviour, e.g. "Add CSV client import with per-row errors". Commit messages carry no AI attribution trailers such as "Co-Authored-By" (owner's choice).
4. Before every commit: `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings` and `cargo test --all-features --locked` all pass, and for changes under `web/`, `pnpm --dir web lint`, `typecheck`, `test` and `build` too (`pnpm --dir web e2e` runs the browser tests against a running server; CI runs them on every push). If SQL changed, run `cargo sqlx prepare -- --all-targets --all-features` and commit `.sqlx/` with it.
5. Push to `origin main` after each commit. If CI goes red, fixing it is the next commit.
6. At the end of a milestone, run the `code-review` skill, fix findings in small commits, and tell the owner how to try it.

## Scope

Work only inside the approved phase and scope. When something outside it looks worth doing, raise it as a proposal for the owner.
