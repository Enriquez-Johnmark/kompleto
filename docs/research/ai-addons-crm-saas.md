# AI add-ons for CRM, helpdesk, e-commerce and work-management marketplaces

Status: **research, 2026-10-20.** Question from the owner: "How about an AI agent or AI automation for ERP, CRM, HR, or any other type of software — what would we build?" This file covers the CRM / helpdesk / e-commerce / work-management slice: which software marketplace gives a solo PH developer organic, self-serve customers for an AI add-on, and which add-on has an evidenced gap.

Builds on (does not redo): [new-ideas-marketplaces.md](new-ideas-marketplaces.md) (Shopify, Google Workspace, Slack, Atlassian already covered; MK1–MK5), [competitor-analysis.md](competitor-analysis.md) §3.7–3.10, [follow-up-competitors.md](follow-up-competitors.md). Scores use [scoring-method.md](scoring-method.md). Candidate IDs use the MKT prefix.

**Method limits.** Four parallel research sub-agents were started and all died at a usage limit before reporting; their topics were then covered directly with web search and direct page fetches, so depth is uneven. HubSpot, Shopify and Pipedrive marketplace search pages render client-side and could not be read, so install counts for small AI apps there are mostly **not found**. Zendesk and Freshworks app-review complaints, and idea-forum vote counts, were not collected.

Labels: **[P]** primary (official docs, terms, marketplace listing), **[V]** vendor claim, **[3P]** third party, **[F]** forum / community, **[S]** search snippet only. Confidence H/M/L. All sources accessed 2026-10-20 unless stated.

## Key conclusions

