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
| 7. Rust architecture (engine vs product split) | **in progress** (`docs/architecture.md`) |
| 8. Project skills & rules | blocked on 7 |
| 9. Implementation readiness gate | blocked on 8 |
| 10+. Build engine + P1 (Rust backend; server-rendered pages first, React later) | blocked on 9 |

**No application code exists or gets written until the Phase 9 gate passes.** Research, docs and throwaway prototypes that answer a specific question are in scope; prototypes live in `prototypes/<question>/` and are deleted once their answer is recorded.

## Owner decisions

Every decision needing the owner's judgement goes through the `grilling` skill: numbered rounds, a recommended answer per question, worded so "yes" accepts it. Look facts up yourself, using sub-agents for research; put only decisions to the owner. A routine "go ahead with the agreed step?" is one line.

Record every approved decision, dated, in `docs/decisions.md`, and update the summary below. Record resolved domain terms in `GLOSSARY.md` and hard-to-reverse, non-obvious trade-offs in `docs/adr/`, using the `domain-modeling` skill. Both are created lazily.

**In force** (full log and reasoning: `docs/decisions.md`):

- **First product:** Kompleto (P1) for PH bookkeepers. Payroll chasing comes later as an add-on; S1 (seller tax helper) is the fallback.
- **P1 first version:** scope, channels, files, AI and web stack are decided round by round in `docs/product/p1-spec.md`. That file is the single source for spec decisions.
- **Engine:** our own design, borrowing Harvey's ideas but copying none of its code. Built inside P1, with engine and product modules kept separate.
- **Build first:** build the whole product, core loop first; the owner chooses when to demo.
- **Go-to-market:** free written outreach (≤ 15–20 personalised messages a week), then 3–5 free 4-week pilots, then ₱1,500/month via GCash.
- **Budget:** ~₱600/month until a business pays. No customer data goes to free LLM tiers that train on inputs.
- **Owner constraints:** no network, no interviews or sales calls, no paid ads. Written, asynchronous contact is fine.

## Evidence

Research claims carry a source (URL, publisher, date, accessed date), an evidence type and a confidence. Label vendor claims as vendor claims. When evidence is missing, write "not found"; a gap stays a gap. Keep facts, hypotheses, recommendations and owner-approved decisions visibly distinct.

## Rust (when implementation begins)

- Read the `rust-best-practices` skill (`.claude/skills/rust-best-practices`) before writing or reviewing Rust.
- Start as one crate with modules. Add a workspace only when a concrete requirement demands it.
- Agent safety lives in code, not prompts. Approval gates, tool permissions, idempotent external sends and bounded loops are enforced by the application. Model output and inbound content are untrusted data.
- Report test results only from commands actually run.

## Scope

Work only inside the approved phase and scope. When something outside it looks worth doing, raise it as a proposal for the owner.
