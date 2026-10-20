# Opportunity scorecard

## Current result (2026-10-21)

| Rank | ID | Opportunity | Balanced | Revenue-first | Builder-first | Status |
|---|---|---|---|---|---|---|
| 1 | P1 | Bookkeeper document, 2307 & BIR-deadline chaser | 76.0 | 76.4 | 76.8 | **Owner-approved first product**; kill-checked (§12) |
| 2 | S1 | Seller tax desk for Shopee/Lazada/TikTok sellers | 74.4 | 75.4 | 74.6 | Fallback; not kill-checked |
| 3 | B2 | Lead & quote follow-up, one US/AU service vertical | 72.6 | 72.2 | 74.2 | Shelved |
| 4 | HR2 | Payroll-input & remittance chaser for bookkeepers | 72.6 | 72.0 | 74.4 | Planned P1 add-on |

Monte Carlo (±1 on every judgement score, 20,000 trials): P1 is top-1 in 36% of trials and top-3 in 75%; S1 is top-1 in 22%. The sections below are the history of how the ranking got here: §1–6 are the original 2026-10-18 ranking, where B led; §7–12 are the dated updates. Run `python3 -I scoring/score.py` for the full current table.

---

Original status: **preliminary ranking, 2026-10-18.** Method, scale and weights are in [scoring-method.md](scoring-method.md), fixed before scoring. The scores are reproducible with `python3 -I scoring/score.py` (totals) and `python3 -I scoring/mc.py` (sensitivity). Short refs (BIZ, PH, COMP, INT, IND) are defined in [opportunity-landscape.md](opportunity-landscape.md).

**Read this as a ranking of hypotheses, not of validated businesses.** Only 0–10 of 16 criteria per candidate are evidence-backed. Every candidate's willingness-to-pay in its exact target segment is unvalidated.

## 1. Ranking under three weight profiles

`score = Σ weight × criterion / 5`, so the range is 20–100. The rank under each profile is in brackets.

