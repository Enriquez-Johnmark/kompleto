# Philippine industries sweep: AI-agent product opportunities (sectors)

Status: **complete, 2026-10-18.** Sector sweep for a solo PH-based Rust developer (no network, no ads, ≤ ₱600/month). All sources accessed 2026-10-18. Sector-size figures marked "my calculation" apply a published share to a published total. Scoring per [scoring-method.md](scoring-method.md); earlier candidates in [opportunity-scorecard.md](opportunity-scorecard.md) §8 [1].

## Key conclusions

1. **The money-losing workflows are in healthcare, but they are closed.** PhilHealth denied 483,000 claims (₱4.7B) and returned 304,082 (₱4.2B) in 2024, mostly for clerical errors and late filing [7]. eClaims runs only on PhilHealth-certified software, HMO portals have no public APIs, and the data is health data [12]. Not buildable or sellable under these constraints. *Confidence: H on the pain, M on the verdict.*
2. **Post-sale collections is the under-served, low-liability pattern that recurs across sectors.** It shows up as contractor progress billings, school tuition and promissory notes, and coop loan amortisations. Each item is a dated obligation that someone must chase politely, which matches durable schedules plus approve-to-send. It is also distinct from the already-researched pre-sale follow-up (B). *Confidence: M.*
3. **Construction has the clearest local price signal.** A PH vendor lists progress-billing software for small contractors at ₱7,500-10,000/month with no collections or AI features [44]. Contractors also pay ₱3,000-8,000 per BOQ [26]. *Confidence: M (list prices, customer counts unknown).*
4. **Education has the clearest legal driver.** RA 11984 removed "no permit, no exam" as a collection lever but kept dated promissory notes and records-withholding [15][47]. No PH price for a reminders layer was found. *Confidence: M.*
5. **Channel gates kill several ideas.** Booking.com connectivity onboarding is reported paused [33]. Viber bots cost EUR 100/month per bot since Feb 2024 [42]. BOC lodgement goes only through accredited VASPs, and E2M is being overhauled by 2027 [23][24].
6. **Hospitality and LGU permits are not materially new.** They are vertical skins of B/S2, or yearly reminders.
7. **Ranking (Balanced):** CON1 71.8 > EDU1 71.2 > DIS1 68.8 > LOG1 67.8 > COOP1 66.4. None clearly beats S1 (74.4) from the earlier scorecard [1]. CON1 and EDU1 sit just above B (70.8), and 11-12 of their 16 scores are judgement.
8. **Top pick from this sweep: CON1 (contractor progress-billing & collections desk).** It has the strongest local price evidence, low-sensitivity commercial data, no gated integrations, and an SEO channel shown to work in this niche by Taglish contractor blogs [28]. EDU1 is a close second with better pain evidence but minors' data and seasonal buying.

## 1. Healthcare beyond clinic booking

**Size.** About 35,000 health MSMEs (my calculation: 2.81% "human health and social work" share x 1,241,733 MSMEs in the 2023 List of Establishments; share from a Statista chart of DTI data whose year label is ambiguous) [2][3]. About 53,000 licensed drugstores in 2021 out of ~67,000 licensed drug establishments (Statista citing DOH; 2025 count paywalled) [4].

**Workflows an approval-gated agent could take over.**
- *HMO LOA requests and HMO claims (clinics/doctors).* Patients or clinic staff request an LOA by email/portal/kiosk with name, birth date, member ID, doctor, facility, visit date and complaint; Maxicare's email route promises up to 24 h and the LOA is valid 30 days; tests ordered after a consult need a *separate* same-day LOA [5]. SeriousMD already tracks HMO claimable amounts per service and reports what each HMO owes [6], so the "HMO receivables" ledger is partly served inside clinic EMRs.
- *PhilHealth eClaims (small hospitals, lying-ins, dialysis, ASCs).* The pain is large and documented: in 2024 PhilHealth denied 483,000 claims (₱4.7B) and returned 304,082 claims to hospitals (RTH, ₱4.2B); officials say over a third of claims are RTH "mainly because they contain clerical errors", and DOH says most denials were filed after the 60-day deadline [7]. A 2025 circular reconsiders claims denied solely for late filing [8]. Regional example: Negros Occidental had ₱245M RTH and ₱60M denied for 2025 filings as of 31 July [9].
- *Pharmacies:* stock/expiry and FDA licence renewals (FDA moved LTO applications for outlets to eServices from 20 Jan 2025, per a market report) [10]. *Diagnostic labs:* result release to patients/HMOs.

**Payment evidence.** EMR vendors (SeriousMD, others) sell HMO tracking, but PH prices are "on request" [6][11]. No published per-claim fee for outsourced PhilHealth claims processing was found.

**Gatekeeping.** Severe. Only facilities using **PhilHealth-certified software** may run eClaims; vendors (HITPs) must re-certify for every eClaims web-service version [12][13]. HMO portals are per-HMO, closed, with no public APIs found. Every workflow handles **sensitive personal information** (health data) under the Data Privacy Act. A pre-submission "claim checker" that never touches eClaims avoids certification but still processes health records.

**Free distribution.** Doctors' and hospital billing-staff FB groups exist but sizes were not measured. **Competition:** EMRs (SeriousMD etc.) and certified HIS vendors own the integration points.

