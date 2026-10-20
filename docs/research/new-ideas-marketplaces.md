# New ideas from self-serve marketplaces

Status: **research, 2026-10-18.** This file looks for *new* product ideas, ones not already in the [opportunity-landscape](opportunity-landscape.md) inventory. It uses one channel: global app marketplaces where a solo developer in the Philippines can list for free or nearly free and get installs without calls or ads. Scores use [scoring-method.md](scoring-method.md).

**Method and its limits.**
- Facts come from official docs, terms pages and marketplace listings that were fetched directly, including review pages filtered to 1 star. Numbered sources are at the end.
- **The session's shared web-search budget ran out early.** Most facts therefore come from pages fetched directly rather than found by search.
- As a result, several secondary platforms were not checked: Zapier, Notion, Obsidian, Home Assistant and Microsoft Teams. **No solo-developer revenue case studies were found** for any marketplace.
- Complaint patterns summarise 1-star review pages. Quoted phrases are ones actually seen on the cited page.
- Labels: **[P]** primary (official docs or marketplace page), **[V]** vendor claim, **[3P]** third party, **[S]** search snippet. Confidence H/M/L.
- Everything was accessed 2026-10-18.

## Key findings

1. **Shopify is the strongest channel.**
   - The economics are the best of any marketplace checked: a one-time US$19 registration fee and 0% revenue share on the first US$1M of lifetime app revenue [1].
   - First-party AI tools and incumbents with thousands of reviews already cover the obvious AI features, so a newcomer has to win a narrow job [8][10][12].
   - Payout runs through Hyperwallet. The Philippines is **not confirmed** on any official page (M) [5][6].
2. **The repeated complaint across Shopify categories is not bad AI.** It is surprise usage billing, AI bots that replace humans and can't be turned off, and forced plan or UI changes [12][14][16][21][24]. That favours a product whose selling point is *nothing reaches a customer without your approval, at a flat price*.
3. **WordPress.org and Apify are cheap, PH-payable second channels.**
   - WordPress.org listing is free, and Freemius or Lemon Squeezy can pay out to the Philippines [32][40].
   - Apify keeps 20% and pays by PayPal or Wise from US$20, with no country exclusions listed [43][45].
   - Neither suits approval-gated agents as naturally as Shopify does.
4. **Four platforms are ruled out by payment or format:**
   - Discord Premium Apps pays only US/EU/UK teams [64].
   - Stripe doesn't support PH [38], and ExtensionPay (the usual Chrome-extension billing tool) needs Stripe [29].
   - ChatGPT apps may not sell digital goods [54].
   - MCP registries and the Claude directory have no billing [50][51][53].

## 1. Marketplace comparison

