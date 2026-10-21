# Domain and email at the lowest cost

Status: research note, 2026-10-21; nothing here is an owner-approved decision.

Builds on `hosting-and-email-infra.md`, `inbound-email.md` and `p1-launch-prerequisites.md` §4. Only new or changed facts are given here.

**How this was checked.** The build sandbox could not reach most vendor sites (porkbun.com, resend.com, spaceship.com, developers.cloudflare.com, any RDAP/whois service). Cloudflare's docs were read from their published source on GitHub (`cloudflare/cloudflare-docs`, branch `production`, fetched 2026-10-21) [Z1]–[Z5]. Everything else comes from web-search excerpts of the named page, marked **SR** (seen in search result, not fetched). All sources accessed 2026-10-21. Evidence types: **VP** vendor pricing page, **VD** vendor docs, **TP** third party, **F** forum. What a vendor says about itself is a vendor claim.

**Exchange rate.** ₱58/US$ is out of date. Latest BSP reference rate found: ₱61.35 (1 Jul 2026) [Z30]; dotPH's own site converts at ₱63.00–63.50 [Z24]; ANZ forecasts ₱63 by end-2026 [Z30]. **This note uses ₱62/US$.** An October 2026 rate: not found.

## Recommendation (inference, not a decision)

| | Choice | Cost |
|---|---|---|
| Registrar | **Cloudflare Registrar**, `getkompleto.com`. At cost, same price every renewal, no upsell. Forces Cloudflare DNS, which inbound routing needs anyway. Pays by PayPal | **US$10.46 ≈ ₱650** if bought by 31 Oct 2026; **US$11.17 ≈ ₱690** from 1 Nov. Renewal ≈ ₱690 |
| Outbound sender | **Resend Free**: 3,000/month, 100/day, 3 domains, no card, 1 webhook endpoint | ₱0 |
| Inbound route | **Cloudflare Email Routing → Email Worker → POST to app** (Workers Free) | ₱0 |
| **First-year total** | Domain only | **≈ ₱650–690**, or up to ≈ ₱780 if 12% PH VAT is added (not confirmed) |

**How to pay without an international credit card (inference, M).** Cloudflare accepts PayPal [Z5]. A PH PayPal account can link a Maya virtual Visa card [Z27], which is created in the Maya app and loaded from GCash or cash. No registrar in this note takes GCash directly (not found). Test the Maya card on PayPal with a small amount first; a 2026 blog reports GCash's virtual card being declined by foreign SaaS merchants [Z26] (TP, L).

**Single biggest risk: outbound capacity on BIR deadline days, with no card-free way out.** Resend Free stops at 100 emails a day. Every way past that costs money and needs a card: Resend Pro US$20, Cloudflare Workers Paid US$5, or Amazon SES, whose free tier has **closed to new accounts** (below). Brevo Free (300/day, no card, vendor claim) is the only ₱0 overflow found.

Smaller point: buying before 1 Nov 2026 saves US$0.71 (≈ ₱44) a year and locks the old price for the term bought [Z8]. That is 10 days away and earlier than "just before pilots"; it is the owner's call.

## 1. Where to register the .com

**Fact: wholesale price.** Verisign's .com fee rises from US$10.26 to **US$10.97 on 1 Nov 2026**; with the US$0.20 ICANN fee a registrar's floor is **US$11.17** [Z8] (TP, multiple outlets, H). Verisign may raise it again later, up to about US$13.42 by the end of the contract cycle [Z8] (TP, M).

| Registrar | First year | Renewal | Payment methods | Source |
|---|---|---|---|---|
| **Cloudflare** | At cost: US$10.46 now, US$11.17 from 1 Nov | Same as first year; "only renew at the list price set by the registry" | Visa, Mastercard, Amex, Discover, **PayPal**, Apple Pay, Google Pay, Stripe Link, UnionPay (account-wide billing) | [Z1] VD H; [Z5] VD H; price [Z8] H |
| Porkbun | US$11.08 (US$10.08 with promo, max 3) | US$11.08 | Cards, **PayPal** (not for auto-renew), crypto via Coinbase, Alipay, Apple/Google/Amazon Pay | [Z9] TP SR M; [Z10] VD SR H |
| Spaceship | US$2.90 promo reported | US$10.18 | Cards, **PayPal**, Bitcoin, Alipay, Apple/Google Pay | [Z9] TP SR L–M; [Z11] VD SR H |
| Dynadot | US$10.88 (US$10.12 promo until 1 Nov) | ≈ US$10.88 | Cards, **PayPal**, Alipay, Payoneer, Wise, crypto | [Z12] TP SR M; [Z13] VD SR H |
| Namecheap | **US$15.98 from 1 Nov** (+ICANN fee) | **US$19.98 from 1 Nov** | Cards, PayPal, crypto (aggregator) | [Z14] VD SR H; [Z9] TP L |

