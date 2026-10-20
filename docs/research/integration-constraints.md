# Integration, Platform-Policy and LLM-Cost Constraints (PH SME AI Agent, Rust)

Accessed: 2026-10-18 (all sources). Scope: whether a solo developer or small team can build and run an AI agent product for Philippine small businesses in Rust. Candidate products: inquiry handling and lead follow-up on messaging channels, booking and reminders, quote follow-up, document intake, and a personal email/admin assistant.

Evidence labels: **[OD]** official docs, **[PP]** official pricing page, **[DR]** developer report or blog, **[VC]** vendor or reseller claim, **[N]** news. Confidence is **H/M/L**. Numbers in brackets such as [11] point to the Sources list at the end. Where something could not be verified, the text says "not found".

> Research limits: the shared web-search budget ran out partway through. After that, findings come only from direct fetches of official pages. A few official pages returned HTTP 403 to the fetcher (Xendit PH pricing, Viber help center, TAC Security, NTC news). For those, the figures come from search-result excerpts of the official page and are marked M.

---

## 1. Key conclusions

**Cheap and fast for a new solo SaaS: no platform gatekeeper**
- **Web chat widget + LLM + Postgres.** No approvals needed. LLM cost per SME is about **$1–$15/month** on the cheapest capable tier (Claude Haiku 5.5, gpt-6-luna, DeepSeek-V4-Flash) [1][6][9].
- **Document and email intake through your own inbound address**, where the SME forwards mail or sets up auto-forwarding. Postmark Pro is **$16.50/mo** and includes inbound processing [46]. SES inbound is **$0.10 per 1,000 emails** [47]. No Google or Microsoft review is involved.
- **SMS reminders via Semaphore.** **₱0.56 per credit**, and a sender name is approved in **up to 5 business days** [55][56].
- **Payment links via PayMongo**, with no monthly fee [48]. PayMongo needs merchant onboarding (KYB) of your registered business. Stripe does **not** support Philippine accounts [53].

**Gated but feasible in weeks: Meta channels**
- **Messenger and Instagram DM** are free per message. Serving other businesses' Pages needs Business Verification, App Review for Advanced Access, and Tech Provider "Access Verification". Meta's only official timeline is the last step: a decision in "approximately 5 days" [29]. App Review has no official SLA. One vendor estimate puts Instagram Advanced Access at 2–4 weeks [32] (L).
- **Breaking change:** the ACCOUNT_UPDATE, CONFIRMED_EVENT_UPDATE and POST_PURCHASE_UPDATE tags fail with **error 100 after April 27, 2026** [23]. Any reminder or follow-up outside the 24-hour window now needs **Utility Messages templates** (a separate `page_utility_messaging` permission) [27] or paid marketing messages.
- **WhatsApp Cloud API as a Tech Provider** has the same verification chain. Until verified, onboarding is capped at **10 new business customers per rolling 7 days**; after verification it is 200 [15]. **Service replies inside the 24h window became chargeable on October 1, 2026** [12]. For a PH SME with 1,000 conversations/month, WhatsApp fees can now be larger than the LLM bill (§3.4).

**Slow, gated or expensive: avoid for the MVP**
- **Gmail API with read access.** `gmail.readonly`, `gmail.modify` and `gmail.compose` are **restricted** scopes [35]. Storing or transmitting that data on servers triggers an **annual CASA security assessment** [36]. Verification "can potentially take several weeks" [36]. A Tier 2 assessment costs about **$540–$720/yr** at TAC Security [40] (M).
- **Microsoft Graph mail.** Multitenant apps need publisher verification through a verified Microsoft AI Cloud Partner Program account [44]. Mail.ReadWrite is blocked from end-user consent under Microsoft's recommended policy [45] (M), so expect admin consent.
- **Viber bots.** Since **5 Feb 2024**, bots "can only be created on commercial terms" [33]. The maintenance fee is **EUR 100 per bot per month** [34] (M), which is uneconomic per SME.
- **A personal email/admin assistant on WhatsApp is prohibited outright.** Meta bars AI providers where AI is the "primary (rather than incidental or ancillary) functionality" [14].

---

## 2. Meta Messenger Platform and Instagram Messaging

