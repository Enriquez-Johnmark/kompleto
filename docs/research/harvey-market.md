# Harvey market: sell it (P-R) or use it (P-SELF)?

| | |
|---|---|
| Date | 2026-10-20 (all sources accessed 2026-10-20 unless stated) |
| Status | Research complete (gaps listed under Unresolved questions). Not an owner decision. |
| Inputs | `harvey-teardown.md` (commit e114e95), `scoring-method.md`, `scoring/score.py` (candidate P, 60.6 Balanced, 0/16 evidence), `integration-constraints.md` |
| Labels | Evidence type: **P** primary/official, **VC** vendor claim, **TP** third-party, **F** forum/user report. Confidence: H/M/L. |

## Key conclusions

1. **Selling a Rust Harvey (P-R) has no evidenced gap.**
   - Harvey is free and MIT-licensed, and growing fast: 1,221 → 1,776 stars and 221 → 354 forks in two days, with ~4 merged PRs a day [H1][H2].
   - Apollo has a free plan. Instantly ($37–47/mo) already ships a human-in-the-loop AI reply mode [C1][C2].
   - Approval-first, self-hosting and low price are all already taken. Rust is not something buyers ask for: Harvey users ask for providers, Maps discovery, warmup and a better review desk [H7].
   - **Score: P-R = 62.8 Balanced (#30), C6 = 1 (disqualifier).**
2. **The money near Harvey is in the services around it, not the agent.** Precleared (compliance checks), Reoon (verification) and Tellwing (sending) all use Harvey PRs as a distribution channel [H2][H5][H6]. No fork or hosted Harvey is commercialised (not found) [H3][H4].
3. **A hosted product cannot inherit Harvey's "no API keys" model.** Anthropic's Claude Code terms: developers "should use API key authentication" and Anthropic does "not permit third-party developers to … route requests through Free, Pro, or Max plan credentials on behalf of their users" [L9]. A hosted P-R therefore pays for the LLM itself, and its Gmail read/send also needs annual CASA (~$540+/yr) [integration-constraints.md].
4. **Cold email is legal but heavily constrained, and the platform terms bind harder than the law.**
   - No PH anti-spam statute is in force (RA 10175 §4(c)(3) was struck down in *Disini*, 2014) [L1].
   - The NPC allows direct marketing on legitimate interest after an assessment [L3][L5].
   - CAN-SPAM applies to B2B (up to $53,088 per email) [L6]. UK PECR allows email to corporate subscribers but not sole traders [L7].
   - But the Workspace AUP bans facilitating "unsolicited mass email" [D6], Gmail's policy bans "unsolicited commercial mail" [D4], and Anthropic's Usage Policy bans facilitating spam [L11].
5. **Framing (b) is viable only small and by hand.**
   - The PRC lists 283 accredited CPA firms and a 76-page individual list, and BIR had 5,358 accredited tax agents in 2022, but **none of these lists has emails** [F1][F2].
   - The Maps Places API has no email field and forbids storing its content except place IDs [F5][F6].
   - Expect a few hundred firms with published business emails, found one by one.
6. **P-SELF is a feature of P1's core, not a product.** Harvey's outbox, sequencing and stop-on-reply loop is the same durable core as P1's chase loop. Discovery, scraping and warmup are not needed at 20–40 emails a week. The scoring method does not fit an internal tool; a forced string is given for reference only.

## Harvey today

All figures are from `gh api` on 2026-10-20 [H1] (P, H) unless stated.

| Metric | 2026-10-18 (teardown) | 2026-10-20 | Change |
|---|---|---|---|
| Stars | 1,221 | **1,776** | +555 in ~2 days |
| Forks | 221 | **354** | +133 |
| Watchers | 14 | 20 | |
| Open issues + PRs | 11 | 11 (4 issues, 7 PRs open) | |
| Releases | v0.2.1, v0.3.0 (both 2026-10-13) | no new release | |
| Last push | 2026-10-15 | 2026-10-20 14:51 UTC | |

- **Commits since e114e95.** 22 commits on `main` from 2026-10-15 to 2026-10-20 [H2] (P, H). They are maintenance merges by the owner of community PRs:
  - dashboard access protection and isolated Claude calls (#30, which addresses two teardown findings: dashboard CSRF and `CLAUDE.md`/settings leaking into `claude -p`);
  - discovery data clean-up (#23/#31) and cohort pagination and CSV export (#21/#32);
  - a Reoon email-verification fix, a "company inbox" fallback (`info@`-style addresses) and an opt-in "Humanizer" rewrite pass (#29/#36);
  - Precleared checkout links (#16/#33).
- **Growth is a launch spike, not proven use.** About 45% of all stars arrived in two days. One fork's description links an Instagram reel [H3] (P, M). Star velocity measures attention, not senders. Not found: any count of active installs, emails sent, or a testimonial from a paying business that used Harvey.
- **Forks are not commercialising it.** The 15 most-starred forks have at most 1 star each and keep the upstream description unchanged [H3] (P, H). Not found: a hosted Harvey, a paid tier, or a fork sold as SaaS. The maintainer's profile links ethanplus.ai [H4]; whether it sells a hosted Harvey was not checked beyond the repo (gap).
- **Vendors are using Harvey as a distribution channel.** Three paid services have shipped or proposed integrations through PRs:
  - **Precleared** (pre-send legal/compliance check per recipient): 10 checks/day keyless, 25/day with a free key, then prepaid packs; its contributor is the second-largest committer (13 commits) [H2][H5] (P, H for the code; pricing VC).
  - **Tellwing** (email platform), PR #13, with the author's disclosure "I build Tellwing" [H6] (P, H).
  - **Reoon** (email verification), merged in #36 [H2] (P, H).
  - **Lesson:** the money around an open-source outbound agent sits in the paid services next to it (data, verification, compliance, sending infrastructure), not in the agent itself.
- **What users ask for** (open items, 2026-10-20) [H7] (P, H; small sample, F-grade signal):
  - more model providers: Google Antigravity (#9, #10), DeepSeek per-token API (#35), newer GPT versions (#14, closed);
  - **Google Maps / Places as a discovery source** (#28): local-business discovery is the core use case;
  - **warmup readiness, send ramp and region scheduling** (#18, #19): deliverability is the pain users hit first;
  - **a better approval desk**: show the facts behind each draft and allow editing (#34). Approval-first is used, and the current desk is too thin;
  - a **CAN-SPAM postal address and opt-out footer** (#13, still open): the compliance gap from the teardown is still not merged;
  - localisation (German #24, pt-BR #26) and an MCP server (#35);
  - #20 (an Indian school lead-gen system spec) and #27 ("sales", empty) are people treating the repo as a lead-gen service.
- **Complaints.** No issue reports deliverability failures, bans or bugs from real campaigns. The issue tracker is too young (7 months old, most activity in the last 8 weeks) to show field use.

**Read-out.** Harvey is a fast-growing, free, MIT-licensed, actively merged project. A Rust rewrite would compete with a free incumbent that already has momentum, 354 forks and vendors building on it, and its owner merges about 4 PRs a day. Its users' top asks (deliverability, discovery data, a better review desk) are mostly not about the language it is written in.

## Competitor matrix

Prices are USD per month unless stated. They come from vendor pricing pages fetched on 2026-10-20 (VC, M: several pages contradict themselves, as noted) or from third-party roundups (TP, L–M). "Draft-and-approve" means a human can hold AI-written cold or reply email for review before it sends.

### Cold-email sending platforms (self-serve)

| Tool | Entry price | AI features | Draft-and-approve? | Source |
|---|---|---|---|---|
| **Instantly** | Outreach Growth **$47/mo** ($37.6 annual), 5,000 emails, 1,000 contacts. Bundles from $94 (Starter) to $555 (Agency). The page contradicts itself on some tiers. | AI Sales Agent, AI Reply Agent, AI email writer, web-researcher agent, credits | **Yes.** The AI Reply Agent has a "human in the loop" mode: drafts wait in Unibox or Slack for edit and approval. Each draft costs 5 credits even if never sent. | [C1] VC; [C2] VC (help centre) |
| **Smartlead** | Base **$39/mo** ($32 annual), 6,000 emails, 2,000 contacts. Pro $94, Unlimited Smart $174, Prime $379 | AI warmup. AI copy sold as a managed service. No AI reply agent on the pricing page | Not found | [C3] VC |
| **lemlist** | Email plan **$69/user/mo** ($55 annual), 50,000 emails/mo. Multichannel $109 ($87) | lemAgent; signal, enrichment and messaging agents; AI reply intent; AI-generated replies | Not stated on the pricing page ("campaign review", "step preview" exist; unclear for AI content) | [C4] VC |
| **Apollo** | **Free plan** (Gmail only; sequencing included). Basic ~$49, Professional ~$79, Organization ~$119 per user/mo annual (~$59/$99/$149 monthly) | AI email writer and research on Professional (unspecified on the page) | Not stated | [C5] VC (page has no prices); [C6] TP for prices |
| **Reply.io** | Email Volume ~$49/user/mo annual (~$59 monthly); Multichannel ~$89. Jason AI SDR is a separate product from ~**$500/mo** (one source says $259) | Jason AI SDR | Not found | [C7] TP (official page returned 403) |
| **Woodpecker** | Usage-based: **$7 per 100 contacted prospects**; mailboxes $4–6 each; integrations add-on $20 | AI email writer, interest detection, Claude integration | Not found | [C8] VC |
| **GMass** (Gmail add-on) | **₱1,200/mo** Standard (₱833 annual), "unlimited" emails subject to account limits | AI campaign creation, AI spintax, Spam Solver | Not found | [C9] VC (PH-localised prices) |

### AI SDRs (autonomous, sales-led)

| Tool | Price | Draft-and-approve? | Source |
|---|---|---|---|
| **11x** (Alice) | ~$3,750/mo annual (~$45k/yr) per one 2026 guide; others estimate $50–90k/yr. Not published by the vendor | Not found | [C10] TP, L |
| **Artisan** (Ava) | Credit-based. Third-party estimates $400–$2,000+/mo; one says $600/mo annual. Pricing "has shifted multiple times" | Not found | [C11] TP, L |
| **AiSDR** | Pay-as-you-go **$1.50 per person prospected**; team plans unpublished; managed service $2,500/mo | **Partly:** "co-pilot mode" for replies (unexplained). The $149/campaign managed service says "You review and approve before anything sends" | [C12] VC |
| **Regie.ai** | ~$180–$499/user/mo with a 10-seat minimum and annual contract (TP); "AI Agents" package from ~$35k | Not found | [C13] TP, L |

### Open-source and self-hosted (GitHub search, 2026-10-20)

| Project | Stars | License | Note | Source |
|---|---|---|---|---|
| **ethanplusai/harvey** | **1,776** | MIT | The incumbent. Free, approval-first by default on native providers, BYO Claude/ChatGPT subscription | [H1] P |
| PaulleDemon/Email-automation | 184 | other | Open-source cold-email sender, no AI agent focus | [C14] P |
| Kandid-ai/AgentSDR | 47 | MIT | "Open-source AI SDR": email, LinkedIn, WhatsApp, enrichment, AI CRM; has a hosted site (agentsdr.ai) | [C14] P |
| pypesdev/coldflow | 19 | GPL-3.0 | "Stop paying enterprise prices for cold email" | [C14] P |
| issacops/opencloser-v2 | 15 | MIT | Open-source AI SDR and calling | [C14] P |
| personizeai/revenue-os | 12 | MIT | "Open-source alternative to … Artisan, 11x" | [C14] P |
| xyluxx/openoutbound | 5 | Apache-2.0 | AI SDR engine with MCP server, CLI, REST | [C14] P |
| ZUES-ops-dot/OutreachIQ | 0 | — | The only **Rust** cold-email engine found (multi-tenant, warmup) | [C14] P |

**Read-out.** Harvey is the only open-source outbound agent with real traction; its nearest rival has under 3% of its stars. No Rust project has traction. Many small "open-source AI SDR" repos launched in 2026, which is a sign of a crowded, low-barrier category rather than an open one.

## Gaps

Each candidate gap is tested against the evidence above.

| Candidate gap | Evidence | Verdict |
|---|---|---|
| **Approval-first** | Harvey does it free by default (native providers). Instantly's AI Reply Agent has a human-in-the-loop mode [C2]. AiSDR has a co-pilot mode and approval in the managed service [C12]. Harvey PR #34 (a richer review desk with edits) is open [H7]. | **No gap.** At best a UX edge (a better review desk), which Harvey is about to merge. |
| **Price** | Apollo has a free plan [C5]. Harvey is free (BYO subscription). Smartlead $39, Instantly $47, Woodpecker pay-per-use [C3][C1][C8]. | **No gap** at the low end. The gap exists only against AI SDRs ($500–$4,000/mo) [C10–C13], and those buyers are sales-led companies we cannot reach. |
| **Self-hosting / privacy** | Harvey is self-hosted with a local SQLite file. Several MIT/Apache repos exist [C14]. | **No gap.** Harvey already owns "self-hosted, your data stays local". |
| **Rust (single binary, lower resource use, safety)** | Harvey users ask for providers, discovery data, deliverability and a better review desk (#9, #18, #28, #34, #35). Nobody asks for a smaller footprint or for speed [H7]. | **No buyer-visible gap.** Rust is a builder benefit, not a buying reason, in this category. |
| **Correctness and safety** (exactly-once sends, auth on the dashboard, injection hardening; see teardown) | Real, but Harvey merged dashboard protection and Claude isolation on 2026-10-20 (#30) [H2]. The rest can be fixed upstream in days. | **Thin, closing.** Not a moat. |
| **PH/SEA agencies and local SMB outbound** | Not found: any PH-specific cold-email tool or demand signal. GMass already prices in pesos (₱833–1,200/mo) [C9]. Harvey's local-business discovery is generic (Maps request #28). | **Unproven.** A PH segment would need its own demand evidence, which we cannot gather by interview. The legal picture (below) is also weaker for PH-to-PH cold email than for US. |
| **Hosted Harvey (managed, multi-user)** | Not found: anyone offering it. But a hosted product that reads and sends from customers' Gmail needs restricted scopes and an annual CASA assessment, and a hosted product cannot legally run on customers' consumer Claude subscriptions (see Legal). | **Real but expensive.** It turns a free tool into a paid SaaS that competes with Instantly/Smartlead on deliverability, with LLM and CASA costs we must pay. |

**Conclusion.** No evidenced gap a solo PH developer can own. The best sub-variant of P-R is **"hosted, approval-first Harvey for solo founders and small agencies"** (open-core: MIT engine plus a paid hosted tier with API-key LLM, a better review desk, a deliverability checklist and a compliance footer). Even this competes on price with Instantly ($37–47) and Smartlead ($32–39), which already include sending infrastructure and warmup, and with Harvey itself, which is free.

## Deliverability & platform rules

### Mailbox-provider rules

| Provider | Applies to everyone | Applies to bulk senders (>5,000/day to that provider) | Source |
|---|---|---|---|
| **Gmail** (from 2024-02-01) | SPF **or** DKIM, valid PTR, TLS; spam rate in Postmaster Tools "below 0.3%". Ideal is "below 0.10%" | adds DMARC with alignment; marketing messages "must support one-click unsubscribe" plus a visible unsubscribe link | [D1] P, H |
| **Yahoo** | SPF or DKIM; spam rate "below 0.3%"; valid forward/reverse DNS | SPF **and** DKIM, DMARC at least p=none and aligned; one-click List-Unsubscribe (RFC 8058); "Honor unsubscribes within 2 days" | [D2] P, H |
| **Microsoft Outlook.com** (consumer: hotmail, live, outlook.com; from 2025-05-05) | — | SPF, DKIM and DMARC. Non-compliant mail from domains sending "over 5,000 messages daily" goes to Junk, with rejection to follow at a later date | [D3] TP (Petri, 2025-04-07) quoting Microsoft; the Microsoft page did not render, M |

**What this means for a small sender.** At Harvey's ~50 emails/day, no bulk-sender rules apply. The hard limit is the **spam-complaint rate**: 0.3% is 3 complaints per 1,000 emails. Cold email to people who did not ask for it is where complaints come from, so a few hundred badly targeted emails can damage a domain. That is why Harvey's users ask for warmup checks and send ramps (PR #18) [H7]. Setting up SPF, DKIM and DMARC is cheap anyway and should be done regardless of volume.

### Account-provider terms (more binding than the volume rules)

- **Personal Gmail:** "Don't use Gmail to distribute spam or unsolicited commercial mail" and "You are not allowed to use Gmail to send email in violation of the CAN-SPAM Act or other anti-spam laws" [D4] (P, H). The limit is 500 emails/day [D5] (P, H).
- **Google Workspace:** the acceptable-use policy forbids using the services "to generate, distribute, publish or facilitate unsolicited mass email, promotions, advertisements, or other solicitations" ("spam") [D6] (P, H). Limits are 2,000 messages/day per user and 3,000 external recipients/day [D7] (P, H).
- **Implication.** Low-volume, individually written, relevant B2B emails are widely sent from Workspace accounts, and "mass" is not defined. But automated cold sequences run from a Google account sit in a grey zone that Google can enforce by suspending the account. **A hosted P-R product would pass this risk on to every customer, and our own sending domain would carry it under P-SELF.** No Google statement found that defines a safe volume (gap).

### Hosted product: Gmail API and Microsoft Graph verification

This is from `integration-constraints.md` §3, not redone here:

- A hosted multi-user product that reads replies needs restricted Gmail scopes (`gmail.readonly`/`gmail.modify`). Storing that data on our servers triggers an **annual CASA security assessment**, about **$540–720/yr** for Tier 2 (TAC Security, vendor pricing, M), which takes "several weeks" [integration-constraints.md §3, refs 35, 36, 40].
- Send-only (`gmail.send`) is a sensitive scope: 3–5 business days of verification and no CASA. But Harvey's core loop needs to read replies for stop-on-reply, so send-only does not work.
- Microsoft Graph needs publisher verification through the AI Cloud Partner Program (free), and Mail.ReadWrite usually needs admin consent [integration-constraints.md, refs 44, 45].
- **CASA at ~$540+/yr is roughly 75% of a year of our ₱600/month budget** (₱7,200/yr ≈ $125 at ~₱58/$; the exchange rate is an assumption). **It blocks a hosted P-R until revenue arrives.** Exceptions cover personal use, "only a few users", and development [integration-constraints.md ref 36], so P-SELF (one user: us) and a few pilots are unaffected.
- **Workaround:** SMTP/IMAP with app passwords or a dedicated sending provider avoids OAuth verification, at a cost in user experience and security (integration-constraints.md, ref 43).

## Legal

This is research, not legal advice. Where a conclusion is our reading, it is labelled **interpretation**.

### Philippines

- **There is no PH anti-spam statute in force.**
  - The Cybercrime Prevention Act's ban on unsolicited commercial communications (RA 10175 §4(c)(3)) was **struck down as unconstitutional** in *Disini v. Secretary of Justice* (G.R. No. 203335, February 2014) on free-expression grounds [L1] (TP: Columbia Global Freedom of Expression case summary, H).
  - Anti-spam bills keep being filed: Senate SB 2460 (2021, texts/calls), House Bills 797/485/1342/1687 under committee review in December 2025, and HB 270 (consent before promotional calls, texts and emails). **None has passed, as far as found** [L2] (TP: news and Senate records via search, M). The status of the 2025 House bills was not found.
- **The Data Privacy Act (RA 10173) applies to named business contacts** (a person's name and work email are personal information).
  - **NPC Circular 2023-04 (Guidelines on Consent), §14:** where processing is limited to personal (not sensitive) information, a controller "may consider direct marketing as a legitimate interest under Section 12(f)" and need not get consent, but it must first assess whether direct marketing falls within its legitimate interest. Otherwise consent is required [L3] (P via search excerpt and secondary guide [L4]; the NPC PDF returned 403, M).
  - **NPC Advisory Opinion 2023-016:** "the DPA does not consider the soft opt-in approach as valid consent for purposes of marketing communication", so the controller may rely on legitimate interest under §12(f), which needs the three-part test: legitimate purpose, necessity, and balancing against the data subject's rights [L5] (P via search excerpt, M).
  - **Interpretation.** Cold email from a PH sender to a PH business contact found on a public listing can rest on legitimate interest, *if* we document a legitimate-interest assessment, collect only name, role, business email and the public source, honour objections at once (DPA right to object), and send low volumes that are clearly relevant to the recipient's business. Mass, untargeted sending would fail the balancing test. **Not found:** an NPC decision or opinion specifically on B2B cold email or scraping public directories.
  - **Directory terms matter too.** Copying a professional directory (PICPA, BIR lists) into a marketing list may breach that directory's own terms or purpose limits, even where the DPA allows the email (see Framing (b)).

### United States: CAN-SPAM

- "The law makes no exception for business-to-business email." Requirements: truthful headers, a non-deceptive subject, disclosure that the message is an advertisement, "your valid physical postal address", a clear opt-out, and opt-outs honoured "within 10 business days". Penalties are "up to $53,088" per email (2024 inflation update) [L6] (P: FTC, H).
- CAN-SPAM is opt-out, not opt-in, so cold B2B email is lawful if it complies. **Harvey still does not add the postal address and opt-out footer** (PR #13 is open) [H7], so a default Harvey user sending to the US is non-compliant.

### UK and EU: PECR and GDPR

- **UK PECR:** "You can email or text any corporate body" without consent, but "Sole traders and some partnerships are treated as individuals" and need consent or the soft opt-in. Every message must identify the sender and offer a simple opt-out [L7] (P: ICO, H). Emails to named employees also bring in UK GDPR (legitimate interest assessment). The ICO notes the guidance "is under review" after the Data (Use and Access) Act [L7].
- **EU:** GDPR Recital 47 allows direct marketing as a possible legitimate interest. But national ePrivacy rules decide whether unsolicited B2B email is allowed, and several member states (Germany, for example) require prior consent even for B2B [L8] (TP: law-firm and EDPB-consultation summaries, M; country-by-country detail not verified here).
- **Implication for P-R.** A product sold to "solo founders and agencies" worldwide would have to enforce per-country rules (consent countries, sole-trader rules). That is the exact niche **Precleared** sells into (£39 per 2,000 checks) [H5] (VC).

### Anthropic terms: consumer subscription vs API key

The exact text, accessed 2026-10-20:

- **Claude Code "Legal and compliance"** [L9] (P, H):
  > "**OAuth authentication** is intended exclusively for purchasers of Claude Free, Pro, Max, Team, and Enterprise subscription plans and is designed to support ordinary use of Claude Code and other native Anthropic applications."
  >
  > "**Developers** building products or services that interact with Claude's capabilities, including those using the Agent SDK, should use API key authentication through Claude Console or a supported cloud provider. Anthropic does not permit third-party developers to offer Claude.ai login into their own applications, or to route requests through Free, Pro, or Max plan credentials on behalf of their users. Moreover, developers may not collect, store, or intermediate Claude.ai credentials or session tokens — sign-in to a Claude account must complete through Anthropic's own flow."
  >
  > "Advertised usage limits for Pro and Max plans assume ordinary, individual usage of Claude Code and the Agent SDK."
  >
  > "[Hosting Claude Code in your products] requires agreeing to our Commercial Terms of Service … Customers may not pay for, resell, or intermediate Claude usage on their end users' behalf. Each end user must authenticate with their own Anthropic API key, Claude subscription plan credentials, or 3P inference provider credential…"
- **Consumer Terms** (effective October 8, 2025) [L10] (P, H). Section 3 bars users, "Except when you are accessing our Services via an Anthropic API Key or where we otherwise explicitly permit it", from accessing the Services "through automated or non-human means, whether through a bot, script, or otherwise". It also bars using them "to develop any products or services that compete with our Services" or to "resell the Services".
- **Usage Policy** (the page shows "Effective date: November 12, 2026") [L11] (P, H). It forbids using Claude to "Promote or facilitate the generation or distribution of spam" and to "Utilize automation in account creation or to engage in spammy behavior".
- **Interpretation.**
  1. **A hosted P-R product cannot run on customers' Pro/Max subscriptions.** It must use our API key (we pay for and resell the LLM, under the Commercial Terms) or the customer's own API key. Harvey's "no API keys needed" pitch cannot carry over to a hosted product.
  2. **A self-hosted, open-source P-R** in which each user runs the unmodified `claude` binary on their own subscription is the pattern Harvey uses. The docs say subscriptions support "ordinary use", and the Consumer Terms forbid automated access except where permitted. A 24/7 daemon calling `claude -p` is at best a grey zone that Anthropic "may enforce … without prior notice". Harvey reading the OAuth token from the Keychain to call an undocumented usage endpoint (teardown: `quota.py`) looks close to the "collect, store, or intermediate … session tokens" prohibition.
  3. **For P-SELF, use an API key.** At ≤ 50 drafts a day the cost is small (see P1's LLM budget), and it removes the grey zone entirely.
  4. **Spam:** if outreach is not targeted and low-volume, it risks breaching the Usage Policy whatever the auth method.

## Framing (b): self-use for distribution

**The question.** The owner has no network and no ad budget. Can a Harvey-style engine of our own find P1's first customers (PH bookkeepers and small CPA firms) by low-volume, approval-first cold email?

### Can PH bookkeepers be found legitimately from public lists?

| Source | What it gives | Count | Emails? | Source |
|---|---|---|---|---|
| **PRC: Active Accredited Firms/Partnerships in Public Practice of Accountancy** (as of 2026-09-30) | Firm name, accreditation validity dates | **283 firms** (counted from the PDF) | **No.** Names only | [F1] P, H |
| **PRC: Active Accredited Individual CPAs in Public Practice** (as of 2026-09-30) | Individual CPAs in public practice | 76-page PDF. Text extraction failed, so the **count was not established**; at the firm list's ~50 rows a page it would be ~3,000–3,800 (estimate, L) | Not verified (fields not extracted) | [F1] P |
| **BIR RNAB master list of accredited tax agents/practitioners** | Required by RR 4-2010 to be posted on the BIR site | **5,358** as of Feb 2022 (Manila Times column citing a BIR PDF) | Not found | [F2] TP, M; [F3] P (regulation) |
| **PICPA** | Claims 34,000+ active members in 84 chapters (`ph-industries-services.md` [A1]) | — | **No public member directory found** | [F4] (search, not found) |
| **Google Maps / Places API** | Business listings for "bookkeeping services" etc. Text Search Pro: 5,000 free calls/month, then $32 per 1,000; Enterprise (contact fields): 1,000 free, then $35 per 1,000 | Not measured | **No.** Places has no email field; emails must come from each firm's own website or Facebook page | [F5] P, H |
| **PSA establishment count** | 607 accounting, bookkeeping, auditing and tax-consultancy establishments (2018) | 607 | — | `ph-industries-services.md` [A3] |

- **Places API terms:** "You must not pre-fetch, cache, or store Places API content beyond the allowed exceptions". Only the place ID may be stored indefinitely [F6] (P, H). **Building a stored prospect list out of Maps data breaks the terms**, so the list has to be built from each firm's own published contact page, keeping the Maps place ID only as a pointer. The full Maps Service Terms text did not render (gap).
- **Interpretation.** Several thousand named PH CPAs and tax practitioners are in public government lists, but **none of those lists carries emails**. Each email has to be found by hand or by a scraper on the firm's own site or Facebook page, which is Harvey's "Scout" job and its most ToS-risky part (search-engine scraping). A realistic, defensible list is **a few hundred firms with a published business email**, built slowly. That fits a 20–40 emails/week cadence, which is about the volume the owner already approved for Facebook Page messages (≤ 15–20/week; AGENTS.md).
- **The bigger cost is the segment's own channel habits.** `ph-industries-services.md` already found that PH bookkeepers live in Facebook groups, r/AccountingPH and Viber. Cold email is a secondary channel for them. Whether PH bookkeepers reply to cold email at all: **not found.**

### Is P-SELF the same core as P1's chase loop?

| Engine part | Harvey-style outbound (P-SELF) | P1 document/2307 chaser |
|---|---|---|
| Outbox with approval ladder (`pending_review → approved → sent`) | yes | yes |
| Sequence of follow-ups with delays | yes (3-step cold sequence) | yes (reminders before each cut-off) |
| Stop on reply / on document received | yes | yes (plus "document received" from the bookkeeper) |
| Reply intent classification and drafted answer | yes | yes (Taglish) |
| Idempotent send, kill switch, daily cap | yes | yes |
| Prospect discovery, email finding, verification, warmup | **yes (most of Harvey's code and risk)** | no |
| Per-client deadline calendar, 2307 ledger, Viber "copy & open" handoff | no | **yes** |
| Recipient relationship | strangers (cold) | the bookkeeper's own clients (existing relationship; far lower legal risk) |

**Interpretation.** The **durable "draft → approve → send → stop-on-reply → track" core is shared**: roughly the outbox, sequencing, reply handling and safety rails. That is about half of what either product needs (judgement). The halves that differ are the risky ones: discovery and deliverability for outbound, and deadlines/2307s/Viber for P1. So the sensible shape is **one Rust core, P1 first, and our own outreach as the second user of that core**. Building outbound-specific discovery, scraping and warmup is not justified by 20–40 emails a week. That volume can be hand-researched, and the existing approval queue can send it.

**Cheaper alternatives already exist for self-use.** Harvey itself is MIT and free. Gmail by hand covers 20–40 emails a week. Both cost nothing to build. **A separate Rust outbound engine only makes sense as dogfood of P1's core, not as a project in its own right.**

## Candidate scores

Scale and weights follow `scoring-method.md` (5 = favourable; J capped at 4). Balanced totals were computed with the `score.py` weights in a scratch copy; **`score.py` was not edited.**

### P-R: Rust Harvey competitor for solo founders and agencies

**Best sub-variant found:** open-core. An MIT Rust engine (self-hosted, approval-first, BYO API key) plus a paid hosted tier with our API key, a richer review desk, a deliverability readiness check and compliance footers. Paid-only is worse: it would compete with free Harvey and Apollo's free plan with no community.

`3E 4E 3J 5E 4E 1E 3J 3J 2E 3J 2E 3J 2E 4E 4J 4E` → **Balanced 62.8** (#30 of 54 when added to the existing list), Revenue-first 64.2, Builder-first 63.0. 10/16 evidence. **Disqualifier: C6 = 1.**

| # | Score | Justification |
|---|---|---|
| C1 | 3E | Pipeline matters (people pay $37 to $4,000/mo for outbound [C1–C13]), but cold email is one channel among many for a solo founder. Not lost revenue in itself. |
| C2 | 4E | Daily send/reply loop (Harvey's 15-min heartbeat; per-month volume pricing [C1][C3]). |
| C3 | 3J | One booked meeting can be worth ≫ $40, but cold reply rates are low and outcomes uncertain. No data found. |
| C4 | 5E | Many people pay for partial solutions: Instantly, Smartlead, lemlist, Woodpecker, AI SDRs [C1–C13]. Harvey has 1,776 stars and 354 forks [H1]. |
| C5 | 4E | Published price points $32–$555/mo in the segment [C1][C3][C4]. But the free tiers (Apollo, Harvey) set the floor for solo founders. |
| C6 | **1E** | Free, MIT, fast-moving incumbent in exactly this niche (Harvey); free Apollo plan; Instantly already ships human-in-the-loop AI replies [C2]; 6+ new OSS "AI SDR" repos in 2026 [C14]. No evidenced gap (see Gaps). |
| C7 | 3J | GitHub, HN and Reddit launches are free and written, but Harvey already holds that attention. Using cold email to sell a cold-email tool is plausible. No ads. |
| C8 | 3J | A self-hosted CLI MVP fits in 8 weeks. Hosted multi-tenant with deliverability does not, realistically. |
| C9 | 2E | Hosted Gmail needs restricted scopes and annual CASA (~$540+/yr); Graph needs publisher verification and admin consent (integration-constraints.md). |
| C10 | 3J | Approval-first helps, but a sent cold email cannot be recalled, and complaint rates above 0.3% damage the customer's domain [D1][D2]. |
| C11 | 2E | Third-party personal data processed at scale; CAN-SPAM up to $53,088 per email [L6]; consent-only B2B countries in the EU [L8]; Workspace AUP forbids facilitating "unsolicited mass email" [D6]; Anthropic AUP bars facilitating spam [L11]. Not 1 only because the data is low-sensitivity business contact data and US/PH/UK-corporate rules are opt-out. **Borderline 1 for an autopilot or EU-facing product.** |
| C12 | 3J | A self-host demo in about 4–6 weeks. The hosted tier waits on Google/Microsoft verification (weeks) [integration-constraints.md]. |
| C13 | 2E | A hosted product must pay for API-key LLM calls (subscriptions cannot be resold [L9]), and deliverability support (DNS, warmup, blocklists) is the top user ask [H7]. |
| C14 | 4E | Monthly subscriptions throughout the category [C1–C8]. |
| C15 | 4J | The same core serves follow-up, chasing (P1, B) and reply drafting. |
| C16 | 4E | Exercises approvals, a durable outbox, idempotent sends, a kill switch and untrusted-input handling (teardown). |

### P-SELF: our own outbound engine, used for distribution (enabler, **not a revenue product**)

**The method does not fit.** C3, C4, C5 and C14 measure a buyer paying us, and C6 measures room in a market. For an internal tool, there is no buyer and no market. A forced string is given below **for reference only. It must not be ranked against revenue candidates**, and its total is not comparable.

Criteria are reinterpreted: C1 = severity of *our* distribution problem; C4 = evidence the channel works for this segment; C5 = neutral (N/A); C6 = whether free alternatives make building unnecessary; C14 = N/A (no revenue).

`4E 3J 3J 2J 3J 1E 3E 4J 4E 4J 3E 4J 4J 2J 4J 4E` → Balanced 62.4 (**not comparable**), Revenue-first 59.6, Builder-first 66.6.

| # | Score | Justification |
|---|---|---|
| C1 | 4E | No network, no interviews, no ads (owner constraints, AGENTS.md). Distribution is the binding constraint. |
| C2 | 3J | Weekly batches of 20–40 emails. |
| C3 | 3J | One paying bookkeeper is ₱1,500/mo. Worth it only if build cost is near zero (i.e., it reuses P1's core). |
| C4 | 2J | Not found: any evidence that PH bookkeepers answer cold email. They live in FB groups and Viber (ph-industries-services.md). |
| C5 | 3J | N/A, neutral placeholder. |
| C6 | 1E | Free substitutes make building unnecessary: Harvey (MIT, free) and hand-sent Gmail at this volume [H1]. |
| C7 | 3E | PRC lists 283 accredited firms by name, and BIR had 5,358 accredited agents in 2022, but no list carries emails, so each one is found by hand [F1][F2][F5]. |
| C8 | 4J | Smaller than Harvey (no scraping, LinkedIn or warmup) if built on P1's outbox. |
| C9 | 4E | Single user: the CASA "personal use / few users" exception applies; Gmail API or SMTP (integration-constraints.md ref 36). |
| C10 | 4J | Every email is approved by hand, at low volume. |
| C11 | 3E | No PH anti-spam statute [L1]. NPC allows legitimate interest for direct marketing after an assessment [L3][L5]. Volumes this low stay clear of "mass" [D6]. A documented LIA and an opt-out line are still needed. |
| C12 | 4J | 2–3 weeks on top of P1's outbox. |
| C13 | 4J | Tens of LLM drafts a week on an API key cost cents. |
| C14 | 2J | N/A. Not a revenue product. |
| C15 | 4J | It is P1's core with a different recipient list. |
| C16 | 4E | Same core as P-R/P1 (teardown). |

**Recommendation (not an owner decision):** do not pursue P-R. Treat P-SELF as a *feature of P1's core* (dogfooding: "our own outreach is customer zero"), capped at about 20–40 hand-checked emails a week to firms with published business emails. Any separate outbound-engine work should go through grilling as its own decision, after P1's core exists.

## Unresolved questions

1. **Do PH bookkeepers reply to cold email?** Not found. The only way to find out is a small test (e.g., 50 hand-written emails), which is an owner decision about the validation method.
2. **Exact count and fields of the PRC individual-CPA list:** text extraction failed (76 pages). Does it carry addresses or emails?
3. **Current BIR RNAB master list:** only a 2022 figure (5,358) via a newspaper column. The BIR page did not render.
4. **NPC primary texts:** Circular 2023-04 §14 and AO 2023-016 returned 403. Their wording here comes from search excerpts and a secondary guide (M). There is no NPC ruling on B2B cold email specifically.
5. **Status of the PH House anti-spam bills** (HB 797/485/1342/1687, December 2025 committee stage) after 2025: not found.
6. **Whether a 24/7 `claude -p` daemon on a personal subscription is "ordinary, individual usage":** Anthropic's text does not settle it. Avoid it by using an API key.
7. **Harvey real-world usage** (active installs, send volumes, outcomes): not found. Star growth may be a social-media spike.
8. **Google Maps Platform Service Specific Terms** full text (scraping and database clauses): did not render. Only the Places policy summary was verified.
9. **Microsoft Outlook.com** current enforcement (junk vs reject): only a 2025 third-party report of Microsoft's post. The Microsoft page did not render.

## Sources

All accessed 2026-10-20. Types: P primary/official, VC vendor claim, TP third-party, F forum/user report.

**Harvey**
- [H1] GitHub API, `repos/ethanplusai/harvey` (stars 1,776, forks 354, watchers 20, pushed 2026-10-20). P, H.
- [H2] GitHub API, commits since 2026-10-15 on `main` (22 commits, e114e95…66bcf57). P, H.
- [H3] GitHub API, `repos/ethanplusai/harvey/forks?sort=stargazers` (top forks ≤ 1 star; one fork links an Instagram reel). P, H.
- [H4] ethanplus.ai homepage (consulting plus products; Harvey listed under "Open source"; no hosted Harvey). Accessed 2026-10-20. P, M.
- [H5] Precleared homepage, https://precleared.dev (pricing: free 10/25 REST checks a day; £39 per 2,000; £9 starter). VC, M. Code: `harvey/integrations/precleared.py` at commit 44b8cf4. P, H.
- [H6] Harvey PR #13 (Tellwing disclosure, compliance footer), https://github.com/ethanplusai/harvey/pull/13. P, H.
- [H7] GitHub API, `repos/ethanplusai/harvey/issues?state=all` (#9, #10, #13, #14, #18, #19, #20, #24, #27, #28, #34, #35). P, H (as a demand signal, F).

**Competitors**
- [C1] Instantly, https://instantly.ai/pricing. VC, M (the page contradicts itself on some tiers).
- [C2] Instantly help centre, "AI Reply Agent", https://help.instantly.ai/articles/11774076-ai-reply-agent, and Instantly Academy lesson 6 (HITL mode; 5 credits per draft). VC, M (via search summary).
- [C3] Smartlead, https://www.smartlead.ai/pricing. VC, M.
- [C4] lemlist, https://www.lemlist.com/pricing. VC, M.
- [C5] Apollo, https://www.apollo.io/pricing (plans named; no prices rendered). VC, M.
- [C6] Apollo prices via roundups: landbase.com/blog/apollo-pricing (2026-03-31), docket.io, zeliq.com. TP, M.
- [C7] Reply.io prices via joinvalley.co/blog/reply-io-pricing-explained-2026, lemlist.com/fr/blog/reply-io-review, sendkit.ai/replyio-pricing (reply.io returned 403). TP, L–M.
- [C8] Woodpecker, https://woodpecker.co/pricing/. VC, M.
- [C9] GMass, https://www.gmass.co/pricing (PH peso prices). VC, M.
- [C10] 11x pricing via marketbetter.ai/blog/11x-ai-pricing-2026, landbase.com/blog/11x-ai-pricing (2026-04-07), checkthat.ai. TP, L.
- [C11] Artisan pricing via 11x.ai/guides/artisan-pricing (competitor), marketbetter.ai, syncgtm.com/blog/artisan-review, artisan.co blog. TP/VC, L.
- [C12] AiSDR, https://aisdr.com/pricing/. VC, M.
- [C13] Regie.ai via costbench.com/software/ai-sales-tools/regie-ai (incl. 2026-10-07 changelog), marketbetter.ai, toolradar.com. TP, L.
- [C14] GitHub search (`gh search repos`: "cold email agent", "AI SDR", "open source cold email", "cold email" language:rust). P, H for the star counts on the access date.

**Deliverability and platform**
- [D1] Google, "Email sender guidelines", https://support.google.com/a/answer/81126. P, H.
- [D2] Yahoo Sender Hub, "Best practices", https://senders.yahooinc.com/best-practices/. P, H.
- [D3] Petri, "Microsoft Outlook to Enforce Stricter Email Authentication Rules for High-Volume Senders", 2025-04-07, https://petri.com/microsoft-outlook-email-authentication-rules/ (reporting Microsoft's techcommunity post 4399730). TP, M.
- [D4] Google, Gmail Program Policies, https://support.google.com/mail/answer/16734397. P, H.
- [D5] Google, Gmail sending limits, https://support.google.com/mail/answer/22839. P, H.
- [D6] Google Workspace / Google Cloud Acceptable Use Policy, https://workspace.google.com/terms/use_policy/. P, H.
- [D7] Google Workspace, "Gmail sending limits in Google Workspace", https://knowledge.workspace.google.com/admin/gmail/gmail-sending-limits-in-google-workspace. P, H.
- Gmail API / Graph verification and CASA: `docs/research/integration-constraints.md` §3 and its refs 35–45 (accessed 2026-10-18).

**Legal**
- [L1] Columbia Global Freedom of Expression, *Disini v. Secretary of Justice*, G.R. No. 203335 (Feb 2014), https://globalfreedomofexpression.columbia.edu/?p=8773. TP, H.
- [L2] PH anti-spam bills: Manila Bulletin item 2025-12-01 (House committee on HB 797/485/1342/1687), Inquirer (SB 2460, Villanueva), legacy.senate.gov.ph records (all via search). TP, M.
- [L3] NPC Circular No. 2023-04, Guidelines on Consent, §14, https://privacy.gov.ph/wp-content/uploads/2023/11/NPC-Circular-No.-2023-04_Guidelines-on-Consent_07Nov2023.pdf (403 on fetch; quoted via search excerpt). P, M.
- [L4] Ascentium, "A Guide to NPC Circular No. 2023-04", https://www.ascentium.com/philippines/guides/guidelines-on-consent-for-data-privacy. TP, M.
- [L5] NPC Advisory Opinion No. 2023-016, https://privacy.gov.ph/wp-content/uploads/2023/11/Advisory-Opinion-No.-2023-016.pdf (403 on fetch; quoted via search excerpt). P, M.
- [L6] FTC, "CAN-SPAM Act: A Compliance Guide for Business", https://www.ftc.gov/business-guidance/resources/can-spam-act-compliance-guide-business. P, H.
- [L7] ICO, "Electronic mail marketing" (PECR guide), https://ico.org.uk/for-organisations/direct-marketing-and-privacy-and-electronic-communications/guide-to-pecr/electronic-and-telephone-marketing/electronic-mail-marketing/. P, H.
- [L8] EU B2B email: activemind.legal guide on legitimate interest and direct marketing; EDPB consultation contribution; DLA Piper 2025-01-17 note (via search). TP, M.
- [L9] Anthropic, Claude Code docs, "Legal and compliance", https://code.claude.com/docs/en/legal-and-compliance. P, H.
- [L10] Anthropic, Consumer Terms of Service (effective 2025-10-08), https://www.anthropic.com/legal/consumer-terms. P, H.
- [L11] Anthropic, Usage Policy (page shows "Effective date: November 12, 2026"), https://www.anthropic.com/legal/aup. P, H.

**Framing (b)**
- [F1] PRC, ACD "Lists of Accreditation/Registration/Compliance/Authority", https://prc.gov.ph/acd-lists-accreditation-registrationcomplianceauthority; PDF "List of Accredited Firms and Partnerships as of September 30, 2026" (283 entries, names and validity only) and "List of Accredited Individual CPAs … as of September 30, 2026" (76 pages). P, H.
- [F2] Manila Times, M. Gomez-Cheng, "Revisiting BIR Revenue Regulation 11-2006", Sept 2022, https://digitaledition.manilatimes.net/article/281822877658259 (5,358 RNAB-accredited, Feb 2022). TP, M.
- [F3] BIR RR 4-2010 (master list to be posted on the BIR website), https://dev.lawyerly.ph/laws/50808. P (via a legal database), M.
- [F4] PICPA public member directory: searched; not found.
- [F5] Google Maps Platform pricing, https://developers.google.com/maps/billing-and-pricing/pricing. P, H.
- [F6] Google, Places API policies, https://developers.google.com/maps/documentation/places/web-service/policies. P, H.
