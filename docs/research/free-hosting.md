# Free hosting for the pilots (₱0 budget)

Status: research note, 2026-10-21; nothing here is an owner-approved decision. It builds on `docs/research/hosting-and-email-infra.md` (the "US$8/month" plan: Vultr Singapore + Cloudflare R2) and does not repeat its tables. Tags like [H14] point to that note's sources; [F1]-style tags point to the Sources list below.

Labels: **PP** vendor pricing page, **VD** vendor docs, **TP** third party, **F** forum or personal blog. Confidence **H/M/L**. All sources accessed 2026-10-21. **Method caveat:** the page-fetch tool failed (DNS errors) for every site today, so every new source here was read through **search-result summaries** of the page, not the page itself. That lowers confidence by about one step against a fetched page. Vendor statements are vendor claims. **Fact**, **Inference** and **Recommendation** are marked where they could be confused.

## Recommendation

*Recommendation, not owner-approved.*

**The hinge is a payment card.** Every free cloud VM (Oracle, Google, AWS, Azure) needs a real credit card, or a debit card that works like one, at sign-up. Oracle, Google and Azure say in their own docs that **prepaid or virtual cards are not accepted** [F2], [F11], [F13]. *Inference (M):* the GCash Card and the Maya virtual card are prepaid or virtual wallet cards, so expect them to be rejected. Not tested.

- **Primary (if the owner has a bank-issued credit card): Oracle Cloud Always Free, home region Singapore, account upgraded to Pay As You Go (PAYG).** One Ampere A1 VM (up to 2 OCPU / 12 GB RAM) runs the Rust binary, Postgres and Caddy, as in the VPS plan. Encrypted files go on its block volume (200 GB free) or in R2. Set a budget alert at US$1.
- **Fallback (no card needed): the owner's Mac + Cloudflare Tunnel.** The app and Postgres run on the Mac; `cloudflared` publishes it at `app.getkompleto.com` with no open ports. Cloudflare DNS, Tunnel and Email Routing are free.
- **Total: US$0/month** for both. The only spend stays the domain (already budgeted). PAYG upgrade reportedly puts a temporary US$100 authorisation hold on the card [F6] (TP, L).
- **Single biggest risk:** **losing the server without warning.** For Oracle that means idle reclamation, a capacity shortage when restarting, or an account suspension with free-tier support only [H14], [F4], [F8]. For the Mac it means a power or internet outage at home [F28], [F29]. Either way the 5-minute reply rule (G1) and the hourly jobs stop until it is back. **Mitigation:** a nightly encrypted `pg_dump` kept off the machine, a tested restore script, and an uptime alert.

**Why not the others** *(inference, M)*: Google's free VM is US-only, has 1 GB egress a month, and its public IPv4 may cost about US$3.65/month. AWS and Azure are free only for 6 or 12 months. Render, Zeabur and Railway free tiers sleep or are too small. Koyeb closed its free tier to new users. Fly.io has none. Free Postgres hosts pause, cap compute or have no backups. Details below.

**Before the first pilot** *(recommendation)*: (1) the owner checks which cards they hold; (2) if a credit card exists, sign up for Oracle with home region Singapore and try to create an A1 VM in week 1, since the home region can't be changed later [F1]; (3) whichever host wins, move to the paid Vultr plan from `hosting-and-email-infra.md` when the first firm pays ₱1,500.

## 1. Oracle Cloud Always Free

