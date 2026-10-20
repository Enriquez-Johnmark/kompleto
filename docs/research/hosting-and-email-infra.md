# Hosting and email infrastructure for Kompleto (P1)

Status: research note, 2026-10-21. **Nothing here is an owner-approved decision.** It feeds the Phase 7 architecture gate. Inputs: `docs/product/p1-spec.md` (acceptance criteria A1–J2), the "Technical direction" section of `docs/research/README.md`, and `docs/research/p1-launch-prerequisites.md` §4 (email).

Budget: ₱600/month total (~US$10). The domain and LLM already take ~₱80, so **hosting must fit in about US$8/month** until a customer pays. Prices are in US$ excluding tax unless stated. The peso figures use ~₱61/US$ (BSP reference, Aug 2026, [L40] in `p1-launch-prerequisites.md`).

Labels: **PP** = vendor pricing page, **VD** = vendor docs, **API** = vendor's public machine-readable API, **OD** = official docs or terms, **TP** = third party, **F** = forum, **SR** = taken from a search-result summary rather than a fetched page. Confidence is **H/M/L**. Tags like [H5] point to Sources. Every source was accessed on 2026-10-21. **Fact**, **Inference** and **Recommendation** are marked where they could be confused.

## Recommendation

*Recommendation, not owner-approved.*

**Stack: one US$5 Singapore VPS running the Rust binary, Postgres and Caddy. Encrypted files go to Cloudflare R2. Replies arrive through Cloudflare Email Routing and an Email Worker that POSTs the raw MIME to the app. Pilot mail goes out through Resend Free, and SES ap-southeast-1 takes over once production access is granted.**

| Line item | What | US$/month | Source |
|---|---|---|---|
| Compute + Postgres | Vultr Cloud Compute Regular `vc2-1c-1gb`, Singapore: 1 vCPU, 1 GB RAM, 25 GB SSD, 1 TB transfer | **5.00** | [H5] API, H |
| Machine backups | Vultr automatic backups, +20% of the instance price | **1.00** | [H6] VD, M–H |
| Possible PH VAT | 12% VAT on digital services (RA 12023), if Vultr charges it. Not confirmed | **0.72** (worst case) | [L41] TP, M |
| Database backups | Nightly `pg_dump`, encrypted, to R2. 30-day lifecycle rule | 0 (inside R2 free) | [H20], [H21] PP/VD, H |
| Object storage | Cloudflare R2 Standard. Free: 10 GB-month, 1M Class A, 10M Class B, free egress | **0** | [H20] PP, H |
| DNS + reply address | Cloudflare DNS + Email Routing + Email Worker, Free plan | **0** | [H29], [H32] VD, H |
| Outbound email (pilots) | Resend Free: 3,000/month, **100/day** | **0** | [H36] PP, H |
| Outbound email (after SES approval) | SES ap-southeast-1, $0.10/1,000. ~3,000/month ≈ $0.30. $0 while the US$200 credits last | **0–0.30** | [H26] PP, H |
| Bounce notifications (SES) | SNS HTTPS deliveries. Reported free tier 100k/month | **0** | [H28] TP, M |
| TLS | Caddy with automatic Let's Encrypt/ZeroSSL | **0** | [H47] VD, H |
| Uptime + cron heartbeats | Better Stack Free (10 monitors) + Healthchecks.io Hobbyist (20 checks) | **0** | [H50], [H51] PP, H |
| Error tracking | Sentry Developer (free): 5k errors, 1 user | **0** | [H52] PP, H |
| **Total** | | **≈ 6.72 worst case (≈ ₱410)**; 6.00 without VAT | computed |

With the ₱80 for domain and LLM, the total is about **₱490 of ₱600**. That leaves ~₱110 of headroom.

**Why this shape** *(inference, M–H)*
- One small box at US$5 is the cheapest **always-on** option in Singapore. Every free compute or app-host tier either sleeps (Render), has no free plan (Fly.io), or carries reclamation risk (Oracle). The app needs to be always on for the job table, the 5-minute reply SLA (G1) and the deletion jobs (F3).
- At 3–20 firms the database is tiny: no files, a few thousand rows a month. A 1 GB VPS holds Postgres, the binary and Caddy. Add a 1–2 GB swap file as a safety margin.
- **Files live off the box in R2**, so the VPS backup holds only the OS and the database. An **R2 lifecycle rule deleting objects after 90 days** is a server-side backstop for F3 ("never kept beyond 90 days"), separate from the app's own 30-day deletion.
- **DB backups are kept ≤ 30 days**, so closing an account (I2, "within 30 days") also clears the backups in time. Vultr keeps only the two most recent automatic backups [H6].
- **Inbound goes through Cloudflare, not Resend.** Resend counts every received email against the 3,000/month quota [H36]. Cloudflare Email Routing is free and hands the Worker the raw MIME [H31].

