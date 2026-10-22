# Kompleto (P1) spec: first version

Status: **Phase 6 complete except the AI model confirmation** (the blind Taglish test). Built up round by round through grilling; everything under "Decided" and the acceptance criteria are owner-approved, with dates. Background: `docs/product/p1-brief.md`.

## Decided

### Name (2026-10-21)

- **Kompleto** ("complete"), on **getkompleto.com**. The domain was unregistered when checked on 2026-10-21. It is bought just before M6 (hosting and real email), not before 1 Nov 2026 as first planned: the budget is ₱0 until pilots (after-M4 round, `docs/decisions.md`).

### Users (2026-10-21)

- **Firm:** independent bookkeepers and small firms (1–5 staff) handling about 10–60 clients.
- **Language:** English screens; reminders to clients in Taglish or English.
- **The bookkeeper's clients** never log in; they use a private upload link.

### Scope of the first version (2026-10-21)

| In | Later |
|---|---|
| Client list, added by form or CSV/Excel upload | ~~Payroll-input and remittance chasing~~ (built at M13) |
| BIR forms each client files | ~~Matching 2307s against invoices~~ (M12: Payors from a sales export, names only) |
| Deadline calendar | ~~Xero / QuickBooks import~~ (M11, from export files) |
| Checklist of documents owed per month or quarter | SMS (link-free reminders only) |
| Reminder drafts the bookkeeper approves | |
| Sending by email with a private upload link | In-app billing (pilots pay by GCash, by hand) |
| Phone-friendly upload page | ~~Staff logins; public sign-up~~ (M9, M10) |
| Received / missing tracker for 2307s | |
| "What's still missing" dashboard | |
| Data export and account closure | |

### Accounts and login (2026-10-21)

- ~~**No public sign-up during pilots.** We invite each bookkeeper and create their account.~~ Superseded 2026-10-22: a public "Request an account" page, opened only after the operator approves (M10).
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
- **Details (M5 round, 2026-10-21):**
  - **Receiving:** the app takes raw inbound email at a secured endpoint. Which provider delivers it (Cloudflare Email Routing or Resend's inbound email) is chosen at M6 with the domain (`docs/research/inbound-email.md`).
  - **Matching:** each Reminder's reply address carries a signed code (`reply+CODE@…`) naming its Client and Period. Mail without a valid code matches only when exactly one Client has the sender's address, and then counts against that Client's latest Reminder. Unmatched mail is logged by time and sender domain only; its content isn't kept.
  - **Auto-replies:** a message is automatic if `Auto-Submitted` is anything but `no`, it has `X-Autoreply` or `X-Autorespond`, `Precedence` is bulk, list, junk or auto_reply, it has `List-Id`, or its subject starts with "Automatic reply", "Out of Office" or "Auto:". When unsure it counts as automatic: a missed Reply only means Reminders continue.
  - **A human Reply** to the signed reply address cancels the Client's waiting Reminders for that Period (as G2 does) and shows on the dashboard and the Client page. One matched only by the sender's address, which anyone can forge, is shown and announced, but Reminders continue and its attachments aren't kept (owner, 2026-10-21, after the M5 code review).
  - **Notifying the Firm** (refines "forwards a copy"): within 5 minutes the Firm's login address gets "Nena (Nena's Store) replied about October 2026" with one link to read the Reply in Kompleto after logging in. The Contact's text is never copied into email, so our domain never relays content that could be phishing.
  - **Attachments:** photo and PDF attachments are kept as Uploads labelled "Sent by email", under the upload rules (types, sizes, per-link limits, deletion). Anything else is dropped.
  - **Retention:** Reply text is deleted after 90 days; only "replied on <date>" remains.

### Bounces (M5 round, 2026-10-21)

- Resend reports Bounces and spam complaints to a signed webhook. Only events matching a message we sent count (G3).
- A permanent Bounce or a complaint stops all Reminders to that Client's address, for every Period, until the Firm changes the Client's email. Temporary Bounces only show on the dashboard.

### Dashboard (M5 round, 2026-10-21)

