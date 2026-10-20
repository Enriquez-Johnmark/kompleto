# New product ideas: Philippines and SEA, self-serve

Status: **research pass, 2026-10-18.** All sources accessed 2026-10-18. The scoring uses [scoring-method.md](scoring-method.md) unchanged. Candidates already in [opportunity-landscape.md](opportunity-landscape.md) are excluded. Bracketed numbers point to the Sources list at the end. INT and PH refer to [integration-constraints.md](integration-constraints.md) and [market-philippines.md](market-philippines.md).

**Method limit.** The session's shared web-search budget ran out early in this pass. Several facts were therefore marked "not found" when they probably exist: Facebook-group member counts, Lazada and TikTok Shop developer terms, PRC examinee counts, and Filipino-language AI product prices. Where a page could not be fetched and only a search-engine summary was available, the source line says "via search summary". Treat those sources as one notch less reliable.

---

## 1. Key conclusions

1. **Platform owners are now giving away marketplace chat, as Meta did for Messenger.**
   - Shopee runs its own Chat AI Assistant inside Seller Centre. It answers FAQs from the seller's auto-replies and passes hard chats back to the seller [5][7].
   - Shopee reports that use of its "AI customer service assistant" grew 75% among Visayas and Mindanao merchants (company report) [2].
   - TikTok Shop's official chatbot gets weekend messages exempted from response-rate penalties (vendor summary, Malaysia) [8].
   - BigSeller's free plan advertises auto-reply [11]. Paid multichannel AI inboxes such as Zaapi start at $79/mo [12].
   - **A marketplace chat bot is the wrong first product.** *Confidence: M–H.*
2. **Facebook live-selling order capture is crowded too.** BigSeller (free plan), EasyStore (from ₱2,499/mo), Pancake POS (free) and Ulive already turn "mine" comments into orders [11][15][16][17][18][19]. Building on Facebook comments also hits Meta's App Review and Tech Provider gates (INT §1). *Confidence: M.*
3. **The strongest new signal is regulation that creates recurring paperwork for millions of online sellers.**
   - Shopee has required all sellers to register with the BIR since March 2024 [28].
   - Under RR 16-2023, Shopee and Lazada have withheld an effective 0.5% of gross remittances since 15 July 2024. The rule also covers GCash and Maya as digital financial service providers [29][30][32].
   - Sellers at or below ₱500k must file a BIR-stamped sworn declaration by 20 January each year to stay exempt [29].
   - BIR district offices say they monitor Facebook, Shopee, Lazada and TikTok for unregistered sellers [31].
   - Paid tax tools already exist: Taxumo's 8% plan costs ₱2,699 a quarter, and Taxumo claims 144,000+ users [24][25]. Juan's paid plans start at ₱2,000/mo [26].
   - On their own pages, neither incumbent mentions importing marketplace payout reports or reconciling Form 2307 certificates [25][26][27].
   - *Confidence: M (rules via professional and vendor summaries, not BIR text).*
4. **Fake GCash receipts are a real, documented seller risk.**
   - On 14 July 2025 GCash's operator Mynt warned about AI-generated fake receipts. It told sellers not to release goods against a screenshot and to check the in-app Transactions tab [21].
   - The pain is real, but the root-cause fix is a payment link or QR Ph with automatic confirmation (PayMongo 2.23% for GCash, PH §7).
   - A screenshot checker cannot verify anything on its own, because personal GCash accounts have no API.
   - *Confidence: H for the warning, L for willingness to pay.*
5. **Education and household paperwork show weak willingness to pay.**
   - Nursing board-exam reviewer apps charge US$1.99/mo or $14.99/yr and have too few ratings to display [38][39].
   - DepEd lesson logs are given away by template sites [36], and teachers are pointed at free ChatGPT and Copilot [37].
   - Only about 6% of 1.4M kasambahay were enrolled in SSS by their employers (2019 PSA–DOLE survey) [34]. Non-compliance is the norm, so there is little compliance pull.