| Marketplace | Listing cost | Revenue share / fees | Approval burden | Organic-install evidence | PH payout | Verdict |
|---|---|---|---|---|---|---|
| **Shopify App Store** | US$19 one-time Partner fee [1] (P,H) | 0% on first US$1M lifetime (revenue from 2025-01-01), 15% above; 2.9% processing fee [1] (P,H) | Draft → Submitted → Reviewed → Published; **no published timeline** [2]. "Built for Shopify" (BFS) needs ≥50 net paid installs, ≥5 reviews, Web Vitals at p75 [3] | Store says "over 16,000 apps" [8] (V); Let's Metrix scores 19,565 apps [7] (3P,M); English sitemap lists 28,639 app URLs (own count, M) [8]. Ranking uses listing text, reviews and off-platform traffic; BFS gives a ranking boost [4][9]. Many 2026 AI entrants sit at 0 reviews [20][22][25] | Hyperwallet, min US$25, twice monthly [5]. Shopify's partner blog says PayPal, local bank or wire, "depending on your region" [6] (S,M). **PH not confirmed** | **Best channel** |
| **WordPress.org plugins** | Free | None; paid upgrades sold off-site. Freemius 3.8% + $0.50, plus 2.3% for WP products [32] (V,H) | Review queue peaked at ~1,050 plugins in mid-April 2026, then fell to "nearly zero"; AI-assisted checks now [30] (P,H). Wait time not published | Install bands are public (e.g., 6k+, 30k+). Newcomer install curves: **not found** | Freemius pays by PayPal, Payoneer or Wise, $100 minimum [32]. Lemon Squeezy lists PH for bank payouts [40] (P,H) | **Good second channel**; low price norms |
| **Apify Store** | Free | Apify keeps 20%; for pay-per-event Actors, platform costs come out of payouts [43] (P,H). Rental model retired 2026-10-01 [44] | No listing review found | 88,364 Actors (store API) [47]. Indie Actors at 2.7k–3.7k monthly users [47] (P,H). "$1.7M paid out last month" and 5,600 developers [46] (V,M); a 2023 post said the top independent creator earned over $4k/month [48] (V,M) | PayPal or Wise from $20, KYC, no country list [45] (P,H; PH M) | **Good for cash; weak foundation fit** |
| **Atlassian Marketplace (Forge)** | None found | 100% until US$1M lifetime Forge revenue (Forge-only apps), then 83–84% [55] (P,H) | Forge listing decision targeted "within one week" [57] (P,H) | Most AI apps have <300 installs; AI release notes is the exception (1,674 installs) [58] (P,H) | Bank EFT, may hold until $500 owed [56] (P,H; PH M) | **Viable niche channel**; Forge runs JavaScript, so Rust only behind Forge Remote (J) |
| **Google Workspace Marketplace** | No listing fee [33] (P,H) | Developer bills on their own [33] (M) | OAuth verification; restricted scopes (Gmail, full Drive) need a CASA assessment, revalidated yearly [36][37] (P,H). `drive.file` is non-sensitive [35]. CASA fee: **not found** | Very large winners: YAMM 16M+ installs, GPT for Sheets 7M+ [77][78]. AI triage add-ons are tiny (3–842 installs) [79] | Via Lemon Squeezy or Paddle [40][41] | Possible for Sheets-only tools; Gmail tools carry CASA cost |
| **Chrome Web Store** | One-time fee, US$5 per third parties [27] (P/3P,M) | No store payments (deprecated; official page now 404, M). ExtensionPay takes 5% but needs Stripe [29] | "Most… within a few days"; can take weeks [28] (P,H) | Not researched | Stripe not in PH [38]. Lemon Squeezy works [40] | OK as a free-tier funnel only |
| **Slack Marketplace** | None found | No Slack billing [59] | Needs ≥10 active workspaces and ≥10 weekly active users first [59] (P,H) | Not found | Own billing | Poor first channel |
| **Microsoft Marketplace / AppSource** | Partner Center | 3% on transactable offers [60] | Multitenant SaaS plus Entra ID plus Fulfillment APIs [60] | Not found | PH is a buyer country [61]; publisher payout to PH: **not found** | Enterprise-shaped; no |
| **GitHub Marketplace** | None found | 5%; payout once $500 is owed in a month [62] | Paid plans need a verified org plus ≥100 installs [62] | Not found | Method or country: not found | No (gated paid tier) |
| **Figma Community** | — | 15%; payouts in ~70 countries including PH [63] | "Not approving new creators to sell paid files" [63]; plugin status unclear | — | PH listed [63] | Not agent-shaped |
| **Discord Premium Apps** | — | Not stated | — | — | **US/EU/UK only** [64] (P,H) | Ruled out |
| **Telegram bots (Stars)** | Free | Developer gets ~$0.013 per Star; payout via Fragment, which may not pay "in certain countries" [65] | None | Not found | PH: not found | Consumer micro-payments; no |
| **MCP Registry / Smithery / MCPize** | Free | Official registry and Smithery: no monetisation [50][51]. MCPize claims publishers keep 80% [52] (V,L) | Registry in preview; API frozen at v0.1 [50] | Independent revenue data: **not found** | MCPize payout: not found | Distribution add-on only |
| **Claude Connectors Directory / ChatGPT apps** | Paid Claude plan to submit [53] | No billing [53]. ChatGPT bans digital-goods sales [54] | OAuth, privacy policy, test account [53] | Not found | n/a | Distribution add-on only |
| **Raycast** | Free (PR to repo) [67] | Paid extensions: not found | PR review | Not found | n/a | No |
| Zapier, Notion, Obsidian, Home Assistant, Teams store | **Not checked** (search budget) | — | — | — | — | Open |

**Payment rails for a PH developer** (the binding constraint off Shopify):
- **Stripe:** PH is not listed [38], and Connect cross-border payouts cover only US/UK/EEA/CA/CH [39] (P,H).
- **Lemon Squeezy:** names PH for bank payouts [40] (P,H).
- **Paddle:** excludes only sanctioned countries, so PH is allowed by omission [41] (P,M).
- **PayPal PH:** receives with a 3.5% conversion fee and is free to withdraw above ₱7,000 [42] (P,H).

## 2. Candidate products

Shopify's own free tools set the bar every Shopify idea must clear:
- **Sidekick** writes descriptions, SEO content and emails, builds Flow automations, runs background tasks, and asks for approval before applying changes [10].
- **Shopify Inbox** is now "an AI-powered sales associate" [12].
- **Built-in features** cover abandoned-checkout emails and order-tracking pages [13].

So no candidate below is "AI writes your product copy" or "AI chatbot". Since June 2026, **Sidekick app extensions** let third-party apps answer inside Sidekick [11]. That is a distribution surface as well as a competitor.

### MK1: Shopify order-exception agent ("no stuck order goes silent")

