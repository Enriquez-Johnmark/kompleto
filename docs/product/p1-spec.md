# Kompleto (P1) spec: first version

Status: **Phase 6 complete except the AI model confirmation** (the blind Taglish test). Built up round by round through grilling; everything under "Decided" and the acceptance criteria are owner-approved, with dates. Background: `docs/product/p1-brief.md`.

## Decided

### Name (2026-10-21)

- **Kompleto** ("complete"), on **getkompleto.com**. The domain was unregistered when checked on 2026-10-21; the owner buys it before 1 Nov 2026, when the .com price rises.

### Users (2026-10-21)

- **Firm:** independent bookkeepers and small firms (1–5 staff) handling about 10–60 clients.
- **Language:** English screens; reminders to clients in Taglish or English.
- **The bookkeeper's clients** never log in; they use a private upload link.

### Scope of the first version (2026-10-21)

| In | Later |
|---|---|
| Client list, added by form or CSV/Excel upload | Payroll-input and remittance chasing |
| BIR forms each client files | Matching 2307s against invoices |
| Deadline calendar | Xero / QuickBooks import |
| Checklist of documents owed per month or quarter | SMS (link-free reminders only) |
| Reminder drafts the bookkeeper approves | React dashboard |
| Sending by email with a private upload link | In-app billing (pilots pay by GCash, by hand) |
| Phone-friendly upload page | Staff logins; public sign-up |
| Received / missing tracker for 2307s | |
| "What's still missing" dashboard | |
| Data export and account closure | |

### Accounts and login (2026-10-21)

- **No public sign-up during pilots.** We invite each bookkeeper and create their account.
- **Login by one-time email link**, so there are no passwords to store.
- **One login per firm.**
- **Details (M2 round, 2026-10-21):**
  - We create firms with a command (`kompleto firm add`); there is no sign-up page.
  - Login links work once, for 15 minutes; only a hash of the token is stored.
  - A session lasts 30 days.
  - At most 5 login emails per firm per hour. Extra requests see the same screen and get no email.

### Adding clients (2026-10-21)

- One at a time through a form, or by uploading a **CSV/Excel file** with business name, contact name, email and forms filed.
- **Details (M2 round, 2026-10-21):**
  - **Fields:** business name (unique within a firm, ignoring case), contact first name, email (may repeat across clients), reminder language (Taglish or English) and forms filed. One Contact per Client. No TIN, phone, address or amounts are stored.
  - **Files:** `.csv` (UTF-8, or Windows-1252 as a fallback) or `.xlsx` (first sheet). Columns are found by header, ignoring case: Business name, Contact first name, Email, Language, Forms. At most 500 rows and 2 MB. A template file can be downloaded.
  - **Two steps:** uploading shows a preview of valid rows and each invalid row with its reason; "Add N clients" adds every valid row in one transaction. Nothing is written before then, and an error during it undoes the whole import.

### BIR deadline calendar (2026-10-21)

- One **built-in calendar** of BIR forms and due dates, maintained by us; we apply BIR extensions when they're announced.
- The bookkeeper ticks which forms each client files and can override a date for one client.
- **Details (M2 round, 2026-10-21; sources in `docs/research/bir-calendar.md`):**
  - **Forms:** 0619-E, 1601-C, 1601-EQ, 2550Q, 2551Q, 1701Q, 1702Q, 1701/1701A, 1702 (RT/EX/MX), 1604-C and 1604-E. Fiscal-year corporations are not built in.
  - **Dates:** the manual/eBIRForms due dates. eFPS filers' staggered dates aren't built in; the bookkeeper overrides them per client.
  - **Weekends and holidays:** a due date on a Saturday, Sunday or national holiday moves to the next working day (EO 292 s.28; RMC 65-2016). Holidays come from each year's proclamation. Local holidays are ignored.
  - **One data file:** the forms' rules, the holidays and nationwide extensions live in one data file in the repo; applying an extension is one entry plus a deploy. Regional extensions are applied as per-client overrides.
  - **Per-client override:** the Due date of one form for one Period for one Client. Overrides that repeat every month come later.

### Collection periods and documents (M2 round, 2026-10-21)

