# AI add-ons for HR, payroll, timekeeping and people-ops software

Status: **complete for this pass** (2026-10-20; the research was interrupted once by a rate limit and then resumed; gaps are marked "not found"). Research slice answering the owner's question "an AI agent or automation for ERP, CRM, HR or other software — what would we build?", HR slice only. Home market: Philippines; second channel: global HR app marketplaces.

Scoring uses [scoring-method.md](scoring-method.md) (C1–C16, 5 = favourable, J capped at 4). Calibration anchor: P1 bookkeeper chaser, `4E 4E 4J 4E 4E 4E 3E 4J 4E 4J 3J 4J 4J 5E 4J 4J`, 78.6 Balanced.

Evidence types: **P** primary (law, regulator, official docs), **V** vendor claim, **T** third-party (press, analyst, review site), **F** forum/community. Confidence: H/M/L. All sources accessed 2026-10-20 unless noted.

## Key conclusions

Facts carry source numbers; **Recommendation** and **Judgement** are labelled.

1. **Global HR marketplaces are closed to us.** BambooHR requires ≥ 100 total customers to list [19]; Personio asks for 50+ shared customers and ≥ 10 adoptions [25]; Gusto gates production keys behind pre-approval, QA and a third-party security review that asks for audits such as SOC 2 [21]; Rippling approves at its sole discretion [27]. Only Zoho Marketplace is open (free listing, two-business-day review) [28]. So the "global marketplace second channel" does not work for HR.
2. **The platform owners already ship the obvious AI add-on.** Sprout ships Sidekick: employee and HR Q&A grounded in PH labour rules, Attendance and Payslip agents, COE generation, and Tagalog support [13]. Ask BambooHR (May 2025) [20], Gusto's Gus (Sept 2024) [22] and Deel's AI Workforce (Aug 2025: hiring, time-off and offboarding agents) [24] do the same. An employee-helpdesk or HR-Q&A add-on scores C6 = 1–2.
3. **Every HR candidate processes sensitive personal information.** SSS, TIN, PhilHealth and Pag-IBIG numbers, tax returns and health records are SPI under RA 10173 Sec. 3(l). Penalties are 3–6 years plus ₱500k–₱4M, including for negligent access [8]. NPC registration becomes mandatory once SPI of ≥ 1,000 people is processed, or immediately for any automated decision-making such as applicant screening [7]. C11 ≤ 2 throughout, and 1 for recruitment screening.
4. **PH payroll computation is cheap and commoditised.** KAMI charges ₱2,500 base + ₱120 per person per month [32]; Sprout's outsourced starter is ₱7,900 a month for 10 employees [29]; PayrollHero is listed at US$3.50 per user (stale listing) [33]; free calculators exist [3][36]. An agent can earn margin by chasing missing inputs and deadlines, not by computing pay. Computing pay would also pull in BIR CAS questions [9] and lower C10.
5. **The best HR idea is a module of P1, not a rival to it.** **HR2** (payroll-input and remittance chaser for bookkeepers who run client payroll) scores 72.6 Balanced against P1's 78.6. It reuses P1's statutory calendar, chaser loop and buyer, and adds SSS, PhilHealth and Pag-IBIG windows plus payroll cut-offs [3]. **Recommendation:** treat HR2 as a P1 expansion in the Phase 6 spec, not a new lead. Every standalone HR candidate scores ≤ 60.4.

## Landscape

PH-home HR/payroll vendors and the global platforms PH SMEs meet. "Not found" means this pass did not surface it, not that it does not exist.

| Vendor | Price (published or third-party) | API / marketplace for third parties | Built-in AI (= competition) | Source, type, confidence |
|---|---|---|---|---|
| Sprout Solutions (PH) | Outsourced Payroll Starter **₱7,900/mo for 10 employees** (vendor); other pages reportedly show ₱5,000 or ₱10,000 (third-party); standard plans by quote; third-party estimate ₱120–200 PEPM | **API exists** (HR, timekeeping, payroll; sandbox + prod portals). Onboarding "starts with your assigned Customer Success Manager", with Full- or Restricted-Access credentials, so it is built for customers and partners, not open sign-up. **Sprout Marketplace** lists ≥ 15 partner apps (JuanTax, Manatal, Hubstaff, Doconchain, Nova AI Recruiter…); no public developer application found, only "Book a Meeting" and a channel-partner programme (referral commissions, reseller margins, prices on request) | **Sidekick**: HR/payroll Q&A grounded in PH rules, role-based; Attendance Agent; Payslip Agent; COE generation; Concierge; HR Link (English/Tagalog). Webinar 21 Jan 2026 | [29] V M / T L; [30] P M; [31] P H / V M; [13] V H |
| KAMI Workforce (PH) | Lite ₱2,500 base + ₱120/person/mo; Plus ₱3,500 + ₱180; Super ₱5,500 + ₱280; Managed Payroll ₱5,500 + ₱380/employee (annual billing) | Not found | Not checked | [32] V H |
| PayrollHero (PH) | US$3.50/user/mo + US$100 setup (G2, profile inactive > 1 yr); US$4–10 tiers (SoftwareSuggest) | Not found | Not found | [33] T L |
| Salarium (PH) | No published pricing (TrustRadius) | Not found (site unreachable on 2026-10-20) | Not found | [34] T L |
| Payruler, JuanHR, Kayana, GreatDay HR | Not found | Not found | Not found | Searches returned nothing usable |
| Mekari Talenta (ID, regional) | Not found | Not found | AI candidate screening, AI liveness attendance validation, review summaries | [35] V M |
| Kalibrr (PH recruiting) | One aggregator says US$120 promo package (undated, unverified) | Not found | AI candidate matching on paid plans | [17] V/T L |
| SweldoPH, SmartSalaryTool | **Free** calculators and payslip generators; SmartSalaryTool claims multi-employee payroll with Excel export | n/a | n/a | [3] T M; [36] V L |
| BambooHR | Not checked | Marketplace gated: ≥ 100 customers | Ask BambooHR; Bamboo AI beta | [19][20] |
| Gusto | Not checked; US payroll (PH employers not supported, judgement M) | Gated production access | Gus | [21][22] |
| Deel | Not checked; serves foreign companies hiring in PH (EOR/contractor) | App Store via OAuth2, "request publishing"; vendor claims 40,000+ customers | Deel IQ; AI Workforce | [23][24] |