- **Problem.** Shipments stall, get marked delivered but go missing, or sit unfulfilled. Customers then email "where is my order", file chargebacks or leave bad reviews. Tracking apps show status; they don't act on the exceptions.
- **Buyer.** Small Shopify merchants doing roughly 50–2,000 orders a month who answer their own support (J).
- **Evidence of payment** [24] (P,H):
  - ParcelPanel (CWILL): 5.0 from 3,185 reviews; free for 20 orders, $11/mo for 200.
  - AfterShip: 4.6 from 1,504 reviews; free for 50 shipments, $11/mo plus $0.08–0.12 per extra shipment.
- **Competitors focused on exceptions** [25] (P,H):
  - ReturnSense (Apr 2026, 0 reviews): an "exceptions inbox", $19/mo.
  - IssueDesk: 1 review, $9/mo.
  - The narrow job exists, and nobody has won it yet.
- **Evidenced complaints** [24]:
  - AfterShip: over-quota per-order charges, cancelling needs a manager, custom domain moved to the Enterprise tier.
  - ParcelPanel/CWILL: confusion over a forced platform migration; one reviewer wrote "Better to build your own things with AI."
- **Agent workflow (approval-gated):**
  1. A durable scheduled job reads fulfilments and tracking events every few hours.
  2. Rules plus the LLM classify each exception: stalled N days, delivery failed, unfulfilled past the promised date, address problem.
  3. The agent drafts a customer email in the store's voice, with order facts filled in by tool calls. Where relevant it also drafts a carrier-claim note or a reship/refund *proposal*.
  4. The merchant approves, edits or skips each item in an embedded queue. Sends are idempotent, and refunds and reships are never automatic.
  5. Durable follow-ups check again later and close the case.
- **MVP size.** About 6–8 weeks (J):
  - Embedded app with App Bridge and a Rust backend.
  - Orders and fulfilment webhooks.
  - Email through the merchant's domain or Shopify Messaging (first 10,000 emails a month free) [13].
  - **Risk:** detailed carrier events may need a paid tracking API. Its cost is **not found**, so the MVP should start with Shopify's fulfilment status and tracking URLs.
- **PH payout path.** Shopify billing, then Hyperwallet (PayPal or bank by region; PH unconfirmed) [5][6].
- **What would disprove it:**
  - Fewer than 10 installs after 60 days listed with an optimised listing.
  - Or merchants install but approve fewer than ~20% of drafts, which would mean the exceptions aren't worth acting on.
  - Or Shopify adds exception handling to Sidekick or Flow templates.

### MK2: Shopify approval-first support reply drafter ("never a bot in front of your customers")

- **Problem.** Owner-operators answer order, shipping and returns emails by hand. AI chat tools put a bot in front of customers, which merchants dislike, and they bill per conversation or resolution.
- **Buyer.** Stores with 1–3 people handling support (J).
- **Evidence of payment** (P,H):
  - Gorgias: 4.3 from 706 reviews; Starter $10/mo for 50 tickets, then $0.40 per ticket; the AI Agent is an add-on [14].
  - Chatty (BFS): 4.9 from 1,879 reviews; $19.99/mo, then $0.40 per extra AI conversation [17].
  - Tidio: 4.8 from 1,356 reviews; Lyro AI $39/mo for 200 conversations [16].
  - Re:amaze: $29 per user, 5 AI resolutions per user per month [15].
  - Commslayer (launched Jan 2024): 4.9 from 237 reviews; free tier includes 300 AI drafts [18]. This shows a newcomer can reach hundreds of reviews in about 2.5 years.
- **Direct new entrants** (all 0 reviews) [20]:
  - Reslo "AI Order Replies": 10 drafts free, $19/mo for 150.
  - ReplyPilot: $19 for 500.
  - SupportReply: $4.99 for 50.
  - Sensbot: $19 for 200.
  - **The crowd is real.**
- **Evidenced complaints:**
  - **Shopify Inbox** (free, 4.6 from 5,813 reviews; Aug–Sep 2026 1-star reviews) [12]: the AI "cannot be turned off", there is no handover to a human, and a forced sign-in button.
  - **Gorgias** [14]: an AI agent upsold inside the app that "hallucinates", "AI stuff is half baked", billing continued after access was cut.
  - **Tidio** [16]: double charges, listing price differs from the website, "the AI bot runs you around in circles".
  - **Re:amaze** [15]: AI-only vendor support.
- **Agent workflow:**
  1. A forwarding address or IMAP inbox feeds a durable queue.
  2. The agent classifies each email (WISMO, return, address change, product question, other).
  3. Tool calls pull the order, fulfilment and policy data.
  4. It drafts a reply that cites the facts used.
  5. The owner approves with one click, and sends are idempotent.
  6. Proposed actions (cancel, refund, address edit) are separate approval items.
  7. A flat monthly price with no per-resolution charge.