| Item | Finding | Evidence |
|---|---|---|
| A1 Ampere allowance | **Fact:** 1,500 OCPU-hours and 9,000 GB-hours a month (= 2 OCPU / 12 GB always on). Halved from 4 OCPU / 24 GB. Reported effective date conflicts: 15 Jun 2026 (InfoQ) vs 18 Aug 2026 (Oracle email quoted by labmemo). Instances over the limit are shut down or terminated. | [H14] VD H (fetched earlier today); [F5] TP M |
| A1 for PAYG accounts | Reports say PAYG accounts may still get 4 OCPU / 24 GB free. Oracle has not confirmed this. | [F5] TP L |
| E2.1.Micro | **Fact (vendor):** up to 2 AMD VMs; third parties describe 1/8 OCPU, 1 GB RAM each. Created in one availability domain only. | [F1] VD H; [F7] TP M |
| Storage | **Fact (vendor):** 200 GB block volume in total; each boot volume is at least 47 GB. | [F1] VD H |
| Home region | **Fact (vendor):** Always Free resources exist only in the home region picked at sign-up. It can't be changed. | [F1], [F2] VD H |
| Singapore | **Not found:** no official statement on Always Free capacity in `ap-singapore-1`. A forum post says Singapore has **one availability domain**, so "try another AD" does not help. Community guides disagree on whether Singapore A1 capacity is scarce or easy. | [F7] F L |
| Capacity errors | "Out of host capacity" is common for A1 in busy regions. Oracle's advice: retry, change AD, or upgrade to PAYG. A resize behaves like a fresh launch and can fail too. | [H14] VD H; [F7] TP M |
| Idle reclamation (exact wording) | **Fact (vendor):** "Idle Always Free compute instances may be reclaimed by Oracle. Oracle will deem virtual machine and bare metal compute instances as idle if, during a 7-day period, the following are true: CPU utilization for the 95th percentile is less than 20%; Network utilization is less than 20%; Memory utilization is less than 20% (applies to A1 shapes only)." | [H14] VD H; [F1] VD H |
| Does PAYG avoid reclamation? | **Not in current official text.** A 2023 Oracle email, reproduced on a blog, says: "you can keep idle compute instances from being stopped by converting your account to Pay As You Go (PAYG)" and "you will not be charged as long as your usage … remains within the Always Free limits". | [F4] TP (quotes Oracle) M |
| Cost after PAYG | **Fact (vendor):** Oracle doesn't charge for Always Free resources after an upgrade; only use above the limits is billed. | [F3] VD H |
| Card at sign-up | **Fact (vendor):** "credit cards and debit cards that function like credit cards" are accepted; **debit cards with a PIN, virtual, single-use and prepaid cards are not**. Periodic authorisation holds are released in 3–5 days. Reported holds: US$1 at sign-up, US$100 at PAYG upgrade. | [F2] VD H; [F6] TP L |
| PH cards / GCash | **Not found:** no Philippines-specific rule. GCash and Maya cards are likely refused as prepaid/virtual (inference, M). | [F2], [F30] |
| Account risk | Anecdotes of instances or whole tenancies stopped without notice (2022–2026); free tier has no full technical support. | [F8] F L; [H15] F L |

**Inference (M–H):** Kompleto at pilot scale is "idle" by Oracle's test: p95 CPU, network and memory all well under 20%. Without PAYG, reclamation is likely. "Keep-busy" scripts exist but waste resources and break the spirit of the rule, so they are not recommended. With PAYG, the 2023 email suggests the instance is safe. That is a secondhand, dated source, so treat it as **M–L** until Oracle confirms it.

## 2. Big-cloud free tiers

| | Always-on VM | How long | Card | Fit |
|---|---|---|---|---|
| **Google Cloud** | 1 e2-micro (2 shared vCPU burst, 1 GB) + 30 GB standard disk, **only in us-west1, us-central1, us-east1**. 1 GB/month egress from North America. Hours pooled across those regions. [F9] VD H | Always free | Required; **prepaid cards not accepted**; debit cards needing 2FA also refused for payment [F11] VD H | US region adds ~150–250 ms latency from PH (inference). In-use external IPv4 is US$0.005/h (≈ US$3.65/mo) and third parties say the free tier doesn't cover it [F10] VD H / TP L. Not confirmed either way. The "balanced" default boot disk is not free; pick "standard" [F9] TP M. |
| **AWS** (accounts from 15 Jul 2025) | No free VM as such. US$100 credit at sign-up, plus up to US$100 for onboarding tasks. | **Free plan ends after 6 months or when credits run out.** Then upgrade to paid within 90 days or the account closes [F12] VD H | Required (TP reports; official PH rule not found) | Covers a small Singapore EC2/Lightsail box for ~6 months (inference). Not ₱0 after that. |
| **Azure** | 750 h/month each of B1s, B2pts v2, B2ats v2 for **12 months**; US$200 credit for 30 days. Must move to pay-as-you-go within 30 days to keep the 12-month services [F13] VD H | 12 months | Required: phone + credit or **non-prepaid** debit card; "virtual or prepaid credit cards aren't accepted"; US$1 hold [F13] VD H | Region list for free B1s and public-IP cost: **not found**. A 12-month bridge, not a permanent answer. |
| **Google Cloud Run** (serverless) | Scale-to-zero containers: 2M requests, 180,000 vCPU-s, 360,000 GiB-s a month free; Singapore listed [F31] VD M | Always free | Same as Google Cloud | Needs a different design: no always-on job poller; jobs triggered by a scheduler and webhooks. Cloud Scheduler free quota **not found**. |

