# LLM options for P1 reminder drafting

Status: research note, 2026-10-21. Nothing here is an owner-approved decision. "LLM provider and model" is still open for the Phase 6 spec (`docs/product/p1-brief.md`).
Accessed: 2026-10-21 for every source unless stated. This note re-verifies the LLM section (§9) of `integration-constraints.md` (verified 2026-10-18). **The 2026-10-18 prices and model IDs still hold**; what is new is listed under "What changed since 2026-10-18".

Evidence labels: **[PP]** official pricing page, **[OD]** official docs or terms, **[PA]** paper (peer-reviewed or preprint), **[DR]** developer or third-party report, **[VC]** vendor claim. Confidence **H/M/L**. Numbers in brackets such as [3] point to Sources. **Fact**, **Assumption** and **Recommendation** are labelled where they could be confused.

## Summary

- **Recommendation (not a decision): Claude Haiku 5.5 (`claude-haiku-5-5`) on the Anthropic API, called with thinking turned off, plain non-batch requests, and structured JSON output.** Keep Claude Sonnet 5.5 (`claude-sonnet-5-5`) as the fallback if the Taglish eval shows Haiku isn't good enough.
- **Cost per firm per month (300 calls × 600 in / 250 out tokens):**
  - Haiku 5.5: **$0.056, about ₱3.50**.
  - Sonnet 5.5: **$1.11, about ₱70**.
  - At 5 firms: Haiku **$0.28 (≈ ₱17)**, Sonnet **$5.55 (≈ ₱350)**.
  - Both fit the ₱600/month budget. At this volume the LLM is a rounding error, so pick on Taglish quality and data terms, not price.
- **Cheapest alternatives at the same volume:**
  - OpenAI `gpt-6-luna`: $0.056/firm. Same list price as Haiku 5.5.
  - Google `gemini-2.5-flash-lite` (paid tier): $0.048/firm, the cheapest. It is an older 2025 model.
- **Why Haiku over the equally cheap gpt-6-luna** (recommendation, M confidence):
  - Anthropic's commercial terms say plainly "Anthropic may not train models on Customer Content" [6].
  - Anthropic offers native JSON-schema output [10].
  - The project already plans a thin Anthropic client [integration-constraints §9].
  - Quality evidence for Taglish does **not** separate the two (see Taglish evidence). An in-house eval decides this, not the published benchmarks.
- **Blockers:** none hard. Watch these:
  1. A new Anthropic org starts on **prepaid credits**, which are non-refundable and expire after 1 year [7]. Minimum top-up: not found on an official page.
  2. **Haiku 5.5 thinks by default.** Thinking tokens bill as output, so send `thinking: {"type": "disabled"}`, or the cost can rise about 4× (still under ₱15/firm) [9].
  3. No published benchmark covers Taglish *writing* by current (2026) models. Run a 30–50 draft in-house eval before choosing.

## Price table

USD per 1M tokens, standard (synchronous) tier. "Cache read" is the per-token price of a prompt-cache hit.

