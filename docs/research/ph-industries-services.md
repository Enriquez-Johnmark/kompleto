# PH industries sweep — services & professional

Status: **complete, 2026-10-18.** Scope: services and professional industries, screened against the constraints in [opportunity-scorecard.md](opportunity-scorecard.md) §7–8 (no network, no sales calls, no paid ads, ≤ ₱600/mo, free channels, Rust agent core). Already-researched ideas are skipped (see [opportunity-landscape.md](opportunity-landscape.md)). Source tags such as [A7] refer to the Sources list at the end. Facebook group sizes and Reddit threads were mostly unreachable by the search tools, so first-hand pain evidence is thin throughout.

## Key conclusions

1. **Accounting and bookkeeping is the best fit in this sweep.** The buyer is a bookkeeper or small CPA firm. One sale covers 10–60 SMEs, and each SME already pays ₱2,500–15,000/mo for bookkeeping [A11][A12].
   - The work is a durable, per-client calendar of BIR deadlines, plus repeated written chases for receipts and **2307 certificates** owed by third parties [A5][A7].
   - That is scheduled work, extraction, drafting and human approval: the agent core exactly.
   - **The gap:** PH tools do bookkeeping but not chasing (Built for Bookkeepers ₱999/mo [A13]; Juan ₱2,000/org [A14]). Global chasers do chasing but not BIR or 2307s (DocChase $12–49/mo [P1-1]; BilagPilot, Sep 2026 [P1-2]).
   - It is materially different from S1 (the seller tax desk): a different buyer with ~10–60× the clients per account. It could later feed S1.
   - _(Confidence M. Pain quotes are vendor claims; no first-hand forum evidence was reachable.)_
2. **BPO/IT-BPM is big and changing, but closed to a solo vendor.**
   - IBPAP cut its 2028 outlook from $59B / 2.5M to $43–50.5B / 1.85–2.14M, citing AI. It now targets "AI-enabled" workers measured by revenue per worker [B1][B2].
   - QA and agent-assist tools sell into enterprise procurement and SOC 2 requirements [B19].
   - The VA end-of-day-report drafter already ships at Magic with the same approval pattern [B12].
3. **DMW recruitment paperwork is an unoccupied niche, but the data is toxic for a solo founder.**
   - 2.7M deployments in 2025 [M1], and no AI tool aimed at agency document checklists was found.
   - But it handles medical, NBI and passport data (C11 = 1) and needs cold outreach to licensed firms.
4. **Insurance agents (233,736 IC licence rows [I1]) and lenders/co-ops are not good targets.**
   - Insurers give agents free tools, including AI [I8][I9].
   - Co-op cores already bundle SMS payment reminders [I10].
   - Collections are tightly regulated (SEC MC 18 s.2019 [I14], NPC 20-01 [I15]) and reputationally tainted by online-lending harassment [I16].
   - Agent follow-up duplicates the earlier candidate B (lead/quote follow-up).
5. **HOA/condo administration is real but cheap.**
   - 26,000+ HOAs [R4], and dues collection is DHSUD's named top problem.
   - Four PH tools already sell at ₱5–35 per household per month, none with AI [R11–R14].
   - It is a weak standalone product. It fits as a later vertical for a generic "dues and arrears" reminder core, as do private-school arrears after RA 11984 [O6].
6. **Constraint fit, top pick.** P1 needs ~₱0 to start: free tier of an email provider, SMS at ₱0.56 per credit only when used [INT].
   - **Free channels:** a public "BIR deadline + 2307 tracker" page, Taglish posts in tax/bookkeeping FB groups and r/AccountingPH.
   - **Viber/Messenger:** the bookkeeper sends from their own account via "copy & open". That avoids Viber bot fees (EUR 100/mo) and Meta's messaging-window rules [INT].

| Candidate | Score string (C1..C16) | Balanced | Revenue-first | Builder-first |
|---|---|---|---|---|
| **P1 Bookkeeper document/2307/deadline chaser** | 4E 4E 4J 4E 4E 4E 3E 4J 4E 4J 3J 4J 4J 5E 4J 4J | **78.6** | 78.8 | 79.0 |
| P2 DMW agency applicant-document desk | 4E 4J 4J 3E 3E 4E 2E 3J 2E 3J 1E 3J 3J 4J 3J 4J | 63.4 | 64.2 | 63.2 |
| P3 HOA/condo dues & notices clerk | 3E 3J 3J 4E 2E 3E 2J 4J 3E 3J 3E 4J 3J 4J 3J 4J | 62.4 | 61.2 | 65.2 |

For reference, the scorecard's current #1, S1, scores Balanced 74.4. These scores come from a separate pass, so P1's lead over S1 is within calibration noise. Treat P1 as **co-leader with S1**, not a clear winner. The two share most of the build: BIR calendar, 2307 extraction, Taglish drafting.

## 1. BPO / IT-BPM and VA/freelancer ecosystem

**Size.**
- **IT-BPM:** IBPAP reports 2025 at US$40.3B revenue and 1.9M FTEs. It projects 2026 at $42.3B / 1.96M [B1].
- **IBPAP cut its 2028 outlook in July 2026:** from the 2022 roadmap's $59B / 2.5M down to a $43.3–50.5B / 1.85–2.14M range. Reasons given: AI, and geopolitics slowing buyer decisions [B1][B2].
  - Its Executive Director said "AI has affected some entry-level jobs" but those workers were redeployed [B1].
  - The new frame is "AI-enabled Digital Filipino Workers", measured by revenue per worker rather than headcount [B2].
  - Earlier, IBPAP said AI's impact would be 80–90% job transformation and 10–20% automation [B3].
- **Online freelancers:** no official PSA/DICT count. Private estimates run ~1.3–1.5M on international platforms (Payoneer/GCash, PayPal, Taxumo) [B4][B5]. Outsource Accelerator estimates ~1M VAs (via AFP) [B6].
- **Firms:** PSA counted 2,147 IT-BPM establishments in 2019 [B7]. IBPAP has 400+ members [B8].

**Repetitive written workflows.**

1. **QA scoring of calls and chats.**
   - Manual QA reviews roughly 1–5% of interactions (vendor claims) [B9].
   - PH job ads show QA analysts scoring against scorecards, with monthly audit targets and calibration, at ₱28–40k/mo [B10].
2. **VA end-of-day (EOD) reports to clients.**
   - Described as the VA's "most important daily communication" (vendor blog) [B11].
   - **Magic, a VA company, already ships the exact approval-gated pattern.** AI drafts the EOD report from tracked tasks, and the VA edits or regenerates it before publishing it to the client [B12].
3. **Time tracking / activity reports** for client invoicing (Hubstaff, Time Doctor) [B13].

**Pain and payment.**
- **AI QA tools:** EvaluAgent publishes US$35/user/mo for AutoQA, or $65 with conversation intelligence [B14]. Zendesk QA is ~$35/agent as an add-on [B15]. Observe.AI, MaestroQA and Level AI are quote-only [B14][B16].
- **Trackers:** Hubstaff Team is $10/seat [B13]. Time Doctor Premium is ~$16.70–20, with an "Unusual Activity AI report" (third-party pricing) [B17].
- **EOD/standup tools** are cheap: Standuply from ~$1.50/user [B18].
- No first-hand Reddit or FB threads on EOD or QA pain were found. The search tool could not reach them.

**Gatekeeping.**
- **BPOs:** US clients demand SOC 2 / ISO 27001 from PH vendors [B19]. NPC Advisory 2024-04 on AI applies to processors, so it covers BPOs [B20]. Call recordings are customer data that the client owns. In practice, an unknown solo vendor cannot get a QA tool past a BPO's procurement. **Gatekeeping is high.**
- **Freelance VAs:** much lighter, since they are self-employed. But their data still belongs to their clients.

**Free distribution.**
- Filipina Homebased Moms (FHMoms) has "nearly 500,000" members (undated) [B21]. Facebook is the largest job source for PH freelancers (Payoneer 2022) [B22]. Other named groups have no public counts.
- Reach is excellent. But VAs are price-sensitive individuals paying out of pocket.