**Reading.** In PH the incumbent with the API, the marketplace and the AI is the same company: Sprout. Its marketplace is partner-curated, and its AI already covers employee Q&A, attendance, payslips and COEs. Smaller PH vendors publish no APIs. A third-party agent would therefore have to work *beside* whatever payroll tool or spreadsheet the client uses (email, Sheets, Messenger, uploads), not *inside* it. That is the same integration-free shape as P1.

## Pain points

Statutory calendar for a PH employer (facts; source numbers refer to the Sources list). These deadlines are the backbone of any "chaser" product: fixed, recurring, penalised, and identical for every employer.

| Obligation | Rule | Penalty / enforcement | Source, type, confidence |
|---|---|---|---|
| SSS contributions | Due by last day of the month after the contribution month (regular employers); PRN from My.SSS required | 2% per month, simple | [3] T M; [4] T M (SSS Circular 2018-022 moved away from the last-digit schedule) |
| PhilHealth premiums | Window in following month by last digit of PEN: 0–4 → 11th–15th, 5–9 → 16th–20th | ≥ 3% per month, compounded | [3] T M |
| Pag-IBIG contributions | Window by first letter of employer name: A–D 10th–14th, E–L 15th–19th, M–Q 20th–24th, R–Z 25th–end | 1/10 of 1% per day | [3] T M |
| BIR 1601-C (withholding on compensation) | 10th of following month (manual/eBIRForms); eFPS staggered 11th–15th; December due 15 Jan | 25% surcharge + 12% interest + compromise | [3] T M |
| BIR 2316 to employees | By 31 Jan; duplicate to BIR by 28 Feb (RMC 29-2024, substituted filing) | NIRC Sec. 250 | [5] V/T M |
| BIR 1604-C + alphalist | By 31 Jan | NIRC Sec. 250 | [5] V/T M |
| 13th-month pay | Pay by 24 Dec; compliance report via DOLE Online Compliance Portal by 15 Jan (LA 16-2025 for the 2025 cycle) | DOLE regional monitoring; no exemptions or deferments accepted | [6] T M |
| Annual report on wages (AERW) | Upload to reports.dole.gov.ph; 2024 report deadline extended to 31 Aug 2025 | DOLE inspection | [10] T M |
| OSH reporting (WAIR, AMR, safety committee) | WAIR by 20th of following month; safety committee for ≥ 10 employees reported within 30 days | Per-day fines (unverified figure) | [11] T L |
| Final pay | Within 30 days of separation (LA 06-2020), reiterated by DOLE Jan 2026 | Complaints to DOLE field office | [1] T H; [2] T M |
| Certificate of employment | Within 3 days of request | Same | [1] T H |
| New-hire registration | Register with BIR (1902 for no-TIN), SSS, PhilHealth, Pag-IBIG; "before first payroll" per one guide | Not found | [16] T M |

### Pain-point assessment