1. **Payment rails, not demand, decide which marketplaces are open to a PH solo developer.** (Facts, H/M.)
   - **Ruled out:** Salesforce AppExchange (US$2,550 security review + US$150/yr listing for paid apps [13]); Zendesk paid apps and Notion Marketplace (both run on the developer's own Stripe account [17][21], and Stripe does not onboard PH businesses); ClickUp (no third-party marketplace found [23]).
   - **Bill-it-yourself (Lemon Squeezy/Paddle needed):** HubSpot [1], Pipedrive [7][8], Airtable [22], Slack, Google Workspace.
   - **Native billing with a plausible PH payout:** Zoho (wire or PayPal, monthly, ≥US$100 [11]); monday.com (bills for you, 0% share to US$200k lifetime [18][19], payout method not found); Freshworks (80:20 [15], payout method not found); Shopify (0% to US$1M, Hyperwallet PH still unconfirmed [24]).
2. **No CRM/helpdesk/work-management marketplace showed evidence of organic traction for small apps that beats Shopify.** Zoho's installs were 97% Zoho's own apps (2022) [12]; HubSpot needs 3 unaffiliated active installs *before* listing [1], which a founder with no network has to earn off-platform; monday's traction evidence is vendor claims [20]. Install counts for small AI apps on HubSpot/Pipedrive: not found.
3. **Platform owners now ship the obvious agents, and HubSpot closes its agent marketplace to third parties.** HubSpot's Agent Marketplace "only lists HubSpot's pre-built agents" [4]; Agent Builder is in Starter [5]; monday agents, Notion Agent, Freddy AI Agent and Zia agents are all native (see table). A third-party "AI agent for CRM X" competes with a bundled feature. The remaining gaps are **tier gaps** (e.g., Freddy Copilot drafting only on Freshdesk Pro/Enterprise at US$29/agent [36]; HubSpot Customer Agent only on Professional/Enterprise at US$0.50/resolution [35]) that the vendor can close at will.
4. **The PH-local gap on Shopify is real but small.** J&T's own PH Shopify app has 1.2 stars from 4 reviews [26]; Shopify has no native PH courier integration [25] (V). But PH-specific Shopify apps barely accumulate reviews (PayMongo: 2 [28]) and StoreLeads counts hundreds, not tens of thousands, of stores per PH city [29]. PH sellers sit mostly on Shopee/Lazada/TikTok/FB (see [new-ideas-philippines.md](new-ideas-philippines.md)).
5. **Recommendation (research view, not owner-approved):** none of the MKT candidates beats MK2 (70.6) or P1 (78.6) under Balanced; the best, MKT4 (Freshdesk Growth-tier approval-first drafter), scores 66.4. This slice does **not** change the current ranking. If the owner wants a marketplace route, Shopify (MK2/MK1) remains the one to test; Zoho and monday are the only other marketplaces worth a payout check because they bill natively.

## Marketplace comparison

| Marketplace | Solo PH dev can sell? | Listing gate / review time | Fees / share | Native recurring billing | Organic-traction evidence | Verdict |
|---|---|---|---|---|---|---|
| **Shopify** (prior doc) | Yes; payout via Hyperwallet, **PH unconfirmed** [24] | No published SLA | US$19 once; 0% to US$1M | Yes | Commslayer 237 reviews in ~2.5 yrs (prior doc) | Best channel (unchanged) |
| **HubSpot** | Yes (no entity rule found) | **≥3 unaffiliated active installs before listing**; initial review ≤10 business days [1] | None found | **No** (own billing) | Listings show installs (Aircall 16K [2]); small AI apps: not found | Weak: chicken-and-egg listing gate |
| **Pipedrive** | Not stated | Up to 21 business days [8]; forum: 2 wks–2 mo [9] | Not found | Not found (own billing) | "over 100,000 companies" see it [7] (V) | Weak |
| **Zoho (CRM/Desk)** | Likely: wire or PayPal on request [11] | Not found | Share % not found | **Yes**, monthly payout ≥US$100 [11] | 97% of installs are Zoho apps (2022) [12] | Payout-friendly, discovery weak |
| **Salesforce AppExchange** | Fee-blocked | Security review | US$2,550 + US$150/yr [13]; 15% share [14] | Yes (Checkout) | n/a | **Ruled out** |
| **Freshworks** | Unclear (payout not found) | ~1 week (2023) [15] | 80:20 [15] | Yes (per account/agent) | Not found | Possible; check payout |
| **Zendesk** | **Blocked**: developer's Stripe [17] | Not found | Not found | Via Stripe | Not found | Ruled out for paid |
| **monday.com** | Unclear (payout not found) | Not found | 0% to US$200k lifetime, then 15% [18][19] | **Yes** | 1.6M installs, 49% of accounts (V) [20] | Possible; check payout |
| **Notion** | **Likely blocked** (Stripe onboarding) [21] | Waitlist | 10% + US$0.40 [21] | Templates only | Not found | No (templates, not agents) |
| **Airtable** | Yes, but no billing | Reviewed [22] | n/a | **No** | Not found | No |
| **ClickUp** | No marketplace found [23] | — | — | — | — | No |
| **Google Workspace / Slack** (prior doc) | Yes / Yes | CASA for Gmail; Slack ≥10 active workspaces | — | No / No | GPT for Sheets 7M+ (prior doc) | Secondary |

**Per-marketplace notes:**

- **HubSpot App Marketplace.** Listing needs "at least three active, unique installs" (production accounts not affiliated with the developer, activity in the last 30 days); initial review "within 10 business days", full review up to 60 days after feedback; one app in review at a time [1] (P,H). No listing fee and no billing/revenue-share terms on the requirements page; developer bills on their own [1] (P,H for absence on that page). Listings show install counts: Aircall 16K installs, 318 reviews, 4.3 [2] (P,H); HubSpot-built AgentAI ~2K installs [3] (S,M). The **Agent Marketplace lists only HubSpot's pre-built agents**; third parties can only contribute "agent tools" to HubSpot's agents [4][6] (P,H).
- **Pipedrive Marketplace.** Visible to "over 100,000 companies" [7] (V,H). Public-app review "may take up to 21 business days" [8] (P,H); developer forum reports waits of 2 weeks to 2 months [9] (F,M). Native billing or revenue share: **not found** (developer bills on their own) [7][8].
- **Zoho Marketplace.** Developer Console free to build and sell [10] (P,H). Zoho collects payment and pays commission monthly by wire transfer (~15 working days) or PayPal on request; statements only above US$100 [11] (P,H). **Revenue-share percentage not found.** A 2022 third-party teardown: ~1.1K apps, ~1.2M installs, "97% of all installs belong to Zoho apps", ~32% of apps paid; largest indie paid developer (Ulgebra) has 27 apps and 9 staff [12] (3P,M, 2022).
- **Salesforce AppExchange.** Security review US$2,550 plus US$150/yr listing fee for paid apps (free apps exempt), per Salesforce packaging guides v224–236 [13] (P,M: older guide versions); revenue share 15% of net revenue via AppExchange Checkout [14] (P,M). **The up-front fee is roughly 250× the ₱600 monthly budget (J, at ~₱57/US$); ruled out.**
- **Freshworks Marketplace.** Paid apps on an "80:20 revenue-sharing model", priced per account or per agent; review "ideally takes about a week" [15] (F/official staff post, 2023-04-11, M). A 2023 incentive offered 100% share for 6–12 months [16] (F,L: dated). Payout method/countries: **not found**.
- **Zendesk Marketplace.** Paid apps are charged through Zendesk's Stripe app; developer connects a Stripe account; per-account or per-agent monthly plans, trials 1–90 days [17] (P,H). Revenue share: **not found**. **Stripe does not onboard PH businesses** [new-ideas-marketplaces.md, src 38], so paid listing is effectively blocked for a PH sole developer (J, M).
- **monday.com marketplace.** monday bills, handles renewals and tax, pays out; 0% share until US$200,000 lifetime app revenue, then 85/15 [18][19] (P,H). Vendor claims 1.6M app installs and 49% of accounts have installed an app [20] (V,M). Payout method/countries and review SLA: **not found**. Vendor case study: Pioneera founder reached ~US$30K MRR "within a few months" [20] (V,L).
- **Notion Marketplace.** Templates only (not agents); 10% + US$0.40 per sale (some locale pages say 8%) and biweekly payouts ≥ US$20; selling needs waitlist approval **and Stripe onboarding** [21] (P,M). PH eligibility for that Stripe flow: **not found** (likely blocked, J).
- **Airtable Marketplace.** Reviewed extensions marketplace, but Airtable "does not want transactions run through its platform"; developers monetise off-platform with API keys [22] (P,H).
- **ClickUp.** No third-party app marketplace found; a user feature request asks for one [23] (F,M).
- **Shopify, Google Workspace, Slack, Atlassian:** see [new-ideas-marketplaces.md](new-ideas-marketplaces.md) §1 (Shopify US$19 once, 0% share to US$1M; Slack needs ≥10 active workspaces before listing and has no billing; Google Workspace has no billing and CASA for Gmail scopes). **Shopify Hyperwallet payout to PH is still unconfirmed** after two more searches [24] (P,M).

## Built-in AI per platform

| Platform | What ships | Tier / price | Source |
|---|---|---|---|
| HubSpot | Breeze Assistant; Customer, Prospecting and Data agents; Agent Builder; AI email writer. Agent Marketplace lists only HubSpot-built agents | Agent Hub and Agent Builder on Starter, Pro, Enterprise [5]; **Customer Agent needs Professional/Enterprise, US$0.50 per resolution** (resolution = not handed to a human for 72h), channels incl. email, chat, WhatsApp, Messenger [35] | [4][5][35] (P/V,H) |
| Pipedrive | AI Sales Assistant (beta, all plans), AI email drafting, Pulse (closed beta), MCP server (2026-06-30) | Tier placement conflicts between third parties | [37] (3P,L) |
| Zoho | Zia, Zia Agent Studio, Zia LLM | Agent Studio tier conflicts (Standard vs Ultimate) between third parties | [38] (3P,L) |
| Salesforce | Agentforce | Not researched (marketplace ruled out by fee) | — |
| Freshworks | Freddy AI Agent (Growth and up, first 500 sessions free once, then US$49/100 sessions); **Freddy AI Copilot only on Pro/Enterprise at US$29/agent/mo**; Insights on Enterprise | Growth US$19, Pro US$55, Enterprise US$89 per agent/mo annual | [36] (P,H) |
| Zendesk | AI agents, Copilot | Automated-resolution price not published officially; 3P US$1.50–2.00 (prior doc §3.7) | [competitor-analysis.md] (3P,M) |
| Shopify | Sidekick (approval before applying changes; app extensions since June 2026), Inbox AI | Free (prior doc) | [new-ideas-marketplaces.md] (P,H) |
| monday.com | monday AI, monday agents, sidekick, vibe | Seats + AI credits; agents consume credits from 2026-06-08 on Pro and below; ~10–250+ credits per run [39] | [39] (P,M) |
| Notion | Notion AI + Notion Agent in Business (US$20/member/mo annual); Custom Agents on credits (~US$10/1,000) | 3P, multiple sources agree on Business bundling | [40] (3P,M) |
| Google Workspace | Gemini | Not researched this session | — |

**Implication for C6:** every CRM/helpdesk/work platform here ships a native agent. A third-party add-on only has room where the vendor's AI is **tier-gated** (Freshdesk Copilot, HubSpot Customer Agent), **locale-blind** (PH couriers, Viber/SMS, Taglish), or **autonomous-only** where buyers want approval. Each of these is closable by the vendor (J).

## Evidenced gaps

**G1. Philippine couriers are not integrated into Shopify, and the courier's own app is broken.**
- A PH e-commerce vendor (self-interested) says "Shopify has no native integration with J&T Express, LBC, Lalamove, GoGo Xpress, Grab Express, Ninja Van" and "You cannot book a J&T pickup from your Shopify admin" (2026-05-27) [25] (V,M).
- J&T's own "J&T Philippines" Shopify app (launched 2025-08-19, free) has **1.2 stars from 4 reviews**: "doesn't want to sync in store", "not working in my store", "the dropdown tabs for province/city/barangay are too small" [26][27] (P,H).
- **But the PH Shopify base is small and rarely reviews.** PayMongo's Shopify payments app has only 2 reviews [28] (P,H). StoreLeads counts 821 live Shopify stores in Makati (2026-10-16), ~629 in Pasig (2026-06-19); no national total was retrievable [29][30] (3P,M). Top apps in Makati stores are global ones (Judge.me 139 stores, Shopify Inbox 130, Klaviyo 91) [29] (3P,M). A PH-only Shopify app therefore has a ceiling of perhaps a few thousand stores (J), most of them selling via Shopee/Lazada/TikTok/FB as well.

**G2. COD verification is a paid, cheap, crowded niche, and PH figures are vendor claims.**
- PH COD share: figures conflict between vendor blogs (78%, 42%, "86%" from a 2022 Statista citation by Ninja Van) [31][32] (V,L). Failed-COD rate "15% in the Philippines" and RTO "20–40%" are vendor claims [31] (V,L). BSP-reported digital share of retail payment volume 64.7% in 2025 (via search summary) [31] (S,L).
- Competitor price anchor: "Level – Verify COD & reduce RTO" (launched 2025-12-26): free for 50 WhatsApp verifications/month, US$4.99 Starter, US$9.99 Pro; **0 reviews**; WhatsApp only, no Viber/SMS; no PH mention [33] (P,H). Fast2SMS has a free COD WhatsApp/SMS alert app (India) [34] (V,M).
- No PH-focused (Viber/SMS/Taglish) COD-confirmation Shopify app was found (absence, M). Note PH buyers mostly use Messenger and Viber rather than WhatsApp (see [market-philippines.md](market-philippines.md)); Messenger outbound to a buyer who never messaged the page is not possible.

**G3. Tier gaps in helpdesk AI.**
- Freshdesk Growth (US$19/agent) includes the customer-facing Freddy AI Agent but **not** Copilot (agent-side drafting), which is a US$29/agent add-on on Pro (US$55) and Enterprise only [36] (P,H). A small team on Growth that wants approve-before-send drafts has no first-party option without a ~3× plan jump (J on the framing, H on prices).
- HubSpot Customer Agent requires Professional/Enterprise and bills US$0.50 per resolution [35] (P,H). Starter users get Breeze Assistant and Agent Builder [5]; whether Starter can draft inbox replies natively: **not verified**.
- A HubSpot Ideas thread asks to allow multiple Breeze Customer Agents per account (HubSpot limits accounts to one) [41] (F,M; vote count not collected).

**G4. PH messaging channels are absent from CRM marketplaces (absence finding, M/L).**
- Zoho CRM has SMS extensions for Twilio, Nexmo, SMS Alert, BulkGate and others, but **no Viber or Semaphore extension was found** [42] (S,M). Promotexter, a Viber business-messaging partner in the PH, advertises "CRM integrations" without naming Zoho [43] (V,L).
- Evidence that PH SMEs use Zoho, HubSpot or Pipedrive in volume: **not found**.

**Not collected (gaps stay gaps):** high-vote idea threads on Pipedrive, Zoho, Zendesk, Freshworks and monday; 1–2★ reviews of third-party AI apps on HubSpot/Zoho/Freshworks/monday.

## Candidates with score strings

Order C1..C16; weighted totals computed with the `scoring/score.py` weights (not added to that file). Reference: MK2 70.6, P1 78.6 (Balanced).

| ID | Candidate | C1..C16 | Balanced | Revenue-first | Builder-first |
|---|---|---|---|---|---|
| MKT4 | Freshdesk Growth-tier approval-first reply drafter | `3J 4J 3J 4E 3E 3E 2J 4J 3E 4J 3J 4J 3J 4J 3J 4J` | 66.4 | 65.4 | 68.6 |
| MKT2 | HubSpot Free/Starter approval-first inbox & stale-deal drafter | `3J 4J 3J 4E 3E 2E 2E 4J 3E 4J 3J 4J 3J 4J 3J 4J` | 64.8 | 63.8 | 67.4 |
| MKT5 | monday.com stale-item chaser agent | `2J 4J 2J 3E 3J 2E 3E 4J 3J 4J 4J 4J 4J 4J 3J 4J` | 63.6 | 62.2 | 66.8 |
| MKT3 | Zoho CRM Viber/SMS follow-up drafter for PH/SEA SMEs | `3J 4J 3J 2J 3J 3J 2E 4J 3J 4J 3J 4J 3J 4J 3J 4J` | 63.2 | 61.4 | 66.2 |
| MKT1 | Shopify PH COD-confirmation & courier-exception desk | `4J 4J 3J 3E 2E 3E 2E 3J 2J 4J 3J 3J 3J 4J 4J 4J` | 62.2 | 60.8 | 64.0 |

No candidate has a disqualifying 1 on C5, C6 or C11. All rank below MK2 and P1 under all three profiles.

### MKT4: Freshdesk Growth-tier approval-first reply drafter
- **One line:** for Freshdesk Growth teams, drafts every ticket reply from KB and past tickets; the agent approves, edits or discards; nothing auto-sends.
- **Buyer:** 2–10-agent support teams on Freshdesk Growth (J).
- **Price anchor:** Freddy Copilot US$29/agent/mo, Pro/Enterprise only [36] (P,H). Plausible add-on price US$9–15/agent/mo (J).
- **Competitors:** Freddy Copilot (tier-gated), Freddy AI Agent (autonomous, on Growth) [36]; generic AI reply apps on the Freshworks Marketplace (not counted).
- **Agent loop:** ticket webhook → classify → retrieve KB/past replies (tool calls) → draft with cited sources → agent approves/edits in the ticket sidebar → send via Freshdesk API (idempotent) → track reopen rate.
- **Scores:** C1 3J support lag costs CSAT, not revenue directly · C2 4J many tickets daily · C3 3J saves minutes per ticket · C4 4E Freshworks sells Copilot and AI sessions [36] · C5 3E per-agent pricing allowed, 80:20 [15] · C6 3E Copilot absent on Growth [36], but Freshworks can move it down a tier · C7 2J no small-app traction evidence; payout unknown · C8 4J standard app + Rust backend · C9 3E ~1-week review (2023) [15] · C10 4J drafts are reviewed · C11 3J customer PII in tickets · C12 4J · C13 3J LLM per ticket · C14 4J · C15 3J same core ports to HubSpot/Help Scout, not Zendesk (Stripe) · C16 4J approvals, tools, idempotent sends.

### MKT2: HubSpot Free/Starter approval-first inbox & stale-deal drafter
- **One line:** for HubSpot Free/Starter users, drafts replies to inbox emails and follow-ups for deals idle N days; owner approves in a CRM card.
- **Buyer:** small sales/service teams on HubSpot Free or Starter (US$7–20/seat, competitor-analysis §3.10).
- **Price anchor:** Customer Agent US$0.50/resolution on Pro/Enterprise only [35] (P,H).
- **Competitors:** Breeze Assistant and Agent Builder on Starter [5]; AI email writer; Breeze Prospecting agent; Sinch Breeze texting with approval (V).
- **Agent loop:** scheduled scan of deals/conversations → draft grounded in CRM records → owner approves in a CRM card → send through HubSpot email API (idempotent) → log activity, re-check later.
- **Scores:** C1 3J · C2 4J · C3 3J · C4 4E HubSpot monetises agents per resolution [35] · C5 3E Starter buyers are low-ARPU · C6 2E HubSpot ships Agent Builder on Starter and a native agent stack [5] · C7 2E 3-install listing gate with no network; no billing [1] · C8 4J · C9 3E 10-business-day initial review [1] · C10 4J · C11 3J · C12 4J · C13 3J · C14 4J · C15 3J Pipedrive/Zoho ports · C16 4J.

### MKT5: monday.com stale-item chaser agent
- **One line:** watches boards for overdue/idle items and drafts nudges to owners or external clients; the PM approves each nudge.
- **Buyer:** agencies and ops teams on monday.com (J).
- **Price anchor:** monday AI credits ~US$0.01/credit, agent runs ~10–250+ credits (3P/P) [39]; third-party price for this job: not found.
- **Competitors:** monday agents (native, credit-billed) [39]; monday automations (non-AI reminders).
- **Agent loop:** durable schedule → query board via API → draft nudge with item context → approve in an item view → post update/email → re-check.
- **Scores:** C1 2J late tasks are annoying, rarely costly · C2 4J · C3 2J · C4 3E vendor-claimed app installs and a vendor case study [20] · C5 3J · C6 2E monday agents native [39] · C7 3E native billing, ranking by reviews (V) [20] · C8 4J · C9 3J review SLA not found · C10 4J · C11 4J work data, low sensitivity · C12 4J · C13 4J · C14 4J · C15 3J · C16 4J durable schedules + approvals.

### MKT3: Zoho CRM Viber/SMS follow-up drafter for PH/SEA SMEs
- **One line:** a Zoho CRM/Desk extension that drafts Taglish follow-ups for stale leads/quotes and sends via Viber or PH SMS after approval.
- **Buyer:** PH/SEA SMEs already on Zoho CRM (count not found).
- **Price anchor:** not found for Viber/SMS extensions on Zoho; Zoho CRM itself US$14–52/user/mo annual (3P) [38].
- **Competitors:** Zia (tier unclear) [38]; Twilio/SMS Alert/BulkGate extensions [42]; Ulgebra-type indie extension shops [12].
- **Agent loop:** same as finalist B, but triggered from Zoho records; channel adapters for Viber Business Messages and SMS.
- **Scores:** C1 3J · C2 4J · C3 3J · C4 2J PH Zoho demand not found · C5 3J · C6 3J no Viber/Semaphore extension found [42] · C7 2E 97% of installs Zoho-owned [12] · C8 4J · C9 3J review time not found; Viber needs a partner · C10 4J · C11 3J · C12 4J · C13 3J per-message channel fees · C14 4J · C15 3J · C16 4J.

### MKT1: Shopify PH COD-confirmation & courier-exception desk
- **One line:** for PH Shopify stores, drafts Viber/SMS COD confirmations and courier-exception messages (J&T/LBC/Ninja Van), merchant approves; a PH-localised MK1.
- **Buyer:** PH Shopify merchants with COD orders (J).
- **Price anchor:** Level US$4.99–9.99/mo, 0 reviews [33] (P,H).
- **Competitors:** Level (WhatsApp only) [33]; Fast2SMS (free, India) [34]; J&T's own app (1.2★) [26]; PH platforms such as Prosperna [25].
- **Agent loop:** order webhook → COD-risk rules → draft confirmation in Taglish → merchant approves (or auto within approved template) → send via SMS/Viber → on no reply, propose hold/cancel for approval; poll courier status → draft exception notices.
- **Scores:** C1 4J failed COD costs shipping both ways (no PH primary data) · C2 4J daily · C3 3J · C4 3E paid COD-verification apps exist [33] · C5 2E anchors at US$5–10 [33] · C6 3E no PH-localised app found; J&T app broken [26] · C7 2E PH-only apps get 2–4 reviews [26][28]; small base [29] · C8 3J courier APIs unverified · C9 2J courier and Viber partner access · C10 4J · C11 3J addresses/phones (Data Privacy Act) · C12 3J · C13 3J SMS/Viber fees · C14 4J · C15 4J extends to Shopee/Lazada sellers (S-series) · C16 4J.

**Considered and dropped:** Pipedrive stalled-deal drafter (same as MKT2 with weaker evidence: no billing, 21-day review, AI Sales Assistant native [8][37]); Zendesk drafter (Stripe-blocked [17]); Salesforce anything (fee [13]); Notion/Airtable/ClickUp (no billing or no marketplace [21][22][23]).

## Unresolved questions

1. **Payout to PH** on Shopify (Hyperwallet), monday.com and Freshworks: check each partner dashboard before any build. This is the binding fact for every marketplace route.
2. Zoho Marketplace revenue-share percentage and listing review time: not found.
3. Install/review counts for small AI apps on HubSpot, Pipedrive, Zoho, Freshworks and monday: not found (client-rendered pages).
4. Whether HubSpot Starter's Breeze Assistant already drafts inbox replies for approval (would drop MKT2's C6 to 1).
5. Count of PH Shopify stores nationally, and of PH Zoho/HubSpot/Freshdesk customers: not found.
6. Primary PH data on COD share and failed-delivery/RTS rates: only vendor claims found.
7. Viber Business Messages and PH SMS (Semaphore etc.) costs and partner requirements for a solo developer: not researched here.
8. Salesforce security-review fee in the current (2026) guide: only older guide versions seen.