| Provider · model (exact API ID) | Input | Cache write | Cache read | Output | Batch (in / out) | Source |
|---|---|---|---|---|---|---|
| **Anthropic · Claude Haiku 5.5** `claude-haiku-5-5` (prompt ≤ 100k tok) | **0.10** | 0.125 (5 m) / 0.20 (1 h) | 0.01 | **0.50** | 0.05 / 0.25 | [1] PP H |
| Anthropic · Claude Haiku 5.5, prompt > 100k tok (not relevant to P1) | 0.50 | 0.625 / 1.00 | 0.05 | 2.50 | 0.25 / 1.25 | [1] PP H |
| **Anthropic · Claude Sonnet 5.5** `claude-sonnet-5-5` (next tier up) | **2.00** | 2.50 / 4.00 | 0.10 | **10.00** | 1.00 / 5.00 | [1] PP H |
| Anthropic · Claude Haiku 4.5 (legacy) | 1.00 | 1.25 / 2.00 | 0.10 | 5.00 | 0.50 / 2.50 | [1] PP H |
| **OpenAI · GPT-6 Luna** `gpt-6-luna` | **0.10** | n/a | 0.01 | **0.50** | 0.05 / 0.25 (Flex same) | [12] PP H |
| OpenAI · `gpt-5.6-luna` | 0.20 | n/a | 0.02 | 1.20 | 0.10 / 0.60 | [12] PP H |
| OpenAI · `gpt-5-nano` (cheapest listed, **shuts down 2026-12-11**) | 0.05 | n/a | 0.005 | 0.40 | 0.025 / 0.20 | [12][14] PP/OD H |
| **Google · Gemini 2.5 Flash-Lite** `gemini-2.5-flash-lite` (paid tier) | **0.10** | storage $1.00/h | 0.01 | **0.40** | 0.05 / 0.20 | [15] PP H |
| Google · Gemini 3.1 Flash-Lite `gemini-3.1-flash-lite` (paid; earliest shutdown 2027-05-07) | 0.25 | storage $1.00/h | 0.025 | 1.50 | 0.125 / 0.75 | [15][16] PP/OD H |
| Google · Gemini 3.5 Flash-Lite `gemini-3.5-flash-lite` (paid; newest Flash-Lite, GA 2026-07-21) | 0.30 | storage $1.00/h | 0.03 | 2.50 | 0.15 / 1.25 | [15][16] PP/OD H |

Pricing modifiers that matter for P1:
- **Anthropic.**
  - Batch API is −50% on input and output [1].
  - `inference_geo: "us"` is ×1.1 [1]. Not needed: the default is `global`.
  - Claude 4.7+ models use a tokenizer that makes "approximately 30% more tokens for the same text" [1]. This is already in the per-token prices, but token *counts* differ from other vendors' counts.
  - Minimum cacheable prefix is 512 tokens on Haiku 5.5 and Sonnet 5.5 [11]. A ~600-token prompt with a ~400-token fixed system prompt **will not be cached**. Caching is irrelevant at this size.
  - Tool use adds 286–406 system tokens per call on Haiku 5.5 [1]. JSON-schema output (`output_config.format`) avoids this.
- **OpenAI.** Regional-processing (data-residency) endpoints add 10% for models released on or after 2026-03-05 [12]. There is no Asian processing region, only Asian storage [13].
- **Google.** Every Flash-Lite model has a **free tier that trains on inputs** ("Used to improve our products: Yes"). The paid tier is "No" [15]. **Never use the free tier.**

Model status (fact):
- **Anthropic.**
  - Current lineup: Fable 5.1, Opus 5.5, Sonnet 5.5, Haiku 5.5 [2].
  - Haiku 5.5 is described as "for high-volume, latency-sensitive tasks such as classification, extraction, and routing". It has a 1M context and 128K max output. Retirement is "not sooner than October 7, 2027" [2].
  - Sonnet 5.5 retirement is not sooner than September 28, 2027 [2].
- **OpenAI.**
  - `gpt-6-luna` is "our most efficient model for focused, high-volume tasks" and is not deprecated [17][14].
  - `gpt-5-nano` shuts down 2026-12-11 and `gpt-4.1-nano` on 2026-10-23. Both are replaced by `gpt-5.6-luna` [14].
- **Google.**
  - `gemini-2.5-flash-lite` has no shutdown date announced.
  - `gemini-3.1-flash-lite`'s earliest shutdown is 2027-05-07, replaced by `gemini-3.5-flash-lite` [16].

### What changed since 2026-10-18 (integration-constraints §9)
- All prices for Haiku 5.5, Sonnet 5.5, Opus 5.5, Haiku 4.5, gpt-6-luna and gemini-3.1-flash-lite are **unchanged**.
- **New in this note:**
  - `gemini-2.5-flash-lite` ($0.10/$0.40) and `gemini-3.5-flash-lite` ($0.30/$2.50) are on the pricing page.
  - `gpt-5-nano` and `gpt-5.6-luna` are on OpenAI's pricing page.
  - Gemini **3.8 Flash** was not re-checked; it isn't needed for P1.
