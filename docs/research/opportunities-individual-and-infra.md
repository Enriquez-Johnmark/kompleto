# Opportunities: individuals (consumer/prosumer) and developer/agent infrastructure

Researched 2026-10-18 (all sources accessed 2026-10-18). Business SME automation is covered in a separate document.

**Scoring convention (aligned with `scoring-method.md`):** every score is 1–5 where **5 is always favourable to us**. "Competitive room" therefore means 5 = little competition, and "Risk" is inverted: 5 = low risk. **E** = evidence-backed (cited), **J** = judgement (capped at 4). Bracketed numbers like [3] point to the numbered Sources list at the end.

**Evidence labels used in the text:** *vendor claim* (company-reported, not independently checked), *official stats*, *primary research*, *product docs*, *customer testimony*, *independent analysis*, *hypothesis* (my inference; untested).

---

## 1. Summary table

| # | Opportunity | Severity | Frequency | WTP evidence | Competitive room | Distribution ease | Risk (5 = low) |
|---|---|---|---|---|---|---|---|
| I-1 | General personal agent (email + calendar + tasks, consumer) | 3 J: real annoyance, rarely money-losing | 5 E: daily inbox use; agent users 63% daily [1] | 3 E: 55% of US AI users pay, but mostly for general assistants [1] | 1 E: Gemini Spark, ChatGPT agent/Pulse, Claude Cowork, Comet free [8][11][14][15] | 2 J: paid acquisition against free incumbents | 2 E: CASA + restricted scopes, AI-app churn 30% above norm [3][4][6] |
| I-2 | Prosumer email triage/drafting (professionals) | 4 E: Fyxer grew $1M→$17M ARR on this [21] | 5 E: daily | 4 E: $37–49/mo Fyxer, $30–120 Shortwave [20][25] | 2 E: Fyxer, Superhuman, Shortwave, Gemini in Gmail [10][19] | 2 J: crowded paid channels | 3 E: CASA ~US$540–720/yr is affordable; mis-sorting complaints [6][30] |
| I-3 | Personal paperwork assistant (bills, receipts, forms), generic/US | 3 J | 2 J: monthly | 2 E: only usage signals (23% of bill payers use AI) [1] | 2 E: Gemini Spark advertises statement parsing [8] | 2 J | 2 J: financial/identity data |
| I-4 | OFW household admin (remittance, bills, SSS/Pag-IBIG/PhilHealth) | 3 J: plausible, no complaint data on admin itself | 3 J: monthly cycles | 2 E: cost complaints are about contribution *rates*, not admin [35] | 3 J: GCash covers payment, nothing covers coordination | 4 J: dense OFW online communities; builder is Filipino | 2 J: payments + PH Data Privacy Act; gov portals lack APIs |
| I-5 | Filipino freelancer admin (invoices, BIR deadlines, client follow-up) | 3 E: BIR enforcement rising (secondary) [39] | 3 J: weekly–quarterly | 2 J: price-sensitive segment; not verified | 3 J: local tax apps exist (not verified here) | 4 J: Facebook groups, builder access | 3 J: tax advice liability |
| D-1 | Durable agent execution / resumability | 4 E: Temporal >$250M ARR claimed [41] | 4 J | 4 E: Temporal 4,300+ paying customers (secondary) [41] | 1 E: Temporal, Restate (Rust), Trigger.dev, Cloudflare, Vercel, Claude Managed Agents [40][43][45][46][47] | 2 J: OSS stars race, Rust not the buyer's language | 2 J: capital-intensive, platform absorption |
| D-2 | Agent memory / context / provenance | 3 J | 4 J | 2 E: Mem0 raised $24M but revenue not disclosed [50] | 2 E: Mem0 67k stars, Graphiti 32k, Letta 25k, built-in memory tools [40][51] | 2 J | 2 E: labs ship memory primitives [51] |
| D-3 | Human approval / permission layer | 4 J: consequential actions need it | 4 J | 2 E: HumanLayer moved away from approvals API [54] | 2 E: Auth0 includes async auth in plans; MCP elicitation [48][52] | 3 J: niche OSS libs can spread | 3 J |
| D-4 | Integration infrastructure for agents | 4 E: investors call integrations the bottleneck [67] | 5 J | 4 E: Nango "several million ARR" (vendor claim) [57] | 1 E: Composio free tier 100k calls, Arcade $60M, Workday bought Pipedream [55][56][58] | 1 J: connector catalog war | 1 E: per-provider reviews (CASA per Google app) [6] |
| D-5 | Agent observability | 3 J | 5 J | 3 E: LangSmith ARR $12–16M (sources to TechCrunch) [65] | 1 E: Langfuse→ClickHouse, Helicone→Mintlify, Braintrust $800M [59][61][62] | 2 J | 2 E: consolidation signals weak standalone economics [59][61] |

**Read:** no row scores well on both *competitive room* and *WTP evidence*. The two rows with the best WTP evidence (I-2, D-1/D-4) are the most crowded. The two rows with some room (I-4, I-5) have the weakest WTP evidence and need customer interviews before any build.

---

## 2. Cross-cutting evidence