- **MVP size.** About 5–7 weeks (J). The inbound email plumbing and a reliable order lookup are the hard parts.
- **PH payout path.** Same as MK1.
- **What would disprove it:**
  - The 0-review cohort [20] stays at 0 for months. That would show the category can't be reached without ads; check in 90 days.
  - Or listing tests get impressions but under 1% install conversion.
  - Or Shopify Inbox adds draft-only and human-handover modes.

### MK3: WordPress content-decay refresh agent

- **Problem.** Older posts lose rankings and traffic. Publishers and affiliate sites know they should refresh them, but finding which ones and rewriting them is slow.
- **Buyer.** Bloggers, affiliate sites and small content teams on WordPress (J).
- **Evidence of payment** (P,H):
  - AIOSEO (2M+ installs, 4.7 from 5,224 ratings) sells "content decay tracking" and AI credits [72].
  - Rank Math (4M+ installs) sells Content AI tiers; one annual price shown is €71.88/yr [71].
  - Link Whisper (30k+ installs) sells AI credits [73].
- **The gap.** A "content refresh" search returned mostly cache plugins. **No dedicated AI refresh plugin with traction was found** [76] (P,M).
- **Evidenced complaints:**
  - Link Whisper (4.1 stars) [73]: slow paid support, double charges, "Incredibly crazy selection of suggested links", database bloat.
  - AI Engine [74]: "Hidden charges from OpenAI".
  - Pattern: low-quality AI suggestions and surprise costs.
- **Agent workflow:**
  1. A weekly durable job pulls Search Console metrics per URL. The scope class has not been verified, but it is believed non-restricted (J).
  2. It flags decaying posts.
  3. It drafts a refresh as a WordPress *revision* or draft: updated facts, sections, internal links and meta.
  4. It shows a diff, and the owner approves it into publication.
  5. It tracks the URL's traffic for 4–8 weeks afterwards and reports the effect.
- **MVP size.** About 6–8 weeks: a PHP plugin shell, a Rust service, the Google OAuth flow, and diff UI (J).
- **PH payout path.** Freemius (PayPal, Payoneer or Wise) or Lemon Squeezy (PH bank) [32][40].
- **What would disprove it:**
  - Fewer than 200 active installs in 90 days.
  - Or under 2% free-to-paid conversion.
  - Or refreshed posts show no traffic change, so value can't be shown.

### MK4: Apify AI-extraction Actors (pay-per-event)

- **Problem.** Developers and AI agents need clean, structured records (company profile, contacts, pricing, job details) from messy pages. Raw scrapers return HTML or loose text.
- **Buyer.** Developers, growth teams and, increasingly, agents. Apify supports agent payments via x402 and Skyfire through its MCP server [49] (P,H).
- **Evidence of payment:**
  - "$1.7M paid out last month" [46] (V,M).
  - Indie AI and data Actors with 2,764–3,746 monthly users, e.g. b2b-leads-finder and YouTube-transcript and Reddit Actors [47] (P,H).
  - Apify's own `rag-web-browser` has 33,211 monthly users [47].
- **Competitors.** 88,364 Actors in the store: 11,665 tagged AI and 33,119 lead generation [47] (P,H).
  - Google Maps: 922 Actors, only 4 with ≥1,000 monthly users, and the top 4 belong to Apify-affiliated publishers [47].
  - The leading Maps scraper charges from $1.50 per 1,000 places [47].
- **Evidenced complaints.** Low-star review text was **not collected**. The structural risk is documented instead: platform costs are deducted from pay-per-event payouts, so profit can go negative [43] (P,H).
- **Agent workflow.**
  - The Actor fetches pages, uses the LLM to extract into a fixed schema with a confidence score, and returns rows.
  - Low-confidence rows go to an optional review dataset rather than through human approval.
  - **Approvals and durable scheduling are barely exercised.**
- **MVP size.** About 2–4 weeks per Actor. Actors are Docker images, so Rust works (J).
- **PH payout path.** PayPal or Wise from $20 [45].
- **What would disprove it:**
  - Under 50 monthly users after 60 days.
  - Or margins negative after LLM and platform costs at competitive per-result prices.

### MK5: Atlassian Forge release-notes agent

- **Problem.** Teams turn closed Jira issues into release notes for customers and stakeholders by hand.
- **Buyer.** Software teams on Jira Cloud (J).
- **Evidence of payment** (P,H for installs, M for prices) [58]:
  - Amoeboids "(AI powered) Automated Release Notes": 1,674 paid installs (plus 2,004 on the free edition), 4.53 from 48 ratings, $6/mo at 10 users.
  - Released Hub: 890 installs, 4.97 from 39 ratings, $0 at 10 users.
- **AI entrants with weak traction:**
  - InLabs: 74 installs.
  - TitanApps: 56 installs, 3.1 stars.
