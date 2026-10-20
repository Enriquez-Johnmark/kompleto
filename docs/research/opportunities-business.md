# Business-facing automation opportunities: market evidence

Researched 2026-10-18 for choosing the first product of a Rust-built AI agent and automation platform built by a solo developer or small team. The scope is global, with notes on Southeast Asia and the Philippines where evidence exists. A separate document covers the Philippines market in depth.

**How to read this.** Bracketed numbers such as [12] point to the Sources list at the end. Every source is labelled with an evidence type. **Vendor claim** means the figure comes from a company that sells a fix for the problem it measures, so treat it as indicative only. **Derived** means I calculated the figure from cited inputs. **Hypothesis** means no direct evidence was found. Estimates are given as labelled ranges.

---

## 1. Summary table

The scale follows `docs/research/scoring-method.md`: **5 is always favourable to us**. So "Competitive room" 5 means few competitors, "Integration ease" 5 means easy, and "Risk (inv.)" 5 means low risk. E means the score is backed by evidence; J means it is a judgement call, and judgement scores are capped at 4.

| # | Opportunity | Severity | Frequency | WTP evidence | Competitive room | Integration ease | Risk (inv.) |
|---|---|---|---|---|---|---|---|
| 1 | Speed-to-lead, lead qualification, quote/proposal follow-up | **5 E**: faster contact strongly predicts qualification [1][2]; 55% of home-services firms never ask callers to book [4] | **5 E**: every inbound lead, daily | **4 E**: GoHighLevel AI Employee add-on $50–97 per sub-account [28]; Jobber bundles quote follow-ups [47] | **2 E**: GoHighLevel, Jobber and Podium bundle it [25][27][28] | **3 J**: web forms, email and SMS are easy; CRM/FSM APIs vary | **4 J**: outbound messages are reviewable; consent and SMS rules apply |
| 2 | Inquiry handling across Messenger, WhatsApp, IG, Viber, email, web chat | **4 E**: slow replies lose sales (indirect evidence [1][4]); one PH case cut support cost 20% (vendor) [34] | **5 E**: Meta reports 600M+ business conversations a day globally [34] | **5 E**: respond.io at $35M ARR with ~10k customers [31]; WATI, ManyChat, SleekFlow all paid [32][36][37] | **1 E**: Meta's own Business Agent launched globally on WhatsApp in June 2026 [33] | **3 E**: Meta APIs are public but need business verification; Viber is separate | **3 J**: brand-voice errors, PII in chats |
| 3 | AI receptionist, booking, reminders, no-show reduction | **4 E**: home-services calls reach a person only 52% of the time [4]; clinic no-shows pool around 23% [11] | **5 E**: daily calls; Jobber reports "over 20 calls/week" per customer [25] | **5 E**: Beside has 20k+ paying customers [18]; Retell ~$45–60M ARR (estimate) [21]; Smith.ai and Goodcall publish prices [22][23] | **1 E**: very crowded, with incumbents bundling it (Jobber, Weave, Podium, GoHighLevel) [25][26][27][28] | **2 E**: telephony plus calendar or PMS; Weave's version only works with 3 dental PMSs [26] | **2 E**: callers hang up [29][30]; health data where clinics are involved |
| 4 | Quotation and proposal preparation, admin coordination | **3 J**: slow quoting is plausible but undocumented | **4 J**: weekly to daily for trades and agencies | **3 E**: PandaDoc $19–49 per user [42]; FSM suites include quoting [47] | **3 J**: generic tools are cheap; vertical pricing logic is open | **3 J**: needs price lists and FSM/CRM write access | **4 J**: drafts go to a human before sending |
| 5 | Document intake, extraction, validation (invoices, IDs, KYC, claims) | **3 E**: manual invoice cost about $10–22 each (secondary citation of APQC/Ardent) [40] | **4 J**: daily for bookkeepers and AP | **4 E**: Nanonets, Docsumo, Rossum etc. charge per page [41] | **2 J**: crowded, and LLMs commoditise the extraction step | **3 J**: files in are easy; ledger write-back is harder | **2 J**: KYC and claims carry regulatory liability |
| 6 | SME back-office: AR collections, payment reminders, bookkeeping prep | **4 E**: 56% of US small businesses are owed money, averaging $17.5k [38] | **3 E**: weekly or monthly cycles | **4 E**: Chaser from $259/mo, Paidnice from $69/mo [39] | **3 E**: few SMB-priced players; Xero and QuickBooks have basic reminders | **4 J**: Xero and QuickBooks Online APIs are public | **4 J**: reminder tone needs care; money moves only through existing rails |
| 7 | Vertical workflows (clinics/dental, travel/tours, tutoring, agencies, real estate, home services) | **4 E** for dental (81% cite no-shows and late cancels [16]; 62% name staffing as the top challenge [17]); **2 E** for salons (no-shows 1–4% [14]) | **4 J** | **4 E**: vertical SaaS charges a premium (Weave, Jobber) [25][26] | **3 J**: varies; tours and tutoring look thinner | **2 J**: each vertical has its own PMS or booking system | **3 J**: high in health, lower in tours |
| 8 | Research, reporting, decision support for SMEs | **2 J** | **2 J**: monthly | **2 J**: no SMB-specific paid evidence found | **3 J** | **4 J** | **4 J** |
| 9 | Workflow glue between existing SME software | **3 J** | **4 J** | **5 E**: Zapier, Make and n8n publish paid tiers [43] | **1 E**: cheap incumbents; n8n can be self-hosted free [43] | **3 J** | **4 J** |

