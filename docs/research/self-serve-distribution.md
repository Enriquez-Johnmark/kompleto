# Self-serve distribution: can C or B be sold without a network?

Status: **research, 2026-10-18.** This file adds one constraint to the [scorecard](opportunity-scorecard.md): the owner has **no business network and cannot do customer interviews or sales calls**. Buyers must find, try and buy on their own. It looks at the marketplaces and channels that allow that for **C** (AR collections agent on Xero/QBO) and **B** (PH lead & quote follow-up). It does not rewrite earlier files. Short refs (INT, IND, COMP, PH) are as defined in [opportunity-landscape.md](opportunity-landscape.md).

**Method limits.** The Xero App Store and Xero developer FAQ were read directly as server-rendered HTML (high confidence). Intuit's developer docs and QuickBooks App Store listings render only with JavaScript, the browser extension was not connected, and the web-search budget ran out near the end. So **Intuit facts come from search-engine extracts of Intuit's pages plus third-party summaries (medium confidence), and QuickBooks App Store ratings and review counts were not retrieved.** No number below is estimated unless it is labelled as an estimate.

Labels: **[Primary]** official docs, terms or marketplace page. **[Vendor claim]** a company's own marketing. **[3P]** third-party blog or aggregator. **[Snippet]** official page read only through a search-engine extract. Confidence: H/M/L.

---

## Key conclusions

1. **The Xero App Store is a real self-serve channel for C, but you cannot start there.** Since 2 March 2026 Xero charges developers by connections and data egress. The old revenue share is gone [1][3]. **A listing is "not available" on the Starter and Core tiers.** It is optional from **Plus (AUD 245/mo, up to 1,000 connections)** and needs app certification [1]. Xero's own partner guide says certification requires "at least three customers who love your app" [4]. So the first customers have to come from somewhere else. *H for tiers and fees; M for the three-customer rule, which comes from a pre-2026 PDF and a third-party guide [4][6].*
2. **A Philippine developer can join Xero.** The commercial terms contract with Xero (NZ) Limited and route "New Zealand and the rest of the world" to NZ law. They name no country exclusion [3]. Billing is by credit card [1]. Xero no longer collects subscription payments for apps. Developers bill customers themselves [1], so the Stripe gap for PH entities still matters (INT). *H.*
3. **Organic traction on Xero exists, but it is long-tailed and slow.** In Xero's debtor-management category, Chaser has 374 reviews over 11 years and Paidnice has 83 over about 4.8 years. Most of the other 14–16 apps have 1–18 reviews [7][8]. Trove listed in November 2025 and has 12 reviews and a "2026 Growth App" badge. Solido listed in February 2026 and has 1 review [8]. Reviews are only a proxy for customers. *H for the counts; L as a proxy for revenue.*
4. **C's exact concept is already listed.** Solido's Xero listing says reminders are "drafted against one invoice, and approved before it sends", sent from the customer's own Gmail or Microsoft 365 mailbox [8]. Trove sells "credit control AI" with AI-drafted replies from GBP 50/mo [11]. Chaser and Paidnice market AI too [8][9]. Intuit's own **Payments Agent** in QuickBooks Online (US, from 1 July 2025) creates and sends AI-drafted invoice reminders [18]. **C's competitive room is smaller than the scorecard's 3E assumed.** *H.*
5. **The QuickBooks App Store costs more to get into and is less certain for a PH developer.** The Intuit App Partner Program (US 28 July 2025, then UK/AU/CA from 3 November 2025) has a free Builder tier capped at 500k CorePlus API credits per month. Silver costs US$300/mo [12][13][14]. Intuit's FAQ, as extracted by search, says Builder listings are "subject to removal" from the marketplace unless the partner subscribes to Silver [13]. Every production app also has to pass an app assessment and a security review, whether listed or not [15]. No source says whether a Philippine-based developer can subscribe. *M.*
6. **Neither accounting platform offers a meaningful local (PH) market for C.** Xero launched a US$7/mo "Lite" plan in the Philippines on 20 May 2026. Lite allows only 5 invoices, which is useless for collections [21][22]. Xero's own release puts Filipino MSME digital-accounting adoption at 11% (single cited source) [21]. QuickBooks has a Philippines site [23]. The PH region of the Xero App Store lists only 9 debtor-management apps, against 16–18 in the US, UK and AU [7]. *M.*
7. **B has no equivalent of an app store.** Meta's partner directory is for Business Solution/Tech Providers with volume thresholds [30], not a place where micro-SMEs shop. B's no-call channels are Facebook groups, content, freelancer/agency affiliates and a free tier. The best self-serve precedents are global DIY bot vendors: ManyChat (vendor claim of 1M+ businesses and 25,000+ affiliate partners [31]) and Botcake/Pancake (free plan; vendor claim of 80,000 stores, mainly Vietnam [32][33]). **No PH-specific growth data was found for any of them.** *L–M.*
8. **Regulation is light for first-party B2B reminders, but sending email on a customer's behalf has real rules.** The US FDCPA covers only consumer debts and third-party collectors. Creditors collecting in their own name are excluded [34]. California's Rosenthal Act covers creditors collecting their own *consumer* debts [35]. AU's RG 96 applies to creditors as well as collection agents [36]. UK CONC debt-collecting rules attach to consumer credit [37]. Since February 2024, Gmail requires SPF/DKIM, DMARC with alignment above 5,000 messages/day, and a spam rate below 0.3% [38]. Sending from the customer's own Gmail needs the *sensitive* `gmail.send` scope. Reading replies needs a *restricted* scope, which brings a security assessment [39]. *H.*

