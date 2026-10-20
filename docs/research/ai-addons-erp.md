# AI agents & automation as add-ons to ERP / accounting / inventory software

Status: **research, 2026-10-20.** Phase 5 input answering the owner's question "an AI agent for ERP… what would we build?" for the **ERP / accounting / inventory / operations** slice (CRM, HR and SaaS tools are covered elsewhere). Nothing here is an owner-approved decision. It does not rewrite earlier files.
Scoring: [scoring-method.md](scoring-method.md) (C1–C16, 5 = favourable, E/J tags, J capped at 4). The same weights reproduce P1 = 78.6 and C = 71.8, so the totals below are comparable with [opportunity-scorecard.md](opportunity-scorecard.md) §9.
Builds on: [self-serve-distribution.md](self-serve-distribution.md) (Xero/QBO/Zoho store facts, not redone here).

**Labels.** Evidence type: **P** primary (official docs, regulator, marketplace listing), **V** vendor claim, **T** third party (advisory firm, press, aggregator), **F** forum. Confidence H/M/L. All sources accessed 2026-10-20.

**Method limits.** Reddit could not be fetched. The PwC PH page returned 403 and was read only as a search extract. Odoo App Store figures are public purchase and download counters, not revenue. Three early research threads stopped at a usage limit. Their saved notes are used here, and a later thread covered Business Central, NetSuite and SAP B1. What none of them found is listed under Unresolved questions.

---

## Key conclusions

1. **The platforms already ship the obvious ERP agents, mostly free or bundled.** The table in §Built-in AI covers each one. Odoo 19 has AI agents, AI fields, "Ask AI" search, bill OCR at €0.10–0.15 per document, and PO matching on imported bills [7][8][9]. Business Central lists the Payables Agent, Sales Order Agent, bank-reconciliation assist and Copilot mapping of e-documents to PO lines **for all BC countries, Philippines included** [22]. Xero JAX does bank reconciliation and document extraction, and has announced payment follow-ups, document chasing and bill review [35]. Intuit runs Accounting and Payments agents [37]. Zoho Autoscan turns bills into drafts, matches them to POs, and since May 2026 its scans are free within plan quotas [38]. **Generic AP capture, "ask your ERP", collections and PO follow-up all score C6 ≤ 2.**
2. **Marketplace economics vary widely, and none suits a PH solo developer on ₱600/month.**
   - **Odoo** takes 30%, sells each module version once (one-off), and reviews no module at listing. Its only recurring path is IAP credits, at a 25% cut [3].
   - **Frappe Cloud** keeps the first US$500, then splits 80/20 in the developer's favour. It bills monthly and pays non-India developers by PayPal. Apps must be open source [17][18][19].
   - **AppSource for Business Central** charges a 3% fee on per-user monthly or annual subscriptions, the best terms of the group, but the PH Business Central market is served by partner localisation and is small [21][25][26].
   - **Xero** needs the AUD 245/mo tier, and **QuickBooks** needs US$300/mo [56]. **NetSuite's** SDN is US$3,000/yr (undated page) [28].
3. **PH demand for ERP add-ons is tiny on the evidence.** The only BIR apps on the Odoo store have **3 and 1 purchases** in total [4]. The Odoo partner directory shows 39 PH partners, against 66 in Indonesia [5]. No independent survey of PH ERP or accounting-software usage was found, only vendor lists [53]. An ERP-marketplace add-on would therefore be a **global** product, where the owner has no edge and the platform owners compete.
4. **The one evidenced PH gap is BIR-specific: receivable-side 2307 collection and matching.**
   - The CTA has disallowed creditable withholding where 2307s carried a wrong TIN or a period outside the taxable year [49].
   - RMC 14-2025 checks a claimant's SAWT against the payor's alphalist [50].
   - Odoo covers only the issuing side [1][2]. No dedicated tool was found [52].
   - **It overlaps P1 almost completely.** It works better as a P1 module that also reads the ERP than as its own product.
5. **The EIS e-invoicing mandate (31 Dec 2026) is real but a poor fit.**
   - **The pull is real.** It covers CAS/CBA users and small-and-up e-commerce taxpayers [39]. No Odoo, ERPNext, Xero or QBO EIS connector was found [44].
   - **The obstacles are larger.** ESP rules have not been issued, an ESP must be PH-licensed with local accountable officers, and each taxpayer needs its own PTI and certification [42]. Failing certification is a ground to revoke the PTI [42]. The work is deterministic plumbing with high liability (ERP3 59.8).

**Bottom line:** no ERP candidate beats P1 (78.6). The best one, ERP1 (71.6), ties C and is mostly a subset of P1. **Recommendation (for the owner to decide): do not open an ERP-marketplace track now.** Add 2307 receivable matching to P1's spec as a module, with optional read-only import from Xero, QBO or Odoo.

---

## ERP usage in PH