- **Correction to §9.3, Anthropic retention.** The current docs page now says "Conversation content (your prompts and Claude's outputs) is not retained by default", except for Covered Models (Fable/Mythos), which need 30-day retention [4]. The privacy-center article (updated 2026-07-01) still says "within 30 days" [5]. The two official pages disagree. **Plan on 30 days for the DPA** (the conservative reading).

## Data terms

| | Anthropic API | OpenAI API | Gemini API (paid tier) |
|---|---|---|---|
| Training on API inputs | **No.** "Anthropic may not train models on Customer Content from Services" (Commercial Terms §B, effective June 17, 2025) [6]. "Retained data is never used for model training without your express permission" [4]. OD H | **No by default.** "data sent to the OpenAI API is not used to train or improve OpenAI models", unless you opt in [13]. OD H | **No.** "Google doesn't use your prompts or responses to improve our products" (Paid Services) [18]. OD H |
| Free tier trains? | No free tier, only small trial credits for new users [1] | n/a | **Yes.** Unpaid Services content is used "to provide, improve, and develop", with human review; "Do not submit sensitive, confidential, or personal information to the Unpaid Services" [18]. A project is "paid" only with an active Cloud billing account [18]. OD H |
| Default retention | Docs: not retained by default (except Fable/Mythos: 30 days) [4]. Privacy center: deleted "within 30 days" [5]. Usage-policy flags: up to 2 years [5]. **Batch API: 29 days** [4]. OD H/M (pages conflict) | Abuse-monitoring logs "up to 30 days" [13] | "logs prompts and responses for a limited period of time" for abuse detection, with no number given [18]. May be "stored transiently or cached in any country" [18] |
| Zero data retention | Via sales, per org. Messages API and structured outputs are eligible; **Batch is not** [4] | "subject to prior approval" [13] | not found |
| DPA | Incorporated into the Commercial Terms by reference [6] | not checked | not checked |
| Governing law (non-EEA customer) | California [6] | not checked | not checked |

**What this means for P1 (recommendation).**
- The model only sees client names, missing-item lists, deadlines and tone. That is personal data, not document contents.
- Anthropic, OpenAI and Gemini *paid* all fit the "no training" rule. List the chosen provider as a sub-processor in the DPA.
- Prefer synchronous calls over the Batch API. Batch keeps data 29 days and isn't ZDR-eligible [4], and it saves only ~$0.03/firm.

### Account opening and billing from the Philippines (Anthropic)
- **Philippines is a supported country** for the API and Claude.ai [8] (OD, M: the fetched page header reads "Effective November 12, 2026", a future date, so recheck it).
- **Prepaid credits are required.** "Buy credits before you use the API". Credits "expire one year from the purchase date" and are "non-refundable"; auto-reload is optional [7] (OD H). Commercial Terms §H.1: Anthropic "may require prepayment … in the form of credits" [6].
- **Minimum credit purchase: not found** on an official page. A third-party guide says $5 [19] (DR, L).
- **Payment:** "Major credit cards are accepted for standard accounts" [1] (PP H). Whether a PH-issued debit or virtual card (e.g. GCash/Maya virtual Visa) works: not found. The owner must test this.
- **New-org limits.**
  - New organizations "may start in the Evaluation tier", with limits below Start-tier, while history builds [3].
  - Start tier: monthly spend cap **$500**; Haiku 5.5 at 1,000 RPM / 2M ITPM / 400k OTPM [3].
  - P1's ~300 calls/month per firm is nowhere near these limits.
  - Set a self-imposed **spend limit** in the Console (e.g. $10) as a budget guard. Requests over it return HTTP 400 [3].

## Cost arithmetic