---

## 1. Xero App Store

### Requirements and process
- **Tiers (from 2 March 2026)** [1][2] *[Primary, H]*. All fees are monthly, in AUD, excluding tax:

| Tier | Fee | Max connections | Egress included | App Store listing |
|---|---|---|---|---|
| Starter | free | 5 | n/a (1,000 calls/org/day) | not available |
| Core | AUD 35 | 50 | 10 GB | not available |
| Plus | AUD 245 | 1,000 | 50 GB | optional |
| Advanced | AUD 1,445 | 10,000 | 250 GB | optional |
| Enterprise | on application | unlimited | on application | required |

  Egress overage is AUD 2.40/GB. Ingress is unlimited. New apps start on Starter. Adding a 6th connection requires a payment method, which moves the app to Core. Moving beyond Core requires meeting "tier requirements", and connections are capped until you do [1].
- **Certification.** The commercial terms say certification covers connection, branding and naming, scopes, error handling, data integrity and security, account and payment mapping, and tax rates, plus annual recertification [3] *[Primary, H]*. Xero's FAQ ties "App certification" to Core and above [1]. Apideck says it starts at Plus [6] *[3P, M]*. **That conflict is unresolved.** A security assessment, initial and annual, starts at Advanced and covers the premium endpoints [1].
- **Customers before certification.** The partner PDF says to find "at least three customers who love your app" [4] *[Primary but undated, pre-2026, M]*. Apideck says three active connections within 30 days [6] *[3P, M]*.
- **Listing obligations.** Listed apps must keep "Sign up with Xero" working, show local-currency prices, and handle their own billing and disputes [1][3] *[Primary, H]*.
- **AI clause.** API data may not be used "to train or contribute to the creation of any AI or machine learning model". Bots that "simulate user actions" are also banned [1] *[Primary, H]*. Inference on customer data is not prohibited by that text, but the agent design must keep Xero data out of any fine-tuning.

### Fees and revenue share
- There is **no revenue share** any more. Xero says it "will no longer be pricing based on a revenue-share model". Xero App Store Subscriptions billing closed to new apps on 4 December 2025, and existing customers had to be migrated by 1 July 2026 [1] *[Primary, H]*.
- **Effective cost to be listed:** AUD 245/mo for Plus (about US$160 at roughly 0.65 USD/AUD, *estimate*), plus billing infrastructure of your own.

### Timeline
- Certification "can take several months" [6] *[3P, L–M]*. Xero gives no official duration [1].

### Eligibility from the Philippines
- There is no country restriction in the commercial terms. Developers outside AU, UK, US and NZ fall under NZ law [3]. Developers must give Xero their entity and tax details, and pay any withholding tax gross-up [3] *[Primary, H]*.