| System | PH evidence | Type, confidence |
|---|---|---|
| **Odoo** | 39 partners in PH (Gold 4, Silver 8, Ready 27), vs Indonesia 66, Thailand 44, Vietnam 34, Malaysia 24, Singapore 21; 4,490 worldwide. Top PH partners by declared references: Odev 75, Toolkit 61, Top C 51, Agilis 42, Something Somewhere 33; page-1 sum 453 [5]. The PH localisation exists (below) [1][2]. | P, H for the counts; references are partner-declared, not user counts |
| **ERPNext / Frappe** | PH implementers and user counts: **not found** | — |
| **SAP Business One** | Partner count: **not found**. Named PH partners in directories: ANSI Information Systems (Gold), APPTechnology Experts, Ark One, N-PAX, Direc Business Technologies, 1028 Business [34] | T, L–M |
| **NetSuite** | Named PH Solution Providers: ITG (since 2010), CloudTech, CloudWare, Octal, Phoenix Global, Mustard Seed, Gur Lavi (2023), JCurve PH and others. Customer count **not found** [32] | P (2013 PR, old) / T, M |
| **Dynamics 365 BC** | Philippines is an available BC Online country, localised **by partners** on the W1 base app, with no Microsoft PH localisation [21] | P, H |
| **Xero / QuickBooks / Zoho Books** | Xero PH Lite at US$7/mo since 20 May 2026, capped at 5 invoices; Xero cites 11% digital-accounting adoption among Filipino MSMEs [56]. Apideck and HashMicro call QuickBooks, Xero and Zoho Books "popular" in PH [53] | V, L |
| **PH-local / regional** | HashMicro PH's own "12 best accounting software 2026" lists QuickBooks, Xero, QNE (described as BIR CAS-ready), JuanTax, Sage 50, Zoho Books and Excel, and says Xero lacks built-in BIR integration [53]. AutoCount (Malaysia) reports BIR accreditation for its POS and offers "CAS-ready" accounting [53] | V, L–M |

**Reading:** PH SMEs and mid-market firms spread across QuickBooks/Xero/Zoho at the low end, QNE/AutoCount/HashMicro and local systems in the middle, and SAP B1/NetSuite/Odoo through partners above that. **No source gives market shares.** The Odoo store's PH apps sell almost nothing (§Marketplaces), which is consistent with PH ERP buyers using partners and not app stores.

---

## Marketplaces

Xero, QBO and Zoho basics are in [self-serve-distribution.md](self-serve-distribution.md) [56]. Only additions are shown for them.

| Marketplace | Listing fee | Revenue share | Recurring billing? | Review | PH developer eligible / payout | Organic-install evidence |
|---|---|---|---|---|---|---|
| **Odoo Apps Store** | none stated | Odoo keeps **30%** of app sales and **25%** of IAP credit sales [3] | **No.** One-off price, and "every version of the module is sold separately" (v13+) [3]. Recurring only via IAP credits or the developer's own off-store credits (Bista's model [6]) | Odoo does "not currently review every module published"; the developer must give support, keep the lowest price on the web, and give Odoo repo access [3] | Payout once ≥ EUR 400 by invoice/transfer; it "may take several days or weeks, especially if you are outside of Europe"; PayPal not mentioned [3]. PH eligibility not stated | **Yes, public counters.** Paid AI apps: Dashboard Ninja with AI US$582, 3,134 purchases; Claude Connector RAG/MCP US$202, 114 (14 last month); Odoo MCP Server PRO US$314, 61; Account Invoice Digitization US$143, 86; AI Invoice OCR US$59, 8. **PH BIR apps: "Accounting PH – BIR 2306 & 2307" US$243, 3 purchases; "SLS/SLP" US$197, 1** [4] |
| **Frappe Cloud Marketplace** | none found | Frappe keeps the **first US$500**, then 80/20 to the developer (since 1 Apr 2023) [17] | **Yes**: "only monthly recurring plans", prorated [18] | Review required; app must be MIT- or GPL-compatible open source on GitHub [19] | Outside India "the only option… is Paypal", invoiced monthly [17] | Install counts not found. Self-hosters can install open-source apps free |
| **Microsoft AppSource (BC extension)** | "no fees for listing" (search extract) [26] | **3%** store fee on transactable offers [26] | **Yes**: per-user, monthly or annual, optional 1-month trial; no metered billing for BC offers [25] | Marketplace validation; duration **not found** | PH is a "Microsoft-managed" tax country for sales [27]. One payout country per seller ID; US tax forms required (search extract, M). Explicit PH-publisher eligibility **not found** | not found |
| **NetSuite SuiteApp / SDN** | SDN **US$3,000/yr** (Select) or US$10,000 (Premier, invite-only), undated UK page [28]; BFN fee not found; BFN requires SDN | not found | not found | BFN verification | not found | not found. Oracle added a "SuiteApp.AI" marketplace in Oct 2025 [29] |
| **SAP Business One (SAP Store)** | current fee **not found**; a c.2013 report says integration certification is "available for a fee" [34] | not found | not found | Certification | not found | not found |
| **Zoho Marketplace** | not found | ChannelE2E (undated) calls it "commission-free"; the older Creator marketplace let developers keep 100% [20] (T, L) | not verified | not found | not found | not found |
| **Xero / QBO** | AUD 245/mo (Plus) / US$300/mo (Silver) to list [56] | none (Xero) | developer bills directly | certification + ~3 customers (Xero) | PH allowed (Xero); unknown (QBO) [56] | slow, long-tailed [56] |

**Takeaways.**
- **Recurring revenue.** Frappe and AppSource support recurring revenue natively. Odoo's paid market is real (thousands of purchases for AI dashboards) but mostly one-off and global, with PH BIR apps barely selling.
- **Cost to list.** The cheapest channels are Odoo and Frappe (₱0 to list) and AppSource (free listing, 3% fee). The C9 penalties on gated programs apply to Xero, QBO, NetSuite and SAP.

---

## Built-in AI and gaps

