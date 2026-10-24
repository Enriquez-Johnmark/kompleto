# Product names: trademark and name-clash check

Status: research note, 2026-10-24. **Not legal advice and not an owner decision.** A Philippine trademark lawyer, or an IPOPHL search report, should confirm each name before anything is filed. Applies to the provisional names in `docs/decisions.md` (2026-10-23, "Product portfolio and names").

**How this was checked.** All sources were accessed 2026-10-24. Each claim cites a numbered source with an evidence type: **OD** official or registry data, **VD** vendor docs or store listing, **VC** vendor claim, **N** news, **TP** third party, **SR** search-result excerpt only. Confidence is **H**, **M** or **L**.

- **Trademarks.** IPOPHL's own search tools would not load from this environment. `onlineservices.ipophil.gov.ph` and `www.ipophil.gov.ph` answer with an Imperva/Incapsula bot challenge, and `/esearch/` returns HTTP 403 [S3]. The WIPO Global Brand Database answers with an ALTCHA proof-of-work challenge before any search [S4]. These were not bypassed. Instead, Philippine marks were searched in **TMview**, the EUIPO-run registry aggregator. Its records come from IPOPHL's own feed (record links point to `asean-tmview.org`) and include Madrid marks designating the Philippines. Its JSON search API was queried directly [S1]. Each name was searched as "contains" and as "fuzzy" (TMview's similarity match), filtered to the Philippine office and to Philippine territory. Similar spellings were searched too (balance, completo, skedyul, cotizacion, record, recibo, rekordbox, cairn, loop…), and the unique names were also searched across every office.
- **Coverage gap: the most important caveat.** The newest Philippine application in TMview is dated **2025-05-01** [S2]. So Philippine filings from May 2025 to today (about 18 months) **were not checked**, and statuses may be stale. The Philippines is first-to-file: registration goes to the earliest filing (IP Code §123.1(d)), so a recent filing by someone else is exactly the risk this check cannot see [S15]. Read every "none found" below as "none found in IPOPHL data up to 2025-05-01".
- **Business registries.** DTI's BNRS search sits behind a captcha, and the SEC sites (`sec.gov.ph`, `checkwithsec.sec.gov.ph`, `esparc.sec.gov.ph`) return HTTP 403 [S36]. **DTI/SEC business names: not checked.**
- **Same-name products.** Checked through web search, Google Play (PH store) search and listings, GitHub repository search, crates.io/npm/PyPI, RDAP (`.com`), Google DNS (`.ph`/`.com.ph` NS records) and certificate-transparency logs.
- **.ph caveat.** dotPH answers every unregistered name with a wildcard A record (`45.79.222.138`, seen for a random test name), so only a **NS record** shows a name is registered [S37]. A registered but undelegated `.ph` name would also look free.
- Domain findings the caller gave were not redone; they are cited as [S0].

## Summary

| Name (product) | PH trademark result (to 2025-05-01) | Main clashes | Best domain | Verdict |
|---|---|---|---|---|
| **Loopwright** (engine) | None in PH | **US application LOOPWRIGHT** (Gradera, Inc., class 42, SN 99942826, filed 2026-07-15) for almost the same service: human oversight and approval of AI-agent work. 19 GitHub repos named loopwright, several of them AI-agent loop tools; `.com/.dev/.io/.ai` all registered | getloopwright.com | **Switch** |
| **Cairnpost** (engine backup) | None anywhere in TMview | **CairnPost**, a proprietary CRM for small service businesses (Firefly Software, Montana), in development, which owns cairnpost.com (registered 2026-03-14) | getcairnpost.com | **Keep with caution**, but weaker than the alternatives below |
| **Kompleto** (bookkeeper chaser) | None identical or close in PH; KOMPLETT (class 7/9/20) ended | Foreign only: KOMPLETO registered in Brazil (classes 9/35/36) and Spain (35). No PH product found | getkompleto.com; kompleto.ph free | **Keep** |
| **Balanse** (payment follow-up) | BALANSE SKINCARE **registered**, class 3; BALANSE class 25 **filed**; THE BALANCE (Dotdash, Madrid, designating PH) **registered** in classes 9/41/42 for finance-content apps | Balanse Skincare owns balanse.ph | balanseapp.com | **Keep with caution** |
| **Sakto** (backup) | ~60 PH marks contain SAKTO, incl. SAKTO (ABS-CBN, 41, registered), TRANSAKTO (AC Infrastructure Holdings, 36, registered), GO SAKTO (Globe, 38, ended) | **GCash "GLoan Sakto"** loans (Fuse Lending); sakto.ph and .com.ph taken; sakto.com is Sakto Corporation | getsakto.com | **Clash: don't fall back to it** |
| **Kotasyon** (quotation follow-up) | None in PH; nothing in software classes anywhere | None found | getkotasyon.com; kotasyon.ph free | **Keep** |
| **Kontrata** (backup) | KONTRATA.PH, class 45, ended | None found | getkontrata.com; kontrata.ph free | Usable backup |
| **Biyahe** (tour-inquiry follow-up) | No standalone BIYAHE mark. Travel-field marks: BIYAHEKO.PH (39, registered 2024), BIYAHEROES (35/39, registered), BIYAHERO (42, registered), EBIYAHE (39, registered) | Active PH **"Biyahe" travel-itinerary app** (Google Play, 5K+ installs, updated Sep 2026); biyahe.ph and .com.ph taken; 128 GitHub repos | getbiyahe.com | **Keep with caution, or re-decide** |
| **Turista** (backup) | **TURISTA and TURISTA PH registered** (Turista.ph Technologies, Inc., classes 9/35/38/39/43, filed 2022) | Covers travel software and tour arranging | (getturista.com taken) | **Clash: unusable** |
| **Iskedyul** (clinic reminders) | None in PH; nothing in software classes anywhere | Only student GitHub projects | getiskedyul.com; iskedyul.ph free | **Keep** |
| **Klinika** (backup) | Only ended or unrelated marks (KLINIKAR, QX KLINIKA BOTIKA…) | **klinika.ph is a clinic management system** for doctors, and the KlinikaHub appointment app is in alpha: the same field | (getklinika.com taken) | **Clash** (moot, since Iskedyul stays) |
| **Resibo** (seller records) | Only ended or unrelated (PREMYO SA RESIBO, RESIVO class 9 ended 2023) | **Three PH products in the same field:** resibo.com.ph (BIR e-invoicing for Shopee/TikTok sellers), "Resibo by InvoicePH" (Shopify), "ResiboNow" (BIR invoicing app); resibo.ph parked | resiboapp.com | **Switch to Rekord** |
| **Rekord** (backup) | REKORD (Lesaffre, class 30, yeast) registered; **REKKORD** (Madrid, designating PH, class 9, music/audio software) registered | Indian record-keeping app "Rekord" (now Recordbook, 1M+ installs); small "Rekord Ledger" and "Rekord" screen-recorder apps; JS ORM `rekord` on npm | getrekord.com; rekord.ph free | **Adopt, with caution** |

**Names the owner must re-decide:** the **engine** (Loopwright clashes; Cairnpost is only a cautious backup), and **Biyahe** (its backup Turista is unusable, so if Biyahe's own risk is too much there is no fallback). Alternatives for both are in §4. **Resibo → Rekord** follows the agreed fallback rule, but it renames a planned product, so the owner should confirm it.

**A risk that applies to all the Filipino names (inference, M).** The IP Code bars marks that consist *exclusively* of generic or descriptive words for the goods (§123.1(h)–(j)); a descriptive mark can register only after it becomes distinctive, with 5 years' use as prima facie proof (§123.2) [S15]. Kompleto, Balanse, Kotasyon, Iskedyul, Klinika, Resibo and Rekord are ordinary Filipino words, and several describe what the product handles (a receipt app called "Resibo", a schedule app called "Iskedyul"). IPOPHL may refuse a word mark, or grant one too weak to stop look-alikes; a stylised logo mark is the usual workaround. Whether the Office treats each word as descriptive for its goods is for the lawyer: not found here.

## 1. Engine: Loopwright and Cairnpost

### Loopwright: switch

- **Identical US application, same field.** LOOPWRIGHT, US serial **99942826**, filed **2026-07-15** by **Gradera, Inc.** (Delaware corporation, 675 15th Street, Suite 2400, Denver, CO). Standard-character word mark, class 42, filed on an intent-to-use basis, status "LIVE/APPLICATION/Awaiting Examination". The attorney of record is Bonnie Page, Macro Law Group [S5] (OD, H). The services include *"SaaS … providing a unified dashboard and unified inbox for human oversight, review, approval, and completion of work tasks within business processes involving artificial intelligence agents"*, and *"exception management, escalation notifications, and audit reporting"* [S5][S6] (OD, H). That is close to Loopwright's own description: human-approved loops, audit log, AI drafts.
- Gradera's site describes "Software-Orchestrated Services" and "digital workers that collaborate with humans … with governance", with a Denver, CO address [S7] (VC, M: it is the same company name and city as the filing, but the site does not mention Loopwright).
- The filing predates Loopwright's repository (created 2026-10-23) [S8]. A US filing gives no rights in the Philippines, but Loopwright is a public, international, developer-facing project, and the US is where a dispute over a GitHub-hosted name is most likely (inference, M).
- **Crowded name.** GitHub has 19 repositories named loopwright. Several are AI-agent loop tools: dtkmn/loopwright ("Flight recorder for AI agent loops", since 2024-06), inhaq/loopwright ("Loop engine for orchestrating autonomous coding agents"), alksandr/loopwright, rebase-lat/loopwright [S8] (TP, H). A Google Play app "Loopwright: Hum to Song" exists (SocialSure Pvt Ltd) [S12] (VD, H).
- **Domains.** loopwright.com is registered (2025-09-13, Namecheap, Cloudflare DNS). loopwright.dev (Vercel), .io and .ai all have nameservers [S9] (OD, H). getloopwright.com is free [S0].
- PH trademarks: **none found** in PH data up to 2025-05-01; across all TMview offices the only hit is the US application [S1] (OD, H for the data, M for completeness).
- crates.io, npm, PyPI: `loopwright` is free [S13] (OD, H).

**Cost of switching (inference, H).** Loopwright is a day old, so renaming now is cheap: the repo, the crate name, Kompleto's `Cargo.toml` git dependency, ADR 0006 and the docs. The v0.1.0 tag's commit doesn't change.

### Cairnpost: keep with caution (backup)

- **Trademarks:** no CAIRNPOST or CAIRN POST mark in any TMview office, and no CAIRN mark in PH software classes [S1] (OD, H to 2025-05-01).
- **Same-name product:** "CairnPost: a lean, handcrafted CRM for small service businesses. Part of the Firefly Software product suite … Built by Firefly Software in Helena, Montana … In development. Not yet accepting users. License: Proprietary" (GitHub `DukeRupert/cairnpost`, created 2026-02-28) [S10] (VD, H). cairnpost.com was registered on 2026-03-14 at Scaleway [S11] (OD, H); presumably it belongs to the same project (inference, M).
- The fields differ (a CRM vs a developer engine), but both sit in small-business follow-up software, and Firefly owns the .com. If Firefly files a mark, Cairnpost becomes a second Loopwright (inference, M). Web search found no other Cairnpost software [S40] (SR, M).
- Domains: getcairnpost.com free [S0]; cairnpost.ph free [S37].

## 2. Kompleto: keep

- **PH trademarks:** no KOMPLETO, KUMPLETO or COMPLETO in PH data. The fuzzy search finds only KOMPLETT (Gong Suzhen, classes 7/9/20, **Ended**, PH-4-2019-508883, filed 2019-12-13) [S1] (OD, H to 2025-05-01).
- **Foreign marks (not PH):** KOMPLETO and KOMPLETO KARD, Anderson Ladeira Viana, Brazil, classes 9, 35 and 36, **registered** (925433357/925433411/925433497, filed 2022-01-13). KOMPLETO, TU Comercial Digital Marketing y Comunicacion SL, Spain, class 35, registered (M4351258, filed 2025-12-05) [S14] (OD, H). Neither designates PH; they matter only if Kompleto is ever sold in Brazil or Spain.
- **Products:** no PH app or business named Kompleto found [S40] (SR, M); Google Play PH: none [S42] (VD, M). GitHub: the owner's repo, plus `kompletomost-source/kompleto-web` (empty-looking HTML repo, purpose unknown) [S8] (TP, L).
- Domains: getkompleto.com free [S0]; kompleto.ph and kompleto.com.ph free [S37].

## 3. Products

### Balanse: keep with caution (backup Sakto clashes)

PH marks found [S1] (OD, H to 2025-05-01):

| Mark | Owner | Class | Status | Filing no. | Filed |
|---|---|---|---|---|---|
| BALANSE SKINCARE | Gozun, Jose Martina | 3 (face creams) | Registered | PH-4-2023-504909 | 2023-05-11 |
| BALANSE SKIN CARE | Gozun, Jose Martina | 3 | Ended | PH-4-2021-523028 | 2021-09-28 |
| BALANSÈ SKIN CARE | Balansè Skin Care Products Trading | 3 | Ended | PH-4-2021-510403 | 2021-05-04 |
| BALANSE | Carlos, Abegail Punzalan | 25 (clothing) | Filed | PH-4-2024-513342 | 2024-05-13 |
| DISBALANSE | Nolasco, Arnold | 25 | Ended | PH-4-2023-530834 | 2023-11-29 |
| THE BALANCE | Dotdash Media Inc. (Madrid, designating PH) | 9, 41, 42: "downloadable mobile applications for obtaining information … in the fields of finance and financial services" | Registered | WO 1354219 | 2016-12-13 |
| BALANCE | Shinsung Tongsang Co., Ltd. | 35 (sales agency for clothing and bags) | Registered | PH-4-2024-502625 | 2024-01-30 |
| CITI BALANCE TRANSFER / CITI BALANCE CONVERSION | Citigroup Inc. | 36 | Registered | PH-4-2008-7904 / -7905 | 2008-07-03 |

- None is BALANSE for software. The nearest is **THE BALANCE**: class 9 and 42, finance-related, and "balanse" sounds like "balance". Its goods are finance articles and search, not payment reminders, so confusion is arguable rather than likely (inference, M). Skincare and clothing are unrelated goods (inference, H).
- Abroad: BALANSE was filed in the US (Balanse, Inc., class 9; Baird & Warner, class 35), all **Ended** [S1] (OD, H).
- Balanse Skincare holds balanse.ph [S0]. No PH payment or invoicing product named Balanse was found [S40] (SR, M); Google Play PH shows only "Balansert" (unrelated) [S42] (VD, M).
- Domain: **balanseapp.com** free [S0]; balanse.com.ph free [S37].

**Sakto, the backup, is worse.** About 59 PH marks contain SAKTO [S1] (OD, H), among them SAKTO (ABS-CBN, class 41, registered, PH-4-2023-514362), TRANSAKTO (AC Infrastructure Holdings, class 36, registered, PH-4-2020-503553 and -2021-507220), GO SAKTO / GOSAKTO (Globe Telecom, class 38, ended) and COCA-COLA SAKTO (32, registered). In the payments field itself, **GCash and Fuse Lending sell "GLoan Sakto"** nano-loans (Unbox.ph, 2023-09-05) [S18] (N, H). sakto.ph and .com.ph are taken and sakto.com is Sakto Corporation [S0].

### Kotasyon: keep (backup Kontrata is usable)

- **Kotasyon:** no PH mark, contains or fuzzy [S1] (OD, H to 2025-05-01). Across all offices the only hits are "hizli kotasyon" and "online kotasyon" (Turkish reinsurance broker, class 36, **Ended**, 2016) [S1] (OD, H); "kotasyon" is also the Turkish word for "quotation" (inference, M). No product, app or GitHub repo found [S40][S42][S8] (SR/VD/TP, M).
- **Kontrata:** one PH mark, KONTRATA.PH (Rhaegon Collection and Sales Corporation, class 45, **Ended**, PH-4-2018-5827, filed 2018-04-05) [S1] (OD, H). No product found [S40] (SR, M).
- Domains: getkotasyon.com free [S0]; kotasyon.ph and .com.ph free [S37]. getkontrata.com free [S0]; kontrata.ph free [S37].

### Biyahe: keep with caution or re-decide (backup Turista clashes)

PH marks containing BIYAHE in or near the field [S1] (OD, H to 2025-05-01):

| Mark | Owner | Class | Status | Filing no. | Filed |
|---|---|---|---|---|---|
| BIYAHEKO.PH | Biyaheko.com Corporation | 39 (operating tours, travel reservation, itinerary planning) | Registered | PH-4-2024-518753 | 2024-07-09 |
| BIYAHEROES | Oscar Bernabe Simbulan III | 35 (reservation system for hotels, buses, activities), 39 | Registered | PH-4-2015-3581 | 2015-03-31 |
| BIYAHERO | Cubos, Michael | 42 (design and development of software) | Registered | PH-4-2021-514766 | 2021-06-23 |
| EBIYAHE | Tudtud, Melanie; Tudtud, Raymund | 39 | Registered | PH-4-2024-516187 | 2024-06-07 |
| BIYAHE LOKAL | Biyahe Lokal Corporation | 39 | Ended | PH-4-2015-504630 | 2015-08-14 |
| VIYAHE | Must Be Sun Retail Corp | 9, 18 | Filed | PH-4-2023-528333 | 2023-10-31 |

- No standalone BIYAHE mark exists in the data. But the travel field holds several BIYAHE-based marks, two of them for tour reservations (BIYAHEKO.PH, BIYAHEROES). A tour-operator product named Biyahe sits close to them; whether that is confusing similarity is for the lawyer (inference, M).
- **Same-name product:** Google Play app **"Biyahe"** (developer "Biyahe", 5K+ installs, updated 2026-09-21): "Biyahe is like AllTrails but for travel … browse other people's itineraries … focused on Luzon, Philippines" [S19] (VD, H). It is consumer, not B2B, but in the same travel field. GitHub has 128 biyahe repos, e.g. `biyahe-atlas/biyahe` (finance platform for ride-hailing drivers) [S20] (TP, H). biyahe.ph and .com.ph are taken [S0].
- Domain: getbiyahe.com free [S0].

**Turista, the backup, is unusable.** Turista.ph Technologies, Inc. holds **registered** TURISTA (PH-4-2022-511574), TURISTA PH (-511577, -511587, -511592), TURISTA.PH (-511591), TURISTAPH_ (-511588), TURISTARS (REWARDS) (-511582) and TURISTA.PH TECHNOLOGIES, INC. (-511575), all filed 2022-05-16. They cover class 9 (application software, travel software), 35, 38, 39 ("arranging travel tours; travel agency services …") and 43 [S1][S21] (OD, H). getturista.com is taken [S0].

### Iskedyul: keep (backup Klinika clashes)

- **Iskedyul:** no PH mark (contains, fuzzy; "skedyul" and "schedule" in software classes also none) [S1] (OD, H to 2025-05-01). Across all offices the fuzzy match is ISKEDYL, a Pierre Fabre drug brand in class 5 in many countries, not PH [S1] (OD, H): unrelated goods. No product found [S40] (SR, M). GitHub: about 16 small student projects (timetable managers) [S39] (TP, H).
- Domains: getiskedyul.com free [S0]; iskedyul.ph and .com.ph free [S37].
- **Klinika** (backup): PH marks are only ended or unrelated (QX KLINIKA BOTIKA, 35, ended; KLINIKAR, Sime Darby, ended; CLINIKA ONLINE, 44, filed 2018) [S1] (OD, H). But **klinika.ph** is "Klinika – Clinic Management System for Doctors" [S22] (VC, H), and **KlinikaHub** (Develop Kreativity, Google Play, alpha) books clinic appointments [S23] (VD, H). Both are in the same field, so Klinika would clash. Moot while Iskedyul stays.

### Resibo: switch to Rekord

**PH marks:** none for RESIBO alone. RESIBO-containing marks are ended or unrelated: PREMYO SA RESIBO (PhilWeb, classes 28/35/38/41/42, ended; Canada, Rico, classes 9/41, ended 2022), RESIBO BLOWOUT (Petron, 4, ended) and RESIBON NIKKEN (7). Fuzzy finds **RESIVO** (Prism Code Philippines Inc., class 9 "computer software, recorded", **Ended**, PH-4-2023-515622, filed 2023-06-28) [S1] (OD, H to 2025-05-01). The 2025–2026 filings are the gap: **whether resibo.com.ph's operator has filed is not checked** [S2].

**resibo.com.ph in depth:**
- **What:** "BIR e-invoicing for online sellers": it connects Shopee and TikTok Shop, turns orders into structured e-invoices and transmits them to the BIR, ahead of the 31 Dec 2026 e-invoicing deadline [S24] (VC, H). Plans run ₱1,499–9,999/month by order volume, plus Enterprise. "Book a FREE demo"; a "Pilot Program promo code" gives a free month to the first sellers. "Built by a mandated seller, for mandated sellers." "Resibo is software, not a tax adviser" [S24] (VC, H).
- **Who:** "Resibo is operated by **Tri-Point Axis**" (Terms of Service, version 0.2, in effect 2026-09-27) [S25] (VC, H). The Privacy Policy (v0.2, in effect 2026-10-01) names Tri-Point Axis as responsible under the Data Privacy Act and gives privacy@resibo.ph [S26] (VC, H). Whether Tri-Point Axis is a DTI business name, an SEC company or something else: **not found** (DTI/SEC not reachable; web search found nothing) [S36][S40].
- **Since when:** its first TLS certificate was issued 2026-06-13. Later ones cover www (2026-07-08), admin (2026-08-24), mail and webmail (2026-08-26) and demo (2026-09-15) [S27] (OD, H). DNS is at dotPH (`dns1/dns2.domains.ph`) with its own mail server [S28] (OD, H). resibo.ph points to a parking service (parklogic) [S28] (OD, M), although the privacy policy uses a resibo.ph address.
- **How established:** young, about 4 months live, at version 0.2 terms, in a pilot/demo phase. No news coverage, reviews or BIR accreditation listing found [S40] (SR, M). The site, pricing and legal pages are complete and professional (inference, M).
- **Registered mark?** Not in IPOPHL data to 2025-05-01; anything filed since is **not checked**.

**Other Resibo products in the same field:**
- **"Resibo by InvoicePH"** on the Shopify App Store, by Shopconcile (Lucban, Quezon), launched 2026-05-11. It creates PH invoice PDFs with TIN for orders; free plan, Starter US$9/month; 0 reviews [S29] (VD, H).
- **"ResiboNow – Freelancer Tool"** (Wuffalo), Google Play, beta, 10+ installs: "a free, BIR-compliant invoicing and receipt app built for Filipino freelancers … and small-business owners" [S30] (VD, H).
- **"Resibo"** by Load Daddy INC, Google Play, 500+ installs: receipt-scanning rewards [S31] (VD, H).
- GitHub has 80 resibo repos, among them `nkcardel/resibo` ("AI bookkeeping for Filipino sari-sari stores") and `alyssavallejos/resibo` (receipt verification) [S32] (TP, H).

Three live products use the name in PH seller invoicing or receipts, and resibo.com.ph targets the exact same customers (online sellers, BIR deadlines). **Resibo clashes** (inference, H).

### Rekord: adopt, with caution

- **PH marks:** REKORD (Lesaffre et Compagnie, class 30, **registered**, PH-4-2020-512776, filed 2020-09-15; an earlier filing expired). **REKKORD** (Heinrich Lichtenegger, Madrid WO 1741180, designating PH, class 9, **registered**, filed 2023-05-25), whose goods are electrical cables, music recordings and *"downloadable computer software for recording, storing, processing … digital audio and video"* [S1][S41] (OD, H). Also REBOLUSYON REKORDS (9/41, ended). No REKORD in classes 35/36/42 [S1] (OD, H to 2025-05-01).
- REKKORD is class 9 software, but audio/music software, a different market from sales records. Partial overlap; low-to-moderate risk (inference, M). Abroad, REKORD is registered in software classes in Poland, Slovenia and Russia, among others [S1] (OD, H); none designates PH.
- **Products:** the Indian app "Rekord" (`com.foodmonk.rekordapp`) is now **Recordbook**: rekord.in redirects to recordbook.io, "digital register app for Indian small businesses", 1M+ installs on Google Play [S33] (VD, H). It has rebranded away from Rekord and does not target PH (inference, M). Small apps: "Rekord Ledger" (Anickode, 10+, personal expense diary) and "Rekord" screen recorder (Safilink, 1K+, not updated since 2022) [S34] (VD, H). GitHub: `Rekord/rekord`, a JavaScript ORM (180 stars), which is also the npm package [S35] (TP, H). Pioneer's "rekordbox" (DJ software) has no PH mark in the data [S1].
- Domains: getrekord.com free [S0]; rekord.ph and .com.ph free [S37]. crates.io and PyPI free; npm taken [S13].
- Caution: "rekord" is close to the English word "record", which describes sales records (see the descriptiveness note in the Summary).

## 4. Alternatives where both names are in doubt

Checked 2026-10-24: TMview fuzzy search (PH and all offices) [S1], RDAP for `.com` (404 = free), Google DNS NS records for `.ph`/`.com.ph` [S37], and GitHub repository search [S8].

### Engine (replaces Loopwright)

These keep Loopwright's own pattern, an English "-wright" (maker) coinage, rather than a Filipino word. The engine is developer-facing and international, so a Filipino word would lose the point of the name (recommendation, M). If the owner wants Filipino here too, ask for a fresh round.

| Name | Meaning | TM (PH / all offices) | .com | get-.com | .ph | GitHub repos | Notes |
|---|---|---|---|---|---|---|---|
| **Nudgewright** (recommended) | "maker of nudges" (reminders) | none / none | **free** | free | free | 0 | `.dev` free; crate free; web search finds no use [S40] |
| **Remindwright** | "maker of reminders" | none / none | **free** | free | free | 0 | Plainer, longer |
| **Replywright** | "maker of replies" (the loop stops on reply) | none / none | taken | free | free | 0 | `.dev` free; crate free |

Rejected: Chasewright ("Chase Wright" is a personal name with GitHub accounts), Keepwright (KEEPRIGHT marks in CN/DO), Tallywright (US TALLY-RITE, class 9), Sendwright (NZ Post's SENDRIGHT, classes 9/35/42) [S1].

### Tour-inquiry product (if Biyahe is dropped)

These are Filipino spellings of English words a foreigner can guess (Kompleto's style). The spellings follow common Tagalog loanword patterns but were **not checked against a dictionary** (L).

| Name | Meaning | TM (PH / all offices) | .com | get-.com | .ph / .com.ph | GitHub | Notes |
|---|---|---|---|---|---|---|---|
| **Reserbasyon** (recommended) | "reservation", where an inquiry should end | none / one Turkish filing "reservasyon" (classes 35/39/41/43, filed 2024, TR only) | **free** | free | free / free | 0 | Fits the job: inquiry → booking |
| **Ekskursiyon** | "excursion" | none / none | **free** | free | free / free | 0 | |
| **Itineraryo** | "itinerary" | none in PH / ITINERARIO marks in Argentina, Uruguay, Brazil | taken | free | free / free | 0 | |

## 5. What to do before filing (recommendation)

1. Commission an **IPOPHL search report**, or have a PH trademark lawyer search, covering the final names, especially filings after 2025-05-01 that TMview lacks. If possible, check DTI/SEC for "Tri-Point Axis" and for each final name.
2. File early where it matters. The Philippines is first-to-file (§123.1(d)); apart from well-known marks, an unregistered mark has limited protection, although a business or trade name is protected even without registration (§165.2) [S15]. How that applies to an existing user such as resibo.com.ph is a question for the lawyer. Filing steps and fees were not checked here.
3. Treat the engine name as the urgent one: an identical US application in the same niche already exists.
4. Re-run this check if a name is chosen more than a few weeks from now; new GitHub repos and apps under these names appeared almost monthly in 2026 [S8].

## Sources

All accessed 2026-10-24.

- [S0] Caller's brief for this task: earlier RDAP/DNS checks in the same session (every bare `.com` registered; get-/-app `.com` and `.ph` results as listed). Registry data, not re-run. OD, H.
- [S1] EUIPO TMDN, **TMview** search API, `https://www.tmdn.org/tmview/api/search/results` (front end `https://www.tmdn.org/tmview/`). Queries: each name, "C" (contains) and "F" (fuzzy), filtered `fOffices:["PH"]` and `fTerr:["PH"]`; similar spellings filtered to classes 9/35/36/38/42/44; unique names across all offices. Record details from `/tmview/api/trademark/detail/<ST13>`. Raw results were kept in the session scratchpad, not in the repo. OD, H for records found; M for absence (see [S2]).
- [S2] TMview, PH office sorted by application date, newest first: the top results are PH-4-2025-511403…511456, all filed 2025-05-01. OD, H.
- [S3] IPOPHL, `https://onlineservices.ipophil.gov.ph/`, `/tmsearch/`, `/esearch/` and `https://www.ipophil.gov.ph/trademark/`: Imperva/Incapsula challenge page or HTTP 403 to scripted access. OD (observed), H.
- [S4] WIPO, Global Brand Database, `https://branddb.wipo.int/en/similarname`: ALTCHA challenge (`https://api.branddb.wipo.int/captcha`) before any search. OD (observed), H.
- [S5] USPTO, TSDR status, US serial 99942826 "LOOPWRIGHT", `https://tsdr.uspto.gov/statusview/sn99942826` (page generated 2026-10-23 23:53 EDT). OD, H.
- [S6] TMview record US500000099942826 (goods and services, class 42), `https://www.tmdn.org/tmview/api/trademark/detail/US500000099942826`. OD, H.
- [S7] Gradera, home and company pages, `https://www.gradera.ai/` (gradera.com redirects there). VC, M.
- [S8] GitHub repository search (`<name> in:name`) via the GitHub API: loopwright 19 results (incl. dtkmn/loopwright, inhaq/loopwright, alksandr/loopwright, rebase-lat/loopwright, Enriquez-Johnmark/loopwright created 2026-10-23); kompleto 4; nudgewright, replywright, remindwright, kotasyon 0. TP, H.
- [S9] Verisign RDAP, `https://rdap.verisign.com/com/v1/domain/loopwright.com` (registered 2025-09-13, NameCheap); Google DNS NS for loopwright.dev (vercel-dns), .io (registrar-servers), .ai (porkbun). OD, H.
- [S10] Firefly Software, `https://github.com/DukeRupert/cairnpost` README (`https://raw.githubusercontent.com/DukeRupert/cairnpost/main/README.md`), repo created 2026-02-28. VD, H.
- [S11] Verisign RDAP, cairnpost.com (registered 2026-03-14, registrar Scaleway SAS). OD, H.
- [S12] Google Play, "Loopwright: Hum to Song", `https://play.google.com/store/apps/details?id=com.mohdshayan.loopwright` (100+ installs, updated 2026-09-23). VD, H.
- [S13] crates.io (`/api/v1/crates/<name>`), npm registry, PyPI JSON API: 404 for every name except npm `rekord` (200); also 404 for nudgewright, replywright, chasewright crates. OD, H.
- [S14] TMview, KOMPLETO across all offices: BR 925433357/925433411/925433497 (and KOMPLETO KARD), ES M4351258. OD, H.
- [S15] Republic Act 8293, Intellectual Property Code of the Philippines, §§123.1, 123.2, 165.2. Text at Lawphil, `https://lawphil.net/statutes/repacts/ra1997/ra_8293_1997.html`. OD (official text via a secondary host), H.
- [S18] Duey Guison, "GCash and Fuse Make Lending More Accessible with GLoan Sakto", Unbox.ph, 2023-09-05, `https://unbox.ph/news/gcash-and-fuse-make-lending-more-accessible-with-gloan-sakto/`. N, H.
- [S19] Google Play, "Biyahe", `https://play.google.com/store/apps/details?id=com.biyahe.app` (5K+ installs, updated 2026-09-21). VD, H.
- [S20] GitHub, `https://github.com/biyahe-atlas/biyahe` (created 2026-07-05); biyahe search total 128. TP, H.
- [S21] TMview record PH502022000511574 (TURISTA, goods in classes 9/35/38/39/43). OD, H.
- [S22] Klinika, `https://klinika.ph/` (title "Klinika – Clinic Management System for Doctors"). VC, H.
- [S23] Google Play, "KlinikaHub", `https://play.google.com/store/apps/details?id=com.klinikahub.app` (Develop Kreativity, 100+ installs, alpha, updated 2026-07-01). VD, H.
- [S24] Resibo, home page, `https://resibo.com.ph/`. VC, H.
- [S25] Resibo, Terms of Service v0.2 (in effect 2026-09-27), `https://resibo.com.ph/terms`. VC, H.
- [S26] Resibo, Privacy Policy v0.2 (in effect 2026-10-01), `https://resibo.com.ph/privacy`; contact page `https://resibo.com.ph/contact`. VC, H.
- [S27] crt.sh certificate-transparency search, `https://crt.sh/?q=resibo.com.ph` (8 certificates, earliest not-before 2026-06-13). TP (log aggregator of CA records), H.
- [S28] Google DNS: resibo.com.ph NS dns1/dns2.domains.ph, MX mail.resibo.com.ph; resibo.ph A record answered by ns2.jd24.parklogic.com. OD, H (DNS) / M ("parked" is inferred from the parking nameserver).
- [S29] Shopify App Store, "Resibo by InvoicePH", `https://apps.shopify.com/invoiceph` (Shopconcile, Lucban, PH-QUE; launched 2026-05-11; 0 reviews). VD, H.
- [S30] Google Play, "ResiboNow – Freelancer Tool", `https://play.google.com/store/apps/details?id=app.resibonow` (Wuffalo, 10+ installs, updated 2026-10-08). VD, H.
- [S31] Google Play, "Resibo", `https://play.google.com/store/apps/details?id=com.resibocash.app` (Load Daddy INC, 500+ installs). VD, H.
- [S32] GitHub, `https://github.com/nkcardel/resibo`, `https://github.com/alyssavallejos/resibo`; resibo search total 80. TP, H.
- [S33] `https://rekord.in` → `https://recordbook.io/` ("Recordbook — Digital Register App for Small Business India"); Google Play `https://play.google.com/store/apps/details?id=com.foodmonk.rekordapp` ("Record Book Excel RegisterBook", 1M+ installs). VD, H.
- [S34] Google Play, `com.anickode.rekord` ("Rekord Ledger", 10+) and `com.deltaloop.rekord.app` ("Rekord", Safilink, 1K+, updated 2022-10-28). VD, H.
- [S35] GitHub, `https://github.com/Rekord/rekord` (JavaScript REST ORM, 180 stars). TP, H.
- [S36] DTI BNRS, `https://bnrs.dti.gov.ph/search` (captcha `idp.dti.gov.ph/cdn/captcha`); SEC `https://www.sec.gov.ph/`, `https://checkwithsec.sec.gov.ph/`, `https://esparc.sec.gov.ph/` (HTTP 403). OD (observed), H.
- [S37] Google Public DNS, `https://dns.google/resolve?name=<name>.ph&type=NS` (and `.com.ph`) for kompleto, kotasyon, kontrata, iskedyul, rekord, turista, cairnpost, loopwright and the §4 candidates: no NS record. The wildcard `45.79.222.138` is returned for an unregistered test name (`xqzvkw81723.ph`). Verisign RDAP for the §4 candidates. OD, H (DNS) / M (free ≠ unregistered, see method).
- [S39] GitHub, iskedyul search (16 results, e.g. `jmarkfen/iskedyul`, a Django timetable manager). TP, H.
- [S40] Web searches (US-based search engine) for each name with "app", "Philippines" or the product field: no same-name product found for Kompleto, Kotasyon, Kontrata, Iskedyul, Cairnpost, Nudgewright/Replywright, Reserbasyon/Ekskursiyon, or "Tri-Point Axis". SR, M (exact-name recall was weak; a gap stays a gap).
- [S41] TMview record WO500000001741180 (REKKORD, class 9 goods). OD, H.
- [S42] Google Play search, `https://play.google.com/store/search?q=<name>&c=apps&hl=en&gl=PH`, for all 13 names (matches listed above). VD, M.