**Fallback** *(recommendation)*
- **Compute:** Linode/Akamai Nanode in Singapore. US$5, 1 GB, 25 GB, 1 TB, with backups at +US$2 [H7], so US$7 plus possible VAT. DigitalOcean's US$6 1 GB Droplet in SGP1 is the other choice; weekly backups add 20% [H3].
- **Outbound:** if Resend's 100/day cap bites and SES is still denied, **Cloudflare Workers Paid (US$5/mo minimum) gives 3,000 sends/month with no daily limit stated**, then $0.35/1,000 [H44], [H46]. It is beta [H45], and it takes the total to ≈ US$11.7, **over budget**. Resend Pro is US$20 [H36], further over.
- **Inbound:** SES receiving in ap-southeast-1 to S3, plus an SNS HTTPS notification to the app, at $0.10/1,000 emails [H25]–[H27].

**Single biggest risk: outbound sending capacity on BIR deadline days.** All clients share the same BIR deadlines, so the "1 day before cut-off" reminders bunch onto the same day. Five firms × 30 clients is 150 approved sends in a day, which exceeds **Resend Free's 100/day** [H36]. The escape route, **SES production access, can be denied** for new accounts with no AWS billing history (forum reports [L34], L–M). Every paid alternative (Resend Pro US$20, Workers Paid US$5) breaks the US$8 line.

Mitigations *(recommendation)*:
1. Request SES production access in week 1, as `p1-launch-prerequisites.md` already says.
2. The app enforces a per-day send budget and warns the bookkeeper the day before.
3. Approval batches go out from the earliest reminder (7 days before) rather than the last day.
4. Hold Workers Paid as a one-month emergency spend.

Second risk: **the single VPS is a single point of failure**. Restoring a backup could lose "sent" markers from after the last backup and re-send mail (E1). Resend's 24-hour `Idempotency-Key` [H41] covers only same-day restores. *(Inference.)*

## Compute & Postgres

### Options near the Philippines

**Fact:** no candidate offers a **Manila** region.
- Vultr's Asian regions are Bangalore, Mumbai, Delhi, Seoul, Osaka, Tokyo, Singapore and Tel Aviv [H5].
- Linode's nearby regions are Singapore (×2), Jakarta, Tokyo and Osaka [H7].
- Neon, Supabase and Railway list Singapore as their only Southeast Asian region [H11], [H17], [H19].

Singapore is the default.

| Option | Cheapest always-on fit (Singapore) | US$/mo | Postgres | Backups | Notes | Source |
|---|---|---|---|---|---|---|
| **Vultr** | `vc2-1c-1gb`: 1 vCPU, 1 GB, 25 GB, 1 TB; available in `sgp` | **5.00** | Self-hosted on the box | +20% (US$1); keeps the 2 most recent | The 512 MB plans (US$2.50 IPv6-only, US$3.50) are **not** offered in Singapore | [H5] API H; [H6] VD M–H |
| **Linode/Akamai** | `g6-nanode-1`: 1 vCPU, 1 GB, 25 GB, 1 TB; Singapore at base price (only Jakarta and São Paulo cost more) | **5.00** | Self-hosted | +US$2.00/mo | | [H7] API H |
| **DigitalOcean** | Basic: 1 GiB, 1 vCPU, 25 GiB, 1,000 GiB | **6.00** (512 MiB: 4.00) | Self-hosted. Managed Postgres price not on the page | Weekly 20%, daily 30%; snapshots $0.06/GB | Docs: "no regional variations" in bandwidth pricing; overage $0.01/GiB | [H3] PP H; [H4] OD SR M–H |
| **AWS Lightsail** | 1 GB, 2 vCPU, 40 GB, 2 TB: US$7 (IPv4) / US$5 (IPv6-only). 512 MB: US$5 / US$3.50 | 7.00 | Managed from **US$15** | Snapshots $0.05/GB-mo | Singapore isn't on the "half transfer allowance" list | [H8] PP H |
| **Hetzner** | Singapore `CPX02`: 1 vCPU, 1 GB, 20 GB, **0.5 TB** for **€7.99**. `CPX12` (2 GB): €15.99 | ≈ 9+ | Self-hosted | Price not found on the page | Prices rose for new orders from 15 Jun 2026; Singapore costs more than the EU (the same CPX02 is €6.49 in the EU) | [H1] PP H; [H2] TP M |
| **Fly.io** | `shared-cpu-1x` 1 GB in `sin`: **US$8.50** (US$6.70 in iad). 256 MB: US$2.19 in iad; sin price not shown | 8.50 for 1 GB | Managed Postgres from **US$38** | Volumes $0.15/GB | **No free tier for new organisations**; trial is 2 h or 7 days | [H9] PP H |
| **Railway** | Hobby US$5/mo, includes US$5 usage. RAM ≈ $10/GB-mo, CPU ≈ $20/vCPU-mo, volume $0.15/GB, egress $0.05/GB. Singapore region `asia-southeast1-eqsg3a` | ≈ 5 if usage stays ≤ US$5 | Self-run on a volume, or external | Not found | Free plan: US$1/mo credit, 0.5 GB, **0 custom domains** | [H10] PP H; [H11] VD H |
| **Render** | Free web service **spins down after 15 min idle** (~1 min wake). Starter web + Basic Postgres ≈ **US$13**; Singapore offered | 13 | Free Postgres **expires after 30 days**, 1 GB, no backups | | Free tier unusable for an always-on job runner | [H12] VD H; [H13] TP/vendor article SR M |
| **Oracle Always Free** | 2 × AMD micro (1/8 OCPU, 1 GB, 50 Mbps) + Ampere A1 2 OCPU / 12 GB | **0** | Self-hosted; Autonomous DB isn't Postgres | 5 volume backups free | See risks below | [H14] OD H; [H15] TP SR M |