**Verdict.**
- **QA for BPOs:** enterprise procurement plus SOC 2 means no fit with the constraints.
- **VA EOD/report drafter:** reachable, but Magic already ships it and Slack/standup tools are cheap. The buyer, a VA earning in USD, would pay little, and the client sees the value, not the VA.
- **VA *agency* owner** (5–50 VAs) reporting to many clients: closer to a fit, but agency counts were not found and the problem overlaps with "freelancer admin" (rejected earlier).

**Not a top pick.** _(Confidence M.)_

## 2. Accounting & bookkeeping firms

**Size.**
- **CPAs:** PICPA claims "200,000+ CPAs" (cumulative) and 34,000+ active members, in 84 local chapters [A1].
- **New CPAs per exam:** about 3,000 (3,004 of 9,745 in May 2026) [A2].
- **Not found:** the count of currently registered CPAs and of BOA-registered firms.
- **PSA (formal sector only):**
  - 607 accounting, bookkeeping, auditing and tax-consultancy establishments in 2018 [A3].
  - 30,355 employed in 2019, 17,914 in 2020 [A4].
- These counts badly undercount home-based freelance bookkeepers. No estimate of that group was found.

**Repetitive written / document workflow.** A bookkeeper with 10–60 SME clients runs, per client:
- **Monthly and quarterly forms:** 0619-E, 1601-EQ, 2550Q/2551Q, 1701Q. Each has its own deadline, eFPS group staggering, and frequent RMC extensions [A5][A6].
- **Document collection:** sales and expense receipts, GCash screenshots, bank statements.
- **2307 certificates:** the client's *customers* must issue them within 20 days after quarter-end (RR 11-2018). Without them the client cannot claim the tax credit. The standard remedy is a written demand to the customer [A7].
  - The EOPT Act regulations (RR 3/4-2024) keep the 2307 as the proof of withholding, and make withholding due when the payment is due or invoiced, whichever comes first [A8].
- **New 2026 load:**
  - e-invoicing (EIS) compliance, extended to 31 Dec 2026 for covered taxpayers [A9];
  - eAFS system problems that forced a deadline extension [A10].

**Pain and payment.**
- **Bookkeepers already charge per client:**

  | Client type | Monthly fee |
  |---|---|
  | Home-based / freelancer clients | ₱2,500–5,000 |
  | Small businesses | ₱5,000–8,000 |
  | SMEs | ₱8,000–15,000 |

  These are firm blogs, 2025 [A11][A12]. A 30-client bookkeeper bills ~₱75k–240k/mo, so a ₱1,000–2,000/mo tool is 1–2% of revenue.
- **Pain quotes are vendor claims only:** "SME owners forward GCash screenshots and receipt photos… By month-end, dozens are missing"; "Managing 10+ clients means 10+ Excel files" [A13].
- **PH paid tools:**
  - **Built for Bookkeepers:** ₱999/mo per firm, unlimited clients. AI classification with a review queue where nothing posts without approval; generates the four BIR books. Its page does **not** mention 2307 tracking, deadline tracking, or Viber/Messenger document chasing [A13].
  - **Juan Accounting Essentials:** ₱2,000/org/mo, with CPA-firm discounts; 43 partner firms are listed [A14][A15].
  - **Taxumo:** an accountant can reach many taxpayers from one login, but each taxpayer needs its own subscription (from ₱1,374/quarter) [A16].
- **Global practice-management and document-chasing tools:**

  | Tool | Price |
  |---|---|
  | Content Snare | US$35–215/mo [A17] |
  | Financial Cents | $19–69/user/mo [A18] |
  | TaxDome | ~$800–1,200/user/yr (third-party) [A19] |
  | Karbon | ~$59–89/user/mo (third-party) [A20] |

  None is PH-specific: no 2307s, no BIR calendar, no Viber/Messenger.

**Gatekeeping.**
- **CAS/CBA:** software that *keeps the books* must be registered by each taxpayer (Acknowledgement Certificate, RMO 9-2021) [A21].
- **eTSP:** software that *e-files* needs BIR eTSP certification [A22].
- **What stays outside both** (inference, not a sourced ruling):
  - a tool that only chases documents;
  - tracks 2307s and deadlines;
  - drafts messages;
  - leaves the bookkeeper's existing software and eBIRForms/eFPS to keep books and file.
- **Privacy:** NPC Circular 2022-04 registration applies only above its thresholds [A23]. Tax data is sensitive *commercially*, but not health-grade.

**Free distribution.**
- r/AccountingPH: 37k–73k members in undated snapshots [A24].
- The FB page "Freelance Accountants & Bookkeepers Philippines" has ~112k followers. It is a training business aimed at offshore clients, so it is not a peer group [A25].
- QuickBooks PH names four PH tax/accounting FB groups; member counts were not found [A26].
- Bookkeepers are **self-identified, concentrated and searchable**: "bookkeeper 2307", "BIR deadline" are Taglish SEO topics. And **one bookkeeper brings 10–60 SMEs**.

**Verdict: the strongest industry in this sweep.** It is a B2B buyer with existing per-client revenue, a recurring calendar of deadlines, many written chases per month, a local regulatory twist (2307, BIR calendar) that global tools ignore, and an identifiable cheap local competitor that leaves the chase/2307/deadline gap open. → **Candidate 1.** _(Confidence M. Pain is vendor-claimed, not first-hand.)_

## 3. Recruitment / manpower / DMW-licensed agencies

**Size.**
- **Deployment:** more than 2.7M OFWs in 2025, a record and up 15.4% on 2024. About 2.15M were land-based and 589k sea-based (preliminary DMW figures, via BusinessMirror) [M1]. Sources conflict on the 2024 base. One reports 2.66M for Jan–Sep 2024 alone [M2], most likely a deployments-versus-OECs counting difference.
- **Agencies:**
  - Active DMW-licensed agency count, land/sea split: **not found**.
  - DMW's archive list has 3,797 records, but these include closed, banned and suspended agencies [M3].
  - The last clean split found is from 2012: 1,283 agencies (ILO) [M4].
  - PASEI, the largest land-based association, has "around 220" members (secondary source) [M5].
  - DOLE DO 174 contractors and local PRPAs: current count **not found** [M6].
- **Agency share of hiring:** agencies made 92–94% of hires in 2008–2010 [M4].

**Repetitive document workflow.** Each applicant needs a stack of documents:
- passport, PSA birth certificate, NBI clearance, diploma, transcript and CV;
- licences, prior contracts and a medical exam;
- PEOS, then e-Registration, then PDOS;
- the OEC (OFW Pass now covers returning workers only) [M7][M8].

Migrant Workers Offices (MWOs) abroad reject incomplete agency-hire submissions. MWO Dubai requires two complete sets of documents [M9]. MWO Canberra changed its checklist with effect from 1 July 2026 [M10]. Agencies process through DMW web portals (LBECS / e-Services). **There is no public API** [M11][M12].

The agent-shaped work:
- extract and validate each applicant's documents against a per-country, per-employer checklist;
- track expiries (NBI, medical, passport validity);
- draft Taglish follow-ups to applicants for missing items;
- assemble the packet.

**Pain.**
- Deployment delays get news coverage: welders delayed ~6 months over visas [M13]; recruiter groups complained about slow processing [M14]; DFA blamed recruiters for apostille queues [M15].
- Applicant-side complaint volume: **not found**.

**Payment.**
- **Recruiting and applicant-tracking software (generic):**

  | Tool | Price |
  |---|---|
  | Manatal | US$15–55/user/mo [M16] |
  | Recruit CRM | $95–215/user/mo, with AI agents and an MCP connector [M17] |