## Sources

1. HubSpot Developers, "App Marketplace listing requirements", https://developers.hubspot.com/docs/apps/developer-platform/list-apps/listing-your-app/app-marketplace-listing-requirements (P).
2. HubSpot Marketplace, Aircall listing, https://ecosystem.hubspot.com/marketplace/listing/aircall (pricing updated 2026-08-05) (P).
3. HubSpot Marketplace, AgentAI listing (search snippet), https://ecosystem.hubspot.com/marketplace/listing/agentai (S).
4. HubSpot Knowledge Base, "Install agents from the Agent Marketplace", https://knowledge.hubspot.com/ai/install-agents-from-the-agent-marketplace (P).
5. HubSpot, "Breeze AI agents", https://www.hubspot.com/products/artificial-intelligence/breeze-ai-agents (V).
6. HubSpot Developers, "Agent tool listing requirements", https://developers.hubspot.com/docs/apps/developer-platform/list-apps/agent-tool-listing-requirements (P).
7. Pipedrive Developers, "About the Marketplace", https://pipedrive.readme.io/docs/about-the-marketplace (P/V).
8. Pipedrive Developers, "App approval process", https://pipedrive.readme.io/docs/marketplace-app-approval-process (P).
9. Pipedrive Dev Community threads: https://devcommunity.pipedrive.com/t/marketplace-app-review-timeline-2-weeks-no-updates-yet/20214 ; https://devcommunity.pipedrive.com/t/urgent-two-months-of-waiting-application-still-in-review/7184 (F).
10. Zoho Developer, Marketplace FAQs, https://www.zoho.com/developer/help/marketplace/faqs.html (P).
11. Zoho Developer, "Managing Payments", https://www.zoho.com/developer/help/marketplace/payments.html (P).
12. Pro SaaS (Substack), "Zoho marketplace review", 2022-07-10, https://prosaas.substack.com/p/zoho-marketplace-review (3P).
13. Salesforce ISVforce Guide, "Security review fees" (v224–236), e.g. https://developer.salesforce.com/docs/atlas.en-us.236.0.packagingGuide.meta/packagingGuide/security_review_fees.htm (P, older versions).
14. Salesforce ISVforce Guide, "AppExchange Checkout revenue share" (v252), https://developer.salesforce.com/docs/atlas.en-us.252.0.packagingGuide.meta/packagingGuide/appexchange_checkout_rev_share.htm (P).
15. Freshworks Developer Community, "Marketplace Paid App - FAQ", 2023-04-11, https://community.freshworks.dev/t/marketplace-paid-app-faq/9277 (F, staff post).
16. Freshworks Developer Community, "What's new with paid apps program", 2023-04-12, https://community.freshworks.dev/t/whats-new-with-paid-apps-program/9296 (F, staff post).
17. Zendesk Developer Docs, "Set up a paid app or theme", https://developer.zendesk.com/documentation/marketplace/publishing-to-the-marketplace/set-up-paid-app-or-theme/ (P).
18. monday.com Developer changelog, "Announcing the revshare program" (2024), https://developer.monday.com/apps/changelog/announcing-the-revshare-program (P).
19. monday.com App Developer, "Monetization", https://monday.com/appdeveloper/monetization (P).
20. monday.com, "App developers" page and AI hackathon blog, https://monday.com/w/app-developers ; https://monday.com/appdeveloper/blog/ai-app-hackathon-winners/ (V).
21. Notion Help, "Selling templates on Marketplace", https://www.notion.com/help/selling-on-marketplace (P; fee differs by locale copy).
22. Airtable Developers, "Custom Extensions FAQ", https://airtable.com/developers/extensions/guides/faq (P).
23. ClickUp Feedback, "ClickUp App Marketplace for Community-Built ClickApps", https://feedback.clickup.com/feature-requests/p/clickup-app-marketplace-for-community-built-clickapps (F).
24. Shopify Partners blog "Payout methods", https://www.shopify.com/partners/blog/payout-methods ; Shopify Help "Getting paid", https://help.shopify.com/partners/getting-started/getting-paid (P).
25. Prosperna blog, "Shopify shipping Philippines local courier guide", 2026-05-27, https://www.prosperna.com/blog/shopify-shipping-philippines-local-courier-guide (V: competing PH platform).
26. Shopify App Store, "J&T Philippines", https://apps.shopify.com/j-t-philippines (P).
27. AppNavigator, "J&T Philippines for Shopify" (review quotes), https://appnavigator.io/app/j-t-philippines/ (3P).
28. Shopify App Store, "Secure Payments via PayMongo" reviews, https://apps.shopify.com/secure-payments-via-paymongo/reviews (P).
29. StoreLeads, Shopify stores in Makati (updated 2026-10-16), https://storeleads.app/reports/shopify/PH/city/Makati (3P).
30. StoreLeads, Shopify stores in Pasig City (search snippet, 2026-06-19), https://storeleads.app/reports/shopify/PH/city/Pasig%20City (S).
31. Search summary of vendor blogs on PH COD/RTS: EasySell https://easysellapp.com/blogs/wiki/philippines-ecommerce-cod-shopify-logistics-7000-islands-2026 ; eGrow https://www.egrow.com/en/blog/philippines-cod-ecommerce-2026 ; Unbox.ph https://unbox.ph/news/gcash-ecommerce-2026/amp/ (V/S, low).
32. Ninja Van PH blog, "How to reduce COD returns", https://blog.ninjavan.co/en-ph/how-to-reduce-cod-returns/ (V).
33. Shopify App Store, "Level – Verify COD & reduce RTO", https://apps.shopify.com/level (P).
34. Fast2SMS help, "Reduce COD RTO on Shopify", https://www.fast2sms.com/help/?p=17048 (V).
35. HubSpot, "AI customer service agent", https://www.hubspot.com/products/artificial-intelligence/ai-customer-service-agent (P/V).
36. Freshworks, "Freshdesk pricing", https://www.freshworks.com/freshdesk/pricing/ (P).
37. Pipedrive AI third-party summaries (search): https://aiproductivity.ai/pricing/pipedrive/ ; https://theaiagentindex.com/agents/pipedrive-ai (3P, L).
38. Zoho CRM pricing/AI third-party summaries (search): https://usagepricing.com/blueprint/zoho ; https://aiproductivity.ai/pricing/zoho-crm/ ; https://automationatlas.io/answers/zoho-crm-pricing-explained-2026/ (3P, L).
39. monday.com Support, "AI Credits" https://support.monday.com/hc/en-us/articles/29544502265746-AI-Credits ; "The pricing model for monday AI portfolio" https://support.monday.com/hc/en-us/articles/35277848309394 ; "AI Feature Catalog" https://support.monday.com/hc/en-us/articles/24047211522194 ; per-credit price via Plaky https://plaky.com/learn/?p=3796 (P via search summary, M; per-credit price 3P, L).
40. Notion AI pricing third-party summaries (search): https://fast.io/resources/notion-ai-review-2026/ ; https://krater.ai/blog/notion-ai-pricing (3P, M).
41. HubSpot Community Ideas, "Allow Multiple Breeze Customer Agents per HubSpot Account", https://community.hubspot.com/t5/HubSpot-Ideas/Feature-Request-Allow-Multiple-Breeze-Customer-Agents-per/idi-p/1142665 (F).
42. Zoho Marketplace SMS extensions (search), e.g. https://marketplace.zoho.in/app/crm/twilio-sms-texting-for-zoho-crm ; https://bulkgate.com/en/integrations/zoho-sms-module/ (P/S, M).
43. Viber for Business, Promotexter partner page, https://www.forbusiness.viber.com/en/messaging-partners/partner/promotexter/ (V).