| Platform | Shipped natively (as of Oct 2026) | Price / availability | Evidenced gap |
|---|---|---|---|
| **Odoo 19** | AI agents that learn from documents and act on records; AI fields; AI in server actions; "Ask AI" natural-language search; Draft with AI in chatter, templates and bulk email; livechat AI agent; bill OCR with PO-reference matching on imported bills; CRM AI win probability [7]. The default Ask-AI agent "cannot make changes to the database" [8]. Native vendor **Receipt Reminder** emails vendors before the PO receipt date to confirm (since v14; per vendor in v19) [11] | Digitisation IAP: 1 credit = 1 document, €15/100 down to €5,000/100,000 [9]. AI-layer edition and price: no official statement; third parties say Enterprise-only and no extra cost on Online [10] (T, L–M) | No EIS. Native 2307 handling covers only the **issuing** side [1][2]. No parsing of free-text vendor replies found. Community-edition users need add-ons |
| **ERPNext v16 / Frappe** | No AI, LLM or OCR feature on the v16 page [13]. Frappe staff (2026-03-31): Raven is part of the stack and its AI "will be enhanced" [14]. Raven v2 offers no-code AI agents, e.g. invoice parsing with your own OpenAI key [15]. Frappe Assistant Core: a free AGPL MCP bridge [16] | Raven free, self-hosted | **The largest native gap of the group.** But the DIY tools (Raven, the MCP bridge) are free |
| **Dynamics 365 BC** | For **all BC countries incl. PH**: Payables Agent, Sales Order Agent, Expense Agent, bank-reconciliation assist, Copilot e-document → PO-line mapping, analysis assist, autofill, summaries [22]. en-PH is not a validated language for the three agents (they run in en-US) [22]. A partner says the Payables Agent takes PDF only, caps at 100 emails / 500 invoices a day, and has **no PO matching**; Expense Agent is US-only preview [23] (T, M) | Agents bill **Copilot Credits** (agent action 5 credits, generative answer 2) [24]. Price per credit not confirmed | Payables Agent PO matching (per partner [23]), but Copilot e-doc PO-line mapping already exists [22]. Gap closing |
| **NetSuite** | NetSuite Next rolls out from 2026.2 in US/CA and other regions through 2026–27; a partner says it is included at no extra cost [30] (T, M). SuiteAgents announced Oct 2025 (pricing not found). Bill Capture native. AI Connector Service (MCP) for external LLMs [31] (P, H) | see left | Not found for the SMB price band; NetSuite sits above it |
| **SAP Business One** | No B1 AI or Joule feature found. Joule is named for S/4HANA Cloud, SuccessFactors and Ariba [33] | — | Possible gap, but **absence of evidence only (L)**, and add-ons need gated SAP certification |
| **Xero (JAX)** | Live: AI bank-rec (beta Dec 2025: AU/NZ/UK Grow, US Growing, "Standard plans in other markets and above") [36]; document extraction into the ledger; XeroForce agent builder; Claude and M365 Copilot integrations. **Announced, coming: payment follow-ups, document chasing (missing receipts), bill protection** [35] | in plan | Xero is moving into C's and P1's loops. No BIR features [53] |
| **QuickBooks (Intuit)** | Accounting, Payments (AI-drafted reminders), Finance, Customer and Project Management agents: US from 1 Jul 2025; "global availability" announced 14 Nov 2025 incl. Canada and UK [37] | in plan | PH availability not found. No BIR features |
| **Zoho Books** | Autoscan of PDF/image bills with line items, approval routing and **PO matching** [38]; receipt scans free within plan quotas since May 2026 [38]; AI custom fields on Premium+; Zia Agent Studio free to build, models prepaid or BYO key; Zoho Books MCP server [38] (T, M for the last two) | in plan | none evidenced for AP |
| **D365 Supply Chain Mgmt** (not BC, for reference) | **Supplier Communications Agent** emails vendors about POs and updates POs from vendor replies [12] | licensing not found | Shows the platform owner already sells "PO follow-up" at enterprise tier |

**Gaps that survive the evidence:**
- **(a) BIR-specific work.** Receivable 2307 matching and EIS are not covered by any global platform [1][2][44][53].
- **(b) ERPNext native AI.** It has none, but free DIY tools exist [13]–[16].
- **(c) Free-text supplier replies** (email or Viber), outside Microsoft's enterprise tier. Odoo's reminder takes only portal confirmations [11]. Bista's US$147 procurement agent does not email suppliers [6].
- **(d) SAP B1 AI.** Unverified (L).

---

## Pain points