| Pain | Who feels it | Frequency | Who already solves it, price | Agent fit |
|---|---|---|---|---|
| Remittance deadlines (SSS/PhilHealth/Pag-IBIG/1601-C) | Owner or bookkeeper of a 5–50-employee business; outsourced bookkeepers | Monthly, 4 deadlines | Every PH payroll SaaS computes them (see Landscape); free calculators (SweldoPH [3]); outsourced payroll (US$20–250 PEPM, vendor guide [14], L); bookkeepers already bundle it | Weak as a standalone. The computation is commoditised; the *reminder* is trivial. Value only when attached to chasing missing inputs (timesheets, approvals) — overlaps P1 |
| Year-end 2316 / alphalist / 13th-month | Same | Yearly (Dec–Feb crunch) | Payroll SaaS generate the forms; accountants charge for year-end | Seasonal; C2 = 1–2. Bad for a subscription |
| DOLE reports (13th-month, AERW, OSH) | Owners, HR generalist | Yearly / per incident | Not found as a dedicated SME product; law firms and consultants | Low frequency, low revenue; a free checklist, not a product |
| Final pay ≤ 30 days + COE ≤ 3 days | HR / owner on every separation | Per separation (monthly in a 50-person firm with typical turnover — judgement) | Payroll SaaS compute final pay; Sprout's AI Sidekick generates COEs [13] V | Real penalty, real dispute risk, but event-driven and computation must be exact (C10 low) |
| Employee leave/attendance/payslip questions | HR person answering chats; employees | Daily in firms ≥ 20 staff (judgement) | Sprout Sidekick (Attendance Agent, Payslip Agent, Concierge, Tagalog HR Link) ships this inside Sprout [13] V H | Platform owner ships it: C6 = 1–2 on any HRIS that has it. Viable only for firms on Excel + Messenger |
| Onboarding document collection (NBI, TIN, SSS, PhilHealth, Pag-IBIG MID, PEME, IDs) | HR / owner per hire; high-turnover sectors (retail, F&B, BPO, security, staffing) | Per hire; weekly in high-turnover firms (judgement) | HRIS onboarding modules (see Landscape); manual Messenger/email chasing otherwise | Best agent shape: chase → collect → validate completeness → human approves. But the documents are SPI (RA 10173 Sec. 3(l)) → C11 = 2 |
| Recruitment screening for SMEs | Owner of an SME posting on FB groups, JobStreet, Indeed, OnlineJobs.ph | Per opening; bursty | Kalibrr ships AI matching (vendor claim, paid tiers, "book a demo") [17] V M; JobStreet/Indeed post free [17] T M | Crowded, platform-owned; automated screening of candidates is "automated decision-making" → mandatory NPC registration [7] |

**Fact vs judgement.** Deadline rules are facts from secondary sources (law-firm, news and free-calculator sites); primary circular texts were not fetched in this pass. Frequencies marked "judgement" are estimates. No PH SME survey quantifying hours lost to HR admin was found ("not found").

## Marketplaces

Global HR app marketplaces, judged for a solo developer with no customers, no company track record and no SOC 2 report.

| Marketplace | Can a solo dev list? | Review / security | Fees | Platform's own AI (competes with an add-on) | Source, type, confidence |
|---|---|---|---|---|---|
| BambooHR Marketplace | **No.** Listing requires ≥ 100 total customers, Crossbeam connection, partner agreement; one application version asks for 25 *shared* customers. "Anyone can build an integration using the BambooHR API", but not list | Security questionnaire for all; SOC 2 / ISO 27001 preferred, one page says third-party audit required; OAuth/OIDC | Commission flow exists (W-9/W-8 "required for commissions"); % not found | **Ask BambooHR** (employee Q&A on PTO, benefits, policies, with citations), announced 22 May 2025; "Bamboo AI" agent layer (timesheet approvals, time-off signals) private beta July 2026 (third-party) | [19] P H; [20] V H / T M |
| Gusto (App Integrations) | Gated. Production keys need an approved Production Pre-Approval application, Security Review and QA; "not all applications are approved"; partnerships review 1–2 weeks, then third-party (VISO Trust) security review | Pre-approval asks for audits, e.g. SOC 2 Type 2 (partner guide); scopes enforced | Not found | **Gus** AI assistant (announced Sept 2024): answers compliance/tax questions and executes tasks (approve PTO, change salary) with owner confirmation; MCP and ChatGPT integrations later | [21] P H; [22] T M |
| Deel App Store | Most open of the big four on paper: OAuth2 app, "request publishing"; contact with partnerships recommended, not mandatory. Publication criteria not visible in the fetched page | Not specified | Not found | **Deel IQ** (GPT-4 HR/compliance Q&A) and **AI Workforce** (Aug 2025, beta): pre-built HR and payroll agents (Hiring Guru, Border Buddy, Goodbye Genie, Time Off Fairy) | [23] P M; [24] V/T M |
| Personio | **No.** Support article: 50+ shared customers and ≥ 10 active customer adoptions; partner page: existing customers in European markets; Partner Manager review | Integration must be reviewed before live data | Not found | Not checked | [25] P M |
| HiBob | Tech-partner application form; criteria not published | Not found | Not found | Not checked | [26] T L |
| Rippling App Shop | Application (company, customers, goals); two-stage pre-vet + App Shop review; approvals at Rippling's sole discretion | Not found | Not found | Not checked | [27] P/T M |
| Zoho Marketplace (People, CRM, etc.) | **Yes.** Free developer console; listing free; developer sets price; review "two business days" | Not found | Revenue share not found | Zoho People AI not checked | [28] P M |

**Findings.**