Assumptions from the brief:
- Per firm per month: 240 drafts + 60 re-drafts = **300 calls**.
- Each call: **600 input tokens** (system prompt + context) and **250 output tokens**.
- No caching, since the prompt is too short to be worth it (see Price table).
- **Exchange rate (assumption):** ₱63 per USD, rounded up from ~₱62.6 on 2026-09-08 [20] (DR, M).

Token totals per firm: input 300 × 600 = **180,000 = 0.18 MTok**; output 300 × 250 = **75,000 = 0.075 MTok**.

Per-firm monthly cost = 0.18 × input price + 0.075 × output price:

| Model | Input cost | Output cost | **Per firm / month** | ≈ PHP | **5 firms / month** | ≈ PHP |
|---|---|---|---|---|---|---|
| **Claude Haiku 5.5** | 0.18 × 0.10 = $0.0180 | 0.075 × 0.50 = $0.0375 | **$0.0555** | ₱3.50 | **$0.28** | ₱17 |
| Claude Haiku 5.5, Batch API | 0.18 × 0.05 = $0.0090 | 0.075 × 0.25 = $0.0188 | $0.0278 | ₱1.75 | $0.14 | ₱9 |
| **Claude Sonnet 5.5** | 0.18 × 2.00 = $0.36 | 0.075 × 10.00 = $0.75 | **$1.11** | ₱70 | **$5.55** | ₱350 |
| Claude Haiku 4.5 | 0.18 × 1.00 = $0.18 | 0.075 × 5.00 = $0.375 | $0.555 | ₱35 | $2.78 | ₱175 |
| OpenAI gpt-6-luna | 0.18 × 0.10 = $0.0180 | 0.075 × 0.50 = $0.0375 | **$0.0555** | ₱3.50 | $0.28 | ₱17 |
| OpenAI gpt-5.6-luna | 0.18 × 0.20 = $0.036 | 0.075 × 1.20 = $0.090 | $0.126 | ₱8 | $0.63 | ₱40 |
| Gemini 2.5 Flash-Lite (paid) | 0.18 × 0.10 = $0.0180 | 0.075 × 0.40 = $0.0300 | **$0.048** | ₱3 | $0.24 | ₱15 |
| Gemini 3.1 Flash-Lite (paid) | 0.18 × 0.25 = $0.045 | 0.075 × 1.50 = $0.1125 | $0.1575 | ₱10 | $0.79 | ₱50 |
| Gemini 3.5 Flash-Lite (paid) | 0.18 × 0.30 = $0.054 | 0.075 × 2.50 = $0.1875 | $0.2415 | ₱15 | $1.21 | ₱76 |

Sensitivity checks (assumptions):
- **Thinking left on.** Haiku 5.5 and Sonnet 5.5 think by default (adaptive), and thinking tokens bill as output [9].
  - With thinking on, assume +1,000 output tokens per call (assumption, not measured).
  - Haiku 5.5 rises to 0.18 × 0.10 + 0.375 × 0.50 = **$0.21/firm (₱13)**.
  - Sonnet 5.5 rises to 0.18 × 2 + 0.375 × 10 = **$4.11/firm (₱259)**. At 5 firms that is **$20.55 (≈ ₱1,295), which breaks the ₱600 budget.**
  - **Haiku 5.5 accepts `thinking: {"type": "disabled"}`.** Sonnet 5.5 rejects `disabled`; send `{"type": "between_tools"}`, which turns off up-front thinking. With no tools in the request, that should keep Sonnet near its base cost (inference from the docs, not measured) [9].
- **Taglish tokenizes worse than English.** +30% tokens (assumption) moves Haiku 5.5 to $0.072/firm. That is immaterial.
- **Budget headroom.** ₱600 ≈ $9.50/month. On Haiku 5.5 with thinking off, that covers about 170 firms at this volume ($9.50 ÷ $0.0555). On Sonnet 5.5, it covers about 8 firms ($9.50 ÷ $1.11). The LLM line won't strain the pilot budget on either model.