**Verdict.** Real money is lost (₱8.9B denied+RTH in one year) but the buyer is a hospital billing office reached through certified HIS vendors, the data is the most sensitive category, and an error shows up as a denied claim. Not a fit for a solo, ₱600/mo, no-sales builder. *Confidence: M.*

## 2. Education (private schools, tutorial/review centres)

**Size.** DepEd reported 47,818 public and private schools open in August 2024 and 2,634,760 private-school learners out of 23.3M (interim monitoring count; no public/private school split found) [14]. About 22,000 education MSMEs (my calculation from the 1.77% sector share) [3]. Tutorial and review-centre counts: not found.

**Workflows.** Tuition instalment reminders and statements of account; promissory-note tracking; enrollment requirement checklists (PSA birth certificate, Form 137/138, good moral); parent announcements; registrar requests for Form 137/TOR/certifications.

**Pain evidence.** Collection pressure is structural. **RA 11984 (No Permit, No Exam Prohibition Act, signed 11 March 2024)** bars basic-ed, HEI and long-course TVET schools from blocking exams of certified disadvantaged students for non-payment, while preserving promissory notes, withholding of records and other legal collection remedies [15][16]. Before it passed, PACU said a 27-college study showed fees collected would cover ~7.7 months of costs, falling to ~2 months under the ban (industry-sourced, small sample), and a CEAP survey found ~49% of 224 basic-ed schools at a loss or break-even [17]. So "polite, persistent, documented reminders + promissory-note follow-up" is a legitimate, lawful need.

**Payment evidence.** School management systems with fee billing are sold in PH (GenyoX, MIS-PH, GlobalCom DepEd Connect named by a competitor; OpenEduCat sells PHP instalment billing with GCash/Maya/PayMongo) but no peso prices were published [18]. Global tutoring tools: Tutorbase is free with a 1% invoicing fee and automated payment reminders; Teach 'n Go shows outstanding balances to parents [19]. Price points in the PH segment: not found.

