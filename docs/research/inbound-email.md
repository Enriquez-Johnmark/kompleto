# Receiving replies and bounces (M5 research)

Checked 2026-10-21 for the M5 grilling round. developers.cloudflare.com, resend.com and rfc-editor.org could not be fetched from the build sandbox, so the two primary sources are the vendors' own published sources on GitHub:

- **[CF]** cloudflare/cloudflare-docs on GitHub, commit a6411c1 (2026-10-21); Email Routing now sits under `/email-service/`. Vendor documentation.
- **[R-OAS]** resend/resend-openapi `resend.yaml`, commit 69ea636 (2026-10-16). Vendor API specification.

Claims marked **[search]** come from search-result excerpts only: lower confidence. Everything a vendor says about itself is a vendor claim.

## Cloudflare Email Routing and Email Workers

| Claim | Source | Confidence |
|---|---|---|
| Plus addressing (RFC 5233) is supported once enabled in Email Routing settings; `user+detail@` matches the `user@` rule and `+detail` stays in `message.to`. | [CF] `email-service/configuration/email-routing-addresses.mdx`; changelog 2025-07-21 | High |
| A catch-all address (apex domains only) can "Send to a Worker". | [CF] same file | High |
| Inbound Email Routing is "Unlimited" on Workers Free and Paid; the Worker is billed as a normal Worker. | [CF] `email-service/platform/pricing.mdx` | High (vendor claim) |
| Inbound messages up to 25 MiB; heavy handlers on the Free plan can fail with `EXCEEDED_CPU`. | [CF] `email-service/platform/limits.mdx` | High |
| Workers Free: 100,000 requests a day, 10 ms CPU per invocation (waiting on `fetch()` doesn't count), 50 subrequests. Whether email invocations count toward the 100,000: not found. | [CF] `workers/platform/limits.mdx` | High |
| `message.raw` is a stream of the raw MIME (`await new Response(message.raw).arrayBuffer()`); `setReject()` and `forward()` exist. | [CF] `email-service/api/route-emails/email-handler.mdx` | High |
| An Email Worker may `fetch()` an outside HTTPS endpoint (counted as a subrequest). No email example shows it. | [CF] `workers/platform/limits.mdx` | Medium (inferred) |

## Resend

| Claim | Source | Confidence |
|---|---|---|
| Webhook events include `email.bounced`, `email.complained`, `email.delivery_delayed`, `email.failed`, `email.suppressed`, `email.received`. | [R-OAS] | High |
| `data.email_id` is the `id` the send call returned. | [R-OAS] `OutboundEmailEventData` | High |
| `data.bounce.type` is Permanent, Transient or Undetermined, with a `subType` (e.g. NoEmail, MailboxFull). | [R-OAS] `WebhookEventBounce` | High |
| Webhooks are signed with Svix headers (`svix-id`, `svix-timestamp`, `svix-signature`), secret `whsec_…`, verified over the raw body. | [R-OAS]; resend.com/docs/webhooks/verify-webhooks-requests [search] | High |
| Delivery is at-least-once; deliveries older than 5 minutes are rejected (third-party guide, Hookdeck). | [search] | Low |
| Free plan: 3,000 emails a month, 100 a day, webhooks with 1 endpoint. | resend.com/pricing [search] | Medium |
| Per-email `reply_to` and custom `headers` (including `In-Reply-To`, `References`) are supported. | [R-OAS] `SendEmailOptions` | High |
| Setting our own `Message-ID`: not found. The send response returns only `id`; retrieving the email returns `message_id`. | [R-OAS] | High |
| Resend also receives mail (`email.received`, `/emails/receiving`) and stores it if the webhook is down; it appears on the Free plan. Whether received mail counts toward the 3,000 a month: unclear (only a competitor's blog says so). | resend.com/docs/dashboard/receiving/introduction [search] | Medium / Low |

## Recognising automatic replies

| Claim | Source | Confidence |
|---|---|---|
| `Auto-Submitted` (RFC 3834, Aug 2004, Proposed Standard): `no` is the same as no header; any other value (`auto-generated`, `auto-replied`) means automated. | rfc-editor.org/rfc/rfc3834; IANA auto-submitted keywords [search] | High |
| `X-Auto-Response-Suppress` asks others not to auto-reply; Exchange adds it to all mail to distribution lists, so alone it gives false positives. | redmine.org/issues/19558 [search] | Medium |
| `Precedence: bulk/list/junk`, `List-Id`, `List-Unsubscribe` mark bulk or list mail. | arp242.net/autoreply [search] | Medium |
| `X-Autoreply` and `X-Autorespond` are used by some auto-responders. | arp242.net/autoreply; developer.nylas.com [search] | Low |
| Google's and Microsoft's own documentation on these headers: not found. | — | — |
