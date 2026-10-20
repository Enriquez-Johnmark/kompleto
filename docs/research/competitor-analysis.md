# Competitor & Substitute Analysis: AI Inquiry, Booking, Quote Follow-up and Document Intake for Small Businesses (Philippines first)

- **Prepared:** 2026-10-18. All sources were accessed on 2026-10-18 unless noted.
- **Scope:** This file covers competitors and substitutes. Market sizing is covered in a separate document.
- **Product directions tested:** (a) AI inquiry handling and lead follow-up on Messenger, Instagram, WhatsApp, Viber, web and email; (b) booking, scheduling and reminders; (c) quotation and proposal follow-up; (d) document intake and extraction.
- **Evidence labels:** **[Primary]** means the vendor's own page or official docs. **[Vendor claim]** means a vendor's self-reported marketing claim. **[Press]** means a news outlet. **[3P]** means a third-party blog, review aggregator or reseller. **[Review]** means G2, Capterra or Trustpilot. Bracketed numbers point to the Sources list at the end.
- **Method limits:** Several pricing pages are rendered with JavaScript or blocked automated fetches (ManyChat, Pipedrive, HubSpot's main platform page, Klippa, Google Document AI). Where that happened, I either used third-party summaries and labelled them **[3P]**, or I wrote "not retrieved". Nothing in this file infers market share, retention or revenue.

---

## 1. Key conclusions

1. **Meta now gives an AI agent to every business on Messenger, WhatsApp and Instagram, and it is free to start. This is the most important competitive fact for direction (a). Confidence: high.**
   - Meta Business Agent (earlier called "Business AI") went global on 2026-06-03. It answers questions, recommends catalogue products, books appointments, qualifies leads and hands off to a human [1][2].
   - Meta launched it on Messenger in the Philippines on 2025-10-20 [3].
   - Use is free up to a monthly allowance that Meta has not published. Above that, businesses pay through the Meta One business plans: US$14.99, US$49.99, US$149 or US$499 per month (launched 2026-09-15). Meta has not published how many agent responses each tier includes [5][6][7].
   - On the WhatsApp Business Platform (API), Meta's agent costs US$2.00 per 1M tokens, which Meta estimates at about US$0.04–0.05 per message, effective 2026-08-01 [8].
   - **Implication:** a standalone "AI that answers FAQs in your FB Page inbox" is being commoditised by the platform owner. A new product cannot be built mainly on that.

2. **Meta's agent leaves gaps that the published evidence documents. These are the clearest openings.** Confidence: medium.
   - It replies only when the customer messages first [5].
   - Its controls are per app [5]: audience filters, topics to avoid, and automatic pause when a human takes over [4][5].
   - Calendar management is a stated future plan, not a current feature [1][2].
   - It works only on Meta channels: no Viber, SMS or email. A website agent exists only via Meta Business Suite, according to a third-party source [5].
   - No draft-and-approve mode is documented. The model is "AI replies, human can take over" [4][5].
   - Meta itself warns that replies may be inaccurate, and the business stays responsible for what the agent says [5][66].

3. **Proactive follow-up on Messenger got structurally harder in 2026. That makes it both a pain point and a moat question.** Confidence: high.
   - From 2026-04-27, the Messenger message tags CONFIRMED_EVENT_UPDATE, ACCOUNT_UPDATE and POST_PURCHASE_UPDATE return error 100 [16].
   - Meta points businesses to Utility Templates or the Marketing Messages API instead [16].
   - Recurring Notifications were switched off on 2026-02-10 everywhere except AU, the EU, JP, KR and the UK, according to Meta documentation as relayed by ManyChat [17].
   - The HUMAN_AGENT tag (a 7-day window, for human replies only) was not on the deprecation list [17].
   - **Implication:** "AI lead follow-up" on Messenger is now limited to the 24-hour window, paid or approved templates, or human-sent messages. A follow-up product has to move the conversation to channels the business controls (SMS, Viber, email, phone), or keep a human in the send loop.

4. **WhatsApp economics changed on 2026-10-01.** Confidence: high for the structure, medium for the Philippine rate.
   - Free-form (non-template) service messages that are not powered by Meta Business Agent are now charged at the market's utility rate [8].
   - Utility templates sent inside the 24-hour window are also charged now [8].
   - Meta partners report a 1,000-message monthly free allowance per number. Meta's non-template pricing page does not state that allowance [12].
   - The Philippines (Rest of APAC) rate reported by a partner is about US$0.0113 per service message. Philippine marketing templates rose from US$0.0732 to US$0.0842 [12].
   - **Implication:** every third-party WhatsApp bot now has a per-reply cost floor. Meta's own agent on the API (about US$0.04–0.05 per message) is not cheaper than a third-party bot plus a cheap LLM.

5. **The WhatsApp "AI chatbot ban" does not block a business-specific assistant.** Confidence: high.
   - From 2026-01-15, the API terms prohibit AI providers whose general-purpose assistant is the "primary functionality" (for example ChatGPT, Perplexity, Copilot) [13].
   - Bots that serve a business's own customers remain allowed [13].
   - The EU and Italy forced a paid carve-out for general-purpose bots [14]. That does not affect a Philippine SME assistant.

6. **Omnichannel inbox vendors are mature, priced around US$79–US$300+ per month, and aimed at teams.** Confidence: high.
   - respond.io (from US$79/mo annual, with AI Agents from Growth at US$159/mo) [22], SleekFlow (from US$99/mo, with an ASEAN price view) [24], WATI, Trengo (from €299/mo) [31] and Kommo (US$25–45 per user per month, 6-month minimum) [32] all already sell LLM agents with human handoff.
   - Their entry prices are high for a 1–5-person Philippine business, and they are self-serve tools that still require configuration.

7. **ManyChat and Chatfuel dominate DIY Messenger and Instagram automation, but their trust problems are documented.**
   - ManyChat moved to active-contact pricing on 2026-03-02 and cut its free plan to 25 contacts (third-party reports) [18][19].
   - ManyChat's Trustpilot score is 2.1/5 from 306 reviews. The recurring themes are billing surprises, support quality and automations that misfire [20].
   - Their automation is mostly flow-based with AI steps added on.

8. **GoHighLevel is a distribution channel as much as a competitor in the Philippines.**
   - Philippine VAs are hired specifically as "GHL specialists" on OnlineJobs.ph [35].
   - HighLevel's white-label/SaaS mode (Agency Pro at US$497/mo) and its AI Employee add-on (US$50 or US$97 per sub-account per month) let agencies resell AI chat and voice [33][36].
   - Reviews repeatedly cite a 4–8 week learning curve and uneven support [34].

9. **Philippine-made competitors are either services agencies or small commerce-bot startups. No well-funded local AI-agent SaaS turned up.** Confidence: medium (absence of evidence).
   - Bots at Work charges ₱40–50k to build plus a ₱25–45k monthly retainer (vendor-published) [47].
   - ChatGenie is a Messenger commerce platform with about US$1.1M raised in total, per CB Insights [49].
   - Pancake/Botcake (Vietnamese) is heavily used by Facebook sellers [50][51].
   - SeriousMD/NowServing serves clinic booking [46].
   - Globe Business sells general "AI packages" from about ₱1,200/mo, with no chatbot product named [53].
   - The real price anchor is a VA or chat CSR at roughly ₱25–35k/month (third-party salary data) [54].

10. **Quote follow-up and SME document intake look under-served at Philippine SME price points, but demand is unproven.**
    - Document AI vendors are priced for mid-market buyers: Rossum from US$18,000/yr [57], Docsumo by quote plus setup fees [56], Nanonets US$100/mo for credits [55].
    - Proposal tools (PandaDoc and similar) are seat-priced e-signature tools, not chat-native follow-up [64].
    - No competitor found joins "Messenger inquiry → quote → follow-up → booking" for Philippine service SMEs. Confidence: low–medium, because this rests on absence of evidence.

---

## 2. Master matrix

The "AI depth" column uses three levels: **R** means rule or flow-based, **L** means an LLM agent answers, **A** means the agent also takes autonomous actions such as booking or CRM writes. The "Approval model" column uses: **HO** for human handoff/takeover, **D** for draft-for-human (copilot) mode, and **Auto** for autonomous replies. PH presence is based on published evidence only.

| Competitor | Segment | Channels | AI depth | Approval model | Entry price (published) | PH presence |
|---|---|---|---|---|---|---|
| **Meta Business Agent** | Native platform agent | Messenger, WhatsApp, IG (rolling), web agent via MBS [5] | L + A (catalogue, booking; calendar planned) | Auto + HO; topics to avoid; audience filters | Free allowance; Meta One from US$14.99/mo; API US$2/1M tokens [6][8] | **Yes**: launched on Messenger 2025-10-20 [3] |
| ManyChat | DIY Messenger/IG automation | IG, Messenger, WhatsApp, TikTok, SMS, email | R + AI steps | Flows + live chat | Free (25 contacts); Essential ~US$14–17/mo [19] (3P) | Global self-serve; no PH entity found |
| Chatfuel | DIY bot builder | FB, IG, WhatsApp, TikTok, web | R + L | HO via inbox | US$18/mo annual incl. US$20 AI credits [21] | Global self-serve |
| respond.io | Omnichannel inbox + AI agents (Malaysia) | WhatsApp, Messenger, IG, TikTok, email, more | L + A (workflows) | HO; AI Assist (D) | US$79/mo annual Starter; AI Agents from Growth US$159/mo [22] | SEA vendor; Abenson listed as a customer (vendor claim) [23] |
| SleekFlow | Omnichannel inbox + AgentFlow (HK) | WhatsApp, IG, Messenger, TikTok, **Viber**, LINE, Telegram, email | L + A | HO (with summary) | US$99/mo Pro (ASEAN price view) [24] | ASEAN pricing view; no PH office verified |
| WATI | WhatsApp-first inbox | WhatsApp (+FB/IG/TikTok/widget) | R + L (Astra add-on) | HO; AI Co-pilot (D) | Base price not retrieved; Astra "priced separately" [27] | Global |
| Trengo | EU omnichannel helpdesk | Email, WhatsApp, IG, FB, voice, SMS, live chat | L (HelpMate) | HO | €299/mo annual (Boost) + €0.25–0.30 per AI conversation [31] | None found |
| Tidio (Lyro) | Web chat + AI for SMB e-commerce | Live chat, Messenger, IG, WhatsApp, email | L | HO | Free; Starter US$24.17/mo; Lyro from US$32.50/mo (50 conv.) [29] | Global self-serve |
| Intercom Fin | Upper-bound AI support | Chat, email; WhatsApp/SMS pay-as-you-go | L + A | HO | US$29/seat + US$0.99/outcome [30] | Global |
| Kommo | Messenger-first CRM | WhatsApp, IG, Messenger, Telegram, TikTok | L + A (AI booking on Pro) | HO | US$25/user/mo, billed per 6 months [32] | Global |
| GoHighLevel | Agency all-in-one + white-label | SMS, email, FB/IG, WhatsApp (US$10/mo), voice, web chat | L + A (Conversation/Voice AI) | Auto or suggest mode (3P) | US$97/mo; AI Employee US$50–97 per sub-account [33] | **Strong VA/agency ecosystem** [35] |
| HubSpot Starter / Breeze | SMB CRM | Email, chat, FB Messenger (inbox) | L (Customer Agent mainly Pro+) | HO | Starter US$7–20/seat; 500 credits [37] | Global |
| Zoho CRM (Zia) | SMB CRM | Email, web; messaging via other Zoho apps | L in Professional+ | — | US$14/user/mo annual Standard (3P) [39] | Zoho sells in PH; no PH-specific pricing found |
| Calendly | Scheduling links | Web, email | AI (Callie beta, English) | — | Free; US$10/seat/mo annual [41] | Global |
| Fresha | Salon/spa booking marketplace | Web, FB/IG booking, SMS/WhatsApp reminders | — | — | ₱900/mo solo; ₱600/member/mo team [42] | **Yes**: peso pricing published |
| SimplyBook.me | Booking SaaS | Web, FB/IG; SMS/WhatsApp/AI voice credits | AI assistant + AI voice add-on | — | Free; €11.9/mo Basic [43] | Global |
| SeriousMD / NowServing | PH clinic EMR + booking | App, web, SMS | None stated | — | Free (5% + ₱75 service fee); Pro ₱1,950/mo [46] | **PH-made** |
| Bots at Work | PH done-for-you chatbot agency | Messenger, IG, Viber, WhatsApp, web, TikTok, SMS | L | Agency-configured | ₱40–50k build + ₱25–45k/mo (vendor) [47] | **PH-made** |
| ChatGenie | PH social-commerce bot | Messenger, social | R + L | — | Not published [47] | **PH-made**; ~US$1.1M raised (CB Insights) [49] |
| Pancake / Botcake | FB seller inbox + bot (Vietnam) | FB, IG, more | R + L (BYO OpenAI) | — | Botcake free; Pro from US$30 [50] | PH partner site [51] |
| Zapier / Make / n8n / Lindy | General automation | Any via integrations | L + A | Lindy: approvals built in [62] | Zapier US$19.99/mo; n8n €20/mo; Lindy US$49.99/mo [59][61][62] | Global |
| Nanonets / Docsumo / Rossum | Document AI | Email, API | L extraction | Human validation | Nanonets US$100/mo credits; Rossum from US$18k/yr [55][57] | None found |
| VA / CSR (manual) | Labour substitute | All | Human | Human | ~₱25–35k/mo full-time (3P) [54] | **Default substitute** |

---

## 3. Per-competitor profiles

### 3.1 Meta Business Agent (formerly "Business AI"). The most important competitor.

- **Who it's for:** every business on Meta's apps, from the solo WhatsApp Business app user up to enterprises on the Business Agent Platform [1].
- **Problem it solves:** answering customers around the clock with no extra tool.
- **Capabilities (Meta's own description):**
  - Answers business-specific questions, recommends products from the catalogue, books appointments, qualifies leads and closes sales [1].
  - Replies in the customer's language and in the business's tone [1].
  - Gives the owner a "morning briefing" of overnight chats. This is limited to select businesses and has a waitlist [1].
  - Planned but not yet live: market research, calendar management, competitive intelligence, and discovery in WhatsApp search [1][2].
  - What it learns from: Page posts, past WhatsApp chats, website, catalogues, price lists, FAQs and uploaded documents [4].
- **Channels:**
  - WhatsApp and Messenger, with Instagram newly added [1].
  - A website agent created in Meta Business Suite and connectable to Shopify, according to one third-party source [5].
  - No Viber, SMS or email.
- **Enterprise integrations:** the Business Agent Platform connects to "hundreds of systems like Shopify, Zendesk, and Shopee" [1]. Meta does not mention CRM integration for small businesses.
- **Controls:**
  - Audience: everyone, new customers only, or customers who arrived from ads [4].
  - Topics to avoid, active hours, and a test chat before going live [4][5].
  - Replying manually takes over the chat and stops the AI in that conversation. Customers can ask for a person [5].
  - Settings are per app [5].
  - Meta warns that replies may be inaccurate [5]. A third-party guide stresses that the business remains liable for wrong prices or promises [66].
  - No pre-send draft approval is documented.
- **Pricing:**
  - "Getting started is free" [1]. The WhatsApp Business app has a monthly response allowance; once it runs out, the business waits for it to refresh or subscribes to Meta One (3P description) [5].
  - Meta One business tiers: Essential US$14.99, Advanced US$49.99, Expert US$149 and Max US$499 per month. Each step up gives "more Meta Business Agent responses", but Meta has published no quotas, and "pricing and availability may vary by region" [6][7].
  - API (Platform): US$2.00 per 1M tokens from 2026-08-01, estimated at about US$0.04–0.05 per message. Token charges apply even inside the 72-hour free entry-point window [8].
- **Philippine evidence:** launched on Messenger in the Philippines on 2025-10-20 [3].
  - Meta Philippines country director Jerry Bongco promoted it [3].
  - White Coat Manila reported a 20% cut in support costs. This is a customer testimonial distributed by Meta [3].
- **Traction:** "more than one million businesses" use it on WhatsApp and Messenger, per Meta's own claim [1].
- **Gaps and complaints:** I found no independent review corpus on G2, Reddit or similar. The gaps that can be documented are:
  - Inbound only.
  - Calendar integration not live.
  - Meta channels only.
  - Undisclosed quotas.
  - An accuracy disclaimer.
  - Chat data reportedly used to improve Meta's AI (a 3P claim, unverified) [67].
- **Openings:**
  - Work across channels, including Viber and email.
  - Draft-and-approve control for owners who fear wrong prices or quotes.
  - Booking written into the owner's real calendar or clinic system.
  - Follow-up after the 24-hour window through owned channels.
  - Taglish/Filipino quality (Meta's language quality in Taglish is not documented).
- **Confidence:** high on features and pricing structure.
- **Unresolved:**
  - The size of the free allowance in the Philippines.
  - Meta One prices in pesos.
  - Whether Instagram and web agents are live for Philippine accounts.
  - Booking depth: whether it writes to any external calendar.

### 3.2 ManyChat

- **Who it's for:** creators and SMBs who sell through Instagram and Messenger.
- **Problem it solves:** comment-to-DM automation, lead capture and broadcasts.
- **Automation depth:** a visual flow builder with AI steps such as intent recognition and an "AI Step" [18]. Third-party reviews describe it as mostly rule-guided [19].
- **Channels:** Instagram, Messenger, WhatsApp, TikTok, SMS and email.
- **Pricing:** the official page blocked automated fetching (HTTP 403), so these figures are third-party.
  - New accounts since 2026-03-02 have five tiers based on active contacts [19]: Free (25 contacts), Essential (250), Pro (2,500), Business (7,500) and Advanced (25,000).
  - Reported monthly-billed prices are US$0, 17, 39, 99 and 199. Annual billing is reported at US$14, 29, 69 and 139 [19].
  - Third-party sources disagree on whether AI is included or costs a separate US$29/mo [18][19].
  - Treat all of these figures as medium confidence.
- **Onboarding:** self-serve, with a large agency and freelancer ecosystem.
- **Complaints:** Trustpilot 2.1/5 from 306 reviews [20]. Themes:
  - Contact-based billing surprises, difficulty cancelling, and charges after cancellation.
  - Support that is hard to reach and walled off behind an AI chat.
  - Keyword triggers that fail to fire.
  - Account suspensions.
  - G2 and Capterra scores are reported at 4.5–4.6, from solicited reviews [19].
- **Opening:** predictable flat pricing and real support for non-technical owners.
- **Confidence:** medium, because pricing comes from third parties.

### 3.3 Chatfuel

- **Pricing:** Business plan US$18/mo billed yearly, including US$20/mo in AI credits. Agency S plan US$90/mo for 3 companies [21].
- **Features:** lead qualification and scoring, in-chat selling, comment auto-replies, WhatsApp bulk campaigns, in-WhatsApp meeting booking and CRM sync [21].
- **Channels:** Facebook, Instagram, WhatsApp, TikTok and website [21].
- **Onboarding:** self-serve; claims a first automation can be published "in minutes" [21].
- **Usage limits:** referenced but not shown on the pricing page.
- **Confidence:** high on price.

### 3.4 respond.io (Malaysia)

- **Who it's for:** mid-size SMB teams with multi-channel sales and support.
- **Pricing (annual billing, per month)** [22]:
  - Starter US$79: 5 users, 1,000 monthly active contacts (MACs), AI Assist.
  - Growth US$159: adds AI Agents, workflows and broadcasts.
  - Advanced US$279.
  - Implied monthly-billing prices: US$99, 199 and 349.
  - AI credit overage US$15 per 1,000. MAC overage US$12–15 per 100.
  - WhatsApp fees are passed through at cost.
  - A 28-day onboarding programme is offered as an add-on [22].
- **Approval model:** AI Agents with handoff; AI Assist drafts replies for agents.
- **Traction:** US$7M Series A in 2022 led by Headline [23]. "10,000 companies", including the Philippine retailer Abenson (vendor claim, 2022) [23].
- **Reviews:** not examined in depth.
- **Opening:** price. Starter does not include AI Agents.

### 3.5 SleekFlow (Hong Kong)

- **Pricing (ASEAN price view)** [24]:
  - Pro US$99/mo: 3 seats, 3 channels, AI Agents.
  - Premium US$299/mo, including 60-day onboarding.
  - Dedicated onboarding US$999 one-off.
  - WhatsApp number hosting US$10/mo plus a per-message rate card.
- **Channels:** one of the few vendors to list **Viber** alongside WhatsApp, IG, Messenger, TikTok, LINE, Telegram, Gmail and Outlook [24].
- **AgentFlow:** answers from the business's own knowledge base and hands off to a human with a summary [7].
- **Funding:** US$8M Series A in 2022 (Tiger Global) [26]; US$7M Series A+ in 2024, US$15M in total (vendor release) [25].
- **Opening:** entry price and seat limits for micro-businesses.

### 3.6 WATI

- **What it is:** a WhatsApp-first inbox.
- **Pricing** [27]:
  - Base plan prices were truncated in the fetched page and not retrieved.
  - Growth/Pro/Business tiers include 1,000, 2,000 or 5,000 automation triggers and 250, 500 or 1,500 AI Co-pilot credits.
  - Astra AI Agents (web, WhatsApp, voice) are a Pro add-on, "priced separately", with no figure shown [27].
- **Reviews:**
  - G2 4.6 from 493 reviews. Its top complaint is cost at high message volume [28].
  - Capterra reviewers report slow support on verification and delivery issues [28].
  - Trustpilot has a few one-star reviews about surprise charges and bans (low volume).
- **Opening:** WhatsApp cost transparency, especially after the 2026-10-01 charges.

### 3.7 Trengo, Tidio, Intercom Fin, Zendesk, Freshdesk, Gorgias (brief)

- **Trengo:** €299/mo Boost on annual billing, with 10 users and 500 conversations a month. The AI (HelpMate) surcharge is €0.25–0.30 per conversation. "All WhatsApp service conversations are free" was still stated on the page, which may be out of date after 2026-10-01 [31]. Focused on the EU, with no Philippine evidence.
- **Tidio:**
  - Free plan with 50 lifetime Lyro conversations. Starter US$24.17/mo.
  - Lyro add-on from US$32.50/mo for 50 conversations.
  - The Premium tier offers a "guaranteed 50% Lyro resolution rate" with pay-per-resolution billing [29].
  - Messenger, IG, WhatsApp and email are on every plan [29].
  - Closest to a micro-SMB self-serve price, but web-chat-centric.
- **Intercom Fin:** the upper bound. US$0.99 per outcome plus seats at US$29, US$85 or US$132 [30]. WhatsApp is pay-as-you-go.
- **Zendesk:** the automated-resolution price is not published on the official page. Third parties estimate about US$1.50–2.00 and report a 2026-05-18 move to billing only "verified resolutions" [65].
- **Freshdesk Freddy AI Agent:** US$49 per 100 sessions, with the first 500 sessions free once per account (3P) [65].
- **Gorgias:** Shopify-centric e-commerce helpdesk, with low relevance to Philippine service SMEs. Not researched further.
- **Takeaway:** outcome-based AI pricing of about US$0.50–2.00 per resolution is the global norm. That is expensive for Philippine micro-SMEs handling hundreds of cheap inquiries.

### 3.8 Kommo (amoCRM)

- **What it is:** a messenger-first sales CRM.
- **Pricing:** US$25, 35 or 45 per user per month. Billing is per 6 months; monthly payment is not available [32].
- **AI:**
  - AI agents: up to 3 on Advanced and up to 50 on Pro.
  - AI appointment booking and voice-message handling on Pro.
  - 750–2,250 AI credits per user per month; prices for extra credits are not listed [32].
- **Channels:** Messenger and Telegram unlimited; WhatsApp and IG numbers capped by tier; 1 TikTok account. Viber is not listed [32].
- **Opening:** the 6-month prepayment and per-seat pricing are a poor fit for owner-operators.

### 3.9 GoHighLevel (HighLevel)

- **Who it's for:** marketing agencies that run client businesses, mostly in the US. This is a key channel among Philippine VAs.
- **Pricing** [33]:
  - Starter US$97/mo (3 sub-accounts).
  - Unlimited US$297/mo.
  - Agency Pro US$497/mo, which adds SaaS mode, rebilling with markup and automated sub-account creation.
  - AI Employee add-on: Growth US$50/mo or Unlimited US$97/mo per sub-account.
  - WhatsApp US$10/mo per sub-account.
  - Telecom and AI usage are billed on top.
  - Third parties report Conversation AI pay-per-use at US$0.02–0.04 per message and a pricing restructure on 2026-05-20 [36]. These are unverified.
- **Approval model:** Conversation AI with suggestive or auto-pilot modes, according to third-party descriptions. Human takeover is available in the inbox.
- **Philippine relevance:** several OnlineJobs.ph listings seek "GHL specialist" VAs. Pay ranges from US$4.50/hr gigs to US$900–1,200/mo full-time, including AI receptionist set-up [35].
- **Complaints:** Capterra and review blogs cite a steep learning curve (often 4–8 weeks), inconsistent support, and hidden usage costs for SMS, calls and AI [34].
- **Opening:**
  - GHL is US-centric. Its SMS and voice stack and A2P rules are irrelevant in the Philippines, and Viber is absent.
  - A Philippines-native "GHL-lite" for agencies, or a product agencies can resell, is plausible.
  - Equally, Philippine agencies might simply resell GHL plus Meta's agent.
- **Confidence:** high on list price, low on AI usage rates.

### 3.10 HubSpot, Zoho, Pipedrive (brief)

- **HubSpot Sales Hub Starter:**
  - US$7 or US$20 per seat per month, labels ambiguous; most likely annual vs monthly commitment [37].
  - Includes 500 HubSpot Credits [37][38].
  - Professional US$90–100 per seat plus a US$1,500 onboarding fee [37].
  - Third parties say Breeze Customer Agent needs Professional or higher, and report about US$0.50 per resolution (50 credits) [38]. Medium confidence.
- **Zoho CRM:**
  - Free for 3 users. Third parties report Standard US$14, Professional US$23, Enterprise US$40 and Ultimate US$52 per user per month on annual billing [39].
  - Zia AI is in Professional and above [39].
  - The official page did not render prices.
- **Pipedrive:** third parties report Lite US$14, Growth US$39, Premium US$49 and Ultimate US$79 per seat per month on annual billing, with conflicts between sources [40]. The official page blocked the fetch.
- **Assessment:** these CRMs are the systems of record a lead-follow-up agent might write into. They are not Messenger-native for Philippine micro-SMEs.

### 3.11 Booking and reminders

- **Calendly:** Free; Standard US$10 and Teams US$16 per seat per month, billed yearly. Reminders on paid plans. "Callie" email scheduling agent in beta, English only [41]. Link-based, not chat-native.
- **Fresha:**
  - Publishes **peso pricing**: ₱900/mo solo, ₱600 per team member per month [42].
  - Reminders: 20 free SMS/WhatsApp per member, then ₱15 per SMS or ₱5.60–15 per WhatsApp message.
  - FB/IG bookings are free. New clients from the marketplace incur a one-off fee; the amount is not shown [42].
  - The strongest international booking player with visible Philippine pricing.
- **SimplyBook.me:** Free (50 bookings) up to €49.9/mo. SMS, WhatsApp and **AI voice booking** credits at US$8 per 100. FB/IG booking. "Ask Simply" admin AI assistant [43].
- **Setmore:** Free (4 users); Pro about US$5 per user per month on annual billing, though the page is inconsistent. FB/IG "Book Now" buttons; no AI [44].
- **Square Appointments:** not available in the Philippines. A Square community moderator says so, and Square's card processing covers only US, CA, AU, JP, UK, IE, FR and ES (forum-sourced) [45].
- **Booksy:** no Philippine evidence found; not researched further.
- **SeriousMD / NowServing (Philippines):**
  - Clinic EMR plus patient booking.
  - Basic is free with a 5% + ₱75 service fee on bookings. Pro is ₱1,950/mo or ₱22,788/yr, with 100 SMS credits.
  - No AI features listed [46].
  - Dominant mindshare among Philippine doctors (positioning claim, not measured).
- **"Booky" (Philippines):** a deals and food discovery app, not an appointment system. Not a competitor.
- **Opening:**
  - None of these booking tools takes the booking *inside* the Messenger/Viber conversation with AI. They push a link.
  - Meta's agent books in chat but has no live calendar sync [1].
  - The opening is a **bridge**: AI in chat → real calendar or clinic system → reminders via Viber or SMS.

### 3.12 AI receptionist / voice (brief)

- Goodcall from about US$79/mo per 100 unique callers. Rosie from about US$49/mo. Smith.ai from about US$95–97.50 for 30–50 calls, with human staff. Synthflow per-minute, unverified [63]. All figures are third-party roundups.
- All are US-centric phone products.
- **PH relevance:** low for inbound SME inquiries, which arrive mainly as chat. That claim is the market agent's to verify.

### 3.13 Document AI (brief)

- **Nanonets:** US$50 free credits, then US$100/mo for 100 credits; US$0.02–0.30 per workflow block. Its own estimate for an invoice workflow is "under $2" [55].
- **Docsumo:** 14-day trial of 1,000 pages; Business and Enterprise by custom quote; **setup fees extra** [56].
- **Rossum:** Starter from **US$18,000/yr**, with a 1-year minimum [57].
- **Klippa and Google Document AI:** pricing pages were not retrieved (404 or JS-rendered) [58]. Google publishes per-1,000-page processor pricing; I could not verify current figures.
- **Opening:**
  - Every vendor is built for AP and finance teams processing documents in volume.
  - None is aimed at the Philippine SME intake case: IDs, receipts, BIR forms and order slips arriving through Messenger photos.
  - Demand for that is unverified.

### 3.14 General automation (brief)

- **Zapier:** Free (100 tasks); Pro from US$19.99/mo annual. Agents add-on with 400 free activities. Chatbots add-on (2 free) [59].
- **Make:** Free (1,000 credits); paid prices not rendered; has AI agents [60].
- **n8n:** €20/mo Starter, €50 Pro (annual); self-hosted community edition [61].
- **Lindy:** US$49.99/mo for 5,000 credits; **"approvals built in"** for actions with outside impact such as sending email [62].
- **Assessment:** these are builder tools. They mostly compete through agencies and freelancers who assemble Messenger, Sheets and LLM bots for Philippine SMEs.
- **Note:** Lindy's approval-first design shows that draft-and-approve is now an accepted pattern in the market.

### 3.15 Proposal and quote tools (brief)

- PandaDoc is reported at about US$19 per seat (Starter/Essentials) and about US$49 (Business) on annual billing, with document caps and paid add-ons (3P) [64]. Proposify was not retrieved.
- These tools send and track documents. They do not run chat-native follow-ups ("Sir, any update on the quote?") on Messenger or Viber.
- I found no SME product focused on quote follow-up through messaging. Confidence: low–medium, because this is absence of evidence.

### 3.16 Philippine local vendors and substitutes

- **Bots at Work** (done-for-you agency):
  - ₱40–50k build plus a ₱25–45k/mo retainer, with a 2–4 week deployment.
  - Channels: Messenger, IG, Viber, WhatsApp, web, TikTok and SMS. Native Taglish.
  - Clients claimed: AC Motors, Kia PH, Honda PH, Aboitiz Power [47][48].
  - Note: the comparison is published by Bots at Work itself.
- **ChatGenie:**
  - Founded around 2018–2020 (sources conflict). Messenger social-commerce bot with shipping integrations (Grab, Pandago, Ninja Van).
  - Pricing not published.
  - About US$1.12M raised in total (CB Insights); Accelerating Asia 2021; seed-II round of US$0.12M in 2023 [49].
  - 2,500+ merchants as of 2021 (press-reported vendor claim) [49].
- **Other local players:**
  - LetsChat.PH and ModulON AI (small builders) [48].
  - Senti AI, Stratpoint and Proto (enterprise) [48].
- **Pancake / Botcake** (Vietnam; Philippine partner site):
  - FB seller inbox. Botcake Pro from US$30; auto-replies within 24h free up to 500k/mo.
  - AI via the user's own OpenAI key [50][51].
- **Viber:**
  - Business messages and chatbots are sold through resellers. Indicative Philippine rates: about US$0.0019 per transactional message and about US$0.0028 per chatbot session, plus a US$500/mo fee (reseller, undated) [52].
  - Viber launched its SMB toolkit first in the Philippines in 2023 [52].
  - Few SMB AI tools support Viber; SleekFlow and Bots at Work are exceptions.
- **Globe Business:** "AI solutions" bundled with connectivity from about ₱1,200/mo (2026-08-21). No chat agent product named [53].
- **Smart:** no SME chatbot offering found.
- **Manual substitutes:**
  - A chat-support VA: OnlineJobs.ph listing at US$1,150/mo [54]; Indeed PH average VA pay ₱25,269/mo (March 2026) [54]; third-party guides cite ₱25–35k/mo.
  - The owner replying personally: free, but slow and limited to waking hours. This is the most common "competitor".

---

## 4. Meta platform rules shaping this market

| Rule | What it says (as of 2026-10-18) | Impact | Source / confidence |
|---|---|---|---|
| Messenger 24-hour standard window | Businesses can message freely within 24h of the user's last message. A Click-to-Messenger-ads thread-control window of 24h from the last user message is documented. | Bots must reply fast; follow-up after 24h needs another mechanism. | [16] high |
| Messenger message tags | CONFIRMED_EVENT_UPDATE, ACCOUNT_UPDATE and POST_PURCHASE_UPDATE fail with error 100 from **2026-04-27**. Migrate to Utility Templates or the Marketing Messages API. | Ends the old workaround for booking confirmations and reminders outside 24h. | [16] high |
| HUMAN_AGENT tag | Not on the deprecation list; 7-day window for **human** replies. | AI cannot use it. Whether an AI draft sent by a human qualifies is **unresolved**: needs a policy reading. | [17] medium |
| Recurring Notifications (Messenger) | Disabled from 2026-02-10 except AU, EU, JP, KR and UK, per Meta docs as relayed by ManyChat. | Opt-in follow-ups are gone in the Philippines. | [17] medium |
| Utility Messages availability | A user report says utility messages are available in the US, VN and **PH**; ManyChat has not confirmed this. | Possible paid route for reminders in the Philippines. | [17] low |
| WhatsApp per-message pricing | Since 2025-07-01, charges are per delivered template (marketing, utility, authentication) by country. | — | [9] high |
| **WhatsApp 2026-10-01 change** | Non-template service messages **not powered by Meta Business Agent** are charged at utility/authentication rates, with no volume tiers. In-window utility templates are also charged. The 72h free entry point still covers delivery. | Every third-party bot reply now has a cost. | [8] high |
| PH rates | Philippines is in "Rest of APAC". Reported rates: service about US$0.0113 after 1,000 free per number per month (partner-reported; not on Meta's page); marketing US$0.0732 → US$0.0842. | Budget about ₱0.65 per reply above the free tier (at roughly ₱57/US$; exchange rate not verified). | [12] medium |
| Meta Business Agent on API | US$2.00 per 1M tokens from 2026-08-01, about US$0.04–0.05 per message, charged even inside the 72h free entry point window. | Meta's agent costs **more per message** on the API than a third-party reply (about US$0.0113 plus LLM cost). In the free app tier, it costs nothing up to the allowance. | [8][11] high |
| General-purpose AI ban | From 2026-01-15, AI providers whose general-purpose assistant is the "primary functionality" are prohibited, at Meta's "sole discretion". Business customer-service bots are allowed. | A business-specific assistant is fine. Avoid a "general AI assistant for consumers" positioning on WhatsApp. | [13] high |
| EU/Italy carve-out | Italy from 2026-02-16: fee of about US$0.0691 per AI response. EU: 12-month allowance at €0.049–0.1323 per non-template message. | Not relevant to the Philippines; shows Meta monetising AI traffic. | [14] medium (search-level reporting) |

---

## 5. White-space analysis

Each gap below lists what the evidence shows, my confidence, and what would disprove it.

1. **Human-approved AI follow-up across channels for service SMEs (directions a and c). Confidence: medium.**
   - **Why there is room:**
     - Meta's agent is inbound-only and autonomous [5].
     - Messenger outbound tags are gone [16].
     - Inbox vendors (respond.io, SleekFlow) offer follow-up workflows, but at US$79–299/mo and aimed at teams [22][24].
     - Lindy shows approval-first agents are viable, but it is not chat-commerce-native [62].
   - **What would fill it:** an owner-facing "follow-up queue" where AI drafts nudges for stale leads and quotes, the owner approves with one tap, and the message goes through whichever channel is still open or allowed (Messenger within 24h, Viber, SMS, email).
   - **What would disprove it:** Meta shipping outbound follow-up in Business Agent, or the Philippine market agent finding that SMEs don't lose leads to slow follow-up.

2. **Chat-to-calendar booking bridge for Philippine services (direction b). Confidence: medium.**
   - **Why there is room:**
     - Meta books in chat, but calendar management is still "planned" [1].
     - Fresha, SimplyBook.me and SeriousMD take bookings via links or apps, not via AI conversations [42][43][46].
     - Reminder channels in the Philippines are costly or restricted: Fresha charges ₱15 per SMS [42]; Messenger reminder tags are deprecated [16].
   - **Risk:** Meta could close this within months, since calendar management is on its stated roadmap.
   - **Mitigation:** integrate with the systems Philippine clinics and salons already use (SeriousMD, Google Calendar, Fresha), so the product does not have to be a calendar itself.

3. **Viber and multi-channel coverage beyond Meta. Confidence: medium.**
   - **Why there is room:** Meta's agent is Meta-only [1]. Viber support is rare among SMB AI tools; SleekFlow and Bots at Work have it [24][47].
   - **What it depends on:** the market agent's evidence on how much Philippine SMEs use Viber. Viber's chatbot fees (a reported US$500/mo minimum through resellers) may make it uneconomic for micro-SMEs [52].

4. **Done-with-you set-up at "VA-replacement" prices. Confidence: medium.**
   - **The price gap:** agencies charge ₱40–50k up front plus ₱25–45k/mo [47], which is roughly the cost of a VA [54]. Self-serve SaaS starts at US$15–99/mo but needs configuration skills.
   - **What would fill it:** a productised offer between those two, for example a fixed-fee setup plus a ₱2–5k/mo subscription with Taglish and Filipino tuned in. Pricing is my hypothesis; it is untested.

5. **Accuracy guarantees on prices and quotes. Confidence: medium-low.**
   - **Why there is room:** Meta warns of inaccuracy and the business stays liable [5][66]. Global vendors sell "resolution" guarantees (Tidio's guaranteed 50% Lyro resolution rate [29]), not "never misquote" guarantees.
   - **What would fill it:** answers locked to structured data, such as price lists and stock levels, with draft-approve for any reply that commits to a price.
   - **Evidence of demand:** no Philippine-specific evidence found; this needs interviews.

6. **SME document intake, such as Messenger photos of IDs, receipts and forms (direction d). Confidence: low.**
   - **Supply gap:** clear, since all vendors are mid-market AP tools [55][56][57].
   - **Demand:** unverified. Should be treated as an add-on to (a) or (b), not a lead product, until demand is shown.

7. **Areas that look crowded or commoditised (avoid). Confidence: high.**
   - A generic FAQ bot on the FB Page inbox: Meta offers it free, and ManyChat, Chatfuel and Botcake offer it cheaply.
   - Omnichannel team inboxes: respond.io, SleekFlow and others.
   - US-style AI phone receptionists.
   - Full CRM: HubSpot, Zoho and Kommo.

---

## 6. Unresolved questions

1. The Philippine free-response allowance and peso pricing for Meta Business Agent and Meta One. Is Business Agent live on Instagram and web for Philippine accounts?
2. How well Meta's agent handles Taglish (no evidence found).
3. Whether a human approving and sending an AI-drafted message within the HUMAN_AGENT 7-day window complies with Messenger policy.
4. Whether Messenger Utility Messages are generally available in the Philippines, and at what price.
5. Official confirmation of the Philippine WhatsApp service rate and the 1,000-message free allowance on Meta's rate-card CSV.
6. ManyChat's current official tier prices, and whether AI is included or an add-on.
7. Base WATI plan prices and the Astra add-on price.
8. Viber Business official Philippine chatbot pricing; reseller figures conflict.
9. Independent user reviews of Meta Business Agent; none were found.

---

## 7. Sources

All accessed 2026-10-18.

1. Meta Newsroom. "Be There for Every Customer With Meta Business Agent." 2026-06-03. https://about.fb.com/news/2026/06/meta-business-agent/ [Primary, high]
2. TechCrunch. "Meta's AI agent for WhatsApp Business is now available globally." 2026-06-03. https://techcrunch.com/2026/06/03/metas-ai-agent-for-whatsapp-business-is-now-available-globally/ [Press, high]
3. Marketech APAC (L. M. Darang). "Meta brings Business AI to the Philippines..." 2025-10-20. https://marketech-apac.com/meta-brings-business-ai-to-the-philippines-to-enhance-customer-engagement-for-local-businesses/ [Press, high]
4. WhatsApp Business (Meta). "Meta Business Agent on WhatsApp." Undated. https://whatsappbusiness.com/products/business-app-ai-agent/ [Primary, high]
5. Metricool. "Meta Business Agent: How Meta's AI Agent Works." 2026. https://metricool.com/meta-business-agent/ [3P, medium]
6. TechCrunch. "Meta expands subscription push with new AI-focused plans." 2026-09-15. https://techcrunch.com/2026/09/15/meta-expands-subscription-push-with-new-ai-focused-plans/ [Press, high]
7. SleekFlow. "Meta One explained: what it means for B2C business." 2026. https://sleekflow.io/blog/meta-one-for-business [3P/vendor, medium]
8. Meta for Developers. "Upcoming pricing updates for Meta Business Agent, service and utility messages" (non-template messages). https://developers.facebook.com/documentation/business-messaging/whatsapp/pricing/non-template-messages [Primary, high]
9. Meta for Developers. "Pricing on the WhatsApp Business Platform." https://developers.facebook.com/docs/whatsapp/pricing [Primary, high]
10. Meta for Developers. "Pricing updates on the WhatsApp Business Platform." https://developers.facebook.com/docs/whatsapp/pricing/updates-to-pricing/ [Primary, high]
11. Business Today. "WhatsApp introduces token-based pricing for AI Agents from August 1." 2026-07-03. https://www.businesstoday.in/technology/artificial-intelligence/story/whatsapp-introduces-token-based-pricing-for-ai-agents-from-august-1-540699-2026-07-03 [Press, high]
12. Voltade. "WhatsApp pricing from 1 October 2026, by market." 2026. https://voltade.com/blog/whatsapp-replies-stop-being-free-1-october-2026 [3P/partner, medium]
13. TechCrunch. "WhatsApp changes its terms to bar general-purpose chatbots from its platform." 2025-10-18 (updated 2026-01-15). https://techcrunch.com/2025/10/18/whatssapp-changes-its-terms-to-bar-general-purpose-chatbots-from-its-platform [Press, high]
14. TechCrunch. "Meta will allow rival AI chatbots on WhatsApp in Europe, but for a fee." 2026-03-05. https://techcrunch.com/2026/03/05/meta-will-allow-rival-ai-chatbots-on-whatsapp-in-europe-but-for-a-fee/ ; also ContentGrip, "Meta starts charging for WhatsApp AI bots in Italy." https://www.contentgrip.com/whatsapp-ai-chatbot-pricing-italy/ [Press, medium: seen via search summaries]
15. (Reserved.)
16. Meta for Developers. "Changelog — Messenger Platform." Entries 2025-07-22 to 2026-03-27. https://developers.facebook.com/docs/messenger-platform/changelog/ [Primary, high]
17. ManyChat Community. "Meta's deprecation of the Message Tags feature on Messenger." 2026-02/03. https://community.manychat.com/product-updates/meta-s-deprecation-of-the-message-tags-feature-on-messenger-9010 [Vendor/community, medium]
18. Chatarmin. "ManyChat pricing" (updated 2026-09-25). https://chatarmin.com/en/blog/manychat-pricing [3P, medium]
19. eesel.ai. "ManyChat pricing 2026: plans, active contacts, and hidden costs." https://www.eesel.ai/blog/manychat-pricing ; Featurebase, "Manychat Review 2026." https://www.featurebase.app/blog/manychat-review [3P, medium: via search summaries]
20. Trustpilot. "ManyChat reviews." https://www.trustpilot.com/review/manychat.com [Review, medium]
21. Chatfuel. "Pricing." https://chatfuel.com/pricing [Primary, high]
22. respond.io. "Pricing." https://respond.io/pricing [Primary, high]
23. respond.io. "respond.io raises $7 million in Series A funding." 2022. https://respond.io/blog/respond-io-raises-7-million-in-series-a-funding ; Techloy coverage. https://www.techloy.com/malaysian-business-messaging-app-respond-io-bags-7-million-in-series-a/ [Vendor + Press, high]
24. SleekFlow. "Pricing." https://sleekflow.io/pricing [Primary, high]
25. SleekFlow. "SleekFlow Secures US$7M With Atinum Investment." 2024-08. https://sleekflow.io/news/sleekflow-funding-aug-2024-en [Vendor, medium]
26. Gunderson Dettmer. "Tiger Global Leads $8M Series A Financing of SleekFlow." 2022. https://www.gunder.com/en/news-insights/client-news/tiger-global-leads-8m-series-a-financing-of-sleekflow [Law firm release, high]
27. WATI. "Pricing." https://www.wati.io/pricing/ [Primary, medium: truncated]
28. G2. "Wati reviews." https://g2.com/products/wati/reviews ; Capterra. "Wati reviews." https://www.capterra.com/p/204314/WATI/reviews/ ; Trustpilot. https://de.trustpilot.com/review/wati.io [Review, medium: via search summary]
29. Tidio. "Pricing." https://www.tidio.com/pricing/ [Primary, high]
30. Intercom. "Pricing." https://www.intercom.com/pricing [Primary, high]
31. Trengo. "Pricing." https://www.trengo.com/pricing [Primary, high]
32. Kommo. "Pricing." https://www.kommo.com/pricing/ [Primary, high]
33. HighLevel. "Pricing." https://www.gohighlevel.com/pricing [Primary, high]
34. Capterra. "HighLevel reviews." https://www.capterra.com/p/177156/HighLevel/reviews/ ; Ciela, "Is GoHighLevel worth it 2026 (Reddit)." 2026-07-02. https://ciela.ai/blogs/is-gohighlevel-worth-it-2026-reddit [Review + 3P, medium]
35. OnlineJobs.ph job listings: "GoHighLevel / GHL Specialist Virtual Assistant" https://www.onlinejobs.ph/jobseekers/job/1591863 ; "GoHighLevel Specialist" https://www.onlinejobs.ph/jobseekers/job/1494944 ; "Go High Level onboarding support specialist" https://www.onlinejobs.ph/jobseekers/job/1480441 [Primary listings, medium]
36. Apexure. "GoHighLevel pricing." https://www.apexure.com/blog/gohighlevel-pricing ; CloseBot comparison. https://closebot.com/?p=520 ; NetPartners. https://netpartners.marketing/gohighlevel-voice-ai-conversation-ai-pricing-2026/ [3P, low]
37. HubSpot. "Sales Hub pricing." https://www.hubspot.com/pricing/sales [Primary, medium: labels ambiguous]
38. HubSpot Knowledge Base. "Understand HubSpot Credits and billing." https://knowledge.hubspot.com/account-management/understand-hubspot-credits-and-billing ; Impulse Creative on Customer Agent credits. https://impulsecreative.com/blog/unlocking-hubspots-customer-agent-what-you-need-to-know-about-the-rollout-new-breeze-credits [Primary + 3P, medium]
39. Zoho. "Zoho CRM pricing." https://www.zoho.com/crm/zohocrm-pricing.html ; Automation Atlas, "Zoho CRM pricing explained 2026." https://automationatlas.io/answers/zoho-crm-pricing-explained-2026/ [Primary (partial) + 3P, medium]
40. Hackceleration. "How much does Pipedrive cost?" (checked 2026-06). https://www.hackceleration.com/labs/pipedrive-pricing [3P, medium]
41. Calendly. "Pricing." https://calendly.com/pricing [Primary, high]
42. Fresha. "Pricing" (PHP view). https://www.fresha.com/pricing [Primary, high]
43. SimplyBook.me. "Pricing." https://simplybook.me/en/pricing [Primary, high]
44. Setmore. "Pricing." https://www.setmore.com/pricing [Primary, medium: inconsistent page]
45. Square. "Appointments pricing." https://squareup.com/us/en/appointments/pricing ; Square Seller Community, "Does Square work in the Philippines?" https://www.sellercommunity.com/t5/Using-Square/Does-square-work-in-the-Philippines/m-p/642359 [Primary + Forum, medium]
46. SeriousMD. "Pricing." https://seriousmd.com/pricing [Primary, high]
47. Bots at Work. "ChatGenie vs Bots at Work for SMEs in the Philippines." 2026-03-18. https://botsatwork.ph/2026/03/18/chatgenie-vs-bots-at-work-smes/ [Vendor, medium: self-interested]
48. Bots at Work. "Best AI Chatbot Company Philippines: 2026 Guide." 2025-10-07. https://botsatwork.ph/2025/10/07/top-ai-chatbot-company-philippines-2026/ ; ModulON AI blog. https://modulonai.com/blog/ai-chatbots-for-philippine-smes-affordable-secure-with-modulon-ai-in-2026 ; LetsChat.PH. https://www.letschat.ph/ [Vendor, low–medium]
49. CB Insights. "ChatGenie financials." https://www.cbinsights.com/company/chatgenie/financials ; Marketech APAC. "PH commerce platform developer ChatGenie secures fresh funds." https://marketech-apac.com/ph-commerce-platform-developer-chatgenie-secures-fresh-funds-to-bolster-operations/ ; BusinessWorld. https://www.bworldonline.com/?p=417808 [Data provider + Press, medium]
50. Botcake. "Pricing." https://botcake.io/pricing [Primary, medium: via search summary]
51. Pancake Philippines partner page. https://partners.pancake.ph/Meta [Vendor, low]
52. kLink. "Viber for Business." https://www.klink.cloud/viber-for-business ; Alibaba Cloud Chat App docs (Viber). https://www.alibabacloud.com/help/en/chatapp/product-overview/viber ; NewsBytes PH. "PH is first country to roll out Viber's new toolkit for SMEs." 2023-10-11. https://newsbytes.ph/2023/10/11/ph-is-first-country-to-roll-out-vibers-new-toolkit-for-smes/ [Reseller + Press, low–medium]
53. SAMSearch (news digest). "Globe Business champions affordable AI solutions for Philippine MSMEs." 2026-08-21. https://samsearch.co/government-contracting-news/globe-business-champions-affordable-ai-solutions-for-philippine-msmes-177267 [Press digest, low]
54. OnlineJobs.ph. "Chat Support & Sales VA" (updated 2026-05-12). https://www.onlinejobs.ph/jobseekers/job/1644324 ; Indeed PH. "Virtual Assistant salaries." https://ph.indeed.com/cmp/Helpware/salaries/Virtual-Assistant ; Bossjob. "VA salary Philippines 2026." https://bossjob.ph/blog/?p=8893 [Listings + 3P, medium]
55. Nanonets. "Pricing." https://nanonets.com/pricing [Primary, high]
56. Docsumo. "Pricing." https://www.docsumo.com/pricing [Primary, high]
57. Rossum. "Pricing." https://rossum.ai/pricing/ [Primary, high]
58. Google Cloud. "Document AI pricing." https://cloud.google.com/document-ai/pricing [Primary: not rendered; figures not used]
59. Zapier. "Pricing." https://zapier.com/pricing [Primary, high]
60. Make. "Pricing." https://www.make.com/en/pricing [Primary, medium: prices not rendered]
61. n8n. "Pricing." https://n8n.io/pricing/ [Primary, high]
62. Lindy. "Pricing." https://www.lindy.ai/pricing [Primary, high]
63. Dupple. "The 8 Best AI Receptionists in 2026." https://dupple.com/learn/best-ai-receptionists ; Ringly. "HeyRosie alternatives." https://www.ringly.io/blog/heyrosie-alternatives [3P, low]
64. Hackceleration. "How much does PandaDoc cost?" https://hackceleration.com/labs/pandadoc-pricing [3P, low–medium]
65. eesel.ai. "Zendesk AI pricing in 2026." https://eesel.ai/blog/zendesk-ai-pricing/ ; eesel.ai. "Freshdesk Freddy AI pricing explained." https://www.eesel.ai/blog/freshdesk-freddy-ai-pricing [3P, low–medium]
66. Digital Dawn. "Meta Business AI on WhatsApp India: Setup, Cost & Verdict." https://www.digitaldawn.in/meta-business-ai-whatsapp-india/ [3P, low–medium]
67. Club Unbrick. "Meta Business Agent vs. Your Own WhatsApp Automation." https://clubunbrick.co/blog/meta-business-agent-vs-custom-whatsapp-automation/ [3P, low]