- **JobStreet:** free "Lite Ads" since 2024 [M18]. Its seekAi tool (launched in Manila Sep 2026) covers sourcing, ranking, voice screening and reference checks [M19]. JobStreet reports that 39% of PH employers already use AI in recruitment [M19].
- **PH crewing/DMW-processing software with public prices:** **not found**.
- **AI aimed at the DMW-compliance paperwork:** **not found**. The niche looks empty, possibly because it is hard.

**Gatekeeping (heavy).**
- **Sensitive personal information.** Medical results, NBI clearances and government-issued records are sensitive personal information under RA 10173 Sec 3(l) [M20]. Most agencies likely cross NPC Circular 2022-04's threshold of 1,000 data subjects with sensitive personal information, and a SaaS vendor would be their processor [M21].
- **Advertising.** Agencies must register their official social-media accounts with DMW (Advisory 1 s.2025) [M22]. Personal-FB "agents" posting jobs are treated as illegal recruiters [M23].
- **DMW systems.** They were taken offline by ransomware in July 2024 [M24]. Any integration is screen-scraping a government portal, which is not acceptable.

**Free distribution.**
- DMW's public licensed-agency search gives contact details and licence status [M3][M25]. The Approved Job Orders database shows which agencies are active [M26].
- That makes a *written* outreach list easy to build. But that is cold outbound email, which is borderline against the "no sales" constraint.
- No agency-owner FB groups were found.
- DMW is courting agencies on digital accreditation (Aug 2026) [M27].

**Verdict.** A real, unoccupied niche with high volume. But the buyer is a licensed firm reached by cold contact, and the data is the most sensitive in this sweep. → **Candidate 2 (scored, not top pick).** _(Confidence M.)_

## 4. Insurance agents, lending companies, cooperatives, rural banks

**Size.**

| Group | Count | Source / confidence |
|---|---|---|
| Life-agent licence rows on the IC list (20 Nov 2025) | **233,736** | [I1] H for row count |
| — Pru Life UK | ~75.5k | [I1] M |
| — Sun Life | ~48.1k | [I1] M |
| — AIA | ~22.7k | [I1] M |

- The IC rows are licence entries. One person with both traditional and variable-life licences appears twice, so the number of unique agents is lower. The per-insurer split above was counted by this sweep, not published by the IC.
- **Top producers:** Sun Life PH had 444 MDRT qualifiers (Million Dollar Round Table, the global top-producer club) in 2025 [I2]. AIA PH plus BPI AIA had 458 in 2026 [I3].
- **Cooperatives:** 20,962 registered, including 2,874 credit co-ops (2024 CDA figures, cited secondhand) [I4].
- **SEC lending companies:** headline count not found. The SEC revoked 401 lending and 56 financing companies in May 2025 for not filing reports [I5].
- **Rural and cooperative banks:** 374 as of end-July 2025 (Malaya citing BSP) [I24]. Mergers are under way, ahead of minimum-capital rules for 2027 [I6].

**Repetitive workflows.**
- **Insurance agents:**
  - premium-due and lapse follow-ups (insurer job ads list "follow up overdue accounts and prevent policy lapses") [I7];
  - policy-anniversary reviews;
  - birthday messages;
  - prospect follow-up on Messenger.
- **Lenders and co-ops:** payment reminders, promise-to-pay tracking, demand letters.

**Pain and payment.**
- **Insurers supply tools free:**
  - Sun Life gives advisors Google Workspace [I8];
  - AIA markets "AI Roleplay" adviser training [I9].
- **No PH advisor CRM with peso pricing was found.** Agents' own tool spending: **not found**.
- **Co-ops already have reminder tooling bundled:**
  - NATCCO's eKoopBanker runs in 623 sites for 269 co-ops, with a TextBlast facility for "payment obligations" reminders [I10];
  - WeSupport CoopVault has 26 SMS/email templates and promise-to-pay tracking [I11];
  - Loandisk is ~US$59/mo+ (listing) [I12].
- **Informal "pa-utang" lenders (small-scale lending) pay ₱249 for a spreadsheet template** [I13]. That is a signal of very low willingness to pay.

**Gatekeeping (heavy for lending).** Correction to the brief: BSP Circular 1133 (2021) caps interest and fees on small loans. It is not a debt-collection rule [I25].
- **SEC MC 18 s.2019:**
  - no contact before 6am or after 10pm unless more than 15 days past due or the borrower consented;
  - no contacting anyone in the borrower's contact list except named co-makers or guarantors;
  - no disclosure of borrower data;
  - fines from ₱25k–50k [I14].
- **NPC Circular 20-01** (amended by 2022-02) binds lenders *and their processors* [I15].
- **Online lending apps** remain a national harassment problem. PAOCC reportedly logged 47,446 complaints from Aug 2024 to Jan 2026 (via snippet) [I16].
- **Insurance:**
  - agents must label themselves IC-authorised in public materials (CL 2015-12) [I17];
  - e-commerce selling needs a licence, and apps need IC approval (CL 2014-47, CL 2021-66) [I18][I19];
  - the IC has not yet regulated AI [I20].

**Free distribution.**
- **Insurance agents:**
  - LUAP has ~6,000 members (2017, stale) [I21];
  - "Pinoy Insurance Talk" had ~17k members (c.2017) [I22];
  - agency/unit FB groups are private and run by insurers.
- **Lenders and co-ops:**
  - NATCCO federates 1,080 co-ops [I23], but it sells its own software;
  - no association of small lending companies was found.

**Verdict.**
- **Insurance agents:** huge headcount, but they expect free tools from their insurer. Client data is financial and health-adjacent. The follow-up job is B (lead/quote follow-up) again, so it is not materially new.
- **Lending and co-op collections:** regulated, incumbent-bundled, and reputationally toxic in PH because of online-lending-app harassment.
- **No candidate from this industry.** _(Confidence M.)_

## 5. Real estate brokers & property / condo / HOA administration

**Size.**

| Group | Count | Source / confidence |
|---|---|---|
| Licensed brokers and accredited salespersons (national) | **Not found**. PRC publishes exam results only | — |
| New brokers, April 2026 exam | 1,863 passed of 2,296 | [R1] H |
| New brokers, April 2025 exam | 1,306 passed of 1,741 | [R2] H |
| Brokers in a third-party verification database (from PRC records since 2010) | 25,264 | [R3] L–M |
| Registered HOAs and neighbourhood associations | 26,000+ (DHSUD, Jan 2026) | [R4] H |
| Condominium corporations | Not found | — |
| Property-management firms | Not found | — |

**Workflows an approval-gated agent could take over.**

*HOAs and condos:*
- monthly dues statements;
- delinquency reminders and demand letters;
- notices of meetings and elections;
- maintenance and concern tickets.

DHSUD says HOA complaints are "usually related to the elections and terms of office of its officers and the collection of fees" [R4]. Collection has legal steps:
- **HOAs:** penalties must be in the bylaws, with notice and hearing. Cutting basic services is barred [R5][R6].
- **Condos:** the corporation can register a lien and foreclose under RA 4726 [R7].

A one-off compliance deadline is live. HOAs once registered with SEC/HIGC must re-register with DHSUD by **18 Dec 2026** or be automatically suspended [R8].

*Brokers:*
- posting listings across FB, Lamudi and similar portals;
- lead follow-up on Day 1, Day 3 and Day 7, then monthly (vendor advice) [R9].

OnlineJobs.ph posts show this posting and lead-logging work is outsourced at ₱8,000/mo plus per-lead bonuses [R10].

**Pain and payment evidence.** HOA software already exists in PH at very low prices. None of these vendors mentions AI [R11–R14].

| Tool | Price | Features |
|---|---|---|
| HOAS | ₱5–7 per property per month; Growth ₱350/mo for 50 properties | SOAs, penalties, tickets |
| HOAMS | ₱15/25/35 per household per month | PayMongo, reminders |
| MyHOA | Free early access | Claims an 84% collection rate (vendor claim) |
| HOACentralPH | Not published | GCash/Maya payments |

Global comparators:
- TownSq: US$90–435/mo [R15].
- Buildium: from US$62/mo; HOA pricing on request [R16].