- **Evidenced complaints.** Not collected; TitanApps' 3.1 rating is the only signal.
- **Agent workflow.**
  1. When a version is released, a job gathers issues and pull-request titles.
  2. It drafts audience-specific notes (customer, internal, changelog).
  3. A reviewer approves them in Jira.
  4. On approval it publishes to a Confluence page or changelog.
- **MVP size.** About 4–6 weeks (J).
  - Forge functions run JavaScript, so Rust sits behind Forge Remote.
  - Whether that still counts as "Forge-only" for the 100% revenue share is **not verified**.
- **PH payout path.** Bank EFT, $500 hold threshold [56] (PH M).
- **What would disprove it:**
  - Fewer than 30 installs in 90 days.
  - Or Atlassian's own AI covers release notes. Rovo's current scope was **not checked**.

### Considered and dropped

- **Shopify review-reply AI.**
  - Judge.me (5.0 from 48,703 reviews) includes AI replies at $15/mo with a free plan.
  - Loox and Yotpo bundle them too [21].
  - New entrants such as Reviewlark have no reviews [22]. This niche is commoditised.
- **Shopify SEO and alt text.**
  - There are five BFS incumbents with 764–5,488 reviews, from $14.99/mo [23].
  - Sidekick generates SEO content for free [10].
  - Complaints are real, e.g. SearchPie meta edits disappearing on uninstall [23], but there is no room for a newcomer.
- **Shopify abandoned checkout.** Native recovery emails exist [13], and Omnisend is $16/mo with 3,193 reviews [26].
- **WordPress spam and comment triage.**
  - CleanTalk is "$12 per year" with 3,219 ratings [69].
  - Akismet's 1-star reviews are mostly about payment [68], which shows the segment resists paying.
  - AI comment reply is nearly empty (AutoReply AI: 20+ installs [76]). That is a sign of no demand rather than a gap (J).
- **Google Workspace AI add-ons.**
  - GPT for Sheets has 7M+ installs at $25/mo [78], and Gmail AI writers reach 617K+ installs [79].
  - But a Gmail agent needs restricted scopes and a yearly CASA assessment [37] (fee not found).
  - It also overlaps the inventory's prosumer email triage (K).

## 3. Scores

Order is C1..C16. Weighted totals come from the method's script (`scoring/score.py` logic). B and C are shown for reference using the scorecard strings (C with the self-serve adjustments).

| ID | Candidate | C1..C16 | Balanced | Revenue-first | Builder-first |
|---|---|---|---|---|---|
| MK2 | Shopify approval-first support reply drafter | `3J 4J 3J 5E 4E 2E 3E 4J 3J 4J 3J 4J 3J 4J 4J 4J` | 70.6 | 70.6 | 72.0 |
| MK1 | Shopify order-exception agent | `3J 4J 3J 4E 3E 3J 3E 3J 3J 4J 3J 3J 3J 4J 4J 4J` | 66.6 | 66.2 | 67.6 |
| MK3 | WordPress content-decay refresh agent | `3J 2J 3J 3E 3E 3E 3E 3J 3J 4J 4J 3J 3J 3J 3J 4J` | 61.6 | 60.6 | 63.2 |
| MK5 | Atlassian Forge release-notes agent | `2J 2J 3J 4E 2E 2E 3E 3J 4E 4J 4J 4J 4J 4J 3J 3J` | 61.6 | 60.8 | 63.6 |
| MK4 | Apify AI-extraction Actors | `2J 3J 3J 4E 3E 2E 4E 4J 4J 3J 2J 4J 3E 3J 3J 2J` | 61.4 | 61.8 | 61.4 |
| (ref) B | PH lead & quote follow-up | from scorecard | 72.4 | 71.0 | 74.2 |
| (ref) C | AR collections on Xero/QBO | from scorecard | 71.8 | 73.2 | 72.0 |

**Score rationale for the contested cells:**
- **MK2:**
  - C4 = 5E: several paid apps with hundreds to thousands of reviews [14]–[17].
  - C6 = 2E: free Shopify Inbox AI, Commslayer's free 300 drafts and a crowd of $5–19 entrants [12][18][20].
  - C7 = 3E rather than 4: a store with buyer intent, but the 0-review cohort shows discovery is hard without BFS [3][20].
- **MK1:**
  - C6 = 3J: the strong incumbents (ParcelPanel, AfterShip) track rather than resolve, and exception-focused apps have no traction [24][25].
  - C8 = 3J because of the carrier-data dependency.
- **MK5:**
  - C5 = 2E: $6/mo at 10 users, and Released Hub is free at that size [58].
  - C9 = 4E: Forge listing decision targeted within one week [57].
- **MK4:**
  - C11 = 2J: scraping personal data and site terms.
  - C16 = 2J: no human approval and little durable scheduling.
- **Disqualifiers:** none of the candidates has a 1 on C5, C6 or C11.

