# P1 launch prerequisites

Status: research complete (2026-10-21). Not owner-approved. Facts, inferences and recommendations are labelled.

Question: what must the owner do before charging the first customer ₱1,500 for P1, and what does each step cost? Pilot = the free 4-week concierge pilots. Paid = from the first ₱1,500 charge. "Required" means required by law or by the provider; inferences are marked. Tags like [L1] point to Sources.

## Summary table

| Item | Required at pilot? | Required at paid? | Cost | Time | Online? |
|---|---|---|---|---|---|
| DTI business name (BNRS) | No | Yes in practice: needed if trading under a brand name, and for PayMongo / GCash for Business. Not needed by DTI rules if the owner trades in their own name | ₱230 barangay / **₱530 city** / ₱1,030 regional / ₱2,030 national (incl. ₱30 DST), valid 5 yrs [L1] | < 1 hour (unofficial) | Yes |
| Barangay clearance | No | Probably; LGU-dependent and arguable for a purely online service [L3] | Not found | Within RA 11032's 3/7/20 working days | LGU-dependent |
| Mayor's permit | No | Probably; same uncertainty [L3] | Not found | 3–20 working days | LGU-dependent |
| BIR registration (Form 1901 → COR 2303) | No *(inference; no source covers free pilots)* | **Yes**, on or before the first sale [L9] | ₱30 DST; ₱500 annual fee **abolished** since 22 Jan 2024 [L7][L8] | Days to ~2 weeks (unofficial) | Yes (ORUS) |
| 8% income-tax option | — | Optional, recommended | ₱0; 8% only on gross above ₱250,000/yr [L10] | Ticked at registration | Yes |
| Invoices + NIRI (replaced the "Ask for Receipt" notice) | No | Yes | Booklet/ATP cost not found | Not found | Partly |
| Structured e-invoicing (EIS) | No | No: micro taxpayers appear exempt; deadline for small+ e-commerce is 31 Dec 2026 [L18] | — | — | — |
| Books of accounts (manual) | No | Yes, registered before the first ITR deadline [L19] | Notebook cost not found | — | Yes (ORUS QR stamp) |
| BMBE certificate | No | Optional (income-tax exemption) [L20] | ₱0 | Not found | Not found |
| Personal GCash (receive transfer) | — | Fine for the first customers; ₱100k/month incoming cap [L14]; GCash's terms on business use not found | ₱0 | Instant | Yes |
| PayMongo | No | Optional; needs DTI + BIR 2303 [L12] | ₱0 setup; GCash 2.23% + VAT ≈ ₱37 per ₱1,500 [L16] | Up to 14 business days [L17] | Yes |
| GCash for Business / Maya Business | No | Optional; needs DTI + 2303 (+ permit for Maya), GCash wallet ≥ 12 months old [L13][L15] | Fees not found | Not found | Yes |
| Privacy notice (upload page + site) | **Yes** | Yes | ₱0 | ~1 day drafting | Yes |
| Data processing agreement (IRR §44, 9 clauses) | **Yes** (short version) | Yes (addendum to the terms) | ₱0 self-drafted | 1–2 days | Yes (e-sign) |
| DPO designation (owner as own DPO) | **Yes** | Yes | ₱0 | Minutes | — |
| NPC sworn declaration of exemption | **Yes** *(per law-firm summaries)* | Yes until 1,000 individuals | Notary fee not found | Not found | Filing via NPCRS |
| NPC registration (DPS + DPO) | No | When SPI of ≥ 1,000 individuals (~20 firms); within 20 days; yearly renewal [L28] | Fee not found | Not found | Yes (NPCRS) |
| Breach procedure (72 h) | **Yes** | Yes | ₱0 | ~½ day | — |
| Domain (.com) | **Yes** | Yes | ≈ US$10.46/yr → US$11.17 from 1 Nov 2026 (≈ ₱640–680) [L44]; .ph ≈ ₱2,500–2,750 [L45] | Minutes | Yes |
| SPF / DKIM / DMARC | **Yes** | Yes | ₱0 | Hours (DNS) | Yes |
| Email: Resend free | Yes (pilot sender) | Outgrown (100/day cap) | ₱0: 3,000/month, 100/day [L36] | Minutes | Yes |
| Email: Amazon SES production access | No (start the request early) | Yes, or Resend Pro US$20/mo | US$0.10 per 1,000 ≈ ₱6–7; US$200 new-account credits [L35] | Initial reply ≤ 96 h; denials reported [L33][L34] | Yes |

## 1. Business registration

Confidence codes: H = high, M = medium, L = low. Evidence types: OD = official document/page, VD = vendor docs, TP = third-party (law firm, consultancy, blog), F = forum, SR = claim taken from a search-result summary, not a fetched page.

### 1.1 DTI business name (BNRS)

| Point | Finding | Source |
|---|---|---|
| Fee by territorial scope | Barangay ₱200 · City/Municipality ₱500 · Regional ₱1,000 · National ₱2,000. "All registration fees are subject to an additional PhP 30 Documentary Stamp Tax." So ₱230 / ₱530 / ₱1,030 / ₱2,030 | [L1] OD, H |
| Validity | "valid for five (5) years from the date of registration"; late renewal +50% | [L1] OD, H |
| Who | "Any Filipino who is at least 18 years of age" | [L1] OD, H |
| Online? | Yes, fully online at bnrs.dti.gov.ph; payment by GCash, Maya, Landbank Link.Biz or card | [L1] OD, H; [L2] TP, SR, M |
| Time | Under an hour, certificate downloadable after payment (guides). Official processing time not found | [L2] TP, SR, L–M |
| Documents | A valid government ID, an email, 3–5 candidate names. A representative needs an authorisation letter. Full official list: not fetched | [L1] OD (partial), M; [L2] TP, SR, M |
| Is it needed at all? | A business name is "ANY name that is different from THE TRUE NAME OF AN INDIVIDUAL". DTI registration is for *names*; a person trading under their own name does not need one. But **PayMongo and GCash for Business both ask a sole proprietor for the DTI certificate** (§2), so in practice it is needed to take payments through a gateway | [L1] OD, H (definition); inference, M |