| Pain | Who feels it | Frequency | Existing solutions and prices | Room |
|---|---|---|---|---|
| **Vendor bill data entry from photos/PDFs** | AP clerks and bookkeepers at any SME | daily | Odoo OCR €0.10–0.15/doc [9]; Zoho Autoscan free within quota [38]; BC Payables Agent (Copilot Credits) [22][24]; Xero JAX extraction [35]; NetSuite Bill Capture [30]; Odoo store OCR apps US$59–143 one-off [4]; Dext/Hubdoc (not re-priced here) | **None.** Platform-native everywhere |
| **PO ↔ bill matching** | AP and purchasing at distributors and manufacturers | daily–weekly | Odoo matches PO refs on imported bills [7]; Zoho Autoscan → PO [38]; BC Copilot e-doc → PO-line mapping [22] | Low |
| **Supplier confirmation / ETA chasing** | Purchasing at distributors and importers | daily | Odoo Receipt Reminder (free) [11]; Bista Procurement AI Agent US$146.99 + credits (no supplier emails) [6]; D365 SCM Supplier Communications Agent [12] | Narrow: free-text replies only |
| **Inventory reorder / forecasting** | Distributors and retailers | weekly | Odoo native replenishment + Bista agent [6]. PH evidence: vendor blogs and a DLSU paper saying Filipino SMEs are weaker on inventory/MRP systems (T/V, L). No PH survey found | Low, and PH demand unevidenced |
| **AR collections** | Owners and bookkeepers | weekly | See C: Solido, Trove, Chaser; Intuit Payments Agent; Xero payment follow-ups announced [35][56]; Odoo follow-up levels (native) | Low (C6 2E already) |
| **Month-end close checklist / book health** | Bookkeepers and controllers | monthly | Xero Partner Hub "month-end readiness" being expanded [35]; Numeric/FloQast at mid-market (not priced here) | Low at SMB on Xero |
| **Receivable 2307 collection and matching (PH)** | B2B sellers whose clients are withholding agents, and their bookkeepers | quarterly (2307 due by the 20th day after quarter close, or on request) [48] | Manual. Odoo SAWT lists invoices with withholding but does not chase or match [1][2]; no dedicated tool found [52]. Losses: CTA disallowed CWT for a wrong TIN or out-of-period 2307 [49]; BIR matches SAWT vs payor alphalist [50]; missing TIN can deny a refund [51] | **Yes**, but overlaps P1 |
| **BIR report generation (2307 issuing, SLSP, SAWT, QAP, 2550Q)** | Accountants | monthly/quarterly | Odoo native from master/v20 docs (2306/2307 PDF, SLS/SLP .DAT, SAWT/QAP .DAT, 2550Q) [2]; v19 exports 2307 as XLS only [1]; PH Odoo apps US$197–243 one-off with 1–3 purchases [4]; QNE/JuanTax locally [53] | Low |
| **EIS e-invoicing compliance (PH)** | CAS/CBA users, small+ e-commerce, large taxpayers [39] | every invoice, with a 31 Dec 2026 deadline | Enterprise middleware: ecosio, EDICOM, Comarch, Boomi, Appcentric for SAP [44]. No SMB ERP connector found [44] | Gap, but gated (see Regulatory) |
| **Data migration / cleanup** | Firms switching ERP | one-off | Projects: US$500–3,000 freelance, US$5k–20k small-business Odoo migration, US$60/h partner rates [54] (T/V, L–M) | One-off: fails C14 |
| **"Ask your ERP" report Q&A** | Owners and managers | daily | Odoo 19 Ask AI [7][8]; Odoo MCP apps US$95–335 (61–114 purchases) and free MCP servers (511 downloads) [4]; Xero↔Claude/Copilot [35]; NetSuite AI Connector [31]; Zoho Books MCP [38]; Frappe Assistant Core free [16] | **None**: crowded and free |

SMB time-cost context: only secondary aggregators were found. They cite Cornerstone 2023 (invoicing ~4.1 h/week, expense tracking ~4.2 h/week, US) and an unverified SCORE 2025 figure [55] (T, L). No PH figure was found.

---

## Regulatory

- **CAS/CBA.**
  - RMC 5-2021 and RMO 9-2021 replaced the Permit to Use with an **Acknowledgement Certificate** issued by the RDO within 3 working days, with no system demo [45] (T summaries of primary, H).
  - The EOPT invoicing changes (RR 7-2024, RR 11-2024) count as "major enhancements" that require CAS users to re-register. The reconfiguration deadline was 31 Dec 2024 [46] (T, M–H).
  - Under RMC 65-2025 (2 Jul 2025), CAS/CBA users need an AC before use, and **the taxpayer using the system registers, not the software vendor** [47] (T, H).
  - *Inference (M):* a tool that only reads ERP data, drafts messages and matches documents keeps no books and issues no invoices, so it should not need CAS registration. **No BIR text addressing read-only add-ons was found.**
- **EIS e-invoicing.**
  - RMC 98-2026 (22 Sep 2026) keeps the **31 Dec 2026** deadline [39][40].
  - **Covered:** (1) small, medium and large e-commerce/internet taxpayers (micro exempt); (2) LTS taxpayers; (3) EOPT Large Taxpayers; (4) **CAS/CBA users with e-invoicing "and other invoicing software"**; (5) others the CIR names [39].
  - **Process:** a Permit to Issue (PTI) first, then EIS certification within 6 months. Certification at eis-cert.bir.gov.ph is 5 mandatory tests (7 with the optional API callback), and failing it is a ground to revoke the PTI [39][42]. Data goes as JSON [39][42]. Each distinct invoicing system needs its own PTI [39]. Invoices made in Word, Excel or Sheets are invalid [39].
  - **Second group pending:** exporters, POS users and RBEs await a separate RR [41][42].
  - **ESPs:** an ESP must be **organised or licensed in the PH with local accountable officers**, and its guidelines await "a separate revenue issuance" not found as of 2026-10-07 [42]. PwC (search extract) says "no accredited ESPs available yet" [43]. ClearTax says BIR "does not accredit or certify software vendors" [41] (V, M).
  - **Penalties:** ₱1,000–50,000 per offence per a vendor [44] (V, L–M).
- **Withholding certificates.** Customers should issue 2307s by the 20th day after quarter close or on request [48]. The BIR validates claimed 2307s by matching the claimant's SAWT against the payor's 1601EQ/1604E alphalist, and e-copies or faxes are acceptable [50]. CTA case law disallows defective certificates [49][51].
- **Platform AI clauses.** Xero bars using API data to train models (inference is allowed) [56]. *Design rule:* never fine-tune on customer ledger data.
- **Data Privacy Act.** Ledger, TIN and counterparty data are financial personal information when they relate to sole proprietors. Not researched further here; see the earlier P1/S1 notes.

---

## Candidates with score strings

Scores follow [scoring-method.md](scoring-method.md). Totals come from the same weights as `scoring/score.py` (not edited); P1 and C are shown for calibration.

| ID | Candidate | Balanced | Revenue-first | Builder-first |
|---|---|---|---|---|
| P1 | *(reference)* Bookkeeper document, 2307 & BIR-deadline chaser | 78.6 | 78.8 | 79.0 |
| C | *(reference)* AR collections on Xero/QBO | 71.8 | 73.2 | 72.0 |
| **ERP1** | 2307 receivable desk (ledger-connected) | **71.6** | 70.0 | 73.6 |
| ERP2 | Supplier-reply & PO follow-up agent for Odoo | 63.4 | 61.4 | 66.4 |
| ERP4 | ERPNext AP inbox agent with BIR tagging (Frappe Cloud) | 63.0 | 60.6 | 66.4 |
| ERP3 | EIS e-invoicing bridge for SMB ERPs | 59.8 | 63.6 | 54.4 |
| ERP5 | "Ask your ERP" / MCP assistant for Odoo | 58.2 | 56.4 | 61.2 |

