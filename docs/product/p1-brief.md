# Kompleto (P1) brief: bookkeeper document & deadline chaser

Status: **owner-approved as the first product (2026-10-21); named Kompleto, on getkompleto.com.** This brief is the plain-language picture. The detailed spec and acceptance criteria are Phase 6 work and go through grilling; until then, anything below marked *hypothesis* or *open* is not decided.

## Who it's for

- **Buyer:** independent bookkeepers and small accounting firms in the Philippines. One firm handles roughly 10–60 small-business clients and bills each ₱2,500–15,000 a month (firm blogs, 2025; `docs/research/ph-industries-services.md` [A11][A12]).
- **Their clients** (sari-sari stores, clinics, freelancers, small shops) never log in or install anything. They get a message and a private upload link.

## The problem

Every month and quarter, the bookkeeper chases each client for receipts, GCash screenshots, bank statements and the BIR Form 2307s their customers owe them. They do it by hand, in Viber threads and one Excel file per client, often the night before a deadline (vendor descriptions, `docs/research/ph-industries-services.md` [A13]). The cost of a miss:
- a missing 2307 loses the client a tax credit;
- a late return costs at least a ₱1,000 compromise penalty plus a 25% surcharge.

## What P1 does

```
 bookkeeper adds clients, the BIR forms each files, what documents each owes
        │
        ▼
 P1 sees what's missing and which deadline is near
        │
        ▼
 drafts a reminder (Taglish or English) ──► bookkeeper approves / edits
        │
        ▼
 sends by email with a private upload link   (or the bookkeeper sends it on Viber)
        │
        ▼
 client uploads photos/PDFs from their phone ──► P1 marks them received
        │
        ▼
 reply or upload arrives ──► follow-ups stop; bookkeeper sees what's still missing
```

**What we sell, because no tool found does all three together** (`docs/research/p1-kill-checks.md` §1):
1. a BIR deadline calendar per client;
2. a tracker showing which 2307s each client has received and which are still missing;
3. reminders drafted in Taglish or English that the bookkeeper approves before they go out.

The chasing and the upload page on their own are common features. Built for Bookkeepers (₱999/mo) already has a client upload page, and global tools such as Karbon, TaxDome and DocChase send reminders.

## Hard limits

These keep P1 out of BIR accounting-system rules and limit privacy exposure (`docs/research/p1-kill-checks.md` §2, §4–5).

- P1 **never keeps books, computes tax, files returns, exports SAWT or other BIR-format files, or records or issues transactions or 2307s.** The later 2307 module stops at matching what was received.
- P1 is **not the client's records archive.** Uploaded files are kept only until the bookkeeper takes them, under a set retention period.
- **Upload links go by email or the bookkeeper's own Viber, never SMS.** Telcos block links in SMS (NTC 2023), so SMS can only be a link-free nudge.
- P1 handles sensitive personal information (tax returns, likely TINs) as the bookkeeper's processor. Before paid launch we need terms, a data processing addendum listing sub-processors (LLM, email, hosting), retention and deletion rules, and a plan to register with the National Privacy Commission (NPC) once we hold sensitive data on 1,000 people.

## Price (hypothesis)

Per firm, about ₱1,000–1,500 a month, against the ₱999 Built for Bookkeepers anchor (`docs/research/p1-kill-checks.md`). The approved go-to-market is a free 4-week pilot, then ₱1,500 via GCash (`docs/decisions.md`). Possibly positioned as a companion to Built for Bookkeepers rather than a rival (*open*).

## Engine vs P1

The engine does the parts every product needs:
- schedule what's due;
- draft with the LLM;
- run pre-send checks;
- hold sends in an approval outbox;
- send by email;
- read replies and stop follow-ups;
- record every send and every model call.

P1 adds the bookkeeping parts: clients, BIR forms and deadlines, the document checklist, the 2307 tracker, the upload page and the bookkeeper dashboard. The second product (S1, or a P1 add-on such as payroll-input chasing) reuses the engine.

## Later add-ons

- Payroll-input and SSS/PhilHealth/Pag-IBIG remittance chasing for bookkeepers who run client payroll (`docs/research/ai-addons-hr.md`, HR2).
- Received-2307 matching against invoices (`docs/research/ai-addons-erp.md`, ERP1).
- Read-only import from Xero, QuickBooks or Odoo.
- A React dashboard.

## Spec

The first version's decisions and open questions are in [p1-spec.md](p1-spec.md).