- **Inference (M):** every renewal under US$11.17 above is a pre-November price. Porkbun, Spaceship and Dynadot will rise by at least the US$0.71 on 1 Nov. Spaceship's promo saves about ₱500 in year one only. Cloudflare is the cheapest **over time** and has no upsell or promo games.
- **Cloudflare caveats [Z1] VD H:** nameservers must be Cloudflare's; no refunds; auto-renew charges the primary payment method about 30 days before expiry.
- **Is `getkompleto.com` free?** **Not found.** No RDAP, whois or registrar search could be reached from the sandbox, and a web search for the exact name returned no page using it [Z31]. A missing web page does not mean the name is unregistered. Check at dash.cloudflare.com before buying.

### .ph, .com.ph and cheap TLDs

| TLD | First year | Renewal | Note | Source |
|---|---|---|---|---|
| .ph (dotPH direct) | US$48 ≈ ₱2,980 | US$48 | dotPH prices in US$ | [Z24] VP SR M |
| .ph (Porkbun) | US$41.36 (promo) | US$44.99 ≈ ₱2,790 | | [Z25] TP SR M |
| .com.ph | not found | not found | dotPH pages list only resale asks | [Z24] |
| .xyz | ≈ US$1–2 promo | ≈ US$13.97 (Spaceship) – US$22.99 | High abuse rates | [Z15] TP SR L–M |
| .online / .site | ≈ US$1.96 | **US$28.84** (Porkbun) | Renewal ≈ 2.6× a .com | [Z16] VD/TP SR M |
| .app | US$8.75 (Porkbun) | US$14.93 | HTTPS-only (HSTS preloaded), run by Google Registry | [Z16] VD SR M; [Z17] TP SR M |

**Deliverability of cheap TLDs.**
- Fact: in Mar–May 2026, .top had the 2nd-highest count of reported spam domains and .xyz the 10th; per domain registered, .top scored far worse [Z18] (TP, CyberCrime Info Center, M). Spamhaus in 2018 rated .top, .xyz and .online above .ru for botnet abuse [Z19] (TP, dated, L–M).
- Third-party view: the TLD is a "weak signal" next to authentication, domain age and complaints, but .xyz, .site, .top are "high-scrutiny" [Z20] (TP, deliverability vendor blog, L–M). No Gmail or Microsoft statement on TLDs was found.
- **Inference (M):** a cheap TLD saves ≈ ₱600 in year one, then costs more than a .com from year two (.online/.site), and starts every Firm's mail from a less trusted domain. Not worth it. .app is the only reasonable alternative, and it is still dearer than .com.

## 2. Authenticated email at ₱0 without buying a domain

**Technically possible.** SPF, DKIM and DMARC are just DNS records. Any name where we control those records can pass all three.

| Option | What we get | Facts | Source |
|---|---|---|---|
| **eu.org** subdomain (e.g. `kompleto.eu.org`) | Full NS delegation, so any DNS host | On the Public Suffix List, so DMARC treats our name as its own organisational domain (H). Approval reported to take days to over a year; one user says new requests are no longer processed | PSL [Z22] H; [Z21] F L |
| **is-a.dev** subdomain | DNS records via a GitHub pull request; MX supported; NS only in approved cases after 30 days | Service is "for their personal websites". **Not** on the Public Suffix List, so reputation is shared with every other is-a.dev user (inference) | [Z23] VD H; PSL [Z22] H; NS rule TP SR L |
| **Send from each Firm's own Gmail** (Gmail API `gmail.send`) | Mail authenticated by Google as the Firm itself | `gmail.send` is a *sensitive* scope: needs Google app verification, no paid security assessment; unverified apps cap at 100 users. Gmail's daily send limit: not checked | [Z29] VD SR H |

**Is it sensible for a business product? No (inference, M–H).**
- Volunteer services can revoke or lose a name with no notice, and both eu.org's and is-a.dev's terms point at non-commercial use.
- A bookkeeper's client sees `@kompleto.eu.org` or `@x.is-a.dev`. That looks like phishing on a message asking for tax documents.
- Shared reputation (is-a.dev) means another user's spam can land our mail in junk.
- eu.org approval may never come, so it cannot be planned around.
- The Gmail route is a different product design (P1 spec decides channels), so it is only a proposal for later.