### Oracle Always Free: is Singapore usable, and is it reliable?

- **Facts:**
  - Always Free compute must be created **in the home region** chosen at sign-up.
  - "Out of host capacity" errors are documented: retry, change availability domain, or upgrade to Pay as You Go.
  - Oracle reclaims instances it deems **idle**: over 7 days, "CPU utilization for the 95th percentile is less than 20%", "Network utilization is less than 20%", and memory under 20% (A1 only) [H14].
- **Fact (TP):** Oracle **halved** the A1 allowance to 1,500 OCPU-h / 9,000 GB-h a month, enforced from **18 Aug 2026**, and terminated instances above it [H15].
- **Reports:** forum posts describe accounts disabled without notice, and free-tier users have no full technical support [H15] (F/TP, L).
- **Singapore (`ap-singapore-1`):** **not verified** as a sign-up home region with free capacity for new accounts. Whether upgrading to Pay as You Go exempts the instance from idle reclamation: **not found** in the official text fetched.
- **Inference (M):** Kompleto at pilot scale **is** an idle workload by Oracle's definition (p95 CPU well under 20%). Running production on it is a reliability gamble. It is fine for a throwaway staging box.

### Free managed Postgres

| | Neon Free | Supabase Free |
|---|---|---|
| Size | 1 GB per project (20 GB per account) | 500 MB database, shared CPU, 500 MB RAM |
| Compute | **100 CU-hours per project per month** | Shared |
| Sleep | **Suspends after 5 min idle, cannot be turned off on Free** | **Projects pause after 1 week of inactivity** |
| Backups | 6-hour history window (1 GB) | **No automatic backups** |
| Egress | 5 GB per project | 5 GB + 5 GB cached |
| Singapore | `aws-ap-southeast-1` | `ap-southeast-1` |
| Paid step | Launch: $0.106/CU-h, $0.35/GB-mo | Pro US$25/mo |
| Source | [H16] PP H; [H17] VD H | [H18] PP H; [H19] VD H |

**Inference (M):** both clash with the planned `FOR UPDATE SKIP LOCKED` job table (README "Technical direction").
- **Neon:** a poller that runs every few seconds keeps Neon awake around the clock. At an assumed 0.25 CU minimum, that's ~180 CU-h a month, above the 100 CU-h cap. The 0.25 CU minimum is not verified on the fetched page.
- **Neon, other way round:** if Neon is left to sleep, each wake adds latency to the job runner.
- **Supabase:** the database never pauses under constant polling, but it has **no backups**. We'd have to run `pg_dump` ourselves anyway.

### Single VPS vs app host + free managed Postgres

| | **Single VPS (binary + Postgres + Caddy)** | **App host + free Postgres** (e.g. Railway Hobby + Supabase Free, both in Singapore) |
|---|---|---|
| Monthly | US$5–6 (+VAT?) | ~US$5 if Railway usage stays within its US$5 credit (inference) |
| Vendors / bills | 1 (+ Cloudflare free) | 2–3 |
| Postgres control | Full: extensions, `SKIP LOCKED` polling, `pg_dump` cron | Free-tier policy limits (pausing, CU-hours, 500 MB, no backups) |
| Failure modes | Box dies → restore from backup (RPO ≤ 24 h) | Either vendor's free-tier policy changes; cross-provider latency inside Singapore |
| Ops burden | OS patching, Postgres upgrades, firewall | Lower OS burden, but backups still DIY on Supabase Free |
| Fit for E1 (no double send) | Send markers and the queue in one local DB | Same, over the network |