### 2.1 Consumers do pay for AI, but mostly to general assistants

- **Menlo Ventures 2026 survey** (n = 5,067 US adults, Morning Consult, July 2026; *primary research*, US): 55% of AI users pay for at least one AI product; typical payer spends $20–49/month; the top 14% of payers (≥$100/mo) account for 60% of spend; 41% have tried an agent, 24% use one regularly; agent users are 92% payers and 63% daily users; 32% have let AI act without final sign-off [1]. Confidence H for survey figures, M for the $40B spend estimate (a Menlo model).
- The same report shows **where the money goes**: ChatGPT reaches 60% of AI users, Gemini 58%, Claude 20%; users average 3.0 general assistants. Among agent users the leading tools are Codex (35%), Claude Code/Cowork (32%), Perplexity (24%) [1]. It gives **no** category spend for email or life admin. Usage hints only: 23% of bill payers use AI for bills [1].
- Menlo's 2025 "3% pay" figure was a modelled global ratio, not like-for-like with 2026's US survey figure [1][2]. Do not read the jump from 3% to 55% as a trend.
- **Retention is the problem.** RevenueCat (Aug 2026; 3,519 AI apps, 50M+ paid subscriptions; *primary data from a vendor dataset*, global, mobile only): AI apps churn about 30% more than other subscription apps. 12-month retention on monthly plans is 6.1% for AI apps against 9.5% for non-AI apps, and 21.1% against 30.7% on annual plans. Most of the loss happens at the first renewal [3]. Confidence H for mobile apps; web or desktop products may differ.

**Implication (J):** a consumer agent has to replace a $20/mo general assistant the user already pays for, or stack on top of it. Stacking is what the top 14% of payers do, and that group is mostly professionals.

### 2.2 Platform incumbents are shipping the same thing, often bundled

| Incumbent | Feature (date) | Access / price | Source |
|---|---|---|---|
| Google | **Gemini Spark** personal agent: background, cloud-run, recurring tasks (e.g., "parsing monthly credit card statements"), Gmail/Docs integration, confirms before sending email or buying (I/O, May 2026) | Beta, US Google AI Ultra (launched at $250/mo in 2025) | [8][9] |
| Google | Gemini in Gmail: to-do lists, reply drafting (Jan 2026); critics say "isn't agentic" | Gmail | [10] |
| Google | Google AI Plus (Gemini in Gmail/Docs + storage) | PH: ₱139/mo intro, ₱275 after (secondary, may have changed) | [37] |
| OpenAI | ChatGPT agent (Jul 2025): Plus ~40 runs/mo, Pro ~400 (third-party figures; official page blocked to fetch) | Plus $20, Pro $200 | [12] |
| OpenAI | Pulse: proactive daily briefings with optional Gmail/Calendar (Sep 2025, Pro preview) | Pro | [11] |
| OpenAI | Atlas browser with agent mode; pauses on sensitive sites (Oct 2025) | Plus/Pro/Business | [13] |
| OpenAI | ChatGPT Go in PH | ₱300/mo | [36] |
| Anthropic | Claude in Chrome (all paid plans), Cowork with Gmail connector and scheduled tasks | Pro and up | [15][16] |
| Perplexity | Comet browser free to all (Oct 2025) | Free; Max $200 for extras | [14] |
| Microsoft | Copilot bundled into M365 Personal; price rose $6.99→$9.99/mo (Jan 2025) | Bundled | [17] |
| Apple | Gemini-based Siri with personal context and App Intents slipped past iOS 26.4; unveiled at WWDC (Jun 2026) for iOS 27 | Bundled with device | [18] |
| Superhuman (ex-Grammarly) | Superhuman Go, 100 "permission-aware agents", free until Feb 2026 | Bundled | [19] |

**Implication (E/J):** the pitch "an agent that reads my email, remembers context, runs recurring tasks and asks before acting" is now the *announced roadmap of every major platform*. Apple's delays [18] and critics of Gemini in Gmail [10] show that execution quality is still weak. That opens a window, but a closing one. The real moat is reach into one specific workflow that the incumbents don't serve.

### 2.3 Gmail permissions: what CASA actually costs (verified)

- **Google product docs:** `gmail.readonly`, `gmail.modify`, `gmail.compose`, `gmail.metadata` and full `mail.google.com` are **restricted**; `gmail.send` is only **sensitive**. "If you store restricted scope data on servers (or transmit), then you must go through a security assessment" [4]. Assessment uses the App Defense Alliance CASA framework, assigns assurance level AL1/AL2 based on user count and scopes, and **must be revalidated annually** [5]. Confidence H.
- **Cost (vendor-reported, not Google):** CASA Tier 2 via TAC Security at **US$540–720**, Netsentries about $700, Prescient $3,000+ (prices as of Aug 2025); verification plus CASA takes 5–20 days if prepared, against Google's published 4–6 weeks [6]. A MailMate developer reported discounted $540–1,800 and undiscounted $1,800–6,300 (Apr 2025) [7]. Older "$15k–75k" figures circulating online relate to the legacy track [6]. Confidence M.
- **Design implication (hypothesis, needs written confirmation from Google):** since the assessment trigger is server storage or transmission of restricted data, a **local-first Rust desktop agent** that processes Gmail on-device may need restricted-scope verification but **not** CASA. A Google community thread raises the same architecture question without a definitive answer [6]. This is one place where a Rust/local architecture could become a real advantage.