### Does the store produce organic installs?
- Xero gives **no published conversion or traffic data**. The partner PDF promises reach to "millions of customers and 100,000+ accountants and bookkeepers". It reserves "in-product discovery tools and app recommendations" for premium partners, which needs at least 50 reviews averaging 4 stars and US$10k MRR [4] *[Primary, undated; M]*. The 2026 pricing page reserves merchandising, in-product placements and award eligibility for listed apps on Plus and above [2] *[Primary, H]*.
- **Proxy evidence:** Paidnice describes itself as a "small, bootstrapped team". It won Xero's Global Small Business App of the Year in 2025 [8] *[Vendor claim]*. Search extracts of a Xero devblog spotlight say it grew fast within Xero's Developer Growth Program, with co-marketing and help optimising its App Store presence [40] *[Snippet, L–M; page blocked direct fetch]*. Trove reached 12 reviews and a Growth App badge within 11 months of listing [8]. These suggest that a small newcomer can get store-driven traction. They don't prove it, and they don't show revenue.

---

## 2. QuickBooks App Store (Intuit App Partner Program)

- **Tiers** [12][13][14]. Builder: free, 500,000 CorePlus credits per month, then calls are blocked. Silver: US$300/mo, 1M credits, US$3.50 per 1,000 calls over. Gold: US$1,700/mo; a credit figure of 10M is unconfirmed. Platinum: US$4,500/mo, 75M credits, US$0.25 per 1,000 calls over. Core (data-in) calls are free on every tier [14]. Premium APIs start at Silver [14]. *Tier names and the fee model: [Primary] press release, H. Dollar amounts: [3P], M.*
- **Rollout.** US from 28 July 2025, with variable fees starting 1 November 2025 [14][17]. The program expanded to the UK, Australia and Canada (excluding Quebec) from 3 November 2025 [13] *[Snippet, M]*. Intuit says "geographical availability … varies, and different rates may apply" [13] *[Snippet, M]*. **No source mentions the Philippines.** Treat PH eligibility as unknown until Intuit support confirms it.
- **Listing.** Builder-tier listings "will temporarily remain in place, but [are] subject to removal … To avoid a delisting, you must subscribe to Silver" [13] *[Snippet, M]*. An older version of the FAQ described listing differently, so this point is unsettled.
- **Review.** Any app on production QBO data goes through an app-assessment questionnaire and then a security review. A listing adds technical, security and marketing review. Critical, high and medium security findings must be fixed before publishing. Listed apps, and any app with more than 500 connections, are reviewed every year [15] *[Snippet of Intuit docs, M]*. Intuit publishes no typical duration. One third party estimates 6 weeks to more than 6 months [16] *[3P, L]*.
- **The native competitor.** Intuit's **Payments Agent** in QBO "predict[s] late payments, automate[s] invoice tracking" and creates and sends AI-drafted reminders. Intuit claims customers are paid "an average of 5 days faster". The footnote ties that to a comparison of US Intuit Assist beta users, January–August 2024. It rolled out in the US from 1 July 2025 and is not available in QBO Advanced [18] *[Primary; the 5-day figure is a vendor claim, H/M]*. Xero's JAX chat, by contrast, documents creating invoices but not chasing them [25] *[Primary, H, as of this page]*.
- **Competitor ratings on the QuickBooks App Store: not retrieved.** The listings are JavaScript-only. Chaser has 4.9 from 45 reviews on Capterra [Snippet, L]. That is not an Intuit figure.

---

## 3. Other self-serve marketplaces

| Marketplace | Fit | Listing requirement | Fees | Organic installs? |
|---|---|---|---|---|
| **HubSpot App Marketplace** | Weak for C (AR lives in accounting tools). Possible later for B2 | ≥3 active installs from unaffiliated production accounts in the last 30 days. Initial review within 10 business days, full process ≤60 days [26] *[Primary, H]*. Certification needs 60 installs and 6 months listed [27] *[Snippet, M]* | No listing fee or revenue share stated on the requirements page [26] | No data found |
| **Zoho Marketplace** (Zoho Books/CRM) | Possible secondary channel for C; Zoho Books is cited as popular in PH (vendor page) | Free developer console | Zoho collects payment and pays the developer a "commission" monthly; rate not found [28] *[Snippet, M]* | No data found |
| **GoHighLevel Marketplace** | Relevant to B2 only (US/AU agencies). Not B | Apps can be free, freemium or paid. Snapshot listings get a 7–10 day white-label review [29] *[Snippet, M]* | Revenue share **not found** | Not found |
| **Meta Business Partner directory** | Not a channel for B's micro-SME buyers | Solution Partner/Tech Provider path. One third party cites ≥10 clients and ≥2,500 average daily conversations [30] *[3P, L]* | n/a | n/a |
| **Shopify App Store** | Not relevant: neither C nor B targets e-commerce merchants | Not researched | Not researched | n/a |