## 3. App platforms that run a Docker image or Rust binary

| Platform | Free tier now | Always on? | Card | Evidence |
|---|---|---|---|---|
| **Render** | 512 MB web service, 750 instance-hours per workspace per month | **No: sleeps after 15 min idle**, ~1 min wake. Local disk wiped on sleep. Out of hours → all free services suspended until next month. Free Postgres expires after 30 days (+14-day grace) | Not confirmed | [F14] VD H |
| **Koyeb** | **Closed to new users** since Mistral bought Koyeb (Feb 2026); existing users keep it. Old free instance: Frankfurt/Washington only, scale-to-zero after 1 h | No | n/a | [F15] TP M |
| **Fly.io** | **None** for new organisations; trial 2 h of machine time or 7 days | No | Needed after trial | [H9] PP H; [F16] TP M |
| **Railway** | After a 30-day US$5 trial: Free plan with **US$1/month credit**, 0.5 GB RAM, no custom domain | No: US$1 can't pay for an always-on 0.5 GB service (RAM ≈ US$10/GB-month [H10]) | Conflicting reports | [F17] F/TP L–M |
| **Northflank** | Developer Sandbox: 2 services, 2 jobs, 1 add-on (database) | "Always-on compute" on the pricing page, but docs say **not for production** | **Required for everyone** ("to verify user identity") | [F18] PP/VD H; crash report F L |
| **Zeabur** | Free plan: 1 project, 2 services, no custom domain, no persistent storage | **No: sleeps when idle** | Not found | [F19] VD H |

**Inference (M):** none gives an always-on, production-allowed, no-card service. Render could be kept awake by an outside pinger (a 31-day month is 744 h, inside 750 h). That is a terms grey area, the box has 512 MB and no disk, and it still restarts on deploys. **Not recommended.**

## 4. Free managed Postgres

| | Size | Pausing / limits | Singapore | Evidence |
|---|---|---|---|---|
| **Supabase Free** | 500 MB [H18] | **Pauses after a week without enough database activity**; "a few user requests to the database each day" prevents it. Warning email ~1 week before. Restorable for 90 days. No automatic backups [H18]. | Yes [H19] | [F20] VD H |
| **Neon Free** | 0.5 GB per project (third party, Sep 2026; earlier fetch said 1 GB [H16]) | 100 CU-hours per project per month; **suspends after 5 min idle**, can't be disabled on Free | Yes [H17] | [F21] TP M; [H16] PP H |
| **Aiven Free** | 1 CPU, 1 GB RAM, 1 GB disk; backups included; 20 connections | **Powered off after inactivity** (notice first). Aiven "may change the cloud provider, region, or configuration at any time" | Not guaranteed | [F22] VD H |
| **Prisma Postgres Free** | 500 MB; 100k–200k operations/month (sources disagree) | Suspension when over quota; one report of suspension below it | Not found | [F23] VD/F M–L |
| **Xata** | Third parties say the hosted free tier was retired in Apr 2026 (now a US$100 credit); Xata's page still shows "Free" | Conflicting | Not found | [F24] VD/TP L |

**Inference (M):** a job poller every few seconds would burn Neon's 100 CU-h in days. Supabase stays awake under constant use but has no backups. All of them add a second vendor whose free policy can change. **On a VM (Oracle or Mac), run Postgres on the same machine instead.**

## 5. The owner's Mac + Cloudflare Tunnel