**Reading.**
- **No marketplace idea beats B or C under Balanced.** MK2 comes within ~2 points and would rank #3 overall.
- MK2 is also the most evidence-backed demand of any idea here.
- MK2's scores are the same across all three profiles. That is a good sign: it does not win only under Builder-first.

## 4. Top pick: MK2, built to make MK1 its second module

**MK2 (approval-first Shopify support drafter), with MK1's exception watcher as its first upsell.**

- **The marketplace solves the owner's hardest constraint.**
  - Shopify is the one channel checked that has search intent, in-admin billing and 0% revenue share up to US$1M [1].
  - The cash outlay is US$19 once, within the ₱600/month budget.
  - The payout rail (Hyperwallet with PayPal) is plausible for PH but **must be confirmed in the Partner Dashboard before building** [5][6].
- **The evidenced complaint matches the agent core.**
  - Recent reviews attack AI that "cannot be turned off", has no human handover, "hallucinates", and bills per resolution [12][14][16].
  - A draft → approve → idempotent-send design with flat pricing answers each complaint directly.
  - It reuses the same core as B: durable queue, approval gate, tool calls to order data.
- **MK1 shares about 80% of the code** (orders, email sending, approval queue) and adds scheduled watchers. That is the C16 durable-work showcase, and it differentiates MK2 from the $5–19 reply-only crowd.
- **Main risk: discovery.**
  - Many 2026 AI-reply apps sit at 0 reviews [20], and BFS needs 50 paid installs and 5 reviews first [3].
  - Mitigations that cost nothing:
    1. Write the listing copy for the complaint keywords ("human approval", "no chatbot", "flat price").
    2. Use a generous free tier to earn the first reviews.
    3. Drive off-platform traffic from SEO content and Shopify community answers, since Shopify says this helps ranking [9].
- **First falsification test.** Before building fully, check two facts in the Partner Dashboard:
  - That PH payouts work.
  - That a listing can be submitted with a minimal app.
  - Then watch the 0-review cohort [20]. If none of them gain reviews in 90 days, discovery is the binding problem and MK2 drops below B.

## Sources

All accessed 2026-10-18. "Current" means an undated live page.