- **One collection per month.** A Client is asked once a month for the month that just ended. That Period covers all of the client's forms whose period ends that month: monthly forms every month, quarterly forms in March, June, September and December, annual forms in December. A client that files only quarterly forms is asked once per quarter.
- **Cut-off** = the earliest Due date among the Period's forms, minus the cut-off offset (5 days by default, set per firm and changeable per client), but never before the day the Period opens (2026-10-21).
- **Default Owed documents** (the bookkeeper edits them per Client; edits apply to future Periods). This list is a synthesis, not an official BIR list:
  - every month: sales invoices issued (including cancelled ones); purchase and expense invoices and receipts; bank and e-wallet (GCash, Maya) statements;
  - with 1601-C: payroll register and SSS, PhilHealth and Pag-IBIG reports;
  - with 0619-E or 1601-EQ: copies of 2307s issued to suppliers, with payment vouchers;
  - at quarter end: 2307s received from customers (one item until M5 lists Payors);
  - in December, with an annual form: year-end inventory count, fixed-asset purchases and loan statements.
- Items say "invoices", not "official receipts" (RR 7-2024).
- **Starting point:** a newly added client is first chased for the first Period whose Cut-off is still ahead.

### Reminder schedule (2026-10-21)

- **First request:** when a period's documents become due.
- **Reminders:** 7, 3 and 1 day before the bookkeeper's **cut-off**. The cut-off defaults to 5 days before the BIR deadline.
- **Limit:** at most **4 messages per client per period**.
- **Stops:** when everything is uploaded, or when the client replies.
- **Control:** the bookkeeper can change the schedule per client.
- **Details (M2 round, 2026-10-21):**
  - The first-request Draft appears the day after the Period ends; reminder Drafts 7, 3 and 1 days before the Cut-off. All appear at 07:00 Manila time. Weekends don't move them.
  - A reminder that would fall on or before the first request is skipped.
  - Per client, the bookkeeper can change the cut-off offset, choose up to 3 reminder days (each 1–14 days before the Cut-off), or pause the client. The limit of 4 can't be raised. Changes affect only Drafts not yet created.
  - Until M3, Drafts are fixed templates in the client's language listing the Owed documents and the Cut-off.

### Approval (2026-10-21)

- **Every message waits for the bookkeeper's approval.** There is no automatic sending in the first version.
- Approve one at a time, or a whole batch after seeing the list.
- Unapproved drafts **expire once the deadline passes**.
- **Details (M3 round, 2026-10-21):**
  - A **Drafts page** lists every waiting Draft, nearest Cut-off first, with its recipient, full text and an "AI" or "Template" label. Each Draft can be approved, edited (which voids any earlier approval) or discarded.
  - **Batch approval:** the bookkeeper ticks Drafts (or selects all), reviews all of them in full on one page, then approves them. Each approval binds the exact text shown, so a Draft changed meanwhile isn't approved.
  - **Expiry:** an unapproved Draft expires at the end of its Period's Cut-off day (23:59 Manila time).
  - **Pre-send checks** run at approval (the approval is refused, with the reason) and again just before sending (the message is blocked). They block a message whose recipient isn't the Client's stored email, that has more than one link or any link other than the Client's own upload link, or whose body is empty or over 2,000 characters. Login emails must go to the Firm's login address with exactly one login link.
  - **Copy for Viber** is on every Draft and approved message. It copies the same body text; it doesn't approve or send anything.

### Channels and sender identity (2026-10-21)

- Upload links go **only by email from our own domain**. The sender shows the **bookkeeper's firm name** (e.g. "Santos Bookkeeping" <reminders@getkompleto.com>), replies go to the Kompleto reply address, and a small footer says "sent via Kompleto". We never send as the bookkeeper's own mailbox.
- Every reminder has a **"copy for Viber"** button; the bookkeeper pastes it into their own Viber chat.
- SMS waits until later and never carries a link (telcos block SMS links: `docs/research/p1-kill-checks.md` §5).

### Client replies (2026-10-21)

- Clients reply to the **Kompleto reply address**. Kompleto then:
  - stops further reminders to that client;
  - shows the reply on the dashboard;
  - forwards a copy to the bookkeeper's own email.