---

## 4. Xero and QuickBooks adoption in the Philippines and SEA

- **Xero group:** 4.9M subscribers at FY26 (to 31 March 2026, up 11%) [20] *[Press, M]*. "Rest of World", meaning everything outside ANZ, the UK and North America, had 308k subscribers at H1 FY26 (30 September 2025) [19] *[Primary deck via snippet, M]*. **No Philippines or SEA subscriber figure was found.**
- **Xero Philippines:** Lite plan at US$7/mo, launched 20 May 2026 alongside Indonesia. Lite includes App Store access [21]. For more than 5 invoices, the Starter plan is needed [22]. The release cites 60% aspiration and 11% adoption of digital accounting software among Filipino MSMEs, sourced to an Inquirer Business article [21] *[Vendor claim, L for the statistic]*. The Xero App Store has a PH region [7].
- **QuickBooks Philippines:** Intuit runs a PH site with a +63 support number and PHT hours. It lists Simple Start, Essentials and Plus, but the page shows no PH prices [23] *[Primary, H that the market is served]*. QuickBooks Online Accountant was made free in about 170 countries, including PH, in 2023 [24] *[Press release, M]*.
- **Implication:** PH digital-accounting adoption is low and Xero's PH entry product has a 5-invoice cap. Philippine businesses selling on payment terms *and* using Xero/QBO are a thin segment. **C stays an international product.** Selling it locally is at most a side channel. *M.*

---

## 5. B: evidence that no-call acquisition works for SMB messaging tools in PH/SEA

The evidence is thin and nearly all vendor-made:
- **ManyChat:** the official partner page advertises up to 50% commission for 12 months [31] *[Vendor claim via snippet, M]*. A 2026 job post claims "more than 25,000" affiliate partners and "more than one million companies" [31] *[Vendor claim, L–M]*. BuiltWith detects ManyChat on only 78 PH websites [41] *[3P, L; a website count is a poor proxy for Messenger bots]*. ManyChat's Trustpilot score is 2.1/5 (COMP), so self-serve volume did not need high satisfaction.
- **Botcake/Pancake** (Vietnam, with a PH partner site, COMP): free plan, 24-hour-window auto-replies free up to 500k/mo [32] *[Vendor, M]*. The Pancake app listing claims more than 80,000 stores, mainly Vietnam [33] *[Vendor claim, L]*.
- **No PH-specific data** was found for template marketplaces, Facebook-group-driven growth or agency programs (PH §"Facebook groups" was already flagged as a gap).
- **What this does show:** in SEA, DIY messaging tools grow through a free tier, affiliates or freelancers who set them up for SMEs, and Facebook seller communities. None of them relies on founder sales calls. **What it does not show:** that a *paid, ₱1,500–4,000/mo follow-up agent* can convert PH micro-SMEs without a human demo. Meta Business Agent is also free for the inbound part (COMP §3.1).

---

## 6. Regulatory and practical issues for an AR reminder agent

