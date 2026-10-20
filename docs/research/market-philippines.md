# Market research: the Philippines as a first market

Researched 2026-10-18. Scope: the Philippine market for an AI agent or automation product (inquiry handling on messaging channels, lead follow-up, booking and reminders, quotation follow-up, document intake) built by a Philippine-based solo developer or small team. Named competitors are covered in a separate document; competitors appear here only as price evidence.

Notation: `[S#]` refers to the numbered Sources list at the end. Confidence: **H** high, **M** medium, **L** low. Anything marked **estimate** or **hypothesis** is my reasoning, not a sourced figure. Currency conversions assume **₱57–59 per US$1**. I did not verify the October 2026 exchange rate, so treat that as an assumption.

Research limit: the shared web-search budget ran out near the end of the session. Several items in "Evidence gaps" were left open because of that, not because nothing exists.

---

## Key conclusions

1. **The formal SME base is small, and most of it is micro.** The PSA 2023 List of Establishments counts 1.246M establishments. 1.127M (90.4%) are micro (1–9 workers) and only 114.7k are small or medium [S2]. The service sectors that fit this product are a fraction of that: health 32.4k, education 15.0k, admin and support (including travel agencies) 17.4k, real estate 18.3k, and other services such as beauty 87.2k MSMEs [S2]. **(H)**
2. **Messenger is the default inquiry channel and has no close rival.** Facebook ad reach is 95.8M, which is 97.7% of internet users. Messenger reaches 65.8M, or 83.3% of adults 18+, the highest Messenger reach of any country [S3]. Viber, WhatsApp, SMS and email are secondary, and I found no independent 2025–2026 user counts for Viber or WhatsApp in the Philippines. **(H for Facebook and Messenger; L for the others)**
3. **Meta now offers a native, nearly free competitor in this exact channel.** Meta launched "Business AI on Messenger" in the Philippines on 20 Oct 2025 and called the country "a key market for testing" [S18]. In June 2026 it took the Meta Business Agent global. The agent answers questions, books appointments, qualifies leads and hands off to humans, and SMBs get it bundled into WhatsApp Business Premium tiers [S19, S21]. A plain FAQ auto-reply for Messenger is therefore commoditized. Any product needs value beyond the chat itself, such as calendar and deposit handling, quotation follow-up across channels, staff handoff and reporting. **(H that it exists; M on pricing and reach in the Philippines)**
4. **There is evidence of a price ceiling, and it sits at about one quarter of US pricing.** One AI receptionist for dental clinics charges Philippine clinics ₱750 (text only), ₱1,990 (Solo), ₱3,900 (Clinic) and ₱8,900+ (Multi-branch) per month. It charges clinics outside the Philippines US$149, $249 and $499 [S41]. At ₱58 per US$1, the Philippine Solo tier is about US$34, roughly 23% of the US$ tier. A local dental SaaS lists ₱1,000–2,000 per month [S42]. My working ceiling for SME tools is ₱1,000–4,000 per month (US$17–70), with ₱5,000–9,000 only for multi-branch customers. **(M: few data points, all vendor-listed)**
5. **The human alternative is cheap.** Provincial minimum wage works out to about ₱13,700–16,800 per month: Davao ₱540/day, Central Visayas ₱582, CALABARZON ₱645 from 26 Oct 2026. NCR is about ₱19,700 (₱755/day) [S27]. Virtual assistants average ₱27,231 per month on Indeed [S28]. A ₱2,000 tool equals about 10–15% of one provincial staffer. Most micro firms have no dedicated inquiry staffer, though: the owner answers the chats. The pitch has to be about missed or slow replies costing bookings, not about replacing a salary. **(H for wages; the framing is a hypothesis)**
6. **Interest in AI is high, but paid, integrated use is low.** The AWS-sponsored survey reports 32% of businesses "use AI" in 2026, up from 21% in 2025, but 70% of adopters are at the basic stage (public chatbots, productivity tools) and only 12% at the advanced stage [S7]. PIDS, a government think tank, finds 14.9% of firms use AI tools and puts awareness at about one in five firms. Adoption is concentrated in large urban ICT and BPO firms [S4]. Individuals are heavy AI users: 42.4% of internet users used ChatGPT in the past month, 6th highest worldwide [S10]. **(M: the sources disagree, and the higher figures come from vendor surveys)**
7. **Payments are workable but card-light, and recurring billing is the weak spot.** 36% of adults have e-money accounts and 23% have bank accounts (BSP 2025 CFIS) [S22]. PayMongo charges 2.23% for GCash, 1.79% for Maya and 3.125% + ₱13.39 for local cards [S24]. Its recurring-billing product needs an eligibility review and is card-centred [S25]. Stripe does not list the Philippines as a supported country [S26]. Expect manual monthly collection by GCash or payment link, plus churn from failed renewals. **(H for fees; M for recurring-billing coverage)**
8. **Regulation is manageable but real, especially for clinics.** The Data Privacy Act applies, and NPC Advisory 2024-04 (19 Dec 2024) applies it to AI. It requires telling data subjects about AI processing, its purposes and risks, plus fairness and bias monitoring and accountability [S30, S31]. Health data is sensitive personal information. Clinics holding sensitive data on 1,000+ individuals hit the NPC registration threshold (Circular 2022-04) [S32]. As of July 2026 no AI law had been enacted, and bills such as the AI Development and Regulation Act are pending [S33]. **(M–H)**
9. **Depending on Facebook is a material risk.** In August 2026 the government publicly considered a nationwide Facebook block. DICT called it a "last resort" [S14]. Later reports describe a Meta–government arrangement taking a block off the table, but those come from low-reliability sources [S17]. Design for Messenger first, with Viber, SMS or a web fallback. **(M)**
10. **Clinics (medical and dental) and travel and tour operators look like the most reachable first segments. Review and tutorial centers come third.** All three show high-volume, repetitive inquiries arriving by Messenger, have a clear transaction (appointment, quote and deposit, enrollment), and have an existing Philippine-priced reference product (clinics). My estimated reachable base is roughly 3k–8k clinics, 1k–3.5k formal travel and tour operators plus an informal tail, and 0.6k–1.8k review and tutorial centers. That is a ₱50M–400M per year serviceable market for clinics, or about US$1–7M (**estimate**, section 11). Philippine-only economics support a small, profitable business, not a venture-scale one. Testing the same product in USD markets early is justified by the roughly 4x price gap (section 10). **(estimate; L–M)**

---

## 1. SME landscape

