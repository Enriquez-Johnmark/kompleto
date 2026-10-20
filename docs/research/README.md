# Research index

Research for Phases 1–5, carried out 2026-10-18 to 2026-10-20 by parallel research agents plus synthesis. All facts carry sources inside each file. **Nothing here is customer-validated**: no customer has used or paid for anything yet.

## Outcome

- **First product: P1**, a document and BIR-deadline chaser for PH bookkeepers (owner-approved 2026-10-21). It scores 76.0 Balanced and is first under all three weight profiles. It passed its kill-checks with two score cuts. See [../product/p1-brief.md](../product/p1-brief.md).
- **Fallback: S1**, the seller tax helper (74.4). It has not been kill-checked yet.
- **Recommendation, not yet decided:** use the engine for our own written outreach once P1's core works, and do outreach by hand until then ([harvey-market.md](harvey-market.md)).
- **Ruled out as first products:** AI add-ons inside ERP, CRM or HR software (vendors ship the agents themselves; app stores are gated) and a Rust Harvey competitor (a free, fast-growing incumbent; C6 = 1).

## Start here

1. [opportunity-scorecard.md](opportunity-scorecard.md): the ranking, sensitivity analysis and every update (§7–12 cover the decisions that led to P1).
2. [p1-kill-checks.md](p1-kill-checks.md): the five checks P1 passed, and the hard limits they impose.
3. [opportunity-landscape.md](opportunity-landscape.md): the original inventory and why each early candidate was kept or cut.

## Files by area

| Area | File | Status | Main remaining gaps |
|---|---|---|---|
| Scoring method | [scoring-method.md](scoring-method.md) + [scoring/](scoring/) | **Complete**; fixed before scoring | — |
| Business opportunities | [opportunities-business.md](opportunities-business.md) | **Complete** for shortlisting | Tour-operator and tutoring evidence is hypothesis-only |
| Individual & dev-infra opportunities | [opportunities-individual-and-infra.md](opportunities-individual-and-infra.md) | **Complete** for shortlisting | Several funding figures from secondary sources |
| Philippine market | [market-philippines.md](market-philippines.md) | **Partial** | No PH survey of SME software budgets or willingness to pay |
| Competitors (messaging, CRM, booking) | [competitor-analysis.md](competitor-analysis.md), [follow-up-competitors.md](follow-up-competitors.md) | **Complete** | Meta Business Agent PH pricing |
| Integrations, platform rules, LLM & hosting cost | [integration-constraints.md](integration-constraints.md) | **Partial**; LLM section superseded by llm-options.md | Hosting cost and Manila–Singapore latency |
| P1 launch prerequisites (registration, payments, privacy, email) | [p1-launch-prerequisites.md](p1-launch-prerequisites.md) | **Complete** (2026-10-21) | LGU permit fees; NPC registration fee; invoice-booklet cost; NPC circular text blocked (law-firm summaries used) |
| Hosting, storage & email infrastructure | [hosting-and-email-infra.md](hosting-and-email-infra.md) | **Complete** (2026-10-21) | Whether Vultr charges PH VAT; SES production access for a new account; 11 open questions listed in the file |
| LLM models, prices & data terms | [llm-options.md](llm-options.md) | **Complete** (2026-10-21) | Taglish quality untested for 2026 models; Anthropic minimum top-up and PH card acceptance |
| Self-serve distribution (Xero/QBO/Zoho stores) | [self-serve-distribution.md](self-serve-distribution.md) | **Complete** | — |
| New ideas | [new-ideas-philippines.md](new-ideas-philippines.md), [new-ideas-marketplaces.md](new-ideas-marketplaces.md) | **Complete** for shortlisting | S1: BIR accreditation; overlap with Taxumo/Juan |
| PH industry sweep (where P1 came from) | [ph-industries-services.md](ph-industries-services.md), [ph-industries-sectors.md](ph-industries-sectors.md) | **Complete** for shortlisting | — |
| AI add-ons for HR, ERP, CRM & other SaaS | [ai-addons-hr.md](ai-addons-hr.md), [ai-addons-erp.md](ai-addons-erp.md), [ai-addons-crm-saas.md](ai-addons-crm-saas.md) | **Complete** for shortlisting; depth uneven | Whether PH bookkeepers run client payroll (matters for the payroll add-on) |
| P1 kill-checks | [p1-kill-checks.md](p1-kill-checks.md) | **Complete**: no kill | Current BIR accredited tax-agent list; number of independent bookkeepers |
| Harvey teardown | [harvey-teardown.md](harvey-teardown.md) | **Complete** (commit `e114e95`, MIT) | — |
| Harvey market & AI-SDR competitors | [harvey-market.md](harvey-market.md) | **Complete** | PH bookkeepers' reply rates to cold email; NPC guidance PDFs (HTTP 403) |
| Rust crates for the build | [rust-crates.md](rust-crates.md) | **Complete** (2026-10-21); versions from crates.io/GitHub APIs | Resend vs SES for send-once; see its Unresolved questions |
| Agent architectures & Rust ecosystem | [agent-architecture-comparison.md](agent-architecture-comparison.md) | **Complete** | — |
| Validation plan (B, historical) | [customer-validation-plan.md](customer-validation-plan.md) | **Superseded**: pilot measures now live in `docs/product/p1-spec.md` | — |
| Customer evidence | — | **Not started**: comes from outreach and free pilots once P1 is built | Everything |

## Technical direction

**Owner-approved** (see [../decisions.md](../decisions.md)): our own engine design, informed by Harvey but copying none of its code. It is built inside P1, with engine and product modules kept separate.

**Preliminary, for the Phase 7 gate.** Details are in [agent-architecture-comparison.md §4](agent-architecture-comparison.md) and [harvey-teardown.md](harvey-teardown.md).

- **Shape:** one bounded model step inside a fixed, code-driven workflow (schedule → draft → pre-send check → approval → send → record). Code, not the model, picks the next action. No multi-agent design.
- **Approval:** a pending send stores its exact content and a hash of it. Only an approved, hash-matching send can go out; an edit creates a new hash; pending sends expire. Enforced in code.
- **Side effects:** every send has an idempotency key with started and completed markers. A crash is reconciled, never re-sent (the opposite of Harvey's double-send bug).
- **Durability:** Postgres only, with run state, an append-only event log and a `FOR UPDATE SKIP LOCKED` job table. `apalis` 1.0 is the fallback if hand-rolling the queue gets costly.
- **Model layer:** a thin in-house client over `reqwest` + `serde`, behind our own trait. Use API keys, never consumer subscriptions (Anthropic's terms; [harvey-market.md](harvey-market.md)). Anthropic has no official Rust SDK; avoid the crates that impersonate one.
- **Stack candidates:** tokio, axum 0.8, sqlx, tracing + OpenTelemetry, one crate with modules; server-rendered pages first, React later.
- **P1 channels:** email from our own domain for upload links (e.g. Amazon SES), the bookkeeper's own Viber by copy-and-open, and SMS (Semaphore) only as a link-free nudge.
- **Licences:** n8n (Sustainable Use), Dify (modified Apache), Inngest (SSPL) and the Restate server (BSL) are unsuitable to embed. Harvey is MIT; no code is copied.
