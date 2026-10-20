# Opportunity landscape

Status: **synthesis, 2026-10-18; superseded on the ranking.** Later sweeps changed the lead from B to **P1** (bookkeeper document and deadline chaser): see [opportunity-scorecard.md](opportunity-scorecard.md) §8–10. The five facts and the inventory below still hold for the candidates they cover.

Original status: **synthesis, 2026-10-18.** This file is the inventory and the reasoning for what was shortlisted or excluded. It cites the detailed research files; follow those for primary sources:

| Short ref | File |
|---|---|
| **BIZ** | [opportunities-business.md](opportunities-business.md) |
| **IND** | [opportunities-individual-and-infra.md](opportunities-individual-and-infra.md) |
| **PH** | [market-philippines.md](market-philippines.md) |
| **COMP** | [competitor-analysis.md](competitor-analysis.md) |
| **ARCH** | [agent-architecture-comparison.md](agent-architecture-comparison.md) |
| **HARVEY** | [harvey-teardown.md](harvey-teardown.md) |
| **INT** | [integration-constraints.md](integration-constraints.md) |

`BIZ [4]` means "source 4 in opportunities-business.md".

## The five facts that shape everything

1. **The channel owner now gives away the obvious product.** Meta Business Agent launched on Messenger in the Philippines on 2025-10-20 and globally across WhatsApp, Messenger and Instagram on 2026-06-03. It answers FAQs, books appointments, qualifies leads and hands off to a human, and it is free up to an unpublished allowance (COMP §3.1). A generic "AI answers your Facebook Page inbox" product is commoditised. *Confidence: high.*
2. **What Meta's agent does not do is follow up.** It is inbound-only, it has no documented draft-and-approve mode, and it is Meta-channel-only (COMP §3.1, §5). Meta also removed most Messenger outbound message tags on 2026-04-27, leaving the 7-day HUMAN_AGENT tag (COMP §4). So proactive follow-up has become harder for bots and more valuable to owners. *Confidence: medium.*
3. **Follow-through is a behaviour gap, not a tooling gap.** Fast lead response strongly predicts qualification (BIZ [1][2], direction reliable, multipliers shaky). In Invoca's 2026 home-services data, 55% of businesses never ask the caller to book, even among firms already paying for software (BIZ [4], vendor data). *Confidence: medium.*
4. **Philippine price ceilings are about ¼ of US prices.** One vendor charges PH clinics ₱1,990/mo (~US$34) for a plan sold at US$149 abroad (PH §4, §10). The working ceiling is ₱1,500–4,000/mo for small clinics, agencies and travel operators. The human alternative is a chat VA at ₱25–35k/mo, but most micro firms have no such person; the owner replies (PH §4). *Confidence: medium.*
5. **Micro-businesses adopt AI slowly; they buy outcomes.** US Census BTOS shows no significant AI-use change for firms with fewer than 20 employees (BIZ). PH firm-level AI-tool use is about 15% per PIDS (PH §2). Sell "no lead or quote goes cold", not "AI agent". *Confidence: high.*

## Inventory

Every candidate investigated, with its fate. Scores are in [opportunity-scorecard.md](opportunity-scorecard.md).

### Business-facing