Disqualifying 1s: **ERP5 C6 1E** (free incumbents). ERP3 has 1s on C9 and C10. These are not in the C5/C6/C11 disqualifier set but still disqualify it in practice.

### ERP1: 2307 receivable desk
**One line:** each quarter, find every customer invoice where the customer withheld tax, request the missing 2307s from each customer's AP, read the PDFs that come back, match them to invoices, and flag defects before SAWT filing.
**Buyer:** PH B2B service firms and suppliers billing corporates and top withholding agents, and their bookkeepers. **Price anchor:** Built for Bookkeepers ₱999/mo and Juan ₱2,000/org/mo (ph-industries-services.md [A13][A14]); target ₱499–999/mo. **Competitors:** none dedicated found [52]. Odoo SAWT lists invoices without chasing [1][2]. Bookkeepers chase by hand. **P1 is the nearest neighbour.**
**Agent loop:**
1. Import invoices with EWT from Xero/QBO/Odoo (read-only) or CSV.
2. Build a per-customer "2307 owed" list each quarter.
3. Draft request emails in the business's own name. **The human approves** each one.
4. Read the 2307 PDFs that come back.
5. Match them on payee TIN, period, ATC and amount, and flag a wrong TIN, an out-of-period certificate, or a missing signature [49].
6. **The human confirms** the matches. The agent then exports a SAWT-ready list and does no filing.

`4E 3E 4J 3E 3E 4E 3J 4J 4J 4J 3J 4J 4J 3J 4J 4J`
- C1 4E: CWT credits are lost when 2307s are missing or defective [49][50][51].
- C2 3E: quarterly cycle, per customer [48].
- C3 4J: 1–2% of B2B receipts are at stake against a ₱500–1,000/mo price.
- C4 3E: people pay bookkeepers and local tools for adjacent work; no one is evidenced paying for this exact job.
- C5 3E: PH tool prices ₱999–2,000/mo.
- C6 4E: no dedicated tool found; platforms cover only the issuing side [1][2][52]. "Not found" is a search result, M.
- C7 3J: SEO on "2307", PH tax and bookkeeper FB groups; no store needed.
- C8 4J: standard BIR form plus email drafting plus matching.
- C9 4J: CSV import first; Odoo's API is open; Xero's 5-connection free tier is enough for pilots [56].
- C10 4J: every send and every match is approved.
- C11 3J: TINs and tax data (DPA).
- C12 4J: ≤ 4 weeks on CSV.
- C13 4J: quarterly bursts; cheap extraction.
- C14 3J: real but bursty quarterly use, so a churn risk between quarters.
- C15 4J: feeds P1, AR collections and later EIS checks.
- C16 4J: durable per-quarter tasks, approvals, extraction.

### ERP2: Supplier-reply & PO follow-up agent for Odoo
**One line:** read free-text supplier replies (email, later Viber) to open POs, propose ETA and quantity updates on the PO, and draft chasers for unconfirmed or late POs.
**Buyer:** Odoo-using distributors and importers (global). **Price anchor:** Bista Procurement AI Agent US$146.99 one-off + credits [6]; Odoo AI apps US$95–750 one-off [4]. **Competitors:** Odoo Receipt Reminder (free) [11], Bista [6], D365 SCM Supplier Communications Agent [12].
**Agent loop:** watch the purchasing mailbox, classify replies against open POs, and propose field changes. **The human approves** the changes and the outgoing chasers.

`3J 4J 3J 3E 3E 2E 3E 4J 4E 3J 4J 4J 3J 2E 3J 4J`
- C1 3J: late stock hurts, but it is a background annoyance.
- C2 4J: daily for distributors.
- C3 3J: hard to price.
- C4 3E: Odoo buyers buy AI apps (thousands of purchases [4]); procurement counts not shown [6].
- C5 3E: US$100–750 one-off prices [4].
- C6 2E: free native reminder, Bista, and Microsoft's agent [6][11][12].
- C7 3E: public store counters show organic purchases, but newcomers start at 0 [4].
- C8 4J.
- C9 4E: no module review; open API [3].
- C10 3J: a wrong ETA misleads planning.
- C11 4J: commercial data.
- C12 4J.
- C13 3J: per-version maintenance and a support duty [3].
- C14 2E: one-off per version; recurring only via IAP at a 25% cut [3].
- C15 3J.
- C16 4J: mailbox loop, approvals.

### ERP3: EIS e-invoicing bridge for SMB ERPs
**One line:** a connector that turns Odoo, ERPNext, Xero or QBO invoices into BIR EIS JSON, walks the taxpayer through PTI and certification tests, and transmits sales data.
**Buyer:** covered PH CAS users and e-commerce SMEs [39]. **Price anchor:** not found for SMB; enterprise middleware is quote-based [44]. **Competitors:** ecosio, EDICOM, Comarch, Boomi, Appcentric [44]; future accredited ESPs.
**Agent loop:** mostly deterministic. The agent's part is limited to validating invoice fields and explaining rejections. **The human approves** go-live and any resubmission.

`5E 4E 4J 4E 3J 3E 3J 2J 1E 1J 2E 2J 2J 4J 3J 2J`
- C1 5E: legal mandate; PTI revocation [39][42].
- C2 4E: every invoice.
- C3 4J.
- C4 4E: enterprises already buy EIS middleware [44].
- C5 3J: SMB WTP unknown.
- C6 3E: no SMB connector found, but ESPs are coming [42][44].
- C7 3J: deadline-driven search demand.
- C8 2J: specs sit behind the certification portal.
- C9 1E: ESP rules not issued, PH-licensed ESP with local officers, per-taxpayer PTI and certification [42].
- C10 1J: a transmission error is a tax violation.
- C11 2E: regulated tax data.
- C12 2J.
- C13 2J: hand-holding through certification for each taxpayer.
- C14 4J.
- C15 3J.
- C16 2J: plumbing, little agent.