- **US FDCPA.** "Debt" covers only obligations "primarily for personal, family, or household purposes". A "debt collector" collects debts "owed or due … another". Officers and employees collecting "in the name of the creditor" are excluded. **But** a creditor using "any name other than his own which would indicate that a third person is collecting" is covered [34] *[Primary statute, H]*. **Design rule:** reminders must go out in the business's own name and from its own domain or mailbox, never branded as the app. B2B invoices fall outside the FDCPA entirely.
- **California Rosenthal Act:** covers creditors collecting their *own* consumer debts and incorporates most FDCPA prohibitions [35] *[3P legal summary, M]*. This matters only if customers invoice consumers, such as clinics or trades.
- **Australia:** the ACCC/ASIC debt collection guideline (RG 96, April 2021) applies to "creditors who are directly involved in debt collection" as well as agencies, and covers contact frequency and hours [36] *[Primary, H for scope; whether it reaches B2B debts was not confirmed]*.
- **UK:** debt collecting is a credit-related regulated activity under CONC 7 when it relates to consumer credit agreements [37] *[Primary, H]*. Ordinary B2B trade invoices appear to be out of scope. *Inference, M; not verified against the Regulated Activities Order.*
- **Email deliverability** [38] *[Primary, H]*:
  - **All senders to Gmail:** SPF or DKIM, valid forward and reverse DNS, TLS, spam rate below 0.3% in Postmaster Tools, and no impersonation of Gmail From: headers.
  - **Senders of 5,000+ messages a day:** SPF *and* DKIM, plus DMARC (p=none is enough) with the From: domain aligned to SPF or DKIM. One-click unsubscribe applies to marketing and subscribed mail, not transactional reminders.
  - **Platforms sending for clients:** Google says an email provider is "responsible for their sending practices".
  - Yahoo announced parallel 2024 rules. *They were not re-verified in this pass.*
- **Two practical sending models:**
  - **(a) Send from the customer's own mailbox** (Solido, Chaser [8]). Deliverability is good and replies thread naturally. Sending needs `gmail.send` (sensitive: OAuth verification, no assessment). Reading replies needs `gmail.readonly` or `gmail.modify`, which are restricted and bring a security assessment if data is stored or transmitted [39]. That assessment was estimated at about US$540–720/yr (IND [6]). Microsoft 365 needs the equivalent (not researched).
  - **(b) Send through your own ESP on the customer's domain.** Each customer has to add SPF/DKIM DNS records, which means a self-serve onboarding hurdle and support load.

---

## AR/collections apps on the Xero App Store (snapshot 2026-10-18)

Ratings and review counts come from apps.xero.com and are the same across regional pages [7][8]. Prices are the vendor's or listing's "from" prices. QBO ratings were **not retrieved**.

| App | Marketplaces | Xero rating | Xero reviews | Listed since | From price | AI / notable features |
|---|---|---|---|---|---|---|
| Chaser | Xero, QBO (vendor says) | 4.98 | 374 | May 2015 | US$259 / £199 /mo (Compact, revenue <£4M) [9] | AI email generator, AI debtor risk insights; sends from own email [8][9] |
| Paidnice | Xero, QBO [10] | 5.0 | 83 | Dec 2021 | US$69/mo (150 invoices, 600 emails) [10] | "Credit control AI"; late fees, payment plans; MCP server on Pro [8][10] |
| CreditorWatch Collect (formerly Debtor Daddy) | Xero | 4.89 | 91 | Dec 2012 | not retrieved | Software plus outsourced collections [8] |
| Satago | Xero | 4.93 | 92 | Feb 2014 | not retrieved | Reminders, credit risk, invoice finance [8] |
| ezyCollect (Sidetrade) | Xero | 4.94 | 35 | Apr 2015 | not retrieved | Email/SMS/call reminders, credit checks [8] |
| Kolleno | Xero | 5.0 | 18 | Apr 2022 | not retrieved | "AI & Machine Learning" for timing, tone and channel [8] |
| Statey | Xero | 4.76 | 17 | — | not retrieved | Automated statements [7] |
| Trove | Xero | 5.0 | 12 | Nov 2025 | GBP 50 / AUD 95 /mo [11] | "Credit control AI"; AI-drafted email replies; Xero 2026 Growth App [8][11] |
| Apxium | Xero (UK/AU) | 5.0 | 12 | — | not retrieved | — [7] |
| GetPaid | Xero (UK) | 5.0 | 28 | — | not retrieved | — [7] |
| Nagging Panda | Xero | 5.0 | 7 | — | not retrieved | Statements, reminders, late fees [7] |
| Adfin | Xero (UK) | 5.0 | 6 | — | not retrieved | Payments + chasing [7] |
| Solido | Xero | 5.0 | 1 | Feb 2026 | tiers by email volume (100/500/2,000 emails per month); price not shown | **AI-drafted per invoice, approved before sending, sent from own Gmail/M365**: C's exact concept [8] |
| Upflow | Xero | 5.0 | 1 | Oct 2021 | not retrieved | Mid-market AR + payments [8] |
| CollBox, Routable, ConnectBooster, EasyInterest, Garfield, others | Xero | 3.0–5.0 | 2–3 each | — | not retrieved | Assorted [7] |