**What the table says.** The best evidence of pain and willingness to pay is in #1, #2 and #3. #2 and #3 are also the most crowded, and in #2 the platform owner itself now competes. #1 (narrowed to follow-up and quote chasing) and #6 (AR collections) combine evidenced pain, published price points, reachable integrations, and outputs a human can approve before they go out.

---

## 2. Context: AI adoption among small firms (interest vs. paying)

**Adoption rates (official statistics):**

- **Census BTOS [6]** (US official statistics, H). From December 2025 to May 2026, 17–20% of US businesses used AI in any business function; the rate was 19.8% on 3 May 2026.
  - Firms with 250+ employees: 37%. Firms with 100–249: 32%.
  - **Firms with fewer than 20 employees showed no significant change in use**, and fewer than 20% of firms with four or fewer employees used AI.
  - Caveat: the question was reworded in November 2025, so the series before and after is not comparable.
  - A WeWork analysis of the same survey reports 21.7% in July 2026 [7] (secondary analysis, M).
- **OECD [8]** (primary survey, H). A 2024 survey of more than 5,000 SMEs in 7 OECD countries found 31% used generative AI, ranging from 24% in Japan to 39% in Germany.

**Surveys of interest (not proof of payment):**

- **Salesforce [9]** (vendor-commissioned survey, M). 3,350 SMB leaders, fieldwork in August–September 2024: 75% "at least experimenting" with AI, and 91% of AI users say it boosts revenue. These are self-reported figures from a vendor that sells AI, and they show correlation only.
- **Gartner [10]** (independent analysis, H for the claim existing, M for the forecast). Predicts that more than 40% of agentic AI projects will be cancelled by the end of 2027. It also estimates that only about 130 of the "thousands" of agentic vendors are real, the rest being "agent washing".

**Implication (judgement).** Micro-businesses, the likely early buyers for a small team, are **not** adopting AI quickly. Selling "AI agents" to them is weaker than selling an outcome they already pay for, such as calls answered, invoices collected, or quotes won. The paid evidence below is all framed around outcomes.

---

## 3. Per-opportunity analysis

### 3.1 Speed-to-lead, lead qualification, quote and proposal follow-up

**Who has it.** Any business that buys or receives inbound leads: home services, real estate, agencies, B2B services, education and enrolment, travel.

**Current workflow.** Leads arrive by web form, call, DM or marketplace (Zillow, Google Local Services Ads). The owner or a salesperson replies when free. Quotes are sent, and follow-up depends on memory.

**Evidence of the problem and its cost:**

- **HBR 2011 [1]** (primary research, US, M because I read it through secondary quotes; the original is paywalled).
  - Firms that tried to contact a lead within an hour were nearly 7 times as likely to qualify it as firms that waited longer, and more than 60 times as likely as firms that waited 24 hours or more.
  - A secondary report of the same audit says 37% of 2,241 companies responded within an hour and 23% never responded [1a].
  - Limitation: the data is 15+ years old and US B2B.
- **MIT/InsideSales Lead Response Management Study [2]** (2007; co-authored with a vendor; six companies; M–L).
  - It is widely cited for "contact within 5 minutes vs 30 minutes = 100× contact odds, 21× qualification odds".
  - InsideSales itself has also published 10× and 6× for the same comparison, so the exact multiplier is unreliable. Only the direction is solid.