- **Facts:**
  - Cloudflare Tunnel is "available on all plans", including Free. `cloudflared` makes outbound-only connections, so no router ports open [F25] VD H, TP M.
  - Quick Tunnels (`trycloudflare.com`) are test-only with a 200-request limit. A **named** tunnel on a Cloudflare account doesn't have those limits [F25] VD H.
  - Free-plan request bodies are capped at **100 MB** (HTTP 413 above) [F26] VD H. Phone photos and PDFs fit.
  - A Cloudflare account and its DNS need no card. R2 needs a payment method on file (card or PayPal); Cloudflare's billing docs say a primary payment method is required to buy products [F27] VD H; [H22] TP L.
- **Power (fact, news):** Luzon's grid was on red alert several days in mid-May 2026, with rotating outages for over 200,000 Meralco customers. The Visayas grid had 20+ red alerts in August 2026 [F28] TP M–H.
- **Internet (fact, news):** a Philippines–Singapore/Hong Kong subsea cable outage in early October 2026 slowed PH internet. Reports conflict on whether it was fixed on 3 Oct or would take weeks [F29] TP M. No survey of home-fibre outage rates found.
- **Inference (M):**
  - The Mac has far more RAM and CPU than any free VM. It needs no card and no vendor can reclaim it.
  - It must never sleep. Set "prevent automatic sleeping" or `pmset`, and run the app, Postgres and `cloudflared` as launchd services that start on boot.
  - A MacBook's battery rides through short brownouts; the home router does not without a small UPS. A UPS costs money, so it isn't ₱0.
  - When the Mac is off, inbound replies can't reach the app. The Email Worker's fallback forward to the owner's inbox (`hosting-and-email-infra.md`, Inbound) becomes the safety net.
  - Customer documents then sit on a personal computer. Keep FileVault on, keep the files encrypted by the app, and name this in the pilot privacy notice. It is a weaker story for a Data Privacy Act–minded bookkeeper.
  - It ties the service to the owner's house and laptop habits: travel, OS updates, a closed lid.

## 6. Cloudflare R2

- **Fact (vendor):** the free tier is still 10 GB-month of Standard storage, 1M Class A and 10M Class B operations a month, with free egress [H20] PP H.
- **Card:** third parties say R2 asks for a payment method before the first bucket [H22] TP L. Cloudflare accepts Visa, Mastercard, Amex, Discover, PayPal, Apple Pay, Google Pay, Link and UnionPay. With PayPal, the linked card or bank pays if the PayPal balance is short [F27] VD H.
- **Not found:** whether a PH PayPal account with no card linked is accepted. If a failed payment stops R2 access, "requests will return errors" [F27] VD H.
- **Inference (M):** with no card, store the encrypted files on the server's own disk. That works on Oracle (200 GB) and on the Mac. Put the off-machine backups somewhere card-free; which provider is **not researched**.

## Unresolved

1. Does the owner hold a bank-issued credit card, or a debit card that works like one? This decides primary vs fallback.
2. Does a PAYG upgrade exempt Oracle Always Free instances from idle reclamation under the **current** policy? Only a 2023 email says so [F4]. Ask Oracle support in writing after sign-up.
3. Is Oracle A1 or E2.1.Micro capacity actually available in `ap-singapore-1` for a new tenancy? Only an attempt answers it.
4. Is the GCash Card prepaid, and do Oracle, Google, Azure or Cloudflare accept it? Not found; a try costs nothing but may lock in a home region.
5. Card-free off-machine storage for nightly backups (for the Mac fallback): not researched.

## Sources

All accessed 2026-10-21; "n.d." = no date shown; **SR** = read from a search-result summary because page fetches failed today. [H…] tags are in `docs/research/hosting-and-email-infra.md`.