**Counts and size distribution (official).** The latest table-level data is the PSA **2023** List of Establishments, published by DTI as "2023 Philippine MSME Statistics in Brief" (as of 22 Nov 2024) [S2]:

| Size (by employment) | Establishments | Share | Employment |
|---|---|---|---|
| Micro (1–9) | 1,127,058 | 90.43% | 3,219,402 |
| Small (10–99) | 109,912 | 8.82% | 2,490,210 |
| Medium (100–199) | 4,763 | 0.38% | 641,854 |
| **Total MSMEs** | **1,241,733** | **99.63%** | 6,351,466 (66.97%) |
| Large (200+) | 4,640 | 0.37% | 3,132,499 |
| **All** | **1,246,373** | | 9,483,965 |

DTI also has a "2024 Philippine MSME Statistics" page [S1]. It reports 1,241,476 total businesses, 1,236,908 MSMEs (99.63%), 1,125,476 micro (90.66%), 106,799 small and 4,633 medium. Those figures came from infographic alt text that I could not verify, and the page's total is close to the 2023 MSME count, so they may be mislabelled. **Confidence: H for the 2023 table, M for the 2024 page.** Limitation: the List of Establishments covers establishments at a fixed location. It misses the large informal and home-based sector, including many Facebook-only sellers and freelance travel resellers.

**Growth.** Establishments rose from 1,080,810 (2021) to 1,109,684 (2022) to 1,246,373 (2023) [S2]. The 12% jump in 2023 partly reflects post-pandemic recovery and possibly a coverage change. **(H for the figures; L for the explanation)**

**Sector distribution, 2023 MSMEs, selected sections** [S2]:

| PSIC section | MSMEs | Relevance |
|---|---|---|
| G Wholesale and retail trade | 604,313 | Retail sellers. A messaging use case, but low-ticket |
| I Accommodation and food | 190,899 | Resorts and restaurants (reservations) |
| C Manufacturing | 140,180 | Low fit |
| S Other service activities | 87,197 | Salons, spas, laundries, repair. Beauty and wellness lives here |
| K Financial and insurance | 52,924 | Pawnshops, lending, insurance agents |
| Q Human health and social work | 32,378 | Medical and dental clinics, labs, diagnostics |
| R Arts, entertainment, recreation | 21,330 | Events, gyms (partly) |
| L Real estate | 18,297 | Brokers and leasing firms. Individual agents are not counted |
| N Administrative and support | 17,401 | Includes travel agencies and tour operators (PSIC N79), alongside security, manpower and cleaning |
| M Professional, scientific, technical | 17,117 | Agencies, consultancies |
| P Education | 14,986 | Private schools, review and tutorial centers |
| J Information and communication | 11,103 | Small digital agencies |

The DTI 2024 infographic gives sector shares: retail 48.74%, accommodation and food 15.41%, manufacturing 11.27%, other services 7.04% [S1]. **(H)**

**Regional concentration (2023 establishments)** [S2]: NCR 223,705 (18.0%), CALABARZON 190,480 (15.3%), Central Luzon 156,091 (12.5%), Central Visayas 88,468 (7.1%), Western Visayas 85,644, Davao 67,918. NCR, CALABARZON and Central Luzon together hold about 46% of establishments. NCR has the most small firms (31,941), and small firms are more likely to have someone dedicated to handling inquiries. **(H)**

---

## 2. Digitization and AI adoption

| Measure | Figure | Source / type | Conf. |
|---|---|---|---|
| Establishments with internet access | 81%; with computers 90.8% (PSA data cited by PIDS) | [S4] official, via research | M (data year not stated) |
| Firms using AI tools | 14.9%. Awareness about one in five firms. ICT 7.19%, BPO 5.94% | [S4, S5] PIDS primary research | M |
| Businesses "using AI" (2025) | 21% (~250k). 78% of adopters at the basic stage, 8% fully integrated | [S6] **AWS-sponsored** (Strand Partners, n=1,000 business leaders) | M– |
| Businesses "using AI" (2026) | 32% (~396k). 70% basic, 18% multi-function, 12% advanced or agentic. Barriers: skills 47%, technical or data 39%, cost 36% | [S7] **AWS-sponsored** | M– |
| Knowledge workers using AI at work | 86% (global 75%) | [S8] **Microsoft/LinkedIn**, 2024 | M– |
| Internet users using ChatGPT in the past month | 42.4%, 6th worldwide | [S10] GWI Q2 2025 via We Are Social/Meltwater | M |
| Digital users relying on AI-powered tools | 78% | [S9] e-Conomy SEA 2025 (Google/Temasek/Bain), via news | L–M |
| Digital economy | US$36B GMV in 2025 (+16%). Digital payments GTV US$150B. 57% of retail payment volume is digital. 475k video-commerce sellers | [S9] | M |

**Stated interest vs actual adoption.** The figures stack in layers:
- Individuals and employees use free AI heavily (42–86%).
- About one fifth to one third of businesses say they "use AI", mostly free public chatbots.
- About 12–18% have integrated it into business functions.
- The government-research baseline for firm-level AI tools is 14.9% and falls lower among MSMEs [S4, S7].

None of these sources measures **paid** AI-tool adoption by MSMEs (see gaps). The secondary claim that "77% of MSMEs are eager to adopt digital tools but 16% use them" (attributed to a DTI roundtable) could not be traced to a primary source, so it is excluded. **Implication (hypothesis):** owners are familiar with chatbots from personal use, so "AI" will not scare them. Willingness to pay a subscription is the binding constraint, not awareness.

---

## 3. Communication channels

**Platform reach, late 2025 (DataReportal Digital 2026 Philippines, 5 Nov 2025)** [S3]. These are platform ad-audience figures, which may not be unique people, and the publisher warns of platform corrections.

| Platform | Reach | Notes |
|---|---|---|
| Facebook | 95.8M; 97.7% of internet users | +10.3% year on year |
| Messenger | 65.8M; 83.3% of adults 18+; 67.1% of internet users | Highest Messenger reach of any country (Statista, citing the same source) |
| TikTok | 64.0M adults 18+ | Fell 6.9% Jul–Oct 2025 |
| YouTube | 59.6M | |
| Instagram | 26.9M | +24.6% year on year, growing fastest |
| LinkedIn | 22.0M members | Matters for B2B and agency buyers |

Internet users number 98.0M (83.8%), mobile connections 137M, and the median mobile download speed is 59.6 Mbps [S3]. **(H)**