**Recommendation (M):** the single VPS. It costs about the same, is one bill, and no free-tier policy can pause or cap the database. The ops burden is what the owner takes on.

### Backups (cost)

- **Vultr automatic backups:** +20% = US$1/mo. Keeps the 2 most recent [H6].
- **Linode:** +US$2/mo [H7].
- **DigitalOcean:** 20% weekly / 30% daily [H3].
- **Lightsail snapshots:** $0.05/GB-mo [H8].
- **Database-level (recommended):** a nightly `pg_dump`, encrypted (e.g. with `age`) before upload, sent to R2 under a 30-day lifecycle rule [H21]. Pilot-scale dumps are a few MB, so this stays inside R2's 10 GB free tier [H20] *(inference)*.
- **Healthchecks.io:** a ping from the backup cron, so a missed backup raises an alert [H51].

## Object storage

| Option | Free / base | Beyond | Egress | Min. storage duration | Region near PH | Source |
|---|---|---|---|---|---|---|
| **Cloudflare R2** | **10 GB-month, 1M Class A, 10M Class B per month (Standard only)** | $0.015/GB-mo; Class A $4.50/M; Class B $0.36/M | **Free** | None on Standard (IA: 30 days) | Location hint; Singapore region not verified | [H20] PP H |
| Backblaze B2 | **First 10 GB free** | $6.95/TB-mo | Free up to 3× stored, then $0.01/GB | **None** | Regions not checked | [H23] PP H |
| Hetzner Object Storage | **€6.49/mo base** (1 TB + 1 TB egress) | €/TB-hour | Included 1 TB | n/a | **Falkenstein, Helsinki, Nuremberg only; no Singapore** | [H24] PP H |
| VPS disk | 25 GB on the Vultr plan | Upsize the plan | Inside 1 TB transfer | n/a | Singapore | [H5] |

**Sizing** *(assumption)*: an average upload of ~2 MB and a few thousand uploads a month means ~2–6 GB added a month. The app deletes 30 days after "received", with a hard cap of 90 days. Steady state is roughly 2–12 GB.
- **Pilots:** 5 firms × 30 clients × ~5 files ≈ 750 files, about 1.5 GB a month. Well inside R2's 10 GB.
- **At 20 firms:** this may pass 10 GB. The overage is $0.015/GB, so 5 GB extra costs about US$0.08 a month.

**Recommendation:**
- **Store files in R2, encrypted by the app before upload** (per-file key, AES-256-GCM, master key only on the VPS), so R2 holds ciphertext only.
- Add an **R2 lifecycle rule that deletes after 90 days** as a backstop for F3. R2 removes expired objects "typically … within 24 hours" [H21], so set the rule at 89 days.
- **Account caveat:** third-party guides say R2 may ask for a **payment method** before the first bucket is created [H22] (TP, L). Not confirmed by Cloudflare's docs.
- **Disk-only alternative:** works, and is simplest at pilot scale. But files then sit in the VPS backups, and Vultr's backup retention isn't under our control for the 90-day promise.

## Inbound email

Goal: replies to the Kompleto reply address reach the Rust app **as raw MIME within 5 minutes** (G1). Bounces must reach the app and match a message we actually sent (G3).

| Option | Raw MIME to the app? | Free limits / price | Notes | Source |
|---|---|---|---|---|
| **Cloudflare Email Routing + Email Worker** | **Yes.** The Worker gets `message.raw` (a `ReadableStream`, "Raw MIME email content stream") plus `rawSize` and headers. Posting it with `fetch()` isn't documented by Cloudflare, but community Workers do exactly that | **Email Routing: "Available on Free and Paid plans."** Workers Free: 100,000 requests/day, **10 ms CPU** per invocation, 50 subrequests, 128 MB. Inbound size limit **25 MiB**; 200 rules per domain | Free-plan warning: "complex handlers may exceed these limits." A POST-only Worker uses almost no CPU (I/O waits don't count; inference). `forward()` can only target a verified address. `setReject()` returns a permanent SMTP error | [H29]–[H32] VD H; [H33] TP SR L–M |
| **Amazon SES receiving** | **Yes.** The SNS action carries "a raw, unmodified copy of the email" up to **150 KB**; **larger mail bounces**. For larger mail, use the S3 action plus an SNS notification, and the app fetches from S3 | Inbound $0.10/1,000 emails + $0.09/1,000 chunks of 256 KB. SNS HTTP/S deliveries reportedly 100k/month free, then $0.60/M. US$200 new-account credits | **ap-southeast-1 is a receiving region** (`inbound-smtp.ap-southeast-1.amazonaws.com`). Resources must sit in the same region (except S3). SNS messages must be signature-verified by the app | [H25] OD H; [H26] PP H; [H27] OD H; [H28] TP SR M |
| **Resend inbound** | **No raw MIME documented.** The webhook carries metadata only. The body, text and headers come from the Receiving API; attachments via temporary URLs | Included on Free, but **each received email counts against the 3,000/month quota**. Free is limited to 3 inboxes | Using it would eat into the outbound allowance | [H34]–[H36] VD/PP H |
| **Postmark inbound** | JSON posted to a webhook; a raw-email field wasn't confirmed on the pricing page | Free plan is 100 emails/month; the comparison table suggests **inbound isn't on Free**. Basic US$15 | Too small or too costly | [H37] PP H (table hard to read: M) |
| **Mailgun routes** | **Yes.** If the forward URL ends in `mime`, Mailgun posts `body-mime` with the raw message | Free: 100 sends/day, **1 inbound route**, **1-day** log/message retention. Basic from US$15 (5 routes) | A workable free backup path, with very short retention | [H38] PP H; [H39] VD SR M–H |

