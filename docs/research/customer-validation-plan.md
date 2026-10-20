# Customer validation plan

Status: **partly superseded (2026-10-20).** The owner chose to build the product first, so the hand-run concierge pilots and the build gate below no longer gate code. The outreach method (free written outreach) and the pilot offer (free 4 weeks, then ₱1,500 via GCash) carry over to the built product. The hypothesis and thresholds below were written for B and are **replaced** by Kompleto's pilot success measures in `docs/product/p1-spec.md` → "Pilots and launch". Kept for the record.

Original status: **agreed method (owner decisions 2026-10-18); not yet started.** Nothing has been validated. Owner approval of a hypothesis does not count as validation.

## Constraints this plan is built around

- No business network; no interviews or calls (owner constraint).
- No paid ads; cash spend ≤ ~₱600/month during pilots (owner decision).
- Written, asynchronous contact with people who sign up is fine (owner constraint).

So the plan measures **what strangers do**: reply, join a pilot, approve drafts, pay. It doesn't measure what they say they'd do.

## Lead hypothesis (finalist B)

> Owners of small Philippine service businesses that sell through quotes or bookings let inquiries and sent quotes go cold. A service that drafts follow-ups in their voice, which they approve with one tap, recovers enough replies and bookings that they will pay ₱1,500/month for it.

| # | Claim | Measured by |
|---|---|---|
| H1 | They have cold leads/quotes worth chasing | Count of open threads each pilot business shares per week |
| H2 | Owners accept an approval step | Share of drafts approved (with or without edits) |
| H3 | Follow-ups recover business | Replies and bookings on followed-up threads, in pesos where known |
| H4 | They'll pay | Paid conversion at the end of the pilot |
| H5 | Strangers respond to written outreach at all | Reply rate to posts and messages |

## Step 1: free written outreach (weeks 1–4)

- **Where:** public Philippine business Facebook groups (dental/clinic owners, travel & tour operators, home services, small business owners). Follow each group's rules on promotion.
- **Direct messages:** at most 15–20 personalised messages a week to business Pages that visibly quote prices or take bookings.
- **Offer:** a free 4-week pilot. "Send us your open inquiries and quotes; we draft follow-ups in your voice; you approve and send."
- **Who does what:** Claude drafts every post and message in English and Taglish; the owner approves and sends from their own account.
- **Log:** a sheet recording channel, segment, date, and outcome (no reply / reply / pilot).

## Step 2: concierge pilots (4 weeks each, 3–5 businesses)

No product code.

1. **Consent (written):** what data is shared, who sees it, that drafts go through an LLM provider under its API data terms, the deletion date, and permission to keep **anonymised** conversations for evaluation. Treat this as personal-information processing under RA 10173.
2. **Daily or every few days:** the business sends screenshots or copies of open inquiries and sent quotes, plus its price list once.
3. **Drafting:** Claude drafts each follow-up from the business's own price list and tone. The owner of this project reviews it, then sends it to the business owner.
4. **Approval:** the business owner approves, edits or rejects, and sends it themselves. This keeps a human as the sender, which also sidesteps Messenger's 24-hour and message-tag rules.
5. **Per draft, log:** approved/edited/rejected, size of the edit, reply received, booking made, and peso value.
6. **End of pilot:** offer ₱1,500 for the next month via GCash.

Pilot data doubles as the first **Taglish evaluation set**: real messages, drafts and owner edits.

## Decision thresholds

| Outcome | Criteria | Action |
|---|---|---|
| **Continue to build** | ≥ 3 pilots run **and** ≥ 70% of drafts approved **and** ≥ 1 recovered reply or booking per business per week **and** ≥ 1 business pays after its pilot | Lock vertical (the segment that converted); Phase 6 spec |
| **Pivot vertical** | Pilots work in one segment only | Narrow outreach to that segment |
| **Pivot to C (app stores)** | < 3 pilots recruited in ~4 weeks of outreach | Switch to the Xero/QuickBooks reminder agent, which reaches buyers through app stores rather than outreach |
| **Stop B** | Owners reject most drafts, **or** no recoveries, **or** nobody pays | Reassess with the scorecard; C or B2 |

These thresholds were set before any data was collected so they can't drift.

## What remains unvalidated even if B passes

- Retention beyond the first paid month.
- Support load at more than 5 customers.
- Whether Meta adds outbound follow-up to Business Agent. Check its announcements monthly.
- Whether AI-drafted, owner-approved sends through our app (rather than by the owner) are allowed for Messenger follow-up beyond 24 hours.
