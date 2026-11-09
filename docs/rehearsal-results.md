# Go-live rehearsal: results

Run on 2026-11-09 in a Claude Code cloud session instead of the owner's Mac (the owner can't run it locally and the domain isn't bought yet). It follows [`deploy-rehearsal.md`](deploy-rehearsal.md) as far as a cloud machine allows: the **release build**, installed the way [`deploy.md`](deploy.md) installs it, with Postgres 17, a **real ClamAV** with live signatures, **Claude wording the drafts** through the owner's API key, and email caught by Mailpit instead of sent through Resend.

**Result: everything that can run without the domain and a Resend account passed.** What is left is listed at the end; none of it is code.

## Build and start

| Measure | Result |
|---|---|
| Release build (`cargo build --release`, 4 cores, cold) | 3 min 22 s |
| Program size | 27 MB |
| Web app build | 35 s; 4 MB |
| Start-up to healthy, migrations included | 139 ms |
| `/health` | `ok`: app, database and ClamAV all answer |

## The bookkeeper's whole loop, in a real browser

13 of 13 browser journeys passed against the release build (`pnpm e2e`, `KOMPLETO_BIN` set to it): login, Clients by form and by import, Xero files, Payors, drafts reviewed in full, payroll chasing, the upload page, staff, the landing page and sign-up.

Then a rehearsal journey covering what those don't:

| Step | Result |
|---|---|
| The operator adds a Firm from the command line | pass |
| Its login email arrives; the owner logs in with the link | pass |
| The owner adds a Taglish Client filing 1601-C | pass |
| The server restarts; the session survives | pass |
| The scheduler drafts the Reminder, **worded by Claude Haiku** (labelled "AI" on the Drafts page) | pass |
| The owner reviews it in full and approves it | pass |
| The Reminder is sent **once**, in Taglish, signed by the Firm, with a signed reply address | pass |
| The Client opens the link on a phone and uploads a photo; it is scanned by ClamAV and accepted ("✓ Na-upload na") | pass |
| The bookkeeper downloads it: the same 74,306 bytes, decrypted | pass |
| Someone asks for an account; the operator approves it; the welcome email is sent | pass |
| The activity log shows the logins, approvals and new Clients | pass |
| Closing the account: the old session is refused (401), no login link is sent, the upload link is dead (404) | pass |
| 8 days later the Firm is erased: account, Client, stored file (and its encrypted file on disk), emails and audit entries all gone | pass |

The virus test file (EICAR) is found by the real ClamAV and refused (`f1_a_real_clamd_finds_the_eicar_test_file_and_passes_clean_files`, the one test normally skipped, passed against it).

The reminder Claude wrote, as the Client received it:

> **Mga dokumento para sa November 2026 BIR filings - Aling Nena's Sari-Sari Store**

## Backups

| Measure | Result |
|---|---|
| `backup.sh` on the rehearsal database with one upload | 0.36 s; 95 KB, encrypted (no plain text inside) |
| `restore.sh` into an empty database and folder | pass; no restored email can send twice |
| A second server on the **restored** copy returns the Client's photo, byte for byte, to the bookkeeper's old session | pass |
| Round trip on a busier database (2 Firms, 8 Clients, 38 audit entries, 10 emails) | 2.9 s; every count matches |

## Speed

The release build, logged in as the demo Firm (15 Clients, 45 days of history, 96 Reminders sent, 62 uploads), 400 requests per page, 10 at a time. No request failed.

| Page | Requests a second | Median | 95th percentile |
|---|---|---|---|
| `/health` | 1,656 | 5.1 ms | 12.1 ms |
| Dashboard | 106 | 92.1 ms | 117.9 ms |
| Clients | 1,995 | 4.7 ms | 7.8 ms |
| One Client | 678 | 14.6 ms | 19.8 ms |
| Drafts | 1,633 | 5.4 ms | 10.3 ms |
| 2307 tracker | 1,901 | 5.1 ms | 7.8 ms |
| Activity | 1,167 | 8.3 ms | 12.6 ms |
| The app's page (`/`) | 246 | 44.0 ms | 48.0 ms |

Memory under that load: Kompleto 33 MB, Postgres 118 MB, ClamAV about 1 GB. The whole stack fits easily in Oracle's free 12 GB machine or on a Mac. A pilot of 3–5 Firms is far below these numbers.

## The AI model (the Taglish test)

Haiku and Sonnet each wrote the 10 test cases with Kompleto's exact request. A blind judge (Claude Opus, told to judge as a Manila bookkeeper) compared each pair twice with the order swapped, so a verdict that flipped counts as a tie: **Sonnet 8, Haiku 1, tie 1**, with 9 of 10 verdicts the same in both orders. The judge's reasons: Haiku's openings were sometimes awkward ("Hi Nena po!") and its Tagalog stiffer; Sonnet read like a real bookkeeper and matched each reminder's stage. Cost per draft: Haiku US$0.0002, Sonnet US$0.003, under US$1 a month for a 5-Firm pilot either way. Details: `prototypes/taglish-test/` (`judge`). The model is the owner's call (`LLM_MODEL`).

## What the rehearsal could not cover, and why

| Not covered | Why | When it's checked |
|---|---|---|
| Real email through Resend, and SPF, DKIM and DMARC passing | Needs a Resend account (free, no card) and, for the Firm's own address, the domain | `deploy.md` step 9 |
| Replies reaching Kompleto by email | Needs the domain's Email Routing and the Worker | `deploy.md` step 9 |
| The public address through the Cloudflare tunnel | Needs the domain (a named tunnel) | Go-live day |
| The launchd or systemd units starting it at boot | No Mac or systemd in a cloud container | Go-live day |
| Backups to Google Drive | Needs the owner's Google account; the same scripts ran with a local folder in its place | Go-live day |

Two things that look like problems here aren't, on a real server: ClamAV needed this sandbox's proxy certificate to fetch its signatures, and the host's `pg_dump` was an older version than the database (`deploy.md` installs Postgres 17's own tools).