## Taglish evidence

**Overall: weak.** No published benchmark measures Taglish *writing* (code-switched, business-polite email) for Claude Haiku/Sonnet 5.5, gpt-6-luna or Gemini Flash-Lite. Treat every quality claim below as indirect.

| Evidence | What it shows | Type · confidence |
|---|---|---|
| **FilBench** (Miranda et al., EMNLP 2025) [21] | 27 LLMs on Filipino, Tagalog and Cebuano. The only commercial models are OpenAI's: `gpt-4o-2024-08-06` scored 72.73 overall and **46.48 on Generation**; `gpt-4o-mini` scored 42.32 overall and **23.29 on Generation**. (The paper also reports GPT-4o as 72.23% and 75.56% in other places.) "Text generation suffers the most, with the lowest scores across models." In 100 GPT-4o failures, 47% were overgeneration, 34% poor instruction-following and 19% hallucination. Few-shot examples helped: Tatoeba (TGL) went from 51.88 to 61.65 with 5 shots. **No Claude or Gemini models, and no 2026 models.** | PA · H for what it measured; L as a proxy for 2026 models |
| **Batayan** (Montalan et al., ACL 2025) [22] | 8 tasks covering understanding, reasoning and generation, in **both Tagalog and code-switched Taglish**, built by native speakers to avoid "translationese". Reports "significant performance gaps that signal the under-representation of Filipino in pre-training corpora". The abstract does not name the models tested. | PA · H for the existence of the benchmark; per-model results not read (abstract only) |
| **SEA-HELM leaderboard** (AI Singapore; updated 2026-09-18) [23] | Includes Filipino (TL). Lists 16 closed-weight models, but the fetched page did not show their names or TL scores (JS-rendered). | OD of the leaderboard owner · not usable as fetched |
| **Anthropic multilingual table** [24] | Gives scores relative to English for Sonnet 4.5 and Haiku 4.5 across 14 languages. **Tagalog/Filipino is not listed.** Indonesian, the nearest Austronesian proxy, is 94.2% for Haiku 4.5. There are no rows for the 5.5 models. | VC (Anthropic's own eval) · L as a Taglish proxy |
| Search for Taglish evaluations of Claude Haiku 5.5, gpt-6-luna or Gemini Flash-Lite | **Not found.** | — |

**What this means (recommendation).**
- The published evidence can't rank the three cheap models on Taglish. FilBench says small models generate Filipino noticeably worse than flagship models (gpt-4o-mini vs gpt-4o), so the Haiku-vs-Sonnet question is real.
- **Build a small in-house eval before Phase 7.** Write ~30–50 reminder briefs: real-shaped names, missing items and deadlines, with fake data. For each brief, draft with Haiku 5.5 (thinking off), Sonnet 5.5 and gpt-6-luna.
- Have the owner rate each draft blind on: natural Taglish, politeness/register, correct facts (items and dates), and length 80–150 words.
- Cost of this eval: under $1.
- Include 2–3 example reminders (few-shot) in the system prompt. FilBench found few-shot examples reduce instruction-following failures [21].

## API shape notes for the Rust client

All facts below are from official Anthropic docs, accessed 2026-10-21 (OD, H unless stated).

**Endpoint and headers** [25][26][10]
- `POST https://api.anthropic.com/v1/messages`
- Headers:
  - `x-api-key: <key>`
  - `anthropic-version: 2023-06-01`. This is required ("you must send an `anthropic-version` request header"), and 2023-06-01 is the latest listed version [26].
  - `content-type: application/json`
- Each response carries a `request-id` header (e.g. `req_018E…`). Log it with every model call [27].
- Rate-limit headers: `anthropic-ratelimit-{requests,input-tokens,output-tokens}-{limit,remaining,reset}` and `retry-after` [3].

**Request body for P1** (shape only; values illustrative)
```json
{
  "model": "claude-haiku-5-5",
  "max_tokens": 600,
  "system": "You draft short, polite reminder emails for a Philippine bookkeeper...",
  "messages": [{"role": "user", "content": "<client name, missing items, deadline, tone>"}],
  "thinking": {"type": "disabled"},
  "output_config": {
    "effort": "low",
    "format": {
      "type": "json_schema",
      "schema": {
        "type": "object",
        "properties": {
          "subject": {"type": "string"},
          "body": {"type": "string"},
          "language": {"type": "string", "enum": ["taglish", "english"]}
        },
        "required": ["subject", "body", "language"],
        "additionalProperties": false
      }
    }
  }
}
```
- **`thinking`.**
  - Haiku 5.5 thinks by default.
  - `{"type": "disabled"}` is accepted at effort `high` or below. At `xhigh` or `max` it returns 400 [9][27].
  - Sonnet 5.5 rejects `disabled`; use `{"type": "between_tools"}` instead [9].
  - `"enabled"` with `budget_tokens` is rejected on both [27].
  - Thinking tokens bill as output and count toward `max_tokens` [9].
- **`output_config.format`** (JSON schema) [10]:
  - No beta header is needed. `claude-haiku-5-5` is supported.
  - The old `output_format` parameter is deprecated.
  - Constrained decoding guarantees valid JSON, **except** on `stop_reason: "refusal"` or `"max_tokens"`.
  - Every object needs `additionalProperties: false`.
  - `minLength`/`maxLength` and `minimum`/`maximum` are **not supported** (400). Enforce the 80–150 word limit in Rust after parsing.
  - Enum string casing may change; compare case-insensitively.
  - Only the schema is cached (≤ 24 h), not the prompts [4].
- **Fields to omit.**
  - `temperature`, `top_p` and `top_k` are not supported on Claude 4.7+ models (400 if set to a non-default value) [28].
  - Assistant-message prefill is not supported on 4.6+ models (400) [27].
- **Tools are an alternative to JSON output.**
  - `tools` with `strict: true`, plus `tool_choice`.
  - Forced tool use (`tool_choice` `any`/`tool`) is **rejected on Sonnet 5.5 and Opus 5.5** [27].
  - Prefer `output_config.format` for P1.

**Response body** [28]
```json
{
  "id": "msg_…", "type": "message", "role": "assistant",
  "content": [{"type": "text", "text": "{\"subject\":…}"}],
  "model": "claude-haiku-5-5",
  "stop_reason": "end_turn", "stop_sequence": null,
  "usage": {"input_tokens": 0, "output_tokens": 0}
}
```
- With structured output, the JSON arrives as a string in the `text` content block [10]. Parse it with `serde_json` into a typed struct, and treat it as untrusted input per AGENTS.md.
- **`content` can also hold `thinking` blocks** when thinking is on. Deserialize `content` as a tagged enum on `type` with an `#[serde(other)]` catch-all, because Anthropic may add new variants [26].
- **`usage`** also carries `cache_creation_input_tokens` and `cache_read_input_tokens` [3][11]. Store all usage fields per call for the cost ledger.
- **`stop_reason` values:** `end_turn`, `max_tokens`, `stop_sequence`, `tool_use`, `pause_turn`, `refusal` (with `stop_details`), `model_context_window_exceeded` [29]. P1 should accept only `end_turn`. Treat `max_tokens` and `refusal` as a failed draft, never a sendable one.

**Errors** [27]
- Body: `{"type": "error", "error": {"type": "...", "message": "..."}, "request_id": "req_..."}`.
- Status codes and handling:

| Status | Type | Handling |
|---|---|---|
| 400 | `invalid_request_error` | Also returned when *your own* spend limit is reached. Don't retry. |
| 401 | `authentication_error` | Don't retry. |
| 402 | `billing_error` | Don't retry. |
| 403 | `permission_error` | Don't retry. |
| 404 | `not_found_error` | Don't retry. |
| 413 | `request_too_large` | Don't retry. |
| 429 | `rate_limit_error` | Retry after `retry-after`. **Exception:** a tier spend-cap 429 has **no `retry-after`** and `error.details.error_code: "enforced_spend_limit_reached"` [3]. Stop retrying until the month resets. |
| 500 | `api_error` | Retry with backoff. |
| 504 | `timeout_error` | Retry with backoff. |
| 529 | `overloaded_error` | Retry with backoff. |

- Official SDKs retry twice by default with exponential backoff. Mirror that with a bounded retry count in the Rust client.

**Rust SDK status** [30][31] (OD + crates.io registry data, H)
- **No official Rust SDK.** Anthropic's official SDKs are Python, TypeScript, C#, Go, Java, PHP and Ruby. The `anthropics` GitHub org has no `anthropic-sdk-rust` repo (API returns 404).
- **Impostor and misleading crates.** All are unofficial; don't depend on them for the thin client.

| Crate | Version / last updated | Owner | Problem |
|---|---|---|---|
| **`anthropic_rust`** | 0.1.3 | `tmikus` | **Lists `github.com/anthropics/anthropic-sdk-rust` as its repository, which does not exist.** Impostor-style metadata. |
| `anthropic` | 0.0.8, 2024-09 | `AbdelStark` | Describes itself as "Anthropic Rust SDK" |
| `anthropic-sdk` | 0.1.5, 2024-07 | `Mixpeal` | Describes itself as "Anthropic Rust SDK" |
| `anthropic-rs` | 2024-09 | not checked | Describes itself as "Anthropic Rust SDK" |
| `claude-sdk` | 2.1.0, 2026-06 | `mcfearsome` | "Native Rust SDK for the Claude API" |
| `claude` | — | — | Unrelated: a money library |

- Maintained third-party multi-provider options exist: `genai` 0.7.0 (2026-10-20) and `rig-core` 0.45.0 (2026-10-19). They are a valid comparison point, but the plan stays a thin reqwest client: one endpoint and ~6 types.

## Unresolved questions

1. **Minimum first credit purchase** on the Anthropic Console: not found officially; a third-party guide says $5 [19]. The owner sees it at Settings › Billing.
2. **Whether a Philippine-issued card is accepted.** This covers GCash/Maya virtual Visa and local debit cards. Not found; the owner must test at signup. If declined, the fallback is the same model via Google Cloud or another billing route (not researched).
3. **Anthropic default retention.** The docs say "not retained by default" [4]; the privacy center says "within 30 days" [5]. Ask support, or state 30 days in the DPA.
4. **The "Evaluation tier" limits** for brand-new orgs are not published [3]. P1's volume is tiny, so the risk is low.
5. **Taglish quality ranking** of Haiku 5.5 vs Sonnet 5.5 vs gpt-6-luna. No public evidence; needs the in-house eval described above.
6. **Real token counts.** 600/250 is an assumption. Measure with the Token Counting API or `usage` on the first 20 eval drafts. Taglish may tokenize worse than English.
7. **Supported-countries page date** reads "Effective November 12, 2026", a future date. Recheck that the Philippines is still listed at signup.

## Sources

All accessed 2026-10-21.

1. Anthropic, "Pricing", platform.claude.com/docs/en/about-claude/pricing. Official pricing page · H.
2. Anthropic, "Models overview", platform.claude.com/docs/en/about-claude/models/overview. OD · H.
3. Anthropic, "Rate limits", platform.claude.com/docs/en/api/rate-limits. OD · H.
4. Anthropic, "API and data retention", platform.claude.com/docs/en/manage-claude/api-and-data-retention. OD · H.
5. Anthropic Privacy Center, "How long do you store my organization's data?", privacy.claude.com/en/articles/7996866-how-long-do-you-store-my-organization-s-data (updated July 1, 2026). OD · H.
6. Anthropic, "Commercial Terms of Service", www.anthropic.com/legal/commercial-terms (effective June 17, 2025). OD (legal terms) · H.
7. Anthropic Help Center, "How do I pay for my API usage?", support.claude.com/en/articles/8977456-how-do-i-pay-for-my-api-usage (no calendar date shown). OD · H.
8. Anthropic, "Supported countries and regions", www.anthropic.com/supported-countries (page header "Effective November 12, 2026"). OD · M.
9. Anthropic, "Thinking", platform.claude.com/docs/en/build-with-claude/thinking. OD · H.
10. Anthropic, "Structured outputs", platform.claude.com/docs/en/build-with-claude/structured-outputs. OD · H.
11. Anthropic, "Prompt caching", platform.claude.com/docs/en/build-with-claude/prompt-caching. OD · H.
12. OpenAI, "Pricing", developers.openai.com/api/docs/pricing. Official pricing page · H.
13. OpenAI, "Data controls in the OpenAI platform", developers.openai.com/api/docs/guides/your-data (no page date). OD · H.
14. OpenAI, "Deprecations", developers.openai.com/api/docs/deprecations. OD · H.
15. Google, "Gemini Developer API pricing", ai.google.dev/gemini-api/docs/pricing. Official pricing page · H.
16. Google, "Gemini deprecations", ai.google.dev/gemini-api/docs/deprecations. OD · H.
17. OpenAI, "Models", developers.openai.com/api/docs/models. OD · H.
18. Google, "Gemini API Additional Terms of Service", ai.google.dev/gemini-api/terms (last updated 2026-04-28; effective March 23, 2026). OD (legal terms) · H.
19. Search-result excerpt of a third-party API-key guide (docs.mydbsync.com, "Claude API key") stating a $5 minimum credit purchase. DR · L. Not confirmed on an Anthropic page.
20. PayMongo currency tool citing a BSP reference rate of ~₱62.60/USD for 2026-09-08 (www.paymongo.com/tools/currency-converter, via search excerpt). DR · M. The BSP daily table (bsp.gov.ph/statistics/external/day99_data.aspx) was last updated 2026-07-16 per the search excerpt.
21. Miranda et al., "FilBench: Can LLMs Understand and Generate Filipino?", arxiv.org/html/2508.03523v1 (EMNLP 2025). PA · H.
22. Montalan et al., "Batayan: A Filipino NLP benchmark for evaluating Large Language Models", arxiv.org/abs/2502.14911 (ACL 2025 main; v2 19 Jun 2025). PA · H (abstract only read).
23. AI Singapore, SEA-HELM leaderboard, leaderboard.sea-lion.ai (updated 2026-09-18). Leaderboard · L (model names and TL scores not visible to the fetcher).
24. Anthropic, "Multilingual support", platform.claude.com/docs/en/build-with-claude/multilingual-support. VC/OD · H for what it lists.
25. Anthropic, "Messages" API reference, platform.claude.com/docs/en/api/messages. OD · H (request side only; the page is too large to read fully).
26. Anthropic, "Versions", platform.claude.com/docs/en/api/versioning. OD · H.
27. Anthropic, "Claude API errors", platform.claude.com/docs/en/api/errors. OD · H.
28. Anthropic, "Using the Messages API", platform.claude.com/docs/en/build-with-claude/working-with-messages. OD · H.
29. Anthropic, "Handling stop reasons", platform.claude.com/docs/en/build-with-claude/handling-stop-reasons. OD · H.
30. Anthropic, "SDKs, CLI, and libraries", platform.claude.com/docs/en/cli-sdks-libraries/overview. OD · H.
31. crates.io registry API (crates.io/api/v1/crates/{name} and /owners) for `anthropic_rust`, `anthropic`, `anthropic-sdk`, `anthropic-rs`, `claude-sdk`, `claude`, `genai`, `rig-core`; GitHub API api.github.com/repos/anthropics/anthropic-sdk-rust (HTTP 404) and the anthropics org repo list. Registry data · H.