1. The four best-known SMB HR marketplaces (BambooHR, Gusto, Personio, Rippling) are gated by customer counts, partner approval or security audits a solo developer with no customers cannot pass. BambooHR's ≥ 100-customer floor and Personio's 50 shared customers are explicit disqualifiers (P, H). This mirrors the earlier marketplace finding that only Shopify/Atlassian/Zoho-style stores are open to newcomers.
2. Each platform already ships its own AI assistant for the obvious add-on jobs: employee policy/PTO Q&A (Ask BambooHR, Gus, Deel IQ, Sprout Sidekick), time-off and offboarding agents (Deel AI Workforce). **No evidenced gap was found** for a third-party AI add-on in these stores ("not found" — no forum thread or review complaint surfaced in this pass).
3. Gusto and Deel do not run payroll for PH-domiciled employers in the way a PH SME needs (Gusto is US payroll; Deel serves foreign companies hiring in PH as EOR/contractor). The PH home market and these marketplaces do not share buyers (judgement, M; Gusto PH support not separately verified).
4. Zoho Marketplace is the only open, cheap listing route found, but Zoho People's PH footprint and Zoho's own AI ("Zia") were not checked.

## Regulatory

**Data Privacy Act (RA 10173).**

- Sensitive personal information (SPI) includes information "issued by government agencies peculiar to an individual", explicitly social security numbers and tax returns, plus health records (Sec. 3(l)(3); [8] P H via full-text copies; OCA Circular 53-2023 applies it to GSIS, PhilHealth, Pag-IBIG and TIN numbers). **Every HR candidate here touches SPI**: SSS/TIN/PhilHealth/Pag-IBIG numbers, PEME results, NBI clearance, payslips.
- Penalties for unauthorised processing of SPI: 3–6 years' imprisonment and ₱500,000–₱4,000,000 fine (Sec. 25(b)); the same range for SPI access due to negligence (Sec. 26(b)) [8] P H. A solo developer acting as personal information processor (PIP) for an employer (Sec. 14) is personally exposed to the negligence offence.
- NPC registration (Circular 2022-04, effective 11 Jan 2023): mandatory for controllers or processors with ≥ 250 employees, processing SPI of ≥ 1,000 individuals, or likely to pose risk; **any system involving automated decision-making or profiling must register regardless of size** [7] P/T H. A processor serving 20 SMEs × 50 employees would cross the 1,000-SPI-subject threshold; an AI recruitment screener would trigger the automated-decision rule immediately.
- Sending employee SPI to an LLM API creates a cross-border transfer and a sub-processor chain; AGENTS.md already bans free tiers that train on inputs. A data-processing agreement with each customer would be needed (judgement, H).

**BIR / DOLE accreditation.**

- No DOLE accreditation for payroll software was found ("not found").
- BIR: a payroll system is listed as a component of a Computerised Accounting System (CAS) under RMO 9-2021; CAS is mandatory for large taxpayers and optional for others, and a registered CAS user needs a permit before use [9] T M. Whether a *payroll-only* SaaS used by a non-large taxpayer must be registered was **not found** in primary text. A tool that only reminds, chases documents and drafts messages (never computes withholding or produces books) very probably sits outside CAS (inference, M). Computing payroll or 2316/alphalist output moves the product into the zone where clients may ask for CAS compliance — a reason to stay out of computation.

**Implication for scoring.** C11 is 2 for any candidate that stores government IDs, payslips or health data; 1 if it profiles or ranks applicants. C10 is ≤ 2 wherever the agent computes pay.

## Candidates with score strings

| ID | Candidate | C1..C16 | Balanced | Revenue-first | Builder-first |
|---|---|---|---|---|---|
| HR2 | Payroll-input & remittance chaser for bookkeepers who run client payroll | `4E 4E 4J 3E 3J 3J 3E 4J 4J 4J 2E 4J 4J 5E 4J 4J` | 72.6 | 72.0 | 74.4 |
| HR1 | Onboarding-document & gov't-registration chaser, high-turnover PH SMEs | `3J 3J 3J 3J 2J 3J 2J 4J 3J 4J 2E 4J 3J 3J 4J 4J` | 60.4 | 57.8 | 64.4 |
| HR4 | Separation desk: final-pay 30-day / COE 3-day clock | `4E 2J 3J 2J 2J 3J 2J 4J 4J 3J 2E 4J 4J 2J 3J 3J` | 57.4 | 55.0 | 60.4 |
| HR5 | AI add-on in a global HR marketplace | `3J 3J 3J 3J 3J 1E 2E 3J 1E 4J 2J 3J 3J 4J 4J 4J` | 56.0 | 55.8 | 58.2 |
| HR3 | Messenger HR helpdesk for SMEs on spreadsheets | `2J 4J 2J 2J 2J 2E 2J 3J 3J 3J 2E 3J 3J 4J 3J 4J` | 52.4 | 50.8 | 56.0 |
| HR6 | AI applicant screener for SME job posts | `3J 3J 3J 3E 2J 2E 2J 3J 2J 2J 1E 3J 3J 3J 3J 3J` | 51.0 | 51.2 | 52.0 |

Weighted totals computed with the scoring-method weights (script kept in the session scratchpad, not in `scoring/score.py`). Calibration: P1 = 78.6 / 78.8 / 79.0.

### HR2 · Payroll-input & remittance chaser for bookkeepers who run client payroll (72.6)