- [F1] Oracle Docs, "Always Free Resources", https://docs.oracle.com/en-us/iaas/Content/FreeTier/resourceref.htm and https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm, n.d., VD, SR, H
- [F2] Oracle, "FAQ on Oracle's Cloud Free Tier", https://www.oracle.com/cloud/free/faq/, n.d., VD, SR, H
- [F3] Oracle Docs, "Oracle Cloud Infrastructure Free Tier", https://docs.oracle.com/iaas/Content/FreeTier/freetier.htm, n.d., VD, SR, H
- [F4] NetSec (51sec.org), "Oracle Cloud Cleaning Up Idle Compute Instances & How to Keep it" (reproduces an Oracle email), https://blog.51sec.org/2023/02/oracle-cloud-cleaning-up-idle-compute.html, 2023-02, TP, SR, M–L
- [F5] InfoQ, "Oracle cloud free tier limits", https://infoq.com/news/2026/07/oracle-cloud-free-tier-limits/, 2026-07; Linuxiac, "Oracle Quietly Cuts Free Tier Ampere A1 Resources in Half", https://linuxiac.com/?p=214114, 2026; labmemo.com, https://labmemo.com/?p=55273, 2026; TP, SR, M (cut) / L (PAYG treatment)
- [F6] Halocard, "Oracle" merchant page, https://halocard.co/merchants/oracle, n.d., TP (card vendor), SR, L
- [F7] LowEndTalk comment, https://lowendtalk.com/discussion/comment/3609767/, n.d., F, SR, L; braindetox.kr, "Running a Free Backend Forever on Oracle Cloud Always Free", https://braindetox.kr/en/posts/oracle_always_free_backend_2026.html, 2026, TP, SR, M; issoh.co.jp, https://www.issoh.co.jp/tech/details/17531/, n.d., TP, SR, M
- [F8] fingon.iki.fi, "I am done with Oracle cloud for now", https://www.fingon.iki.fi/blog/i-am-done-with-oracle-cloud-for-now/, 2026; Oracle Forums, "machine got terminated without reason", https://forums.oracle.com/ords/apexds/post/machine-got-terminated-without-reason-2645; F, SR, L
- [F9] Google Cloud, "Compute Engine free tier: getting started", https://cloud.google.com/free/docs/compute-getting-started, n.d., VD, SR, H; DEV Community GCP free tier guide, https://dev.to/jeaniscoding/how-to-host-your-side-projects-for-0-the-ultimate-gcp-free-tier-guide-3p07, TP, SR, M
- [F10] Google Cloud, "VPC pricing: external IP address price change", https://cloud.google.com/vpc/pricing-announce-external-ips, n.d., VD, SR, H; GitHub codingyoddha, "gcp-free-tier-compute-terraform", https://github.com/codingyoddha/gcp-free-tier-compute-terraform, TP, SR, L
- [F11] Google Cloud Billing, "Add, remove, or update a payment method", https://docs.cloud.google.com/billing/docs/how-to/payment-methods; "Free cloud features and trial offer", https://docs.cloud.google.com/free, n.d., VD, SR, H
- [F12] AWS What's New, "AWS Free Tier now offers $200 in credits and 6-month free plan", https://aws.amazon.com/about-aws/whats-new/2025/07/aws-free-tier-credits-month-free-plan/, 2025-07-15; AWS News Blog, https://aws.amazon.com/blogs/aws/aws-free-tier-update-new-customers-can-get-started-and-explore-aws-with-up-to-200-in-credits/, VD, SR, H
- [F13] Microsoft, "Azure free account", https://azure.microsoft.com/en-us/free/ and FAQ https://azure.microsoft.com/free/free-account-faq; "Troubleshoot issues when you sign up", https://docs.azure.cn/en-us/cost-management-billing/troubleshoot-subscription/troubleshoot-azure-sign-up, n.d., VD, SR, H
- [F14] Render Docs, "Deploy for Free", https://render.com/docs/free, n.d., VD, SR, H
- [F15] TechCrunch, "Mistral AI buys Koyeb", https://techcrunch.com/2026/02/17/mistral-ai-buys-koyeb-in-first-acquisition-to-back-its-cloud-ambitions, 2026-02-17; Pulse2 report (same event); Koyeb Docs, "Instances", https://www.koyeb.com/docs/reference/instances; TP/VD, SR, M
- [F16] Flavio Copes, "Every hosting provider's free tier, side by side (checked September 2026)", https://flaviocopes.com/hosting-free-tiers/, 2026-09, TP, SR, M
- [F17] Railway Central Station, "Clarification on Free Serverless Deployment & PostgreSQL Access", https://station.railway.com/questions/clarification-on-free-serverless-deploym-3d247324; Kuberns, "Railway Free Tier in 2026", https://kuberns.com/blogs/railway-free-tier/; F/TP, SR, L–M
- [F18] Northflank, "Pricing", https://northflank.com/pricing; Docs, "Cloud provider billing", https://northflank.com/docs/v1/application/billing/cloud-provider-billing, n.d., PP/VD, SR, H; Modmail docs PR (crash report), https://github.com/modmail-dev/modmail-docs/pull/55/files, 2025, F, SR, L
- [F19] Zeabur Docs, "Free Plan" and "Dev Plan", https://zeabur.com/docs/en-US/pricing/free-plan, https://zeabur.com/docs/en-US/pricing/dev-plan, n.d., VD, SR, H
- [F20] Supabase Docs, "Project Pausing", https://supabase.com/docs/guides/platform/free-project-pausing, n.d., VD, SR, H
- [F21] Jet Admin, "Neon Pricing in 2026", https://www.jetadmin.io/blog/neon-pricing/, 2026-09-26, TP, SR, M
- [F22] Aiven Docs, "PostgreSQL free tier", https://aiven.io/docs/products/postgresql/concepts/pg-free-tier, n.d.; Aiven blog, "New developer tier for Aiven for Postgres", https://aiven.io/blog/new-developer-tier-for-aiven-for-postgres, 2025-11-12; VD, SR, H
- [F23] Prisma, "Prisma Postgres vs Neon pricing 2026", https://www.prisma.io/blog/prisma-postgres-vs-neon-pricing-2026; Vercel Community, "Prisma Postgres suspended before reaching 100k limit", https://community.vercel.com/t/prisma-postgres-suspended-before-reaching-100k-limit/26355, 2025-10; VD/F, SR, M–L
- [F24] Xata, "Pricing" (machine-readable, updated 2026-05-25), https://xata.io/pricing.md; AgentDeals, "Best Xata Alternatives", https://agentdeals.dev/alternative-to/xata; VD/TP, SR, L
- [F25] Cloudflare Docs, Tunnel (full text index), https://developers.cloudflare.com/tunnel/llms-full.txt; "Quick Tunnels", https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/do-more-with-tunnels/trycloudflare/, n.d., VD, SR, H; abcsteps.com tunnel guide, TP, SR, M
- [F26] Cloudflare Docs, "Workers limits" (request body size by account plan), https://developers.cloudflare.com/workers/platform/limits/, n.d., VD, SR, H
- [F27] Cloudflare Docs, "Create billing profile", https://developers.cloudflare.com/billing/create-billing-profile/; "Billing FAQ", https://developers.cloudflare.com/billing/understand/faq/; "Billing policy", n.d., VD, SR, H
- [F28] Philstar, "Grid alerts trigger power outages in Luzon, Visayas", https://philstar.com/nation/2026/05/14/2527853/grid-alerts-trigger-power-outages-luzon-visayas, 2026-05-14; Daily Tribune, "Third straight day of red alerts rattles Luzon", https://tribune.net.ph/2026/05/15/third-straight-day-of-red-alerts-rattles-luzon, 2026-05-15; Daily Tribune, Visayas/Mindanao red alert, https://tribune.net.ph/2026/09/03/visayas-at-mindanao-grid-nasa-red-alert, 2026-09-03; TP (news), SR, M–H
- [F29] GMA News, "Slow, intermittent internet in PH linked to submarine cable outage", https://www.gmanetwork.com/news/scitech/technology/1004568/slow-intermittent-internet-in-ph-on-friday-oct-10-2026-linked-to-submarine-cable-outage/story/, 2026-10; The Star, "Philippines-Singapore internet link restored after cable outage", https://www.thestar.com.my/aseanplus/aseanplus-news/2026/10/03/philippines-singapore-internet-link-restored-after-cable-outage, 2026-10-03; Newsbytes.PH, https://newsbytes.ph/?p=112189, 2026-10-03; TP (news), SR, M
- [F30] Maya, "Activate virtual card", https://maya.ph/activate-virtual-card, n.d., VD, SR, H; Visa Philippines, "GCash launches new 'GCash Card' powered by Visa", https://www.visa.com.ph/about-visa/newsroom/press-releases/gcash-launches-new-gcash-card-powered-by-visa.html, 2023-10, VD, SR, H. That these cards are prepaid or virtual in Oracle's and Google's sense is an inference.
- [F31] Google Cloud, "Cloud Run pricing", https://cloud.google.com/run/pricing, n.d., PP, SR, M (the summary was fragmentary)