**Cheapest reliable path (recommendation, M–H):**

1. **MX for `getkompleto.com` → Cloudflare Email Routing.** The domain is already planned for Cloudflare Registrar (`p1-launch-prerequisites.md` §4.3), so DNS stays there.
   - Rule: `reply@` (or a catch-all, if per-message reply tokens are used) → Email Worker. Whether Email Routing needs Cloudflare nameservers, or supports subdomains, isn't stated on the overview page [H29].
2. **The Worker POSTs `message.raw`** to `https://app.getkompleto.com/inbound/email`.
   - Headers: an HMAC signature, `rawSize`, and the envelope `from`/`to`.
   - If the POST fails, the Worker calls `message.forward()` to the owner's verified address, so no reply is lost. That address is an inbox, not the app, so the forwarded copy needs a manual follow-up.
3. **The app parses the MIME** with the Rust crate `mail-parser` (crate choice: inference, not researched here). It then classifies:
   - **DSN** (`multipart/report; report-type=delivery-status`, RFC 3464) → bounce handling, counted only if it matches a Message-ID we sent (G3).
   - **Auto-reply** (`Auto-Submitted`, RFC 3834; out-of-office) → product decision needed; see Unresolved.
   - **Human reply** → stop reminders, show on the dashboard, forward to the bookkeeper (G1).
4. **Fallback:** SES receiving in ap-southeast-1, with an S3 action plus an SNS HTTPS notification. Costs pennies, inside the AWS credits.

**Where bounces actually arrive** *(inference, M–H)*: with Resend or SES sending, the envelope Return-Path is the provider's or our custom MAIL FROM subdomain. SES publishes an MX for the MAIL FROM domain that points at its regional feedback endpoint [H43]. So **most bounces arrive as provider events** (next section), not as DSNs to our reply address. Late or misbehaving receivers sometimes mail the From/Reply-To address, which is why the app still classifies DSNs arriving through the Worker.

## Outbound email & bounces

| | **Resend Free** | **SES ap-southeast-1** | Cloudflare Email Sending (fallback) |
|---|---|---|---|
| Allowance | **3,000/month, 100/day**, 3 domains, 1 webhook endpoint, 30-day retention | Sandbox: 200/24 h, 1/s, verified recipients only, until production access; then pay per use | Workers Paid (min US$5/mo): **3,000/month included**, then $0.35/1,000; arbitrary recipients need Workers Paid |
| Price beyond | Pro US$20 for 50k (no daily limit); $0.90/1,000 overage | **$0.10/1,000** + $0.12/GB attachments; US$200 credits for new accounts (use within 12 months) | $0.35/1,000 |
| Bounce/complaint reporting | Webhooks: `email.bounced` (permanent rejection), `email.complained`, `email.delivery_delayed`, `email.suppressed`, `email.failed`. Retries keep the same `svix-id` (dedupe on it). Bounce payload fields not documented on that page | **SNS topic** per identity, or **event publishing** via a configuration set to SNS/Firehose (also EventBridge per AWS docs; not fetched). If no method is configured, SES forwards feedback by email to the Return-Path. SNS topic must be in the same region | Not documented; "Queues" can receive "Email Sending events" |
| Idempotency | **`Idempotency-Key` header, kept 24 h**; a duplicate returns the original response without re-sending | None at API level; rely on our own send markers (E1) | Not found |
| Status | GA | GA; **production approval risk** for new accounts | **Beta** |
| Source | [H36] PP H; [H40], [H41] VD H | [H26] PP H; [H42], [H43] OD H; sandbox [L33] | [H44]–[H46] VD H |