6. **Top pick: a seller tax desk.** It is an approval-gated agent that turns marketplace and e-wallet records into BIR-ready numbers, deadlines and draft forms for online sellers. Section 3.1 explains why. It needs **no platform API approval** (sellers upload exports) and faces **no platform-owner competitor** (platforms withhold tax but do not file it). It also has the clearest paid comparables in the segment.

---

## 2. Areas 1–7 in brief

### Area 1: Shopee, Lazada and TikTok Shop sellers

- **Size.**
  - TikTok Shop said in 2024 that it had "more than 2 million local businesses" registered, mostly SMEs. A company official repeated "2 million sellers" in October 2024 [3].
  - Shopee publishes no absolute PH seller count. Its impact report says local sellers "doubled each year" over a decade and that 400,000+ Filipino sellers joined the Shopee International Platform [1].
  - Cube Asia's estimate puts Shopee PH at US$12B GMV and 399M orders in 2025 [4] (via search summary).
  - Lazada seller count: not found.
- **Pain (response-rate rules).**
  - Shopee Preferred status has needed a chat response rate of 75% in one 2024 summary [9]. A 2026 Malaysia guide says above 85% [8].
  - Penalty points accumulate, and 15 points freeze a PH account for 28 days (Ginee, via search summary) [10].
  - TikTok Shop moved to a 12-hour response-rate metric in July and August 2025 (Malaysia) [8].
  - The PH-specific current thresholds were not verified in Seller Centre.
- **APIs.**
  - Shopee Open Platform has three account types. The ISV type needs a registered business, a **live product with existing e-commerce integrations**, HTTPS and a test login [13]. That is a chicken-and-egg requirement for a new product.
  - A seller's own "in-house" app is reviewed in about 3 working days plus 24 hours, per an integrator's documentation [14]. Chat APIs exist, but the precise access rules were not found [13][14].
  - Lazada and TikTok Shop partner terms could not be fetched (the pages are JavaScript-only). Not found.