The domain is about ₱650 a year. That is the cheapest reliable way to meet E2.

## 3. Resend Free today

| Item | Value | Source |
|---|---|---|
| Monthly / daily | **3,000 / month, 100 / day**; sending pauses at the cap, no overage. Every To/CC/BCC recipient counts | [Z6] VD SR H |
| Domains | **3** (raised from 1; one aggregator still says 1) | [Z6] VD SR H (changelog) |
| Card | **Not required** ("no credit card required", vendor claim) | [Z6] VD SR H |
| Data retention | 30 days (one aggregator says 1 day) | [Z6] VD SR M |
| Inbound | Included on Free. **Each received email counts as 1 against both the daily and monthly limits.** This settles open question 6 in `hosting-and-email-infra.md` | [Z7] VD SR H |
| Webhooks | Included; **1 endpoint** on Free, 30-day event retention | [Z6] VP SR H |

**Inference (H):** keep inbound off Resend. A reply received there would use one of the 100 daily sends.

## 4. Other free outbound senders

| Provider | Free allowance | Card? | Bounce webhooks on free | Source |
|---|---|---|---|---|
| **Brevo** | 300/day (≈ 9,000/month); one Brevo blog says 200/day | No (vendor claim) | Yes, webhooks incl. bounces (vendor claim) | [Z32] VP/VD SR M |
| **Amazon SES** | **3,000/month free tier closed to new customers since 21 Jul 2026.** New accounts start on the "Essentials" plan at **US$0.16/1,000**; can switch to à la carte (≈ US$0.10/1,000). US$200 general AWS credits for new accounts. Sandbox until production access | Yes, AWS sign-up needs a card; prepaid cards reportedly refused | Yes (SNS/event publishing) | [Z33] VP/VD SR H; card [Z34] F L–M |
| Mailjet | 6,000/month, 200/day; Mailjet logo on mail | not found | not found | [Z35] TP SR M |
| SMTP2GO | 1,000/month, 200/day (25/hour until a domain is verified); no expiry | No (vendor claim) | not found | [Z36] VD SR H |
| MailerSend | 500/month, 100/day after approval | Reported yes (TP; not confirmed by vendor) | not found | [Z37] VD/TP SR M |
| Postmark | 100/month, no overage | No | Unclear on free | [Z38] VD/TP SR M |
| Cloudflare Email Sending (beta) | **Not available on Workers Free**; Workers Paid US$5/month includes 3,000, then US$0.35/1,000. Hard bounces count. Free only to our own verified addresses | Yes (paid plan) | Bounces suppressed automatically; notification to sender | [Z2], [Z3] VD H |

**Notes.**
- **Brevo and E2 (third party, M):** Brevo's free plan uses its own bounce domain, so SPF passes for Brevo's domain but does not *align* with ours; DMARC passes through DKIM alignment only [Z32]. If E2 is read as "SPF must align", Brevo Free fails it. Resend uses a `send.` subdomain of ours for the bounce address (from `hosting-and-email-infra.md`), so both align.
- **SES change (H):** `p1-launch-prerequisites.md` §4.1 assumed the old free tier. That no longer applies to a new account, and SES now needs a working card before anything else.
- **Inference (M):** Brevo Free is the only ₱0, no-card overflow for deadline days. Adding it means a second provider adapter in `engine::outbox` and a second webhook. Raise it as a proposal only if pilot volume nears 100/day.

## 5. Cloudflare Email Routing + Email Workers for inbound

| Question | Answer | Source |
|---|---|---|
| Needs Cloudflare DNS? | **Yes.** "You must be using Cloudflare DNS to use Email Service." Registering at Cloudflare makes this automatic | [Z4] VD H |
| Price | Inbound routing **unlimited** on Workers Free; the Worker is billed as a normal Worker | [Z2] VD H |
| Workers Free limits | 100,000 requests/day (error 1027 when exceeded), **10 ms CPU** per invocation, 50 subrequests | [Z3a] VD H |
| Routing limits | 200 rules per domain, 200 verified destinations, **25 MiB** inbound size; heavy Free handlers can fail with `EXCEEDED_CPU` | [Z3] VD H |
| Card needed? | Not stated by Cloudflare. A third-party guide says the free plan needs no card; Cloudflare's own student and startup offers ask for one, but those are paid offers | [Z28] TP SR L–M |

**Inference (M):** inbound costs ₱0 and needs no card. A domain bought at Porkbun or Spaceship would also work, after pointing its nameservers at Cloudflare's free plan.