**Recommendation:**
- **Pilots:** Resend Free with an idempotency key per send.
- **Bounces:** receive `email.bounced` and `email.complained` on `POST /webhooks/resend`, verifying the Svix signature. Svix signing is assumed from the `svix-id` header and not confirmed on the fetched page.
- **Matching (G3):** match each event to our send record by Resend `email_id`.
- **After SES approval:** move to SES. Add a configuration set with event publishing to an SNS topic in ap-southeast-1, and an **HTTPS subscription to `POST /webhooks/ses`** that verifies the SNS signature and confirms the subscription. Match on the SES `messageId`.
- **Also keep a local suppression list in Postgres**, so a bounced address stops reminders regardless of provider (G3).
- **Watch:** Resend's Free plan allows **one webhook endpoint** [H36], so bounces and any other events share it.

## Ops basics

| Need | Option | Free limits | Source |
|---|---|---|---|
| TLS | **Caddy** automatic HTTPS: gets and renews certificates from Let's Encrypt / ZeroSSL; needs ports 80/443 open, A/AAAA records and a persistent data directory; "redirects HTTP … to HTTPS … automatically" | Free | [H47] VD H |
| Uptime | **Better Stack Free** | "10 monitors & heartbeats", 1 status page, Slack and email alerts, 3 GB logs kept 3 days. Labelled "Free for personal projects" | [H50] PP H |
| Uptime (alt.) | UptimeRobot Free | 50 monitors, **5-min interval**, 5 integrations, "Good for hobby and non-profit projects". **Commercial use is disputed:** the Dec 2024 terms reportedly banned it; one source says the May 2026 terms allow it | [H48] PP H; [H49] TP SR L–M |
| Cron heartbeats | **Healthchecks.io Hobbyist** | 20 checks, 100 log entries per check | [H51] PP H |
| Errors | **Sentry Developer (free)** | 5k errors, 1 user, 30-day lookback, 5M spans, 5 GB logs. Team plan US$29/mo | [H52] PP H |
| Logs | `tracing` to journald on the VPS, rotated | Free | Inference |

**Recommendation:**
- **TLS:** Caddy in front of axum.
- **Uptime:** Better Stack Free for an HTTPS check on `/health` (its "personal projects" label is a terms caveat, so UptimeRobot is the backup). Use Healthchecks.io for the nightly backup, the deletion job (F3) and the reminder scheduler.
- **Errors:** Sentry Free with PII sending off and request bodies scrubbed. Sentry becomes a **sub-processor** to list in the DPA (`p1-launch-prerequisites.md` §3).
- **No Sentry:** journald logs plus a daily error-count alert are enough at pilot scale.

## Unresolved questions

1. **Does Vultr (or Linode/DO) add 12% PH VAT** to a PH customer's bill? It's budgeted as the worst case (+US$0.72). Not confirmed.
2. **Is Oracle `ap-singapore-1` open to new Always Free sign-ups with A1/E2 capacity, and does a Pay-as-You-Go upgrade exempt instances from idle reclamation?** Not found in the official text.
3. **Cloudflare Email Worker failure semantics:** if the Worker throws (instead of calling `setReject`), does the sending server get a temporary 4xx and retry? Not found. This decides whether forwarding to a verified backup address is the only safety net.
4. **Does Cloudflare officially support `fetch()` with `message.raw` as the body,** and is there a size or CPU ceiling on Workers Free for a 10–25 MB reply with attachments? Only community examples were found [H33]. A one-hour throwaway prototype answers it.
5. **Email Routing on a subdomain and plus-addressing** (`reply+token@`): not stated on the fetched page. A catch-all rule is the workaround.
6. **Resend Free: does inbound or the 100/day cap apply to received mail,** and what fields does the `email.bounced` payload carry? Webhook signing (Svix) is assumed.
7. **Neon Free minimum compute size** (for the CU-hour estimate): not verified.
8. **R2 payment-method requirement and data location:** third-party only. Does a location hint place data in Singapore or only "APAC"? Not checked.
9. **Product decision for the owner:** should an **auto-reply / out-of-office** stop reminders (G1 says "a reply" stops them)? It needs a grilling round in Phase 7, not research.
10. **Brevo, SMTP2GO and other free outbound tiers** as overflow for the 100/day cap: Brevo's pricing page returned no content. Not researched further.
11. **Hetzner Singapore backup price** and the "€7.40 per TB" traffic overage wording: the page is ambiguous.

## Sources

All accessed 2026-10-21. Format: tag, publisher, title, URL, date, evidence type, confidence. "n.d." = no date on the page. [L33], [L34], [L40] and [L41] are in `docs/research/p1-launch-prerequisites.md`.