| ID | Opportunity | Fate | One-line reason |
|---|---|---|---|
| **B** | Owner-approved lead & quote follow-up for PH service SMEs | **Shortlist #1** | Evidenced gap left by Meta (no follow-up), low liability with draft→approve, strong foundation fit |
| **B2** | Same product for one US/AU service vertical | **Shortlist #2** | Same workflow at roughly 4–5× revenue per customer abroad (PH §10); crowded by GoHighLevel and Jobber bundles (BIZ [25][28]); harder to reach from PH |
| **C** | AR collections / payment-reminder agent on Xero/QBO | **Shortlist #3** | Best-evidenced pain and price points (56% of US small businesses owed money; Chaser from $259, Paidnice $69: BIZ [38][39]); international-only and distribution-heavy for a PH-based builder |
| **A** | Clinic/dental inquiry → booking + reminders (PH) | **Shortlist #4, folded into B as a vertical** | Highest-evidence segment (PH §8A, HeyDenta pricing), but Meta books appointments and has calendar management on its roadmap; health data raises compliance |
| G | Travel/tour inquiry → quote → follow-up (PH) | Deferred as a standalone; **candidate vertical for B** | Strong pain hypothesis but almost no evidence (PH §8B); seasonal |
| D | Generic Messenger FAQ/inquiry bot (PH) | **Rejected** | Meta gives it away (COMP §3.1); ManyChat/Chatfuel/Botcake are cheap |
| E | Voice AI receptionist | **Rejected** | Most crowded category; incumbents bundle it; caller hang-ups; voice is rented infrastructure with no Rust advantage; voice-first is not how PH SMEs take inquiries (BIZ §3, PH §3) |
| F | SME document intake & extraction | Deferred → **add-on** | Supply gap real but SME demand unverified; vendors priced for mid-market (COMP §3.13); fits later as "read the payment screenshot / ID photo" inside B |
| H | Workflow glue between SME apps | **Rejected** | Zapier/Make/n8n are cheap; n8n self-host is free (BIZ [43]) |
| I | SME research/reporting assistant | **Rejected** | No evidence SMEs pay; general assistants cover it |
| P | Outbound prospecting / cold email (Harvey's domain) | **Rejected for first product** | Crowded AI-SDR market; deliverability and anti-spam/consent risk; not researched deeply because nothing pointed to an under-served gap |
| — | Quotation/proposal preparation (standalone) | Merged into B | Slow quoting plausible but undocumented (BIZ §4); the follow-up after a quote is the evidenced part |

### Individuals

| ID | Opportunity | Fate | Reason |
|---|---|---|---|
| J | General personal agent | **Rejected** | Gemini Spark, ChatGPT agent/Pulse, Claude Cowork, free Comet; AI-app churn ~30% above norm (IND [3][8][11]) |
| K | Prosumer email triage/drafting | Deferred | Proven WTP (Fyxer $17M ARR, IND [21]) but crowded and paid-acquisition-driven; CASA cost is affordable (~US$540–720/yr, IND [6]) so it stays a later option |
| L | OFW household admin | Deferred → **validation-only** | 2.19M OFWs, but no evidence of admin pain or WTP (IND [35]); a 4-week concierge test is cheap if the owner wants a consumer bet |
| M | Filipino freelancer admin | Deferred | Price-sensitive, tax-advice liability, unverified |

### Developer & agent infrastructure

| ID | Opportunity | Fate | Reason |
|---|---|---|---|
| N1 | Durable agent execution | **Rejected as a product; build internally** | Temporal, Restate (Rust), Cloudflare, Vercel, Claude Managed Agents (IND [40]–[47]) |
| N2 | Approval/permission layer | **Rejected as a product; build internally** | HumanLayer pivoted away; Auth0 and MCP absorb it (IND [48][52][54]) |
| N3 | Integration platform | **Rejected** | Composio free tier, Arcade $60M, Workday bought Pipedream; per-provider security reviews (IND [55]–[58]) |
| N4 | Observability | **Rejected** | Consolidating: Langfuse → ClickHouse, Helicone → Mintlify (IND [59][61]) |
| — | Memory/context | **Rejected** | Model labs ship memory primitives (IND [51]) |

**Bias check.** The infra options score best *only* under the deliberately biased Builder-first profile, which is the warning pattern defined in [scoring-method.md](scoring-method.md). The durable-run, approval, audit and provider-layer work still gets built, as the internal core of whichever product wins. That is how the "reusable foundation" ambition is served without betting the business on infra.

## The recommended shape (detail in the scorecard)

**An owner-approved follow-through agent for Philippine service SMEs.** It watches the business's inquiries and sent quotes. It notices leads and quotes going cold and drafts the right nudge in Taglish/English from the business's own price list. The owner approves with one tap, and it sends through whichever channel is allowed: Messenger inside the window or via a human, plus email, SMS or Viber. It records outcomes.

What makes it more than "an LLM calling a tool":
- **Durable scheduled work:** follow-ups days later.
- **A real approval gate** enforced in code.
- **Idempotent external sends:** Harvey's double-send bug is the counter-example (HARVEY).
- **Channel policy awareness:** Messenger windows and tags.
- **Measurable ROI:** replies and bookings recovered.

The **vertical** (clinics vs travel/tours vs home services vs a US/AU vertical) is an owner decision, presented in the decision round.