**Recommendation:** City/Municipality scope (₱530) is the cheapest scope that is commonly accepted. Barangay scope (₱230) is legally valid but name protection is only barangay-wide, which does not matter for an online SaaS. *Inference, M.* National (₱2,030) only matters if the owner wants exclusive use of the name nationwide.

### 1.2 Barangay clearance and mayor's permit

| Point | Finding | Source |
|---|---|---|
| Needed for a home-based online business? | LGU practice varies. A purely online service provider "with no premises, inventory, staff, customer visits, or signboard" has "a more arguable case" for not needing a full permit; many LGUs still require one for home-based businesses. Some LGUs (e.g. Zamboanga City) have a special home-based permit | [L3] TP (AI-assisted law commentary), L–M; [L4] news (PTV), SR, M |
| Who asks for it | Maya Business lists a mayor's/business permit among sole-proprietor documents [L15]; PayMongo's sole-prop list does **not** [L12]; the official GCash help page lists DTI + BIR 2303 only [L13]. The BIR's July 2025 checklist (per secondary summaries) asks for ID + DTI certificate if a business name is used, not a mayor's permit [L6] | see rows |
| Cost | **Not found as an official figure.** Fees are set by each LGU revenue code, based on declared capitalisation for a new business. Barangay clearance fee varies by barangay | [L3] TP, L–M |
| Time | RA 11032 (Ease of Doing Business) caps processing at 3 / 7 / 20 working days (simple / complex / highly technical) | [L3] TP citing RA 11032, M |
| Online? | Depends on the LGU; many cities have online BPLO portals, many do not | [L3] TP, L |

**Inference (M):** for a ₱1,500/month SaaS sold online from home, the mayor's permit is the most uncertain and most expensive-to-guess line. **No reliable fee figure was found**; ask the owner's own city BPLO in writing (an email to the BPLO fits the no-calls constraint).

### 1.3 BIR registration

| Point | Finding | Source |
|---|---|---|
| Form | BIR Form 1901 (self-employed / sole proprietor / mixed-income). Output: Certificate of Registration (Form 2303) | [L6] TP, SR, M; [L5] TP, M |
| Online? | Yes, via **ORUS** (orus.bir.gov.ph): upload ID (and a selfie holding it), DTI certificate if a business name is used; eCOR printable online | [L5][L6] TP/consultancy, SR, M |
| Cost | ₱30 loose documentary stamp tax (sources conflict on whether still charged). **Annual Registration Fee of ₱500 abolished**: RA 11976 (Ease of Paying Taxes Act, signed 5 Jan 2024) deleted NIRC §236(B); BIR stopped collecting from 22 Jan 2024 | [L7] TP (Baker McKenzie), M–H; [L8] PIA (gov news), SR, H; [L5] TP, M |
| Deadline | Register "on or before the commencement of business operations" (NIRC §236(A)) | [L9] TP (Grant Thornton, 2024-09-03), H |
| In person? / time | RMC 91-2024 (per summaries): self-employed register on or before starting business, generally **no RDO visit needed** unless ORUS fails; proof is a QR-code stamp. Processing time: "a few business days" to "one to two weeks" (unofficial); **official timeline not found** | [L5b] TP (Grant Thornton; batasnatin summary of RMC 91-2024), SR, M; [L6] TP, SR, L |
| Mayor's permit needed for BIR? | A 1901 guide lists the mayor's/barangay permit "if already issued", suggesting BIR no longer insists on it first. Not confirmed from a BIR issuance | [L6] TP, SR, L–M |
| Penalty if late | Voluntary late registration: ₱1,000 compromise (§275). Unregistered and found by BIR: ₱5,000 (micro) / ₱15,000 (small) compromise (§258) | [L9] TP, M–H |
| 8% option | Non-VAT self-employed with gross sales + other non-operating income ≤ ₱3M may pay **8% on gross sales above ₱250,000**, in lieu of graduated income tax *and* the percentage tax. Elect on Form 1901 at registration or on the first-quarter 1701Q; irrevocable for the year | [L10] TP (Grant Thornton, Juan.tax, Deloitte; RR 8-2018/RMO 23-2018), SR, M–H |
| What 8% means for P1 | At ₱1,500 × 10 firms × 12 = ₱180,000/yr, gross is below ₱250,000, so **income tax is ₱0** under 8%; quarterly/annual returns are still filed (nil). *Inference, M* | inference |
| Invoices | Since RA 11976 the invoice (not the official receipt) is the primary document. The old "Ask for Receipt" Notice (ARN) was replaced by the **Notice to Issue Receipt/Invoice (NIRI)** under RMO 43-2022 (ARNs valid until 30 Jun 2023). A new registrant may use BIR-printed invoices or apply for an Authority to Print; cost of printed invoice booklets: not found | [L11] TP (Forvis Mazars, Grant Thornton), SR, M |
| E-invoicing | Mandatory structured e-invoicing (RR 11-2025 as amended by RR 26-2025) covers e-commerce taxpayers classified **small, medium or large**, deadline **31 Dec 2026**; RMC 98-2026 (22 Sep 2026) kept the date. **Micro taxpayers (gross < ₱3M) appear exempt** from the e-commerce category. P1's owner would be micro | [L18] TP (Grant Thornton 2026, Sovos), SR, M |
| Books of accounts | New registrants may choose manual, loose-leaf or computerised books (RMC 65-2025, Jul 2025). Manual books are registered before the first quarterly/annual ITR deadline; ORUS issues a QR stamp. Cost: a columnar journal + ledger notebook (price not found; commonly bookstore items). Books are still required under the 8% option | [L19] TP (Grant Thornton 2025-07-16), M; [L10] TP, M |