**Compute and Postgres**
- [H1] Hetzner, "Regular Performance" cloud plans (EU and SIN1 tables), https://www.hetzner.com/cloud/regular-performance ; overview https://www.hetzner.com/cloud/, n.d., PP (fetched), H
- [H2] Northflank, "Hetzner cloud server price increases", https://northflank.com/blog/hetzner-cloud-server-price-increases, 2026-06-17, TP (fetched), M
- [H3] DigitalOcean, "Droplet pricing", https://www.digitalocean.com/pricing/droplets, n.d., PP (fetched), H
- [H4] DigitalOcean Docs, "Bandwidth Billing", https://docs.digitalocean.com/platform/billing/bandwidth/ ; "Regional availability", https://docs.digitalocean.com/platform/regional-availability, n.d., OD, SR, M–H
- [H5] Vultr public API, `GET https://api.vultr.com/v2/plans?type=vc2` and `GET https://api.vultr.com/v2/regions` (queried 2026-10-21; `vc2-1c-1gb` monthly_cost 5, 1 vCPU, 1024 MB, 25 GB, 1024 GB bandwidth, available in 31 locations including `sgp`), API, H. Pricing page https://www.vultr.com/pricing/ returned 403
- [H6] Vultr Docs, "How much does it cost to enable automatic backups?", https://docs.vultr.com/support/platform/billing/how-much-does-it-cost-to-enable-automatic-backups ; "Automatic Backups", https://docs.vultr.com/vps-automatic-backups, 2025-12-16, VD, SR, M–H
- [H7] Linode/Akamai public API, `GET https://api.linode.com/v4/linode/types` and `/v4/regions` (queried 2026-10-21; `g6-nanode-1` $5/mo, region prices only for id-cgk $6 and br-gru $7; backups addon $2/mo), API, H
- [H8] AWS, "Amazon Lightsail pricing", https://aws.amazon.com/lightsail/pricing/, n.d., PP (fetched), H
- [H9] Fly.io, "Pricing", https://docs.fly.io/about/pricing, n.d., PP (fetched), H
- [H10] Railway, "Pricing", https://railway.com/pricing, n.d., PP (fetched), H
- [H11] Railway Docs, "Deployment regions", https://docs.railway.com/reference/deployment-regions, n.d., VD (fetched), H
- [H12] Render Docs, "Deploy for free", https://render.com/docs/free, n.d., VD (fetched), H
- [H13] Render, "How much does cloud application hosting cost for small businesses", https://render.com/articles/how-much-does-cloud-application-hosting-cost-for-small-businesses (cites July 2026), vendor article, SR, M; region list from third-party comparisons, SR, L–M. Pricing page https://render.com/pricing returned no figures
- [H14] Oracle, "Always Free Resources", https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm, n.d., OD (fetched), H
- [H15] InfoQ, "Oracle Cloud free tier limits", https://infoq.com/news/2026/07/oracle-cloud-free-tier-limits/, 2026-07 ; labmemo.com (Oracle notice, 18 Aug 2026), https://labmemo.com/?p=55273 ; Oracle Forums, "machine got terminated without reason", https://forums.oracle.com/ords/apexds/post/machine-got-terminated-without-reason-2645, TP/F, SR, M (limits) / L (terminations)
- [H16] Neon, "Pricing", https://neon.com/pricing, n.d., PP (fetched), H
- [H17] Neon Docs, "Regions", https://neon.com/docs/introduction/regions, n.d., VD (fetched), H
- [H18] Supabase, "Pricing", https://supabase.com/pricing, n.d., PP (fetched), H
- [H19] Supabase Docs, "Available regions", https://supabase.com/docs/guides/platform/regions, n.d., VD (fetched), H

**Object storage**
- [H20] Cloudflare Docs, "R2 pricing", https://developers.cloudflare.com/r2/pricing/, n.d., PP (fetched), H
- [H21] Cloudflare Docs, "R2 object lifecycles", https://developers.cloudflare.com/r2/buckets/object-lifecycles/, n.d., VD (fetched), H
- [H22] tech-insider.org, "How to set up Cloudflare R2 (2026)", https://tech-insider.org/au/how-to-set-up-cloudflare-r2-2026/ ; Flexxible docs, https://docs.flexxible.com/flexxclient/en/portal/guides/r2storageforms, TP, SR, L
- [H23] Backblaze, "B2 Cloud Storage pricing", https://www.backblaze.com/cloud-storage/pricing, n.d., PP (fetched), H
- [H24] Hetzner, "Object Storage", https://www.hetzner.com/storage/object-storage/, n.d., PP (fetched), H