- **One line:** each pay cycle, chase every client for DTRs/timesheets, leave and overtime approvals and new-hire/separation changes. Then track the SSS, PhilHealth, Pag-IBIG and 1601-C windows per client, and chase proof of payment (PRN receipts) after each one. The agent never computes pay.
- **Buyer:** freelance bookkeepers and small accounting firms (P1's buyer) who also do payroll for 5–50-employee clients.
- **Price anchor:** KAMI Managed Payroll ₱5,500 + ₱380 per employee per month [32]; Sprout outsourced starter ₱7,900 per month for 10 employees [29]. The bookkeeper's own payroll fee: not found.
- **Competitors:** payroll SaaS (compute but do not chase clients); P1-adjacent bookkeeping tools (Built for Bookkeepers, Juan, unverified for payroll); manual Messenger chasing.
- **Agent loop:** the calendar fires a cut-off → agent drafts Taglish chase messages per client → bookkeeper approves/sends → agent parses replies and attachments for completeness (missing employees, unsigned approvals) → flags gaps → after a remittance, chases and files the payment proof. Human approves every outbound message; no figures are produced.
- **Scores** `4E 4E 4J 3E 3J 3J 3E 4J 4J 4J 2E 4J 4J 5E 4J 4J`
  - C1 4E: statutory penalties of 2%/mo (SSS), ≥ 3%/mo compounded (PhilHealth), 25% + 12% (BIR) [3].
  - C2 4E: semi-monthly payroll plus 4 monthly remittance windows per client [3].
  - C3 4J: one missed PhilHealth window costs more than a month's subscription; unproven at scale.
  - C4 3E: PH employers pay for outsourced payroll [29][32][15]; that bookkeepers pay for *chasing* tools is not found.
  - C5 3J: bookkeepers' WTP inherits P1's evidence, but no payroll-specific price point was found.
  - C6 3J: payroll SaaS compute but do not chase inputs from many clients; no direct competitor found, but searches were shallow.
  - C7 3E: same reach as P1 (r/AccountingPH, bookkeeper FB groups; see P1 evidence).
  - C8 4J: reuses P1's chaser plus a new calendar table.
  - C9 4J: no integration needed (email/Sheets/upload links); Messenger needs Meta app review.
  - C10 4J: drafts only; completeness flags are reviewable.
  - C11 2E: DTRs, payslip inputs and remittance proofs contain SSS/TIN numbers and pay, which is SPI [8].
  - C12 4J: P1 core plus a payroll calendar is ≤ 4 weeks after P1.
  - C13 4J: low tokens per cycle, self-serve.
  - C14 5E: obligations recur by law every month [3].
  - C15 4J: extends P1 and later SME-direct use.
  - C16 4J: durable scheduled tasks, approval gates, document parsing.

### HR1 · Onboarding-document & government-registration chaser for high-turnover PH SMEs (60.4)

- **One line:** for each new hire, chase NBI clearance, TIN (or BIR 1902), SSS, PhilHealth and Pag-IBIG numbers, PEME and IDs over Messenger/email. The agent checks each upload is the right document and not expired, tracks registration status before first payroll, and reminds HR before NBI or health-card renewals are due.
- **Buyer:** owner/HR of retail, F&B, security or staffing SMEs with frequent hiring.
- **Price anchor:** HRIS per-head pricing, ₱120–₱280 per person per month on KAMI [32]; no standalone onboarding-chaser price found.
- **Competitors:** HRIS onboarding modules (Sprout, KAMI); Sprout Sidekick for Sprout customers [13]; manual chasing.
- **Agent loop:** hire event → agent sends the checklist and chases → classifies uploads (document type, expiry) → HR approves each document as accepted and approves every reminder. Registration itself stays with HR.
- **Scores** `3J 3J 3J 3J 2J 3J 2J 4J 3J 4J 2E 4J 3J 3J 4J 4J`
  - C1 3J: delayed registration risks missed contributions; the penalty for late reporting was not found.
  - C2 3J: per hire; weekly only in high-turnover firms.
  - C3 3J.
  - C4 3J: HRIS onboarding is paid for as part of suites; standalone demand not found.
  - C5 2J: SMEs not on an HRIS pay little; the anchor is ₱120/head.
  - C6 3J: open for non-HRIS firms; closed for Sprout customers.
  - C7 2J: no evidenced self-serve channel to SME HR (the only FB HR page found is a dated snapshot [18]).
  - C8 4J.
  - C9 3J: Messenger needs app review; email/upload links are fine.
  - C10 4J: HR accepts every document.
  - C11 2E: government IDs, NBI clearance and PEME are SPI [8]; volume hits the NPC 1,000-subject threshold quickly [7].
  - C12 4J.
  - C13 3J: document-image processing costs more per customer.
  - C14 3J: tracks hiring volume.
  - C15 4J: renewals and offboarding are adjacent.
  - C16 4J.

### HR5 · AI add-on listed in a global HR marketplace (BambooHR / Gusto / Deel class) (56.0)

- **One line:** e.g. a policy-Q&A or onboarding-chaser app sold through a platform app store.
- **Buyer:** SMB HR teams on those platforms.
- **Price anchor:** not found (no comparable add-on price collected).
- **Competitors:** the platforms' own AI (Ask BambooHR, Gus, Deel AI Workforce) [20][22][24].
- **Agent loop:** platform webhook → agent drafts → HR approves in-app.
- **Scores** `3J 3J 3J 3J 3J 1E 2E 3J 1E 4J 2J 3J 3J 4J 4J 4J`
  - C6 1E: the platform owners ship this themselves [20][22][24].
  - C7 2E: we cannot get listed [19][25][27].
  - C9 1E: customer-count floors and security audits [19][21][25].
  - C11 2J: employee PII under GDPR/US state law.
  - Other scores 3–4J: generic SMB-HR value; no evidence either way.

### HR4 · Separation desk: final-pay 30-day and COE 3-day clock (57.4)

- **One line:** on each resignation or termination, run the clearance checklist, the 30-day final-pay clock and the 3-day COE clock. The agent drafts the COE and the employee updates; the accountant computes final pay.
- **Buyer:** SME owner/HR.
- **Price anchor:** not found.
- **Competitors:** payroll SaaS final-pay modules; Sprout Sidekick COE generation [13]; Deel's "Goodbye Genie" offboarding agent [24].
- **Agent loop:** separation logged → agent opens clock and checklist → chases returns of company property and sign-offs → drafts COE and status messages → HR approves each.
- **Scores** `4E 2J 3J 2J 2J 3J 2J 4J 4J 3J 2E 4J 4J 2J 3J 3J`
  - C1 4E: DOLE complaints; deadlines reiterated in Jan 2026 [1][2].
  - C2 2J: event-driven.
  - C4/C5 2J: no one sells this standalone (not found).
  - C6 3J: bundled by HRIS/payroll vendors.
  - C10 3J: COE text is low-risk; final-pay figures are not ours.
  - C11 2E: payslip data.
  - C14 2J: a weak subscription.

### HR3 · Messenger HR helpdesk for SMEs on spreadsheets (52.4)

- **One line:** employees ask leave balance, payslip and policy questions on Messenger; the agent answers from a Google Sheet and the handbook, and drafts COE requests for HR.
- **Buyer:** 20–100-employee SMEs without an HRIS.
- **Price anchor:** the HRIS floor (KAMI ₱2,500 base [32]) already includes employee self-service.
- **Competitors:** Sprout Sidekick [13], Ask BambooHR [20], Gus [22], and every HRIS's self-service portal.
- **Scores** `2J 4J 2J 2J 2J 2E 2J 3J 3J 3J 2E 3J 3J 4J 3J 4J`
  - C6 2E: the platform owners ship it; the only open segment is firms that have not bought an HRIS, which are least likely to pay (C5 2J).
  - C10 3J: auto-answers to employees about pay are hard to gate with an approval step without defeating the purpose.
  - C11 2E.

### HR6 · AI applicant screener for SME job posts (FB groups, JobStreet, Indeed) (51.0)

- **One line:** collect applicants from posts and Messenger, then rank and schedule them.
- **Buyer:** SME owners hiring.
- **Price anchor:** Kalibrr paid tiers (quote-based) [17].
- **Competitors:** Kalibrr AI matching [17]; Talenta AI screening [35]; Nova AI Recruiter in the Sprout Marketplace [31].
- **Scores** `3J 3J 3J 3E 2J 2E 2J 3J 2J 2J 1E 3J 3J 3J 3J 3J`
  - C11 1E (**disqualifying**): ranking applicants is automated decision-making, so NPC registration is mandatory regardless of size [7]; discrimination risk.
  - C6 2E: crowded, with platform AI.
  - C9 2J: job boards offer no applicant API to us (not found).
  - C10 2J: wrongly rejecting a candidate is not reviewable after the fact.

### Ranking (Balanced)

| # | ID | Balanced | Revenue-first | Builder-first | Flags |
|---|---|---|---|---|---|
| — | P1 (reference) | 78.6 | 78.8 | 79.0 | |
| 1 | HR2 | 72.6 | 72.0 | 74.4 | C11 = 2; overlaps P1 |
| 2 | HR1 | 60.4 | 57.8 | 64.4 | C11 = 2 |
| 3 | HR4 | 57.4 | 55.0 | 60.4 | |
| 4 | HR5 | 56.0 | 55.8 | 58.2 | C6 = 1 and C9 = 1 (disqualifying C6) |
| 5 | HR3 | 52.4 | 50.8 | 56.0 | |
| 6 | HR6 | 51.0 | 51.2 | 52.0 | C11 = 1 (disqualifying) |

Scorer-bias caveat as in the scorecard §9: one agent scored its own finds. HR2's 6-point gap to P1 is larger than one-point judgement noise.

## Unresolved questions

1. Do PH freelance bookkeepers routinely run client payroll and chase DTRs, and what do they charge for it? (Not found. This decides whether HR2 is a real P1 expansion. Check r/AccountingPH and bookkeeper FB groups in writing.)
2. Does a reminder- and chase-only tool that touches payroll inputs need BIR CAS registration? Inferred no, but not confirmed in primary text (same open question as P1's kill-risk 2).
3. Primary texts not fetched: SSS Circular 2019-012, PhilHealth Circular 2022-0013, HDMF Circular 275, DOLE LA 06-2020 and 16-2025, and the operative section of NPC Circular 2022-04. Deadlines here come from secondary sources.
4. Payruler, JuanHR, Kayana, GreatDay HR, Salarium: pricing, APIs and AI features not found. Salarium's site was down (HTTP 522) on 2026-10-20.
5. Sprout API: can a non-customer third party get sandbox access, and what are the partner terms? The docs route onboarding through a Customer Success Manager.
6. Zoho Marketplace: the revenue share, Zoho People's PH usage, and whether Zoho's own AI covers the same jobs. Not checked.
7. Deel App Store publication criteria, review time and fees: not visible on the fetched page.
8. PH SME payroll-tool adoption rate: a vendor blog's "40% spreadsheets / 18% manual" figure could not be found on the cited page, so it was **not used**.

## Sources

All accessed 2026-10-20. Type: P primary, V vendor claim, T third-party, F forum. Several entries were read through search-engine summaries rather than full page fetches; those are marked (summary).

1. Platon Martinez, "DOLE Labor Advisory No. 06 Series of 2020: guidelines on payment of final pay and issuance of COE." https://platonmartinez.com/articles/dole-labor-advisory-no-06-series-of-2020-guidelines-on-the-payment-of-final-pay-and-issuance-of-certificate-of-employment — T, H (summary).
2. Manila Bulletin, "Final pay and COE must be released on time, DOLE reminds employers," 2026-01-22. https://mb.com.ph/2026/01/22/final-pay-and-coe-must-be-released-on-time-dole-reminds-employers — T, M (summary).
3. SweldoPH (editor Jerome Ko), "SSS, PhilHealth, Pag-IBIG & BIR Remittance Deadlines 2026 (Employer Guide)." https://sweldoph.com/guides/payroll-remittance-deadlines-2026 — T (free calculator site citing RA 11199, RA 11223, RA 9679, RR 26-2002), M.
4. Manila Bulletin via PressReader, "SSS extends monthly contribution deadline," 2018-11-10. https://www.pressreader.com/philippines/manila-bulletin/20181110/281913069131934 — T, M (summary).
5. Sprout, "How to Submit BIR Form 1604-C and Alphalist in 2026." https://sprout.ph/articles/first-time-bir-1604c-alphalist-filing-guide/ — V, M; Triple i Consulting, substituted filing (RMC 29-2024). https://www.tripleiconsulting.com/tax-blog-requirements-itr-substituted-filings/ — T, M (summary).
6. SunStar, "Firms told: Submit 13th month pay compliance report" (LA 16-2025). https://www.sunstar.com.ph/manila/firms-told-submit-13th-month-pay-compliance-report — T, M (summary).
7. NPC Circular 2022-04. https://privacy.gov.ph/wp-content/uploads/2023/05/Circular-2022-04.pdf — P; thresholds via Baker McKenzie InsightPlus. https://insightplus.bakermckenzie.com/bm/data-technology/philippines-new-npc-circular-on-registration-of-data-protection-officers-and-data-processing-systems-takes-effect_1 — T, H (summary).
8. RA 10173 Data Privacy Act, full text, Lawphil. https://lawphil.net/statutes/repacts/ra2012/ra_10173_2012.html — P, H (Secs. 25–26 fetched); Sec. 3(l)(3) via https://batasnatin.com/laws/ra-10173 and OCA Circular 53-2023. https://oca.judiciary.gov.ph/wp-content/uploads/2023/02/OCA-Circular-No.-53-2023.pdf — P, H (summary).
9. KPMG Philippines, "Special InTAX March 2021" (RMO 9-2021, CAS components incl. payroll). https://kpmg.com/ph/en/home/insights/2021/03/special-intax-march2021-issue1-volume1.html — T, M (summary).
10. Malaya, "DOLE extends anew deadline for annual wage report submission," 2025-09-21. https://malaya.com.ph/news/national-news/dole-extends-anew-deadline-for-annual-wage-report-submission — T, M (summary).
11. Open Exam Prep, "Mandatory DOLE OSH reporting: WAIR, AEDR & timelines." https://open-exam-prep.com/study-guides/ph-dole-cosh-so2/ch7/wair-aedr-reporting — T, L (summary).
12. DTI, "2023 Philippine MSME Statistics" (1,241,733 MSMEs; 109,912 small, 4,763 medium). https://dti.gov.ph/businesses/msmes/msmeresources/msme-statistics — P, H (summary).
13. Sprout, "AI Sidekick." https://sprout.ph/product/ai-sidekick/ ; webinar 2026-01-21 https://events.sprout.ph/on-demand/from-questions-to-confident-action-ai-support-in-everyday-hr-and-payroll-work/ ; https://sprout.ph/artificial-intelligence/ — V, H that the features are claimed (summary).
14. Connext Global, payroll outsourcing cost guide (US$20–260 PEPM). https://connextglobal.com/payroll-outsourcing-philippines-guide-us-firms/ — V, L (summary).
15. Triple i Consulting, "Monthly Government Remittance and Payroll Processing Services," 2026-06-16 (no published price). https://www.tripleiconsulting.com/?p=8715 — V, H (fetched).
16. Moneymax, "Employee registration: BIR, SSS, Pag-IBIG, PhilHealth." https://www.moneymax.ph/blog/employee-registration-bir-sss-pagibig-philhealth — T, M (summary).
17. Kalibrr for employers. https://www.kalibrr.com/employers — V, M; Second Talent, PH job boards 2026. https://www.secondtalent.com/resources/job-boards-to-hire-employees-in-the-philippines/ — T, M (summary).
18. People Dynamics, "Facebook pages to follow for Filipino HR professionals" (HR GUIDE PH ~27,000 likes, undated). https://peopledynamics.co/facebook-pages-to-follow-for-filipino-hr-professionals/ — T, L (summary).
19. BambooHR, Marketplace requirements and application. https://www.bamboohr.com/partner-programs/marketplace-requirements ; https://partners.bamboohr.com/apps-marketplace-requirements-checklist ; https://www.bamboohr.com/partner-programs/apps-marketplace-application — P, H (summary; versions conflict on details).
20. BambooHR press release "BambooHR AI platform," 2025-05-22. https://www.bamboohr.com/about-bamboohr/press-release/bamboohr-ai-platform ; help: https://bamboohr.screenstepslive.com/m/61969/l/1814975-bamboohr-ai — V, H; Bamboo AI beta (July 2026): https://www.usecarly.com/blog/best-ai-automation-tools-bamboohr/ — T, L (summary).
21. Gusto, App Integrations docs, Security Review. https://docs.gusto.com/app-integrations/docs/security-review ; https://docs.gusto.com/app-integrations/docs/introduction — P, H (summary).
22. CPA Practice Advisor, "Gusto introduces 'Gus' AI assistant for small businesses," 2024-09-16. https://www.cpapracticeadvisor.com/2024/09/16/gusto-introduces-gus-ai-assistant-for-small-businesses/110546/ ; https://gusto.com/company-news/gus-ai-assistant — T/V, M (summary).
23. Deel, "Getting started as an API partner." https://developer.deel.com/docs/partner-get-started — P, M (fetched; "40,000+ active Deel customers" is a vendor claim).
24. Deel, "Deel launches AI Workforce" (Aug 2025). https://www.deel.com/blog/deel-launches-ai-workforce/ — V, H; Workplace Journal, 2025-08. https://workplacejournal.co.uk/2025/08/deel-launches-ai-workforce-to-support-hr-and-payroll-teams/ — T, M; Maginative on Deel IQ. https://www.maginative.com/article/deel-launches-ai-powered-hr-assistant-to-simplify-global-hiring/ — T, M (summary).
25. Personio, "First steps towards a product partnership." https://support.personio.de/hc/en-us/articles/360011430518-First-steps-towards-a-product-partnership-with-Personio ; https://personio.com/partner/tech ; https://developer.personio.de/page/marketplace-terms-of-service-agreement — P, M (summary; sources differ).
26. HiBob marketplace tutorials. https://apidocs.hibob.com/docs/marketplace-integration-tutorials ; Merge help. https://help.merge.dev/articles/4654475776-hibob-how-do-i-set-up-partner-credentials — P/T, L (summary).
27. Rippling for developers. https://www.rippling.com/developers ; Merge guide. https://help.merge.dev/articles/9265388-rippling-how-do-i-set-up-my-integration — P/T, M (summary).
28. Zoho Marketplace FAQ. https://www.zoho.com/developer/help/marketplace/faqs.html — P, M (summary).
29. Sprout, "Outsourced Payroll Services" (₱7,900/mo for 10 employees). https://sprout.ph/product/payroll-outsourcing/ — V, M; FitGap. https://us.fitgap.com/products/014145/sprout-solutions — T, L; Akrivia HCM estimate. https://akriviahcm.com/blog/sprout-alternatives-philippines-2026 — T, L (summary).
30. Sprout developer portal. https://developers-sb.sprout.ph/get-started ; https://guide.sprout.ph/sprout-developer-portal ; https://api-docs-partners.sprout.ph/view/metadata/2sB34eGgav — P, M (summary).
31. Sprout Marketplace. https://market.sprout.ph/ — P, H (fetched); Partner program. https://sprout.ph/partner-with-sprout/ — V, M (summary).
32. KAMI Workforce pricing. https://kamiworkforce.com/pricing — V, H (summary of pricing page).
33. G2, PayrollHero pricing. https://www.g2.com/products/payrollhero/pricing — T, L; SoftwareSuggest. https://www.softwaresuggest.com/payrollhero — T, L (summary).
34. TrustRadius, Salarium pricing. https://www.trustradius.com/products/salarium/pricing — T, L (summary).
35. Mekari Talenta (SG site). https://www.talenta.co/en-sg/ — V, M (summary).
36. PeerPush, SmartSalaryTool. https://peerpush.com/p/smartsalarytool — V, L (summary).