### 1.4 Optional: BMBE certificate

Barangay Micro Business Enterprise (RA 9178 as amended by RA 10644): **free** Certificate of Authority from the Negosyo Center/DTI, valid 2 years, for enterprises with assets ≤ ₱3M (excluding land). Gives **income-tax exemption** on income from operations and an exemption from the 1% e-wallet withholding (§2.4). Needs the DTI certificate first. Professional-practice businesses and licensure-exam passers are excluded; an online software service looks eligible. Whether the LGU still requires a mayor's permit: not found. [L20] OD (DTI flyer/news), SR, M; [L21] TP, SR, L–M. *Worth a phase-6 look, not a launch blocker.*

### 1.5 Operating unregistered during the free pilots

- **No tax law found that exempts a pre-revenue free trial.** Registration is due "on or before the commencement of business". Secondary sources say the BIR reads "commencement" broadly, starting no later than the first sale or commercial transaction, and that a business being "small, new, home-based, or not yet profitable" does not delay it [L9][L22] (TP, SR, M). **No source squarely answers the free-pilot case** (not found).
- **Inference (M):** free concierge pilots with no fee, no invoice and no payment are not a sale, so the practical risk of running them as an individual is low. The first peso of revenue (the ₱1,500 GCash charge) is clearly "business". **Register DTI + BIR before sending the first payment request**, ideally during week 3 of the pilots so the COR is in hand by the end of week 4.
- The privacy obligations in §3 apply to pilots regardless of registration; the DPA does not depend on a business being registered.

## 2. Getting paid