- **Paid tools.**
  - BigSeller: free up to 1,500 orders/mo and 3 stores; paid from SGD 39/mo, about ₱1,650 (vendor's own blog) [11].
  - Zaapi: $79–179/mo; AI usage billed at $40 per 1,000 messages; Shopee and Lazada review management from Pro [12].
  - SiteGiant PH: "contact us" pricing.
  - Ginee and Duoke: prices not found.
- **Complaints about these tools:** not found.

### Area 2: Facebook live sellers and "mine" selling

- **Workflow.** A buyer types "MINE" or "MINE [code]". The seller confirms each order by Messenger after the live and sends an invoice or payment link (GCash, Maya, bank or COD). Missed comments and overselling are the stated pains [15][16].
- **Tools.**
  - BigSeller: Facebook Live Order with automatic capture and inventory sync, on its free plan [15].
  - EasyStore PH: keyword comment sends an automatic Messenger checkout link. Standard plan ₱2,499/mo [16][17].
  - Pancake: "automate order capture from livestreams and chats"; POS free; claims 260,000+ businesses worldwide [19].
  - Ulive: Malaysia-focused [18].
  - Botcake (Pancake's bot): free up to 500,000 automated messages a month; Pro from $30; AI replies use the seller's own OpenAI key [20].
- **Courier context.** Ninja Van cites 86% COD use (Statista 2022). It recommends confirming name, address and phone before shipping to cut returns [23]. A PH return-to-sender rate was not found.

### Area 3: Filipino-language AI tools

- **Senti AI** sells Filipino-language customer-experience AI to banks and retailers. It is described as bootstrapped to profitability; no figures [40].
- **GradeChum** grades handwritten Tagalog, Bisaya and Ilocano work and has a ₱15M seed round [40].
- A June 2026 report mentions a customer-support chatbot that "learned to speak Filipino"; no usage figures [41].
- **No self-serve Taglish writing, caption or reply tool with published paying users was found.** Filipino fluency is a feature of the product, not a market on its own.

### Area 4: Government paperwork

- **BIR for online sellers.** This is the strongest area; see §3.1.
  - Taxumo: free tier; 8% plan ₱1,374–2,699 per quarter depending on term; Micro ₱2,124–4,995 per quarter; retainer accountant ₱3,000/mo [24].
  - JuanTax calls itself the "first BIR-accredited tax platform" [27]. Juan's paid plans start at ₱2,000/mo [26].
  - Late filing costs a 25% surcharge, 12% yearly interest and a ₱1,000 compromise penalty per return (professional summary) [30].
- **Household employers.** RA 10361 duties exist, but compliance is about 6% for SSS, 5% for PhilHealth and 3.4% for Pag-IBIG [34]. Only 2.5% of kasambahay have written contracts [35]. See §3.5.
- **DTI and LGU renewals.** Not researched; no budget left.

### Area 5: Education

- **PRC exams.** Reviewer apps are cheap and thin on reviews: US$1.99/mo or $14.99/yr [38], or $1.99 one-time [39]. Examinee counts: not found.
- **DepEd.** Free MATATAG lesson logs are published weekly by template sites [36]. A DepEd division publication encourages ChatGPT and Copilot for lesson planning [37]. No paid teacher AI tool with users was found.
- **Agent-core fit is low.** This is content and drill work, not tools, approvals or durable tasks.

### Area 6: GCash, Maya and payment proof

- Mynt's warning names sellers as the target [21].
- A vendor sells screenshot extraction (reference number, amount, sender) and recommends matching against in-app history [22]. Its price was not found.
- RR 16-2023 also makes GCash and Maya withholding agents for sellers [30]. Wallet records therefore matter for tax too, which links Areas 4 and 6.

### Area 7: Other

- **COD order confirmation to cut returns** [23] fits as a feature of §3.3, not as a product.
- **Indonesia** has ordered four marketplaces to collect seller income tax with a turnover exemption (PMK 37/2025, via search summary) [33]. That is the same pattern as RR 16-2023, so it is a SEA expansion path for §3.1.

---

## 3. Candidate deep-dives

Score strings are C1–C16 in order, from [scoring-method.md](scoring-method.md). Weighted totals were computed with that file's weights.

### 3.1 S1: Seller tax desk (BIR compliance agent for online sellers)

**Problem.**
- Sellers must be BIR-registered to sell on Shopee [28].
- They must track cumulative gross sales against ₱500k, file the sworn declaration by 20 January, and track 0.5% withheld by each platform and e-wallet.
- They must collect the Form 2307 certificates that platforms should issue within 20 days after each quarter [29][30].
- They must file quarterly and annual returns. Late returns cost a 25% surcharge, 12% interest and ₱1,000 per return [30].
- Without a TIN, withheld amounts cannot be claimed as credit. Incomplete business information can hide listings [29].

**Who has it.**
- TikTok Shop: about 2M registered sellers [3].
- Shopee: not published in absolute terms, but large [1][4].
- The share above ₱500k is not found.

**Frequency.** Continuous record-keeping, quarterly filings and an annual declaration. *Score: medium.*

**Evidence of payment.**
- Taxumo ₱2,699/quarter for the 8% plan (price: H; "144,000+ users" is a **vendor claim**) [24][25].
- Juan from ₱2,000/mo (vendor) [26].
- Taxumo's ₱3,000/mo retainer shows the human alternative [24].

**Competitors.**
- Taxumo and JuanTax/Juan, plus local bookkeepers.
- **Gap (J):** neither homepage mentions importing Shopee, Lazada or TikTok payout reports or reconciling 2307s [25][27]. This must be checked by signing up to their free tiers.

**Complaints.** No published complaints about Taxumo or JuanTax were found (search budget).

**Agent workflow (approval-gated).**
1. The seller forwards or uploads platform income exports, GCash or Maya statements, 2307 PDFs and receipt photos.
2. The agent extracts and categorises each item, then keeps a running gross-sales ledger and a withheld-tax ledger.
3. It raises durable, scheduled alerts: approaching ₱500k, sworn declaration due 20 January, quarter-end, and missing 2307s 20 days after quarter-end.
4. It drafts a Taglish request to platform support for a missing 2307 and the seller **approves before it is sent**.
5. It prepares draft return values with a worked explanation, and the seller confirms and files in eBIRForms or a BIR-accredited channel.
6. The product **never files and never pays**.

**Integrations and gatekeeping.**
- None needed for the MVP: file upload, email forwarding and SMS reminders (Semaphore ₱0.56/text, INT §8).
- The builder can open his own Shopee, Lazada and TikTok seller accounts to get the export formats.
- Direct filing would need BIR accreditation. Whether a "prepare only" tool needs any accreditation was **not found** and is the first thing to check.

**Free distribution.**
- Taglish TikTok and Reels explainers ("Paano mag-sworn declaration sa Shopee"), SEO on long-tail rule terms, and a free "₱500k threshold tracker" as the hook.
- Signal of search demand: seller-tool vendors publish repeated PH tax explainers [28][29][30] (J).
- Facebook seller-group sizes: not found.
- Nothing about the product makes it spread by itself.

**MVP size (one developer).** Parsers for 3 marketplace exports and 2 wallet statements, an extraction pipeline, a ledger, a deadline scheduler, draft-form worksheets and a reminder channel: about 5–7 weeks (J).

**Regulatory and liability risk.**
- Financial data falls under the Data Privacy Act.
- Wrong numbers lead to penalties.
- Mitigations: "prepare and explain, you file", visible computations, and an option to refer to a partner CPA.

**What would disprove it.**
- (a) Taxumo or Juan already import marketplace reports.
- (b) Under about 1% of viewers of 10 explainer videos or 3–5 group posts sign up for the free tracker.
- (c) Most sellers are under ₱500k, unregistered, and unafraid of enforcement.
- (d) BIR rules treat a preparation tool as software that needs accreditation.

**Score: `4E 3J 4J 5E 4E 3J 4J 4J 4J 3J 2J 4J 3J 5E 4E 4J`**
- Balanced 76.0 · Revenue-first 79.0 · Builder-first 75.8 · 5/16 evidence-backed.
- C4 = 5E because people in this segment already pay Taxumo and Juan.
- C11 = 2J because of tax and financial data.

### 3.2 S2: Payment-proof checker and paid-orders ledger

**Problem.** AI-generated fake GCash receipts target sellers who release goods against screenshots [21].

**Who has it.**
- Any social seller who takes e-wallet transfers.
- Count not found. A vendor projects 45M social-commerce users [15]; GCash claims about 81M users (PH §7).

**Frequency.** Every prepaid order.

**Evidence of payment.**
- Indirect only: sellers pay payment-link fees (PayMongo 2.23% for GCash, PH §7) to avoid screenshots.
- A screenshot-extraction vendor exists, but its price was not found [22].

**Competitors.** Payment links, QR Ph with automatic confirmation, and GCash's own Transactions tab [21].

**Workflow.**
1. The seller forwards a screenshot.
2. The agent extracts reference number, amount, sender and time.
3. It flags duplicates and mismatches against the order.
4. When the seller uploads a wallet statement, it matches each proof to a statement line.
5. The seller **approves "mark paid / release"**.
6. Unmatched proofs get a scheduled re-check.

**Integration.** No API for personal GCash accounts. Whether statements can be exported in a parseable format was not verified. **Verification therefore depends on the seller's statement**, and the tool must never claim a proof is "verified".

**Distribution.** Scam-awareness content spreads naturally in seller groups (J).

**MVP.** About 3–4 weeks.

**Risk.** A false "looks fine" leads to lost goods. Financial data.

**What would disprove it.**
- Sellers simply switch to QR Ph or payment links.
- There is no statement export.

**Score: `4E 4E 3J 3J 2J 2J 4J 4J 2J 2J 2J 4J 4J 4J 4J 3J`**
- Balanced 63.2 · Revenue-first 65.2 · Builder-first 63.6.
- **Best used as a feature of S1 or S3**, not as a product on its own.

### 3.3 S3: Live-seller "mine-to-paid" order desk

**Problem.**
- Claims made in the live comments must become paid, shipped orders.
- Missed comments, unpaid claims and wrong addresses cost sales and cause COD returns [15][16][23].

**Who has it.** Facebook and TikTok live sellers. Count not found. A vendor projects that 62% of PH buyers engage in live commerce [15] (vendor, L).

**Frequency.** Every live session.

**Evidence of payment.** EasyStore sells live orders inside ₱2,499/mo plans [17], but BigSeller and Pancake give capture away [11][19].

**Competitors.**
- Capture is commoditised.
- **Possible gap (J):** the after-live chase. That means payment-deadline follow-up, offering the item to the next claimant when a buyer does not pay, COD confirmation before booking, and a proof check (S2).
- None of the vendor pages describe deadline-driven re-offer with approval [15][16][19].

**Workflow.**
1. Claims are ingested.
2. The agent drafts invoice DMs.
3. A durable timer runs to the payment deadline.
4. It drafts "release to next claimant?" and the seller **approves**.
5. It sends a COD confirmation message.
6. It drafts the courier booking and the seller approves.

**Integrations.**
- Facebook comments and Messenger for other businesses' Pages need Business Verification, App Review and Tech Provider access, about 2–6 weeks (INT §1).
- TikTok Live APIs: not found.
- Courier APIs: not researched.

**Distribution.** Live sellers congregate in Facebook seller groups (J; sizes not found).

**MVP.** About 8 weeks, plus gated review.

**What would disprove it.** BigSeller's free tier already handles unpaid-claim follow-up.

**Score: `3E 4J 3J 4E 3E 2E 4J 3J 2E 4J 3J 3J 3J 4J 4J 4J`**
- Balanced 65.6 · Revenue-first 67.2 · Builder-first 66.0.

### 3.4 S4: Marketplace inbox, review and returns agent (Shopee, Lazada, TikTok)

**Problem.**
- Response-rate metrics gate Preferred status, campaigns and penalties [8][9][10].
- Reviews and return disputes need timely replies.

**Who has it.** 2M+ TikTok Shop sellers [3] plus Shopee and Lazada sellers [1].

**Frequency.** Daily.

**Evidence of payment.** BigSeller paid plans from about ₱1,650/mo [11]; Zaapi $79+/mo plus AI token fees [12].

**Competitors.**
- Shopee's own Chat AI Assistant [5][6][7] and TikTok's official chatbot [8].
- BigSeller auto-reply on its free plan [11], Duoke, Ginee, Zaapi [8][12].
- **C6 = 1E: the platform owners do it.**

**Workflow.** Draft replies to unanswered chats, reviews and return disputes, with the seller approving bulk sends. Scheduled sweeps before the 12-hour windows close.

**Integration.** Shopee ISV approval needs a live product with existing integrations [13]. Lazada and TikTok terms were not found.

**Distribution.** Platform app markets might exist, but they were not verified.

**What would disprove it.** It is already disproved for chat. Only returns and dispute drafting might survive, and that is unvalidated.

**Score: `4E 5E 3J 5E 3E 1E 4J 3J 2E 3J 4J 3J 3J 4J 4J 3J`**
- Balanced 67.6 · Revenue-first 70.2 · Builder-first 66.0.
- **Disqualifying 1 on C6.**

### 3.5 S5: Kasambahay employer compliance assistant

**Problem.**
- RA 10361 requires household employers to register helpers and remit SSS, PhilHealth and Pag-IBIG, plus contracts and 13th-month pay.
- Compliance is about 6% for SSS [34], and only 2.5% of helpers have written contracts [35].

**Who has it.** Employers of 1.4M kasambahay [34]. Employer count not found.

**Frequency.** Monthly.

**Evidence of payment.** None found.

**Competitors.** None found. The agencies' own portals are the only tools.

**Workflow.**
- Monthly contribution computation and reminders.
- A payslip and 13th-month draft.
- A contract template filled in Taglish.
- The employer approves before sending to the helper.

**Integration.** No agency APIs; the employer pays through the portals or GCash.

**Distribution.** Parent and household Facebook groups (J).

**Risk.** Identity and wage data. Low enforcement means low pull.

**What would disprove it.** Being able to collect without paying (weak enforcement) is already evidence against it.

**Score: `2J 2E 2J 2J 2J 4J 3J 4J 3J 3J 2J 4J 3J 4J 3J 3J`**
- Balanced 56.2 · Revenue-first 56.4 · Builder-first 58.6.

### Summary

| ID | Candidate | Balanced | Revenue-first | Builder-first | E-backed | Disqualifier |
|---|---|---|---|---|---|---|
| S1 | Seller tax desk | **76.0** | **79.0** | 75.8 | 5/16 | — |
| S4 | Marketplace inbox/review agent | 67.6 | 70.2 | 66.0 | 6/16 | **C6** |
| S3 | Live-seller mine-to-paid desk | 65.6 | 67.2 | 66.0 | 5/16 | — |
| S2 | Payment-proof checker | 63.2 | 65.2 | 63.6 | 2/16 | — |
| S5 | Kasambahay compliance | 56.2 | 56.4 | 58.6 | 1/16 | — |

**Caveats on these numbers.**
- S1's total is above B's 74.0 in [opportunity-scorecard.md](opportunity-scorecard.md), but it rests on 11 judgement scores.
- Its top-line advantage comes from C4/C5 evidence that M (Filipino freelancer admin) lacked when it was scored. That evidence is Taxumo's prices, which would also lift M.
- S1 and M share tax-advice liability. S1 is distinct because RR 16-2023 and the platforms' BIR-registration rules create a specific, dated trigger for a segment that can be named.

---

## 4. Recommendation

**Top pick: S1, the seller tax desk.**
1. **Paid demand is evidenced in the segment.** Taxumo's published prices and Juan's ₱2,000/mo floor exist, while S1 could undercut them.
2. **The pull is regulatory and recurring.** Withholding, the 20 January declaration, quarterly returns and BIR monitoring of seller platforms all repeat [29][30][31].
3. **No platform owner competes.** Shopee, Lazada, TikTok and GCash withhold tax; they do not prepare sellers' returns.
4. **No API gate.** The MVP works on exports the builder can get from his own seller accounts.
5. **Taglish explainer content is a free channel the builder can use himself,** without calls.
6. **It uses the agent core:** extraction, tool calls, durable deadline tasks, and approval before any outbound message or "filed" status.
7. **There is a SEA path.** Indonesia's PMK 37/2025 follows the same pattern [33].

**Cheapest test (no calls, about ₱0).** Over 3 weeks:
1. Publish a free ₱500k-threshold and 2307 tracker page.
2. Publish 10 Taglish short videos.
3. Make 3–5 permitted seller-group posts.
4. Ask sign-ups in writing to upload one quarter's export.
5. Offer a ₱299 "quarter prep" by GCash.

**Kill criteria:** fewer than 30 tracker sign-ups, or fewer than 5 paid preps.

Before building, check whether Taxumo or Juan already import marketplace payouts, and what the BIR says about tax-preparation software.

---

## Sources

All accessed 2026-10-18. Type: N = news; VB = vendor blog/page (vendor claims labelled in text); PR = company press release; PP = pricing page; PB = professional blog; G = government/publication; SS = seen via search summary only.

1. "PH sellers see fivefold sales jump selling online." SunStar Cebu, 2025-12-12 (Shopee impact report). https://www.sunstar.com.ph/cebu/ph-sellers-see-fivefold-sales-jump-selling-online. N on company report. M.
2. "VisMin emerges as Philippines e-commerce growth engine." The Freeman/Philstar, 2026-07-10 (Shopee report). https://www.philstar.com/the-freeman/the-freeman/cebu-sports/2026/07/10/2541174/vismin-emerges-philippines-e-commerce-growth-engine. N on company report. M.
3. "With 2 million sellers, TikTok Shop says already among 'top players' in PH." ABS-CBN, 2024-10-09. https://abs-cbn.com/business/2024/10/9/with-2-million-sellers-tiktok-shop-already-among-top-players-in-ph-official-1539. N quoting company official; SS (403). M.
4. "E-commerce in the Philippines." Cube Asia, 2026. https://cube.asia/e-commerce-in-philippines/. Third-party estimate; SS. L–M.
5. "How Shopee Sellers Can Leverage Chat AI Assistant." BigSeller, 2025-05-02. https://www.bigseller.pro/blog/articleDetails/3223/shopee-seller-chat-AI-assistant.htm. VB. M.
6. "Shopee Shop AI Assistant." Exabytes, 2025-06-23. https://www.exabytes.my/blog/shopee-shop-ai-assistant/. VB. L–M.
7. "E-commerce upgraded." Daily Tribune, 2026-04-08. https://tribune.net.ph/2026/04/07/e-commerce-upgraded. N. M.
8. "Malaysia E-commerce Chat Response Rate Complete Guide 2026." Duoke, 2026-05-14. https://www.duoke.com/en/blog/article/398-Malaysia-E-commerce-Chat-Response-Rate-Complete-Guide-2026-Lazada-Shopee-and-TikTok-Shop. VB (Malaysia, internally inconsistent). L–M.
9. "How to Become a Shopee Preferred Seller in 6 Easy Steps." BigSeller, 2024-08-15. https://www.bigseller.pro/blog/articleDetails/2165/how-to-become-a-shopee-preferred-seller-in-6-easy-steps.htm. VB. L.
10. "Shopee Philippines penalty system." Ginee, date not found. https://ginee.com/ph/insights/shopee-philippines-penalty-system. VB; SS (404 on fetch). L.
11. "Top 5 Inventory Management Systems for E-commerce Sellers in the Philippines (2026)." BigSeller, 2026-05-22. https://www.bigseller.pro/blog/articleDetails/4578/top-5-inventory-management-systems-for-ph-ecommeerce-sellers-2026-.htm. VB (own pricing). M.
12. "Pricing." Zaapi, current. https://www.zaapi.com/pricing. PP. M.
13. "Shopee API essentials." Rollout, 2024-08-11. https://rollout.com/integration-guides/shopee/api-essentials. Third-party guide. M.
14. "Shopee." Qiscus documentation, current. https://documentation.qiscus.com/app-center/shopee. Integrator docs. M.
15. "Mastering Facebook Live Orders 2026: Tips for Filipino Sellers." BigSeller, 2026-05-07. https://www.bigseller.pro/blog/articleDetails/4531/mastering-facebook-live-orders-2026-tips-for-filipino-sellers.htm. VB (stats are vendor projections). L–M.
16. "Facebook LIVE Selling: 7 Strategies." EasyStore (en-PH), 2026-07-31. https://blog.easystore.co/en-ph/facebook-live-selling-strategies. VB. M.
17. "Pricing." EasyStore Philippines, current. https://www.easystore.co/en-ph/pricing. PP. M.
18. "Guide to Live Selling on Facebook and Instagram Without Manual Work." Ulive, 2025-08-14. https://blog.ulive.me/posts/guide-to-live-selling-on-facebook-and-instagram-without-manual-work. VB. L.
19. Pancake homepage and pricing, current. https://pancake.ph/ ; https://pancake.ph/pricing. VB/PP (260,000+ businesses = vendor claim). L–M.
20. "Pricing." Botcake, current. https://botcake.io/pricing. PP. M.
21. "GCash warns public on emerging AI-generated fake receipts scams…" Mynt newsroom, 2025-07-14. https://mynt.com.ph/newsroom/gcash-warns-public-on-emerging-ai-generated-fake-receipts-scams-urges-users-to-always-check-transactions-tab. PR. H.
22. "GCash Screenshot: How to Extract the Amount and Reference Number." imagetotable.ai, 2026-07-09 (updated 2026-09-16). https://imagetotable.ai/blog/gcash-screenshot-payment-details. VB. M.
23. "How to Reduce COD Parcel Rejection and Returns." Ninja Van PH, 2024-01-24. https://blog.ninjavan.co/en-ph/how-to-reduce-cod-returns/. Courier blog (cites Statista 2022). M.
24. "Pricing." Taxumo, current (© 2026). https://www.taxumo.com/pricing. PP. H (prices).
25. Taxumo homepage, current. https://www.taxumo.com/. VB ("144,000+ users" = vendor claim). L–M for claim.
26. Juan (juan.ac) homepage, current. https://www.juan.ac/. VB ("from PHP 2,000/month"). M.
27. JuanTax homepage, current. https://juan.tax/. VB. M.
28. "Shopee Philippines: All Sellers Must Register with the BIR to Conduct Business." BigSeller, 2024-03-11 (reports Shopee notice; cites RR 16-2023 of 2023-12-27). https://bigseller.pro/blog/articleDetails/1557.htm?source=app. VB. M.
29. "Tax Policies of Shopee 2025." BigSeller, 2025-01-27. https://www.bigseller.pro/blog/articleDetails/2826/Tax-policies-of-Shopee-2025.htm. VB. M.
30. "What is the Philippine RR 16-2023 Withholding Tax for Online Sellers?" Triple i Consulting, 2025-05-05. https://www.tripleiconsulting.com/?p=6037. PB. M.
31. "BIR monitoring online sellers." SunStar Davao, 2024-01-16. https://www.sunstar.com.ph/davao/bir-monitoring-online-sellers. N. M.
32. Inquirer Plus on Shopee/Lazada withholding from 15 July 2024, date not captured. https://plus.inquirer.net/?p=109477. N; SS (403). M.
33. "Indonesia orders four e-commerce platforms to collect tax from sellers." DealStreetAsia, 2025. https://www.dealstreetasia.com/stories/indonesia-e-commerce-tax-487446. N; SS. L–M.
34. "Few kasambahay enrolled in SSS, PhilHealth." Manila Times, 2023-09-22 (2019 PSA–DOLE survey). https://manilatimes.net/2023/09/22/news/national/few-kasambahay-enrolled-in-sss-philhealth/1911124/amp. N on government survey. M.
35. "Only 35k househelpers have written contracts." Visayan Daily Star, date not found. https://visayandailystar.com/only-35k-househelpers-have-written-contracts/. N; SS. L.
36. DepEd Club MATATAG Daily Lesson Logs, 2025. https://depedclub.com/q1-week-1-dll-matatag-melc-june-2025/. Template site; SS. M.
37. "AI Tools Serve as Essential Allies in Teaching." DepEd Bataan, 2025-07. https://depedbataan.com/wp-content/uploads/2025/07/AI-TOOLS-SERVE-AS-ESSENTIAL-ALLIES-IN-TEACHING.pdf. G; SS. L–M.
38. "PNLE Reviewer PH: Nursing." Apple App Store, current. https://apps.apple.com/cg/app/id6761256752. Store listing; SS. M.
39. "PassPNLE Nursing Exam Reviewer." Apple App Store, current. https://apps.apple.com/app/id6781995075. Store listing; SS. M.
40. "Top 10 AI Startups to Watch in the Philippines in 2026." Nucamp, 2026-04-21. https://www.nucamp.co/blog/coding-bootcamp-philippines-phl-philippiness-top-10-startups-that-tech-professionals-should-watch-out-for-in-2025. Blog listicle. L.
41. "How will Filipinos use AI in 2026?" PhilSTAR Life, 2026-02. https://philstarlife.com/amp/article/491645-ai-technology-2026-predictions. N; SS. L–M.