No forum or Reddit evidence of dues pain was found. Broker-side prices for portal packages were not found.

**Gatekeeping.**
- **RESA (RA 9646):** salespersons must work under a licensed broker (max 20 per broker). Advertising property counts as "practice" [R17][R18]. Enforcement against unlicensed "kolorum" agents tightened in 2026 [R19].
- **Privacy:** an NPC decision exists on an HOA that allegedly shamed delinquent owners [R20]. So delinquency notices must be private, one-to-one, and never posted in group chats. An approval gate helps here.

**Free distribution.**
- **Brokers:** REBAP has 54 chapters and its own FB page [R21]. PAREB's MLS directory lists 787 entries [R22]. FB group sizes are not indexable; one 54.3K-member PH real-estate group was listed for sale [R23].
- **HOAs:** DHSUD publishes an HOA list by region [R24], but it returned 403 and was not verified. HOA officers are volunteers, with no strong association to reach them through.

**Market.** Metro Manila unsold condo stock hit a record 82,900 units in Q2 2026. Rents per sqm fell 18–59% outside BGC (Leechiu via GMA) [R25]. Brokers and condo-owner landlords are under pressure. That pressure raises the value of follow-up and lowers the budget for paying for it.

**Verdict.**
- **HOA/condo dues & notices agent:** plausible. The incumbent tools are cheap CRUD with no drafting or approval workflow. But price anchors are tiny (₱5–35 per household per month), and there is no proof volunteer officers pay.
- **Broker follow-up:** overlaps the already-researched B (lead/quote follow-up) and is not materially new.

_(Confidence M.)_

## 6. Other service industries

Two further industries were screened briefly. Neither beat sections 1–5.

**Customs brokers.** PRC licenses customs brokers by exam. The November 2025 exam passed 2,238 of 3,078 takers [O1]. The total registered base was not found. The workflow is document-heavy: invoices and packing lists become an import entry. But entries must be lodged into the BOC e2m system through one of a few BOC-accredited Value-Added Service Providers (VASPs), and all traffic passes a single BOC gateway [O2][O3]. Valuation is closely scrutinised [O4]. An agent can only *pre-fill* data for a broker's existing VASP tool, and there is no public API to that tool. Customs brokers are also a small, closed professional community. **Verdict: gated; skip.** _(Confidence M.)_

**Private schools (tuition arrears).** Private basic-education enrolment was 2.63M in SY 2024–25 (DepEd via news) [O5]. RA 11984 (signed March 2024) bars schools from denying exams to *disadvantaged* students with unpaid fees. Schools keep the right to require promissory notes, withhold credentials and pursue collection [O6]. COCOPEA argued that bans on "no permit, no exam" threaten school cash flow [O7]. That makes arrears reminders and promissory-note tracking plausible pain. But no PH school-billing vendor with public prices was found, only academic prototypes and HitPay's per-transaction billing [O8]. Buyers are school administrators, reached by school associations rather than open FB groups. Data concerns minors. **Verdict: interesting, but evidence is thin. Park it as a possible vertical for a generic "dues/arrears reminder" core (see Candidate 3).** _(Confidence L–M.)_

## Candidate deep-dives

Scores are on the 16 criteria of [scoring-method.md](scoring-method.md), in order C1..C16. 5 is always favourable. E = evidence-backed, J = judgement (capped at 4). Weighted totals were computed with the method's three weight profiles.

**Calibration caveat:** these were scored in a separate pass from [opportunity-scorecard.md](opportunity-scorecard.md). Treat a gap of 1–4 points against that file as noise.

### Candidate 1 (P1): "Kulang" (missing documents) desk: a per-client deadline, document and 2307 chaser for PH bookkeepers

**Problem.** A freelance bookkeeper or small CPA firm with 10–60 SME clients works to a per-client BIR calendar:
- 0619-E, 1601-EQ, 2550Q/2551Q, 1701Q, with staggered eFPS deadlines and frequent RMC extensions [A5][A6][A10].

Each deadline depends on documents the client has not sent:
- receipts, GCash screenshots, bank statements;
- 2307s that the client's *customers* owe within 20 days of quarter-end [A7].

Chasing happens by hand in Viber and Messenger threads and in per-client Excel files. A vendor describes it as "Receipts arrive via Viber at midnight… By month-end, dozens are missing" and "Usually done the night before the deadline" (vendor claim) [A13]. A missing 2307 means the client loses the tax credit. A late return costs at least a ₱1,000 compromise penalty plus a 25% surcharge (see new-ideas-philippines.md [30]).

**Buyer.** The bookkeeper or firm, not the SME. One sale covers 10–60 businesses. Bookkeepers bill ₱2,500–15,000 per client per month [A11][A12].

**Evidence of payment.**
- **PH bookkeepers already pay for tooling:**
  - Built for Bookkeepers: ₱999/mo per firm [A13].
  - Juan Accounting Essentials: ₱2,000/org/mo, with CPA-firm discounts [A14].
  - Taxumo: per-taxpayer subscriptions [A16].
