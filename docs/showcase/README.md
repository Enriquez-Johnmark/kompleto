# Showcase

Screenshots and a two-and-a-half-minute captioned walkthrough of a demo Firm, Santos Bookkeeping, as of Thursday 7 January 2027: three days before the December cut-off, with 15 clients, reminders sent, uploads in, two replies, a bounced address and 10 drafts waiting. The owner, Bea, shares the work with a staff member, Carlo, and three clients have payroll chasing on.

| File | Shows |
|---|---|
| `walkthrough.mp4` | The whole loop in about 2½ minutes: dashboard, drafts (BIR and payroll), review and approve, the client uploading on the upload page, marking received, replies, the 2307 tracker and Payors from a sales export, BIR calendar, activity, staff, settings and help |
| `desktop-1-dashboard.png` | What each client still owes, nearest cut-off first |
| `desktop-2-drafts.png` | Drafts waiting for approval, in Taglish or English |
| `desktop-3-review-before-approving.png` | Reviewing several drafts in full before approving them |
| `desktop-4-client-documents-and-payors.png` | A client's documents to ask for, and its Payors |
| `desktop-5-client-uploads.png` | A client's uploads, with download and mark received |
| `desktop-6-2307-tracker.png` | Which Payors' 2307s have arrived |
| `desktop-7-bir-calendar.png` | The BIR deadlines Kompleto uses |
| `desktop-8-activity.png` | The activity log |
| `desktop-9-help.png` | The help page |
| `desktop-10-payroll-chasing.png` | A client's payroll chasing: paydays and contribution due days |
| `desktop-11-payors-from-a-sales-export.png` | Payors picked from the customers who withheld tax in a Xero sales export |
| `desktop-12-import-from-xero.png` | Importing a Xero contact list, with the language and forms chosen for it |
| `desktop-13-staff.png` | The owner and a staff member in Settings |
| `desktop-14-request-an-account.png` | The public "Request an account" page |
| `phone-1-dashboard.png`, `phone-2-drafts.png` | The same pages on a phone |
| `phone-3-client-upload-page.png` | What the client sees when they open their link |

Everything here is made-up data: the businesses, people and `.example.test` addresses don't exist.

## Making them again

They are made by Playwright scripts in `web/showcase/`, against the real app. The walkthrough approves the waiting drafts and uploads a file, so each run needs a demo Firm that hasn't been used yet.

```sh
docker compose up -d                 # Postgres and Mailpit (ClamAV optional)
pnpm --dir web build
cargo build
DEMO_DATE=2027-01-07 cargo run -- serve
# in another terminal; the address must be new to this database:
SHOWCASE_EMAIL=hello@santos.example pnpm --dir web showcase
# or run against a fresh database, with DATABASE_URL set for both commands
```

The first step makes the demo Firm (`kompleto demo-firm`) and logs in; then come the screenshots, then the video. The video is recorded with Chromium's screencast and assembled with `ffmpeg`, which must be installed.
