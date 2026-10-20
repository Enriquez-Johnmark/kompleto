# P1 kill-checks: bookkeeper document & deadline chaser

Status: **complete, 2026-10-20.** Scope: five kill-checks on P1 as defined in [ph-industries-services.md](ph-industries-services.md) (Candidate 1) and flagged in [opportunity-scorecard.md](opportunity-scorecard.md) §9–10. Verdicts: PASS (no kill), WARN (score cut), KILL. Source tags such as [K1] refer to the Sources list at the end. All sources accessed 2026-10-20 unless stated.

Evidence types: OD = official document/regulator, PP = vendor pricing page, VD = vendor docs/help centre, VC = vendor claim, N = news, TP = third party, SR = search-result excerpt only (page not fetched). Confidence: H / M / L.

## Verdict summary

**No kill.** P1 stays first under all three weight profiles, but its lead over S1 shrinks to 1.0–2.2 points, which is within the calibration noise noted in [opportunity-scorecard.md](opportunity-scorecard.md) §9.

| # | Check | Verdict | Score effect | One-line reason |
|---|---|---|---|---|
| 1 | Incumbent overlap | **WARN** | C6 4E → **3E** | Built for Bookkeepers (₱999/mo per firm, unlimited clients) already gives clients a mobile upload page for GCash screenshots, Viber photos and bank PDFs. Requests + reminders + upload links are standard in six global tools. Still unoccupied: BIR per-client calendar, received-2307 ledger, approval-gated Taglish drafts |
| 2 | BIR CAS/CBA, eTSP | **PASS** (guardrails) | none | RMO 9-2021 covers systems that generate books, accounting records or BIR-required reports, or preserve books (ESS). eTSP covers filing, payment, registration and e-invoicing services. A chaser does none of these. It crosses the line if it exports SAWT/BIR-format files, becomes the records archive, or records/issues transactions |
| 3 | Reachability | **PASS** | none (C7 stays 3E) | r/AccountingPH ~80k and r/taxPH ~161k (GummySearch, Oct 2026); PICPA 84 chapters / 34k active members; RR 4-2010 says the accredited tax-agent list is public, but the current list was not found. The size of the buyer pool (independent bookkeepers) is not found |
| 4 | Data privacy (RA 10173) | **WARN** | C11 3J → **2E** | P1 is a processor, often a sub-processor. It handles statutorily sensitive data (tax returns; TINs very likely), so it will hit the NPC registration trigger (≥ 1,000 individuals' sensitive data) early, and IRR §44 contract terms plus a sub-processor list are mandatory. Nothing disqualifying |
| 5 | Sending channels | **PASS** (design change) | none | SMS links are blocked (NTC 2023 memo; no A2P exemption found), so SMS becomes a link-free nudge. Upload links go by email (SES ~US$0.10/1k) and the bookkeeper's own Viber via copy-and-open. Semaphore needs a business-tied account |

## 1. Incumbent overlap

**Verdict: WARN. C6 Competitive room 4E → 3E.** No tool found combines all three P1 parts for PH: (a) drafted, approved requests and reminders to the bookkeeper's clients, (b) a received-2307 ledger that shows what is missing, and (c) a per-client BIR deadline calendar. But overlap is larger than [ph-industries-services.md](ph-industries-services.md) recorded. The PH bookkeeper incumbent already ships the client *upload* half, and the *chasing* half is a commodity feature in every global practice tool.

| Tool | Requests / reminders to clients | Client upload | Received-2307 tracking | Per-client BIR calendar | Published price | Evidence |
|---|---|---|---|---|---|---|
| **Built for Bookkeepers** (PH) | Not mentioned | **Yes:** "mobile upload page"; clients upload GCash screenshots, Viber photos, bank PDFs from any phone browser; duplicate detection | Not mentioned | Not mentioned | **₱999/mo per firm, unlimited client companies**, "No contracts" | [K1] VD/PP, M |
| **Juan Accounting** (ex-JuanTax; PH) | Not found | Not found | **Partial:** received 2307s can be *encoded* ("How to encode 2307 that you received"); issued 2307s can be emailed to suppliers. No missing/received tracking or reminders | Not found; forms pre-filled "when a filing period opens" | Essentials from **₱2,000/mo SRP** (Security Bank partner listing); "CPA firms can get special discounts and plans" | [K2][K3][K4] VD, M–H |
| **Taxumo** (PH) | Not found | n/a (self-filer tool) | Not found | Not found on pricing page | 8% Plan ₱2,699/qtr; Micro ₱4,995/qtr; SMB ₱6,995/qtr (quarterly billing; lower annual/2-yr rates); Retainer ₱3,000/mo + subscription. "each profile will require its own subscription" | [K5] PP, H |
| **QNE** (PH/MY) | Not found | Not found | Not found | Not found | Not found | search only, L |
| **Xero** (PH site) | Xero HQ **Ask** ("a secure way for you to request information or documents from clients") is shown on the AU site; PH availability not found. A native "Document Request Portal" is still a product idea ("Gaining support") | Via Ask (AU) | No BIR features | Tax Manager beta is AU-only (ATO) | PH site, **USD**: Lite $7, Starter $29, Standard $50, Premium $75 /mo | [K6][K7][K8] PP/VD, M–H |
| **QuickBooks Online** (PH site) | QuickBooks Online Accountant has document requests (up to 10 documents, optional due date, needs Box set-up); no built-in automatic reminder found | Yes (Shared Documents) | No BIR features found | Not found | PH pricing page loads prices dynamically: not visible | [K9][K10] VD/SR, M |
| **BilagPilot** (Norway; US/EU) | Yes, email or SMS, automatic reminders; status sent/opened/uploaded/completed | Yes, secure no-login link | No; no PH/BIR mention | No | Only "14-day free trial" stated | [K11] trade editorial on developer info, M |
| **DocChase** | Yes, automatic email reminders (SMS per a third-party directory) | Yes, "magic upload link, no login" | No | No | **US$12–49/mo** (maker's launch post) | [K12] VC, M |
| **Content Snare** | Yes, "Unlimited reminders" (email); SMS allowance 40–400/mo per plan | Yes | No | No | USD/mo annual: Basic $35, Plus $71, Pro $119, Custom $215+ (monthly $42–258+) | [K13] PP, H |
| **Financial Cents** | Yes: "Automate info & document requests from clients, with reminders and easy upload links"; auto follow-ups only on Scale/Enterprise | Yes (client portal) | No | Generic due dates only | Solo $19/mo; Team $49/user; Scale $69/user (annual) | [K14] PP, H |
| **Karbon** | Client Requests on all plans; "Automatic Client Reminders" on Business | Yes (portal) | No | Not mentioned | Team $59, Business $89 per user/mo (annual) | [K15] PP, H |
| **TaxDome** | Yes, automatic reminders (help centre) | Yes (portal) | No | US-centric | Essentials $800, Pro $1,000, Business $1,200 per user/**year** | [K16][K17] TP, M (direct pricing page returned 403) |

Also seen, not verified: Symph "FinanceOS" (PH) advertises a filing calendar with email/SMS reminders and TIN tracking at ₱2,500/mo for 5 users, on a `dev.` subdomain that refused connection. It looks like a single-business tool, not a multi-client bookkeeper tool [K18] (SR, L). ChaseDocs chases documents over WhatsApp [K12b] (SR, L).

**What this means.**
- **The upload-link half of P1 is not differentiated.** Built for Bookkeepers already gives each bookkeeper's clients a mobile upload page for exactly P1's document types, at ₱999/mo per firm with unlimited clients. Adding a "request/remind" button to it is a small step.
- **Generic chasing is commoditised abroad** (six products with request + reminder + upload link). None does BIR forms, 2307 matching by payor/quarter, or Taglish. The defensible part of P1 is the **BIR calendar + 2307 ledger + approval-gated drafts**, not the chasing itself.
- **Price anchor.** The PH incumbent charges ₱999/mo per firm for unlimited clients. P1 must price at or near this (₱1,000–1,500/mo per firm), not per client. This matches the owner-approved ₱1,500 offer, so C5 holds at 4E.
- **Integration route.** Built for Bookkeepers' upload page is a possible complement rather than a rival: P1 could chase and track while clients upload where they already do. Not tested.

Why C6 drops to 3 (mixed): the gap is evidenced, but the nearest PH incumbent owns the same buyer, the same documents and the upload flow.

## 2. BIR registration (CAS/CBA, eTSP)

**Verdict: PASS, with design guardrails. No score change.** On the primary text, a tool that only requests, receives and forwards documents and tracks deadlines is not CAS, CBA, a CAS component or an eTSP. Three specific features would bring it inside the definitions, and the spec must keep them out or plan for registration.

**Primary text: RMO No. 9-2021** (BIR PDF, header dated February 1, 2021; secondary sources say issued February 19, 2021) [K19] OD, H. §III Definition of terms:

> 1. Computerized Accounting System (CAS) – the integration of different component systems and processes to produce computer-generated books of accounts and accounting records, including system-generated receipts/invoices, reports and documents. It involves computerized processing of full accounting cycle, i.e., from the inception of the transaction, issuance of receipts/invoices up to the generation of financial reports. …
>
> 2. Computerized Books of Accounts (CBA) – a system/software/application that can generate books of accounts only, such as but not limited to General Journal, General Ledger, Sales Journal, Purchase Journal, and Inventory Book.
>
> 3. Components of CAS (or CBA) – any system/application/software adopted to generate accounting records, reports and/or documents, which includes online transaction or application/software that serves as ordering, booking, collection, receipting/invoicing and other related business transaction facilities. …
>
> 5. Electronic Storage System (ESS) – a system used by the taxpayer or accredited tax agent for preserving books of accounts and other accounting records.
>
> 14. Tax Service Provider – also known as third-party software provider, and referred to as software/supplier/developer/professionals that provides services to users/organizations related to taxes and bookkeeping requirements.

§IV Scope covers Large Taxpayers mandated to use CAS; non-large taxpayers "who opted to use" CAS, CBA or components, listed as systems that (i) generate the General Journal and subsidiary books, (ii) record "Sales, Purchases, Accounts Receivable, Accounts Payable, Inventory, and Payroll", (iii) generate subsidiary ledgers and accounting records such as receipts/invoices and vouchers, (iv) "generates reports as required by the BIR, such as … Summary List of Sales and Purchases, etc.", or (v) POS/receipting systems; and (3) "Taxpayers engaged in business who shall use an Electronic Storage System (ESS), Middleware, and Other Similar Systems."

§V.6: "Taxpayers who will avail the services of Tax Service Providers (TSPs) or third-party software provider for the use of a system/software must register such system with the BIR prior to the effectivity of their contract", with a Joint Sworn Statement executed by "both parties". §VIII supersedes RMO 29-2002 and RMO 21-2000 on these points.

**Who registers.** The duty falls on the **taxpayer-user** (here, each SME client, or the bookkeeping firm for its own books), filed at its RDO; the vendor only co-signs the Joint Sworn Statement. The Acknowledgement Certificate issues in 3 working days with no demo [K19]. So crossing the line would not need a vendor licence, but it would add a per-client BIR filing to onboarding, which would wreck self-serve adoption.

**No successor found.** No 2025–2026 issuance replacing RMO 9-2021 was found. A vendor guide updated Aug 2026 still describes the Acknowledgement Certificate regime [K20] (TP, M). Not found: a newer primary text.

**eTSP.** RMC No. 64-2021 (April 30, 2021) [K21] OD, H: the eTSPCert System under RMO 8-2019 lets TSPs "register and to have their tax return filing and/or payment solutions tested and certified". It extends the scope to "online taxpayer registration, online tax clearance processing, online application and processing of certificate authorizing registration…, and electronic invoicing / receipting, among others", with separate guidelines per service. P1 files nothing, pays nothing and issues no invoices, so eTSP does not apply.

**Where P1 would cross the line** (inference from the text above; not confirmed with BIR or counsel):
1. **Generating BIR-required reports** from the 2307 ledger, e.g. a SAWT (Summary Alphalist of Withholding Taxes) attachment or DAT file. This fits §IV.2.c.iv. Keep exports to a plain working summary for the bookkeeper's own system, not a BIR-format file.
2. **Being the place where receipts and 2307s are kept as the client's records.** This fits the ESS definition ("preserving books of accounts and other accounting records"). Design intake as transient: hand files to the bookkeeper, set a retention limit, and do not market P1 as the archive.
3. **Recording sales, purchases or AR/AP, or issuing 2307s or invoices** (§IV.2.c.ii–iii; eTSP e-invoicing). Out of P1's stated scope; it must stay out. This also means the ERP1 module ([opportunity-scorecard.md](opportunity-scorecard.md) §10) must stop at matching and must not post or generate tax forms.

## 3. Reachability

**Verdict: PASS. C7 stays 3E.** Free written channels exist and are larger than the earlier snapshots showed. But the big ones skew towards students, job-seekers and self-filers, not bookkeeping-firm owners. The size of the actual buyer pool (independent bookkeepers and small firms) is **not found**.

| Channel | Size | Date | Fit for P1 buyers | Evidence |
|---|---|---|---|---|
| r/AccountingPH | **~80k members**, +20k (33%) in a year | GummySearch "Last updated" 2026-10-11 | Mixed: top themes are job-hunting and CPA board exams (earlier snapshot). Practising bookkeepers are a minority of unknown size | [K22] TP, L–M. Reddit itself blocked our fetcher. A search excerpt says GummySearch closed on 2025-11-30, which conflicts with the Oct 2026 date; earlier snapshots showed 37k–73k [A24] |
| r/taxPH | **~161k members**, +19k (13.7%) in a year | GummySearch, 2026-10-02 | Taxpayers and freelancers asking BIR questions; bookkeepers answer there. Closer to S1's buyer than P1's, but a place to be seen answering | [K23] TP, L–M |
| FB groups named by QuickBooks PH: "Philippine Accounting, Tax and Audit Professionals (PATAP)", "Philippine Accounting Updates", "BIR Online Filing Assistance and Tax Updates", "Taxation and Accounting in the Philippines", "BIR Tax Help and Tips Philippines" | **Member counts not found** (Facebook not readable by our tools) | Article updated 2026-01-19 | Practitioner-oriented by name | [K24] vendor content, M |
| FB page "Freelance Accountants & Bookkeepers Philippines" | ~112k followers (earlier pass) | 2026-10 | Training business aimed at offshore clients; not a peer group | [A25] |
| PICPA | 200,000+ CPAs (cumulative), **34,000+ active members, 84 local chapters, 19 international** | Homepage, accessed 2026-10-20 | Chapter events and newsletters are association-gated; no member-firm directory on the homepage. National convention 18–21 Nov 2026 | [K25] association claim, M |
| BIR accredited tax agents list | RR 4-2010 requires a Master List of accredited tax agents/practitioners kept by the RNAB, "posted on the BIR website and/or published in a newspaper". **Current list and URL not found** (bir.gov.ph is a JavaScript app our fetcher could not read). A 2022 column reportedly cites 5,358 entries as of Feb 2022 | 2010 rule; 2022 figure | Best named-buyer list if it is public: each entry is a practitioner who files for clients. Use only for ≤ 15–20 personalised written messages a week, within the approved outreach method | [K26] law summary, M; [K27] SR only, L |
| OnlineJobs.ph bookkeepers | Profile count **not found**. Sample profiles and posts are for offshore (US) clients at ~US$4–6/hr | 2026 | Poor: these bookkeepers serve foreign clients and do not deal with the BIR | [K28] SR, L |
| BIR 2026 digital tax calendar | Free, generic (not per client) | RMC 110-2025, Jan 2026 | Not a channel, but a free substitute for the calendar part; P1's per-client calendar must clearly beat it | [K29] law-firm news, M |

**How many independent bookkeepers and small firms?** Not found. The best official figures are still [A3][A4]: 607 accounting, bookkeeping, auditing and tax-consultancy establishments (PSA 2018 CPBI), and 30,355 employed in 2019 and 17,914 in 2020 (PSA ASPBI), as recorded in [ph-industries-services.md](ph-industries-services.md). These count formal establishments and miss freelancers. The accredited-tax-agent list (~5,000 in 2022, unverified) is the nearest proxy for "people who file for others".

**Implication.** Reach exists, but conversion depends on content that bookkeepers search for (2307 tracker, per-client deadline sheet) more than on group posts. The Phase 5 "≥ 3 pilots in ~4 weeks" bar is the real test of C7.

## 4. Data privacy (RA 10173)

**Verdict: WARN. C11 Regulatory/privacy risk 3J → 2E.** Nothing disqualifies P1. But it processes data the Act names as *sensitive* (tax returns, and most likely TINs). It also sits at the end of a controller → processor → sub-processor chain, and the earlier 3J understated that. The new score matches S1 (2J) and HR2 (2E), which handle similar data.

**Role: P1 is a personal information processor (PIP), and often a sub-processor.**
- RA 10173 §3(i): a PIP is a person "to whom a personal information controller may outsource the processing of personal data" [K30] (statute via Lawphil, H). P1 stores, organises and extracts data on the bookkeeper's documented instructions, so it is a PIP.
- The chain usually has three links. The **SME client** is the controller of its own records, which hold its customers' and suppliers' names, TINs and amounts. The **bookkeeper** processes them for the SME. **P1** therefore processes for the bookkeeper, which makes it a sub-processor of the SME's data. P1 is also a PIP for the bookkeeper's own data (client contacts used for reminders), where the bookkeeper is the controller. *(Inference from the definitions; confidence M.)*
- IRR §3 defines data sharing to exclude "outsourcing, or the disclosure or transfer of personal data by a personal information controller to a personal information processor" [K31] (IRR via ChanRobles, M–H). **The Data Sharing Agreement rules (NPC Circular 2020-03) therefore do not apply to P1's core flow.** The outsourcing rules do.

**Sensitive personal information.** RA 10173 §3(l)(3) covers information "issued by government agencies peculiar to an individual", including "social security numbers, … licenses … and tax returns" [K30]. Tax returns are named. A TIN is a BIR-issued identifier peculiar to an individual, so it very likely qualifies. An NPC advisory opinion (AO 2025-015, per a search summary) calls government-issued identifiers expressly sensitive [K32] (SR, L–M). No opinion naming the TIN specifically was found. Bank statements are financial data and not listed as sensitive, but they are confidential.

**What the outsourcing rules require of P1** (IRR §44, contract between controller and processor) [K31]. The contract (P1's terms + a data processing addendum) must bind P1 to:
- process "only upon the documented instructions" of the controller, including any cross-border transfer;
- impose confidentiality on staff and "implement appropriate security measures";
- "Not engage another processor without prior instruction". P1's own processors are the LLM API, the email/SMS provider and hosting. **List them in the addendum and get the bookkeeper's prior authorisation.** The bookkeeper in turn needs the SME's authorisation to use P1 (a clause in the bookkeeper's engagement letter, which P1 can supply as a template);
- assist with data-subject requests and compliance; "delete or return all personal data" at the end of service; allow audits; flag unlawful instructions.

NPC does not pre-approve outsourcing agreements but can request them during a compliance check [K33] (law-firm summary, M). IRR §45: the PIP must also comply with the Act directly [K31].

**Registration (NPC Circular 2022-04, in force 11 Jan 2023).** Controllers *and processors* must register their data processing system and DPO if any of these holds: ≥ 250 employees; processing sensitive personal information of **≥ 1,000 individuals**; or processing likely to pose a risk to data subjects' rights and freedoms. Some summaries also say systems involving **automated decision-making or profiling** must always register. Those not covered file a sworn declaration of exemption [K34][K35] (law-firm summaries, M; the NPC PDF was blocked by a bot check, so the exact text is not verified). The older IRR §47 used 250 persons / not occasional / ≥ 1,000 individuals' SPI [K31].
- **P1 will cross 1,000 individuals quickly.** About 20 bookkeepers × 30 SMEs × a handful of individual payors/payees per 2307 batch is already more than 1,000 people. **Plan to register with the NPC and appoint a DPO (the founder) once beyond the pilots.** Registration fee: not found.
- **LLM extraction is not "automated decision-making"** about individuals as long as a human approves every send and every low-confidence match. Keep it that way; that is also the approval-gate design.

**Disqualifying?** No.
- Cross-border processing is allowed. The controller stays accountable "whether domestically or internationally" (§21), so the LLM and hosting providers need contractual terms, no training on inputs (already an owner rule), and a region choice that is documented.
- **Breach notification**: the controller must notify the NPC within 72 hours (NPC Circular 16-03). As a processor, P1 must tell the bookkeeper immediately. *(Recalled from the circular; not re-fetched in this pass, L–M.)*

**Spec implications:** DPA-grade terms + processing addendum with a sub-processor list; data retention limits (which also helps with the BIR ESS line in §2); encryption at rest; per-firm data isolation; deletion on exit; an access log. This is about 1–2 weeks of non-feature work before paid launch. It is not needed for the 4-week demo slice or the concierge pilots, so C12 stays 4J.

## 5. Sending channels

**Verdict: PASS, with a design change. No score change.** The NTC link rule does break "upload link in SMS". It does not break the upload-link flow, because the link can travel by email and by the bookkeeper's own Viber/Messenger. SMS becomes a link-free nudge.

| Channel | Cost | PH availability | Link allowed? | Role in P1 | Evidence |
|---|---|---|---|---|---|
| **Amazon SES** (own domain) | **US$0.10 per 1,000** outbound; attachments $0.12/GB; new-account credits up to $200 for 6 months | Global AWS account, card billing; Singapore region (ap-southeast-1) for sending. No PH restriction found | Yes | **Default sender** for request emails with the upload link. ~3,600 emails/mo (20 firms × 30 clients × 6) ≈ US$0.36 | [K36] PP, H; [INT] §5 |
| **Postmark** | Free dev tier 100/mo; Basic **$15/mo**, Pro $16.50, Platform $18 for 10k; overage $1.20–1.80/1k; "no tier between 100 and 10,000" | Card billing; no PH restriction found | Yes | Alternative if SES deliverability set-up is slow; inbound parsing included on Pro [INT] | [K37] PP, H |
| **Resend** | Free **3,000/mo, capped at 100/day**, 3 domains; Pro $20/mo for 50k; overage $0.90/1k | "all major credit cards"; no country restriction stated | Yes | Fine for pilots. The **100/day cap breaks during a deadline crunch** (one firm × 60 clients × 2 contacts), so it is not for production | [K38] PP, H |
| **Semaphore SMS** | **₱0.56 per credit** (FAQ: "inclusive of VAT"); 1 credit = 160 chars, long messages billed per 153-char segment; first sender name free, more cost 200 credits/mo | PH aggregator. Sender names approved in up to 5 business days; **rejected "if the account isn't tied to a business"** | **Not reliably.** See below | Link-free nudge only: "Ma'am Liza, kulang pa po ang Sept bank statement. Paki-check po ang email ni [bookkeeper]." | [K39] VD, H; [INT] §8 |
| **Viber** (bookkeeper's own account, "copy & open") | Free | Native | Yes (it's not SMS) | **Main channel for clients who ignore email.** P1 drafts; the bookkeeper taps to open the chat with text + link pre-filled and sends it themselves | [INT] §4 |
| Viber bot / Business Messages | Bot EUR 100/mo; Business Messages PH price not found | Partner-sold | Yes | Not used | [INT] §4 |

**The NTC rule.** NTC's Sept 2022 memo ordered telcos to block malicious links. The amended memo of Sept 27, 2023 instructs telcos "to block or deactivate text messages containing clickable domains, URLs, URL shortening services links, Smart Links, and QR Codes" [K40] (news, M; NTC text not reached). A 2025 vendor guide says Globe "Blocks all SMS messages containing any clickable URL links" and Smart blocks shorteners [K41] (TP, L–M). **No A2P or registered-sender exemption was found** [INT] §8. Because phones auto-link bare domains, even "type kulang.ph/AB12" text is at risk. **Treat any link in SMS as undeliverable.**

**Budget check against the ₱600/mo pilot cap.**
- 5 pilot firms × 30 clients × 4 SMS/mo = 600 SMS ≈ **₱336**. That fits, with email at ~₱0.
- At 20 paying firms the same rate is ₱1,344/mo in SMS. So SMS must either be capped per firm or passed through in the price. At ₱1,500/firm/month revenue, that's affordable but must be designed in.

**Owner action implied.** Semaphore wants the account tied to a business. Whether the owner has a DTI/SEC registration is not recorded in the repo. Without one, pilots run on email + Viber copy-and-open only, which is enough to test the loop.

## Updated score string

Proposed (a recommendation; `scoring/score.py` is not edited):

```
P1  4E 4E 4J 4E 4E 3E 3E 4J 4E 4J 2E 4J 4J 5E 4J 4J
```

| | Balanced | Revenue-first | Builder-first |
|---|---|---|---|
| P1 before | 78.6 | 78.8 | 79.0 |
| **P1 after** | **76.0** | **76.4** | **76.8** |
| S1 (unchanged) | 74.4 | 75.4 | 74.6 |
| Next best | HR2 / B2 72.6 | C 73.2 | HR2 74.4 |

Computed with the weights in [scoring-method.md](scoring-method.md) against the 55 candidates in `scoring/score.py` (script run 2026-10-20). Judgement share drops from 8/16 to 7/16.

**Changes:**
- **C6 4E → 3E.** §1: the PH incumbent owns the same buyer, documents and upload flow at ₱999/firm; generic chasing is commoditised.
- **C11 3J → 2E.** §4: tax returns are statutory sensitive personal information, registration will be triggered, and there is a sub-processor chain. This aligns P1 with S1 (2J) and HR2 (2E).

**Considered and left unchanged:**
- **C5 4E.** The ₱999/firm anchor caps the price, but the published PH price is still above the cost floor.
- **C7 3E.** Channels are bigger than earlier snapshots but not buyer-specific.
- **C9 4E.** Email/SES and Semaphore have no gated review; Semaphore needs a business account.
- **C12 4J.** Privacy work is needed before paid launch, not before the demo.

**Ranking consequence.** P1 is still #1 in every profile, but the P1–S1 gap is now 1.6 / 1.0 / 2.2. Treat them as **co-leaders**, as [ph-industries-services.md](ph-industries-services.md) already suggested. The pilot results, not the scores, should break the tie.

## Unresolved questions

1. **Does Built for Bookkeepers send requests or reminders from its upload page?** Its homepage is silent. Check its help pages or ask by email, via the owner. If it does, C6 drops further, and "complement BfB" becomes the positioning.
2. **Received-2307 tracking in QNE and Juan's CPA-firm plans**: not found. Check the QNE help centre, or ask Juan sales in writing.
3. **Current BIR accredited-tax-agent list**: URL and size not found; bir.gov.ph did not render for our fetcher. Try it in a real browser.
4. **Real r/AccountingPH count**: Reddit blocked direct reads. GummySearch's Oct 2026 date conflicts with a report that it shut in Nov 2025. Check it by hand on reddit.com, and check the subreddit's self-promotion rules.
5. **FB group member counts** for the five named groups: not visible to our tools.
6. **NPC Circular 2022-04 exact Section 5 text** (incl. whether automated decision-making always triggers registration) and any registration fee: the NPC site was behind a bot check.
7. **Whether the TIN alone is sensitive personal information**: no NPC opinion naming it was found.
8. **A2P link exemption** for registered sender IDs: not found. Ask Semaphore support in writing before relying on any SMS link.
9. **Does the owner have a DTI/SEC business registration?** It is needed for the Semaphore sender name (and PayMongo, per [integration-constraints.md](integration-constraints.md)).
10. **Size of the buyer pool** (independent bookkeepers and small firms doing BIR work for SMEs): not found.

## Sources

All accessed 2026-10-20. Format: tag, publisher, title, URL, date, evidence type, confidence. "SR" means the claim comes from a search-result summary, not a fetched page.

**Incumbents**
- [K1] Built for Bookkeepers, homepage (features and pricing), https://builtforbookkeepers.com/, (c)2026, VD/PP + vendor claims, M
- [K2] Juan, "FAQs on moving JuanTax Plus to Juan Accounting", https://help.juan.tax/en/articles/482-faqs-on-moving-juantax-plus-to-juan-accounting, 2026, VD, H
- [K3] Juan, "Managing 2307s", https://help.juan.tax/en/articles/401-managing-2307s, n.d., VD, H
- [K4] Security Bank, Juan Accounting partner listing (Essentials from PHP 2,000/mo SRP), https://www.securitybank.com/?p=4681859, n.d., partner listing, SR, M
- [K5] Taxumo, Pricing, https://www.taxumo.com/pricing/, n.d., PP, H
- [K6] Xero (AU), "Xero HQ", https://www.xero.com/au/xero-hq, n.d., VD, M (AU page; PH availability not found)
- [K7] Xero Product Ideas, "Xero – Document Request Portal", https://productideas.xero.com/forums/967130-contacts-documents/suggestions/51551126-xero-document-request-portal, n.d., vendor forum, SR, M
- [K8] Xero (PH), Pricing plans, https://www.xero.com/ph/pricing-plans/, n.d., PP, H
- [K9] Dummies, "How to request documents from QuickBooks Online clients", https://dummies.com/article/how-to-request-documents-from-quickbooks-online-clients-138373, n.d., third-party guide, SR, M
- [K10] Intuit QuickBooks (PH), Pricing, https://quickbooks.intuit.com/ph/pricing/, n.d., PP (prices load dynamically; not visible), L
- [K11] Insightful Accountant, "App Aware: BilagPilot", https://blog.insightfulaccountant.com/app-aware-bilagpilot, 2026-09-08, trade editorial on developer info, M
- [K12] Hunted.space, "DocChase" (maker's launch post), https://hunted.space/product/docchase, n.d., VC, M
- [K12b] Google Workspace Marketplace, "ChaseDocs", https://workspace.google.com/marketplace/app/chasedocs/764317794188, n.d., VC, SR, L
- [K13] Content Snare, Pricing, https://contentsnare.com/pricing/, n.d., PP, H
- [K14] Financial Cents, Pricing, https://financial-cents.com/pricing/, n.d., PP, H
- [K15] Karbon, Pricing, https://karbonhq.com/pricing/, n.d., PP, H
- [K16] Uku, "TaxDome pricing 2026" (competitor article) and Capterra TaxDome pricing, https://getuku.com/articles/taxdome-pricing/ ; https://www.capterra.com/p/186749/TaxDome/pricing/, 2026, TP, SR, M (TaxDome's own pricing page returned 403)
- [K17] TaxDome Help, "Automatic reminders", https://da.help.taxdome.com/article/119-automatic-reminders, n.d., VD, SR, M
- [K18] Symph, "FinanceOS" (dev site), https://finance-operating-system.dev.apps.symph.co/, n.d., VC, SR (fetch refused), L

**BIR**
- [K19] BIR, Revenue Memorandum Order No. 9-2021, "Prescribing the Simplified Guidelines and Procedures on the Use of CAS, CBA, and/or its Components, including ESS, Middleware and Other Similar Systems", https://bir-cdn.bir.gov.ph/local/pdf/RMO%20No.%209-2021.pdf, header dated 2021-02-01 (secondary sources: issued 2021-02-19), OD (primary text read), H
- [K20] ClearTax PH, "BIR CAS registration Philippines", https://www.cleartax.com/ph/bir-cas-registration, updated 2026-08 (per search summary), vendor guide, SR, M
- [K21] BIR, Revenue Memorandum Circular No. 64-2021, "Expanding the Electronic Tax Software Provider Certification System", https://bir-cdn.bir.gov.ph/local/pdf/RMC%20No.%2064-2021.pdf, 2021-04-30, OD (primary text read), H

**Reachability**
- [K22] GummySearch, r/AccountingPH, https://gummysearch.com/r/AccountingPH, "Last updated" 2026-10-11, TP, L–M (conflicting report that GummySearch closed 2025-11-30)
- [K23] GummySearch, r/taxPH, https://gummysearch.com/r/taxPH, "Last updated" 2026-10-02, TP, L–M
- [K24] QuickBooks PH, "Accounting websites every pro should be reading", https://quickbooks.intuit.com/ph/resources/accountants/accounting-websites-every-pro-should-be-reading/, published 2023-05-24, updated 2026-01-19 (metadata), vendor content, M
- [K25] PICPA, homepage stats, https://picpa.com.ph/, n.d., association claim, M
- [K26] Lawyerly / jur.ph, BIR Revenue Regulations No. 4-2010 (amending RR 11-2006 §9 on the RNAB Master List), https://dev.lawyerly.ph/laws/50808 ; https://jur.ph/law/summary/amending-revenue-regulations-tax-practitioners-accreditation, 2010-02-24, legal database, SR, M
- [K27] The Manila Times, "Revisiting BIR Revenue Regulation 11-2006" (5,358 accredited as of Feb 2022), https://digitaledition.manilatimes.net/article/281822877658259, 2022, opinion column, SR only (page body not retrievable), L
- [K28] OnlineJobs.ph, sample bookkeeper profile and job posts, https://www.onlinejobs.ph/jobseekers/info/3881364, 2026, listings, SR, L
- [K29] Global Law Experts, "BIR launches first-ever interactive digital tax calendar for 2026" (RMC 110-2025), https://globallawexperts.com/bir-launches-first-ever-interactive-digital-tax-calendar-for-2026/, 2026-01-20 (metadata), law-firm news, M

**Data privacy**
- [K30] Lawphil, Republic Act No. 10173 (Data Privacy Act of 2012), https://lawphil.net/statutes/repacts/ra2012/ra_10173_2012.html, 2012, statute (official text mirror), H
- [K31] ChanRobles, Implementing Rules and Regulations of RA 10173, https://chanrobles.com/laws/republicacts/102_republicacts.php?id=10712, 2016, regulation (mirror; NPC PDF blocked), M–H
- [K32] SyCipLaw, "TMT Quick Notes – Data Privacy, January 2026" (cites NPC AO 2025-015 on government-issued identifiers), https://syciplaw.com/syciplaw-tmt-quick-notes-data-privacy-january-2026/, 2026-01, law-firm note, SR, L–M
- [K33] PNL Law, "Outsourcing and Subcontracting Agreements (Rule X): Data Privacy Act", https://pnl-law.com/blog/?p=6132, n.d., law-firm blog, SR, M
- [K34] Baker McKenzie (via Global Compliance News), "Philippines: new NPC circular on registration of DPOs and data processing systems takes effect", https://www.globalcompliancenews.com/2023/01/14/https-insightplus-bakermckenzie-com-bm-technology-media-telecommunications_1-philippines-new-npc-circular-on-registration-of-data-protection-officers-and-data-processing-systems-takes-effect_0111202/, 2023-01-14, law firm, SR, M
- [K35] Cruz Marcelo & Tenefrancia, NPC Circular 2022-04 summary, https://cruzmarcelo.com/?p=19068, 2022–23, law firm, SR, M. Primary text (not reachable: bot check): https://privacy.gov.ph/wp-content/uploads/2023/05/Circular-2022-04.pdf

**Channels**
- [K36] AWS, "Amazon SES pricing", https://aws.amazon.com/ses/pricing/, n.d., PP, H
- [K37] Postmark, Pricing, https://postmarkapp.com/pricing, n.d., PP, H
- [K38] Resend, Pricing, https://resend.com/pricing, n.d., PP, H
- [K39] Semaphore, FAQ, https://semaphore.co/faq, n.d., VD, H
- [K40] Globe Telecom press release "Globe lauds NTC memo requiring telcos to block SMS with clickable links" (403 to fetcher; quote via search excerpt) and BusinessWorld "NTC orders telcos to deactivate, block links in scam text messages", https://www.globe.com.ph/about-us/newsroom/corporate/globe-lauds-ntc-memo-requiring-telcos-to-block-sms-with-clickable-links ; https://www.bworldonline.com/?p=474395, 2022–2023, company release / news, SR, M
- [K41] Sent.dm, "Philippines SMS compliance guidance", https://sent.dm/resources/sms-compliance/ph-sms-guidance, c.2025, vendor guide, TP, L–M

**Internal and earlier tags**
- [INT] [integration-constraints.md](integration-constraints.md): §4 Viber (bot EUR 100/mo), §5 Email (SES, Postmark), §8 SMS (Semaphore ₱0.56; NTC link blocking; A2P exemption not found)
- [A3][A4][A24][A25] as listed in [ph-industries-services.md](ph-industries-services.md)