### ERP4: ERPNext AP inbox agent with BIR tagging (Frappe Cloud app)
**One line:** turn emailed or photographed vendor bills into draft ERPNext purchase invoices with PH VAT/EWT ATC codes and TIN checks, matched to POs.
**Buyer:** ERPNext users (global; PH count unknown). **Price anchor:** not found for Frappe AI apps. Frappe takes the first US$500, then 20% [17]. **Competitors:** Raven v2 DIY agents [15], Frappe Assistant Core (free) [16], ECOSIRE and NextAI (prices not found).
**Agent loop:** inbox → extract → draft purchase invoice → PO match. **The human approves** each draft.

`3J 4J 3J 2J 2J 4E 2J 4J 4J 3J 3J 4J 3J 4E 3J 4J`
- C1 3J.
- C2 4J: daily.
- C3 3J.
- C4 2J: no evidence that ERPNext users pay for AI apps.
- C5 2J: open-source user base expects free.
- C6 4E: ERPNext v16 has no native AI [13][14].
- C7 2J: marketplace traffic unknown, and the PH ERPNext base is unknown.
- C8 4J.
- C9 4J: open API; review length unknown.
- C10 3J.
- C11 3J.
- C12 4J.
- C13 3J.
- C14 4E: monthly plans only [18].
- C15 3J.
- C16 4J.

### ERP5: "Ask your ERP" / MCP assistant for Odoo
**One line:** natural-language Q&A and report building over Odoo data through an MCP bridge to Claude or ChatGPT.
**Price anchor:** Odoo MCP apps US$95–335 one-off [4]. **Competitors:** Odoo 19 Ask AI [7][8], free MCP servers (511 downloads), paid MCP apps (61–114 purchases) [4], and every other platform's native MCP or LLM connector [31][35][38].
**Agent loop:** read-only queries. The human only reads the answers.

`2J 3J 2J 4E 3E 1E 3E 4J 4E 4J 3J 4J 3J 2E 3J 3J`
- C1 2J: convenience.
- C2 3J.
- C3 2J.
- C4 4E: MCP apps do sell [4].
- C5 3E: one-off US$95–335.
- C6 1E: native and free incumbents [4][7][8].
- C7 3E: store counters.
- C8 4J.
- C9 4E.
- C10 4J: read-only.
- C11 3J.
- C12 4J.
- C13 3J.
- C14 2E: one-off [3].
- C15 3J.
- C16 3J: thin agent core.

**Reading the set:** ERP1 is the only candidate near the top, and it is P1's BIR work seen from the ledger side. The marketplace-native ideas (ERP2, ERP4, ERP5) fail on C6 or C14 because the platforms ship the feature or the store sells only one-off licences. ERP3 has the strongest pull but is gated by regulation that does not yet exist.

---

## Unresolved questions

1. Do Juan/JuanTax, Built for Bookkeepers, QNE or Taxumo already track **received** 2307s against invoices? This is the kill-check for ERP1, and it is also P1's kill-risk 1.
2. How many PH firms use each ERP? No market-share source was found. A DTI/PSA ICT-use survey may exist (not searched).
3. Odoo App Store payout to a PH developer (bank or PayPal?) and whether Odoo permits subscription-style IAP for third parties at 25%.
4. Frappe Cloud Marketplace install counts and review time.
5. AppSource: explicit PH publisher/payout eligibility; BC app validation time; the per-credit price for BC agents.
6. Whether the Odoo AI layer is Enterprise-only (third parties disagree).
7. Xero JAX payment follow-ups and document chasing: launch date and regions, including PH.
8. EIS: the ESP revenue issuance; whether a PH sole proprietor can be an ESP; whether "other invoicing software" users who invoice from Xero/QBO are covered.
9. NetSuite's current SDN fee and BFN cost; SAP B1 add-on certification cost; whether SAP B1 ships any AI.
10. Whether a read-only, draft-only add-on needs any BIR registration (inferred no; no BIR text found).

---

## Sources

All accessed 2026-10-20.