- The AI never answers clients by itself.

### 2307 tracker (2026-10-21)

- For each client, the bookkeeper lists the **Payors** (businesses that withhold tax and owe the client a 2307) expected to send 2307s each quarter.
- The client picks which Payor an uploaded 2307 is from, and the bookkeeper confirms it as received.
- The tracker shows what is still missing. **The AI does not read documents** in the first version.

### Uploaded files (2026-10-21)

- Stored **encrypted**; the bookkeeper downloads them from the dashboard.
- **Deleted automatically 30 days after being marked received, and never kept longer than 90 days.** Kompleto is not a records archive (BIR and privacy limits: `docs/research/p1-kill-checks.md` §2, §4).

### Leaving (2026-10-21)

- A firm can **download all its data**: client list, status history, and any files still held.
- Closing the account **deletes everything within 30 days**, except what the law requires us to keep (our own billing records).

### AI (2026-10-21)

- **Claude Haiku 5.5** (`claude-haiku-5-5`, thinking disabled, JSON-schema output), at about ₱3.50 per firm per month. **Sonnet 5.5** is the fallback if Haiku's Taglish isn't good enough. Prices: `docs/research/llm-options.md`.
- The choice is confirmed by a **blind Taglish test** before Phase 6 closes (`prototypes/taglish-test/`).
- Paid API with no training on our data, behind our own small client so the provider can be swapped.
- If the AI is unavailable, Kompleto **falls back to fixed message templates**, so reminders still go out.
- **Details (M3 round, 2026-10-21):**
  - The API key is optional: without `ANTHROPIC_API_KEY` every Draft is a template. The blind Taglish test runs once Anthropic credits are bought.
  - Thinking off, JSON-schema output, a 15-second timeout and no retry; any failure gives a template Draft at once. The model is a configuration value.
  - **The model writes only the wording:** a subject, an opening and a closing in the Client's language. Kompleto inserts the exact Owed documents, the Cut-off date, the firm's name and (from M4) the upload link, so the model can't garble or invent facts and never sees the firm name or a link.
  - **Tone:** first request friendly; 7 days before the Cut-off a friendly reminder; 3 days firmer; 1 day urgent but polite.
  - **Invalid output** (gives a template instead): anything but the exact JSON shape; a subject over 120 characters or more than one line; an opening or closing over 600 characters; any link, email address, `₱`/`PHP` amount or run of 9 or more digits.
  - **Costs:** every call is recorded with firm, time, model, tokens, US$ cost and Anthropic's request ID. Settings shows the month's AI cost; `kompleto llm-costs --month YYYY-MM` lists every Firm's.
  - Drafting runs inside the hourly scheduler, one call per Draft due.

### What the AI sees (2026-10-21)

- **Only:**
  - the client's business name and contact's first name;
  - the list of missing items;
  - the deadline;
  - the language and tone.
- **Never:** document contents, files, TINs or amounts. Tax documents never go to the AI provider.

### Web technology (2026-10-21)

- **Server-rendered pages from the Rust backend**; no React in the first version.

### Pilots and launch (2026-10-21)

- **Price:** ₱1,500 per firm per month, after a free 4-week pilot (go-to-market decision, `docs/decisions.md`).
- **Pilot success:** a pilot round with 3–5 firms succeeds only if all four hold:
  - ≥ 70% of drafts are approved as-is or with small edits;
  - ≥ 50% of requested documents arrive before the cut-off;
  - each firm uses Kompleto at least once a week;
  - ≥ 1 firm pays ₱1,500 for the next month.

  A miss means reassessing, with S1 as the fallback. These replace the old thresholds written for B.
- **Registration timing:** pilots run without a registered business.
  - **Before the first pilot:** the privacy paperwork (privacy notice, data processing agreement, owner as data protection officer, NPC sworn declaration). Claude drafts it.
  - **In pilot week 3:** register the DTI name (₱530) and register with the BIR online (₱30, 8% option); ask the barangay and city hall whether a home-based online service needs a permit.
  - **First payment:** by personal GCash, with a BIR invoice issued.

  Costs and sources: `docs/research/p1-launch-prerequisites.md`.

## Hard limits (from research; binding)