**Viber, WhatsApp, SMS.** I found no independent 2025–2026 penetration figures. Rakuten Viber, the vendor, reports a 65% increase in new Business Accounts in the Philippines in 2025, with interest from "technicians, consultants, health and beauty" [S13]. **(L: vendor claim)** In a 2025 survey of 265 Baguio dentists, 67.5% used Facebook and 8.7% Viber. Messenger was not asked about [S40]. **(M for Baguio only)** WhatsApp appears to be a minority channel used mostly by people dealing with foreign clients and OFW families (**hypothesis**, not verified).

**Business messaging behaviour.** A 2022 Meta/BCG study, **Meta-sponsored**, reported:
- 71% of Filipinos surveyed had adopted business messaging.
- 97% of Philippine businesses see messaging apps as important.
- Three in five consumers chat with businesses at least every two to three weeks [S11, S12].

**(M–: dated and sponsored)** Meta's own launch materials describe the Philippines as central to "conversational commerce" [S18].

**How SMEs take inquiries and bookings today.** I found no systematic survey (gap). The available evidence:
- (a) Seller shorthand and "bump"/"HM" (how much) culture on Facebook. PayMongo's guide advises sellers to pre-answer price, availability, payment and shipping questions to cut back-and-forth [S51]. **(L–M, vendor blog)**
- (b) Payment proof by GCash screenshot is common enough that GCash's operator warned in July 2025 about AI-generated fake receipts. Payment-link vendors pitch themselves against "screenshots lost in chat threads" [S53]. **(L–M)**
- (c) Anecdote: a milk-tea business takes 70–80% of delivery orders via Messenger [S12]. **(L)**

**Hypothesis to validate in interviews:** a Facebook Page, Messenger replies typed by the owner or a staffer, a Google Sheet or notebook for the schedule, GCash or bank transfer with a screenshot for the deposit, and Viber or SMS for reminders.