| Option | Who can sign up | Cost | Time | Source |
|---|---|---|---|---|
| **Personal GCash** (Send Money to the owner's fully verified wallet) | Any fully verified user | ₱0 for GCash-to-GCash | Instant | [L14] OD, H (limits) |
| Limits | Fully verified: **₱100,000 monthly incoming** and wallet limit; cumulative incoming counts even if spent. 10 firms × ₱1,500 = ₱15,000/mo, well inside | [L14] OD, H |
| Is business use allowed? | GCash terms on receiving business payments into a personal wallet: **not found**. No invoice is produced by GCash itself; the owner still issues a BIR invoice if registered | not found |
| **GCash for Business** (merchant) | "Authorized signatory of a government-registered business". Sole prop: **DTI certificate + BIR 2303**; fully verified GCash account active **≥ 12 months**; email not used before for GCash for Business; low-risk industry; one account per company | [L13] OD (GCash Help, SR; page 403 to fetcher), M–H |
| GCash for Business fees | Not found | — |
| A third-party 2026 guide describes a "Basic" GCash for Business tier needing only ID + selfie. It contradicts the official help page | [L13b] TP, L |
| **PayMongo** | Sole proprietorship: **DTI certificate, owner's government ID, BIR Form 2303**. No individual/unregistered type is listed for Philippine entities | [L12] VD, H |
| PayMongo fees | **Setup ₱0, no monthly fee.** GCash **2.23%**, Maya 1.79%, QR Ph 1.34%, cards 3.125% + ₱13.39, online banking 0.71% or ₱13.39; **all VAT-exclusive** (+12%). ₱1,500 via GCash ≈ ₱33.45 + VAT ≈ **₱37.46** per payment | [L16] PP, H |
| PayMongo time | Activation "usually takes up to 14 business days" after documents are submitted | [L17] VD, SR, M |
| **Maya Business** | Sole prop: valid ID, business/mayor's permit, BIR 2303, DTI registration (agent/merchant pages). Payment Links need a **Maya Business Pro** upgrade, which asks for a valid BIR 2303 | [L15] VD, SR, M |
| Maya fees | MDR not found | — |
| **1% withholding on e-wallet/gateway remittances** | RR 16-2023: Maya (and other digital financial service providers) withhold 1% on half of gross remittances from 13 Oct 2024, unless the merchant files a **sworn declaration stamped by its RDO** (annual remittances ≤ ₱500,000) before 10 Jan each year, or has a BMBE certificate | [L23] VD (Maya), H for Maya; applicability to PayMongo inferred, M |

**Inference (M):** for 1–10 customers at ₱1,500, a **personal GCash transfer to the owner's verified wallet plus a BIR-registered invoice** is the cheapest legal way to get paid. It costs ₱0 in fees, needs no merchant onboarding, and avoids the 1% withholding paperwork. PayMongo becomes worth it when self-serve card payment or automated payment links matter (Phase 10+), and it then needs DTI + BIR 2303 anyway.

## 3. Data privacy (RA 10173)

Background: P1 is a personal information processor (PIP), often a sub-processor, and handles sensitive personal information (tax returns, likely TINs); see [p1-kill-checks.md §4](p1-kill-checks.md). The Act applies to pilots as much as to paid use: it turns on processing personal data, not on charging or on being a registered business. *(Inference from RA 10173 §4 scope, M–H.)*

| Item | What is required | Pilot? | Paid? | Cost / time / online | Source |
|---|---|---|---|---|---|
| **Privacy notice** | Data subjects have the right to be informed (RA 10173 §16). P1 needs (a) a notice on the upload page aimed at the bookkeeper's clients, naming the bookkeeper as controller and P1 as processor, and (b) a website privacy policy for bookkeepers as users | Yes | Yes | ₱0, ~1 day to draft, online | [L24] statute, H; drafting detail inference, M |
| **Outsourcing agreement / DPA** | IRR §44: processing by a PIP "shall be governed by a contract or other legal act" stating subject matter, duration, nature, purpose, data types, data-subject categories, controller rights and obligations, and processing location. It must bind P1 to nine things: (1) process only on documented instructions, incl. cross-border transfers; (2) staff confidentiality; (3) appropriate security measures; (4) "Not engage another processor without prior instruction" (list the LLM API, email, hosting); (5) help answer data-subject requests; (6) help the controller comply; (7) "delete or return all personal data" at end of service, including copies; (8) give compliance information and allow audits; (9) "Immediately inform" the controller of an unlawful instruction. IRR §45: the PIP also complies with the Act directly | Yes, a short signed version (the pilot bookkeeper is already sending client data) | Yes, as an addendum to the terms of service | ₱0 if self-drafted; 1–2 days; signable by email/e-signature | [L25] IRR via ChanRobles, M–H |
| **DPO** | IRR §26(a): "Any natural or juridical person … involved in the processing of personal data shall designate an individual or individuals who shall function as data protection officer, compliance officer or otherwise be accountable". NPC Advisory 2017-01 says the DPO "should be a full-time or organic employee", independent, and may hold other roles absent a conflict of interest. NPC AO 2024-006 found "no legal hindrance" to senior officers doubling as DPO if they keep enough autonomy. **No NPC text found that addresses a one-person business.** Inference (M): a sole proprietor designates themself, since §26(a) covers natural persons and there is no one else | Yes (designation is a one-line written record) | Yes, and the DPO is registered with the NPC once registration applies | ₱0 | [L25] M–H; [L26] TP (Alburo Law, 2022-06-01), M; [L27] OD (NPC AO 2024-006), SR, M |
| **NPC registration** | NPC Circular 2022-04 (in force 11 Jan 2023): mandatory if ≥ 250 employees, or SPI of **≥ 1,000 individuals**, or processing likely to pose a risk to data subjects. **PIPs not covered "must submit a Sworn Declaration and Undertaking for Exemption from Registration"** (notarised). New systems/DPOs register within **20 days** of starting; certificate valid **1 year**, renew within 30 days before expiry; online only via the NPC Registration System (NPCRS); sanctions include compliance orders, processing bans and administrative fines | Pilots (5 firms, < 1,000 people): **sworn declaration of exemption**, not registration *(per summaries; primary text not reachable)* | Register within 20 days of crossing 1,000 individuals, which P1 reaches at ~20 firms (kill-checks §4) | Registration fee: **not found** (no summary mentions one). Notary fee for the sworn declaration: not found. Online | [L28] TP (SyCip via Mondaq, 2023-05-23), M; [L29] TP (Baker McKenzie, 2023-01), SR, M; [L30] OD (privacy.gov.ph FAQ), SR, M |
| **Breach notification** | IRR §38: the controller notifies the NPC and affected data subjects "within seventy-two (72) hours upon knowledge of, or when there is reasonable belief by the personal information controller or personal information processor" that a notifiable breach occurred. Notification is required when SPI or identity-fraud-enabling information was likely acquired by an unauthorised person with a real risk of serious harm. NPC Circular 16-03 is still the breach-management circular (no replacement found); filings go through the NPC Data Breach Notification Management System (DBNMS). P1 as processor must tell the bookkeeper fast enough for the 72-hour clock (put "within 24 hours" in the DPA; *recommendation*) | Yes (a one-page breach procedure) | Yes | ₱0 | [L25] IRR §38, M–H; [L31] TP/OD index, SR, M |
| **Security measures** | IRR §44(3) and §45: "appropriate security measures". Spec items: encryption at rest, per-firm isolation, access log, deletion at retention end | Yes (minimal) | Yes | Engineering time, not cash | [L25] M–H; kill-checks §4 |

**Free official templates.** The NPC publishes a *Privacy Toolkit* with guidelines, templates and forms (seen only as a third-party Scribd copy [L32], L); the official download page could not be reached (privacy.gov.ph returned 403 to our fetcher). An official NPC privacy-notice or DPA **template: not found.** Commercial template sites (Genie AI) exist but are not NPC material [L32]. *Recommendation:* draft the DPA directly from the nine IRR §44 clauses above; that is the checklist the NPC would apply.

## 4. Email sending

### 4.1 Amazon SES from a PH-opened AWS account

| Point | Finding | Source |
|---|---|---|
| Sandbox | All new accounts start in the sandbox, **per region**: send only to verified addresses, max **200 messages / 24 h**, **1 / second** | [L33] OD (AWS docs), H |
| Exit process | Console → Account dashboard → Request production access: pick Transactional or Marketing, website URL, contacts, and acknowledge "only send email to individuals who've explicitly requested it" and that bounces/complaints are handled. Verifying the domain first "helps to get your production access request approved faster". AWS gives "an initial response … within 96 hours"; longer if they need more information | [L33] OD, H |
| Typical approval issues | Forum reports of denials for new accounts with **no paid billing history** ("show a pattern of use of other AWS services and a consistent paid billing history"), and denials with no reason. Not confirmed by official AWS docs | [L34] F (AWS re:Post), L–M |
| PH-specific restriction | None found. Region: ap-southeast-1 (Singapore) is the nearest SES region; sandbox status is per region, so request production access in that region | [L33] H (per-region rule); region choice inference, M |
| Price | **US$0.10 per 1,000** outbound (à la carte); attachments $0.12/GB. New accounts: up to **US$200 Free Tier credits**, free plan lasts 6 months, credits usable within 12 months | [L35] PP, H |
| 1,000 emails | US$0.10 ≈ **₱6.10** at ~₱61/US$ (BSP reference, Aug 2026 [L40], M), ≈ ₱6.83 with 12% VAT on digital services (RA 12023 [L41], TP, M; whether AWS bills PH VAT not confirmed). In practice ₱0 while Free Tier credits last | computed |

### 4.2 Resend and Postmark free tiers

| | Resend | Postmark |
|---|---|---|
| Free allowance | **3,000 transactional emails/month, capped at 100/day**, 3 domains, 30-day data retention; separate free marketing plan (1,000 contacts) | **100 emails/month**, does not expire |
| Next tier | Pro US$20/mo for 50,000 | Basic US$15/mo for 10,000; extra $1.80/1,000 |
| Transactional vs marketing | Separate allowances; AUP bans "unsolicited messages of any kind", purchased/scraped lists; "All mail must be sent to recipients who have explicitly opted in"; bulk mail needs one-click opt-out honoured within 7 days | Separate "Message Streams"; "transactional and marketing emails never mix" |
| Source | [L36] PP, H; [L37] OD (AUP), H | [L38] PP, H |

**Fit for P1 (inference, M).** Pilot volume (5 firms × 30 clients × ~4 emails) ≈ 600/month, inside Resend free, but one firm chasing 60 clients × 2 contacts on a deadline day hits the 100/day cap. Postmark's 100/month is test-only. **Use Resend free for pilots; move to SES once production access is granted**, which costs pennies at P1's volume. **Consent risk:** P1's recipients are the bookkeeper's clients, who never opted in to P1. Both the SES acknowledgement and Resend's AUP require opted-in recipients. These are one-to-one service messages from a party the recipient already deals with, which is the transactional case, but the terms should make the bookkeeper warrant that the client agreed to receive document requests by email. *(Inference, M.)*

### 4.3 Domain and authentication

| Record | Purpose | Needed? | Source |
|---|---|---|---|
| **DKIM** (SES Easy DKIM: CNAME records published from the SES console) | Signs mail; aligns with the From domain | Yes | [L39] OD (AWS), H |
| **SPF** via a **custom MAIL FROM** subdomain (e.g. `mail.example.com`: an MX record + SPF TXT) | SPF alignment for DMARC | Recommended | [L39] H |
| **DMARC** TXT at `_dmarc.example.com`, start with `v=DMARC1; p=none; rua=mailto:…`, tighten later | Required by Gmail for 5,000+/day senders; AWS advises a phased p=none → quarantine → reject | Yes (cheap insurance) | [L39] H; [L42] OD (Google), H |
| Gmail rules for **all** senders (since 1 Feb 2024) | SPF **or** DKIM; spam rate < 0.3%. 5,000+/day: SPF **and** DKIM, DMARC (p=none OK), From alignment, one-click unsubscribe for marketing | | [L42] OD, H |

**Domain cost.**
- **.com:** Cloudflare Registrar sells "at or below what registries and ICANN charge" [L43] VD, H. The wholesale .com price is US$10.26 + $0.20 ICANN fee ≈ **US$10.46** until 31 Oct 2026, rising to **US$10.97 + $0.20 ≈ US$11.17 from 1 Nov 2026** [L44] TP (Domain Name Wire, 2026-04-23), M–H. So about **₱640–680/yr**, ≈ ₱55/month. Registering or renewing before 1 Nov 2026 locks the lower price for the term bought [L44] (single commentator, M).
- **.ph:** cheapest registrars **US$41.36–44.00 first year, US$44.99–59.66 renewal** (snapshot 2026-10-20) [L45] TP aggregator, M, i.e. ≈ ₱2,520–2,750/yr. A local blog quotes ₱2,940/yr [L46] TP, L. Whether Cloudflare sells .ph: not found.
- **Recommendation:** .com. A .ph costs ~4× as much and takes most of a month's ₱600 budget.

## Cheapest legal path to the first paid customer

*Recommendation, built from the findings above. Not owner-approved.*

**During the pilots (weeks 1–4): about ₱680 cash**
1. Buy a **.com** domain, ideally **before 1 Nov 2026** when the wholesale price rises. About ₱640–680/yr.
2. Set up DKIM, an SPF custom MAIL FROM and DMARC `p=none`. Send pilot mail through **Resend free**. ₱0.
3. Ask for **SES production access in ap-southeast-1** in week 1, with the domain verified first. Denials happen to new accounts, so leave time to retry. ₱0 (within US$200 credits).
4. Paperwork: privacy notice; a short pilot **DPA covering the nine IRR §44 clauses** and the sub-processor list; a written designation of the owner as DPO; a one-page 72-hour breach procedure; the **NPC sworn declaration of exemption** (notary fee not found). ₱0 plus the notary fee.

**Week 3 (so registration is in hand by the first invoice): about ₱560 known cash**

5. **DTI BNRS**, city/municipality scope: ₱530. ₱230 barangay scope is the legal floor.
6. Email the city **BPLO and the barangay** (written, which fits the no-calls rule) to ask whether a home-based online service needs a clearance and mayor's permit, and the fee. Pay it if required. **Cost unknown.**
7. **BIR ORUS Form 1901**: choose the **8% option**, pay the ₱30 DST, get the eCOR and the NIRI, register **manual books**, and arrange BIR-printed or ATP invoices. Invoice and book costs not found.

**End of week 4: first charge**

8. The bookkeeper sends **₱1,500 to the owner's personal, fully verified GCash** (₱0 fee) and gets a BIR invoice.
9. File the quarterly 1701Q. Tax is ₱0 while gross stays under ₱250,000/yr.

**Total known cash before the first ₱1,500: about ₱1,240** (₱680 domain + ₱530 DTI + ₱30 DST). On top of that come **unpriced items**: barangay clearance, mayor's permit (if the LGU requires them), the invoice booklet, a journal/ledger notebook, and one notarisation. Floor if the owner trades under their own name and skips DTI: about ₱710 plus the same unknowns.

**Later, not before the first charge:** NPC registration within 20 days of passing 1,000 individuals (about 20 firms); PayMongo once self-serve card or automated payments matter; the RR 16-2023 sworn declaration if a merchant e-wallet/gateway account is opened; and a look at BMBE.

## Unresolved questions

1. **LGU permit:** does the owner's own city/barangay require a barangay clearance and mayor's permit for a home-based online software service, and what does it cost? No reliable source. Ask the BPLO in writing.
2. **Free pilots without BIR registration:** no source answers this directly. Ask the RDO in writing, or accept the inference in §1.5.
3. **Does the owner already have DTI or BIR registration** (for example as a freelancer)? If BIR-registered, they need an update (Form 1905) rather than a 1901. Not recorded in the repo.
4. **NPC Circular 2022-04 primary text:** whether sub-1,000 processors must always file the sworn declaration, any registration fee, and whether automated decision-making always triggers registration. privacy.gov.ph returns 403/bot-check to our tools; try a real browser.
5. **Official NPC privacy-notice or DPA templates:** not found. The NPC Privacy Toolkit was seen only as a third-party copy.
6. **Owner as their own DPO** in a one-person business: no NPC text found. The inference is that it is allowed.
7. **Cost of a BIR-printed invoice booklet or ATP printing, manual books, and notarisation:** not found.
8. **BIR ORUS processing time** for a new self-employed registration: official figure not found.
9. **GCash's terms on business receipts into a personal wallet**, and GCash for Business / Maya fees: not found.
10. **Whether AWS / Resend / Cloudflare add 12% PH VAT** under RA 12023 on the owner's bills: not confirmed.
11. **SES approval for an SES-only new account:** forum reports of denials for "no billing history". Real risk not measurable; Resend Pro (US$20/mo) is the fallback, and it exceeds the ₱600 budget by itself.
12. **Consent wording:** whether the bookkeeper's warranty that clients agreed to email reminders satisfies the SES and Resend consent terms. This is an inference, not a provider statement.

## Sources

All accessed 2026-10-21. Format: tag, publisher, title, URL, date, evidence type, confidence. OD = official, VD = vendor docs, PP = vendor pricing page, TP = third-party, F = forum, SR = taken from a search-result summary rather than a fetched page.

**Business registration and tax**
- [L1] DTI, BNRS FAQ (fees, DST, validity, eligibility), https://bnrs.dti.gov.ph/faq ; definition of business name from https://bnrs.dti.gov.ph/, n.d., OD, H
- [L2] Wise PH, "DTI online registration: guide for Filipino freelancers", https://wise.com/ph/blog/dti-online-registration ; Triple i Consulting, "DTI online registration: a practical guide", https://www.tripleiconsulting.com/dti-online-registration-practical-guide-for-business-name-registration/, 2025–26, TP, SR, M
- [L3] Respicio & Co., "Mayor's Permit Requirements for Online Businesses in the Philippines", https://www.respicio.ph/commentaries/mayors-permit-requirements-for-online-businesses-in-the-philippines, n.d. (self-described AI-assisted), TP, L–M
- [L4] PTV News, "Zambo City registers home-based businesses", https://ptvnews.ph/?p=115296, n.d., news, SR, M
- [L5] KPMG Philippines, "Checklist of Documentary Requirements (Annex CDR 2025c)", https://assets.kpmg.com/content/dam/kpmgsites/ph/pdf/InTAX/2025/Annex-CDR-2025c.pdf, 2025, TP (reproduces BIR checklist), SR, M
- [L5b] P&A Grant Thornton, "Clarifications on the registration procedures under EOPT Act", https://www.grantthornton.com.ph/insights/articles-and-updates1/tax-notes/clarifications-on-the-registration-procedures-under-eopt-act/ ; batasnatin, RMC 91-2024 summary, https://batasnatin.com/laws/rmc-no-91-2024-clarification-on-registration-procedures-pursuant-to-revenue-r, 2024, TP, SR, M
- [L6] Respicio & Co., "BIR Registration Requirements for Online Sellers", https://www.respicio.ph/commentaries/legal-remedies-for-a-forged-deed-or-fraudulent-property-transfer-3 ; Lawyer-Philippines, "How to Obtain BIR Registration Online", https://www.lawyer-philippines.com/articles/how-to-obtain-bir-registration-online, n.d., TP, SR, L–M
- [L7] Baker McKenzie, "Philippines: Annual Registration Fee will not be collected starting 22 January 2024", https://insightplus.bakermckenzie.com/bm/tax/philippines-annual-registration-fee-will-not-be-collected-starting-22-january-2024, 2024-01, TP (law firm), SR, M–H
- [L8] Philippine Information Agency, "BIR waives annual registration fee for business taxpayers", https://mirror.pia.gov.ph/news/2024/01/15/bir-waives-annual-registration-fee-for-business-taxpayers, 2024-01-15, government news, SR, H
- [L9] P&A Grant Thornton, Tax Alert "Mandatory registration of persons engaged in business and administrative sanctions and criminal liabilities for non-registration", https://www.grantthornton.com.ph/technical-alerts/tax-alert/2024/mandatory-registration-of-persons-engaged-in-business-and-administrative-sanctions-and-criminal-liabilities-for-non-registration/, 2024-09-03, TP (fetched), H for deadline, M–H for penalties
- [L10] P&A Grant Thornton, "Availing yourself of 8% tax rate for self-employed individuals", https://www.grantthornton.com.ph/insights/articles-and-updates1/tax-notes/availing-yourself-of-8-tax-rate-for-self-employed-individuals/ ; Juan.tax, "Know taxes: 8% income tax rate", https://juan.tax/blog/know-taxes-8-income-tax-rate ; Deloitte PH, "Tax in a dot, June 2018", https://www.deloitte.com/content/dam/Deloitte/ph/Documents/tax/ph-tax-in-a-dot-june2018.pdf, 2018–2026, TP, SR, M–H
- [L11] Forvis Mazars PH, "BIR replaces 'Ask for Receipt' notices … (RMO 43-2022)", https://www.forvismazars.com/ph/en/insights/tax-alerts/bir-rmo-43-2022 ; Triple i Consulting, "BIR extends deadline for transition to ARN and NIRI", https://www.tripleiconsulting.com/deadline-extension-bir-transition-ask-receipt-notice-arn-notice-issue-receipt-invoice-niri/, 2022–23, TP, SR, M
- [L18] P&A Grant Thornton, "BIR issues guidelines on electronic invoicing, retains 31 Dec 2026 deadline" (RMC 98-2026), https://www.grantthornton.com.ph/technical-alerts/tax-alert/2026/bir-issues-guidelines-on-electronic-invoicing-retains-31-dec-2026-deadline/ ; Sovos, "Philippines: deadline for mandatory structured e-invoicing extended", https://sovos.com/regulatory-updates/vat/philippines-deadline-for-mandatory-structured-e-invoicing-extended/, 2026, TP, SR, M
- [L19] P&A Grant Thornton, "BIR clarifies bookkeeping options for new business registrants" (RMC 65-2025), https://www.grantthornton.com.ph/insights/articles-and-updates1/tax-notes/bir-clarifies-bookkeeping-options-for-new-business-registrants/, 2025-07-16, TP (fetched), M
- [L20] DTI, "DTI offers free registration for barangay-based ventures", https://dti.gov.ph/negosyo/exports/emb-news/dti-offers-free-registration-for-barangay-based-ventures ; DTI BMBE flyer, https://esigaw.dti.gov.ph/wp-content/uploads/2024/07/Updates-BMBE-Flyer.pdf, n.d./2024, OD, SR, M
- [L21] Alburo Law Offices, "Register as BMBE and enjoy the perks", https://www.alburolaw.com/register-as-bmbe-and-enjoy-the-perks/, n.d., TP, SR, L–M
- [L22] Respicio & Co., "Penalties for Failure to Register Business with BIR", https://www.respicio.ph/commentaries/penalties-for-failure-to-register-business-with-bir, n.d., TP, SR, L–M

**Getting paid**
- [L12] PayMongo Docs, "Philippine Entities" (activation documents), https://docs.paymongo.com/docs/account-settings-philippine-entities, n.d., VD (fetched), H
- [L13] GCash Help Center, "What are the requirements to create a GCash for Business merchant account?", https://help.gcash.com/hc/en-us/articles/48456974006041, n.d., OD/VD, SR (fetch 403), M–H
- [L13b] itsourcecode, "How to Set Up GCash for Business in 2026", https://itsourcecode.com/gcash/how-to-set-up-gcash-for-business-2026/, 2026, TP, SR, L (contradicts L13)
- [L14] GCash Help Center, "What are my GCash wallet and transaction limits?", https://help.gcash.com/hc/articles/360021112894, n.d., OD/VD, SR, H
- [L15] Maya, "Be an Agent" (sole-prop documents), https://www.maya.ph/mayacenter/agent/be-an-agent ; Maya Business Support, Pro upgrade with BIR 2303, https://businesssupport.maya.ph/s/article/Can-I-use-my-existing-Maya-account-to-log-in-to-the-new-Maya-Business-How ; Maya, "Payment Links", https://www.maya.ph/business/paymentlinks, n.d., VD, SR, M
- [L16] PayMongo, Pricing, https://www.paymongo.com/pricing, n.d., PP (fetched), H
- [L17] PayMongo Developers, "Activate account", https://developers.paymongo.com/docs/account-activation, n.d., VD, SR, M
- [L23] Maya, "Withholding Tax Updates on your Maya Business Account" (RR 16-2023), https://www.maya.ph/business/withholdingtaxupdates, n.d. (effective 2024-10-13), VD (fetched), H

**Data privacy**
- [L24] Lawphil, Republic Act No. 10173, https://lawphil.net/statutes/repacts/ra2012/ra_10173_2012.html, 2012, statute mirror, H (cited via [p1-kill-checks.md](p1-kill-checks.md) [K30])
- [L25] ChanRobles, Implementing Rules and Regulations of RA 10173 (§§26, 38, 44–47 quoted), https://chanrobles.com/laws/republicacts/102_republicacts.php?id=10712, 2016, regulation mirror (fetched), M–H
- [L26] Alburo Law Offices, "Designation of Data Protection Officers" (NPC Advisory 2017-01), https://www.alburolaw.com/designation-of-data-protection-officers/, 2022-06-01, TP (fetched), M. Primary: https://privacy.gov.ph/wp-content/uploads/2022/01/NPC-Advisory-2017-01-sgd-1.pdf (not reachable)
- [L27] NPC, Advisory Opinion No. 2024-006, https://privacy.gov.ph/wp-content/uploads/2024/06/Advisory-Opinion-No.-2024-006.pdf, 2024, OD, SR, M
- [L28] SyCip Salazar Hernandez & Gatmaitan via Mondaq, "All Things Registration: National Privacy Commission Issues New Rules On Registration Of Data Processing Systems", https://mondaq.com/privacy-protection/1319392/all-things-registration-national-privacy-commission-issues-new-rules-on-registration-of-data-processing-systems, 2023-05-23, TP (fetched), M
- [L29] Baker McKenzie, "Philippines: New NPC circular on registration of Data Protection Officers and Data Processing Systems takes effect", https://insightplus.bakermckenzie.com/bm/data-technology/philippines-new-npc-circular-on-registration-of-data-protection-officers-and-data-processing-systems-takes-effect_1, 2023-01, TP, SR, M
- [L30] NPC, registration FAQ / reminder pages, https://privacy.gov.ph/?p=1853 ; https://privacy.gov.ph/?p=43514, n.d., OD, SR (fetch 403), M
- [L31] NPC, "NPC Circular 16-03 – Personal Data Breach Management", https://privacy.gov.ph/?p=553, 2016-12-15, OD, SR, M
- [L32] Scribd, "Privacy Toolkit" (third-party copy of NPC toolkit), https://www.scribd.com/document/364393417/Privacy-Toolkit ; Genie AI PH templates, https://www.genieai.co/en-ph/template/data-processing-addendum, n.d., TP, SR, L

**Email and domain**
- [L33] AWS, "Request production access (Moving out of the Amazon SES sandbox)", https://docs.aws.amazon.com/ses/latest/dg/request-production-access.html, n.d., OD (fetched), H
- [L34] AWS re:Post, "SES production access denied due to no billing history" and "SES production access request is denied", https://repost.aws/questions/QUmX1zW3R_RaikO2vGCGUdpA/ses-production-access-denied-due-to-no-billing-history-circular-limitation-issue ; https://repost.aws/questions/QUXOCq4PgJQI2x-5ar3c_NiQ/ses-production-access-request-is-denied, n.d., F, SR, L–M
- [L35] AWS, "Amazon SES pricing", https://aws.amazon.com/ses/pricing/, n.d., PP (fetched), H
- [L36] Resend, Pricing, https://resend.com/pricing, n.d., PP (fetched), H
- [L37] Resend, Acceptable Use Policy, https://resend.com/legal/acceptable-use, n.d., OD (vendor terms, fetched), H
- [L38] Postmark, Pricing, https://postmarkapp.com/pricing, n.d., PP (fetched), H
- [L39] AWS, "Complying with DMARC authentication protocol in Amazon SES", https://docs.aws.amazon.com/ses/latest/dg/send-email-authentication-dmarc.html, n.d., OD (fetched), H
- [L40] Philnews, BSP reference rate table (₱60.968/US$, 2026-08-10), https://philnews.ph/?p=1359307, 2026-08-10, TP, SR, M. An October 2026 rate was not found; ₱61/US$ is used as an approximation
- [L41] EY Tax News, "Philippines passes VAT on digital services law affecting nonresident providers" (RA 12023), https://taxnews.ey.com/news/2024-1874-philippines-passes-vat-on-digital-services-law-affecting-nonresident-providers, 2024-10-11, TP, SR, M
- [L42] Google Workspace Admin Help, "Email sender guidelines", https://support.google.com/a/answer/81126, n.d., OD (fetched), H
- [L43] Cloudflare, "Cloudflare Registrar", https://www.cloudflare.com/products/registrar/, n.d., VD (fetched), H
- [L44] Domain Name Wire, "Breaking: Verisign raising wholesale .com prices", https://domainnamewire.com/2026/04/23/breaking-verisign-raising-wholesale-com-prices/ (plus search summaries of industry posts on the ICANN fee), 2026-04-23, TP (trade press), SR, M–H
- [L45] DomainOffer, ".PH domain price comparison", https://domainoffer.net/tld/ph, snapshot 2026-10-20, TP aggregator (fetched), M
- [L46] worldngayon.com, "Blog startup cost Philippines", https://worldngayon.com/blog-startup-cost-philippines/, 2026-09, TP (blog), SR, L