---

## 3. Category 1: individuals

### I-1 General personal agent (consumer email, calendar, tasks)

- **Who / workflow:** busy individuals juggling inbox, calendar and to-dos; today they use Gmail plus a to-do app plus ChatGPT copy-paste. Frequency is daily (E [1]).
- **Existing tools and prices:** Martin (YC S23, $2M seed, $21–49/mo per a competitor's page, "30,000+ users" per its own job posts: *vendor claim*, L) [27]; Saner.ai ($8–20/mo, reportedly bootstrapped with about $880K 2025 revenue per Latka: unverified, L) [28]; Lindy (credits, from $49.99/mo, now team-oriented) [24]; Motion **pivoted to businesses** ("AI Employees", $60M at $550M, "8-figure ARR": *vendor claim*) [26].
- **Evidenced gaps:** Trustpilot reviews of Lindy cite hallucinations, "work I did not ask for", and no cancel button (*customer testimony*, L/M) [29]. Critics say Gemini in Gmail "isn't agentic" and its summaries are inaccurate [10].
- **Buyer / motion:** the individual, self-serve, card-on-file.
- **Permissions / sensitivity:** restricted Gmail scopes, plus calendar and contacts; very high privacy sensitivity.
- **Small-team feasibility (J):** an MVP is feasible, but reliability across the open-ended task space is not.
- **Retention driver:** habit plus accumulated memory. Mobile data shows AI apps lose most subscribers at the first renewal [3].
- **Disproving evidence:** Motion and Lindy, the best-funded "personal" agents, both moved to business buyers [24][26]. Gemini Spark targets exactly this job [8].
- **Verdict:** do not start here (E).

### I-2 Prosumer email triage and drafting

- **Who:** professionals, executives and solo operators with high email volume. This is *individual* use but *work-funded*: Menlo finds employer-paid users average $95/mo against $55 for self-paid [1].
- **Traction evidence:** Fyxer: $10M Series A (Mar 2025), $30M Series B (Sep 2025); ARR reported as $1M→$17M across 180k users at the Series B (Sifted, *vendor-reported*) [21]; the company press page later says $1M→$30M in 2025 (*vendor claim*, L) [22]; Sacra estimates $9M (L) [23]. Prices: Fyxer Free, Pro $37/mo annual ($49 monthly), Team $64 [20]; Shortwave Business $30, Premier $45, Max $120 per seat [25].
- **Gaps (customer testimony):** Fyxer reviewers say important emails get filed in the wrong folder and drafts keep coming until the subscription ends [30].
- **Integrations:** Gmail restricted scopes (CASA about $540–720/yr [6]) and Microsoft Graph.
- **Distribution:** crowded; the channel mix of the leaders is not disclosed, but entrants compete against funded marketing budgets (J).
- **Disproving evidence:** Superhuman Go is bundled [19], Gemini in Gmail drafts replies [10], and Fyxer is far ahead with $43M raised [21][22].
- **Verdict:** WTP is proven, but the space is red-ocean. It overlaps the SME doc's territory and only makes sense as a narrow vertical (e.g., a specific profession's inbox) (J).

### I-3 Personal paperwork assistant (generic / US)

- **Workflow:** bills, receipts, statements, government forms; monthly frequency (J). Menlo: 23% of bill payers already use AI for bills, and 23% of people navigating healthcare use AI for it [1]. That is a usage signal, not WTP.
- **Competition:** Gemini Spark's launch examples literally include "parsing monthly credit card statements" [8]. General assistants handle one-off form questions for free (J).
- **Sensitivity:** financial and identity data; errors can cause late fees or penalties.
- **Verdict:** a thin wedge in the generic market; incumbents get there first (J).

### I-4 OFW household admin (Philippine niche)