**Reading the table:** two incumbents hold most of the reviews. Newcomers from 2025–2026 that sell "AI" (Trove, Solido) sit at 1–12 reviews. The category has 16–18 apps in mature regions and 9 in PH [7].

---

## Implications for finalist C vs B under a no-network, no-interview constraint

**C (AR collections agent)**
- *Better than assumed:* buyers of C really do browse an in-product app store, read reviews and start free trials without a call. Every competitor sells through a free trial (Chaser 10 days; Trove and Solido 30 days) [8]. Xero no longer takes a revenue share and accepts developers from anywhere [1][3]. **That is the self-serve channel B lacks.**
- *Worse than assumed:*
  - **The listing is gated.** It needs certification plus about 3 real customers [4][6] and the AUD 245/mo Plus tier [1]. The first users must come from outside the store: SEO or content, the Xero community and forums, or Product Hunt.
  - **The exact product already exists,** listed by Solido, with AI-first entrants (Trove) and AI-marketed incumbents [8]. On QBO, Intuit's Payments Agent does AI-drafted reminders natively [18].
  - **QBO's paid listing is US$300/mo,** and PH eligibility is unconfirmed [13].
  - **Self-billing is needed without Stripe** (INT).
- **My view on C7 Reachability: raise it from 2J to 3J. Do not go to 4.** The "bookkeeper network" route the scorecard assumed is closed, but a marketplace-plus-trial motion that needs no calls exists. Its proven ceiling is slow: tens of reviews a year for good newcomers. The entry costs before any listing are three customers found off-store, and AUD 245/mo before revenue. **At the same time, C6 Competitive room should fall from 3E to 2E,** because of Solido, Trove and Intuit's native agent. Each point is worth 1.6 under Balanced weights, so **C's total stays at about 71.8.** The case for C gets more reachable and less differentiated. It does not get stronger.

**B (PH follow-up agent)**
- B's C7 = 3J assumed local presence, Facebook groups and interviews. Without interviews and calls, B still has:
  - Facebook groups (self-serve posting, within group rules);
  - Taglish content;
  - a free tier;
  - commission for freelance page-admins and agencies (a ManyChat-style affiliate program [31]).
- B has no marketplace with buyer intent. PH micro-SMEs buying a ₱1,500–4,000/mo tool usually expect hand-holding, though asynchronous Messenger chat support is not a "sales call". Meta also gives the inbound bot away free.
- **I would hold B's C7 at 3J with low confidence, or drop it to 2J.** If it drops, B falls to 72.4, just ahead of C at 71.8 (with C6 lowered and C7 raised). If C7 rises and C6 is left unchanged, C reaches 73.4 and would edge ahead of B at 72.4.

**Bottom line:**
- The no-call constraint narrows the gap between B and C rather than settling it.
- It makes **"which channel can a stranger buy through?"** the deciding question.
- For C the answer is concrete but crowded: the Xero App Store after certification.
- For B the answer is plausible but unproven: Facebook groups, a free tier and affiliates.

**Cheapest next tests, with no calls:**
- **C:** build a Starter-tier (free, 5 connections) Xero app. Try to get 3 certification users from forum and content posts within 30 days. If you can't, C fails the no-network test.
- **B:** post a free-tier offer in 3–5 PH business Facebook groups and measure sign-ups and activation with no demos.

---

## Sources

All accessed 2026-10-18.