## Open questions

1. Is `getkompleto.com` available? Not found from the sandbox; check before buying.
2. Does a Maya virtual card on PayPal actually pass Cloudflare checkout? Not found; a one-time test settles it.
3. Does Cloudflare add 12% PH VAT on a Registrar charge? Not found.
4. Gmail's daily send limit and Google verification time for `gmail.send`: not checked (only matters if the Gmail route is proposed).

## Sources

All accessed 2026-10-21. "n.d." = no date shown.

- [Z1] Cloudflare Docs (GitHub source), "Registrar FAQ", "Register a domain", "Renew domains", `src/content/docs/registrar/…`, https://github.com/cloudflare/cloudflare-docs, n.d., VD (fetched), H
- [Z2] Cloudflare Docs (GitHub source), "Email Service pricing", `email-service/platform/pricing.mdx`; overview `email-service/index.mdx`, n.d., VD (fetched), H
- [Z3] Cloudflare Docs (GitHub source), "Email Service limits", `email-service/platform/limits.mdx`; "Deliverability", `email-service/concepts/deliverability.mdx`; "Domains", `email-service/configuration/domains.mdx`, n.d., VD (fetched), H
- [Z3a] Cloudflare Docs (GitHub source), "Workers limits", `workers/platform/limits.mdx`, n.d., VD (fetched), H
- [Z4] Cloudflare Docs (GitHub source), "Route emails", `email-service/get-started/route-emails.mdx`, n.d., VD (fetched), H
- [Z5] Cloudflare Docs (GitHub source), "Billing", `billing/index.mdx`, n.d., VD (fetched), H
- [Z6] Resend, Pricing https://resend.com/pricing; "More domains on the free tier" https://resend.com/changelog/three-domains-on-the-free-tier; "Sending limits" https://resend.com/docs/knowledge-base/resend-sending-limits; GDPR https://resend.com/security/gdpr; https://resend.com/agents, n.d., VP/VD, SR, H (domain-count conflict: toolradar.com, TP, L)
- [Z7] Resend, "Account quotas and limits", https://resend.com/docs/knowledge-base/account-quotas-and-limits, n.d., VD, SR, H
- [Z8] Domain Name Wire, "Verisign raising wholesale .com prices", https://domainnamewire.com/2026/04/23/breaking-verisign-raising-wholesale-com-prices/, 2026-04-23; OSIR https://osir.com/en/blog/verisign-com-price-update-2026/; TLDes https://tldes.com/pages/com-price-increase-2026; Openprovider https://www.openprovider.com/blog/com-price-increase-domain-resellers, TP, SR, H (price), M (lock-in, cap path)
- [Z9] domainoffer.net, Porkbun .com (May 2026) https://domainoffer.net/tld/com/porkbun; Spaceship pages https://domainoffer.net/tld/space/spaceship; Namecheap https://tldes.com/registrars/namecheap, TP aggregator, SR, L–M
- [Z10] Porkbun, "Payment options", https://porkbun.com/support/payment_options; KB "How to use cryptocurrency", https://kb.porkbun.com/article/143-how-to-use-crypto-currency, n.d., VD, SR, H
- [Z11] Spaceship, homepage https://www.spaceship.com/ and "Domain registration agreement" https://www.spaceship.com/legal/domain-registration-agreement/, n.d., VD, SR, H
- [Z12] TLDes, "Cheapest .COM prices", https://tldes.com/com, n.d. (promo valid to 2026-11-01), TP aggregator, SR, M
- [Z13] Dynadot, "Payment methods", https://www.dynadot.com/community/help/question/payment-methods, updated 2026-07-21, VD, SR, H
- [Z14] Namecheap KB, ".com price change – November 1, 2026", https://www.namecheap.com/support/knowledgebase/article.aspx/10840/2201/com-price-change-november-1-2026/, 2026, VD, SR, H
- [Z15] domainoffer.net .xyz pages (e.g. https://domainoffer.net/tld/xyz/domaincostclub), Jul–Sep 2026, TP, SR, L–M
- [Z16] Porkbun, "Domain cost calculator" https://porkbun.com/about/domain-cost-calculator/app and /site; domainoffer.net .online (2026-08-06) https://domainoffer.net/tld/online/porkbun, VD/TP, SR, M
- [Z17] Webnames, "7 things you need to know about .APP", https://blog.webnames.ca/7-things-you-need-to-know-about-app-by-google-registry/; Dynadot https://www.dynadot.com/domain/app, TP/VP, SR, M
- [Z18] CyberCrime Information Center, "Spam activity in TLDs, March–May 2026", https://www.cybercrimeinfocenter.org/spam-activity-in-tlds-march-may-2026, 2026, TP, SR, M
- [Z19] DomainIncite, "Spamhaus ranks most-botted TLDs and registrars", https://domainincite.com/22472-spamhaus-ranks-most-botted-tlds-and-registrars, 2018, TP, SR, L–M
- [Z20] Suped, "Does the TLD affect email deliverability", https://www.suped.com/learn/email-deliverability/does-the-top-level-domain-tld-affect-email-deliverability, n.d., TP (vendor blog), SR, L–M
- [Z21] InfinityFree forum, "Time taken to get EU.ORG domain approval", https://forum.infinityfree.com/t/time-taken-to-get-eu-org-domain-approval/92362, n.d., F, SR, L
- [Z22] Public Suffix List, https://raw.githubusercontent.com/publicsuffix/list/main/public_suffix_list.dat (eu.org listed; is-a.dev not listed), fetched, H
- [Z23] is-a.dev, README, https://github.com/is-a-dev/register (fetched); ImprovMX guide https://docs.is-a.dev/guides/improvmx/ (SR); DEV Community guide (NS rule), TP, SR, L
- [Z24] dotPH, "Products and services", https://www.dot.ph/products; homepage https://www.dot.ph/, n.d., VP, SR, M
- [Z25] domainoffer.net, .PH prices (updated 2026-09-07), https://domainoffer.net/tld/ph, TP, SR, M
- [Z26] Spendfigo, "GCash virtual card keeps getting declined", https://spendfigo.com/blog/gcash-virtual-card-keeps-getting-declined, 2026-03, TP (competitor), SR, L
- [Z27] Maya, "How to use PayMaya Visa to pay online", https://www.maya.ph/stories/how-to-use-paymaya-visa-to-pay-online (VD, SR, M); Wise, "PayPal to PayMaya", https://wise.com/ph/blog/paypal-to-paymaya (TP, SR, M)
- [Z28] Cloudflare for Students https://www.cloudflare.com/students/ and for Startups https://www.cloudflare.com/hatch (VD, SR, H for those offers); blog.reviewaitool.com, 2026-04-17 (TP, SR, L)
- [Z29] Google for Developers, "Choose Gmail API scopes", https://developers.google.com/workspace/gmail/api/auth/scopes; Google Cloud Help "Requesting minimum scopes", https://support.google.com/cloud/answer/13807380, n.d., VD, SR, H
- [Z30] PesoHub, "USD to PHP today" (BSP reference ₱61.35, 2026-07-01), https://www.pesohub.ph/rates/exchange-rates/usd-to-php-today/; worldngayon.com "Peso weakening 2026" (ANZ forecast), TP, SR, M
- [Z31] Web search for "getkompleto.com", no matching page, 2026-10-21, search result, L
- [Z32] Brevo, "Email API" https://www.brevo.com/features/email-api/ and "Transactional email" https://www.brevo.com/products/transactional-email/ (VP/VD, SR, M); CaptainDNS "Brevo technical guide" https://www.captaindns.com/en/blog/brevo-transactional-email-technical-guide (TP, SR, M)
- [Z33] AWS, "Amazon SES pricing" https://aws.amazon.com/ses/pricing/; "Amazon SES introduces pricing plans" https://aws.amazon.com/about-aws/whats-new/2026/07/amazon-ses-pricing-plans/; SES DG "Pricing plans" https://docs.aws.amazon.com/ses/latest/dg/pricing-plans.html, 2026-07, VP/VD, SR, H
- [Z34] freeCodeCamp, "How to create an AWS free tier account" https://www.freecodecamp.org/news/how-to-create-an-aws-free-tier-account/; AWS "CVV support" announcement, 2023-12, TP/VD, SR, L–M
- [Z35] Sender.net, "Mailjet free plan", https://www.sender.net/reviews/mailjet/pricing/free-plan, n.d., TP, SR, M
- [Z36] SMTP2GO Support, https://support.smtp2go.com/hc/en-gb/articles/223087947; "Free plan" https://www.smtp2go.com/?p=644, n.d., VD, SR, H
- [Z37] MailerSend Help, "How your plan and billing works", https://mailersend.com/help/plan-and-billing; freetier.co listing, VD/TP, SR, M
- [Z38] Postmark, "Pricing & billing FAQ", https://postmarkapp.com/support/article/1285-pricing-billing-faq; sender.net review, VD/TP, SR, M