**Inbound email**
- [H25] AWS General Reference, "Amazon SES endpoints and quotas" (Email Receiving endpoints table), https://docs.aws.amazon.com/general/latest/gr/ses.html ; SES DG "Regions and Amazon SES", https://docs.aws.amazon.com/ses/latest/dg/regions.html, n.d., OD (fetched), H
- [H26] AWS, "Amazon SES pricing", https://aws.amazon.com/ses/pricing/, n.d., PP (fetched), H
- [H27] AWS SES DG, "Publish to Amazon SNS topic action", https://docs.aws.amazon.com/ses/latest/dg/receiving-email-action-sns.html, n.d., OD (fetched), H
- [H28] Cloudburn, "Amazon SNS pricing" calculator and blog, https://cloudburn.io/tools/amazon-sns-pricing-calculator ; https://cloudburn.io/blog/amazon-sns-pricing, n.d., TP, SR, M. The official page https://aws.amazon.com/sns/pricing/ returned no rate table
- [H29] Cloudflare Docs, "Email Routing" overview, https://developers.cloudflare.com/email-routing/, n.d., VD (fetched), H
- [H30] Cloudflare Docs, "Email Routing limits", https://developers.cloudflare.com/email-routing/limits/, n.d., VD (fetched), H
- [H31] Cloudflare Docs, "Email Workers runtime API", https://developers.cloudflare.com/email-routing/email-workers/runtime-api/, n.d., VD (fetched), H
- [H32] Cloudflare Docs, "Workers limits", https://developers.cloudflare.com/workers/platform/limits/, n.d., VD (fetched), H
- [H33] GitHub nhdms, "cloudflare-email-webhook-worker", https://github.com/nhdms/cloudflare-email-webhook-worker ; DEV Community, "How to process incoming emails and trigger webhooks … using Cloudflare", https://dev.to/elvisans/how-to-process-incoming-emails-and-trigger-webhooks-in-app-actions-and-more-using-cloudflare-5d07, n.d., TP, SR, L–M
- [H34] Resend Docs, "Receiving emails", https://resend.com/docs/dashboard/receiving/introduction, n.d., VD (fetched), H
- [H35] Resend Docs, "Get email content", https://resend.com/docs/dashboard/receiving/get-email-content, n.d., VD (fetched), H
- [H36] Resend, "Pricing", https://resend.com/pricing, n.d., PP (fetched), H
- [H37] Postmark, "Pricing", https://postmarkapp.com/pricing, n.d., PP (fetched), H (Free-plan inbound row: M)
- [H38] Mailgun, "Pricing", https://www.mailgun.com/pricing/, n.d., PP (fetched), H
- [H39] Mailgun Docs, "Receive via HTTP" and "Receiving, forwarding and storing messages", https://documentation.mailgun.com/docs/mailgun/user-manual/receive-forward-store/receive-http, n.d., VD, SR, M–H

**Outbound email and bounces**
- [H40] Resend Docs, "Webhook event types", https://resend.com/docs/dashboard/webhooks/event-types, n.d., VD (fetched), H
- [H41] Resend Docs, "Idempotency keys", https://resend.com/docs/dashboard/emails/idempotency-keys, n.d., VD (fetched), H
- [H42] AWS SES DG, "Setting up event notifications for Amazon SES", https://docs.aws.amazon.com/ses/latest/dg/monitor-sending-activity-using-notifications.html, n.d., OD (fetched), H
- [H43] AWS SES DG, "Regions and Amazon SES" (feedback notifications, custom MAIL FROM feedback endpoints), https://docs.aws.amazon.com/ses/latest/dg/regions.html, n.d., OD (fetched), H
- [H44] Cloudflare Docs, "Email Service pricing", https://developers.cloudflare.com/email-service/platform/pricing/, n.d., VD (fetched), H
- [H45] Cloudflare Docs, "Email Service" overview (Email Sending Beta), https://developers.cloudflare.com/email-service/, n.d., VD (fetched), H
- [H46] Cloudflare Docs, "Workers pricing", https://developers.cloudflare.com/workers/platform/pricing/, n.d., VD (fetched), H

**Ops**
- [H47] Caddy, "Automatic HTTPS", https://caddyserver.com/docs/automatic-https, n.d., VD (fetched), H
- [H48] UptimeRobot, "Pricing", https://uptimerobot.com/pricing/, n.d., PP (fetched), H
- [H49] host-tracker.com, "UptimeRobot alternative" (reports May 2026 terms allowing commercial use), https://www.host-tracker.com/en/blog/uptimerobot-alternative ; DEV Community, "UptimeRobot commercial use: free alternatives for business sites in 2026", https://dev.to/velprove/uptimerobot-commercial-use-free-alternatives-for-business-sites-in-2026-5d75, 2026, TP, SR, L–M (conflicting)
- [H50] Better Stack, "Pricing", https://betterstack.com/pricing, n.d., PP (fetched), H
- [H51] Healthchecks.io, "Pricing", https://healthchecks.io/pricing/, n.d., PP (fetched), H
- [H52] Sentry, "Pricing", https://sentry.io/pricing/, n.d., PP (fetched), H