### Requirements and permissions
- **Permissions requested via Facebook Login for Business:** `pages_show_list`, `pages_manage_metadata`, `pages_messaging`, `pages_read_engagement`, `business_management`. Instagram adds `instagram_basic` and `instagram_manage_messages` [21] (OD, H). `business_management` is a dependency to call out in App Review [21][28].
- **Access levels.** Standard Access is limited to people with a Role on your app or Page. **Advanced Access "requires App Review"**. "App Review isn't required if you only send and receive messages for your own Facebook Page" [21] (OD, H). You can therefore build and demo on your own Page with no review.
- **Business Verification** is needed "if people without a Role on the app use your app" [21]. Apps that let other businesses access their data "must be connected to a Business that has completed Business Verification" [30] (OD, H). Accepted documents are not listed on the developer page. A registered business (DTI/SEC) is the practical prerequisite (L; Meta's help article could not be fetched).
- **Access Verification (Tech Provider).** This applies to any business whose app is used by other businesses and requests permissions such as `business_management`. Such a business "must be verified as a Tech Provider before other businesses can use the app"; "a decision will be made within approximately 5 days"; Business Verification must be completed first [29] (OD, H).
- **App Review submission rules.** The webhook must return `200 OK` "within 20 seconds"; the Page must be published; and Meta asks for an average response time "under one minute" [26] (OD, H). For apps serving other businesses, the screencast must show the Facebook Login for Business flow, granting permissions, and setting up the automated experience [28] (OD, H).
- **Timelines.** Access Verification takes about 5 days [29] (OD, H). App Review has no official SLA (not found). Instagram Advanced Access is "2–4 weeks" per a Phyllo blog dated Sept 2026 [32] (VC, L). **Planning assumption: 2–6 weeks end-to-end**, assuming the business documents are ready.

### Messaging window, tags and follow-ups
- **24-hour standard window.** It opens on a user action such as a message, CTA click or reaction, and promotional content is allowed inside it [22] (OD, H).
- **HUMAN_AGENT tag.** Lets a human respond "within 7 days". Automated messages are a disallowed use. The Human Agent feature "is not available in standard access or development mode", so it needs App Review [22][28][31] (OD + DR, H).
- **Deprecated tags.** The changelog entry of Mar 27, 2026 says requests using CONFIRMED_EVENT_UPDATE, ACCOUNT_UPDATE and POST_PURCHASE_UPDATE "will receive error code 100… after April 27, 2026". It points to **Utility Templates or the Marketing Messages API** as replacements [23] (OD, H).
- **Utility Messages.** Covers "order or account status updates, and appointment or event reminders". Templates are approved "within seconds of creation", must not contain marketing, and need the `page_utility_messaging` permission [27] (OD, H). Pricing is not stated in the doc (not found).
- **What this means for products.** Booking reminders fit Utility templates. A "just checking in on your quote" nudge after 24h is marketing in nature, so it needs Marketing Messages, which are paid and require opt-in, or a human-agent reply within 7 days. Plan lead follow-up so it happens **inside 24h**, or ask for opt-in to SMS or email.

### Handover between bot and human inbox
- **Handover Protocol is retired.** "Meta no longer supports Handover Protocol for Messenger" [24]. The Instagram retirement was Oct 23, 2025 [23]. The replacement is **Conversation Routing**, "mostly backward compatible" [24] (OD, H).
- **How control moves.** Endpoints: `pass_thread_control`, `take_thread_control`, `release_thread_control`, `request_thread_control` (default app only), and `extend_thread_control` (up to 7 days). If a human replies from the Meta Business Suite Inbox, "the Inbox takes control" [24] (OD, H).
- **Setup needs.** A default app assigned in Page settings; webhooks subscribed to `messages`, `messaging_handover`, `standby` and related fields; and, per the doc, a Meta Ad Account [24] (OD, H).
- **Instagram Inbox app ID** for routing: `1217981644879628` [24].

### Rate limits and webhooks
- **Messenger.** Calls per 24h = **200 × engaged users**. Send API: **300 calls/s per Page** for text, 10/s for audio and video. Conversations API: **2 calls/s** [25] (OD, H).
- **Instagram.** Send API 300/s per account. No 24h limit is listed [25] (OD, H).
- These limits are far above SME volumes and are not a constraint.

### Costs and Rust support
- **Cost.** No per-message fee was found for standard Messenger or Instagram replies (M; only Sponsored and Marketing messages are described as paid [22]).
- **Rust.** No official SDK. Community crates are stale or tiny (`fb_messenger` last updated 2021; `russenger` 0.3.6-rc, Apr 2025) [71]. **Use plain HTTPS (reqwest) plus an axum webhook handler with HMAC signature verification.** This is low risk: the API surface is small.

---

## 3. WhatsApp Business Platform (Cloud API)

### 3.1 Onboarding as a Tech Provider (Embedded Signup)
- **Steps:** Business Verification, then App Review (Advanced Access for `whatsapp_business_messaging` and `whatsapp_business_management`, with two demo videos), then Access Verification [16][15] (OD, H). "Your business must be verified before you can start the app review process" [16].
- **Onboarding caps.** "By default, you can onboard up to 10 new business customers in a rolling 7-day window." After Business Verification, App Review and Access Verification, the limit is "200 new business customers in a rolling 7-day window" [15] (OD, H). A pilot of 10/week is fine.
- **Billing.** Onboarded customers "must add a payment method to their WhatsApp Business account" [15] (OD, H). Meta says businesses without a payment method by **Sept 30, 2026** have service messages stopped from Oct 1 [12][18] (OD excerpt + VC, M).
- **Version.** "Embedded Signup v2 is deprecated on October 15, 2026"; build on **v4** [15] (OD, H).
- **Alternative.** Going through a Solution Partner/BSP (e.g. 360dialog, Twilio) adds a markup but can shortcut some steps [16] (OD, M).

### 3.2 Pricing (current as of Oct 2026)
- **Per-message since July 1, 2025.** "Effective July 1, 2025, Meta charges on a per-message basis" [11] (OD, H). Template categories are Marketing, Utility and Authentication. Rates change only at the start of a quarter [11].
- **NEW: service messages are chargeable from Oct 1, 2026.**
  - Meta says: "Effective October 1, 2026, Meta will charge for service messages, which have not been charged since November 2024", and "Meta will charge for utility messages sent in response to users within an open 24-hour customer service window." Also: "By market, rates for service messages are the same as the rates for utility and authentication messages" [12] (OD, H).
  - **Free tier:** 1,000 free service messages per business phone number per month, with charges from the 1,001st [18][12] (VC + search excerpt of Meta page, **M**; a direct fetch of Meta's page did not show this line).
  - Note: the marketing page whatsappbusiness.com still says service messages are free [19]. It is stale.
- **Free Entry Point.** Replies to users who arrive from a Click-to-WhatsApp ad or Page CTA open a free 72h window [11] (OD, H).
- **Philippines rates.** PH (+63) is billed in the **"Rest of Asia Pacific"** market [11] (OD, H). Rate card effective Oct 1, 2026, per Gallabox's reproduction [17] (VC, **M**; Meta's CSV not fetched directly):

| Category (Rest of APAC, USD per delivered msg) | Rate |
|---|---|
| Marketing | $0.0968 (was $0.0842 before Oct 1) |
| Utility / Authentication | $0.0130 |
| Service (after free tier) | $0.0130 |

- **Meta Business Agent (Meta's own AI agent).** Priced at "One global rate of $2.00 USD per 1M tokens", about 4–5¢ per message [12] (OD, H). **Strategic note:** Meta now sells a first-party competitor inside the same channel.

### 3.3 AI-provider policy (exact wording)
Meta Terms for WhatsApp Business Platform, §4.7 "AI Providers" (page "Last Modified: September 23, 2026") [14] (OD, H):

> "Providers and developers of artificial intelligence or machine learning technologies, including but not limited to large language models, generative artificial intelligence platforms, … are strictly prohibited from accessing or using the WhatsApp Business Platform, whether directly or indirectly, for the purposes of providing, delivering, offering, selling, or otherwise making available such technologies when such technologies are the primary (rather than incidental or ancillary) functionality being made available for use, as determined by Meta in its sole discretion…"

- **Data restriction.** §4.7 also says that when a business uses an AI provider as its Solution Provider, WhatsApp data (including anonymized, aggregated or derived forms) may not be used to build or improve AI models, except fine-tuning for that business's exclusive use [14] (OD, H).
- **Customer service is allowed.** Meta told TechCrunch that businesses using AI for customer service (e.g. a travel company's support bot) are not affected [20] (N, M).
- **Exempt markets.** General-purpose assistants are allowed only "where Meta is legally required", currently Brazil (+55) with per-message AI-provider charges [13] (OD, H).
- **Implications:**
  - (a) An SME inquiry or booking bot is allowed, because AI is ancillary to the business's own service.
  - (b) A **personal "ask me anything" admin assistant on WhatsApp is not allowed.**
  - (c) Never pool customers' WhatsApp conversations to train or improve a shared model.

### 3.4 Worked WhatsApp channel cost per SME
Assumptions: 1,000 conversations/month × 6 turns, one business reply per turn, one number, no ad entry.
- Calculation: 6,000 service messages − 1,000 free = 5,000 × $0.0130 = **≈ $65/month**.
- Add any utility reminders, which are charged even inside the window [12].
- If leads come from Click-to-WhatsApp ads, the 72h FEP window makes those replies free [11].
- **Compare:** the low-tier LLM cost for the same volume is $1–$15 (§9). On WhatsApp, Meta's fee is now usually the largest variable cost.

### 3.5 Rust support
- No official SDK.
- Community crate `whatsapp` 0.8.1 (Cloud API client; updated 2026-09-23; about 1.5k downloads) [71].
- **Do not use `whatsapp-rust` or `wacore`.** They are unofficial **WhatsApp Web** protocol clients [71], and using them for a business product risks number bans and terms violations.
- Recommendation: plain HTTPS plus webhooks.

---

## 4. Viber (Rakuten Viber) bots and business messages
- **Commercial terms only.** "since 5.02.24, Viber bots can only be created on commercial terms"; applications go to Rakuten Viber or verified partners [33] (OD, H).
- **Fees.** The maintenance fee is **EUR 100 per bot per calendar month**. Chatbot-initiated messages are billed per delivered message by the subscriber's country code. Session messages (24h after a user reply) are unlimited within the fee [34] (OD via search excerpt, M).
- **PH per-message rate:** not found.
- **Viber Business Messages** (the paid sender-ID product): PH availability and pricing not found in this pass. It is sold through partners.
- **Technical.**
  - Messages only to subscribed users. One welcome message on `conversation_started`, within 5 minutes.
  - Broadcast rate limit: 300 receivers per request, 500 requests per 10 s.
  - Webhooks need a trusted-CA TLS certificate and an HMAC-SHA256 signature (`X-Viber-Content-Signature`) [33] (OD, H).
- **Rust.** No crate of note [71]. The REST API is simple.
- **Verdict.** EUR 100/month per SME bot exceeds a plausible PH SME subscription price, so treat Viber as a later, partner-negotiated add-on.

---

## 5. Email

### Gmail API
- **Scope tiers.** Restricted: `mail.google.com`, `gmail.readonly`, `gmail.compose`, `gmail.insert`, `gmail.modify`, `gmail.metadata`, `gmail.settings.*`. **Sensitive: `gmail.send`**. Non-sensitive: `gmail.labels` [35] (OD, H).
- **Security assessment trigger.** "If you store or transmit restricted scope data on servers, then you need to complete a security assessment" (CASA). Apps must "complete a security assessment at least every 12 months" [36][38] (OD, H).
- **Timeline.** "can potentially take several weeks"; brand verification "typically takes 2-3 business days" [36] (OD, H).
- **Exceptions.** Personal use or "only a few users"; development and testing; internal Workspace org [36] (OD, H). These allow a founder-only or pilot (few users) build without CASA, but not a commercial launch.
- **Cost.** TAC Security CASA Tier 2: $540 (basic) or $720 (premium) per app per year; Tier 3: $3,600 [40] (VC via search excerpt, M). Other labs exist [40]. Google's page does not state cost [36].
- **Send-only option.** `gmail.send` (sensitive) needs only sensitive-scope verification, "typically 3-5 business days" [37] (OD, H).

### Microsoft Graph (Outlook / M365)
- **Publisher verification.**
  - Requires a verified **Microsoft AI Cloud Partner Program** account (partner global account), an Entra work or school tenant, and a custom publisher domain (not `*.onmicrosoft.com`).
  - Free: "Microsoft doesn't charge developers for publisher verification" [44] (OD, H).
  - Without it, when risk-based step-up consent is enabled, "users can't consent to most newly registered multitenant apps that aren't publisher verified" [44] (OD, H).
- **Mail permissions.** Mail.ReadWrite is "considered a high-impact permission and is blocked from end-user consent in the Microsoft-recommended user consent policy" [45] (MS Q&A, M). Expect a tenant-admin consent step per SME.

### Alternatives (recommended for MVP)
- **IMAP with app passwords.** Google turned off less-secure-app (basic auth) access for IMAP, SMTP and POP from **March 14, 2025**, with an exception for app passwords [43] (OD via excerpt, M). App passwords need 2-Step Verification, and the user experience is poor for SMEs. Usable as a stopgap; not recommended.
- **Forwarding to your own inbound domain** (e.g. `acme@in.yourapp.ph`). No Google or Microsoft review.
  - Postmark Pro $16.50/mo with inbound included, 10k emails, $1.30 per extra 1k [46] (PP, H).
  - Amazon SES receiving $0.10 per 1,000 emails + $0.09 per 1,000 256 KB chunks [47] (PP, H). SES receiving availability in ap-southeast-1 was not confirmed [47].
  - Covers document intake (invoices, POs, IDs) and lead capture from contact forms.
- **Rust.**
  - `lettre` (SMTP send; 20.9M downloads, updated 2026-08) [71]
  - `mail-parser` (Stalwart; updated 2026-09) [71]
  - `async-imap` (updated 2026-09) [71]
  - `aws-sdk-sesv2` (official AWS) [71]
  - `postmark` crate 2.0.1 (community) [71]
  - `google-gmail1` (generated, community-maintained by Byron) [71]
  - `graph-rs-sdk` (community, last release Apr 2025) [71]

---

## 6. Calendars and booking
- **Google Calendar.** Google's own example of a sensitive scope is reading events stored in Google Calendar, and sensitive verification "typically takes 3-5 business days" [37] (OD, H).
  - The Calendar API scope page does not label sensitivity per scope [41] (OD, H).
  - A Nylas guide claims `calendar.events` is restricted. This conflicts with Google's example (DR, L).
  - Requirements: a verified domain homepage (not just a login page), a privacy policy, a demo video showing the OAuth grant, and per-scope justification [39] (OD, H).
  - Unverified apps carry a "user cap" [37]. 100 test users in testing mode, per an excerpt (M).
  - Developer report: a verification pending since 22 Apr 2026, with the console estimating **4–6 weeks** [42] (DR, L).
  - Narrower scopes to prefer: `calendar.app.created`, `calendar.events.owned`, `calendar.freebusy` [41].
- **Microsoft Graph calendar.** Same publisher-verification and consent model as mail [44]. Calendar-specific consent classification: not verified (not found).
- **MVP approach.** Make the product's own Postgres the booking calendar. Send ICS attachments, "add to Google Calendar" links and reminders over SMS or Messenger Utility templates. Add Google Calendar OAuth (sensitive, about 1 week) after launch.
- **Rust.** `google-calendar3` (generated, community) [71], or plain HTTP.

---

## 7. Payments (PH)

| Provider | Key fees (VAT-exclusive) | Links / API | Recurring | Evidence |
|---|---|---|---|---|
| **PayMongo** | Local cards 3.125% + ₱13.39; GCash 2.23%; Maya 1.79%; GrabPay 1.96%; ShopeePay 1.70%; QR Ph 1.34%; online banking 0.71% or ₱13.39; no monthly fee; setup free [48] | Payment links, pages, API [48] | Subscriptions API (plans, customers, subscriptions). Needs account configuration via support. Auto-charge on cards and Maya. GCash support conflicts between docs and marketing [49] | PP/OD, H (fees); M (GCash recurring) |
| **Xendit PH** | Local cards 3.2% + ₱10; GCash 2.3%; Maya 1.8%; GrabPay and ShopeePay 2.0%; QR Ph 1.4% or ₱15 (whichever higher); disbursement ₱10 [50] | Invoices, payment links, API | Generic Subscriptions product using a scheduler and payment tokens. PH method coverage not stated [51] | PP via excerpt, M |
| **Maya Business** | Fees not verified (3.5% + ₱10 for cards per a PayMongo comparison, VC, L) | Checkout, Vault (tokenization "for one-time or scheduled payments"), Pay with Maya, Payment Links, email invoices; "All Maya APIs and SDKs are free for developers to use" [52] | Scheduled payments via Vault. No dedicated subscription API found [52] | OD, M |
| Stripe | **Not available**: the Philippines is absent from the supported-countries list [53] | — | — | OD, H |
| Paddle (merchant of record, for USD/intl billing) | 5% + 50¢ per transaction [54] | Checkout | Yes (MoR) | PP, H |

- **SaaS billing to PH SMEs.** A monthly invoice plus a PayMongo or Xendit payment link (GCash, Maya, QR Ph) is the simplest working pattern. Card auto-renew is possible via the PayMongo Subscriptions API.
- **SMEs' own payment links.** Each SME needs its own merchant account (KYB). Building "connect your PayMongo/Xendit" support is account-linking work, not a platform approval.
- **Rust.** No PayMongo crate. `xendit` 0.0.2 (2024, negligible) [71]. Use plain HTTP and verify webhook signatures.

---

## 8. SMS (PH)
- **Semaphore.**
  - **₱0.56 per text** on all networks (Globe, Smart, Sun, DITO). The homepage says "ex. VAT"; the FAQ says "inclusive of VAT", so the two pages conflict [55][56] (PP, H on price; M on VAT).
  - Sender names "take up to 5 business days to approve" and need a sample message. The first sender name is free; additional ones cost 200 credits/month [56] (OD, H).
- **Twilio (PH).**
  - Outbound **$0.241 per segment**, inbound $0.024, failed message $0.001.
  - Alphanumeric sender IDs are listed at the same outbound price.
  - PH mobile numbers cost $142/mo [57] (PP, H).
  - About 7× Semaphore's per-message price (exchange rate not sourced here).
- **Globe Labs.** The API needs subscriber consent (opt-in) per the sample repo. Pricing not found (L).
- **Link blocking (NTC).**
  - A Sept 2023 amended memo has telcos block or deactivate SMS containing clickable URLs, shorteners and QR codes [58] (Globe press release, M).
  - A Sept 12, 2025 NTC memo ordered deactivation of clickable person-to-person URLs [59] (N, M).
  - Whether registered A2P sender IDs are exempt: not found.
  - **Design implication:** don't rely on links in SMS (payment, booking or quote links). Send plain-text reminders and route actions to Messenger, WhatsApp or the web.
- **SIM Registration Act (RA 11934).** Relevant only if you run your own SIM-based gateway. Juridical entities register SIMs with a certificate of registration and a board resolution or SPA [60] (law-firm summary, M). Using an aggregator API avoids this.
- **Rust.** No Semaphore crate. Twilio crates are stale (2021–2024) [71]. Use plain HTTP.

---

## 9. LLM providers: pricing, data terms, Filipino quality

### 9.1 Current pricing (USD per 1M tokens, verified 2026-10-18)

| Provider / model | Input | Cache write | Cache read | Output | Batch | Source |
|---|---|---|---|---|---|---|
| **Claude Haiku 5.5** (prompt ≤100k) | 0.10 | 0.125 (5m) / 0.20 (1h) | 0.01 | 0.50 | −50% | [1] PP H |
| Claude Haiku 4.5 | 1.00 | 1.25 | 0.10 | 5.00 | −50% | [1] |
| **Claude Sonnet 5.5** | 2.00 | 2.50 | 0.10 | 10.00 | −50% | [1] |
| Claude Opus 5.5 | 4.00 | 5.00 | 0.20 | 20.00 | −50% | [1] |
| OpenAI **gpt-6-luna** | 0.10 | n/a | 0.01 | 0.50 | −50% | [6] PP H |
| OpenAI gpt-5-mini | 0.25 | n/a | 0.025 | 2.00 | −50% | [6] |
| OpenAI gpt-5.4-mini | 0.75 | n/a | 0.075 | 4.50 | −50% | [6] |
| Gemini 3.1 Flash-Lite | 0.25 | n/a (+storage) | 0.025 | 1.50 | −50% | [8] PP H |
| Gemini 3.8 Flash | 0.75 (to Dec 31, 2026), then 1.50 | n/a (+storage) | 0.075, then 0.15 | 3.75, then 7.50 | −50% | [8] |
| DeepInfra DeepSeek-V4-Flash (open weights) | 0.09 | n/a | 0.018 | 0.18 | — | [9] PP H |
| DeepInfra Qwen3.5-35B-A3B | 0.14 | n/a | 0.05 | 1.00 | — | [9] |

Notes:
- Claude 4.7+ models use a tokenizer that "produces approximately 30% more tokens for the same text" [1].
- Haiku 5.5 prices rise above 100k-token prompts [1].
- US-only inference (`inference_geo: "us"`) costs 1.1× [1].
- Anthropic has no official Rust SDK; official SDKs are Python, TypeScript, C#, Go, Java, PHP and Ruby [5].
- Anthropic describes Haiku 5.5 as "for high-volume, latency-sensitive tasks such as classification, extraction, and routing" [2].

### 9.2 Worked per-SME monthly cost (1,000 conversations × 6 turns)

| Assumption | Low | High |
|---|---|---|
| LLM calls | 6,000 (1 per turn) | 12,000 (2 per turn: tool use or extraction) |
| Cached prefix (system prompt + business FAQ/KB + tools) | 3,000 tok; written once per conversation (1,000 writes), read on other calls | 10,000 tok; cache misses on 50% of calls (5-min TTL expires between turns) |
| Uncached input per call (history + user msg + tool results) | 700 tok | 3,000 tok |
| Output per call (incl. thinking) | 150 tok | 400 tok |
| Monthly totals | 4.2M uncached, 3M cache-write, 15M cache-read, 0.9M out | 36M uncached, 60M write, 60M read, 4.8M out |

Non-Anthropic providers have no write premium, so cache writes are billed as normal input.

| Model | Low $/SME/mo | High $/SME/mo |
|---|---|---|
| Claude Haiku 5.5 | **1.40** | **14.10** |
| Claude Haiku 4.5 | 13.95 | 141.00 |
| Claude Sonnet 5.5 | 26.40 | 276.00 |
| OpenAI gpt-6-luna | 1.32 | 12.60 |
| OpenAI gpt-5-mini | 3.97 | 35.10 |
| OpenAI gpt-5.4-mini | 10.57 | 98.10 |
| Gemini 3.1 Flash-Lite | 3.52 | 32.70 |
| Gemini 3.8 Flash (2026 price / 2027 price) | 9.90 / 19.80 | 94.50 / 189.00 |
| DeepSeek-V4-Flash (DeepInfra) | 1.08 | 10.58 |

Reading the numbers:
- With current small models, the LLM cost of a 1,000-conversation SME is **about $1–$15/month**.
- Routing only hard turns (complex quotes, document extraction) to a Sonnet-class model keeps the blended cost under about $30.
- Taglish text may tokenize less efficiently than English (assumption; not measured). Budget +30% on the high case.
- Document intake (PDF/image pages) is extra and depends on page count.

### 9.3 Data retention and usage
- **Anthropic.**
  - Inputs and outputs are deleted "within 30 days of receipt or generation" by default [4] (OD, H).
  - Retained data "is never used for model training without your express permission" [3] (OD, H).
  - ZDR by contacting sales, enabled per organization [3].
  - Fable 5.x and Mythos 5.x "require 30-day data retention" (no ZDR) [3].
- **OpenAI.**
  - API data "is not used to train or improve OpenAI models" unless you opt in [7] (OD, H).
  - Abuse-monitoring logs are kept "up to 30 days" [7].
  - ZDR is "subject to prior approval" [7].
  - Singapore offers regional *storage* only, not processing [7].
- **Gemini.**
  - The **free tier is "Used to improve our products: Yes"**; the paid tier is "No" [8] (OD, H).
  - **Never put customer data on the free tier.**
- **DeepInfra.** The pricing page has no retention statement (not found) [9]. Review its terms before sending PII.
- **WhatsApp-specific.** WhatsApp data must not be used to build or improve AI models beyond the business's own fine-tune [14]. Default no-training API terms satisfy this; shared fine-tuning on pooled chats does not.
- **Philippine Data Privacy Act** obligations: not researched in this pass.

### 9.4 Tagalog / Taglish quality evidence
- **FilBench** (Miranda et al., EMNLP 2025) evaluated 27 LLMs on Filipino, Tagalog and Cebuano [10] (paper, H).
  - Top model GPT-4o: 72.23%. SEA-LION v3 70B: 61.07%.
  - Text generation was the weakest category (hallucination, poor instruction-following) [10].
- No published Taglish customer-chat benchmark covering current (2026) models was found. **Build a small in-house eval** of 50–100 real anonymized PH inquiries before choosing a model.

---

## 10. Hosting near the Philippines (Singapore)

| Option | Small production setup | Postgres | Notes | Source |
|---|---|---|---|---|
| **Fly.io `sin`** | shared-cpu-1x 1 GB ≈ **$8.50/mo** in sin (vs $6.70 in iad); APAC egress $0.04/GB | Managed Postgres from **$38/mo** (Basic, 1 GB, HA included) + $0.28/GB; sin availability not stated | Simple Rust deploys (Docker) | [61][62] PP H |
| **DigitalOcean SGP1** | Droplet $6 (1 GiB, 1,000 GiB transfer); $12 (2 GiB) | Managed PG **$15.15/mo** (1 GiB) | Regional price differences not stated | [64][65] PP H |
| **AWS Lightsail (Singapore)** | $5 (0.5 GB, 1 TB transfer); $3.50 IPv6-only | Managed DB **$15** standard, $30 HA | Singapore not in the half-transfer list | [66] PP H |
| **Hetzner SIN1** | CPX02 1 vCPU/1 GB … CPX22 2 vCPU/4 GB; **only 0.5–1 TB traffic in SIN** (vs 20 TB EU) | Self-managed | Prices not shown; "This product is currently unavailable" on the page at access time | [63] PP, M |
| **Supabase (ap-southeast-1)** | — | Pro **$25/mo** incl. $10 compute credit (Micro 1 GB), 8 GB disk | Singapore region confirmed | [67][68] PP/OD H |
| **Neon** | — | Pay-as-you-go $0.106/CU-hr, $0.35/GB-mo, no minimum | Singapore region not confirmed | [69] PP, M |
| **Railway** | Hobby $5/mo (includes $5 of usage); $20/vCPU-mo, $10/GB-RAM-mo | — | No SEA region on the pricing page | [70] PP, M |

**Suggested MVP stack**, about $25–$45/month:
- Fly.io sin app (1–2 machines, ~$9–17) + Supabase Pro Singapore ($25), or
- DigitalOcean SGP1 droplet + managed PG (~$21).

Manila-to-Singapore latency was not measured in this research. Singapore is the nearest major region offered by every option above.

---

## 11. Rust SDK availability (crates.io, queried 2026-10-18) [71]

| Integration | Official Rust SDK? | Best option |
|---|---|---|
| Anthropic | No [5] | HTTP; `genai` 0.6.5 (multi-provider, updated 2026-10-16), `rig-core` 0.44 (agent framework). The `anthropic` crate is stale (2024) |
| OpenAI | No | `async-openai` 0.42.2 (9.4M downloads, updated 2026-10-10) |
| Gemini / Vertex | Yes (Google Cloud Rust libraries) | `google-cloud-aiplatform-v1` 1.17 (googleapis/google-cloud-rust) |
| Bedrock (Claude) | Yes (AWS) | `aws-sdk-bedrockruntime` |
| Messenger / Instagram | No | HTTP + webhooks (community crates stale) |
| WhatsApp Cloud API | No | HTTP; `whatsapp` 0.8.1 (small community). Avoid WhatsApp Web crates |
| Viber | No | HTTP |
| Gmail / Calendar | Generated community crates | `google-gmail1`, `google-calendar3` (Byron/google-apis-rs, 2026-01) |
| Microsoft Graph | No (community) | `graph-rs-sdk` 3.0.1 (Apr 2025), or HTTP |
| Email infrastructure | Yes (AWS) / community | `aws-sdk-sesv2`, `lettre`, `mail-parser`, `async-imap`, `postmark` |
| PayMongo / Xendit / Maya | No | HTTP + webhook HMAC |
| Semaphore / Twilio | No (Twilio crates stale) | HTTP |

Every integration on the list is "REST + webhooks", so Rust has no blocking gaps. The cost is writing about 10 thin clients, which is roughly 1–3 days each with serde types.

---

## 12. Feasibility ranking for an MVP (easiest to hardest)

1. **Web chat widget + LLM** (no approvals; $1–15/SME LLM cost). Ships in days.
2. **Inbound email address / forwarding for document intake** (Postmark or SES; no OAuth review). Ships in days.
3. **SMS reminders via Semaphore** (sender name ≤5 business days; ₱0.56/msg). Text only, because links are risky.
4. **Payment links via PayMongo/Xendit** (merchant KYB of your registered business; no platform review). Recurring is possible on cards and Maya.
5. **Messenger on your own Page, then other businesses' Pages.** Free per message and the highest-value inquiry channel. Gated by Business Verification, App Review and Access Verification (about 5 days official for the last step; 2–6 weeks total assumed). Outside-24h follow-ups need Utility templates after the tag deprecation.
6. **Instagram DM.** Same gate as Messenger (permission `instagram_manage_messages`); the vendor estimate is 2–4 weeks.
7. **Google Calendar sync.** Sensitive scope: 3–5 business days officially, with real-world reports of weeks. Domain, privacy policy and video required.
8. **WhatsApp Cloud API (Tech Provider).** Same Meta verification chain plus template management. 10 onboardings/week until verified. **Per-reply fees from Oct 1, 2026** (~$65/mo for a 1,000-conversation SME after the free tier). AI policy permits customer service only.
9. **Microsoft Graph mail/calendar.** Needs Partner Program publisher verification and per-tenant admin consent for Mail.ReadWrite.
10. **Viber bots.** Commercial contract plus EUR 100/bot/month, which breaks SME unit economics.
11. **Gmail API read/modify (personal email assistant).** Restricted scopes, CASA every 12 months ($540+/yr), "several weeks". Defer until there is revenue, or use forwarding.

**Start now:** register the business (DTI/SEC), then file Meta Business Verification and Access Verification in week 1. These run in parallel with building items 1–4.

---

## Sources

1. Anthropic, "Pricing", platform.claude.com/docs/en/about-claude/pricing. Pricing page; accessed 2026-10-18; H.
2. Anthropic, "Models overview", platform.claude.com/docs/en/about-claude/models/overview. Official docs; accessed 2026-10-18; H.
3. Anthropic, "API and data retention", platform.claude.com/docs/en/manage-claude/api-and-data-retention. Official docs; accessed 2026-10-18; H.
4. Anthropic Privacy Center, "How long do you store my organization's data?", privacy.claude.com/en/articles/7996866. Official; accessed 2026-10-18; H.
5. Anthropic, "SDKs, CLI, and libraries", platform.claude.com/docs/en/cli-sdks-libraries/overview. Official docs; accessed 2026-10-18; H.
6. OpenAI, "Pricing", developers.openai.com/api/docs/pricing. Pricing page; accessed 2026-10-18; H.
7. OpenAI, "Data controls in the OpenAI platform" (your-data), developers.openai.com/api/docs/guides/your-data. Official docs; accessed 2026-10-18; H.
8. Google, "Gemini Developer API pricing", ai.google.dev/gemini-api/docs/pricing. Pricing page; accessed 2026-10-18; H.
9. DeepInfra, "Pricing", deepinfra.com/pricing. Pricing page; accessed 2026-10-18; H.
10. Miranda et al., "FilBench: Can LLMs Understand and Generate Filipino?", arxiv.org/abs/2508.03523 (EMNLP 2025). Paper; accessed 2026-10-18; H.
11. Meta, "Pricing on the WhatsApp Business Platform", developers.facebook.com/documentation/business-messaging/whatsapp/pricing. Official docs; accessed 2026-10-18; H.
12. Meta, "Upcoming pricing updates for Meta Business Agent, service and utility messages", developers.facebook.com/documentation/business-messaging/whatsapp/pricing/non-template-messages. Official docs; accessed 2026-10-18; H (free-tier line via search excerpt: M).
13. Meta, "New pricing policy for AI Providers leveraging the WhatsApp Business Platform", developers.facebook.com/documentation/business-messaging/whatsapp/pricing/ai-providers. Official docs (updated May 12, 2026); accessed 2026-10-18; H.
14. Meta, "Meta Terms for WhatsApp Business Platform" §4.7, facebook.com/legal/Meta-Terms-for-WhatsApp-Business-Platform. Official terms (Last Modified Sept 23, 2026); accessed 2026-10-18; H.
15. Meta, "Embedded Signup", developers.facebook.com/docs/whatsapp/embedded-signup. Official docs; accessed 2026-10-18; H.
16. Meta, "Get started for Tech Providers", developers.facebook.com/docs/whatsapp/solution-providers/get-started-for-tech-providers. Official docs; accessed 2026-10-18; H.
17. Gallabox, "WhatsApp rate card", docs.gallabox.com/pricing-and-billing/whatsapp-pricing/rate-card. Vendor reproduction of Meta's rate card (eff. Oct 1, 2026); accessed 2026-10-18; M.
18. 360dialog, "Service Message Charging Starts October 1, 2026", 360dialog.com/blog/whatsapp-service-message-charging-october-2026/. Vendor (BSP) blog, cites Meta docs; accessed 2026-10-18; M.
19. WhatsApp Business, "Platform pricing", whatsappbusiness.com/products/platform-pricing/. Marketing page (stale on service charges); accessed 2026-10-18; L.
20. TechCrunch, "WhatsApp changes its terms to bar general-purpose chatbots", techcrunch.com/?p=3082985 (Oct 2025). News; seen via search summaries; accessed 2026-10-18; M.
21. Meta, "Messenger Platform overview", developers.facebook.com/docs/messenger-platform/overview. Official docs; accessed 2026-10-18; H.
22. Meta, "Messenger Platform policy overview", developers.facebook.com/docs/messenger-platform/policy/policy-overview. Official docs; accessed 2026-10-18; H.
23. Meta, "Messenger Platform changelog", developers.facebook.com/docs/messenger-platform/changelog. Official docs (entries Mar 27, 2026; Oct 23, 2025); accessed 2026-10-18; H.
24. Meta, "Conversation Routing", developers.facebook.com/docs/messenger-platform/conversation-routing. Official docs; accessed 2026-10-18; H.
25. Meta, "Rate limiting (Messenger/Instagram)", developers.facebook.com/documentation/business-messaging/messenger-platform/overview/rate-limiting. Official docs; accessed 2026-10-18; H.
26. Meta, "Messenger App Review", developers.facebook.com/docs/messenger-platform/app-review. Official docs; accessed 2026-10-18; H.
27. Meta, "Utility Messages", developers.facebook.com/docs/messenger-platform/send-messages/utility-messages. Official docs; accessed 2026-10-18; H.
28. Meta, "Instagram Messaging App Review: apps for other businesses", developers.facebook.com/documentation/business-messaging/instagram-messaging/app-review/apps-for-other-businesses. Official docs; accessed 2026-10-18; H.
29. Meta, "Access Verification", developers.facebook.com/docs/development/release/access-verification. Official docs; accessed 2026-10-18; H.
30. Meta, "Business Verification", developers.facebook.com/docs/development/release/business-verification. Official docs; accessed 2026-10-18; H.
31. A. Singh, "Meta Deprecated Three Message Tags…", singhamandeep.com/?p=1586 (2026). Developer report; accessed 2026-10-18; M.
32. Phyllo, "Instagram API Permission Scopes Explained", getphyllo.com/post/instagram-api-permissions-scopes (Sept 2026). Vendor blog; accessed 2026-10-18; L.
33. Rakuten Viber, "REST Bot API", developers.viber.com/docs/api/rest-bot-api/. Official docs; accessed 2026-10-18; H.
34. Rakuten Viber Help Center, "Bot commercial model", help.viber.com/hc/articles/15247629658525. Official (via search excerpt; direct fetch 403); accessed 2026-10-18; M.
35. Google, "Choose Gmail API scopes", developers.google.com/workspace/gmail/api/auth/scopes. Official docs; accessed 2026-10-18; H.
36. Google, "Restricted scope verification", developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification. Official docs; accessed 2026-10-18; H.
37. Google, "Sensitive scope verification", developers.google.com/identity/protocols/oauth2/production-readiness/sensitive-scope-verification. Official docs; accessed 2026-10-18; H.
38. Google Cloud Help, "Security assessment", support.google.com/cloud/answer/13465431. Official; accessed 2026-10-18; H.
39. Google Cloud Help, "OAuth app verification requirements", support.google.com/cloud/answer/13464321. Official; accessed 2026-10-18; H.
40. TAC Security, "ESOF AppSec – ADA CASA" pricing, tacsecurity.com/esof/. Vendor pricing (via search excerpt; direct fetch 403); accessed 2026-10-18; M.
41. Google, "Choose Google Calendar API scopes", developers.google.com/workspace/calendar/api/auth. Official docs; accessed 2026-10-18; H.
42. Google Developer forum, "OAuth verification pending since 22 Apr 2026…", discuss.google.dev/t/…/392005. Developer report; accessed 2026-10-18; L.
43. Google Workspace Admin Help, "Transition from less secure apps to OAuth", knowledge.workspace.google.com/admin/sync/transition-from-less-secure-apps-to-oauth. Official (via search excerpt); accessed 2026-10-18; M.
44. Microsoft Learn, "Publisher verification overview", learn.microsoft.com/en-us/entra/identity-platform/publisher-verification-overview (updated 2026-06-15). Official docs; accessed 2026-10-18; H.
45. Microsoft Q&A, "How to add Mail.ReadWrite permission in user consent…", learn.microsoft.com/en-us/answers/questions/5861727 (Apr 2026). Community/MS Q&A; accessed 2026-10-18; M.
46. Postmark, "Pricing", postmarkapp.com/pricing. Pricing page; accessed 2026-10-18; H.
47. AWS, "Amazon SES pricing", aws.amazon.com/ses/pricing/. Pricing page; accessed 2026-10-18; H.
48. PayMongo, "Pricing", paymongo.com/pricing. Pricing page; accessed 2026-10-18; H.
49. PayMongo, "Subscriptions" docs, docs.paymongo.com/docs/payment-acceptance-subscriptions and developers.paymongo.com/docs/subscriptions-api. Official docs (via search excerpt); accessed 2026-10-18; M.
50. Xendit, "Pricing (Philippines)", xendit.co/en-ph/pricing/. Pricing page (via search excerpt; direct fetch 403); accessed 2026-10-18; M.
51. Xendit, "Subscriptions overview", docs.xendit.co/docs/subscriptions-overview. Official docs; accessed 2026-10-18; M.
52. Maya, "Maya Developers", developers.maya.ph. Official; accessed 2026-10-18; M.
53. Stripe, "Stripe global availability", stripe.com/global. Official; accessed 2026-10-18; H.
54. Paddle, "Pricing", paddle.com/pricing. Pricing page; accessed 2026-10-18; H.
55. Semaphore, homepage, semaphore.co. Pricing; accessed 2026-10-18; H.
56. Semaphore, "FAQ", semaphore.co/faq. Official; accessed 2026-10-18; H.
57. Twilio, "SMS pricing – Philippines", twilio.com/en-us/sms/pricing/ph. Pricing page; accessed 2026-10-18; H.
58. Globe Telecom, "Globe lauds NTC memo requiring telcos to block SMS with clickable links", globe.com.ph/about-us/newsroom/…. Company press release (2023); accessed 2026-10-18; M.
59. BusinessWorld / Malaya, "NTC orders telcos to deactivate, block links in scam text messages", bworldonline.com/?p=474395 (Sept 2025). News (via search excerpt); accessed 2026-10-18; M.
60. NDV Law, "What must businesses submit for enterprise SIM registration?", ndvlaw.com/…; RA 11934 IRR (NTC MC 001-12-2022). Law-firm summary; accessed 2026-10-18; M.
61. Fly.io, "Pricing", docs.fly.io/about/pricing. Pricing page; accessed 2026-10-18; H.
62. Fly.io, "Managed Postgres", docs.fly.io/mpg. Official docs; accessed 2026-10-18; H.
63. Hetzner, "Cloud – Regular Performance", hetzner.com/cloud/regular-performance. Pricing page (prices not rendered); accessed 2026-10-18; M.
64. DigitalOcean, "Droplet pricing", digitalocean.com/pricing/droplets. Pricing page; accessed 2026-10-18; H.
65. DigitalOcean, "Managed Databases pricing", digitalocean.com/pricing/managed-databases. Pricing page; accessed 2026-10-18; H.
66. AWS, "Amazon Lightsail pricing", aws.amazon.com/lightsail/pricing/. Pricing page; accessed 2026-10-18; H.
67. Supabase, "Pricing", supabase.com/pricing. Pricing page; accessed 2026-10-18; H.
68. Supabase, "Available regions", supabase.com/docs/guides/platform/regions. Official docs; accessed 2026-10-18; H.
69. Neon, "Pricing", neon.com/pricing. Pricing page; accessed 2026-10-18; H.
70. Railway, "Pricing plans", docs.railway.com/reference/pricing/plans. Official docs; accessed 2026-10-18; H.
71. crates.io registry API (crate metadata: versions, downloads, last-updated), crates.io/api/v1/crates. Registry data; queried 2026-10-18; H.