| # | Title / page | Publisher | Date | URL | Type | Conf. |
|---|---|---|---|---|---|---|
| 1 | Revenue share for App Store developers | Shopify | current | https://shopify.dev/docs/apps/launch/distribution/revenue-share | P | H |
| 2 | App Store review process | Shopify | current | https://shopify.dev/docs/apps/launch/app-store-review/review-process | P | H |
| 3 | Built for Shopify requirements | Shopify | current | https://shopify.dev/docs/apps/launch/built-for-shopify/requirements | P | H |
| 4 | Built for Shopify (benefits) | Shopify | current | https://shopify.dev/docs/apps/launch/built-for-shopify | P | H |
| 5 | Getting paid (Partners) | Shopify Help | current | https://help.shopify.com/en/partners/getting-started/getting-paid | P | H |
| 6 | New payout methods for Partners | Shopify Partners blog | undated | https://www.shopify.com/partners/blog/payout-methods | S | M |
| 7 | Shopify app index | Let's Metrix | ©2026 | https://letsmetrix.com | 3P/V | M |
| 8 | App Store homepage; English app sitemap (lastmod 2026-10-12) | Shopify | current | https://apps.shopify.com/ ; https://apps.shopify.com/sitemap_apps_en.xml | P | H (count interpretation M) |
| 9 | Market your app | Shopify | current | https://shopify.dev/docs/apps/launch/marketing | P | H |
| 10 | Sidekick (built-in feature; help) | Shopify | current | https://apps.shopify.com/built-in-features/sidekick ; https://help.shopify.com/en/manual/shopify-admin/productivity-tools/sidekick | P | H |
| 11 | Guide: Sidekick app extensions | Shopify App Store stories | 2026-06-17 | https://apps.shopify.com/stories/guide-sidekick-app-extensions | P | H |
| 12 | Shopify Inbox listing and 1-star reviews | Shopify | current | https://apps.shopify.com/inbox/reviews?ratings%5B%5D=1&sort_by=newest | P | H |
| 13 | Flow; Shopify Messaging; abandoned checkout; order tracking | Shopify | current | https://apps.shopify.com/flow ; https://apps.shopify.com/shopify-messaging ; https://apps.shopify.com/built-in-features/abandoned-checkout-recovery ; https://apps.shopify.com/built-in-features/order-tracking | P | H |
| 14 | Gorgias listing and reviews | Shopify App Store | current | https://apps.shopify.com/helpdesk | P | H |
| 15 | Re:amaze listing and reviews | Shopify App Store | current | https://apps.shopify.com/reamaze | P | H |
| 16 | Tidio listing and reviews | Shopify App Store | current | https://apps.shopify.com/tidio-chat | P | H |
| 17 | Chatty listing | Shopify App Store | current | https://apps.shopify.com/chatty | P | H |
| 18 | Commslayer listing | Shopify App Store | current | https://apps.shopify.com/commslayer | P | H |
| 19 | Zendesk listing | Shopify App Store | current | https://apps.shopify.com/zendesk | P | H |
| 20 | Reslo AI Order Replies; ReplyPilot; SupportReply; Sensbot; others | Shopify App Store | 2025–26 | https://apps.shopify.com/ai-order-issue-resolver ; https://apps.shopify.com/replypilot-2 ; https://apps.shopify.com/supportreply | P | H |
| 21 | Judge.me; Loox; Yotpo listings and reviews | Shopify App Store | current | https://apps.shopify.com/judgeme | P | H |
| 22 | Reviewlark AI Replies | Shopify App Store | 2026-09-23 | https://apps.shopify.com/ai-review-replies | P | H |
| 23 | Booster SEO; Avada; SearchPie; StoreSEO; Smart SEO | Shopify App Store | current | https://apps.shopify.com/seo-image-optimizer ; https://apps.shopify.com/avada-seo-suite ; https://apps.shopify.com/seo-booster ; https://apps.shopify.com/storeseo ; https://apps.shopify.com/smart-seo | P | H |
| 24 | ParcelPanel (CWILL); AfterShip listings and reviews | Shopify App Store | current | https://apps.shopify.com/parcelpanel | P | H |
| 25 | ReturnSense; IssueDesk | Shopify App Store | 2026 | https://apps.shopify.com/delayradar ; https://apps.shopify.com/order-issue-tracker | P | H |
| 26 | Recart; Omnisend | Shopify App Store | current | https://apps.shopify.com/ | P | H |
| 27 | Register as a Chrome Web Store developer | Google | current | https://developer.chrome.com/docs/webstore/register | P (fee amount 3P) | H/M |
| 28 | Chrome Web Store review process | Google | current | https://developer.chrome.com/docs/webstore/review-process | P | H |
| 29 | ExtensionPay | ExtensionPay | current | https://extensionpay.com | V | H |
| 30 | Update on the status of the team, June 2026 | WordPress.org Plugins Team | 2026-06-13 | https://make.wordpress.org/plugins/2026/06/13/update-on-the-status-of-the-team-june-2026/ | P | H |
| 31 | Not used (number kept so later references stay stable) | — | — | — | — | — |
| 32 | Freemius pricing | Freemius | current | https://freemius.com/pricing | V | H |
| 33 | Workspace Marketplace FAQ | Google | current | https://developers.google.com/workspace/marketplace/faq | P | H |
| 34 | About app review | Google | current | https://developers.google.com/workspace/marketplace/about-app-review | P | H |
| 35 | Sheets API scopes | Google | current | https://developers.google.com/workspace/sheets/api/scopes | P | H |
| 36 | Restricted scopes | Google Cloud Help | current | https://support.google.com/cloud/answer/13464325 | P | H |
| 37 | Security assessment (CASA) | Google Cloud Help | current | https://support.google.com/cloud/answer/13465431 | P | H |
| 38 | Stripe global availability | Stripe | current | https://stripe.com/global | P | H |
| 39 | Connect cross-border payouts | Stripe | current | https://docs.stripe.com/connect/cross-border-payouts | P | H |
| 40 | Supported countries | Lemon Squeezy | current | https://docs.lemonsqueezy.com/help/getting-started/supported-countries | P | H |
| 41 | Which countries are supported by Paddle | Paddle | current | https://www.paddle.com/help/start/intro-to-paddle/which-countries-are-supported-by-paddle | P | M |
| 42 | PayPal fees (PH) | PayPal | current | https://www.paypal.com/ph/webapps/mpp/paypal-fees | P | H |
| 43 | Monetize your Actor | Apify | current | https://docs.apify.com/platform/actors/publishing/monetize | P | H |
| 44 | Rental pricing (retirement) | Apify | current | https://docs.apify.com/actors/publishing/monetize/rental.md | P | H |
| 45 | Monthly payouts | Apify | current | https://docs.apify.com/actors/publishing/monetize/monthly-payouts.md | P | H (PH M) |
| 46 | Actor developers program | Apify | undated (testimonials to 2026-03) | https://apify.com/partners/actor-developers | V | M |
| 47 | Store API (counts, monthly users, pricing) | Apify | live 2026-10-18 | https://api.apify.com/v2/store ; https://apify.com/compass/crawler-google-places | P | H |
| 48 | Programmer passive income via web scraping | Apify blog | 2023 | https://blog.apify.com/programmer-passive-income-web-scraping/ | V/S | M |
| 49 | Agentic payments | Apify | current | https://docs.apify.com/agents/payments.md | P | H |
| 50 | MCP Registry README | modelcontextprotocol | 2025–26 | https://github.com/modelcontextprotocol/registry | P | H |
| 51 | Smithery docs | Smithery | current | https://smithery.ai/docs | P | M |
| 52 | MCPize | MCPize | ©2026 | https://mcpize.com | V | L |
| 53 | Connector submission | Anthropic | FAQ 2026-08-06 | https://claude.com/docs/connectors/building/submission | P | H |
| 54 | Apps SDK submission guidelines; monetization | OpenAI | undated | https://developers.openai.com/apps-sdk/app-submission-guidelines ; https://developers.openai.com/apps-sdk/build/monetization | P | H |
| 55 | Pricing, payment and billing | Atlassian | current | https://developer.atlassian.com/platform/marketplace/pricing-payment-and-billing/ | P | H |
| 56 | Marketplace Publisher Agreement | Atlassian | rev. 2025-12-01 | https://www.atlassian.com/licensing/marketplace/publisheragreement | P | H |
| 57 | Listing Forge apps | Atlassian | updated 2026-08-26 | https://developer.atlassian.com/platform/marketplace/listing-forge-apps/ | P | H |
| 58 | Marketplace REST API (release-notes and AI app listings) | Atlassian | live 2026-10-18 | https://marketplace.atlassian.com/rest/2/addons | P | H (prices M) |
| 59 | Slack Marketplace guidelines and requirements | Slack | current | https://docs.slack.dev/slack-marketplace/slack-marketplace-app-guidelines-and-requirements | P | H |
| 60 | Commercial transaction capabilities | Microsoft Learn | 2026-07-22 | https://learn.microsoft.com/en-us/partner-center/marketplace-offers/marketplace-commercial-transaction-capabilities-and-considerations | P | H |
| 61 | Geo availability and currencies | Microsoft Learn | 2026-01-21 | https://learn.microsoft.com/en-us/partner-center/marketplace-offers/marketplace-geo-availability-currencies | P | H |
| 62 | Receiving payment; requirements for listing | GitHub Docs | current | https://docs.github.com/en/apps/github-marketplace | P | H |
| 63 | Sell on Figma Community | Figma Help | current | https://help.figma.com/hc/en-us/articles/12067637274519 | P | H |
| 64 | Enabling monetization | Discord | current | https://docs.discord.com/developers/monetization/enabling-monetization | P | H |
| 65 | Bot developer terms §6.2.4 | Telegram | current | https://telegram.org/tos/bot-developers | P | H |
| 66 | Telegram Stars | Telegram | 2024-06-06 | https://telegram.org/blog/telegram-stars | P | M |
| 67 | Publish an extension | Raycast | current | https://developers.raycast.com/basics/publish-an-extension | P | M |
| 68 | Akismet listing, pricing, 1-star reviews | WordPress.org; Akismet | current | https://wordpress.org/support/plugin/akismet/reviews/?filter=1 ; https://akismet.com/pricing/ | P/V | H |
| 69 | CleanTalk listing and 1-star reviews | WordPress.org | current | https://wordpress.org/plugins/cleantalk-spam-protect/ | P | H |
| 70 | Maspik; OOPSpam | WordPress.org | current | https://wordpress.org/plugins/contact-forms-anti-spam/ ; https://wordpress.org/plugins/oopspam-anti-spam/ | P | H |
| 71 | Rank Math listing; Content AI | WordPress.org; Rank Math | current | https://wordpress.org/plugins/seo-by-rank-math/ ; https://rankmath.com/content-ai/ | P/V | H (price M) |
| 72 | All in One SEO | WordPress.org | current | https://wordpress.org/plugins/all-in-one-seo-pack/ | P | H |
| 73 | Link Whisper listing and 1-star reviews | WordPress.org | current | https://wordpress.org/plugins/link-whisper/ | P | H |
| 74 | AI Engine | WordPress.org | current | https://wordpress.org/plugins/ai-engine/ | P | H |
| 75 | Uncanny Automator; Bit Flows | WordPress.org | current | https://wordpress.org/plugins/uncanny-automator/ ; https://wordpress.org/plugins/bit-pi/ | P | H |
| 76 | Plugin searches: "ai comment reply"; "woocommerce ai customer support"; "content refresh" | WordPress.org | live | https://wordpress.org/plugins/search/ai+comment+reply/ | P | M |
| 77 | YAMM; Mailmeteor; GMass pricing | Google Workspace Marketplace; GMass | current | https://workspace.google.com/marketplace ; https://www.gmass.co/pricing | P/V | H |
| 78 | GPT for Sheets and Docs; pricing | Workspace Marketplace; Talarian | updated 2026-10-15 | https://gptforwork.com/pricing | P/V | H |
| 79 | Marketplace search "ai gmail" | Google Workspace Marketplace | live | https://workspace.google.com/marketplace/search/ai%20gmail | P | M |