- **Who / scale (official stats):** 2.19M OFWs in 2024 (PSA Survey on Overseas Filipinos, released Dec 2025; 57.2% women; 74.5% in Asia, Saudi Arabia 21.9%) [31]. Cash remittances hit a record **US$35.63B in 2025** (BSP; 7.3% of GDP) [32]. Confidence H.
- **Current workflow (partly evidenced):** OFWs remit to family; 94–97% of recipient households spend remittances primarily on food and household needs (BSP Consumer Expectations Survey via Inquirer, Q3 2025) [33]. The SSS rate rose to 15% in Jan 2025 [35]. OFWs abroad self-pay it with no employer in the loop, and lapses stall pension and loan eligibility (secondary guide) [69]. GCash Overseas already lets OFWs pay household bills and, since Apr 2026, OWWA membership from abroad [34].
- **Evidenced gaps:** *not found* for the coordination problem itself (has the family paid Meralco? is SSS current? where's the receipt?). Migrante's documented complaints are about contribution **rates and mandatory membership**, not admin difficulty [35]. **Evidence of unmet need for an agent: not found** (hypothesis only).
- **Buyer:** the OFW (payer) for the family; household WTP not verified. ChatGPT Go at ₱300/mo [36] and Google AI Plus at about ₱139–275/mo [37] anchor what AI is expected to cost in PH.
- **Integrations / permissions:** PH government portals (My.SSS, Virtual Pag-IBIG, PhilHealth) and utilities have no public agent APIs (J). Any payment execution touches BSP-regulated rails, so the product should stay *read, remind, reconcile*, never *move money*. PH Data Privacy Act obligations apply (J).
- **Small-team feasibility (J):** good for a Messenger/WhatsApp-style reminder and receipt-ledger agent with photo OCR, shared between the OFW and the family. No Gmail CASA is needed if it works from forwarded receipts and photos.
- **First users (J):** OFW Facebook groups, Filipino communities in the Gulf, Singapore and Hong Kong; the builder's own network.
- **Retention driver (J):** a monthly bill cycle and family accountability (a shared ledger).
- **Disproving evidence:** GCash (super-app incumbent) can add family views. Low-income households are price-sensitive. No complaint corpus was found.
- **Verdict:** the only individual niche with distribution advantage and little direct competition, but **WTP is unproven**. Run 15–20 OFW interviews and a concierge test before writing platform code (J).

### I-5 Filipino freelancer admin

- **Scale:** about 1.5M registered online freelancers (GCash/Payoneer figure via PIDS; year and method unclear; L) [38].
- **Pain (secondary):** BIR reportedly stepping up monitoring of platform income, with penalties for missing official receipts (*independent analysis/vendor blog*, L) [39].
- **Workflow:** invoices, foreign payments, quarterly BIR filings, client follow-ups. Local tax-filing apps exist (not verified in this pass).
- **Buyer:** the freelancer, who is income-earning, so WTP is higher than for I-4 (J). This overlaps with SME micro-business.
- **Verdict:** worth folding into the I-4/SME interviews as an adjacent segment. Tax-advice liability means any product should remind, organise and prepare, not file (J).

---

## 4. Category 2: developer and agent infrastructure

### Adoption snapshot (GitHub API, 2026-10-18; *primary data*, H) [40]

| Repo | Stars | Lang | Note |
|---|---|---|---|
| langchain-ai/langgraph | 43,413 | Python | framework incumbent |
| langfuse/langfuse | 35,966 | TS | now ClickHouse-owned |
| getzep/graphiti | 31,897 | Python | memory graph |
| ComposioHQ/composio | 30,611 | TS | integrations |
| openai/openai-agents-python | 30,101 | Python | lab SDK |
| letta-ai/letta | 25,173 | — | memory/agents |
| temporalio/temporal | 23,778 | Go | durable execution |
| trigger.dev | 16,563 | TS | durable jobs |
| NangoHQ/nango | 12,755 | TS | integrations |
| Arize-ai/phoenix | 11,887 | Python | observability |
| humanlayer/humanlayer | 11,703 | TS | now coding-agent tooling |
| **0xPlaygrounds/rig** | **8,903** | **Rust** | largest Rust agent framework found |
| Helicone/helicone | 6,225 | TS | maintenance mode |
| inngest/inngest | 5,954 | Go | durable |
| permitio/opal | 5,524 | Python | authz |
| **restatedev/restate** | **4,567** | **Rust** | durable execution, already in Rust |
| **modelcontextprotocol/rust-sdk** | **4,025** | **Rust** | official MCP Rust SDK |
| **golemcloud/golem** | **1,513** | **Rust** | durable WASM agents |
| mem0ai/mem0 | 67,355 | Python | memory |

**Implication (E):** the Rust agent ecosystem is roughly one-fifth the size of the Python/TS ecosystem by stars, and the two Rust durable-execution projects (Restate, Golem) already exist. Infra buyers mostly write Python or TypeScript. Rust is a *performance and packaging* advantage (single binary, local-first, low memory), not a distribution channel.

### Model labs and standards absorbing infra (E)

- **MCP** moved to the Linux Foundation's Agentic AI Foundation in Dec 2025, with AWS, Anthropic, Google, Microsoft and OpenAI as platinum members [49]. The 2025-11-25 spec added URL-mode elicitation, incremental scope consent, OAuth Client ID Metadata Documents, and **experimental durable "tasks"** [48].
- **Anthropic:** memory tool (client-side `/memories`, all Claude 4+ models), context editing, compaction [51]; **Claude Managed Agents** (public beta Apr 2026): hosted runtime with sandboxing, checkpointing and credential management, reportedly $0.08/runtime-hour (secondary), plus self-hosted sandboxes and MCP tunnels (May 2026) [47].
- **Cloudflare Workflows + Agents SDK** (durable steps, human-approval flows) [46]; **Vercel Workflow** GA, claiming 100M+ runs and 1,500+ customers (*vendor claim*) [45].
- **Auth0 for AI Agents** GA (Nov 2025): Token Vault, async authorization via CIBA, FGA for RAG, **included in existing plans** [52].

### D-1 Durable execution / resumability

- **Players:** Temporal raised $300M at about $5B (Feb 2026) and then a reported $550M Series E at $12.55B (Lightspeed; *secondary/aggregator*, M). It claims >$250M annualised revenue, 200% YoY growth and 4,300+ paying customers (*vendor claim via secondary*, M) [41][42]. Trigger.dev raised a $16M Series A (Dec 2025) [43]. Restate (Rust) raised a $7M seed led by Redpoint, with conflicting dates [44]. Inngest and DBOS also compete, alongside the platform offerings above.
- **Gap:** durable *agent* loops (LLM call + tool + human wait + resume) are being productised by Temporal, Cloudflare, Vercel and Anthropic simultaneously (E).
- **Monetisation:** strong for the leader, weak for a newcomer, because buyers choose on trust and longevity (J).
- **Verdict:** the money is real but already taken. Don't sell this; build it *inside* the product (J).

### D-2 Memory / context / provenance

- **Players:** Mem0 ($24M seed + A, Oct 2025; API calls 35M→186M per quarter Q1→Q3 2025, *vendor claim*) [50]; Letta and Zep/Graphiti (funding not verified in this pass).
- **Absorption:** Anthropic's memory tool makes "memory" a client-side file protocol [51]; ChatGPT has memory and Pulse [11].
- **Gap (hypothesis):** *provenance*, meaning which source and which approval produced a remembered fact or action, is not a headline feature anywhere I found. It is an under-served sub-feature, not yet evidence of a market.
- **Verdict:** stars are plentiful but revenue evidence is not; low priority as a standalone product (J).

### D-3 Human approval and permissions for consequential actions

- **Evidence:** HumanLayer, the reference "approval API" startup (YC F24), shifted towards context engineering and coding-agent tooling (CodeLayer) during 2025. A competitor claims the approvals SDK is deprecated (L) [54]. Permit.io ($8M Series A, 2024) offers Approval Flows and an Access Request MCP [53]. Arcade raised a $60M Series A (Jun 2026) positioned as the "secure action layer" [55]. Auth0 bundles async approval [52]. MCP elicitation standardises the prompt to the user [48]. Gemini Spark and Atlas ship confirmations natively [8][13].
- **Verdict:** this is essential as a **feature of your own product** (and a good Rust module), but weak as a standalone business (E: the specialist moved away from it, and identity incumbents bundle it).

### D-4 Integration infrastructure

- **Players / prices:** Composio ($25M Series A led by Lightspeed; Hobby free with **100k tool calls/mo**, Pro $29 plus $0.0003/call) [56]; Arcade (Free 2k calls, Team $25 plus $0.01/call and $0.10/auth event) [55][68]; Nango ($7.5M seed; "several million ARR", cash-flow positive: *vendor claim*; Apache-2.0 core per third parties) [57]; Workday is acquiring Pipedream (3,000+ connectors, announced Nov 2025) [58]; Merge launched Agent Handler [66]; Paragon offers ActionKit.
- **Why it's hard for a solo developer (E):** price compression to near zero at the low end [56], plus a recurring Google CASA per app [6] and per-provider review processes, multiplied by the size of the catalog.
- **Verdict:** worst fit. Consume Composio, Nango or MCP servers rather than compete with them (J).

### D-5 Agent observability

- **Evidence of consolidation:** ClickHouse acquired Langfuse (Jan 2026; "2,000+ paying customers", per the counsel's release) [59]. Mintlify acquired Helicone (Mar 2026), and Helicone moved to **maintenance mode** [61]. Braintrust raised an $80M Series B at $800M (Feb 2026) [62]. Arize raised a $70M Series C (Feb 2025) [63]. LangChain raised $125M at $1.25B (Oct 2025) [64], with LangSmith ARR reported at $12–16M (anonymous sources to TechCrunch; M) [65].
- **Pricing floor:** Langfuse Hobby is free, Core $29, Pro $199, Enterprise $2,499, and it is free to self-host [60].
- **Verdict:** the two leading OSS tools were absorbed into larger platforms within 3 months, which signals that standalone economics are hard even with traction (J from E).

---

## 5. Should a small team start here?

### Category 1 (individuals): **No as a general product; maybe as a validated niche.**

- General personal agents (I-1) and generic paperwork (I-3) sit directly on the published roadmaps of Google, OpenAI, Anthropic, Apple and Microsoft, which bundle at $0–20/mo [8][11][15][17]. AI apps also churn about 30% more than other subscription apps [3].
- The proven-WTP slice (I-2, professionals' email) is really a work purchase and is already won by funded players [20][21][25].
- **Conditional yes:** I-4/I-5 (OFW households, Filipino freelancers) is the only area with a distribution advantage for this builder and no direct agent competitor. Treat it as a **4-week validation experiment** before committing: 15–20 interviews, a concierge "receipt and deadline ledger" over Messenger, and a measured willingness to pay at or below ChatGPT Go's ₱300 anchor [36]. Kill it if fewer than about 5 of 20 interviewees pre-pay.
- **Architecture note:** local-first processing may avoid CASA [4][5], which matters if the niche later needs Gmail.

### Category 2 (developer infra): **No as the first business.**

- Every sub-area has a well-funded leader plus platform absorption: Temporal at $12.55B, Composio's free tier, Auth0's bundling, MCP's standards, and Claude Managed Agents [41][47][48][52][56].
- The observability shake-out shows that even OSS projects with traction get acquired rather than standing alone [59][61].
- The Rust audience is small [40], and Restate and Golem already occupy Rust durable execution.
- **What to do instead:** build durable steps, approvals, audit and provenance as the **internal core** of whichever end-user product wins, and open-source a small, well-scoped Rust crate later (e.g., approval and audit primitives over MCP) for credibility and hiring. Don't plan on revenue from it.
- **Caution against builder bias:** the infra rows score well only on "fun to build" (Foundation fit, C16), which is exactly the warning pattern `scoring-method.md` defines under Builder-first.

---

## 6. Sources

Format: [n] Title, Publisher, publication date, URL. Accessed 2026-10-18. Type; geography; confidence; limitations.

1. "2026: The State of Consumer AI", Menlo Ventures, 2026-09-16. https://menlovc.com/perspective/2026-the-state-of-consumer-ai/ Primary research (Morning Consult survey n=5,067, Jul 2026); US (spend is global); H for survey, M for spend; no category spend or churn data.
2. "Only 3% of AI users pay for tools", Semafor, 2025-06-27. https://www.semafor.com/article/06/27/2025/only-3-of-ai-users-pay-for-tools-leaving-huge-untapped-revenue-for-companies-report Independent analysis of Menlo 2025; global; M; the 3% is a modelled ratio.
3. "AI app retention study", RevenueCat, 2026-08-18. https://www.revenuecat.com/blog/growth/ai-app-retention-study Primary data (vendor dataset, 3,519 apps, 50M+ subscriptions); global; H; mobile apps only.
4. "Choose Gmail API scopes", Google for Developers, undated (live). https://developers.google.com/workspace/gmail/api/auth/scopes Product docs; global; H.
5. "Security assessment" (restricted scope verification), Google Cloud Help, undated (live). https://support.google.com/cloud/answer/13465431 Product docs; global; H; no cost stated.
6. "Google security review", Nango docs, prices as of Aug 2025. https://nango.dev/docs/api-integrations/google-shared/google-security-review Vendor docs; global; M; lab prices change.
7. "MailMate passed the CASA tier 2 security assessment", MailMate mailing list, Apr 2025. https://lists.freron.com/mailmate/2025-April/018283.html Customer testimony; global; M.
8. Gemini Spark coverage, Thurrott.com, 2026-05-30. https://www.thurrott.com/?p=336765 Independent news; US; M; beta features may change.
9. "Everything Google announced at I/O", MacRumors, 2025-05-20. https://macrumors.com/2025/05/20/google-i-o-announcements Independent news; US; M (AI Ultra $250/mo launch price).
10. "Gmail gets Gemini, but falls short of true agentic AI", Semafor, 2026-01-09. https://www.semafor.com/article/01/09/2026/gmail-gets-gemini-but-falls-short-of-true-agentic-ai Independent analysis; US; M; opinion piece.
11. "Introducing ChatGPT Pulse", OpenAI, 2025-09-25. https://openai.com/blog/introducing-chatgpt-pulse Product announcement (read via secondary coverage, as the page blocked fetch); global; M.
12. ChatGPT agent plan limits: Motion blog "Is ChatGPT Pro worth it?" https://www.usemotion.com/blog/is-chatgpt-pro-worth-it-features-pricing-and-comparison and Yahoo/BGR https://tech.yahoo.com/ai/articles/chatgpt-agent-ai-now-available-130400858.html, 2025. Independent/third-party; global; L–M; OpenAI help page blocked (403), so limits may have changed.
13. "OpenAI launches ChatGPT Atlas", PureAI, 2025-10-21. https://pureai.com/articles/2025/10/21/openai-launches-chatgpt-atlas.aspx Independent news; global; M.
14. "Perplexity's Comet AI browser is now free for everyone", Engadget, Oct 2025. https://engadget.com/ai/perplexitys-comet-ai-browser-is-now-free-for-everyone-170017509.html Independent news; global; M.
15. "Get started with Claude in Chrome", Anthropic Help Center, 2026 (live). https://support.claude.com/en/articles/12012173-get-started-with-claude-in-chrome Product docs (via search snippet); global; M.
16. "Set up Claude Cowork", Anthropic, 2026. https://claude.com/resources/tutorials/cowork-onboarding-guide Product docs; global; M.
17. "Microsoft raises price of consumer version of Microsoft 365", CNBC via NBC New York, Jan 2025. https://www.nbcnewyork.com/news/business/money-report/microsoft-raises-price-of-consumer-version-of-microsoft-365-for-first-time-in-12-years/6110515/ Independent news; US; H.
18. "New Siri not in iOS 26.4 after all?", heise online, Feb 2026. https://heise.de/-11173976 Independent news; global; M; shipped status of iOS 27 Siri not verified.
19. "Announcing company rebrand to Superhuman", Grammarly/Superhuman, 2025-10-29. https://grammarly.com/blog/company/announcing-company-rebrand-to-superhuman Vendor announcement; global; H for facts, vendor claim for "100 agents".
20. Fyxer pricing page, Fyxer, live. https://www.fyxer.com/pricing Product docs; global; H.
21. "AI exec assistant startup Fyxer raises $30m", Sifted, Sep 2025. https://sifted.eu/articles/ai-exec-assistant-startup-fyxer-raises-30m-to-expand-to-the-us Independent news reporting vendor ARR; UK/US; M.
22. Fyxer press page, Fyxer, 2025–26. https://www.fyxer.com/press Vendor claim ($1M→$30M ARR, $43M raised); L.
23. Fyxer profile, Sacra, 2025. https://sacra.com/c/fyxer-ai Independent estimate ($9M ARR); L–M.
24. Lindy pricing page, Lindy, live. https://www.lindy.ai/pricing Product docs; global; H.
25. Shortwave pricing page, Shortwave, live. https://www.shortwave.com/pricing/ Product docs; global; H (Individual tab not captured).
26. "Motion raises $60M at $550M valuation", Business Wire, 2025-09-05. https://www.businesswire.com/news/home/20250905188051/en/ Vendor press release; US; M; ARR is a vendor claim.
27. Martin AI: eesel review https://www.eesel.ai/blog/martin-ai-review ; Chorus comparison https://chorus.com/compare/chorus-vs-martin ; 2025–26. Third-party/competitor; US; L.
28. Saner.ai: eesel pricing https://www.eesel.ai/blog/saner-ai-pricing ; Latka https://getlatka.com/companies/saner.ai ; 2026. Third-party; L; revenue unverified.
29. Lindy reviews, Trustpilot, 2025–26. https://www.trustpilot.com/review/lindy.ai Customer testimony; global; L–M; self-selected.
30. Fyxer reviews, Trustpilot, 2025–26. https://ca.trustpilot.com/review/fyxer.com?page=10 Customer testimony; global; L–M.
31. "PSA: OFWs increase by 1.5% in 2024", Inquirer, Dec 2025. https://globalnation.inquirer.net/?p=302711 ; also GMA https://www.gmanetwork.com/news/pinoyabroad/dispatch/969755/ofws-grew-to-2-19m-in-2024-psa/story/ Official stats via news; PH; H.
32. "Remittances reach record high $35.6 billion", Philstar, 2026-02-17. https://www.philstar.com/business/2026/02/17/2508451/remittances-reach-record-high-356-billion/amp/ Official stats (BSP) via news; PH; H.
33. "BSP data: OFW remittances go to food, basic needs", Inquirer, Dec 2025. https://newsinfo.inquirer.net/?p=2153866 Official stats via news; PH; M.
34. GCash OWWA payments, Fintech News Philippines, Apr 2026. https://fintechnews.ph/71078/e-wallets/gcash-ofw-digital-payments-owwa/ Independent news; PH; M.
35. "OFWs reject SSS contribution rate hike", Migrante International, Jan 2025. https://migranteinternational.org/ofws-reject-sss-contribution-rate-hike/ Advocacy statement; PH/diaspora; M; advocacy framing.
36. "ChatGPT Go arrives in the Philippines for Php 300/month", Unbox PH, Oct 2025. https://unbox.ph/news/chatgpt-go-philippines/ Independent news; PH; M.
37. Google AI Plus Philippines pricing, NoypiGeeks, late 2025. https://www.noypigeeks.com/?p=254747 Independent news; PH; L–M; US price cut in Jun 2026 may have changed PH pricing.
38. "The growing number of Filipino freelancers", PIDS (In the News), date unclear. https://www.pids.gov.ph/details/news/in-the-news/the-growing-number-of-filipino-freelancers-2 Secondary citation of GCash/Payoneer; PH; L.
39. "Philippines BIR freelancer & influencer tax 2026 guide", Beancount.io, 2026-08-13. https://beancount.io/blog/2026/08/13/philippines-bir-freelancer-influencer-tax-2026-guide Vendor blog; PH; L; not checked against BIR issuances.
40. GitHub REST API repository metadata (stars, language, licence), GitHub, queried 2026-10-18. https://api.github.com/repos/{owner}/{repo} Primary data; global; H; stars are a proxy, not revenue.
41. "Lightspeed leads Temporal's $550M funding round at $12.55B valuation", AF.NET, ~Sep 2026. https://af.net/realtime/lightspeed-leads-temporals-550m-funding-round-at-12-55b-valuation/ Aggregator news of vendor claims; US; M–L; official release not retrieved.
42. "Temporal in talks to double valuation to $5 billion", Bloomberg Government, Jan 2026. https://news.bgov.com/private-equity/temporal-in-talks-to-double-valuation-to-5-billion-as-ai-booms Independent news; US; M.
43. "Trigger.dev raises $16M Series A", Trigger.dev, Dec 2025. https://trigger.dev/blog/series-a Vendor announcement; H.
44. "Restate secures $7M seed funding", Techleap, 2024/2025 (dates conflict). https://finder.techleap.nl/news/feed/restate-secures-7m-seed-funding Independent aggregator; EU; M.
45. "A new programming model for durable execution", Vercel, 2025–26. https://vercel.com/blog/a-new-programming-model-for-durable-execution Vendor claim (100M runs, 1,500 customers); M.
46. "Workflows" (Agents docs), Cloudflare, live. https://developers.cloudflare.com/agents/concepts/workflows Product docs; H.
47. "Anthropic launches Managed Agents", Unite.AI, Apr 2026. https://www.unite.ai/anthropic-launches-managed-agents-to-run-enterprise-ai-workloads ; self-hosted sandboxes https://noqta.tn/en/news/anthropic-claude-self-hosted-sandboxes-mcp-tunnels-2026 Independent news; global; M; pricing secondary.
48. "Changelog: 2025-11-25", Model Context Protocol, 2025-11-25. https://modelcontextprotocol.io/specification/2025-11-25/changelog Standard/spec; H.
49. "Linux Foundation announces the formation of the Agentic AI Foundation", Linux Foundation, 2025-12-09. https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation Primary press release; H.
50. "Mem0 raises $24M to power long-term memory for AI", YourStory, 2025-10-28. https://yourstory.com/ai-story/san-francisco-startup-mem0-raises-24-million-long-term-ai-memory-power Independent news of vendor claims; M.
51. "Memory tool", Claude Platform docs, live. https://platform.claude.com/docs/en/agents-and-tools/tool-use/memory-tool Product docs; H.
52. "Auth0 for AI Agents is generally available", Auth0/Okta, Nov 2025. https://auth0.com/blog/auth0-for-ai-agents-generally-available/ Vendor docs; H; per-tier pricing not verified.
53. "Permit.io raises $8 million", Business Wire, 2024-02-13. https://www.businesswire.com/news/home/20240213359627/en/ Vendor press release; M; 2024 (older).
54. "How three YC startups built their companies with Claude Code" (HumanLayer story), Anthropic, 2025. https://www.claude.com/blog/building-companies-with-claude-code ; Pushary "vs HumanLayer" https://pushary.com/vs/humanlayer Vendor/competitor; M/L.
55. "Arcade raises $60 million Series A", Pulse 2.0, Jun 2026. https://pulse2.com/arcade-raises-60-million-series-a-to-secure-ai-agents-in-production/ Independent news; M.
56. Composio pricing, Composio, live (captured 2026). https://composio.dev/pricing.md Product docs; H. Series A ($25M, Lightspeed): https://thenote.app/post/en/composio-which-provides-tools-to-help-companies-build-ai-agents-raised-a-25m-jhqwzqnaug?amp=true (M; date not confirmed).
57. "Nango raises $7.5M led by Gradient", Nango, 2026. https://nango.dev/blog/nango-raises-7-5m-led-by-gradient Vendor claim; M; amount/date conflicts across aggregators.
58. "Workday signs definitive agreement to acquire Pipedream", Workday Newsroom, 2025-11-19. https://newsroom.workday.com/2025-11-19-Workday-Signs-Definitive-Agreement-to-Acquire-Pipedream Primary press release; H; closing not confirmed.
59. "Langfuse joins ClickHouse", Langfuse, Jan 2026. https://langfuse.com/blog/joining-clickhouse ; Orrick release https://www.orrick.com/en/news/2026/01/open-source-llm-observability-langfuse-acquired-by-clickhouse-inc Primary/counsel; H/M.
60. Langfuse pricing, Langfuse, live. https://langfuse.com/pricing Product docs; H.
61. "Helicone joins Mintlify", Helicone, 2026-03-03. https://helicone.ai/blog/joining-mintlify Primary; H.
62. "Braintrust raises $80M at $800M", Axios Pro, 2026-02-17. https://www.axios.com/pro/enterprise-software-deals/2026/02/17/ai-observability-braintrust-80-million-800-million Independent news; H.
63. Arize vs Braintrust, CB Insights, 2025–26. https://www.cbinsights.com/compare/arize-ai-vs-braintrust-data Independent database; M ($70M Series C, Feb 2025).
64. "LangChain is now a unicorn with a fresh $125 million", Fortune, 2025-10-20. https://fortune.com/2025/10/20/exclusive-early-ai-darling-langchain-is-now-a-unicorn-with-a-fresh-125-million-in-funding/ Independent news; H.
65. LangSmith ARR report, TechCrunch, 2025. https://techcrunch.com/?p=3025809 Independent news (anonymous sources); M.
66. Merge Agent Handler for Employees, SiliconANGLE, 2026-06-01. https://siliconangle.com/?p=767938 Independent news; M.
67. "Nango: integration infrastructure for AI agents", Gradient, 2026. https://gradient.com/blog/posts/nango-integration-infrastructure-ai-agents/ Investor claim; L–M.
68. Arcade pricing page, Arcade.dev, live. https://www.arcade.dev/pricing Product docs; H.
69. "OFW contributions guide", SweldoPH, 2026. https://sweldoph.com/guides/ofw-contributions Third-party guide; PH; L–M; not checked against SSS circulars.
