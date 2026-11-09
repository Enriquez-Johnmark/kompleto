# Rehearsing the go-live

Status: written 2026-11-09, not yet followed. A dry run of [`deploy.md`](deploy.md) on the owner's Mac (path B) **before any domain is bought and before any bookkeeper says yes**, so the first real set-up takes an hour, not days. It costs ₱0 and needs no card.

What is different from the real thing:

| Real (`deploy.md`) | Rehearsal |
|---|---|
| `https://app.getkompleto.com` through a named tunnel | A temporary `https://<random>.trycloudflare.com` address (`cloudflared tunnel --url`), new each time it starts |
| Resend sending from `reminders@getkompleto.com` | Resend's test sender, `onboarding@resend.dev`, which delivers only to the address the Resend account was made with |
| Replies through Cloudflare Email Routing and the Worker | **Not rehearsed**: it needs the domain. Replies to a rehearsal email are lost; reply handling is tested in development with Mailpit, and again in `deploy.md` step 9 |
| Real Clients | Only a test Firm whose Clients are your own inbox. Nothing real goes in. |

Tick each box as you go, and write down anything that didn't match: a changed vendor screen, a missing step, a confusing error. That list is the point of the rehearsal; send it to me and I'll fix `deploy.md`.

## 1. Secrets (`deploy.md` step 1)

- [ ] Make the four secrets and save them in your password manager. They are rehearsal secrets: make new ones on go-live day.

## 2. Tools on the Mac (`deploy.md` path B, step 1)

- [ ] `brew install postgresql@17 cloudflared rclone gnupg node@22`, then `corepack enable`
- [ ] `brew services start postgresql@17`
- [ ] Create the database **`kompleto_rehearsal`** (not `kompleto`), owned by the `kompleto` user with the rehearsal Postgres password
- [ ] Docker Desktop running, then ClamAV: `docker run -d --name clamav --restart unless-stopped -p 127.0.0.1:3310:3310 clamav/clamav:1.4.3`. Wait for it to load its signatures (a few minutes) before step 6.

## 3. Build (`deploy.md` path B, step 2)

- [ ] `cargo build --release`
- [ ] `pnpm --dir web install --frozen-lockfile && pnpm --dir web build`
- [ ] Copy `target/release/kompleto`, `deploy/backup.sh` and `deploy/mac/run.sh` into `~/kompleto/`, and `web/dist` to `~/kompleto/web`
- [ ] Note how long the build took on the Mac: ______

## 4. The temporary address

- [ ] `cloudflared tunnel --url http://127.0.0.1:3000` in its own terminal. It prints an `https://….trycloudflare.com` address: that is `PUBLIC_BASE_URL` below. Leave it running; if it stops, the address changes and Kompleto must restart with the new one.

## 5. Sending: Resend's test sender (`deploy.md` step 3, without the domain)

- [ ] Create a Resend account **with the inbox you'll use as the test Client**
- [ ] **API Keys → Create**, sending access only → `RESEND_API_KEY`
- [ ] **Webhooks → Add endpoint**: `<PUBLIC_BASE_URL>/hooks/resend`, events `email.bounced` and `email.complained` → `RESEND_WEBHOOK_SECRET`. Update it whenever the temporary address changes.

## 6. Settings and start (`deploy.md` path B, steps 2–3)

Write `~/kompleto/kompleto.env` (`chmod 600`):

```
DATABASE_URL=postgres://kompleto:<rehearsal postgres password>@localhost:5432/kompleto_rehearsal
BIND_ADDR=127.0.0.1:3000
PUBLIC_BASE_URL=<the trycloudflare address from step 4>
MAIL_PROVIDER=resend
RESEND_API_KEY=<from step 5>
RESEND_WEBHOOK_SECRET=<from step 5>
SEND_BUDGET=60
MAIL_FROM_ADDRESS=onboarding@resend.dev
INBOUND_SECRET=<rehearsal secret>
FILE_MASTER_KEY=<rehearsal secret>
UPLOAD_DIR=~/kompleto/uploads-rehearsal
WEB_DIR=~/kompleto/web
SUPPORT_EMAIL=<your own inbox>
CLAMD_ADDR=127.0.0.1:3310
ANTHROPIC_API_KEY=<your key>                  # Claude words the drafts; without it, templates
```

- [ ] Install the launchd files (`deploy/mac/*.plist` → `~/Library/LaunchAgents/`, `launchctl load` each)
- [ ] `https://<address>/health` answers `ok` in a browser on your phone (app, database and ClamAV all up)
- [ ] Restart the Mac. Kompleto comes back by itself, and `/health` says `ok` again once the tunnel from step 4 is running (rehearsal only: the real tunnel in `deploy.md` step 6 is a service)

## 7. The loop, end to end (`deploy.md` step 9, without replies)

- [ ] `kompleto firm add --name "Rehearsal Bookkeeping" --email <your inbox>` with the settings loaded; log in from the emailed link
- [ ] Add one Client whose email is **your Resend inbox**. On its page, change one filing's **Due date** to 8 days from today: with the default 5-day offset its Cut-off is in 3 days, so a reminder is due now. Within the hour (the scheduler runs hourly) a Draft appears on the Drafts page; approve it
- [ ] The Reminder arrives in your inbox; it shows "AI" in Kompleto if Claude worded it
- [ ] In Gmail, **Show original**: SPF and DKIM pass (for `resend.dev`; your own domain's DMARC is checked on go-live day)
- [ ] Open the upload link on your phone, take a photo, upload it; it shows on the Client's page and opens after download (F1, and ClamAV scanned it)
- [ ] Ask for an account at `/signup` with another address of yours, then `kompleto requests` and `kompleto requests approve ID`: the welcome email arrives (L1)
- [ ] The Activity page lists all of it (J2)
- [ ] Dashboard and Drafts on your phone are usable

## 8. Backups (`deploy.md` step 7)

- [ ] `rclone config` → remote `gdrive`, Google Drive, scope `drive.file`
- [ ] Run `backup.sh` once by hand with the settings loaded (`BACKUP_REMOTE=gdrive:kompleto-rehearsal-backups`); the file appears in Drive
- [ ] Restore it into a second empty database with `deploy/restore.sh <file>`; the test Firm and its Client are there
- [ ] Note the backup's size and how long it took: ______

## 9. Closing and cleaning up

- [ ] Close the rehearsal Firm in Settings (I2): its login stops working at once
- [ ] Stop the tunnel, `launchctl unload` both plists, drop `kompleto_rehearsal` and the restore database, delete `~/kompleto/uploads-rehearsal` and the Drive folder, and revoke the Resend API key
- [ ] Send me your notes: everything that didn't match, and how long each part took

## On go-live day

Follow `deploy.md` from step 2 with **new** secrets. The rehearsal settings change to: the real database name, `PUBLIC_BASE_URL=https://app.getkompleto.com`, `MAIL_FROM_ADDRESS=reminders@getkompleto.com`, the real upload folder and backup remote, `SUPPORT_EMAIL=help@getkompleto.com`, and a new Resend key and webhook on the verified domain. Then step 9 tests replies (G1) and DMARC, which the rehearsal couldn't.