**Gatekeeping.** Student/minor data is personal information; balances are not "sensitive" under the DPA but still need consent and care. LRN/LIS (DepEd's learner system) has no public API (not found). Reminders go by SMS/email/Messenger; Messenger outbound to parents is limited by Meta's 24-hour/human-agent rules (see earlier research [1]).

**Free distribution.** Associations are public and directory-based (COCOPEA, CEAP, PAPSCU, regional private-school associations); no registrar/tutorial-owner FB group sizes were found [20]. School owners are reachable, but procurement is often a board/principal decision with a June-August buying window. *Inference.*

**Competition.** Bundled SIS vendors plus spreadsheets; no Taglish AI collections layer found.

**Verdict.** A credible niche product: "tuition follow-through" for small private schools and tutorial centres (100-800 students) that sit on spreadsheets. Seasonal procurement and a slow, committee-style buyer are the risk. *Confidence: M.*

## 3. Logistics, couriers, customs brokers, trucking

**Size.** About 13,300 transport-and-storage MSMEs (my calculation from the 1.07% share) [3]. Customs brokers are PRC-licensed: 2,238 of 3,078 passed the November 2025 licensure exam (72.71%), against 1,265 of 2,871 in November 2024 [21]. The cumulative number of licensed brokers and PSA freight-forwarder counts: not found.

**Workflows.**
- *Customs brokerage:* chase the importer for commercial invoice, packing list, B/L/AWB and regulatory import permits; build the import entry; lodge through a BOC-accredited VASP into E2M; send status and duty/tax figures to the client; chase payment.
- *Trucking/forwarding:* trip booking via Viber/FB, POD (proof of delivery) collection, POD-based billing and statements. Local platforms (Pakargo.ph by Roadwise Logistics; Transportify) employ "POD billing" staff and offer POD document return for B2B clients [22].

**Gatekeeping.** Lodgement is closed. Only licensed brokers and registered importers may submit import declarations, and only via an accredited VASP; a draft order named three (e-Konek, Cargo Data Exchange Centre, InterCommerce Network Services); VASP fees are set by providers subject to BOC approval, amounts not found [23]. BOC says it will fully modernize E2M by 2027 [24], so anything wired to today's lodgement format faces a platform change. Tariff classification and valuation are regulated professional judgments; an agent drafting them carries liability. Shipment data is commercial, not DPA-sensitive.

**Payment evidence.** Brokers already pay per-entry VASP fees (amounts not found); global forwarder suites (CargoWise etc.) are enterprise-priced and not aimed at single-broker shops. *Inference.* Track-POD/DispatchTrack sell POD/dispatch globally; PH-specific prices not found [22].

**Free distribution.** Customs-broker reviewer and licensure communities are visible around PRC exams (results publicised by exam sites) [21]; group sizes not measured. Trucking owners: not found.

**Verdict.** The *client-side paperwork chase* for small customs brokerages ("send me the packing list", "your entry is lodged, duties are ₱X", "pay by Friday") fits the agent core and avoids lodgement and classification. Evidence of pain and willingness to pay is thin, and the E2M overhaul adds risk. Worth a watch, not a lead. *Confidence: L-M.*

## 4. Construction, contractors, architects/engineers

**Size.** PCAB reported 16,528 regular licensed contractors in its 2022 milestones; small categories (C, D, Trade/E) were ~62% of licences in 2017 [25]. About 5,100 construction MSMEs by the 0.41% sector share (my calculation; many home-builders operate unlicensed or as sole proprietors) [3].

**Workflows.**
- *Quotation / BOQ / POW / DUPA:* quantity take-off from drawings into an itemised BOQ with DUPA build-ups (DPWH mark-ups now per D.O. 30, s. 2025) [26].
- *Building permit packets:* PD 1096 permits via the LGU Office of the Building Official; notarised Unified Application Form in four copies, zoning clearance, sealed plans; occupancy permits need as-built plans and the construction logbook [27][28]. Missing documents are the usual cause of delay (contractor guide) [27].
- *Progress billing:* % completion statements, retention, change orders, chasing owner payments.

**Payment evidence (strongest in this sector).** AEDO Engineering sells an itemised BOQ for **₱3,000-8,000** per job (3-5 days, DUPA included) and a **₱15,000/month** design retainer for contractors and developers (page dated 2026-09-04) [26]. Vestimate (PH) sells plan-trace-to-BOQ with DPWH-format export, free to start; paid price not found [29]. Permit fees run from a few thousand to ₱15,000-50,000+ for residences (commercial guides; LGU-specific) [27].

**Gatekeeping.** Plans must be signed and sealed by licensed architects/engineers; an agent can draft but never seal. OBO portals vary by LGU (some online, most not); no APIs. Data is commercial, low DPA sensitivity.

**Free distribution.** Contractor and estimator communities exist on FB; sizes not measured. AEDO's SEO-led Taglish blog ("Paano kumuha ng building permit") shows SEO is a working channel in this niche [28].

**Competition.** Freelance estimators (₱3k/BOQ), Excel templates, Vestimate, Odoo BOQ modules [29].

**Verdict.** Two openings. (a) **Progress billing & collections for small contractors** fits the agent core (scheduled statements, owner-approved nudges, change-order paper trail) with low liability. (b) **BOQ drafting** has evidenced per-job payment but is a vision/take-off accuracy problem, where errors cost money and need an estimator's review; it is less agent-core and more estimating software. *Confidence: M.*

## 5. Food & hospitality

**Size.** 190,899 accommodation and food-service MSMEs in 2023, the second-largest MSME sector (DTI summary of PSA) [2]. DOT data counted 335,592 room keys in 18,818 accommodation establishments, accredited and unaccredited, at the launch of the 2023-2028 hotel plan [30].

**Workflows.** Small resorts and inns take inquiries on FB/Messenger ("how much po for 10 pax?"), quote rates, collect a GCash/bank downpayment, confirm, remind before arrival and collect the balance; OTA guest messages arrive separately via Agoda YCS and the Booking.com extranet. Caterers and event venues send menu-based quotes and chase decisions.

**Payment evidence.** Codingclave sells a PH hotel PMS with a two-way OTA channel manager at **₱1,699/month** cloud, ₱14,999/year, or a ₱67,999 one-time licence, aimed at inns and single resorts [31]. Global guides put channel managers at US$50-150/month for independent hotels, or US$3-10 per room [31]. PH vendors Hotel Link and Djubo (GCash/Maya/Xendit) sell locally; prices not published [31].

**Gatekeeping.** OTA messaging is gated. Booking.com's Messaging API is for certified connectivity partners, and a third-party report card (verified July 2026) says new connectivity-partner onboarding is **paused** [32][33]. Agoda YCS messaging API: not found. FB inquiries are covered by Meta's free Business Agent for inbound answers (earlier research [1]). Data sensitivity is low (guest names, dates).

**Pain evidence.** No public evidence of resort no-show or downpayment-chasing rates was found. *Not found.*

**Free distribution.** Resort and homestay owner FB groups and regional tourism councils likely exist; sizes not measured.

**Verdict.** Not materially new. The FB-side workflow is lead/quote follow-up (B) plus GCash-proof matching (S2) for a seasonal vertical; the OTA side is closed to a solo developer. Treat "small resort reservation desk" as a vertical skin of B, not a separate product. *Confidence: M.*

## 6. Agriculture & cooperatives

**Size.** 20,962 registered cooperatives, of which 7,957 multipurpose and 2,874 credit cooperatives (FY2023 CDA operations report, via a secondary source; primary not reached) [34]. Fisheries/farm enterprises as software buyers: not researched further (no paid tools found).

**Workflows.**
- *Member loan collections:* amortisation schedules, payroll-deduction and cash-collection reconciliation, overdue notices, promise-to-pay tracking.
- *Mandatory CDA reports:* the CAPR, audited financial statements, social audit and performance audit are due 120 days after year-end (30 April) via CDA's online system. Late filing costs **₱100/day**, can block the Certificate of Compliance, and can lead to revocation of registration; non-filing of BIR annexes risks the coop's tax-exemption certificate. Micro coops under ₱3M assets are excluded from the fine. For FY2025 the deadline was extended to 31 May 2026 under MC 2026-07 [35][36].
- *Member communication:* general-assembly notices, dividend and patronage-refund statements.

**Payment evidence.** PH-specific coop systems exist and sell on demos, not public prices. CoopVault (WeSupport Inc.) advertises collections with promise-to-pay tracking, overdue letters (4 types), batch overdue jobs, 26 SMS/email templates, CDA/BSP reporting and CIC (RA 9510) export, with no AI features listed (vendor claims) [37]. Citrus Accounting is reviewed for member ledgers, loan repayments and dividend calculations [38]. Prices: not found.

**Gatekeeping.** Member loan data is personal financial information; credit coops also report to the Credit Information Corporation under RA 9510 [37]. CDA filing has no public API found. Large credit coops fall under stricter regimes; small ones run on Excel or desktop systems. *Inference.*

**Free distribution.** CDA regional offices publish reminders on public pages [35]; coop federations and unions hold seminars. Sizes of coop-officer FB groups: not found.

**Verdict.** "Collections follow-through for small multipurpose coops" is plausible (high frequency, lawful, approval-gated nudges), but incumbents already ship PTP tracking and overdue letters, the data is financial, and buying decisions go through boards. The **annual-report deadline** is a sharp yearly pain but is accountant work (audited FS), not an agent loop. Watch-list. *Confidence: M.*

## 7. Retail, sari-sari, distributors

**Size.** 604,313 MSMEs in wholesale and retail trade (incl. motor repair) in 2023, the largest sector [2]. GrowSari describes over a million sari-sari stores as its market; third-party profiles cite 50,000+ stores ordering monthly on its platform (aggregator figure, possibly stale) [39]. The number of small distributors and sub-distributors: not found.

**Workflows.** Store owners order from distributor sales agents by Viber, Messenger or text in free-form Taglish ("2 case Lucky Me beef, 1 bundle Safeguard"); the agent re-keys into a booking sheet or DMS, confirms stock/price, schedules delivery on the route day, and tracks credit (utang) terms and collections.

**Payment evidence.** BeatRoute sells distributor/retailer apps in PH with a Viber ordering interface and warns that reps must be paid commission on portal and Viber orders "or they will quietly block it" (vendor) [40]. Globally, AI order-intake for wholesalers is a funded category: a YC-launched company sells AI agents for quoting/order entry; WizCommerce raised US$8M; Take App sells WhatsApp wholesale ordering at US$50/month; a vendor estimates manual B2B order keying at US$30-80 per order vs US$1-5 automated (vendor, US data) [41]. PH SME prices for chat-order capture: not found.

**Gatekeeping.** **Viber, the channel PH distributors favour, charges EUR 100 per bot per month for bots created since 5 Feb 2024, plus per-message fees, and approval is at Viber's discretion** [42], which breaks the ₱600/month budget. Agents' personal Viber/Messenger chats have no API; a Messenger Page works for inbound but needs the store to message the distributor's Page. Low DPA sensitivity.

**Competition.** GrowSari and similar B2B apps disintermediate distributors for FMCG [39]; BeatRoute-class DMS for mid-size distributors [40]; Meta's free Business Agent for generic Page chat [1].

**Verdict.** The extraction task (Taglish chat → SKU lines → approved order → route schedule → credit reminder) is a strong agent-core fit and a real Taglish edge, but the channel economics are bad: Viber bots cost ~₱6,500/month before any message, and the FMCG end is being captured by funded B2B platforms. Best as a **paste/forward-in** tool (agent copies chat into the app) for non-FMCG distributors (hardware, agri-inputs, rice, LPG). *Confidence: M.*

## 8. LGU-facing services (non-government buyer)

**Workflows.** Business-permit renewals (every January), barangay clearances, building/occupancy permits (see §4), fire-safety inspection certificates. Buyers would be the businesses or the accountants/"liaison officers" who process renewals for them.

**Evidence.** Requirements are LGU-specific and published as Citizen's Charters (e.g., Ormoc OBO, Naga) [27]. The professional liaison/permit-processing service market exists (permit runners), but published prices were not found in this sweep. Renewals are yearly (C2 low) and portals differ per LGU with no APIs. **Verdict:** a yearly deadline-reminder and document-checklist feature inside another product (e.g., S1 tax desk or the contractor product), not a standalone business. *Confidence: L.*


## Candidate deep-dives

Scores use [scoring-method.md](scoring-method.md): C1..C16 in order, 5 always favourable, E = evidence-backed, J = judgement (capped at 4). Weighted totals computed with the method's three profiles. For context, the current leaders in [opportunity-scorecard.md](opportunity-scorecard.md) §8 are S1 74.4 and B 70.8 (Balanced); scores here were made by a different pass and are comparable only roughly.

| ID | Candidate | Balanced | Revenue-first | Builder-first | J share |
|---|---|---|---|---|---|
| **CON1** | Progress-billing & collections desk for small contractors | **71.8** | **71.0** | 73.4 | 12/16 |
| **EDU1** | Tuition & promissory-note follow-through for small private schools / tutorial centres | 71.2 | 70.0 | 73.4 | 11/16 |
| **DIS1** | Taglish chat-order desk for non-FMCG distributors (paste/forward-in) | 68.8 | 68.6 | 70.4 | 13/16 |
| LOG1 | Customs-broker document-chase & client status desk (watch-list) | 67.8 | 66.0 | 70.6 | 16/16 |
| COOP1 | Member-loan collections for small coops (watch-list) | 66.4 | 66.2 | 68.6 | 14/16 |

No disqualifying 1 on C5, C6 or C11 in any of them.

### CON1: Progress-billing & collections desk for small contractors

**Problem.** Small contractors bill homeowners and developers by percentage of accomplishment, with retention withheld (commonly 10%) until acceptance. Payments slip, and unpaid progress billings stop sites: Supreme Court cases show subcontractors suspending work when down payments and progress billings were withheld, and contracts charging 2%/month on payments over 30 days late [43]. The paperwork (statement of work accomplished, progress certificate, retention ledger, change orders) is done in Excel by the owner-engineer at night. *Pain: E for the mechanism, J for its frequency among small firms.*

**Buyer.** Owner of a PCAB small-category (C/D/Trade) contractor or unlicensed home-builder running 2-10 concurrent residential/light-commercial jobs. 16,528 regular licensees in 2022, about 62% small [25].

**Evidence of payment.** BuildMark (PH) lists progress billing by % accomplishment, PD 1594 retention tracking, SOWA blocks and BIR invoicing for "small construction companies in the Philippines" at **₱7,500/month (first 10 customers), ₱10,000/month standard, ₱100,000/year**; no collections or AI features listed; customer count not stated (vendor page) [44]. (A search summary also mentioned a ₱2,499 one-time contractor app, but the listing could not be opened, so it is not counted.) Contractors pay ₱3,000-8,000 per BOQ and ₱15,000/month estimating retainers [26]. Global: Billdr PRO from US$160/month, CoConstruct from US$99/month [45].

**Competitors.** BuildMark (direct on billing, ~₱10k/mo, no follow-through), Excel, Odoo milestone-billing modules [45], freelance estimators.

**Agent workflow (approval points marked ✔).**
1. Owner forwards the contract/BOQ PDF and photos; agent extracts payment schedule, retention %, milestones (LLM extraction).
2. On each billing date (durable schedule) agent drafts the progress billing + SOWA from the latest accomplishment % the engineer enters or dictates in Taglish. ✔ owner approves amounts.
3. Sends PDF to the client by email/SMS/Messenger-within-window; logs it.
4. Durable follow-ups at +3/+7/+14 days, escalating politely, citing the contract's late-payment clause. ✔ each nudge approved (one tap) until the owner opts into auto-send for gentle reminders.
5. Reads payment proofs (GCash/bank screenshots) and proposes matching. ✔ owner confirms.
6. Change-order requests drafted from chat ("dagdag ng isang CR sa 2nd floor") into a signed-off variation record. ✔ owner and client approve.
7. Retention-release reminder at turnover; weekly cash-position digest.

**Integrations.** Email (SMTP/Gmail send), Semaphore SMS at ~₱0.50-0.56 per SMS [46], PDF generation, Messenger Page inbound only. No gated APIs.

**MVP size (1 dev).** ~5-6 weeks: contract/BOQ extraction, billing generator, schedule engine, approval inbox (web/PWA), SMS/email send with idempotency, payment-proof matcher (reused from S2 work).

**Risks.** Construction clients are few per contractor (low message volume, so value per message must be high); BuildMark may add reminders; contractors may resist "AI writing to my client"; reachability unproven (FB group sizes not found).

**What would disprove it.** (a) A Taglish SEO/FB post offering a free "progress billing + retention template generator" gets < 30 sign-ups in 6 weeks; (b) sign-ups say clients pay on time and the billing doc is the only pain (then it is a template, not an agent); (c) BuildMark or similar ships automated collections below ₱2,000/month.

**Score:** `4E 3J 4J 3E 4E 3E 3J 4J 4J 3J 4J 4J 4J 3J 4J 4J` (Balanced 71.8).

### EDU1: Tuition & promissory-note follow-through for small private schools and tutorial centres

**Problem.** RA 11984 (March 2024) forces schools to let certified disadvantaged students sit exams despite unpaid fees, but explicitly preserves **promissory notes stating the amount and the payment date**, withholding of records, and other legal remedies [15][47]. Each promissory note is a dated promise that someone must follow up. Public shaming (posting names of debtors in group chats) is a likely Data Privacy Act violation per legal commentary [48], so follow-up must be private and one-to-one. Many schools are financially thin: CEAP found ~49% of 224 basic-ed respondents at a loss or break-even; PACU's 27-college sample covered only ~7.7 months of costs from collections [17].

**Buyer.** Owner/administrator or finance officer of a small private school (100-800 learners) or a tutorial/review centre. 2.63M private-school learners in SY 2024-25 [14]; school count split not found.

**Evidence of payment.** SIS vendors with fee billing are sold in PH (GenyoX, MIS-PH, GlobalCom; OpenEduCat with GCash/Maya instalments), prices unpublished [18]. Payment rails are cheap or free to the school: HitPay offers GCash recurring billing for tuition centres with no monthly fee (vendor) [49]; schools such as Ateneo de Iloilo and UP Manila added GCash bills-pay [49]. Tutorbase gives reminders free with a 1% invoicing fee [19]. **No PH price for a reminders/collections layer was found.**

**Competitors.** SIS billing modules, HitPay/ECPay rails, spreadsheets + cashier SMS. None found that schedules promissory-note follow-ups or drafts Taglish messages.

**Agent workflow.**
1. Import the ledger (CSV/Excel from the SIS or cashier) and parent contacts; extract promissory notes from scanned forms (LLM extraction).
2. Durable schedule per instalment and per promissory-note date.
3. Drafts private, polite Taglish/English reminders tailored to the case (first reminder, missed promise, records-release notice). ✔ finance officer approves batches.
4. Sends via SMS (Semaphore) or email; Messenger only when the parent wrote first within 24 h.
5. Reads GCash/bank proofs sent back; proposes ledger matches. ✔ cashier confirms.
6. Guardrails in code: never mentions exams as leverage (RA 11984), never sends to group chats, never discloses balances to anyone but the registered payer.
7. Monthly receivables-ageing digest to the owner.

**Integrations.** CSV/Excel import, SMS, email, PDF statements. SIS APIs not found, so import is manual.

**MVP size.** ~4-5 weeks (simpler than CON1: tabular ledger, fixed message types).

**Risks.** Minors' and household financial data (DPA consent, NPC complaints); seasonal buying (June-August enrollment); principals may prefer a human-signed letter; SMS cost per school is the school's.

**What would disprove it.** Fewer than 20 school/tutorial-centre sign-ups from a free "promissory-note tracker + RA 11984-safe reminder templates" page in one enrollment season; or schools saying their SIS already sends reminders.

**Score:** `4E 3J 4J 3E 3J 3E 3J 4J 4E 4J 3E 4J 4J 4J 4J 4J` (Balanced 71.2).

### DIS1: Taglish chat-order desk for non-FMCG distributors

**Problem.** Store and contractor customers order from distributor agents in free-form Taglish chat; agents re-key into booking sheets, errors and missed orders follow. A vendor estimates manual B2B order keying at US$30-80 per order vs US$1-5 automated (US data, vendor claim) [41].

**Buyer.** Owner of a small hardware, agri-input, rice, LPG or school-supply distributor with 2-15 field agents. Count not found.

**Evidence of payment.** Global category is funded (YC-launched wholesale order agents; WizCommerce US$8M) and priced (Take App US$50/month for WhatsApp wholesale ordering) [41]; BeatRoute sells Viber ordering to PH FMCG distributors (price not published) [40]. PH SME price for this: not found.

**Competitors.** GrowSari-type B2B apps (FMCG) [39], BeatRoute [40], Meta's free agent [1].

**Agent workflow.** Agent forwards or pastes a chat (or the customer messages the distributor's Page) → LLM extracts SKU lines against the price list, flags ambiguities → ✔ agent/owner approves the order → order sheet + delivery slot on the route day → durable reminders: re-order nudges on each customer's usual cycle, credit-due reminders ✔.

**Integrations.** Messenger Page (inbound), email, SMS; **Viber bots cost EUR 100/bot/month plus per-message fees since Feb 2024** [42], so Viber is excluded under the budget.

**MVP size.** ~5 weeks (catalogue matching is the hard part).

**Risks.** Channel lock-in to personal Viber accounts; rep resistance (BeatRoute's own warning) [40]; FMCG captured by funded platforms; reaching distributors without a network.

**What would disprove it.** Owners refuse the paste-in step; extraction accuracy on real Taglish orders < 90% line-level; no sign-ups from hardware/agri-supply FB groups.

**Score:** `3J 4J 4J 4E 3J 3E 2J 4J 2E 3J 4J 4J 4J 4J 4J 4J` (Balanced 68.8).

### Watch-list scores (not deep-dived)
- **LOG1** customs-broker document chase: `3J 4J 3J 2J 3J 3J 3J 4J 4J 4J 4J 4J 4J 4J 3J 4J`. All judgement; E2M overhaul by 2027 [24].
- **COOP1** coop loan collections: `3J 4J 4J 3E 3J 2E 3J 4J 4J 3J 2J 4J 4J 4J 3J 4J`. CoopVault already ships PTP tracking and overdue letters [37]; financial data.

## Sources

Format: publisher, title, date, URL; evidence type; confidence. All accessed 2026-10-18.

1. This repo, opportunity-scorecard.md §7-8 and opportunity-landscape.md, 2026-10-18; internal synthesis; M.
2. DTI, "2023 Philippine MSME Statistics" (PSA 2023 List of Establishments), https://dti.gov.ph/businesses/msmes/msmeresources/msme-statistics (via search summary; page fetch failed); official statistics; H.
3. Statista, "MSME sector by industry, Philippines" (DTI data; year label ambiguous), https://www.statista.com/statistics/1016349/business-industries-msme-sector-philippines; secondary statistics; M.
4. Statista citing DOH, licensed drug establishments by type, https://www.statista.com/topics/7369/pharmaceutical-market-in-the-philippines (2021 figure; 2025 paywalled); secondary statistics; M.
5. Aetna International, "Maxicare Orientation" (PDF, undated), https://www.aetnainternational.com/content/dam/aetna/pdfs/aetna-international/lds/Maxicare-Orientation.pdf; HMO process document; M.
6. SeriousMD Help Center, "Billing: HMO tracking setup", https://help.seriousmd.com/en/articles/4726515-billing-hmo-tracking-setup; vendor documentation; H.
7. ABS-CBN News, "Solon urges PhilHealth to revise claims processing amid billions in denied, return-to-hospital claims", 2025-01-15, https://abs-cbn.com/news/business/2025/1/15/philhealth-urged-to-revise-claims-processing-as-denied-claims-pile-up-1646; news citing PhilHealth testimony; H.
8. PhilHealth, Circular PC2025-0019, 2025, https://www.philhealth.gov.ph/circulars/2025/PC2025-0019.pdf; regulation; M (via search summary).
9. Negros Now Daily, "PhilHealth settles P2.94-B in benefit claims in NegOcc", 2025, https://negrosnowdaily.com/philhealth-settles-p2-94-b-in-benefit-claims-in-negocc/; regional news; M.
10. Ken Research, "Philippines pharmacy retail market", 2025, https://www.kenresearch.com/industry-reports/philippines-pharmacy-retail-market.md; market report; L.
11. SoftwareSuggest, "Best Clinic Management Software in Philippines", https://www.softwaresuggest.com/clinic-management-software/philippines; directory; L.
12. PhilHealth, TS Circular 2016-016 (eClaims software certification), https://www.philhealth.gov.ph/circulars/2016/TS_circ2016-016.pdf; regulation; H.
13. PhilHealth, CSP/HITP FAQs, https://www.philhealth.gov.ph/partners/csp/faqs.html; official FAQ; H.
14. Inquirer / The Post, DepEd SY 2024-2025 enrollment reports, Aug 2024, https://newsinfo.inquirer.net/1970100/deped-still-looking-to-hit-27m-enrollment-target and https://thepost.net.ph/news/nation/deped-over-23m-students-enrolled-for-sy-2024-2025/; news citing DepEd; M.
15. Batas Natin, "RA No. 11984 - No Permit, No Exam Prohibition Act", 2024-03-11, https://batasnatin.com/laws/ra-11984; statute; H.
16. Philippine News Agency, "'No permit, no exam' prohibition law has safeguard for private schools", 2024-03, https://alpha.pna.gov.ph/articles/1221244; government news; H.
17. Inquirer, "Banning 'no permit, no exam' policy puts schools in danger -- Cocopea" and "Some schools to close in 2 months... - PACU", 2023, https://newsinfo.inquirer.net/1720550/banning-no-permit-no-exam-policy-puts-schools-in-danger-cocopea, https://newsinfo.inquirer.net/1770312/some-schools-to-close-in-2-months-if-no-permit-no-exam-ban-is-imposed-pacu/amp; news citing industry surveys; M.
18. OpenEduCat, "School management system Philippines", https://openeducat.org/solutions/school-management-system-philippines/; vendor claims; L-M.
19. Teach 'n Go, https://www.teachngo.com/solutions/tutoring-management-software; GetApp SG, Tutorbase, https://www.getapp.sg/software/2081267/tutorbase; vendor/directory; M.
20. (Reserved; FB group sizes for school registrars/tutorial owners searched, not found.)
21. PRC, "November 2025 Customs Brokers Licensure Examination Results...", 2025-11, https://www.prc.gov.ph/article/november-2025-customs-brokers-licensure-examination-results-released-two-2-working-days; official; H.
22. Bossjob, Roadwise Logistics Corp. (Pakargo.ph), https://bossjob.ph/company/roadwise-logistics-corporation-49661; Apple App Store, Transportify, https://apps.apple.com/ph/app/id1153482546; Capterra, Track-POD, https://www.capterra.com/p/212169/Track-POD/reviews/3832646/; listings; M.
23. PortCalls, "No corporations may register with VASPs, says BOC VASP secretariat" and "Proposed BOC order sets rules for VASP accreditation" (dates not shown), https://portcalls.com/no-corporations-may-register-with-vasps-says-boc-vasp-secretariat, https://archives.portcalls.com/proposed-boc-order-sets-rules-vasp-accreditation/; trade news; M.
24. Context.ph, "BOC fast-tracks digital customs overhaul by 2027", 2026-05-18, https://context.ph/2026/05/18/boc-fast-tracks-digital-customs-overhaul-by-2027/; news; M.
25. CIAP/PCAB, statistics pages (licensed contractors by category, 2017; 2022 milestones), https://construction.gov.ph/statistics/; official statistics; M (via search summary).
26. AEDO Engineering, "BOQ, BOM, POW, DUPA Philippines", 2026-09-04, https://aedoconstruction.com/blog/boq-bom-pow-dupa-philippines/; vendor price list; M.
27. AEDO Engineering, "Building permit Philippines 2026", https://aedoconstruction.com/blog/building-permit-philippines-2026/; ExpatFocus guide, https://www.expatfocus.com/philippines/guide/philippines-property-building-and-renovation; City of Naga (Cebu) and Ormoc OBO Citizen's Charter, https://cityofnagacebu.gov.ph/?p=17460, https://ormoc.gov.ph/assets/docs/citizens_charter/Office%20of%20the%20Building%20Official%20-%20Citizens%20Charter.pdf; guides and LGU charters; M.
28. AEDO Engineering, "Paano kumuha ng building permit sa Pilipinas", https://aedoconstruction.com/blog/paano-kumuha-ng-building-permit-pilipinas/; Taglish SEO content (channel evidence); M.
29. Indie Hackers, Vestimate, https://www.indiehackers.com/product/vestimate; Probuse Odoo BOQ module, https://probuseappdemo.com/blog/odoo-apps-3/project-boq-construction-contracting-1763; listings; L-M.
30. The Post, "With 12 million foreign tourists by 2028, does PH have enough hotel rooms...", https://thepost.ph/travel/with-12-million-foreign-tourists-by-2028-does-ph-have-enough-hotel-rooms-to-accommodate-them/; HospitalityNews.ph on PHISAP, https://hospitalitynews.ph/dot-phoa-releases-phisap-hoping-to-meet-the-projected-456055-room-keys-by-2028/; news citing DOT; M.
31. Codingclave, "Hotel Management Software Philippines 2026", https://codingclave.com/blog/hotel-management-software-philippines-2026; SmartOrder, "OTA Channel Manager Pricing 2026", https://www.smartorder.ai/resources/blog/ota-channel-manager-pricing-2026/; Hotel Link, https://try.hotellinksolutions.com/channel-manager-philippines; Djubo, https://djubo.com/philippines; vendor pages; M.
32. Booking.com Developers, Messaging API docs, https://developers.booking.com/connectivity/docs/messaging-api/understanding-the-messaging-api; official docs; H.
33. Supergood, "API Report Card: Booking.com" (verified July 2026), https://supergood.ai/api-report-card/booking-com; third-party assessment; M.
34. Expressway.ph, "Cooperative auto loan guide" (citing CDA FY2023 operations report), https://www.expressway.ph/car-loan/cooperative-auto-loan-guide; secondary; M-L.
35. CDA Region III, "Reminder letter on the submission of mandatory reports for FY 2025" and extension notice, 2026, https://cda.gov.ph/region-3/reminder-letter-on-the-submission-of-mandatory-reports-for-fy-2025/; official; H.
36. Grant Thornton PH, "CDA's extension of the deadline for the submission of FY 2025 required reports...", 2026, https://www.grantthornton.com.ph/technical-alerts/accounting-alert/2026/cdas-extension-of-the-deadline-for-the-submission-of-fy-2025-required-reports-and-coc-validity-for-cy-2025/; professional alert; H.
37. WeSupport Inc., CoopVault, https://coopvault.w.wesupportinc.com/; vendor claims; M.
38. G2, Techworks Software Solutions (Citrus Accounting), https://www.g2.com/sellers/techworks-software-solutions; reviews; L-M.
39. GrowSari, "About GrowSari", https://business.growsari.com/about-growsari/; Wavemaker spotlight, https://wavemaker.vc/sea/transforming-traditional-trade-spotlight-on-b2b-startup-growsari; vendor/investor; M.
40. BeatRoute, "Order booking app" and "Retailer app" (PH), https://beatroute.io/philippines/resources/blog/order-booking-app/, https://beatroute.io/philippines/fmcg/retailer-app/; vendor claims; M.
41. Y Combinator Launch, Seals AI, https://www.ycombinator.com/launches/LqH-seals-ai-ai-employees-for-wholesalers-distributors; WizCommerce, https://wizcommerce.com/wizorder/; Take App, https://www.take.app/industry/b2b-and-wholesale; Darwin AI blog, https://blog.getdarwin.ai/en/ai-order-taking-whatsapp-email-orders-erp; vendor pages (cost-per-order figure is a vendor claim); M.
42. Rakuten Viber Help Center, "Bot commercial model" and FAQ, https://help.viber.com/hc/articles/15247629658525-Bot-commercial-model; official; H.
43. Supreme Court decisions on unpaid progress billings (2008, 2015), via ChanRobles, https://chanrobles.com/cralaw/2008julydecisions.php?id=688 and https://www.chanrobles.com/cralaw/2015decemberdecisions.php?id=1051; case law; M (case facts via search summary).
44. BuildMark, "Construction Management Software" (PH; pricing page), https://buildmark.it.com/; vendor price list; M.
45. Capterra, Billdr PRO alternatives, https://www.capterra.com/p/10008715/Billdr-PRO/alternatives/; ForConstructionPros on CoConstruct, https://www.forconstructionpros.com/construction-technology/apps/blog/12222724/coconstruct-app-coconstruct-helps-contractors-manage-clients-projects-and-budgets; Probuse Odoo milestone billing, https://probuseappdemo.com/blog/odoo-apps-3/construction-contracting-milestone-based-billing-1654; directories/vendor; M.
46. Semaphore, homepage and FAQ, https://www.semaphore.co, https://api.semaphore.co/faq; vendor price; H.
47. Inquirer, "New law scraps 'no permit, no exam' rule", 2024-03, https://newsinfo.inquirer.net/1919598/new-law-scraps-no-permit-no-exam-rule; news; H.
48. Respicio & Co., "How to file a complaint against a school for tuition issues", https://www.respicio.ph/commentaries/how-to-file-a-complaint-against-a-school-for-tuition-issues; legal commentary (not a regulator ruling); M.
49. HitPay, "Education payment solutions" (PH), https://hitpayapp.com/ph/solutions/education-payment-solutions; Fintech News PH, "Ateneo de Iloilo... GCash", 2026-01, https://fintechnews.ph/69701/digital-payments/gcash-ateneo-de-iloilo-partnership-digital-tuition-payments/; UP Manila GCash notice, https://www.upm.edu.ph/cpt_news/going-e-upm-links-with-gcash-for-ease-of-payment-transactions/; vendor/news; M.

**Search budget note.** Facebook group member counts for every sector (registrars, tutorial owners, contractors, resort owners, coop officers, distributors) could not be found via web search. That is the biggest evidence gap for C7. Checking them by hand on Facebook costs nothing and should come before any build.