| # | Title | Publisher | Date | URL | Type | Conf. |
|---|---|---|---|---|---|---|
| 1 | Pricing and Policy Updates FAQs | Xero Developer | announced 4 Dec 2025; current | https://developer.xero.com/faq/pricing-and-policy-updates | Primary | H |
| 2 | Tier Fees, Connection Limits, and API Volume Usage | Xero Developer | current | https://developer.xero.com/pricing | Primary (table layout partly ambiguous) | H |
| 3 | Xero Developer Platform Commercial Terms | Xero (NZ) Ltd | last updated 4 Dec 2025 | https://developer.xero.com/xero-developer-platform-commercial-terms | Primary (terms) | H |
| 4 | Grow with the Xero App Store (PDF) | Xero Developer | undated, pre-2026 model | https://developer.xero.com/static/otherfiles/Grow-with-the-Xero-App-Store.pdf | Primary, possibly outdated | M |
| 5 | Xero certification checkpoints | Codat | current | https://docs.codat.io/integrations/accounting/xero/partner-certification/checkpoints-app-store | 3P summary of Xero docs | M |
| 6 | Xero API Pricing and the App Partner Program | Apideck | 16 Jan 2026 (updated Oct 2026) | https://www.apideck.com/blog/xero-api-pricing-and-the-app-partner-program | 3P | M |
| 7 | Debtor management apps (US/UK/AU/PH regions) | Xero App Store | snapshot 2026-10-18 | https://apps.xero.com/us/function/debtor-tracking (also /uk/, /au/, /ph/) | Primary marketplace | H |
| 8 | App listings: Chaser, Paidnice, Trove, Solido, CreditorWatch Collect, Satago, ezyCollect, Kolleno, Upflow | Xero App Store | snapshot 2026-10-18 | https://apps.xero.com/us/app/{chaser,paidnice,trove,solido,debtor-daddy,satago,ezycollect,kolleno,upflow} | Primary marketplace; descriptions are vendor claims | H |
| 9 | Chaser pricing | Chaser | current | https://www.chaserhq.com/pricing | Vendor | H |
| 10 | Paidnice pricing | Paidnice | current | https://www.paidnice.com/pricing | Vendor | H |
| 11 | Trove listing, UK pricing plans | Xero App Store | current | https://apps.xero.com/uk/app/trove | Primary (partner-supplied prices) | H |
| 12 | Intuit Launches New App Partner Program… | Intuit / Business Wire | 28 Jul 2025 | https://investors.intuit.com/_assets/_08ee5483ec4c057568cc8774f3fd6aad/intuit/news/2025-07-28_Intuit_Launches_New_App_Partner_Program_to_Drive_1261.pdf | Primary press release | H |
| 13 | Intuit App Partner Program FAQ | Intuit Developer | current | https://developer.intuit.com/app/developer/qbo/docs/get-started/partner-faq | Primary, read via search snippets | M |
| 14 | QuickBooks API Pricing and the Intuit App Partner Program | Apideck | 20 Sep 2025, updated 6 Oct 2026 | https://www.apideck.com/blog/quickbooks-api-pricing-and-the-intuit-app-partner-program | 3P | M |
| 15 | What to expect during the app review process; Maintaining compliance; Security requirements | Intuit Developer | current | https://developer.intuit.com/app/developer/qbo/docs/go-live/list-on-the-app-store/what-to-expect-during-the-review | Primary, via snippets | M |
| 16 | How long does it take to publish your app in the Intuit App Store? | Satva Solutions | 2026 | https://satvasolutions.com/blog/intuit-app-store-approval-timeline-developer-guide | 3P | L |
| 17 | Intuit partner program changes | Codat | 3 Jun 2025 | https://docs.codat.io/updates/250603-intuit-partner-program-changes | 3P | M |
| 18 | Intuit Introduces … Virtual Team of AI Agents | Intuit | 1 Jul 2025 | https://investors.intuit.com/news-events/press-releases/detail/1258/intuit-introduces-ground-breaking-virtual-team-of-ai-agents-to-fuel-growth-for-businesses | Primary; outcome figure is a vendor claim | H/M |
| 19 | Investor Deck H1 FY26 | Xero | Nov 2025 | https://brandfolder.xero.com/NE531UQB/as/ks8qbt4tj62mb8j6hpq6mz/Investor_Deck_H1_FY26_Xero | Primary, via snippet | M |
| 20 | Xero posts 31% jump in full-year revenue | Capital Brief | May 2026 | https://www.capitalbrief.com/briefing/xero-posts-31-jump-in-full-year-revenue-as-melio-growth-accelerates-88005c35-c976-4785-bad7-21212346943e/ | Press, via snippet | M |
| 21 | Xero launches Lite plan in the Philippines (press release) | Xero via ACN Newswire | 20 May 2026 | https://www.acnnewswire.com/press-release/english/107201/ | Vendor release | H (launch) / L (11% stat) |
| 22 | Xero Lite pricing, Philippines | Xero | current | https://www.xero.com/ph/pricing-plans/lite/ | Vendor, via snippet | M |
| 23 | Accounts Receivable Software, Intuit QuickBooks Philippines | Intuit | current | https://quickbooks.intuit.com/ph/accounts-receivable/ | Primary | H |
| 24 | QuickBooks Online Accountant in 170+ countries incl. the Philippines | Intuit / Business Wire | 8 May 2023 | https://www.businesswire.com/news/home/20230508005038/en/ | Press release, via snippet | M |
| 25 | JAX | Xero | current | https://www.xero.com/us/jax/ | Primary | H |
| 26 | App Marketplace listing requirements | HubSpot Developers | current | https://developers.hubspot.com/docs/apps/developer-platform/list-apps/listing-your-app/app-marketplace-listing-requirements | Primary | H |
| 27 | Active install requirement increase for app certification | HubSpot Developers | 2024 | https://developers.hubspot.com/changelog/active-install-requirement-increase-for-app-certification | Primary, via snippet | M |
| 28 | Managing payments (Zoho Marketplace) | Zoho | current | https://www.zoho.com/developer/help/marketplace/payments.html | Primary, via snippet | M |
| 29 | Marketplace modules: Snapshots | HighLevel | current | https://marketplace.gohighlevel.com/docs/marketplace-modules/Snapshots/index.html | Primary, via snippet | M |
| 30 | Solution Providers (WhatsApp); Taqnyat BSP blog | Meta; Taqnyat | current | https://developers.facebook.com/docs/whatsapp/solution-providers ; https://blog.taqnyat.sa/en/post/meta-bsp/ | Primary via snippet (M); 3P (L) | M/L |
| 31 | Manychat affiliate program; Head of Affiliate Partnerships job post | Manychat | 2026 | https://manychat.com/affiliate ; https://jobs.summitpartners.com/companies/manychat/jobs/67283198-head-of-affiliate-partnerships | Vendor claims, via snippet | M/L |
| 32 | Botcake pricing | Botcake | current | https://botcake.io/pricing | Vendor, via snippet | M |
| 33 | Pancake POS (App Store listing) | Pancake | current | https://apps.apple.com/app/id6443717170 | Vendor claim, via snippet | L |
| 34 | 15 U.S.C. § 1692a, Definitions | Cornell LII | current | https://www.law.cornell.edu/uscode/text/15/1692a | Primary statute | H |
| 35 | California Rosenthal Fair Debt Collection Practices Act | HOPB (law firm) | current | https://www.hopb.co/california-rosenthal-fair-debt-collection-practices-act | 3P legal summary | M |
| 36 | RG 96 Debt collection guideline for collectors and creditors | ASIC/ACCC | 13 Apr 2021 | https://asic.gov.au/regulatory-resources/find-a-document/regulatory-guides/rg-96-debt-collection-guideline-for-collectors-and-creditors/ | Primary, via snippet | H (scope) |
| 37 | CONC 7.1 | FCA Handbook | current | https://handbook.fca.org.uk/handbook/conc7/conc7s1 | Primary, via snippet | H/M |
| 38 | Email sender guidelines | Google Workspace Admin Help | current (rules from 1 Feb 2024) | https://support.google.com/a/answer/81126 | Primary | H |
| 39 | Choose Gmail API scopes | Google for Developers | current | https://developers.google.com/workspace/gmail/api/auth/scopes | Primary | H |
| 40 | Developer tips for success: Paidnice spotlight | Xero DevBlog (Medium) | undated | https://devblog.xero.com/developer-tips-for-success-paidnice-spotlight-c74aa7bd39f5 | Primary, via snippet (direct fetch blocked) | L–M |
| 41 | ManyChat usage statistics | BuiltWith | current | https://trends.builtwith.com/widgets/ManyChat | 3P | L |