- **RevenueHero [3]** (vendor claim, undated, about 1,000 B2B sites, L–M). Average response time of 1 day 5 hours 17 minutes; 63.5% never answered a demo request.
- **Invoca, July 2026 [4]** (vendor claim; based on Invoca's own customer base in US home services; M).
  - Only **52%** of calls reached a person (65% if calls of 15 seconds or less are excluded).
  - Answer rates ranged from **32% to 74%** across sub-industries.
  - **55% of businesses never ask leads to book.**
  - Invoca estimates that raising answer rate, lead rate and conversion by 5 points each would yield about 38% more conversions.
- **CallRail, January 2025 [5]** (vendor claim, 1.1M leads, M). Missed-call rates were 32% in healthcare, 28% in legal, 14% in home services and 9% in real estate. Its claim that "85% of unanswered callers won't call back" has no primary source that I could find. **Do not use it.**
- **Real estate.** The often-quoted "74–78% of buyers choose the first agent who responds" could not be traced to an NAR primary source (L). Treat it as unverified.

**Frequency.** Every lead, every day.

**What businesses use today.** CRM or field-service-management (FSM) automations, a VA, or nothing.

**Competitors with published pricing:**

- **GoHighLevel [28]** (product docs): $97, $297 or $497 per month, plus an AI Employee add-on of $50 or $97 per month per sub-account, with AI usage billed separately. Agencies resell it.
- **Jobber [25][47]**: quote follow-ups are included on the Connect plan and above. Its Receptionist add-on is reported at $29/month for 30 conversations, then $0.79 per conversation (secondary source, L).
- **Podium [27]**: Core about $399/month and Pro about $599/month (third-party reports). It removed public pricing in Q4 2025, and the AI Employee add-on is quote-only.
- **Lindy [45]**: general SMB agent at $49.99, $99.99 or $199.99 per month (third-party, July 2026, M).

**Why existing tools fall short.** The evidence is thin. Invoca's "55% never ask to book" [4] shows a **behaviour** gap even among firms that already pay for call tracking. That suggests the gap is execution (follow-through), not access to tools. That is an inference, J.

**Buyer and budget.** The owner in micro firms; a sales or marketing lead in firms of 10–50.

**Sales motion.** Self-serve trial, plus agencies and resellers (the GoHighLevel model shows agencies will resell).

**Integrations.** Web form or webhook, email (Gmail/Microsoft Graph), SMS (Twilio; US A2P 10DLC registration), FSM/CRM (Jobber, HubSpot).

**Data sensitivity.** Low to medium: contact details only. Liability for unwanted outreach exists under TCPA (US) and anti-spam rules (J).

**Deliverability for a small team.** High for a text and email-first version that drafts messages for human approval. Voice is harder.

**First-user acquisition.** A narrow vertical community (e.g., one trade, or tutoring centres), agency partners, or FSM marketplace listings (J).

**Retention driver.** Measurable booked jobs and quote win rate; once it sits in the lead path, switching is disruptive (J).

**What would disprove it:**

- Trial users don't connect their lead sources within the first week.
- Measured response-time gains don't change win rate in their own data.
- Buyers say "Jobber or GoHighLevel already does this" and won't pay extra.

### 3.2 Customer inquiry handling across messaging and web chat

**Who has it.** Retail and e-commerce, clinics, tour operators, schools and review centres, real estate. This is especially relevant in messaging-first markets: SEA, India, LatAm.

**Current workflow.** Staff or the owner answer DMs by hand, often on phones; FAQ answers are copied and pasted.

**Evidence:**

- Meta's Philippines country director cites **600M+ daily conversations between people and businesses** across Meta apps [34] (official company statement, global figure, M).
- WhatsApp paid messaging reportedly reached a **$2B annual run rate in Q4 2025** [35]. This is a secondary citation of Meta's earnings call (M). It shows businesses pay to message at scale.

**Competitors and evidence of payment:**

- **respond.io [31]** (Kuala Lumpur): $62.5M Series B in June 2026, **$35M ARR, up 169% year on year, ~10,000 customers** (company disclosure via press, M–H).
  - Derived: roughly $3,500 per customer per year, or about $290 a month. Buyers are mid-market, not micro.
- **SleekFlow [32]** (HK/SEA): about $15M raised by August 2024 (more by 2025, per aggregators); "2,000+ enterprises"; launched multi-agent "AgentFlow" in July 2025.
- **WATI [36]**: about $69–349/month plus Meta message fees (third-party, M).
- **ManyChat [37]**: Pro from about $15/month, with the AI add-on around $29/month (third-party, conflicting figures, L–M).

**Why existing tools fall short:**

- WATI reviews on G2 and Capterra repeatedly complain about **price at low volume and stacked fees** (plan plus per-message markup plus Meta fees), and some report timeouts or reliability problems [36] (customer testimony, M).
- I found no evidence that answer quality itself is the main complaint.

**Platform-owner threat (decisive).**

- Meta's **Business Agent launched globally on WhatsApp in June 2026** [33]. It answers questions, recommends products, books appointments, qualifies leads, and hands off to a human. It is coming to Instagram DMs and is included in some WhatsApp Business Premium tiers.
- **Business AI on Messenger launched in the Philippines in October 2025** [34]. White Coat Manila reported a 20% cut in support costs (customer testimony relayed by Meta, M).
- A generic "AI replies to DMs" product now competes with a free or bundled feature from the channel owner.

**Buyer.** The owner, or an e-commerce or marketing lead.

**Motion.** Self-serve, plus Meta Business Partner agencies.

**Integrations and permissions:**

- WhatsApp Cloud API and Messenger/IG Graph API need Meta business verification and app review for some permissions (product docs; verify timing).
- Viber Business is a separate commercial channel.

**Data sensitivity.** PII; health details if the business is a clinic.

**Deliverability.** Medium. The channel plumbing is commoditised; the value has to come from workflow (bookings, quotes, payments) behind the chat.

**Retention driver.** Conversation history plus connected catalogue or booking. Switching cost is moderate.

**What would disprove a non-Meta product here:** target businesses in a pilot vertical say Meta's built-in agent is "good enough" once they try it.

### 3.3 AI receptionist, scheduling, reminders, no-show reduction

**Who has it.** Clinics and dental, home services, salons, legal, real estate.

**Current workflow.** Front desk staff, voicemail, call forwarding to the owner's mobile, or a human answering service.

**Evidence of pain:**

- **Missed calls.** Invoca 52% answered [4]; CallRail 14–32% missed by vertical [5]. Both are vendor data; definitions differ, so the range is wide.
- **Outpatient no-shows:**
  - Dantas et al. 2018 (systematic review of 105 studies): **about 23% on average**, ranging from 13.2% (Oceania) to 43.0% (Africa) [11]. Peer-reviewed, global, H. Single studies vary widely.
  - Reminders work modestly. Cochrane: SMS reminders vs none, RR 1.14 for attendance, low-to-moderate quality evidence [12]. Robotham et al. 2016: notified patients about 23% more likely to attend [13].
- **Dental.** In ADA Health Policy Institute monthly polls, about **81–83% of dentists** cite no-shows and late cancellations as what keeps schedules from being full [16] (official professional body survey via trade press, M–H). About **62% named staffing shortages** as the biggest challenge for 2025 [17].
- **Salons (disconfirming).** Zenoti's 2026 North American benchmark puts no-shows at **1–4%** (salons 2%, barbers and nails 4%, med-spa 4%) [14] (vendor data, large base, M–H). Treatwell UK reports about 2.7% for gel nails [15] (vendor data, M).
  - The "10–25% salon no-show" figures on AI-booking blogs are unsourced vendor claims.
  - **Salon no-show reduction is a weak wedge** in markets that already use online booking.

**Evidence of willingness to pay (strong):**

- **Beside [18]** (Fortune, November 2025): $32M raised, **$4M ARR, 20,000+ paying customers**.
  - Derived: about $200 per customer per year, or about $17 a month. This shows **demand at a very low price point** for prosumers and micro firms.
- **Newo [19]** (February 2026): $25M Series A, sold white-label through 200+ partners.
- **Synthflow [20]** (June 2025): $20M Series A from Accel; about $3M 2024 revenue (CB Insights estimate).
- **Retell AI [21]** (voice infrastructure): about $45M ARR at the end of 2025 and about $60M annualised by April 2026, on about $5M raised (Sacra estimate, M).

**Published prices:**

- Goodcall: $79 for 100 callers, $129 for 500, $299 for 2,000 [22].
- Smith.ai AI receptionist: free for 25 calls; Pro from $150/month, with per-call overages of about $2.17–2.50 [23].
- Rosie: $49 for 250 minutes, $149 for 1,000, $299 for 2,000 [24].
- Jobber add-on (see 3.1) [25].
- Weave: AI receptionist sold as an add-on, pricing "still being determined" as of Q4 2025, and initially limited to Open Dental, Dentrix and Eaglesoft [26].
- Infrastructure cost: Vapi about $0.10–0.31 per minute all-in; Bland about $0.11–0.14 per minute plus telephony [44] (third-party, M).

**Why existing tools fall short (evidenced):**

- **Caller rejection.** Jobber community posts report callers "hung up as soon as they heard the AI", and one owner turned it off [30] (customer testimony, anecdotal, L–M).
- Upfirst analysed 450,702 calls handled by 503 AI receptionists and found that **AI settings explained only about 13% of the variation in hang-up rates** between businesses; many hang-ups happen in the first seconds [29] (vendor analysis via news, M).
- A competitor's blog (Solvea) reports billing surprises with Goodcall. That source is weak (L), so it is not cited or scored.

**Buyer.** Owner or practice manager.

**Motion.** Self-serve for trades; demo-led for clinics.

**Integrations.** Telephony (SIP or Twilio, number porting or forwarding); calendars; practice management systems (Dentrix, Open Dental), many of which have gated partner programmes. Pre-built integrations are the main moat for Weave and Jobber.

**Data sensitivity.** High for clinics (US HIPAA business associate agreements; PH Data Privacy Act, sensitive personal information). Lower for trades.

**Deliverability for a small team.** Low to medium. Voice latency and quality are rented from Retell or Vapi, so there is no Rust advantage; the hard parts are PMS integration and accuracy.

**Retention driver.** It sits on the phone number, so switching is sticky. But churn risk from bad call experiences is real (J).

**What would disprove it as a first product:** no reachable vertical where the incumbent suite (Jobber, Weave, Podium, GoHighLevel) is absent and buyers still pay $50+ per month.

### 3.4 Quotation and proposal preparation, admin coordination

**Who.** Trades, agencies, event and tour operators, B2B services.

**Workflow.** Site visit or inquiry, then a spreadsheet or Word/FSM quote, then email.

**Evidence.**

- Pain is mostly anecdotal. The claimed "28% close-rate lift from automated follow-up (Jobber research 2025)" could not be traced to Jobber (L) [47].
- Incumbents: PandaDoc Starter about $19 and Business about $49 per user per month; Proposify about $19–49 per user [42] (third-party, M). Jobber, ServiceTitan and Housecall Pro include quoting [25][47].
- Jobber lists a "Receptionist Companion" that drafts quotes from calls as **coming soon** [25]. The incumbent is moving here too.

**Gap (J).** Turning messy inputs (a WhatsApp thread, photos, a call) into a priced draft quote using the business's own price list. That is a genuine agent task with human approval, but there is no direct evidence of willingness to pay for this specific step.

**Risk.** Low if quotes are always drafts.

**Disproof.** Owners say quoting time is not their bottleneck, or that pricing judgement can't be delegated.

### 3.5 Document intake, extraction, validation, routing

**Who.** Bookkeepers and accounting firms, AP teams, insurers and claims handlers, lenders and KYC teams, logistics.

**Evidence:**

- Manual invoice processing costs about **$10–22 per invoice** (APQC median about $21.40; Ardent Partners about $12.88 for non-best-in-class). These are secondary citations by vendors, so verify before reuse (M–L) [40].
- Competitors: Nanonets about $0.30 per page pay-as-you-go; Docsumo from about $299/month (conflicting third-party data, L–M) [41]. Also Rossum, Dext, and Hubdoc (bundled with Xero).

**Gap.** No evidenced SMB complaint pattern was collected. LLM-based extraction is commoditising quickly (J).

**Liability.** High for KYC and claims (AML rules; in PH, BSP and Data Privacy Act); lower for invoice capture.

**Deliverability.** Medium. Extraction is easy; validation against ledgers and exception handling is where effort goes.

**Retention.** Volume-based and sticky once tied into an accounting workflow.

**Disproof.** Target bookkeepers already get "good enough" capture bundled in Xero, QuickBooks or Dext.

### 3.6 SME back-office: AR collections, payment reminders, bookkeeping prep

**Who.** B2B service SMEs, contractors, agencies, wholesalers, and their bookkeepers.

**Current workflow.** The owner or bookkeeper exports aged receivables and emails or phones debtors by hand, or uses Xero/QuickBooks' basic reminders.

**Evidence:**

- **QuickBooks 2025 Late Payments Report [38]** (vendor-commissioned survey, January 2025, 2,487 US small businesses, M–H).
  - **56% are owed money** from unpaid invoices, averaging **about $17.5k**.
  - **47% have invoices more than 30 days overdue**.
- UK government research is cited by secondary sources as about 86 hours a year spent chasing per affected firm (unverified, L).

**Competitors (published prices):**

- Chaser from **$259/month** [39]; Paidnice from **$69/month** [39] (directory listings, M).
- Upflow is quote-only and mid-market.
- Xero and QuickBooks Online have built-in reminders.

**Why the gap might exist (J, not evidenced).** Built-in reminders are template-based and can't read replies ("we paid last week", "invoice is wrong"). Dedicated tools are priced for firms with finance staff.

**Buyer.** Owner, or the outsourced bookkeeper, who is also a channel (one bookkeeper serves many clients).

**Integrations.** Xero and QuickBooks Online have public OAuth APIs (product docs, H); email; optionally SMS or WhatsApp.

**Sensitivity.** Financial data, medium. The tool only nudges and never moves money.

**Deliverability.** High: read invoices, draft and send sequenced reminders, classify replies, escalate.

**Retention.** Cash collected is measurable every month.

**Disproof.** Pilot users' days sales outstanding (DSO) don't move, or bookkeepers won't let an agent email their clients' customers.

### 3.7 Vertical workflows

| Vertical | Strongest evidenced pain | Notes |
|---|---|---|
| **Dental / clinics** | No-shows and late cancels are the top schedule barrier for 81–83% of US dentists [16]; staffing shortages [17]; outpatient no-shows ~23% [11] | Crowded (Weave, Podium and many AI startups); PMS integrations gated; health-data liability. High value, hard for a solo team. |
| **Home services** | Answer rates of 32–74% across sub-industries [4]; 55% don't ask to book [4] | Jobber and ServiceTitan bundle AI. Good for a lead and quote follow-up layer. |
| **Real estate** | Speed-to-lead claims are mostly unverified [see 3.1] | GoHighLevel and kvCORE-type CRMs dominate. |
| **Salons** | No-shows only 1–4% in NA [14][15] | **Weak wedge.** |
| **Travel/tours, tutoring/review centres** | No primary evidence found (hypothesis) | Messaging-heavy and seasonal; plausible fit for SEA (see the Philippines document). |
| **Agencies** | Not a pain owner. A **channel**: GoHighLevel's model shows agencies resell automation to SMBs [28] | |

### 3.8 Research, reporting, decision support

No SMB-specific evidence of payment was found. General AI assistants (ChatGPT, Claude, Gemini, Microsoft Copilot) serve this need at low prices. Severity and frequency look low for micro firms (J). **Not recommended** as a first product.

### 3.9 Workflow glue between existing SME software

Payment is proven, but at low price points:

- Zapier: about $20–30/month entry.
- Make: about $9–11/month for 10,000 operations.
- n8n: Cloud from about $20/month; **free to self-host** [43] (third-party, M).

Competitive room is minimal. A Rust engine might compete on cost or self-hosting, but that is a developer market, not the business buyer this brief targets. **Better as an internal capability of the platform than as the first product (J).**

---

## 4. Cross-cutting findings

1. **Willingness to pay is proven only where the product is sold as an outcome**: answered calls, booked jobs, collected cash. Sources: Beside [18], respond.io [31], Goodcall and Smith.ai [22][23], Chaser [39]. General "AI adoption" surveys [7][9] measure interest, and official data shows micro-firm AI use is flat [6].
2. **Platform owners and vertical suites are absorbing the obvious features**:
   - Meta Business Agent on WhatsApp, globally since June 2026 [33], and on Messenger in the Philippines since October 2025 [34].
   - Jobber Receptionist, with a quote-drafting "Companion" coming [25].
   - Weave's AI receptionist [26], Podium's AI Employee [27], GoHighLevel's AI Employee [28].

   A first product should sit in a gap these bundles don't cover: cross-tool follow-through, a vertical they ignore, or markets they serve poorly.
3. **Prices split into two tiers (derived).**
   - Prosumer/micro: about $17–100 a month. Beside's derived ~$17/month average [18]; Goodcall $79 [22]; Rosie $49 [24].
   - SMB with staff: about $250–600 a month. respond.io's derived ~$290/month [31]; Chaser $259 [39]; Podium $399+ [27].

   A small team needs either very low support cost at the micro tier, or a buyer at the higher tier.
4. **Voice is commoditised infrastructure.** Retell's estimated $45–60M ARR on about $5M raised [21], and per-minute orchestration at $0.05–0.31 [44], mean voice quality is rented, not built. A Rust core offers no differentiation there. Its advantages are reliability, cost and durability for long-running, multi-step follow-up workflows (J).
5. **Caller and customer acceptance of AI is the biggest unmodelled risk for voice.** Owners report hang-ups and turning the feature off [30]. Text-first, human-approved drafts avoid most of this (J).
6. **Many widely repeated statistics are untraceable or vendor-made.** Examples: "85% won't call back", "78% choose the first agent", "21× in 5 minutes", "salon no-shows 10–25%". Pitch claims should rely on the H/M sources here: HBR [1], Invoca [4], Dantas [11], Census [6], QuickBooks [38].
7. **SEA relevance.**
   - The leading omnichannel messaging vendors for the region are SEA-based or SEA-focused: respond.io (Kuala Lumpur) [31] and SleekFlow [32].
   - Meta has picked the Philippines as a Business AI launch market [34].
   - Messaging-first inquiry handling is therefore validated *and* contested in the region. Philippines-specific demand is covered in the separate document.
8. **Gartner's prediction that 40%+ of agentic projects will be cancelled [10]** is about enterprises, but it signals buyer scepticism. Products with human-approval gates and visible ROI counters are better placed (J).

---

## 5. Which 2–3 look strongest for a small team (judgement, based on the evidence above)

1. **Lead and quote follow-through agent for one service vertical** (part of #1 and #4). Text, email and messaging first, with human-approved drafts.
   - Evidence: the cost of slow follow-up [1][4] and the 55% "never ask to book" behaviour gap [4].
   - Buyers already pay $50–97 per sub-account for adjacent AI [28].
   - Low liability; public integrations.
   - Watch-out: GoHighLevel and Jobber bundles, so pick a vertical or region they underserve.
2. **AR collections and payment-reminder agent on Xero and QuickBooks Online** (#6).
   - Evidenced pain at scale [38]; published prices at $69–259 a month [39].
   - Public APIs; measurable ROI (cash collected); bookkeepers as a multiplier channel.
   - Little evidence of platform-owner AI competition so far (absence of evidence, J).
3. **Messaging-first inquiry-to-booking for an SEA vertical** (#2 + #7; conditional).
   - Strong payment evidence in the region [31][32].
   - Viable only if it delivers workflow Meta's free agent won't (bookings, deposits, quotes, multi-channel including Viber and email). Validate against Meta Business Agent first [33][34].

**Not recommended first:**

- **Voice AI receptionist** (#3): the most funded and most crowded, bundled by incumbents, with caller-rejection risk.
- **Generic workflow glue** (#9) and **research/reporting** (#8).
- **Salon no-show reduction**: disproved by the 1–4% rate [14].
- **KYC and claims document processing** (#5): high liability.

---

## 6. Sources

All accessed 2026-10-18. Type abbreviations: PR = primary research, OS = official statistics, PD = product docs, VC = vendor claim, CT = customer testimony, IA = independent analysis, SEC = secondary report of another source. Conf. = confidence.

| # | Title / description | Publisher | Date | URL | Type | Geography | Limitations | Conf. |
|---|---|---|---|---|---|---|---|---|
| 1 | The Short Life of Online Sales Leads (Oldroyd, McElheran, Elkington) | Harvard Business Review | Mar 2011 | https://hbr.org/2011/03/the-short-life-of-online-sales-leads ; record: https://scholarsarchive.byu.edu/facpub/9711 | PR | US | Paywalled; figures read via secondary quotes; old | M |
| 1a | "Are you letting online leads go cold?" (reports 2,241-company audit: 37% <1h, 23% never) | SmartCompany | 4 Jul 2011 | https://www.smartcompany.com.au/marketing/20110704-are-you-letting-online-leads-go-cold/ | SEC | US/AU | Second-hand | M |
| 2 | Lead Response Management Study (Oldroyd / InsideSales, MIT) | InsideSales.com (hosted PDF) | 2007 | https://www.mortech.com/hs-fs/hub/25649/file-13535879-pdf/docs/mit_study.pdf | PR + VC | US | Six companies; vendor co-author; conflicting multipliers (100×/21× vs 10×/6×) | L–M |
| 3 | Lead Response Times Based on 1000 B2B Websites | RevenueHero | Undated (©2026) | https://revenuehero.io/resources/lead-response-times-report | VC | Global B2B | Method undisclosed; sells routing software | L–M |
| 4 | Home Services Lead Conversion Benchmarks Report 2026 | Invoca | Jul 2026 | https://invoca.com/reports/the-invoca-home-services-lead-conversion-benchmarks-report-2026 | VC (platform data) | US | Invoca customers only; sells call AI | M |
| 5 | CallRail Releases Report Benchmarking Marketing Efforts for Small Businesses | CallRail via BusinessWire | 14 Jan 2025 | https://www.businesswire.com/news/home/20250114793850/en/CallRail-Releases-Report-Benchmarking-Marketing-Efforts-for-Small-Businesses | VC | US | Customer data; "85%" claim unsourced | M |
| 6 | Large Firms With at Least 20 Employees Biggest AI Users (Grundy, Breaux, Khatiwoda) | US Census Bureau | 26 May 2026 | https://census.gov/library/stories/2026/05/ai-use-businesses.html | OS | US | Question reworded Nov 2025 | H |
| 7 | The New AI Divide (BTOS analysis) | WeWork | 2026 | https://www.wework.com/ideas/research-insights/the-new-ai-divide | IA/SEC | US | Not fetched directly; third-party read of Census data | L–M |
| 8 | Generative AI and the SME Workforce: New Survey Evidence | OECD | 2024–25 (exact date not verified) | https://www.oecd.org/en/publications/generative-ai-and-the-sme-workforce_2d08b99d-en.html | PR | 7 OECD countries | 2024 fieldwork; page fetch blocked, figures via search summary | M–H |
| 9 | Nearly 80% of SMBs with AI say it will be a game changer | Salesforce | 4 Dec 2024 | https://salesforce.com/news/stories/smbs-ai-trends-2025 | VC (commissioned survey) | Global | Self-reported; vendor sells AI | M |
| 10 | Gartner Predicts Over 40% of Agentic AI Projects Will Be Canceled by End of 2027 | Gartner | 25 Jun 2025 | https://www.gartner.com/en/newsroom/press-releases/2025-06-25-gartner-predicts-over-40-percent-of-agentic-ai-projects-will-be-canceled-by-end-of-2027 | IA | Global enterprise | Forecast; enterprise focus | M |
| 11 | No-shows in appointment scheduling: a systematic literature review (Dantas et al., Health Policy) | Elsevier (ref page: Textbook of Digital Health) | 2018 | https://www.textbookofdigitalhealth.com/references/dantas2018noshows.html | PR (systematic review) | Global | Heterogeneous settings | H |
| 12 | Mobile phone messaging reminders for attendance at healthcare appointments (CD007458) | Cochrane | 2013 update | https://www.cochrane.org/CD007458 | PR (systematic review) | Global | Low–moderate evidence quality | H |
| 13 | Using digital notifications to improve attendance in clinic: systematic review and meta-analysis (Robotham et al.) | BMJ Open / PubMed | 2016 | https://pubmed.ncbi.nlm.nih.gov/27798006/ | PR | Global | Mixed study designs | H |
| 14 | No-show revenue calculator (Zenoti 2026 Beauty & Wellness Benchmark) | Zenoti | 13 May 2026 | https://www.zenoti.com/en-uk/thecheckin/no-show-revenue-calculator | VC (platform data) | North America | Zenoti customers, mostly larger and with online booking | M–H |
| 15 | Beauty salon no-shows: UK locations with highest rates (Treatwell data) | Scratch Magazine | Mar 2025 | https://scratchmagazine.co.uk/news/beauty-salon-no-shows-uk-locations-highest-rates | VC via trade press | UK | Page blocked (403); figures via search summary | M |
| 16 | Patient no-shows, cancellations still biggest hurdle for dental practice schedules (ADA HPI polls) | Becker's Dental | 2024–25 (monthly series) | https://www.beckersdental.com/dentists/patient-no-shows-cancellations-still-biggest-hurdle-for-dental-practice-schedules/ | OS-like (professional body poll) via trade press | US | Page blocked; figures via search summary; measures a barrier, not a rate | M |
| 17 | Staffing shortages top expected challenges this year | ADA News | Mar 2025 | https://adanews.ada.org/new-dentist/2025/march/staffing-shortages-top-expected-challenges-this-year | PR (ADA survey) | US | Via search summary | M |
| 18 | Exclusive: Beside, an AI voice startup, raises $32 million to build an AI receptionist for small businesses | Fortune | 11 Nov 2025 | https://www.fortune.com/2025/11/11/beside-ai-voice-startup-raises-32-million-ai-receptionist-for-small-business/ | VC (company disclosure via press) | US/UK/global | Unaudited ARR and customer count | M–H |
| 19 | Newo raises USD $25M to scale AI reception platform | IT Brief / GlobeNewswire | 10 Feb 2026 | https://itbrief.news/story/newo-raises-usd-25m-to-scale-ai-reception-platform | VC (press release) | US/global | Company figures | M |
| 20 | Synthflow raises $20M Series A | Synthflow | Jun 2025 | https://synthflow.ai/news/synthflow-raises-20m-series-a | VC | EU/US | Revenue via CB Insights estimate | M |
| 21 | Retell AI company profile | Sacra | 2026 | https://sacra.com/c/retell-ai | IA (estimate) | US/global | Estimates, not disclosures | M |
| 22 | Goodcall pricing | Goodcall | Accessed 2026-10-18 | https://www.goodcall.com/pricing | PD | US | Third-party pages list older prices | H |
| 23 | Smith.ai AI receptionist pricing | Smith.ai | Accessed 2026-10-18 | https://smith.ai/pricing/ai-receptionist | PD | US | Tier display is ambiguous | H |
| 24 | Rosie pricing | Rosie | Accessed 2026-10-18 | https://heyrosie.com/pricing | PD | US | Overage not stated | H |
| 25 | Jobber Receptionist feature page; Jobber add-on price via competitor blog | Jobber; Beside | 2026 | https://www.getjobber.com/features/ai-receptionist/ ; https://www.beside.com/blog/jobber-ai-phone-answering | PD; VC (competitor) | US/CA | Price not on official page; competitor source | M |
| 26 | Weave forecasts $275M–$278M 2026 revenue while expanding omnichannel AI Receptionist | Seeking Alpha | Feb 2026 | https://seekingalpha.com/news/4583517-weave-forecasts-275m-278m-2026-revenue-while-expanding-omnichannel-ai-receptionist | SEC (earnings call) | US | Add-on pricing undecided at the time; PMS limits via third-party review | M |
| 27 | Podium pricing (third-party) and Q4 2025 pricing-page removal | Prospeo; PricingSaaS | 2025–26 | https://prospeo.io/s/podium-pricing-reviews-pros-and-cons ; https://pricingsaas.com/companies/podium/diffs/2024Q3 | SEC | US | Not official | L–M |
| 28 | HighLevel pricing (plans and AI Employee add-on) | HighLevel | Accessed 2026-10-18 | https://www.gohighlevel.com/pricing | PD | Global | AI usage billed separately | H |
| 29 | AI customer service enters next phase as businesses focus on optimisation (Upfirst data: 450,702 calls, 503 AI receptionists) | Devdiscourse / ANI | 31 Jul 2026 | https://www.devdiscourse.com/article/business/3957545-ai-customer-service-enters-next-phase-as-businesses-focus-on-optimisation-rather-than-adoption-report | VC via news | US | Primary report not found | M |
| 30 | "AI in service businesses" forum thread | Jobber Community | 2025–26 | https://community.getjobber.com/discussions/operations-forum/ai-in-service-businesses/1426/replies/5532 | CT | US/CA | Anecdotal; page blocked, content via search summary | L–M |
| 31 | Respond.io puts Malaysia on the global software map with huge $62.5M investment | Asia Tech Review | 17 Jun 2026 | https://www.asiatechreview.com/p/respondio-puts-malaysia-on-the-global | VC via press | SEA/global | CEO-stated ARR | M–H |
| 32 | SleekFlow unveils AgentFlow; SleekFlow Series A+ funding release | Malay Mail; SleekFlow | 14 Jul 2025; Aug 2024 | https://www.malaymail.com/news/money/mediaoutreach/2025/07/14/sleekflow-unveils-agentflow-building-teams-of-ai-agents-to-increase-revenue-for-businesses/394893 ; https://sleekflow.io/de-de/news/sleekflow-funding-aug-2024-de | VC | HK/SEA | 2025 round only via aggregators | M |
| 33 | Meta's AI agent for WhatsApp Business is now available globally | TechCrunch | 3 Jun 2026 | https://www.techcrunch.com/2026/06/03/metas-ai-agent-for-whatsapp-business-is-now-available-globally/ | SEC (journalism) | Global | No pricing figures | H |
| 34 | Meta brings Business AI to the Philippines | Marketech APAC | 20 Oct 2025 | https://marketech-apac.com/meta-brings-business-ai-to-the-philippines-to-enhance-customer-engagement-for-local-businesses/ | VC (Meta) + CT | PH | Single case study | M |
| 35 | 52 WhatsApp Business Statistics for 2026 (cites Meta Q4 2025 earnings) | ChatMaxima | 2026 | https://chatmaxima.com/blog/whatsapp-business-statistics-2026/ | SEC | Global | Verify against the Meta transcript | M |
| 36 | WATI reviews; WATI vs ManyChat comparison | G2; WANotifier | 2025–26 | https://g2.com/products/wati/reviews ; https://wanotifier.com/compare/wati-vs-manychat/ | CT; SEC | India/global | Small review sample; competitor comparison site | M |
| 37 | ManyChat pricing | Hackceleration | 2026 | https://hackceleration.com/labs/manychat-pricing | SEC | Global | Conflicting figures | L–M |
| 38 | 2025 Small Business Late Payments Report | Intuit QuickBooks | 28 May 2025 | https://quickbooks.intuit.com/r/small-business-data/small-business-late-payments-report-2025/ | VC (commissioned survey) | US | Vendor sells payments | M–H |
| 39 | Chaser pricing (Capterra); Chaser vs Paidnice (GetApp AU) | Capterra; GetApp | 2026 | https://capterra.com/p/157101/CHASER/ ; https://www.getapp.com.au/compare/107821/2059198/chaser/vs/paidnice | SEC (directory) | Global/AU | Directory snapshot | M |
| 40 | Invoice processing cost benchmarks (cites APQC and Ardent Partners) | Lido | 2026 | https://www.lido.app/blog/invoice-processing-cost-benchmarks | SEC/VC | US/global | Primary reports not read | L–M |
| 41 | Docsumo vs Nanonets; Nanonets alternatives | ToolRadar; Lido | 2026 | https://toolradar.com/compare/docsumo-vs-nanonets ; https://www.lido.app/blog/best-nanonets-alternatives | SEC | Global | Conflicting prices | L–M |
| 42 | PandaDoc pricing 2026; Proposify pricing | Bind Legal; Verdocs | 2026 | https://bindlegal.com/resources/comparisons/pandadoc-pricing-2026/ ; https://verdocs.com/blog/proposify-pricing | SEC | Global | Third-party | M |
| 43 | Make vs Zapier vs n8n (pricing) | Cut the SaaS | 2026 | https://cut-the-saas.com/guides/make-vs-zapier-vs-n8n | SEC | Global | Third-party; prices conflict | M |
| 44 | Bland AI alternatives and pricing | Fora Soft | Jul 2026 | https://www.forasoft.com/blog/article/bland-ai-alternatives | SEC | Global | Agency blog | M |
| 45 | What does Lindy cost in 2026? | Automation Atlas | Jul 2026 | https://automationatlas.io/answers/lindy-pricing-explained-2026/ | SEC | Global | Funding totals conflict across sources | M |
| 46 | Runable company profile ($21M Series A, Aug 2026, AI agents for SMB operators) | aifunding.me | Aug 2026 | https://aifunding.me/companies/runable | SEC | US | Aggregator only | L |
| 47 | Jobber automations / quote follow-ups help article | Jobber Help Center | 2026 | https://help.getjobber.com/en/articles/automations/ | PD | US/CA | No outcome data; "28% lift" claim untraceable | H for feature, L for lift |
