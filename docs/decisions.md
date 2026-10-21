# Decision log

Every owner-approved decision, newest first, with the date it was made. Decisions that a later one replaced stay here, marked **superseded**, so the reasoning trail survives. `AGENTS.md` lists only the decisions in force.

## 2026-10-21

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