- **Document chasing is a paid product category abroad:**
  - DocChase: US$12–49/mo (vendor's launch comment) [P1-1].
  - BilagPilot: launched Sep 2026; Xero/QBO only; email/SMS [P1-2].
  - Content Snare: $35–215/mo [A17].
  - Financial Cents: $19–69/user [A18].

**Competitors and gap.**

| Tool | Market | Covers | Missing |
|---|---|---|---|
| Built for Bookkeepers | PH | AI categorisation, approval queue, BIR books | Document *requests*, reminders, 2307 tracking, deadline calendar [A13] |
| DocChase, BilagPilot, Content Snare | Global | Generic chasing | BIR calendar, 2307s, Taglish; mostly tied to Xero/QBO |

- Juan and Taxumo are filing tools, not chase tools.
- **Risk:** Built for Bookkeepers or Juan adds a "request documents" button. The defence is the 2307 ledger and Taglish drafting, which is a feature set, not a moat.

**Agent workflow (approval points marked ✋).**
1. **Onboarding.** The bookkeeper adds clients with:
   - tax type (VAT / non-VAT / 8%);
   - eFPS group or eBIRForms;
   - each client's main customers who withhold.

   The system builds a durable per-client calendar of forms, due dates and needed documents.
2. **Scheduled reminders.** N days before each internal cut-off, the agent drafts a Taglish/English request listing exactly what is missing for that client: "Hi Ma'am Liza, para sa Q3 2550Q namin, kulang pa po: (1) Sept bank statement, (2) 2307 from ABC Trading…" ("for our Q3 2550Q, we're still missing…"). ✋ The bookkeeper approves, edits or snoozes it, in a batch across clients.
3. **Sending.**
   - Approved requests go by email from the product's domain and by SMS (Semaphore, ₱0.56/credit [INT]).
   - Viber and Messenger get a one-tap "copy & open chat" handoff. The bookkeeper sends from their own account, so there is no Viber bot fee (EUR 100/mo [INT]) and no Meta messaging-window rules.
   - Each request carries a no-login upload link.
4. **Intake and extraction.** The LLM reads uploads (2307 PDF/photo, receipts) and extracts:
   - payor TIN and name, period, amount and tax withheld;
   - it matches each 2307 to the expected customer and quarter.

   ✋ Low-confidence extractions are queued for review.
5. **2307 ledger.** Per client, per customer, per quarter: received / missing / mismatch. 20 days after quarter-end, the agent drafts a polite written request *from the client to their customer* [A7]. ✋ Both bookkeeper and client approve before anything is sent.
6. **Escalation and summary.**
   - Scheduled re-nudges, with idempotent sends.
   - A weekly "who's blocking which deadline" digest.
   - An export of the 2307 summary into the bookkeeper's existing software.

**Integrations.** Postgres, object storage, an email provider (own domain, no Gmail restricted scopes [INT]), Semaphore SMS, and an LLM API. No BIR integration: the agent never files, so CAS/eTSP do not apply (inference [A21][A22]). Optional later: Xero/QBO read.

**MVP size (one Rust developer).**
- ~5–7 weeks to a usable beta. Calendar engine + client model ~1.5 weeks. Drafting/approval queue ~1.5 weeks. Upload links + 2307 extraction ~2 weeks. Email/SMS/handoff ~1 week.
- A demonstrable slice in ≤4 weeks: calendar + chase drafts + approval + email.

**Risks.**
- Pain evidence is vendor-claimed only.
- Bookkeepers may prefer their existing Viber habits, so the "copy & open" handoff carries the UX.
- The BIR calendar changes often and needs maintenance; that is also the moat.
- Tax data is confidential; the product needs a DPA-grade privacy notice.

**What would disprove it.**
- Fewer than ~10 sign-ups from a free "BIR deadline + 2307 tracker for bookkeepers" page plus posts in r/AccountingPH and PH tax FB groups within 6 weeks.
- Or beta users who approve fewer than half of drafted chases, meaning the drafts are not useful.
- Or bookkeepers telling us in async written replies that clients ignore any channel other than their personal Viber.

**Score (C1..C16):** `4E 4E 4J 4E 4E 4E 3E 4J 4E 4J 3J 4J 4J 5E 4J 4J`. Balanced **78.6**, Revenue-first 78.8, Builder-first 79.0. Judgement share 8/16.

C7 is 3E because the free channels exist (r/AccountingPH; tax FB groups named by QuickBooks PH) but their sizes are low-confidence or not found [A24][A26].

### Candidate 2 (P2): OFW applicant-document desk for DMW-licensed agencies

**Problem.** Agencies collect 10+ documents per applicant against country- and employer-specific checklists. MWOs reject incomplete packets [M9][M10], and deployments slip [M13].

**Buyer.** Documentation officers at DMW-licensed land-based agencies.

**Evidence of payment.**
- Generic applicant-tracking software: Manatal $15–55/user [M16], Recruit CRM $95–215/user [M17].
- No priced DMW-specific tool was found.

**Competitors.**
- JobStreet seekAi does sourcing and screening, not deployment paperwork [M19].
- In-house systems at large agencies [M3].
- **Gap:** checklist/expiry tracking plus applicant follow-ups. Nobody sells it.

**Workflow.**
1. An applicant uploads documents by link.
2. The LLM classifies each document, extracts expiry dates, and checks it against the job-order checklist.
3. The agent drafts Taglish follow-ups for missing or expiring items. ✋ The officer approves.
4. Durable reminders run until the packet is complete.
5. The officer marks the packet ready for DMW e-Services. Submission is manual; there is no API [M11][M12].

**MVP.** ~6–8 weeks.

**Risks.**
- **Health and identity data** (medical results, NBI, passport) under RA 10173. The vendor becomes a processor for agencies that likely must register with NPC [M20][M21].
- Cold outreach to agencies.
- The agency's licence liability makes buyers cautious.

**What would disprove it.**
- No replies from the first ~50 written emails to agencies picked from the DMW directory.
- Or agencies saying in writing that their staff already track this in Google Sheets and won't pay.

**Score:** `4E 4J 4J 3E 3E 4E 2E 3J 2E 3J 1E 3J 3J 4J 3J 4J`. Balanced **63.4**, Revenue-first 64.2, Builder-first 63.2. **Disqualifying 1 on C11** (health and identity data).

### Candidate 3 (P3): AI dues-and-notices clerk for self-managed HOAs and small condo corporations

**Problem.** Fee collection is one of DHSUD's two top HOA complaint areas [R4]. Delinquency notices carry legal and privacy traps: penalties need bylaw basis plus notice and hearing; services cannot be cut; shaming delinquent owners drew an NPC case [R5][R6][R20]. On top of that, the 18 Dec 2026 re-registration deadline [R8] is a one-off spike.

**Buyer.** Volunteer HOA treasurers and secretaries, or small property managers.

**Evidence of payment.** HOAS, HOAMS, MyHOA and HOACentralPH sell dues/SOA/reminder software at ₱5–35 per household per month [R11–R14]. None uses AI.

**Workflow.**
1. Import the dues ledger (CSV).
2. The agent drafts monthly statements of account (SOAs) and graduated, *private* reminder letters, with bylaw-cited penalty language. ✋ An officer approves each batch.
3. Scheduled sends by email and SMS.
4. The LLM classifies and routes concern tickets.
5. Meeting notices and minutes drafts. ✋ Approval before sending.

**MVP.** ~4–6 weeks.

**Risks.**
- Price anchors are tiny.
- Volunteer buyers.
- Four incumbents already own the ledger.
- No forum evidence of pain was found.

**What would disprove it.** An HOA officer group post plus a free "HOA demand letter generator" page yields fewer than 5 paying HOAs at ₱500+/mo.

**Score:** `3E 3J 3J 4E 2E 3E 2J 4J 3E 3J 3E 4J 3J 4J 3J 4J`. Balanced **62.4**, Revenue-first 61.2, Builder-first 65.2.

## Sources

All accessed 2026-10-18. Format: publisher, title, URL, publication date, evidence type, confidence (H/M/L). Vendor pages are vendor claims unless stated. "via snippet" means the sweep read only the search-result excerpt.


**BPO / IT-BPM / VA**

- [B1] Philstar, "IT-BPM industry lowers outlook", https://www.philstar.com/metro/2026/07/15/2542137/it-bpm-industry-lowers-outlook, 2026-07-15, news citing IBPAP, H
- [B2] Daily Tribune, "IT-BPM eyes $50B with AI-enabled workforce", https://tribune.net.ph/2026/07/14/it-bpm-eyes-50b-with-ai-enabled-workforce, 2026-07-14, news, H
- [B3] Newsbytes.PH, "Amid AI onslaught, BPO group calls on PH govt to upskill workforce", https://newsbytes.ph/2024/12/28/amid-ai-onslaught-bpo-group-calls-on-ph-govt-to-upskill-workforce/, 2024-12-28, news, M
- [B4] Rest of World, "Philippines virtual assistants" (stat of the day), https://restofworld.org/stat-of-the-day/philippines-virtual-assistants, n.d., news citing private estimates, M
- [B5] Manila Bulletin, "The Filipino online freelancers: digital workforce of the world", https://mb.com.ph/2025/1/7/the-filipino-online-freelancers-digital-workforce-of-the-world, 2025-01-07, news citing vendor estimates, M
- [B6] Philstar/AFP, "Filipinos see pathway out of poverty in virtual assistant jobs", https://www.philstar.com/business/2025/03/27/2431478/filipinos-see-pathway-poverty-virtual-assistant-jobs, 2025-03-27, news, M
- [B7] PSA, 2019 ASPBI IT-BPM, https://psa.gov.ph/content/2019-annual-survey-philippine-business-and-industry-aspbi-information-technology-business, official, H
- [B8] Daily Tribune, "IT-BPM seen key economic driver", https://tribune.net.ph/2023/10/04/it-bpm-seen-key-economic-driver, 2023-10-04, news, M
- [B9] Voxjar, "Why QA sampling fails", https://voxjar.com/why-qa-sampling-fails, 2025-26, vendor claim, M
- [B10] Kalibrr job ad, Aces Call Center "Quality Analyst", https://www.kalibrr.id/c/aces-call-center-jobs-inc-/jobs/208524/quality-analyst-bpo-mandaluyong, 2025-26, job ad, M-H
- [B11] My Mountain Mover blog (EOD reports), https://mymountainmover.com/?p=15801, n.d., vendor blog, L-M
- [B12] Magic help center, "MagicGPT key feature: End of Day report", https://getmagic.com/help/assistants/magicgpt-key-feature-end-of-day-report/, 2026-04-03, vendor documentation, H
- [B13] Hubstaff pricing, https://hubstaff.com/pricing, accessed 2026-10-18, vendor, M-H
- [B14] EvaluAgent, "MaestroQA vs EvaluAgent", https://www.evaluagent.com/compare/maestroqa-vs-evaluagent/, Aug 2026, vendor claim, H
- [B15] eesel.ai, "Zendesk QA", https://www.eesel.ai/blog/zendesk-qa-quality-assurance, 2026, third party, M
- [B16] CloudTalk, "Observe.AI pricing", https://www.cloudtalk.io/blog/observe-ai-pricing/, 2026, third party (competitor), M
- [B17] Hire in South, "Time Doctor pricing", https://www.hireinsouth.com/post/time-doctor-pricing, 2026, third party, M
- [B18] Stork.ai, "Standuply", https://www.stork.ai/en/standuply, n.d., third party, L-M
- [B19] CertPro, "SOC 2 certification Philippines", https://certpro.com/soc-2-certification-philippines/, n.d., consultancy, L-M
- [B20] NPC Advisory 2024-04, Guidelines on AI systems, https://privacy.gov.ph/wp-content/uploads/2025/02/Advisory-2024.12.19-Guidelines-on-Artificial-Intelligence-w-SGD.pdf, 2024-12-19, official, H
- [B21] Tatler Asia, "MK Bertulfo, FHMoms founder", https://my.asiatatler.com/gen-t/leadership/mk-bertulfo-fhmoms-founder-philippines-education, n.d., news/profile, M
- [B22] Payoneer, "Philippines freelance market 2022 report", https://blog.payoneer.com/freelancers/philippines-freelance-market-2022-report, 2022, vendor survey (page now redirects), M

**Accounting & bookkeeping**

- [A1] PICPA homepage/annual report, https://picpa.com.ph, c.2025 (data 2021), association claim, M
- [A2] ACPAPP repost of Manila Times op-ed "Is the Philippines producing enough CPAs", https://www.acpapp.org.ph/post/is-the-philippines-producing-enough-cpas, 2026-06-17, opinion citing PRC, M-H
- [A3] PSA, 2018 CPBI Table A, Professional, Scientific and Technical Activities, https://psa.gov.ph/system/files/ssd/Table%20A.%20Comparative%20Statistics%20for%20Professional%2C%20Scientific%20and%20Technical%20Activities%20Establishments%20by%20Industry%20Group_Philippines%2C%202018%20and%202012%20CPBI.PDF, official, H
- [A4] PSA, 2019 ASPBI preliminary results Sector M, https://psa.gov.ph/system/files/ssd/2019%20ASPBI%20Preliminary%20Result_Sector%20M_v5_ONS-signed-2.pdf, official, H
- [A5] Forvis Mazars, "December 2025 tax calendar", https://www.forvismazars.com/ph/en/insights/tax-deadlines/december-2025-tax-calendar, 2025, advisory, H
- [A6] PwC PH, Tax Alert 35 (RMC 62-2026), https://www.pwc.com/ph/en/tax/tax-publications/tax-alerts/2026/tax-alert-35.html, 2026-06-09, advisory, H
- [A7] Nopuente Law, "Missing BIR Form 2307", https://nopuentelawfirm.com/blog/missing-bir-form-2307/, n.d., law-firm blog, M
- [A8] KPMG PH, "Clarifications on the revenue regulations implementing the EOPT Act", https://kpmg.com/ph/en/home/insights/2024/08/clarifications-on-the-revenue-regulations-implementing-the-eopt-act.html, 2024-08, advisory, H
- [A9] Aureada Law, "BIR e-invoicing deadline December 31, 2026", https://www.aureadalaw.com/post/bir-e-invoicing-deadline-december-31-2026-is-your-business-covered, 2026, law firm, M
- [A10] Inquirer, "BIR extends AFS submission deadline to May 25", https://business.inquirer.net/591230/bir-extends-afs-submission-deadline-to-may-25, 2026-05, news, H
- [A11] Davao Accountants, "The cost of bookkeeping in the Philippines", https://www.davaoaccountants.com/2025/10/06/the-cost-of-bookkeeping-in-the-philippines-what-startups-and-smes-need-to-know/, 2025-10-06, firm blog, M
- [A12] Loft.ph, "How much should you pay for bookkeeping in the Philippines", https://loft.ph/how-much-should-you-pay-for-bookkeeping-in-the-philippines/, 2025-10-17, firm blog, M
- [A13] Built for Bookkeepers, https://builtforbookkeepers.com/, (c)2026, accessed 2026-10-18, vendor page and claims, M
- [A14] Juan, "FAQs on moving JuanTax Plus to Juan Accounting", https://help.juan.tax/en/articles/482-faqs-on-moving-juantax-plus-to-juan-accounting, 2026, vendor, H
- [A15] Juan partner listing, https://juan.tax/partner-listing/, accessed 2026-10-18, vendor, M
- [A16] Taxumo pricing, https://www.taxumo.com/pricing/, accessed 2026-10-18, vendor, H
- [A17] Content Snare pricing, https://contentsnare.com/pricing/, accessed 2026-10-18, vendor, H
- [A18] Financial Cents pricing, https://financial-cents.com/pricing/, accessed 2026-10-18, vendor, H
- [A19] Toolradar, "TaxDome pricing", https://toolradar.com/tools/taxdome/pricing, 2026-05, third party, M
- [A20] Capterra, "Karbon pricing", https://www.capterra.com/p/156181/Karbon/pricing/, n.d., third party, L-M
- [A21] Grant Thornton PH, "Simplified guidelines on use of CAS/CBA", https://www.grantthornton.com.ph/alerts-and-publications/technical-alerts/tax-alert/2021/simplified-guidelines-and-procedures-on-the-use-of-cascba-and-its-components/, 2021, advisory, H
- [A22] Juan, "BIR certifies JuanTax as the first eTSP provider", https://help.juan.tax/en/articles/212-bir-certifies-juantax-as-the-first-e-tsp-provider, ~2018, vendor, M
- [A23] Baker McKenzie, "Philippines: new NPC circular on registration of DPOs and data processing systems", https://insightplus.bakermckenzie.com/bm/data-technology/philippines-new-npc-circular-on-registration-of-data-protection-officers-and-data-processing-systems-takes-effect_1, 2023, law firm, H
- [A24] GummySearch, r/AccountingPH, https://gummysearch.com/r/AccountingPH, undated snapshots, third party, L
- [A25] Facebook page "Freelance Accountants & Bookkeepers Philippines", https://www.facebook.com/freelanceaccountantsphilippines/, accessed 2026-10, FB page, M
- [A26] QuickBooks PH, "Accounting websites every pro should be reading", https://quickbooks.intuit.com/ph/resources/accountants/accounting-websites-every-pro-should-be-reading/, c.2026-01, vendor content, M

**Recruitment / DMW**

- [M1] BusinessMirror, "OFW deployments hit record 2.7 million in 2025 as Kuwait and Europe open doors", https://businessmirror.com.ph/2026/02/04/ofw-deployments-hit-record-2-7-million-in-2025-as-kuwait-and-europe-open-doors/, 2026-02-04, news citing DMW, M-H
- [M2] Manila Shimbun, DMW deployment Jan-Sep 2024, https://www.manila-shimbun.com/category/english/news280140.html, 2024-11, news citing DMW, M
- [M3] DMW, Licensed recruitment agencies (archive), https://dmw.gov.ph/archives/v1/licensed-recruitment-agencies, n.d., official directory, M
- [M4] ILO, "Assessment of the POEA awards system for licensed land-based and sea-based agencies", https://www.ilo.org/resource/assessment-poea-awards-system-licensed-land-based-and-sea-based-agencies, ~2013, report, M (stale)
- [M5] WKO Salzburg, "Philippinische Fachkräfte anwerben", https://www.wko.at/sbg/news/philippinische-fachkraefte-anwerben, n.d., chamber article, L-M
- [M6] DOLE BLE, Registration of job contractors, https://ble.dole.gov.ph/registration-of-job-contractor/, 2024-2025 lists, official, M
- [M7] Kabayan Remit, "How to be an OFW: process", https://kabayanremit.com/travel/how-to-be-ofw-process/, 2024-02-21, guide, M
- [M8] SunStar, "OEC out, OFW Pass in", https://www.sunstar.com.ph/manila/oec-out-ofw-pass-in, 2025, news, M
- [M9] MWO Dubai, Agency-hire information, https://mwodubai.org/agency-hire-information/, n.d., government office page, M
- [M10] Philippine Embassy Australia, MWO Canberra Advisory 02 s.2026, https://philembassy.org.au/user/pages/05.latest/03.polo/mwo-canberra-advisory-no-02-series-of-2026/, 2026, official, M
- [M11] Digest PH, "New procedure for online registration of landbased workers and LBECS" (POEA MC 007-18), https://www.digest.ph/labor/new-procedure-for-online-registration-of-landbased-workers-and-landbased-e-contracts-system-lbecs, 2018-04-26, regulation summary, H (existence)
- [M12] DMW e-Services portal, https://portal.dmw.gov.ph/Public/RegisterAccount.aspx, live, official, M
- [M13] Daily Tribune, "Pinoy welders' delayed deployment resolved", https://tribune.net.ph/2025/03/27/pinoy-welders-delayed-deployment-resolved, 2025-03-27, news, M
- [M14] Manila Times, "Recruitment groups deplore slow deployment process", https://www.manilatimes.net/2020/09/27/news/latest-stories/recruitment-groups-deplore-slow-deployment-process/772675/, 2020-09-27, news, M
- [M15] Inquirer, "DFA blames recruiters for apostille queue", https://newsinfo.inquirer.net/1573373/dfa-blames-recruiters-for-apostille-queue/amp, 2022, news, M
- [M16] Manatal pricing, https://www.manatal.com/pricing, accessed 2026-10-18, vendor, H
- [M17] Recruit CRM pricing, https://recruitcrm.io/pricing/, accessed 2026-10-18, vendor, H
- [M18] Jobstreet, "Jobstreet democratizes platform", https://ph.jobstreet.com/about/news/article/jobstreet-democratizes-platform, 2024-05, vendor news, H
- [M19] Daily Tribune, "Jobstreet launches seekAi for smarter hiring", https://tribune.net.ph/2026/09/27/jobstreet-launches-seekai-for-smarter-hiring, 2026-09-27, news (vendor survey figure), H
- [M20] NPC, Data Privacy Act of 2012 (RA 10173), https://privacy.gov.ph/data-privacy-act/, 2012, statute, M (read via snippets)
- [M21] NPC Circular 2022-04, https://privacy.gov.ph/wp-content/uploads/2023/05/Circular-2022-04.pdf, 2022-12-05, official, H
- [M22] BusinessMirror, "DMW to require recruitment and manning agencies to register social media accounts", https://businessmirror.com.ph/2025/02/11/dmw-to-require-recruitment-and-manning-agencies-to-register-social-media-accounts/, 2025-02-11, news, H
- [M23] SunStar, "DMW warns jobseekers vs bogus recruitment firm agents", https://www.sunstar.com.ph/manila/dmw-warns-jobseekers-vs-bogus-recruitment-firm-agents, 2024-01, news, M
- [M24] Inquirer, "DMW online systems hit by ransomware attack", https://newsinfo.inquirer.net/1961357/dmw-online-systems-hit-by-ransomware-attack-alternate-measures-up, 2024-07-16, news, H
- [M25] assistance.ph, "DMW licensed recruitment agencies", https://assistance.ph/dmw-licensed-recruitment-agencies-feature/, 2024-03-16, guide, M
- [M26] assistance.ph, "How to apply: DMW approved job orders", https://assistance.ph/how-to-apply-dmw-approved-job-orders/, n.d., guide, M
- [M27] The Filipino Times, "DMW eyes faster license renewal for recruitment agencies with clean records", https://filipinotimes.net/latest-news/2026/08/25/dmw-eyes-faster-license-renewal-for-recruitment-agencies-with-clean-records/, 2026-08-25, news, M-H

**Insurance, lending, co-ops, banks**

- [I1] Insurance Commission, "List of Licensed LIFE Insurance Agents as of November 20, 2025", https://www.insurance.gov.ph/wp-content/uploads/2025/11/November-20-2025-List-of-Licensed-LIFE-Insurance-Agents.pdf, 2025-11-20, regulator data (row count by sub-agent), H rows / M split
- [I2] Manila Bulletin, "Sun Life Philippines tops MDRT rankings for 2025", https://mb.com.ph/2025/07/10/sun-life-philippines-tops-mdrt-rankings-for-2025-joins-global-elite, 2025-07-10, news, H
- [I3] AIA Philippines, "AIA PH and BPI AIA highest MDRT qualifiers in the Philippines 2026", https://www.aia.com.ph/en/about-us/media-centre/press-releases/2026/aia-ph-and-bpi-aia-highest-mdrt-qualifiers-philippines-2026, 2026-07, press release, H
- [I4] Expressway.ph, "Cooperative auto loan guide" (citing CDA 2024 statistics), https://www.expressway.ph/car-loan/cooperative-auto-loan-guide, 2026-09-28, secondary, M
- [I5] Kapronasia, "Philippines SEC increases scrutiny of lending platforms", https://kapronasia.com/insight/blogs/fintech-research/philippines-sec-increases-scrutiny-of-lending-platforms, 2025, secondary, M
- [I6] Fintech News PH, rural banks mergers and capital requirements, https://fintechnews.ph/70190/banking/philippines-rural-banks-mergers-capital-requirements-2027-bsp/, ~2025, secondary, L-M
- [I7] Bossjob.ph, Manila Bankers Life "Conservation Assistant" job ad, (URL not captured by sub-agent), n.d., job ad, M
- [I8] MoneySense, "Sun Life launches digitally enabled selling process for clients", https://moneysense.com.ph/sun-life-launches-digitally-enabled-selling-process-for-clients/, 2020-21, news, M
- [I9] InsiderPH, "Winning AIA, BPI AIA boost adviser excellence with AI training", https://insiderph.com/winning-aia-bpi-aia-boost-adviser-excellence-with-ai-training, 2026-07-24, news, H
- [I10] NATCCO, Digital solutions (eKoopBanker), https://natcco.coop/services/support-services/digital-solutions/, accessed 2026-10, federation/vendor, H
- [I11] WeSupport, CoopVault, https://coopvault.w.wesupportinc.com/, (c)2026, vendor, H
- [I12] Loandisk, https://loandisk.com/ (price from Capterra/GetApp listing), n.d., vendor listing, M
- [I13] Digihustle Hub, "Lending Business Tracker Spreadsheet", https://digihustle-hub.notion.site/Lending-Business-Tracker-Spreadsheet-ac38a22ecd4841ecbfa6b29f7b4d7b44, n.d., seller page, M
- [I14] Grant Thornton PH, "Prohibition on unfair debt collection practices of financing companies, lending companies" (SEC MC 18 s.2019), https://www.grantthornton.com.ph/insights/articles-and-updates1/tax-notes/prohibition-on-unfair-debt-collection-practices-of-financing-companies-lending-companies/, 2019, advisory on primary, H
- [I15] NPC Circular 20-01, https://privacy.gov.ph/wp-content/uploads/2022/02/NPC-Circular-No.-20-01.pdf, 2020-09-14 (amended 2022-02), official, H
- [I16] Inquirer (PAOCC online lending complaints), https://newsinfo.inquirer.net/?p=2197448, 2026-03, news (snippet only), M
- [I17] jur.ph summary of IC CL 2015-12, https://jur.ph/law/summary/directing-proper-identification-insurance-agents, 2015, secondary, M
- [I18] IC Circular Letter 2014-47, https://www.insurance.gov.ph/wp-content/uploads/2022/11/CL2014_47.pdf, 2014, official, M (snippet)
- [I19] IC Circular Letter 2021-66, https://www2.insurance.gov.ph/wp-content/uploads/2022/02/CL2021_66.pdf, 2021, official, M
- [I20] Chambers Practice Guides, "Insurance & Reinsurance 2026: Philippines trends and developments", https://practiceguides.chambers.com/practice-guides/insurance-reinsurance-2026/philippines/trends-and-developments, 2026, legal guide, H
- [I21] Asia Advisers Network (LUAP), https://www.asiaadvisersnetwork.com/Article?aid=73861, 2017, secondary, L-M
- [I22] ThinkPesos, "Top 7 Filipino Facebook groups sharing financial literacy", https://thinkpesos.com/top-7-filipino-facebook-group-sharing-financial-literacy/, ~2017, blog, L
- [I23] NATCCO, About us, https://natcco.coop/home/about-us/, 2026, self-report, M-H
- [I24] Malaya Business Insight, "PH financial resources up at P34.6T — BSP", https://malaya.com.ph/business/ph-financial-resources-up-at-p34-6t-bsp, 2025-09-23, news citing BSP, M-H
- [I25] BSP Circular 1133 s.2021, https://www.bsp.gov.ph/Regulations/Issuances/2021/1133.pdf, 2021-12, official, H

**Real estate, HOA, condo**

- [R1] PRC, "April 2026 Real Estate Brokers Licensure Examination results released in three (3) working days", https://www.prc.gov.ph/article/april-2026-real-estate-brokers-licensure-examination-results-released-three-3-working-days, Apr 2026, official, H
- [R2] PRC, April 2025 REB exam results, https://prc.gov.ph/node/7852, Apr 2025, official, H
- [R3] REN.PH broker verification, https://ren.ph/verify/broker, n.d., third-party database, L-M
- [R4] GMA News, "DHSUD to beef up engagement with HOAs to help address woes", https://www.gmanetwork.com/news/topstories/nation/972068/dhsud-to-beef-up-engagement-with-hoas-to-help-address-woes/story/, 2026-01-08, news citing DHSUD, H
- [R5] Respicio & Co., "DHSUD assistance for homeowners association in collecting delinquent dues", https://www.respicio.ph/commentaries/dhsud-assistance-for-homeowners-association-in-collecting-delinquent-dues-in-the-philippines, n.d., law-firm commentary, L-M
- [R6] DHSUD Department Order 2021-007 (Revised IRR of RA 9904), https://lpr.adb.org/sites/default/files/2024-07/philippines-department-order-no-2021-007.pdf, 2021, official, H
- [R7] REN.PH Academy, "Condominium Act essentials: assessments and liens", https://ren.ph/academy/courses/condominium-act-essentials/assessments-and-liens, n.d., educational, M
- [R8] Daily Tribune, "DHSUD extends HOA re-registration deadline to December", https://tribune.net.ph/2026/06/15/dhsud-extends-hoa-re-registration-deadline-to-december, 2026-06-15, news, H
- [R9] Listd, "Real estate broker lead generation", https://listd.ph/blog/real-estate-broker-lead-generation, 2026-05-18, vendor blog, L-M
- [R10] OnlineJobs.ph job 1590746, https://www.onlinejobs.ph/jobseekers/job/1590746, updated 2026-08-30, job post, M
- [R11] HOAS, https://hoas.ph/, accessed 2026-10-18, vendor pricing, H
- [R12] HOAMS, https://www.hoams.ph/, accessed 2026-10-18, vendor pricing, H
- [R13] MyHOA, https://myhoa.ph/, accessed 2026-10-18, vendor page (claims), M
- [R14] HOACentralPH, https://hoacentralph.com/, accessed 2026-10-18, vendor page (claims), M
- [R15] Toolradar, "TownSq pricing", https://toolradar.com/tools/townsq/pricing, 2026, third-party, M
- [R16] Buildium pricing, https://www.buildium.com/pricing/, accessed 2026-10-18, vendor, H
- [R17] PRBRES Resolution 11 s.2021, https://www.prc.gov.ph/sites/default/files/2021-11%20publishedRES.pdf, 2021, official, H
- [R18] Housal, "RA 9646 RESA broker licensing", https://housal.com/legal/laws/ra-9646-resa-broker-licensing, n.d., secondary summary, M
- [R19] Context.ph, "DOJ tightens net on rogue property brokers", https://context.ph/2026/06/24/doj-tightens-net-on-rogue-property-brokers/, 2026-06-24, news (snippet only), M
- [R20] National Privacy Commission, NPC 21-251 decision, https://privacy.gov.ph/wp-content/uploads/2025/04/NPC-21-251-2024.08.01-AFV-v.-FRA.pdf, 2024-08-01, regulator decision, H (existence)
- [R21] REBAP national FB page, https://www.facebook.com/rebapnational/, n.d., association page, M
- [R22] PAREB MLS members directory, https://www.mlspareb.com/members, n.d., association directory, M
- [R23] SWAPD listing "54.3K Philippines Real Estate Facebook group", https://swapd.co/t/54-3k-philippines-real-estate-facebook-group-2-6k-new-members-in-60-days-9-years-established/2123725, n.d., marketplace listing, L
- [R24] DHSUD list of homeowners associations, https://dhsud.gov.ph/list-of-homeowners-association/, n.d., official (403, unverified), M
- [R25] GMA News, "Metro Manila unsold condo inventory reaches record 82,900 units", https://www.gmanetwork.com/news/money/economy/994028/metro-manila-unsold-condo-inventory-reaches-record-82-900-units/story/, 2026-07-07, news citing Leechiu, H

**Other industries**

- [O1] PRC, "November 2025 Customs Brokers Licensure Examination Results Released in Two (2) Working Days", https://www.prc.gov.ph/article/november-2025-customs-brokers-licensure-examination-results-released-two-2-working-days, Nov 2025, official, H (via search summary)
- [O2] jur.ph summary of BOC CAO 2-2007 e-Customs VASPs, https://jur.ph/law/summary/accreditation-establishment-operation-e-customs-vasps, n.d., legal summary, M
- [O3] PortCalls, "Proposed BOC order sets rules for VASP accreditation", https://portcalls.com/proposed-boc-order-sets-rules-vasp-accreditation/, date not shown, news, M
- [O4] DHL, "Customs audits compliance mistakes" (PH), https://www.dhl.com/discover/en-ph/logistics-advice/essential-guides/customs-audits-compliance-mistakes, n.d., vendor guide, M
- [O5] Inquirer, "DepEd still looking to hit 27M enrollment target", https://newsinfo.inquirer.net/1970100/deped-still-looking-to-hit-27m-enrollment-target, Aug 2024, news citing DepEd, M
- [O6] PIA, "PBBM signs law allowing disadvantaged students to take exams despite unpaid school fees", https://pia.gov.ph/news/2024/03/26/pbbm-signs-law-allowing-disadvantaged-students-to-take-exams-despite-unpaid-school-fees, 2024-03-26, official news, H
- [O7] Inquirer, "Private schools warn: 'No permit, no exam' bills to force shutdowns", https://newsinfo.inquirer.net/1753166/private-schools-warn-no-permit-no-exam-bills-to-force-shutdowns/amp, 2023 (date per URL era; exact not shown), news, M
- [O8] HitPay, "Education payment solutions" (PH), https://hitpayapp.com/ph/solutions/education-payment-solutions, n.d., vendor page, M

**Candidate 1 extra**

- [P1-1] Hunted.space (Product Hunt mirror), "DocChase", https://hunted.space/product/docchase, n.d., launch listing (vendor comment for price), M
- [P1-2] Insightful Accountant, "App Aware: BilagPilot", https://blog.insightfulaccountant.com/app-aware-bilagpilot, 2026-09-08, trade editorial based on developer info, M

**Internal**

- [INT] integration-constraints.md (this repo): Semaphore SMS ₱0.56/credit [55][56]; Viber bots commercial-only, EUR 100/bot/month [33][34]; Gmail restricted scopes/CASA [35][36]