- Never keep books, compute tax, file returns, export SAWT or other BIR-format files, or record or issue transactions or 2307s.
- Never act as the client's records archive (see retention above).
- Never put a link in an SMS.
- No Firm or Client data goes to free LLM tiers that train on inputs.

## Acceptance criteria (owner-approved 2026-10-21)

Kompleto's first version is done when every criterion below passes. Each criterion is checkable by an automated test or a scripted manual check. "Firm" means a bookkeeper's account; "client" means one of the firm's clients.

**Accounts and isolation**
- **A1.** An invited bookkeeper gets a login link that works once and expires after 15 minutes. An address with no account gets the same "check your email" screen and no email, so the screen never reveals who has an account.
- **A2.** One firm can never see or change another firm's clients, messages or files. Requesting any of them with another firm's IDs returns "not found".

**Clients and calendar**
- **B1.** A client can be added by form with business name, contact first name, email, reminder language and forms filed.
- **B2.** A 60-row CSV/Excel import adds every valid row and lists each invalid row with its reason. A failed import changes nothing.
- **B3.** The built-in calendar covers the BIR forms in scope for the next 12 months, and a BIR extension is applied in one place. A per-client date override changes only that client's dates.
- **B4.** For each client and period, Kompleto lists the documents owed and the cut-off date.

**Drafts and approval**
- **D1.** Drafts appear on the decided schedule (first request; 7, 3 and 1 day before cut-off), and never more than 4 per client per period.
- **D2.** Nothing leaves without approval. A message can only be sent from an approved draft whose content matches exactly what was approved; any edit needs a new approval.
- **D3.** Batch approval shows the full list of drafts before anything is approved.
- **D4.** Drafts not approved by the deadline expire and can no longer be sent.
- **D5.** If the AI call fails or returns anything other than valid output, a template draft is created instead and labelled as a template.
- **D6.** The AI request contains only the allowed fields. A test inspects every outgoing request for TIN-shaped numbers, peso amounts or file contents.
- **D7.** A pre-send check blocks a message if:
  - the recipient isn't the client's stored email;
  - it has a link that isn't the client's own upload link;
  - it has more than one link;
  - its body is empty or over the length limit.

**Sending**
- **E1.** An approved email is sent at most once, even if the server crashes mid-send. Tested by killing the process between sending and recording; after restart, nothing is sent twice.
- **E2.** Emails show the firm's name as sender, use the Kompleto reply address and carry the "sent via Kompleto" footer, and pass SPF, DKIM and DMARC.
- **E3.** "Copy for Viber" gives the same text, with the upload link, ready to paste.

**Upload page**
- **F1.** The upload link opens on a phone with no login. It accepts photos and PDFs up to 10 MB each, and the client labels each file. For a 2307, the label includes which Payor it's from.
- **F2.** Each link is unguessable, works only for one client and one period, and stops working 30 days after that period's cut-off.
- **F3.** Files are stored encrypted, deleted 30 days after being marked received, and never kept beyond 90 days. Tested by advancing the clock.

**Replies, bounces and stopping**
- **G1.** A human reply to the reply address (not an automatic out-of-office reply) stops further reminders to that client for the period. It appears on the dashboard and is forwarded to the bookkeeper within 5 minutes.
- **G2.** Once everything owed is uploaded, reminders for that period stop.
- **G3.** A bounce stops reminders to that address and shows on the dashboard. Only bounces that match a message we actually sent count.

**Dashboard and data**
- **H1.** The dashboard lists every client with its missing items, next cut-off and last message, nearest cut-off first.
- **H2.** The 2307 tracker shows, per client and quarter, each expected Payor as received or missing.
- **I1.** A firm can export its data (CSV plus a zip of the files still held).
- **I2.** Closing an account deletes all of the firm's data within 30 days.

**Operations**
- **J1.** Every AI call is recorded with model, tokens, cost and request ID; we can see LLM cost per firm per month.
- **J2.** Every approval, edit, send, reply and deletion is recorded in an audit log with time and actor.

## Open

1. Blind Taglish test result: confirms Haiku 5.5, or switches to Sonnet 5.5. It doesn't affect the architecture, since the model is a configuration value behind our own client.