- The home page is the dashboard (H1): one row per Client, nearest Cut-off first, with what's still missing, the next Cut-off, the last message and its status, and badges for replied, auto-reply, bounced and Drafts waiting. Paused Clients come last.

### 2307 tracker (2026-10-21)

- For each client, the bookkeeper lists the **Payors** (businesses that withhold tax and owe the client a 2307) expected to send 2307s each quarter.
- The client picks which Payor an uploaded 2307 is from, and the bookkeeper confirms it as received.
- The tracker shows what is still missing. **The AI does not read documents** in the first version.
- **Details (M5 round, 2026-10-21):**
  - The Payor list is names only, never TINs. On the upload page a 2307 is labelled by picking a listed Payor, or "Other" with a typed name.
  - For a Client with Payors, the general "2307s received from customers" Owed document becomes one per missing Payor ("BIR Form 2307 from Jollibee Foods"), so Reminders and the upload page ask for exactly those.
  - A tracker page per quarter shows every Client's Payors as received (the Firm marked it received), uploaded (waiting to be checked) or missing. A 2307 from an unlisted Payor shows as unexpected, with "add to list".

### Uploaded files (2026-10-21)

- Stored **encrypted**; the bookkeeper downloads them from the dashboard.
- **Deleted automatically 30 days after being marked received, and never kept longer than 90 days.** Kompleto is not a records archive (BIR and privacy limits: `docs/research/p1-kill-checks.md` §2, §4).
- **Details (M4 round, 2026-10-21):**
  - **Upload links:** one per Client per Period, created with that Period's first Draft and used in all its Reminders. The link is a record ID plus a 128-bit HMAC signature made with a server-only key, so the database holds no usable link. It works until 30 days after the Cut-off; after that the page says it has expired and to ask the Firm for a new one.
  - **In messages:** Kompleto inserts the link into every Reminder and the "copy for Viber" text; the model never sees it. The pre-send check requires exactly one link, and it must be this Client's link for this Period.
  - **The page:** the firm name, the Period and the Owed documents, each shown as uploaded or not yet; fixed text in the Client's reminder language, never written by the model. The Contact can't view, download or delete Uploads.
  - **Labels:** each Upload is one Owed document, or "Something else" with a short note. A 2307 also carries the Payor's name, typed by the Contact until M5 adds the Firm's Payor list.
  - **Files:** JPEG, PNG, HEIC/HEIF, WebP and PDF, up to 10 MB each, recognised by their contents rather than their names, and never opened, resized or converted. One file per request (a few lines of JavaScript send several in turn); at most 4 uploads are processed at once; at most 60 files and 200 MB per link. Pages carry `noindex` and a `same-origin` referrer policy, so the link never reaches another site (`no-referrer` would make browsers send `Origin: null` with the page's own uploads, which the same-origin check refuses; found in the M4 code review).
  - **Storage:** each file has its own key, wrapped by the master key (`FILE_MASTER_KEY`; `kompleto gen-key` makes one) and bound to the file and its Firm. Files sit on local disk behind the engine's file store until R2 replaces it at M6; they are never stored in Postgres, so database backups never hold them.
  - **Received and deletion:** the Client page lists each Upload with Download and Mark received. An hourly sweep deletes the file and its key 30 days after it is marked received, or 90 days after upload, whichever is first, and records each deletion in the audit log. Downloads are attachments with a generated name, never the Contact's own file name.
  - **Stopping (G2, brought into M4):** later Reminders list only the Owed documents not yet uploaded; when none are left, no more Reminders are drafted for that Period and any waiting Draft is cancelled.
  - **No virus scanning** in M4: nothing fits the budget. Files are only ever downloaded as attachments, never shown in the browser. ~~Revisit before paid pilots.~~ Superseded 2026-10-22: Uploads are scanned with ClamAV (free) at M8 (showcase round, `docs/decisions.md`). Built at M8: each Upload and reply attachment is scanned before it is kept; an infected file is refused, and while ClamAV can't answer every file is refused, never kept unscanned. The server won't start in production without ClamAV, and `/health` fails while it's down.

### Leaving (2026-10-21)

- A firm can **download all its data**: client list, status history, and any files still held.
- Closing the account **deletes everything within 30 days**, except what the law requires us to keep (our own billing records).

### AI (2026-10-21)

- **Claude Haiku 5.5** (`claude-haiku-5-5`, thinking disabled, JSON-schema output), at about ₱3.50 per firm per month. **Sonnet 5.5** is the fallback if Haiku's Taglish isn't good enough. Prices: `docs/research/llm-options.md`.
- The choice is confirmed by a **blind Taglish test** before Phase 6 closes (`prototypes/taglish-test/`).
- Paid API with no training on our data, behind our own small client so the provider can be swapped.
- If the AI is unavailable, Kompleto **falls back to fixed message templates**, so reminders still go out.
- **Details (M3 round, 2026-10-21):**
  - The API key is optional: without `ANTHROPIC_API_KEY` every Draft is a template. The blind Taglish test runs by hand in claude.ai with fictional clients (after-M4 round); the app itself only ever uses an API key.
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

- ~~Server-rendered pages from the Rust backend; no React in the first version.~~ Superseded 2026-10-22 (showcase round, `docs/decisions.md`).
- **The bookkeeper's pages are a React app** (Vite, shadcn/ui) served by the Rust binary through a JSON API. The Client's upload page stays server-rendered (`docs/adr/0005-react-app-served-by-the-rust-binary.md`, `docs/frontend.md`).

### Pilots and launch (2026-10-21)

- **Price:** ₱1,500 per firm per month, after a free 4-week pilot (go-to-market decision, `docs/decisions.md`).
- **Pilot success:** a pilot round with 3–5 firms succeeds only if all four hold:
  - ≥ 70% of drafts are approved as-is or with small edits;
  - ≥ 50% of requested documents arrive before the cut-off;
  - each firm uses Kompleto at least once a week;
  - ≥ 1 firm pays ₱1,500 for the next month.

  A miss means reassessing, with S1 as the fallback. These replace the old thresholds written for B.
- **Registration timing:** pilots run without a registered business.
  - **Before the first pilot:** the privacy paperwork (privacy notice, data processing agreement, owner as data protection officer, NPC sworn declaration). Claude drafts it. Drafted at M8: the privacy notice, terms and data processing agreement are public pages (`docs/legal.md`); the owner fills in the bracketed details, designates themself DPO and files the sworn declaration.
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

### Staff logins (M9, owner-approved 2026-10-22)

- **K1.** A Firm has one owner and at most 5 staff. The owner invites staff by email; each person logs in with their own one-time link and works on the same Firm's data. An address belongs to one Firm only. Removing a member ends their sessions and links at once.
- **K2.** Staff can do the daily work (Clients, Drafts, approving, Uploads, Replies). Only the owner can export the Firm's data, close the account, or invite and remove staff; the API refuses staff.
- **K3.** The activity log names the person who did each thing.

### Import from Xero and QuickBooks (M11, owner-approved 2026-10-22)

- **B5.** A Xero contact export or a QuickBooks Online customer list (CSV or Excel) imports like Kompleto's own template: business name, contact first name and email come from their columns, the language and BIR forms chosen for the batch fill the rows that have none, every row is previewed with its problems before anything is added, and no other column (a TIN, for instance) is read.

  Evidence for the column names (accessed 2026-10-22): Xero contact files use `ContactName`, `EmailAddress`, `FirstName` and `LastName` (third-party docs: [Moment](https://docs.moment.team/help/moment-by-topic/integrations/file-exports/file-export-accounting-system/xero), [Zynk](https://docs.zynk.com/workflow/documentation/xero/export-contacts.html); confidence M); QuickBooks Online's customer export has `Name` or `Customer` (it differs by region), `Company` and `Email Address` ([Intuit help](https://quickbooks.intuit.com/learn-support/en-us/help-articles/export-customer-data-to-excel/00/262148), vendor docs; confidence M). Xero's official header list: not found. The reader accepts all these spellings, ignoring case, spaces and Xero's leading `*`.

### Payors from a sales export (M12, owner-approved 2026-10-22)

- **H3.** The bookkeeper uploads one quarter's sales, invoice or account-transactions export (CSV or Excel) for a Client. Kompleto keeps the names of customers on rows that show tax withheld: a withholding column with a non-zero value, or an account, description or tax-type cell naming withholding, CWT, EWT or 2307. The bookkeeper sees the names, with those already listed marked, and adds the ones they tick as Payors. Only names are kept: amounts are never parsed into numbers or stored, no other column (a TIN, for instance) is read, and nothing is matched against 2307 files.

  Evidence for the columns (accessed 2026-10-22): how Philippine Xero and QuickBooks Online users record creditable withholding tax is not found in either vendor's documentation. For comparison, Odoo's Philippine localisation records it as withholding tax lines on invoices and bills ([Odoo docs](https://www.odoo.com/documentation/18.0/applications/finance/fiscal_localizations/philippines.html), vendor docs). Confidence in the column names below: L. Common practice is a CWT receivable account or a negative "Less: CWT" invoice line, so the reader accepts either and the preview lets the bookkeeper untick a wrong name. A title block above the header row, as QuickBooks reports have, is skipped.

### Payroll chasing (M13, owner-approved 2026-10-22)

Kompleto chases the documents payroll needs and the proof that contributions were paid. It never computes pay, contributions or tax.

- **P1.** A Client's payroll chasing is off, twice a month (paydays on the 15th and the last day) or monthly (the last day). When it's on, Kompleto asks before each payday for timesheets or DTRs, overtime and leave approvals, and new hires and resignations. Each payday is its own Period: the first request comes 8 days before payday, the Cut-off is 3 days before it, and the request goes through the usual Draft → approve → send → upload loop, stopping on a Reply or once everything is uploaded.
- **P2.** For a Client with payroll chasing on, Kompleto asks for proof that each month's SSS, PhilHealth and Pag-IBIG contributions were paid, each only after its own due date. Each due date is its own Period, shared by the agencies due that day, so a Reply to one request never stops the request for a receipt due later. The due dates:
  - SSS: the last day of the following month, moved to the next working day;
  - PhilHealth and Pag-IBIG: a day of the following month the bookkeeper sets per Client, defaulting to the 20th and the last day (moved to the next working day).

  A Period opens the day after its due date, and its Cut-off is 7 days after it.

  Evidence (accessed 2026-10-22; official sites were read through search extracts only, as direct fetches failed):
  - **SSS** pays by the last day of the month after the applicable month, and the deadline moves to the next working day ([SSS, Pay contribution](https://www.sss.gov.ph/pay-contribution/); Circular 2019-012; official; confidence H).
  - **PhilHealth** sources conflict. The Employer's Quick Guide says the 10th of the following month. The [payment procedures page](https://www.philhealth.gov.ph/partners/employers/pay_procedures.php) and Circular 01 s.2014 say the 11th–15th or 16th–20th, by the last digit of the PEN. Household employers pay by the 25th (Circular 016-2015). All official; confidence M.
  - **Pag-IBIG** dates are staggered by the employer name's first letter, from the 10th–14th up to the 25th to the month's end (HDMF Circular 275). This comes from secondary sources only; on pagibigfund.gov.ph it is not found. Confidence M.

  Because the bookkeeper's view of each Client's actual date beats any default, both stay settings. The defaults take the latest plausible date, since asking a few days late for a receipt is harmless.

### Sign-up (M10, owner-approved 2026-10-22)

- **L1.** Anyone can request an account with a firm name and an email, accepting the terms and the data processing agreement. A request creates no Firm and sends no email; the operator approves or declines it from the command line, and approving creates the Firm and emails its owner a welcome login link. The answer to a request is the same whether or not the address already has an account, and decided requests are deleted.

## Open

1. Blind Taglish test result: confirms Haiku 5.5, or switches to Sonnet 5.5. It doesn't affect the architecture, since the model is a configuration value behind our own client.