1. Odoo S.A., "Philippines — fiscal localization", Odoo 19.0 docs. https://www.odoo.com/documentation/19.0/applications/finance/fiscal_localizations/philippines.html — P, H.
2. Odoo S.A., same page, master/20.0 docs. https://www.odoo.com/documentation/master/applications/finance/fiscal_localizations/philippines.html — P, H.
3. Odoo S.A., Odoo Apps FAQ (undated). https://apps.odoo.com/apps/faq — P, H.
4. Odoo Apps Store browse/search pages, sorted by purchases/downloads (searches "chatgpt", "AI", "invoice ocr", "philippines"). https://apps.odoo.com/apps/modules/browse?search=chatgpt&order=Purchases (and equivalent queries) — P listing counters, H.
5. Odoo S.A., Partners in the Philippines. https://www.odoo.com/partners/country/philippines-171 — P, H.
6. Bista Solutions, "Procurement AI Agent" listing (v16–20). https://apps.odoo.com/apps/modules/19.0/procurement_ai_agent — P listing; capabilities V; H for price.
7. Odoo S.A., Odoo 19 release notes (2025). https://www.odoo.com/odoo-19-release-notes — P/V, H.
8. Odoo S.A., AI docs 19.0. https://www.odoo.com/documentation/19.0/applications/productivity/ai.html — P, H.
9. Odoo IAP, Documents Digitization price list. https://iap.odoo.com/iap/in-app-services/259 — P, H.
10. Dasolo, "Odoo AI integration costs" (2025), https://www.dasolo.ai/blog/odoo-ai-6/odoo-ai-integration-costs-what-to-expect-196; Braincuber, https://www.braincuber.com/blog/how-small-businesses-affordably-add-ai-odoo — T, L–M.
11. Odoo 19.0 Contacts docs (Receipt Reminder), https://www.odoo.com/documentation/19.0/applications/essentials/contacts.html (via search extract, M); Odoo forum thread 191918, https://www.odoo.com/forum/help-1/odoo-14-purchase-order-receipt-date-updated-by-vendor-online-issue-191918 — F, M.
12. Microsoft Learn, "FAQ: Supplier Communications Agent" (D365 SCM). https://learn.microsoft.com/en-us/dynamics365/supply-chain/faq-supplier-communications-agent — P, H.
13. Frappe Technologies, ERPNext version 16. https://frappe.io/erpnext/version-16 — P, H.
14. Frappe forum, "Is AI integrated in ERPNext natively…" (staff reply 2026-03-31). https://discuss.frappe.io/t/is-ai-integrated-in-the-erpnext-natively-and-if-there-are-any-plans-to-do-so/153609 — F (staff), H.
15. Frappe forum, "Announcing Raven v2…" (2025). https://discuss.frappe.io/t/announcing-raven-v2-now-with-ai-agents-document-alerts-and-more/142581 — V, M.
16. Frappe Cloud Marketplace, Frappe Assistant Core. https://cloud.frappe.io/marketplace/apps/frappe_assistant_core — V, M.
17. Frappe, "Payouts" docs (updated 2026-02-16). https://docs.frappe.io/cloud/marketplace/payouts — P, H.
18. Frappe, "App Pricing Plans" docs (undated). https://docs.frappe.io/cloud/app-plan — P, M (via search extract).
19. Frappe, Marketplace docs. https://docs.frappe.io/cloud/marketplace — P, M.
20. ChannelE2E, "Zoho builds free app store…" (undated). https://channele2e.com/post/zoho-builds-free-app-store-for-small-business-isvs-developers — T, L.
21. Microsoft Learn, BC countries and translations (updated 2026-09-08). https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/compliance/apptest-countries-and-translations — P, H.
22. Microsoft Learn, BC Copilot and agents region/language availability (updated 2026-10-01). https://learn.microsoft.com/en-us/dynamics365/business-central/copilot-agents-region-language-availability — P, H.
23. Tigunia, "Complete guide to Business Central AI agents" (2026-05-29, updated 2026-06-08). https://tigunia.com/blog/complete-guide-to-business-central-ai-agents/ — T (partner), M.
24. Microsoft Learn, Copilot Studio messages/credits management, https://learn.microsoft.com/en-us/microsoft-copilot-studio/requirements-messages-management — P, M–H; demiliani.com (2026-03-24), https://demiliani.com/2026/03/24/dynamics-365-business-central-how-many-copilot-credits-my-agent-consumes/ — T, M.
25. Microsoft Learn, "Sell Business Central apps through AppSource" (updated 2026-02-26). https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/devenv-sell-apps-appsource — P, H.
26. Microsoft Learn, Marketplace commercial transaction capabilities (updated 2026-08-18). https://learn.microsoft.com/partner-center/marketplace/marketplace-commercial-transaction-capabilities-and-considerations — P, H.
27. Microsoft Learn, Marketplace tax details (2026-03-18). https://learn.microsoft.com/en-us/partner-center/marketplace-offers/tax-details-marketplace — P, H.
28. NetSuite UK, SuiteCloud Developer Network page (undated). https://netsuite.co.uk/portal/uk/developers/sdn.shtml — P (possibly stale), L–M.
29. Oracle, "NetSuite expands partner program…" (2025-10-06). https://www.oracle.com/ph/news/announcement/sw25-netsuite-expands-partner-program-accelerate-ai-innovation-2025-10-06/ — V, H that it was announced.
30. Epiq Infotech, "NetSuite Next…" (2026), https://epiqinfo.com/netsuite-next-how-ai-is-reshaping-the-erp-you-thought-you-knew/ — T, M; Futurum Group, SuiteWorld 2025, https://futurumgroup.com/insights/netsuite-accelerates-ai-adoption-with-suiteworld-2025-announcements/ — T, M.
31. Oracle NetSuite docs, AI Connector Service. https://docs.oracle.com/en/cloud/saas/netsuite/ns-online-help/article_4160616848.html — P, H.
32. NetSuite press release (2013), https://netsuite.com.au/portal/au/press/releases/nlpr09-19-13f.shtml — P (old), M; Inquirer Technology (2023), https://technology.inquirer.net/121280/gur-lavi-corp-joins-netsuite-solution-provider-program — T, M.
33. SAP News, "SAP Business AI release highlights Q1 2026" (2026-04). https://news.sap.com/2026/04/sap-business-ai-release-highlights-q1-2026/ — V, L (absence of B1 only).
34. ERP Research and Elioplus directories of SAP partners in PH: https://erpresearch.com/sap-partners/ansi-information-systems-inc, https://elioplus.com/asia-pacific/philippines/makati/channel-partners/sap — T, L–M; PCWorld (c.2013) on SAP partner certification, https://www.pcworld.com/article/453114/sap-expands-partner-program-for-application-development.html — T, L.
35. IT Brief NZ, "Xero adds AI tools to JAX for small business finance" (Xerocon 2026; date not shown). https://itbrief.co.nz/story/xero-adds-ai-tools-to-jax-for-small-business-finance — T reporting V, M.
36. eCommerceNews AU / CFOtech, "Xero rolls out AI bank reconciliation with JAX" (Dec 2025). https://ecommercenews.com.au/story/xero-rolls-out-ai-bank-reconciliation-with-jax-tool — T, M.
37. Intuit IR, "Intuit introduces virtual team of AI agents" (2025-07-01), https://investors.intuit.com/_assets/_08ee5483ec4c057568cc8774f3fd6aad/intuit/news/2025-07-01_Intuit_Introduces_Ground_Breaking_Virtual_Team_of__1258.pdf — P, H; BusinessWire (2025-11-14, Toronto), https://www.businesswire.com/news/home/20251114912265/en — P press, M.
38. Zoho, "How Zoho Books and Zia run your accounting on autopilot", https://www.zoho.com/books/academy/accounting-trends-and-insights/how-zohobooks-and-zia-run-accounting-on-autopilot.html — P/V, H; Zoho Community, "Zoho Books product updates May 2026", https://help.zoho.com/portal/en-gb/community/topic/zoho-books-product-updates-may-2026 — P, M.
39. P&A Grant Thornton, "BIR issues guidelines on electronic invoicing, retains 31 Dec 2026 deadline" (2026-09-24). https://www.grantthornton.com.ph/technical-alerts/tax-alert/2026/bir-issues-guidelines-on-electronic-invoicing-retains-31-dec-2026-deadline/ — T summary of RMC 98-2026, H.
40. Daily Tribune, "BIR sets 31 December e-invoicing deadline" (2026-09-23). https://tribune.net.ph/2026/09/23/bir-sets-31-december-e-invoicing-deadline — T news, M.
41. ClearTax PH, "e-Invoicing in Philippines 2026". https://www.cleartax.com/ph/en/philippines-e-invoicing — V, M.
42. Lookuptax, "EIS e-invoicing Philippines" (modified 2026-10-07). https://lookuptax.com/docs/einvoicing/eis-einvoicing-philippines — T explainer, M.
43. PwC PH, "E-invoicing: when compliance outpaces readiness" (Taxwise 2026; 403, search extract only). https://www.pwc.com/ph/en/tax/tax-publications/taxwise-or-otherwise/2026/e-invoicing-when-compliance-outpaces-readiness.html — T, M.
44. ecosio changelog (2026-02-18), https://hello.ecosio.com/changelog/working-ahead-of-the-philippines-2026-e-invoicing-deadline-with; BusinessWire on Boomi/Yazaki-Torres (2023-06-21), https://www.businesswire.com/news/home/20230621343850/en; Appcentric PH, https://www.appcentric.ph/blog/sap-bir-e-invoicing-eis-compliance-rise-grow-s4hana; Comarch, https://www.comarch.com/trade-and-services/data-management/legal-regulation-changes/philippines-e-invoicing-mandate-extended-new-2026-compliance-deadline/ — V, M.
45. KPMG PH, Special InTax Jan 2021, https://kpmg.com/ph/en/home/insights/2021/01/special-intax-january-2021-issue1-volume1.html; Grant Thornton PH on CAS/CBA registration — T, H.
46. P&A Grant Thornton, "EOPT on CAS: to secure or not to secure new AC" (2024-07-09). https://www.grantthornton.com.ph/insights/articles-and-updates1/lets-talk-tax/eopt-on-computerized-accounting-system-cas-to-secure-or-not-to-secure-new-acknowledgment-certificate/ — T, M–H.
47. KPMG PH, Special InTax July 2025 (RMC 65-2025). https://kpmg.com/ph/en/insights/2025/07/special-intax-july-2025-issue-1-volume-1.html — T, H.
48. "Reminder to request/demand for 2307s before it's too late". https://mmco.com.ph/?p=3561 — T (practitioner), M.
49. Service Resources, Inc. v. CIR, CTA Case No. 10158 (2nd Div., 2023 resolution), via batasnatin.com. https://batasnatin.com/laws/service-resources-inc-v-commissioner-of-internal-revenue-cta-case-no-10158 — P via legal DB, M.
50. RMC No. 14-2025 summary, batasnatin.com. https://batasnatin.com/laws/rmc-no-14-2025-clarifies-certain-issues-pertaining-to-the-mandatory-requireme — P via legal DB, M.
51. KPMG PH, "Lack of TIN in BIR Form No. 2307 may lead to the denial of CWT refund" (2017-09). https://kpmg.com/content/dam/kpmg/ph/pdf/topofmindarticles/2017/September/Sept26LackofTINinBIRForm.pdf — T, H.
52. Search for receivable-side 2307 tracking tools (2026-10-20): none found. Closest is the Payhip "BIR Tax Planning & Compliance Tracker" (US$7.99), https://payhip.com/b/AEQyd — V, M.
53. HashMicro PH, "12 best accounting software in the Philippines (2026)", https://www.hashmicro.com/ph/blog/top-accounting-software-philippines/ — V, L; AutoCount PH newsroom, https://ph.autocountsoft.com/newsroom/autocount-BIR-accredited-in-philippines.html — V, M; Apideck PH accounting integrations, https://www.apideck.com/integrations/country/philippines/accounting — V, L.
54. DEV Community, "What does Odoo migration really cost" (2025), https://dev.to/aspire-softserv/what-does-odoo-migration-really-cost-a-transparent-pricing-breakdown-for-2025-48fj; Gloriumtech Odoo migration services, https://gloriumtech.com/?p=49108 — V/T, L–M.
55. Stealth Agents research aggregator (2026) citing Cornerstone 2023 and SCORE 2025. https://stealthagents.com/research/small-business-administrative-costs-statistics-2026 — T, L.
56. Internal: [self-serve-distribution.md](self-serve-distribution.md) (Xero/QBO tiers, Xero PH Lite, collections competitors, Xero AI clause), with its own sources.