| Rank (Bal.) | ID | Opportunity | Balanced | Revenue-first | Builder-first | Evidence-backed criteria | Disqualifier (a 1 on C5/C6/C11) |
|---|---|---|---|---|---|---|---|
| 1 | B | Owner-approved lead & quote follow-up, PH service SMEs | 74.0 (#1) | 73.0 (#1) | 75.2 (#1) | 6/16 | — |
| 2 | B2 | Lead & quote follow-up, one US/AU service vertical | 72.6 (#2) | 72.2 (#3) | 74.2 (#2) | 5/16 | — |
| 3 | C | AR collections / payment-reminder agent on Xero/QBO (intl) | 71.8 (#3) | 72.8 (#2) | 72.2 (#3) | 7/16 | — |
| 4 | A | Clinic/dental inquiry→booking + reminders (PH) | 71.4 (#4) | 72.0 (#4) | 72.0 (#4) | 8/16 | — |
| 5 | G | Travel/tour inquiry→quote→follow-up (PH) | 66.4 (#5) | 65.0 (#6) | 67.6 (#5) | 3/16 | — |
| 6 | K | Prosumer email triage & drafting | 64.2 (#6) | 64.8 (#7) | 63.4 (#7) | 8/16 | — |
| 7 | N2 | Infra: agent approval/permission layer | 63.8 (#7) | 61.4 (#9) | 67.2 (#6) | 3/16 | — |
| 8 | E | Voice AI receptionist | 63.2 (#8) | 66.0 (#5) | 60.0 (#12) | 10/16 | C6 |
| 9 | N1 | Infra: durable agent execution | 62.8 (#9) | 63.2 (#8) | 63.4 (#8) | 4/16 | C6 |
| 10 | P | Outbound prospecting / cold-email agent | 60.6 (#10) | 60.2 (#10) | 62.8 (#9) | 0/16 | — |
| 11 | D | Generic Messenger FAQ/inquiry bot (PH) | 60.4 (#11) | 60.0 (#11) | 60.8 (#11) | 6/16 | C6 |
| 12 | N4 | Infra: agent observability | 59.6 (#12) | 59.0 (#13) | 61.0 (#10) | 3/16 | C6 |
| 13 | H | Workflow glue between SME apps | 59.0 (#13) | 58.8 (#14) | 60.0 (#13) | 3/16 | C6 |
| 14 | M | Filipino freelancer admin | 58.2 (#14) | 57.6 (#15) | 58.6 (#15) | 1/16 | — |
| 15 | N3 | Infra: integration platform for agents | 57.2 (#15) | 59.2 (#12) | 55.6 (#17) | 5/16 | C6 |
| 16 | F | SME document intake & extraction | 56.6 (#16) | 56.2 (#16) | 57.8 (#16) | 4/16 | — |
| 17 | I | SME research/reporting assistant | 54.6 (#17) | 50.8 (#18) | 59.0 (#14) | 0/16 | — |
| 18 | L | OFW household admin | 54.2 (#18) | 54.4 (#17) | 54.0 (#18) | 2/16 | — |
| 19 | J | General personal agent (consumer) | 50.4 (#19) | 49.2 (#19) | 53.0 (#19) | 6/16 | C6 |

**Finalists** (rule from the method: top 3 Balanced **and** top 5 Revenue-first): **B, B2, C**. **A** is added as a fourth because it is 0.4 points behind C and is the most evidence-backed of the four. Its workflow overlaps B so heavily that it is treated as a *vertical of B* rather than a separate product.

## 2. Score matrix

C1 Severity · C2 Frequency · C3 Economic value · C4 Demand evidence · C5 WTP · C6 Competitive room · C7 Reachability · C8 Build feasibility · C9 Integration ease · C10 Reliability tolerance · C11 Reg/privacy risk (inv.) · C12 Time to MVP · C13 Op cost/support (inv.) · C14 Recurring · C15 Expansion · C16 Foundation fit. E = evidence-backed, J = judgement (capped at 4).

| ID | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | C10 | C11 | C12 | C13 | C14 | C15 | C16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| B | 4E | 4E | 4J | 3E | 3E | 4E | 3J | 4J | 3E | 4J | 4J | 4J | 4J | 4J | 4J | 4J |
| B2 | 4E | 4E | 4J | 4E | 4E | 2E | 2J | 4J | 3J | 4J | 4J | 4J | 4J | 4J | 4J | 4J |
| C | 4E | 3E | 5E | 4E | 4E | 3E | 2J | 4J | 3J | 3J | 3J | 4J | 4J | 4E | 3J | 4J |
| A | 4E | 5E | 4J | 4E | 4E | 2E | 3J | 4J | 3E | 3J | 2E | 4J | 3J | 4E | 4J | 4J |
| G | 3J | 4J | 4J | 2J | 3E | 4E | 3J | 3J | 3E | 4J | 4J | 3J | 3J | 3J | 4J | 4J |
| K | 4E | 5E | 3J | 4E | 4E | 2E | 2J | 3J | 3E | 3J | 3E | 3J | 3J | 3E | 3J | 3J |
| N2 | 4J | 4J | 3J | 2E | 2E | 2E | 3J | 4J | 4J | 3J | 4J | 4J | 4J | 3J | 3J | 4J |
| E | 4E | 5E | 4E | 5E | 4E | 1E | 2J | 3J | 2E | 2E | 3J | 3J | 2E | 4E | 3J | 2J |
| N1 | 4E | 4J | 4J | 4E | 3E | 1E | 2J | 2J | 4J | 2J | 4J | 2J | 3J | 4J | 4J | 4J |
| P | 3J | 4J | 3J | 3J | 3J | 2J | 3J | 4J | 3J | 2J | 2J | 4J | 3J | 3J | 3J | 4J |
| D | 3E | 5E | 3J | 4E | 2E | 1E | 3J | 4J | 3E | 3J | 3J | 4J | 3J | 3J | 3J | 2J |
| N4 | 3J | 4J | 3J | 3E | 3E | 1E | 2J | 3J | 4J | 3J | 4J | 3J | 3J | 4J | 3J | 3J |
| H | 3J | 4J | 3J | 4E | 3E | 1E | 2J | 3J | 3J | 3J | 3J | 3J | 2J | 4J | 4J | 3J |
| M | 3E | 3J | 3J | 2J | 2J | 3J | 4J | 3J | 3J | 3J | 3J | 3J | 3J | 3J | 3J | 3J |
| N3 | 4E | 4J | 4J | 4E | 4E | 1E | 1J | 2J | 1E | 3J | 2J | 2J | 2J | 4J | 4J | 3J |
| F | 3E | 4J | 3E | 4E | 2E | 2J | 2J | 3J | 3J | 2J | 2J | 3J | 3J | 3J | 4J | 3J |
| I | 2J | 2J | 2J | 2J | 2J | 3J | 2J | 4J | 4J | 4J | 4J | 4J | 3J | 2J | 3J | 3J |
| L | 3J | 3J | 3J | 2E | 2E | 3J | 4J | 3J | 2J | 2J | 2J | 3J | 2J | 3J | 3J | 3J |
| J | 3J | 5E | 2J | 3E | 2E | 1E | 2J | 2J | 2E | 2J | 2J | 3J | 3J | 2E | 4J | 4J |

### Justification for the finalists' key scores

| ID | Criterion | Score | Why |
|---|---|---|---|
| B | C6 Competitive room | 4E | Meta Business Agent is inbound-only, has no draft-approve mode and is Meta-only (COMP §3.1, §5). Messenger outbound tags removed (INT §1). Inbox vendors cost US$79–299/mo for teams (COMP §3.4–3.5) |
| B | C5 WTP | 3E | PH clinics pay ₱1,990–3,900/mo for an adjacent AI product (PH §4). No evidence yet for *follow-up* specifically |
| B | C9 Integration ease | 3E | Email forwarding, SMS (Semaphore ₱0.56/text) and web chat are ungated. Messenger for other businesses' Pages needs Business Verification + App Review + tech-provider access, est. 2–6 weeks. Follow-up beyond 24 h needs Utility templates or the human-agent tag (INT) |
| B2 | C6 | 2E | GoHighLevel AI Employee ($50–97/sub-account) and Jobber bundle follow-ups (BIZ [25][28]) |
| B2 | C7 Reachability | 2J | No network abroad; paid acquisition; Stripe unavailable to PH entities (INT) |
| C | C3 Economic value | 5E | 56% of US small businesses owed money, average ~$17.5k (BIZ [38]) vs $69–259/mo tools |
| C | C7 | 2J | International-only buyer; bookkeeper channel plausible but untested |
| A | C6 | 2E | Meta books appointments in chat with calendar management on its roadmap; HeyDenta and SeriousMD in PH (COMP §3.11, PH §8A) |
| A | C11 | 2E | Health data is sensitive personal information; NPC registration likely (PH §5) |

## 3. Sensitivity analysis

**Across weight profiles:** B is #1 under all three. The top four (B, B2, C, A) are the top four under all three. E (voice) jumps to #5 only under Revenue-first, because its demand evidence is strong, but it carries a disqualifying C6.

**Across judgement uncertainty:** every J score was randomly moved by −1, 0 or +1 (20,000 trials, Balanced weights):

| ID | Top-1 share | Top-3 share |
|---|---|---|
| B | 46% | 88% |
| B2 | 25% | 72% |
| C | 14% | 66% |
| A | 13% | 60% |
| G | 1% | 10% |
| every other candidate | ≈0% | ≤ 2% |

**What would flip the winner** (single changes, Balanced weights):
- **Meta adds outbound follow-up to Business Agent:** B's C6 drops 4→2, B falls to 70.8 and B2 (72.6) leads, with C (71.8) second. *This is the biggest single risk*; watch Meta's announcements.
- **PH owners won't pay ≥ ₱1,500/mo:** B's C5 drops 3→2 (−1.8) and B2 leads. This is why B2 is the planned pivot.
- **Owner has a warm bookkeeper network abroad:** C's C7 rises 2→4 (+3.2) and C leads.

**The margins between the top four are small (≈2.6 points).** The research separates the top four from the rest decisively. It does *not* decisively separate the four from each other. Validation (Experiment 1–4 in the [validation plan](customer-validation-plan.md)) has to do that.

## 4. Top candidates compared

| | **B**: PH follow-up | **B2**: US/AU follow-up | **C**: AR collections | **A**: PH clinic booking |
|---|---|---|---|---|
| Buyer | Owner of a 1–20 staff service SME | Owner of a US/AU trades/services SME | Bookkeeper or SME owner using Xero/QBO | Clinic owner/dentist |
| Price hypothesis | ₱1,500–4,000/mo | US$49–99/mo | US$69–259/mo (anchored by Chaser/Paidnice) | ₱1,990–3,900/mo (HeyDenta anchor) |
| Channels | Email forward, SMS, web chat, then Messenger | SMS (US 10DLC registration needed — not researched), email | Email (+ SMS) via accounting data | Messenger, SMS reminders |
| Biggest risk | Meta ships follow-up; PH WTP | Bundled by GoHighLevel and Jobber; reach | Distribution from PH; Xero/QBO app review | Meta calendar roadmap; health-data compliance |
| Builder's edge | Taglish, local presence, Messenger know-how | Lower cost base | Low (no local edge) | Taglish, local presence |
| Shares core with B? | — | Same product, different channels | ~60%: scheduled follow-ups, approvals, drafting | ~70%: inquiry handling, reminders, approvals |

## 5. Major assumptions and unknowns

1. **[Unvalidated]** PH service SMEs lose meaningful revenue to missed follow-up. Only US data exists (BIZ [4]).
2. **[Unvalidated]** Owners will accept a one-tap approval step, rather than wanting full autonomy or distrusting drafts.
3. **[Unknown]** Whether an owner-approved, AI-drafted Messenger reply sent via our app qualifies for the HUMAN_AGENT tag. Meta's wording is about "human agent" responses; treat it as **no** until Meta confirms. Until then, Messenger follow-ups happen inside the 24 h window, through Utility templates for transactional reminders, or by the owner sending from their own inbox.
4. **[Unknown]** Meta Business Agent's PH free allowance and Taglish quality.
5. **[Unknown]** Current-model quality on real Taglish/Bisaya messages. Needs an in-house eval set, which the concierge test produces.
6. **[Estimate]** PH reachable revenue for clinics is roughly ₱47M–374M/yr (PH §11): a profitable small business, not venture scale, without international expansion.
7. **[Fact, H]** Stripe doesn't support PH entities. Billing goes through PayMongo/Xendit, or Paddle as merchant of record for USD (INT).
8. **[Fact, M]** Viber bots cost ~EUR 100/bot/month, which is uneconomic at micro-SME prices. Viber is deferred unless a customer segment pays for it (INT).
9. **[Fact, M]** NTC orders block clickable links in PH SMS, so SMS reminders must be plain text (INT).

## 6. Recommendation (preliminary, pending owner decisions)

**First product hypothesis:** an **owner-approved follow-through agent for Philippine service SMEs** (B). It turns an inbox of inquiries and sent quotes into a daily queue of AI-drafted follow-ups that the owner approves with one tap, sent through whatever channel is allowed. Vertical is chosen by evidence from validation, among clinics, travel/tours, home services and small agencies.

**Initial customer segment:** decided by the mystery-shopper audit and interviews, not now.

**Why not the others first:**
- B2 is the pivot if PH WTP fails.
- C is the pivot if follow-up pain doesn't show up, or if the owner has a bookkeeper network.
- A is absorbed as a candidate vertical.

**Deferred or rejected:** see the [landscape](opportunity-landscape.md#inventory).

**Low-cost validation for each finalist:** see the [validation plan](customer-validation-plan.md).

## 7. Update 2026-10-18: owner constraints applied

The owner stated three constraints after the first ranking: **no business network or interviews**, **no paid-ads budget**, and **cash spend ≤ ~₱600/month** until a business pays. New research is in [self-serve-distribution.md](self-serve-distribution.md). Re-scored in `scoring/score.py`:

| Change | Why |
|---|---|
| B C7 3J→2J, A C7 3J→2J | The reachability score had assumed warm introductions |
| C C7 2J→3J | Xero App Store gives a no-call trial-and-buy path (store cap AUD 245/mo; ~3 real customers needed before certification) |
| C C6 3E→2E | Solido already lists the same "draft → approve → send from your mailbox" concept; Trove from GBP 50/mo; Intuit's native Payments Agent in QBO (US) |

**New ranking (Balanced):** B2 72.6 · B 72.4 · C 71.8 · A 69.8.

**Sensitivity (20,000 trials):** top-3 share is B 80% · B2 79% · C 75%; top-1 share is B 33% · B2 35% · C 22%.

**The scores are now a three-way tie.** The constraints decide it instead:

| Constraint | B (PH follow-up) | B2 (US/AU follow-up) | C (AR on Xero/QBO) |
|---|---|---|---|
| ₱0 acquisition channel exists | **Yes**: public PH business FB groups, Taglish posts | Weak: cold English outreach abroad with no ads and no local edge | Only after ~3 off-store customers; then AUD 245/mo listing |
| Fits ≤ ₱600/mo budget | **Yes** | Mostly (US SMS registration costs not researched) | **No**: the listing tier is ~₱9,000+/mo |
| Billing without Stripe | GCash/PayMongo works locally | Needs a merchant of record (e.g. Paddle) | Same as B2; Xero no longer bills for apps |
| Builder's edge | Taglish, local presence | None | None |

**Conclusion:** under the owner's constraints, **B remains the lead**. C fails the budget constraint, and B2 has no free acquisition channel. Both stay as later options once revenue exists.

## 8. Update 2026-10-18: new ideas and the follow-up competitor check

Sources: [follow-up-competitors.md](follow-up-competitors.md), [new-ideas-philippines.md](new-ideas-philippines.md), [new-ideas-marketplaces.md](new-ideas-marketplaces.md).

**What changed:**
- **B: C6 4E→3E.** Asikaso AI (PH) sells Taglish AI replies with up to 3 AI follow-ups per conversation at ₱1,499/mo. Four more small PH tools sell at ₱799–2,500/mo, and draft-then-approve is common (GoHighLevel Suggestive mode, Lindy). No one does B's full loop, but that is a feature set, not a moat.
- **Ten new candidates scored.** For S1, C4 was lowered 5E→4E because Taxumo's user count is a vendor claim.

**Ranking (Balanced), top 8:**

| # | ID | Candidate | Balanced |
|---|---|---|---|
| 1 | S1 | Seller tax desk for PH online sellers | 74.4 |
| 2 | B2 | Follow-up, US/AU vertical | 72.6 |
| 3 | C | AR collections on Xero/QBO | 71.8 |
| 4 | B | Follow-up, PH service SMEs | 70.8 |
| 5 | MK2 | Approval-first support reply drafter (Shopify) | 70.6 |
| 6 | A | Clinic booking & reminders (PH) | 69.8 |
| 7 | S4 | PH marketplace inbox agent | 67.6 (disqualifier C6) |
| 8 | MK1 | Shopify order-exception agent | 66.6 |

**Sensitivity (top-3 share):** S1 78% · B2 59% · C 49% · B 35% · MK2 34% · A 21%.

**Constraint fit** (no network, no ads, ≤ ₱600/mo):

| | S1 tax desk | B follow-up | MK2 Shopify |
|---|---|---|---|
| Free channel | Taglish videos and SEO, seller FB groups, free threshold tracker | PH business FB groups | Shopify App Store search (buyer intent), but 0-review newcomers struggle |
| Cost to start | ~₱0 | ~₱0 | US$19 once |
| Getting paid | GCash / PayMongo | GCash / PayMongo | Shopify billing → Hyperwallet; **PH payout unconfirmed** |
| Builder's edge | Taglish, PH tax context | Taglish | None |
| Biggest risk | Tax liability; possible BIR accreditation; Taxumo/Juan may already import marketplace reports | Asikaso and others; Meta adds follow-up | Crowded (C6 2E); discovery |
| Judgement share | 11/16 | 10/16 | 12/16 |

B2 and C remain excluded by the constraints (§7).

## 9. Update 2026-10-18: Philippine industry sweep

Sources: [ph-industries-services.md](ph-industries-services.md), [ph-industries-sectors.md](ph-industries-sectors.md). Eleven candidates were added to `scoring/score.py`.

**Ranking (Balanced), top 8:**

| # | ID | Candidate | Balanced |
|---|---|---|---|
| 1 | P1 | Bookkeeper document, 2307 & BIR-deadline chaser | 78.6 |
| 2 | S1 | Seller tax desk | 74.4 |
| 3 | B2 | Follow-up, US/AU vertical (fails constraints) | 72.6 |
| 4 | C | AR collections on Xero/QBO (fails constraints) | 71.8 |
| 5 | CON1 | Progress-billing & collections desk for small contractors | 71.8 |
| 6 | EDU1 | Tuition & promissory-note follow-through for small schools | 71.2 |
| 7 | B | Follow-up, PH service SMEs | 70.8 |
| 8 | MK2 | Shopify approval-first reply drafter | 70.6 |

**Sensitivity (20,000 trials, judgement scores ±1):**

| ID | Top-1 share | Top-3 share |
|---|---|---|
| P1 | 71% | 96% |
| S1 | 13% | 55% |
| B2 | 4% | 31% |
| CON1 | 3% | 24% |

**Scorer-bias caveat.** Each research agent scored its own finds, so calibration may differ between agents. P1's lead of 4.2 over S1 is larger than one-point judgement noise, but P1 also has more E tags than most candidates. Treat P1 as the lead and S1 as a close second. They share most of the build: BIR calendar, 2307 extraction, Taglish drafting, approval-gated chasers.

**Unverified kill-risks for P1** (check at the start of the spec):
1. Whether Built for Bookkeepers (₱999/mo) or Juan already send client document requests.
2. That a tool which only chases documents and tracks deadlines (never computes or files returns) needs no BIR CAS/eTSP registration. This is inferred, not confirmed.
3. Real bookkeeper reach. r/AccountingPH member counts are undated snapshots.

## 10. Update 2026-10-20: AI add-ons for ERP, CRM, HR and other software

The owner asked whether to build an AI agent for ERP, CRM, HR or other business software. Three sweeps answered: [`ai-addons-hr.md`](ai-addons-hr.md), [`ai-addons-erp.md`](ai-addons-erp.md), [`ai-addons-crm-saas.md`](ai-addons-crm-saas.md). Their 16 candidates are now in `scoring/score.py`; the ERP agent checked its arithmetic against P1 = 78.6 and C = 71.8. The same caveat as §8 and §9 applies: each agent scored its own finds.

| ID | Candidate | Balanced | Note |
|---|---|---|---|
| HR2 | Payroll-input & remittance-deadline chaser for bookkeepers | 72.6 | Same buyer and loop as P1; a P1 module |
| ERP1 | 2307 receivable desk (request, read, match received 2307s) | 71.6 | Inside P1's scope; a P1 module |
| MKT4 | Freshdesk Growth-plan reply drafter | 66.4 | Pricing-tier gap Freshworks can close |
| MKT2 | HubSpot Free/Starter drafter | 64.8 | HubSpot's agent marketplace is HubSpot-only |
| MKT5 | monday.com stale-item chaser | 63.6 | |
| ERP2 | Odoo supplier & PO follow-up | 63.4 | Odoo 19 ships supplier reminders free |
| MKT3 | Zoho CRM Taglish follow-up | 63.2 | |
| ERP4 | ERPNext bill capture with BIR codes | 63.0 | Frappe apps must be open source |
| MKT1 | Shopify PH COD & courier desk | 62.2 | |
| HR1, HR4, ERP3, ERP5, HR5, HR3, HR6 | (see source files) | 60.4 to 51.0 | HR5, ERP5 C6 = 1; HR6 C11 = 1; ERP3 C9 = 1 |

**Result: the ranking does not change.** P1 stays first under all three weight profiles (78.6 / 78.8 / 79.0). The Monte Carlo run with the new candidates has P1 top-1 in 68% of trials and top-3 in 94%.

**Why the software-add-on route scores low under our constraints:**

- **The vendors ship the obvious agents themselves**, free or bundled: Sprout Sidekick (PH HR and payroll, Tagalog), Odoo 19 AI agents, Business Central agents, Xero follow-ups, HubSpot Breeze, Freddy, Zia. Remaining gaps are mostly pricing-tier gaps the vendor can close.
- **The marketplaces are gated or don't fit a PH solo developer.** BambooHR requires ≥ 100 customers; Personio 50+ shared customers; Gusto a security review; HubSpot 3 unrelated installs; Salesforce a US$2,550 review; Zendesk and Notion need Stripe, which doesn't onboard PH; Odoo sells one-off licences at a 30% cut.
- **Little evidence of PH demand for add-ons:** the two Odoo BIR-report apps have 3 and 1 purchases in total; Zoho's own apps took 97% of installs (2022 third-party analysis).
- **HR data is sensitive personal information** under RA 10173.

**What this sweep adds to P1:** two expansion modules on the same buyer and engine: payroll-input and remittance chasing (HR2) and received-2307 matching (ERP1), plus optional read-only import from Xero, QuickBooks or Odoo. Open checks: whether PH bookkeepers routinely run client payroll, and whether Juan, Built for Bookkeepers or QNE already track received 2307s.

## 11. Update 2026-10-20: Harvey market check

Source: [`harvey-market.md`](harvey-market.md). The owner asked about recreating Harvey in Rust and competing with it.

| ID | Candidate | Balanced | Note |
|---|---|---|---|
| P-R | Rust Harvey: open-source approval-first outbound engine with a paid hosted tier (the best sub-variant) | 62.8 | **Disqualifying C6 = 1.** Harvey is free and growing (1,776 stars on 2026-10-20); Apollo has a free plan; Instantly already offers human-approved AI replies. Users ask for providers, discovery and warmup, not Rust |
| P-SELF | Our own outbound engine, used to find customers | 62.4 (not comparable) | An internal tool: C3–C6 and C14 assume a paying market, so the score is for reference only |

**Result: no change to the ranking.** P-R replaces the judgement-only P (60.6) as the evidenced version of Harvey's domain.

**What carries forward:** P-SELF is the same draft → approve → send → stop-on-reply → track loop as P1. So our own written outreach becomes P1's engine's first internal use once the core exists, at 20–40 messages a week, with no discovery, scraping or warmup. Before then, outreach is done by hand. Constraints on it: Gmail/Workspace and Anthropic policies ban facilitating spam; a documented legitimate-interest assessment under RA 10173; and Anthropic's terms bar routing customers' requests through consumer Claude subscriptions, so any hosted product uses API keys.

## 12. Update 2026-10-20: P1 kill-checks

Source: [`p1-kill-checks.md`](p1-kill-checks.md). Five checks; none kills P1.

| Check | Verdict | Score change |
|---|---|---|
| Incumbent overlap | WARN: Built for Bookkeepers (₱999/mo) already gives clients a mobile upload page; Karbon, TaxDome, Financial Cents, Content Snare, DocChase and BilagPilot do requests and reminders. None found combines a per-client BIR calendar, a received-2307 ledger and approved Taglish drafts | C6 4E → 3E |
| BIR CAS/CBA and eTSP | PASS, from the BIR's copies of RMO 9-2021 and RMC 64-2021, as long as P1 makes no books, BIR-format reports or filings and doesn't store books | — |
| Reachability | PASS (r/AccountingPH ~80k, r/taxPH ~161k, low confidence; PICPA 34k in 84 chapters) | — |
| Data privacy (RA 10173) | WARN: P1 is a processor or sub-processor of sensitive personal information (tax returns, likely TINs); outsourcing-contract terms mandatory; NPC registration at 1,000 data subjects | C11 3J → 2E |
| Sending channels | PASS with a design change: telcos block SMS links (NTC 2023), so upload links go by email or the bookkeeper's own Viber; SMS is a link-free nudge | — |

**New P1 string:** `4E 4E 4J 4E 4E 3E 3E 4J 4E 4J 2E 4J 4J 5E 4J 4J`, giving **76.0 / 76.4 / 76.8** (Balanced / Revenue-first / Builder-first). P1 stays first in all three profiles. In the Monte Carlo run, P1 is top-1 in 36% of trials and top-3 in 75%; S1 is top-1 in 22%.

**Caveat on the comparison:** P1's score fell because it was checked. S1 (74.4) has not had kill-checks; its open risks are BIR accreditation (a tool that computes sellers' tax is closer to CAS than a chaser) and overlap with Taxumo and Juan. A verified 76.0 is a firmer number than an unverified 74.4.

**Hard limits for the P1 spec** (from the checks): no SAWT or other BIR-format exports; no recording or issuing of transactions or 2307s, so the 2307 module stops at matching; no long-term storage of client records; terms plus a data processing addendum listing sub-processors; retention and deletion rules; links by email or Viber only.
