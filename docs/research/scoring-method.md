# Opportunity scoring method

Status: **fixed before scoring** (written 2026-10-18, before research results were read), so weights could not be tuned to favour a pre-chosen answer.

## Scale (applies to every criterion)

Every criterion is scored 1–5 where **5 is always favourable to us** (so "risk" criteria are inverted: 5 = low risk).

| Score | Meaning |
|---|---|
| 5 | Strongly favourable, supported by direct evidence |
| 4 | Favourable |
| 3 | Neutral / mixed / unknown-but-plausible |
| 2 | Unfavourable |
| 1 | Strongly unfavourable or disqualifying |

Each score carries a tag:

- **E** — evidence-backed (cites a source in the research docs).
- **J** — judgement (reasoned estimate, no direct evidence). Judgement scores are capped at 4 — absence of evidence can't earn a 5.

## Criteria

| # | Criterion | 5 means… | 1 means… |
|---|---|---|---|
| C1 | Problem severity | Lost revenue or serious risk if unsolved | Mild annoyance |
| C2 | Frequency | Daily, many times | Rare / yearly |
| C3 | Economic value | Value ≫ plausible price (≥10×) | Value ≈ or < price |
| C4 | Demand evidence | People already pay for partial solutions in this segment | Only survey interest |
| C5 | Willingness to pay (target segment) | Published price points in target segment ≥ our cost floor with margin | Segment pays nothing / expects free |
| C6 | Competitive room | Clear, evidenced gap; incumbents weak or absent | Free incumbent (platform owner) does it well |
| C7 | Reachability | Cheap, direct channel to buyers we can access | Enterprise sales / paid ads only |
| C8 | Build feasibility | MVP buildable by 1 dev in ≤ 8 weeks | Needs a team / research breakthrough |
| C9 | Integration ease | Public APIs, no gated review, or reviews ≤ 2 weeks | Restricted scopes, partner programs, audits |
| C10 | Reliability tolerance | Mistakes are cheap and reviewable (draft → human approves) | One mistake causes serious harm |
| C11 | Regulatory / privacy risk (inverted) | Low-sensitivity data | Health/financial/identity data, regulated advice |
| C12 | Time to demonstrable MVP | ≤ 4 weeks | > 4 months |
| C13 | Operating cost & support burden (inverted) | Low LLM cost per customer, self-serve | High per-customer cost, done-for-you support |
| C14 | Recurring revenue | Ongoing, daily use, naturally subscription | One-off |
| C15 | Expansion potential | Adjacent workflows, segments, markets reuse the core | Dead end |
| C16 | Foundation fit | Exercises the reusable agent core (tools, approvals, durable tasks) in Rust | Mostly UI/CRUD; little agent core reuse |

## Weight profiles

Weights sum to 100. Three profiles are run so the ranking's sensitivity is visible.

| Criterion | **Balanced** (default) | **Revenue-first** | **Builder-first** |
|---|---|---|---|
| C1 Severity | 8 | 8 | 6 |
| C2 Frequency | 6 | 6 | 5 |
| C3 Economic value | 8 | 10 | 6 |
| C4 Demand evidence | 8 | 10 | 6 |
| C5 WTP | 9 | 12 | 6 |
| C6 Competitive room | 8 | 8 | 6 |
| C7 Reachability | 8 | 10 | 5 |
| C8 Build feasibility | 6 | 4 | 9 |
| C9 Integration ease | 5 | 3 | 7 |
| C10 Reliability tolerance | 5 | 3 | 6 |
| C11 Reg/privacy risk | 5 | 4 | 5 |
| C12 Time to MVP | 5 | 4 | 7 |
| C13 Op cost/support | 5 | 6 | 5 |
| C14 Recurring revenue | 6 | 8 | 5 |
| C15 Expansion | 4 | 2 | 6 |
| C16 Foundation fit | 4 | 2 | 10 |
| **Total** | **100** | **100** | **100** |

**Why Balanced is the default:** the stated objective is "a meaningful problem, feasible, commercially promising, growing into a reusable foundation." Demand-side criteria (C1–C7) carry 55 % because a technically good product nobody buys is the most common failure; feasibility/risk (C8–C13) carry 31 % because a solo builder must ship; strategic fit (C14–C16) carries 14 %.

**Why the alternates:** *Revenue-first* tests whether the winner survives if the owner prioritises income over platform-building. *Builder-first* tests the opposite bias — it deliberately over-weights what is fun/natural for a Rust developer, so if a candidate wins **only** here, that is a warning sign, not a recommendation.

## Weighted score

`score = Σ (weight_i × score_i) / 5` → range 20–100.

An opportunity is a **finalist** only if it is in the top 3 under Balanced **and** in the top 5 under Revenue-first. Any **disqualifying 1** on C5, C6 or C11 is called out regardless of total.