**Platform risk.** The CICC proposed considering a nationwide Facebook block after a livestreamed shooting in August 2026. DICT Secretary Aguda called a ban a "last resort" [S14]. Reports estimate 500k–800k+ registered small businesses rely on Facebook [S15] (the figures' origin is unclear). Later blog reports describe a Meta–government deal (a local office and a rapid-takedown channel) that took the block off the table [S17]. **(M that the risk was real; L on the current resolution)**

---

## 4. Purchasing behaviour and price points

**SME software budgets.** No Philippine survey quantifying MSME software budgets or willingness to pay in pesos was found (gap). PIDS and the AWS survey both list cost and funding among the top barriers (cost 36%) [S4, S7]. **(M)**

**Observed price anchors (all vendor-listed, not transaction data):**

| Product | Philippine price | Comparison | Source |
|---|---|---|---|
| AI dental receptionist, Philippine plans | ₱750 text-only; ₱1,990 Solo; ₱3,900 Clinic (2–3 dentists); from ₱8,900 Multi-branch; extra dentist ₱990; yearly = two months free | US$149 / $249 / $499 for non-Philippine clinics. The Philippine price is about 23–27% of US$ | [S41] (updated 5 Oct 2026) |
| Local dental clinic SaaS | ₱1,000–2,000/mo | | [S42] (L, unverified vendor page) |
| PayMongo Storefront | ₱349/mo | | [S24] |
| Google Workspace via a Philippine reseller | ₱280–336/user/mo, prepaid | Google list US$7 annual / $8.40 flexible | [S43] |
| AI receptionists, US SMB | ~US$30–300/mo, typically $50–150 | | [S50] (vendor blogs) |

**Working price-ceiling estimate (estimate, M–):**
- Micro single-location businesses: ₱500–1,500/mo.
- Small clinics, agencies and travel operators: ₱1,500–4,000/mo.
- Multi-branch: ₱5,000–10,000/mo.
- Above roughly ₱5,000, buyers will compare against hiring a part-time staffer (about ₱8–15k).

**Monthly vs annual.** No survey found. Vendors offer annual plans with a "two months free" discount [S41]. Resellers require prepayment [S43]. **Hypothesis:** monthly is the default given cash-flow constraints, and annual prepayment is realistic only with a discount and after trust exists.

**Payment methods** [S22, S23]:
- Adults with any formal account: 50% (2025 CFIS). E-money: 36%. Bank: 23%.
- A Q1 2026 SWS survey shows ownership of 58% among adults 18+ and e-money of 43%.
- GCash claims about 81M active users, Maya 40–50M (company claims via secondary sources) [S52].
- Credit-card ownership for 2025 was not found. It is low relative to e-money (**M**, inferred from 23% bank ownership).

**Implication:** collect by GCash, Maya or QR Ph, via payment link or manual invoice. Auto-renewing card subscriptions will reach only a minority of SME owners.

**Resellers, agencies, freelancers.** Google Workspace is commonly sold through local resellers with peso pricing [S43]. Facebook-ad agencies and freelance "page admins" are widespread, but I found no quantified data (gap). Part-time admin and chat roles post at about ₱18,000–30,000/mo [S29]. **Hypothesis:** agencies and freelance page admins who manage several SME pages are a plausible channel partner, and a plausible user.

**Cost of the human alternative (official NWPC, as of 18 Oct 2026)** [S27]. Monthly figures use ×313/12 for six-day workers.

| Region | Daily minimum (non-agri) | ≈ Monthly |
|---|---|---|
| NCR (WO NCR-28, 26 Sep 2026) | ₱718–755 | ≈ ₱18,700–19,700 |
| CALABARZON (WO IVA-23, eff. 26 Oct 2026) | ₱550–645 | ≈ ₱14,300–16,800 |
| Central Luzon (RBIII-26, 16 Apr 2026) | ₱515–600 | ≈ ₱13,400–15,700 |
| Central Visayas (ROVII-27, 14 Oct 2026) | ₱542–582 | ≈ ₱14,100–15,200 |
| Davao (RB XI-24, 1 Sep 2026) | ₱525–540 | ≈ ₱13,700–14,100 |

NCR-27's second tranche (₱743–780 from 20 Jan 2027) is listed as "awaiting final decision" from two Regional Trial Courts [S27]. Statutory add-ons (13th-month pay, SSS, PhilHealth and Pag-IBIG employer shares) raise the loaded cost by roughly 15–25% (**estimate**; rates not verified here).

Remote VAs average ₱27,231/mo on Indeed (about 1.1k salaries, updated 12 Oct 2026; Makati ₱38,096, Cebu City ₱30,539) [S28]. Entry-level postings run ₱15–20k, and specialists earn ₱40–60k+ [S29]. **(H for official wages; M for job-board data)**

---

## 5. Regulation

- **Data Privacy Act (RA 10173).** It applies to any processing of customer chats, which contain names, phone numbers and often health or ID data. The SME is the personal information controller. The tool vendor is a **processor**, so a data-processing agreement is needed. **(H)**
- **NPC Advisory No. 2024-04, "Guidelines on the Application of RA 10173… to Artificial Intelligence Systems Processing Personal Data" (19 Dec 2024)** [S30, S31]. It covers controllers and processors across the AI lifecycle and requires:
  - Transparency about the nature, purpose and extent of AI processing, its risks and its likely impacts.
  - Fairness and bias monitoring.
  - Accountability: documented policies and the ability to demonstrate compliance.
  - Meaningful human intervention for automated decisions with significant effects. (Law-firm summaries mention this; I did not verify the wording in the advisory text.)

  **Practical reading (hypothesis):** disclose "you are chatting with an AI assistant", offer a human handoff, write a privacy notice, and don't use customer data to train shared models without consent. **(M–H)**
- **Registration (NPC Circular 2022-04, in force 11 Jan 2023)** [S32]. Registration is mandatory for controllers or processors that meet any of three triggers: 250+ employees; processing sensitive personal information of 1,000+ individuals; or processing likely to pose risks to data subjects' rights. Those below the thresholds file a sworn declaration of exemption. **Clinics with 1,000+ patient records likely must register.** A vendor processing many clinics' data almost certainly crosses the threshold itself. **(M: secondary summaries; verify against the circular)**
- **Health data.** Health information is sensitive personal information under RA 10173. Clinics therefore carry higher consent and security obligations. **(H)**
- **Internet Transactions Act (RA 11967) and its IRR (JAO 24-03).** Full enforcement began after the transitory period ended on 20 June 2025 [S35]. It mostly binds online merchants and e-marketplaces: price and seller-identity disclosure, and DTI takedown powers. For an AI agent quoting prices on a merchant's behalf, **accuracy of quoted prices and terms becomes a merchant liability.** That argues for grounding replies in a maintained price list and for logging. **(M)**
- **AI legislation.** No AI law had been enacted as of the latest sources. The AI Development and Regulation Act (risk-based; strict rules only for high-risk loan, job and health decisions) is pending, as are several deepfake and AI bills in the 20th Congress [S33, S34]. **(M)**
- **Tax on digital services (RA 12023, RR 3-2025).** Non-resident digital service providers must charge 12% VAT on B2C sales; B2B buyers reverse-charge. Registration was due around June 2025 [S36]. This removes part of the price advantage foreign SaaS had over local VAT-registered sellers. **(M–H)**
- **BSP.** If the product only creates payment links through a licensed gateway (PayMongo, Xendit), no BSP licence should be needed. Holding or moving customer funds would bring in National Payment Systems Act and BSP registration rules. **(M: general reading, not verified here; get legal advice before any money-handling feature)**
- **Review centers.** EO 566 (2006) gave CHED oversight of review centers. The Supreme Court ruled on it in *Review Center Association v. Ermita* (G.R. 180046, 2009) [S54]. Current regulatory status was not verified (gap).

---

## 6. Language and localization

- **Usage.** Taglish code-switching is the normal register of Philippine customer chat, especially in NCR. Cebuano or Bisaya dominates in Central Visayas, Davao and Northern Mindanao. Both points rest on linguistic literature and common observation (**M**). I found no quantified share of Taglish vs English vs Filipino in business messages (gap).
- **LLM performance.** FilBench (EMNLP 2025) tested 27 LLMs on Filipino, Tagalog and **Cebuano**. The best score was GPT-4o at 72.23%. SEA-LION v3 70B, the best Southeast Asia-specific model, scored 61.07%. Models were weakest on reading comprehension and translation [S38]. Batayan (ACL 2025) covers Tagalog and code-switched Taglish and reports "significant performance gaps for Filipino" [S39].

  **Limitation:** these benchmarks predate current frontier models and measure academic tasks, not customer-service intent handling. **(M)**
- **Market signal.** At least one Philippine-priced competitor markets English, Tagalog and Taglish replies "in the language the patient uses" as a feature [S41]. Taglish support is table stakes, not a differentiator.
- **Recommendation (hypothesis):** build a 200–500-message Taglish and Bisaya eval set from real (consented, redacted) inquiries before choosing a model. Weak points will likely be "po/opo" politeness, slang and abbreviations ("HM", "pm sent", "avail"), and price and date extraction.

---

## 7. Operating costs locally

- **Hosting and latency.** AWS opened a Local Zone in Metro Manila in June 2023 [S45]. A full AWS region in the Philippines has been reported as planned, but its status was not verified. Singapore regions are the usual default. Latency to Singapore was not measured, and it is unlikely to matter for chat (**hypothesis**). LLM API calls go to US or Singapore endpoints anyway. **(M)**
- **Collecting SaaS revenue (PayMongo, VAT-exclusive)** [S24]:

  | Method | Fee |
  |---|---|
  | GCash | 2.23% |
  | Maya | 1.79% |
  | QR Ph | 1.34% |
  | Local cards | 3.125% + ₱13.39 |
  | International cards | 4.02% + ₱13.39 |
  | Direct online banking | 0.71% or ₱13.39 |
  | Payouts | ₱10 |

  Subscriptions require PayMongo's approval, and sources differ on whether GCash and Maya are supported for recurring charges [S25]. Xendit pricing could not be fetched (403). Stripe does not support Philippine-registered businesses [S26]. **(H for PayMongo fees)**
- **Tax and invoicing (brief)** [S37]:
  - Individuals or sole proprietors with gross sales ≤ ₱3M can elect 8% income tax on gross above ₱250k in lieu of graduated rates and percentage tax.
  - VAT registration is required above ₱3M. Under the EOPT Act the threshold is to be CPI-indexed every three years, and the current indexed figure was not found.
  - Under EOPT, invoices replace official receipts. Non-VAT sellers must issue invoices for sales of ₱500+ or on request.
  - Registration with the BIR (via ORUS) and the LGU is required.

  **(M: professional summaries, not BIR text)**
- **SMS reminders.** Bulk A2P SMS pricing in PHP was not obtained; the vendor page returned 404 (gap). One competitor bundles 150–1,500 SMS and charges ₱1 per extra SMS [S41], which suggests a cost of under ₱1 per SMS (**L–M**).
- **LLM costs (not researched here).** At SME volumes (hundreds to low thousands of conversations a month), per-tenant LLM cost will probably be well under ₱500/mo with small models. It still needs a check against current price sheets, which is a separate task.

---

## 8. Segment snapshots

**A. Medical and dental clinics.**
- **Size:** 32,378 health MSMEs (Q), mostly micro (29,364) [S2]. Dental-clinic, PDA membership and active-dentist counts were not found. The PRC licenses about 2,000 new dentists a year (Nov 2025: 1,282 passers; May 2025: 764).
- **Inquiry profile:** "how much is X", "available ba tomorrow", HMO acceptance, location and hours, then booking and reminders. Repetitive and well suited to automation (**hypothesis**).
- **Seasonality:** low. Some peaks around school breaks and 13th-month pay in December (hypothesis).
- **Tools:** Facebook (67.5% of Baguio dentists) and Viber (8.7%) [S40]. Dedicated Philippine SaaS exists at ₱1–9k/mo [S41, S42].
- **Evidence of pain:** indirect. A competitor exists and invests in Taglish, HMO-claim tracking and recall. A local brand reported a 20% support-cost cut with Meta Business AI (vendor case study) [S18].
- **Regulation:** heavier (sensitive data).

**Fit: strong.** The job is clear, payers are professionals, ticket sizes are high, and repeat visits matter.

**B. Travel and tour operators.**
- **Size:** PSIC N (admin and support) has 17,401 MSMEs, of which travel agencies and tour operators are an unknown share. The DOT accreditation count was not found. There is a large informal layer of Facebook-based "travel and tours" resellers that the List of Establishments misses.
- **Demand context:** 6.48M international arrivals in 2025, still below the roughly 8M pre-pandemic level. ₱694B visitor spending (DOT) [S46]. Online travel up 14% (e-Conomy SEA) [S9]. Domestic trip data not found.
- **Inquiry profile:** high volume and low conversion. Package and price questions, multi-step quotes (dates, pax, inclusions), deposit by GCash or bank transfer, then itinerary documents. Quotation follow-up and payment-proof handling are central (**hypothesis**, consistent with payment-link vendors targeting travel agencies [S53]).
- **Seasonality (hypothesis):** peaks around airline seat sales, Holy Week, summer (Mar–May) and December.
- **Risks:** price-sensitive, fragmented operators. Scam-awareness advisories mean trust signals matter.

**Fit: strong on pain. Willingness to pay unproven.**

**C. Review and tutorial centers.**
- **Size:** 14,986 education MSMEs, dominated by private schools. Review and tutorial centers are an unknown subset (gap).
- **Inquiry profile:** schedules, fees, enrollment deadlines, location and online options. Strongly seasonal around PRC board-exam calendars and school-year starts (hypothesis).
- **Notes:** owners are often professionals, such as licensed nurses, engineers and teachers. Parents of tutorial students prefer Messenger or Viber (hypothesis).

**Fit: moderate–strong but seasonal,** which means churn risk between peaks.

**D. Beauty and wellness (salons, spas, nails, aesthetic clinics).**
- **Size:** part of S (87,197 MSMEs, 83,994 of them micro) [S2]. Aesthetic clinics may sit in Q. Viber reports health-and-beauty interest in Business Accounts [S13].
- **Inquiry profile:** high volume, low ticket, booking and slot management.

**Fit: volume yes, ARPU low (₱500–1,500).** Better as a later self-serve tier.

**E. Home services (aircon cleaning, pest control, repair, cleaning).**
- **Size:** spread across S, N and F. Not separable in the data (gap).
- **Inquiry profile:** quote by photos or location, scheduling, technician dispatch. Viber cites "technicians" as a growth segment [S13].

**Fit: plausible, but fragmented and informal.**

**F. Real estate agents and brokers.**
- **Size:** 18,297 real estate MSMEs [S2]. Individual salespersons and brokers are not counted, and PRC totals were not found.
- **Market:** Metro Manila condos are in a glut. About 80,000 unsold units in Q2 2026 (32,600 ready for occupancy), take-up down 60% quarter on quarter to 500 units, and Colliers forecasts record vacancy of 25.6% by end-2026 [S47].
- **Inquiry profile:** leads come from Facebook groups and Marketplace. Long, follow-up-heavy cycles.

**Fit: real lead follow-up pain,** but agents are individuals with volatile income in a down market. Developers pay brokers, so B2B-to-developer selling is a different sale.

---

## 9. Distribution

- **Paid social is cheap per click.** Facebook-ads benchmarks for the Philippines: CPC about US$0.07–0.16 (vs ~$1.1 globally) and CPM about US$1.10–1.35 (vs ~$20 globally) [S44]. These are **vendor data**, directional only. **(L–M)** Cheap reach does not equal B2B conversion. Expect the funnel to break at trust and payment, not at reach (hypothesis).
- **Facebook groups and communities.** Industry groups (dentists, travel agents, real estate, online sellers) are the default place where Philippine SMEs ask for tools. No quantified data (gap).
- **DTI Negosyo Centers.** There were 1,344 centers in 2022 [S49], and over 1,100 per a later source. Since March 2026 the EU-funded Digital PINAS programme has been training center staff on "AI applications for MSMEs" and data governance [S48]. This is a credible free-workshop and referral channel, but it is government-paced and favours micro clients. **(M)**
- **Associations.** The PDA has chapters (e.g. Baguio [S40]), and there are travel and real estate associations (not researched in detail). They are a likely channel for clinics and travel operators.
- **Agencies and freelancers.** Page-admin freelancers and small digital agencies already manage Messenger for clients, so a white-label or multi-client dashboard could recruit them as resellers. **(hypothesis)**
- **Barriers:**
  - Trust: scams are widespread, and SMEs fear losing customers to a bad bot.
  - Price sensitivity.
  - No card-based auto-renewal.
  - Owner time for setup: price lists and schedules often live in owners' heads.
  - Meta's free or bundled native agent [S18, S19].
  - Facebook policy and regulatory volatility [S14].

---

## 10. International option

- **Price gap.** The same vendor charges US$149 outside the Philippines vs ₱1,990 (~US$34) inside [S41]. US SMB AI-receptionist pricing clusters at US$50–150/mo [S50]. **For the same product, revenue per customer abroad is roughly 4–5x.** (M)
- **Channel mismatch.** Abroad, the products are phone-call and SMS-first (US/AU receptionists) or WhatsApp-first (Malaysia, Indonesia), LINE-first (Thailand) or Zalo-first (Vietnam) [S55; L, vendor blog]. A Messenger-centric Philippine product would need channel adapters. The Philippines is unusual in how much Messenger dominates [S3].
- **Payments for a Philippine-based seller.** Stripe does not support the Philippines [S26]. The options are a merchant of record (Paddle, Lemon Squeezy; Philippine payout terms not verified), a foreign entity (e.g. US LLC or Singapore Pte), or PayMongo international cards (4.02% + ₱13.39) [S24]. **(M)**
- **Advantages.** Philippine teams are fluent in US/AU business English and have BPO-style service norms. Low local cost allows aggressive USD pricing. **(hypothesis)**
- **Suggested sequencing (hypothesis):**
  1. Validate workflows with Philippine clinics or travel operators, where access is easy and Taglish is a moat against foreign tools.
  2. Run a parallel landing-page test in AU or US for the same vertical within the first 2–3 months.
  3. Keep the architecture channel-agnostic: Messenger, WhatsApp, SMS, web chat.

---

## 11. Reachable-market estimates (estimates, not sourced figures)

**Shared assumptions:**
- The base is formal MSMEs from PSA LE 2023 [S2].
- "Reachable" means it has an active Facebook Page or Messenger, at least 10–15 inquiries a day in season, an owner who can pay digitally, and is not served by a chain's in-house system.
- Prices follow the ceiling in §4.
- Conversion at ₱58 per US$1.
- I don't have segment-level data for these shares; each one is a judgment call and should be tested in interviews.

| Segment | Base (formal) | Share in sub-segment | Reachable share | Reachable accounts | ARPU ₱/mo | Serviceable revenue / yr |
|---|---|---|---|---|---|---|
| **Medical and dental clinics** | 32,378 health MSMEs | 40–60% are outpatient medical or dental clinics → 13k–19k | 20–40% | **~2.6k–7.8k** | 1,500–4,000 | ₱47M–374M (~US$0.8–6.4M) |
| **Travel and tour operators** | 17,401 admin and support MSMEs | 25–45% travel or tour → 4.3k–7.8k; plus 0.5–1x that again in informal Facebook resellers (lower ARPU) | 25–45% of formal | **~1.1k–3.5k formal** (+ informal tail) | 1,000–3,000 | ₱13M–126M (~US$0.2–2.2M) formal only |
| **Review and tutorial centers** | 14,986 education MSMEs | 15–30% → 2.2k–4.5k | 25–40% | **~0.6k–1.8k** | 1,000–2,500 | ₱7M–54M (~US$0.1–0.9M) |

**Realistic capture (estimate).** Year-one capture is probably 1–3% of reachable accounts for a solo or small team, given founder-led sales:

| Segment | Year-1 accounts | Year-1 MRR |
|---|---|---|
| Clinics | ~25–230 | ~₱40k–900k |
| Travel | ~10–100 | ~₱10k–300k |

The low ends do not cover a full-time developer's opportunity cost; the high ends would. This range is why section 10's USD test is recommended alongside.

**Why these three:**
- The transaction is clear (appointment, quote and deposit, enrollment).
- Inquiries are repetitive and go through Messenger.
- Buyers are professionals or business operators with ticket sizes high enough to value a booking.
- For clinics, a live Philippine-priced reference product proves willingness to pay [S41].

Beauty and wellness has the biggest count (S section, 87k) but the lowest ARPU. It suits a later self-serve tier. Real estate pain is real, but buyers are individuals in a down market [S47].

---

## Evidence gaps

1. No survey of Philippine MSME **software budgets or willingness to pay** in pesos, and no data on monthly vs annual preference.
2. No measure of **paid** AI-tool adoption among MSMEs. Existing figures mix free chatbot use with integration.
3. **Inquiry and booking workflow** data: no survey of how many SMEs use Messenger vs Viber vs SMS, inquiry volumes, reply times, or lost-lead rates.
4. Independent **Viber and WhatsApp** user counts for the Philippines (2025–2026).
5. Segment counts:
   - DOT-accredited travel and tour operators.
   - Dental clinics, PDA membership, active dentists.
   - Review and tutorial centers.
   - PRC real estate brokers and salespersons.
   - 4-digit PSIC breakdowns of the List of Establishments, which a PSA data request could provide.
6. **Meta Business AI / Business Agent**: Philippine pricing, Tagalog support, and how many Philippine SMEs have switched it on.
7. Current status of the **Facebook block** threat after the reported Meta–government arrangement.
8. 2025 **credit-card ownership** (BSP CFIS full report).
9. Exact NPC Circular 2022-04 and Advisory 2024-04 wording, and whether either has been amended since.
10. Xendit and Maya Business fees. A2P SMS prices. Paddle and Lemon Squeezy payout terms for Philippine residents.
11. Current CPI-indexed VAT threshold under EOPT.
12. Current CHED regulatory status of review centers after G.R. 180046.
13. Head-to-head evaluation of current frontier LLMs on **Taglish and Bisaya customer-service** messages.

---

## Sources

All accessed 2026-10-18. Type: OS = official statistics, PR = primary research, IA = independent analysis, VC = vendor claim or vendor-sponsored, N = news, OP = opinion or blog.

1. "2024 Philippine MSME Statistics." DTI. https://www.dti.gov.ph/dti-knowledge-hub/dti-statistics/dti-msme-statistics. Date not stated. OS. Philippines. Figures from infographic alt text; the total may be mislabelled. M.
2. "2023 Philippine MSME Statistics in Brief (as of 22 Nov 2024)." DTI, from PSA 2023 List of Establishments. https://dtiwebfiles.s3.ap-southeast-1.amazonaws.com/MSME+Resources/2023+Philippine+MSME+Statistics+in+Brief_as+of+22+November+2024.pdf. 22 Nov 2024. OS. Philippines. Formal establishments only. H.
3. "Digital 2026: The Philippines." Simon Kemp, DataReportal/Kepios. https://datareportal.com/reports/digital-2026-philippines. 5 Nov 2025. IA (aggregates platform ad tools). Philippines. Ad audiences are not unique people. H.
4. "PH businesses lag in AI adoption despite digital access—PIDS." PIDS. https://www.pids.gov.ph/details/news/press-releases/ph-businesses-lag-in-ai-adoption-despite-digital-access-pids. 11 Sep 2025. PR (government think tank). Philippines. Underlying data years unclear. M.
5. "Philippines businesses remain slow in adopting AI." Philstar. https://www.philstar.com/business/2025/09/14/2472566/philippines-businesses-remain-slow-adopting-ai-study. 14 Sep 2025. N. M.
6. "New AWS research shows strong AI adoption momentum in the Philippines…" AWS/Amazon press. https://press.aboutamazon.com/aws/2025/8/new-aws-research-shows-strong-ai-adoption-momentum-in-the-philippines-with-startups-outpacing-large-enterprises-in-innovation. Aug 2025. VC (AWS-sponsored, Strand Partners). Philippines. M–.
7. "AI adoption among PH businesses jumps 52% in a year — study." Newsbytes.PH. https://newsbytes.ph/2026/10/15/ai-adoption-among-ph-businesses-jumps-52-in-a-year-study/. 15 Oct 2026. N reporting VC (AWS-sponsored). Philippines. No size breakdown. M–.
8. "Microsoft and LinkedIn Release 2024 Work Trend Index on AI Use at Work in the Philippines." Microsoft Philippines. https://news.microsoft.com/en-ph/2024/05/23/microsoft-and-linkedin-release-2024-work-trend-index-on-ai-use-at-work-in-the-philippines/. 23 May 2024. VC. Knowledge workers only. M–.
9. "PH digital economy to hit $36B in 2025, leads SEA in AI adoption." Newsbytes.PH (on e-Conomy SEA 2025 by Google, Temasek, Bain). https://newsbytes.ph/2025/11/26/ph-digital-economy-to-hit-36b-in-2025-leads-sea-in-ai-adoption/. 26 Nov 2025. N on IA/VC. The "leads" claim is not quantified. M.
10. "Pinoys among top ChatGPT users in the world." PhilstarLife (GWI Q2 2025 via We Are Social/Meltwater). https://philstarlife.com/amp/article/520165-pinoys-among-top-chatgpt-users-in-the-world. 20 Oct 2025. N. Internet users 16+. M.
11. "More Filipino consumers using business messaging platforms." BusinessWorld (Meta/BCG study). https://www.bworldonline.com/?p=468994. Aug 2022. N on VC. Dated. M–.
12. "Small firms' socmed account leaps, Meta says." Daily Tribune. https://archives.tribune.net.ph/2022/08/17/small-firms-socmed-account-leaps-meta-says/. 17 Aug 2022. N on VC. Anecdote included. L–M.
13. "How Rakuten Viber is shaping connections from chats to commerce in the Philippines." Rakuten Today. https://rakuten.today/blog/how-rakuten-viber-is-shaping-connections-from-chats-to-commerce-in-the-philippines.html. 9 May 2025. VC. No user totals. L.
14. "Facebook ban a 'last resort' amid concerns over harmful effects —DICT." GMA News. https://www.gmanetwork.com/news/topstories/nation/999880/facebook-ban-a-last-resort-amid-concerns-over-harmful-effects-dict/story/. 26 Aug 2026. N. H for the event.
15. "Philippines contemplates nationwide Facebook ban…" Manila Bulletin. https://mb.com.ph/2026/08/26/philippines-contemplates-nationwide-facebook-ban-over-safety-concerns-igniting-fears-for-local-msmes-and-the-digital-economy. 26 Aug 2026. N. Seen via search snippet only (403). The MSME figure's origin is unclear. L–M.
16. "Controlling social media." Philstar (column). https://www.philstar.com/business/2026/09/28/2559312/controlling-social-media. 28 Sep 2026. OP. Snippet only. L.
17. "Meta and PH regulators struck an online-safety deal: Facebook ban off the table." astig.ph. https://astig.ph/meta-philippines-online-safety-rapid-takedown-deal-2026/. ~Sep 2026. OP/blog. Snippet only. L.
18. "Meta brings Business AI to the Philippines…" Marketech APAC. https://marketech-apac.com/meta-brings-business-ai-to-the-philippines-to-enhance-customer-engagement-for-local-businesses/. 20 Oct 2025. N (Meta statements). No pricing or language details. M–H.
19. "Meta's AI agent for WhatsApp Business is now available globally." TechCrunch. https://www.techcrunch.com/2026/06/03/metas-ai-agent-for-whatsapp-business-is-now-available-globally/. 3 Jun 2026. N. SMB tier prices not given. H.
20. "Meta launches Business Agent AI across messaging apps." TechBuzz. https://www.techbuzz.ai/articles/meta-launches-business-agent-ai-across-messaging-apps. 3 Jun 2026. N. M.
21. "Meta sells AI agent for businesses in push to monetize service." Bloomberg Law/BGov. https://news.bloomberglaw.com/mergers-and-acquisitions/meta-sells-ai-agent-for-businesses-in-push-to-monetize-service. Jun 2026. N. Snippet only. M.
22. "BSP: Filipino account ownership down to 50% in 2025…" GMA News (BSP 2025 CFIS). https://www.gmanetwork.com/news/money/economy/984131/bsp-filipino-account-ownership-down-to-50-in-2025-household-access-up-to-85/story/. 2026. N on OS. Seen via search summary. M–H.
23. "E-wallet adoption pushes Pinoy account ownership to 58%." Newsbytes.PH (SWS Q1 2026). https://newsbytes.ph/2026/05/22/e-wallet-adoption-pushes-pinoy-account-ownership-to-58/. 22 May 2026. N. Adults 18+, a different base from CFIS. M.
24. "Pricing." PayMongo. https://www.paymongo.com/pricing. Current. VC (list prices, VAT-exclusive). H.
25. "Subscriptions" (developer docs). PayMongo. https://developers.paymongo.com/docs/subscriptions. Current. VC. Payment-method coverage unclear. M.
26. "Stripe global availability." Stripe. https://stripe.com/global. Current. VC. Philippines not listed. H.
27. "National Wages and Productivity Commission" (minimum wage table). DOLE-NWPC. https://nwpc.dole.gov.ph/. As of 18 Oct 2026. OS. NCR-27 under court review. H.
28. "Virtual assistant salaries in the Philippines." Indeed PH. https://ph.indeed.com/career/virtual-assistant/salaries. Updated 12 Oct 2026. Job-board data (~1.1k salaries). Self-reported or posted. M.
29. "Virtual Assistant Salary Philippines 2026." Bossjob (plus JobStreet listings via search). https://bossjob.ph/blog/?p=8893. 2026. Job-board blog. L–M.
30. NPC Advisory No. 2024-04, "Guidelines on the Application of RA 10173… to Artificial Intelligence Systems Processing Personal Data." National Privacy Commission. https://privacy.gov.ph/wp-content/uploads/2024/12/Advisory-2024.12.19-Guidelines-on-Artificial-Intelligence-w-SGD.pdf. 19 Dec 2024. OS/regulatory. Read via summaries. H for existence; M for details.
31. "Philippines: NPC releases new guidelines on artificial intelligence…" Baker McKenzie InsightPlus. https://insightplus.bakermckenzie.com/bm/data-technology/philippines-npc-releases-new-guidelines-on-artificial-intelligence-and-child-oriented-transparency. Dec 2024/Jan 2025. IA (law firm). M–H.
32. NPC Circular 2022-04 (registration). National Privacy Commission. https://privacy.gov.ph/wp-content/uploads/2023/05/Circular-2022-04.pdf. Also the Baker McKenzie summary: https://insightplus.bakermckenzie.com/bm/data-technology/philippines-new-npc-circular-on-registration-of-data-protection-officers-and-data-processing-systems-takes-effect_1. 5 Dec 2022 (in force 11 Jan 2023). OS + IA. Thresholds taken from summaries. M.
33. "Proposed AI bill gains momentum in Congress." Philstar. https://www.philstar.com/business/technology/2026/07/14/2542095/proposed-ai-bill-gains-momentum-congress. 14 Jul 2026. N. M.
34. "Regulating Risk: The 20th Congress of the Philippines' Response to Artificial Intelligence." Foundation for Media Alternatives. https://fma.ph/regulating-risk-the-20th-congress-of-the-philippines-response-to-artificial-intelligence/. 2026. IA (civil society). M.
35. "DTI starts e-commerce law's full enforcement." Daily Tribune. https://tribune.net.ph/2025/06/29/dti-starts-e-commerce-laws-full-enforcement. 29 Jun 2025. N. IRR text at https://dti.gov.ph/sdm_downloads/implementing-rules-and-regulations-of-the-internet-transactions-act-of-2023 (not read). M.
36. "Tax Alert No. 3 [RR No. 3-2025]." PwC Philippines. https://www.pwc.com/ph/en/tax/tax-publications/tax-alerts/2025/tax-alert-3.html. Jan 2025. IA (professional). Dates vary across sources. M–H.
37. "Implementation of the amendments introduced by EOPT Act on VAT and percentage tax." Grant Thornton PH. https://www.grantthornton.com.ph/alerts-and-publications/technical-alerts/tax-alert/2024/implementation-of-the-amendments-introduced-by-eopt-act-on-vat-and-percentage-tax/. 2024. IA. Indexed threshold not found. M.
38. "FilBench: Can LLMs Understand and Generate Filipino?" Miranda et al., EMNLP 2025. https://arxiv.org/abs/2508.03523. Aug 2025. PR. Academic tasks, older models. M–H.
39. "Batayan: A Filipino NLP benchmark for evaluating LLMs." ACL 2025. https://arxiv.org/abs/2502.14911. Feb 2025. PR. Not customer service. M.
40. "The Use of Social Media on Enhancing Dental Care and Practice Among Dental Professionals." JMIR Formative Research 9:e66121. https://pmc.ncbi.nlm.nih.gov/articles/PMC11723565/. 3 Jan 2025. PR. Baguio PDA chapter only (n=265). M.
41. "HeyDenta" listing (pricing updated 5 Oct 2026). SoftwareSuggest. https://www.softwaresuggest.com/heydenta. Oct 2026. VC via directory. One vendor. M.
42. "Royal Dental CloudAI Platform." Vendor page. https://194-164-127-202.sslip.io/. Undated. VC. Unverified, possibly ephemeral. L.
43. "Google Workspace." Z.com Philippines (reseller). https://web.z.com/ph/google-workspace/. Current. VC. Labelled "Business Email Starter". M.
44. "Facebook Ads costs – Philippines (CPC/CPM)." superads.ai. https://www.superads.ai/facebook-ads-costs/cpc-cost-per-click/philippines and https://www.superads.ai/facebook-ads-costs/cpm-cost-per-mille/philippines. 2025–2026. VC data. Directional. L–M.
45. "AWS launches Local Zone in Metro Manila." BusinessWorld. https://bworldonline.com/?p=532589. Jun 2023. N. H.
46. "Philippines logs 6.48 million tourists in 2025; DOT targets more for 2026." ABS-CBN. https://www.abs-cbn.com/news/business/2026/1/21/philippines-logs-6-48-million-tourists-in-2025-dot-targets-more-for-2026-1409. 21 Jan 2026. N on OS. H.
47. "Affordable homes buck Metro Manila condo glut — Colliers." Manila Bulletin. https://mb.com.ph/2026/08/12/affordable-homes-buck-metro-manila-condo-glutcolliers. 12 Aug 2026. N on IA (Colliers). Seen via search summary. M.
48. "DTI, Digital PINAS empower MSMEs in underserved communities through ICT equipment handover." UNDP Philippines. https://www.undp.org/philippines/press-releases/dti-digital-pinas-empower-msmes-underserved-communities-through-ict-equipment-handover. Mar 2026. OS/partner press release. H.
49. "DTI providing MSMEs with economic rudder." Daily Tribune (1,344 Negosyo Centers). https://tribune.net.ph/2022/10/28/dti-providing-msmes-with-economic-rudder. 28 Oct 2022. N. Dated. M.
50. "How much does an AI receptionist cost…2026." My AI Front Desk; also "AI Receptionist Cost in 2026," Hyperleap. https://www.myaifrontdesk.com/blogs/how-much-does-an-ai-receptionist-cost-a-straight-answer-for-2026 ; https://hyperleap.ai/blog/ai-receptionist-cost. 2026. VC. US. L–M.
51. "Online selling terms Philippines." PayMongo blog. https://www.paymongo.com/blog/online-selling-terms-philippines. Undated. VC. L–M.
52. "GCash." Wikipedia (company claim of 81M active users, Jan 2025). https://en.wikipedia.org/wiki/GCash. Current. Secondary. L–M.
53. "Payment links for travel agencies and tour operators in Southeast Asia." HitPay; and "GCash screenshot…" imagetotable.ai (on Mynt's July 2025 fake-receipt warning). https://hitpayapp.com/ph/blog/payment-links-travel-agencies-tour-operators-southeast-asia ; https://imagetotable.ai/blog/gcash-screenshot-payment-details. 2025–2026. VC. L.
54. *Review Center Association of the Philippines v. Ermita*, G.R. No. 180046. Supreme Court via LawPhil. https://lawphil.net/judjuris/juri2009/apr2009/gr_180046_2009.html. Apr 2009. Official (jurisprudence). Holding not read. M.
55. "Most popular messaging apps by country." Infobip. https://www.infobip.com/blog/most-popular-messaging-apps-by-country. 2025–2026. VC blog. L.
