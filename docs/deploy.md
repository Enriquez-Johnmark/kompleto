# Going live

Status: prepared 2026-10-22, not yet followed. It starts once the owner has the domain (decision M6 round, `docs/decisions.md`). Every step is at ₱0 except the domain. Vendor screens change; where a label below no longer matches, look for the nearest one, and note the change here.

Two paths, depending on the card question:

- **Path A, Oracle Cloud Always Free** (needs a bank-issued credit or debit card). Always on.
- **Path B, the owner's Mac** behind a Cloudflare Tunnel (no card). On while the Mac is on and awake.

Both use the same domain, email and backup set-up.

## 1. Secrets, made first

Make these once and keep each in a password manager. Losing the first one loses every stored file; losing the third loses every backup.

| Secret | How to make it | Used as |
|---|---|---|
| File master key | `kompleto gen-key` | `FILE_MASTER_KEY` |
| Inbound secret | `openssl rand -hex 32` | `INBOUND_SECRET`, and the Worker's secret |
| Backup passphrase | `openssl rand -base64 32` | a file named by `BACKUP_PASSPHRASE` |
| Postgres password | `openssl rand -hex 16` | inside `DATABASE_URL` |

## 2. The domain

1. On PayPal, link the Maya virtual card and pay something small first, to be sure it works.
2. Cloudflare dashboard → **Domain Registration → Register Domains** → `getkompleto.com`, paid with PayPal. Cloudflare then runs its DNS, which steps 3–5 need.

## 3. Sending: Resend Free

1. Create a Resend account and add the domain `getkompleto.com`.
2. Add the DNS records Resend shows (SPF and MX on its `send` subdomain, the DKIM record) in Cloudflare DNS, then **Verify**.
3. Add a DMARC record in Cloudflare DNS: name `_dmarc`, type TXT, value `v=DMARC1; p=none; rua=mailto:<your inbox>`. Raise `p=none` to `p=quarantine` after two clean weeks.
4. **API Keys → Create**, with sending access only: this is `RESEND_API_KEY`.
5. **Webhooks → Add endpoint**: `https://app.getkompleto.com/hooks/resend`, events `email.bounced` and `email.complained`. Its signing secret (`whsec_…`) is `RESEND_WEBHOOK_SECRET`.

E2 needs SPF, DKIM and DMARC to pass: check it in step 9.

## 4. Receiving replies: Cloudflare Email Routing

1. Cloudflare → the domain → **Email → Email Routing → Enable**. It adds its own MX records for the domain.
2. **Destination addresses**: add and verify your own inbox.
3. **Custom addresses**: `help@getkompleto.com` → your inbox. This is `SUPPORT_EMAIL`, shown when a Firm closes its account.
4. Deploy the Worker (needs Node on any computer):
   ```
   cd deploy/email-worker
   # set FALLBACK_ADDRESS in wrangler.toml to your verified inbox
   npx wrangler login
   npx wrangler secret put INBOUND_SECRET     # the inbound secret from step 1
   npx wrangler deploy
   ```
5. **Routing rules → Catch-all address → Send to a Worker → `kompleto-replies`**. Every reply address (`reply+CODE@getkompleto.com`) now reaches the app. An email the app can't take goes to your inbox instead of being lost.

Known limit: the free Worker plan allows about 10 ms of CPU per email. Signing a reply with a few photos fits; a reply near the 25 MB limit may not, and then it isn't delivered. If that happens during pilots, raise it: the fix is a different check on the app's side.

## 5. The server

### Path A: Oracle Cloud

1. Sign up with **home region Singapore** (it can't be changed later). Upgrade to Pay As You Go, which stops idle reclaiming, and set a budget alert at US$1.
2. Create an instance: **Ampere A1**, 2 OCPU, 12 GB, Ubuntu 24.04. Keep only SSH open; the tunnel needs no open ports.
3. On it:
   ```
   sudo apt install -y postgresql-common && sudo /usr/share/postgresql-common/pgdg/apt.postgresql.org.sh
   sudo apt install -y postgresql-17 gnupg rclone build-essential pkg-config
   sudo -u postgres createuser kompleto --pwprompt     # the Postgres password
   sudo -u postgres createdb kompleto --owner kompleto
   curl https://sh.rustup.rs -sSf | sh
   # then, in a clone of this (private) repository: `gh auth login` first,
   # or a read-only deploy key added on GitHub
   git clone https://github.com/Enriquez-Johnmark/kompleto && cd kompleto
   cargo build --release
   sudo useradd --system --home /var/lib/kompleto --create-home kompleto
   sudo mkdir -p /opt/kompleto /etc/kompleto
   sudo cp target/release/kompleto deploy/backup.sh /opt/kompleto/
   ```
   Postgres listens only on localhost by default; keep it that way.
4. Write `/etc/kompleto/kompleto.env` (below), then `sudo chmod 600 /etc/kompleto/*.env`, and install the units:
   ```
   sudo cp deploy/linux/*.service deploy/linux/*.timer /etc/systemd/system/
   sudo systemctl enable --now kompleto kompleto-backup.timer
   ```

### Path B: the owner's Mac

1. `brew install postgresql@17 cloudflared rclone gnupg`, then `brew services start postgresql@17`, and create the user and database as in path A.
2. Build with `cargo build --release`. Copy `target/release/kompleto`, `deploy/backup.sh` and `deploy/mac/run.sh` into `~/kompleto/`, and write `~/kompleto/kompleto.env` (below, `chmod 600`).
3. Copy both files in `deploy/mac/*.plist` to `~/Library/LaunchAgents/` and `launchctl load` each.
4. **System Settings → Energy**: prevent automatic sleeping while the display is off. While the Mac sleeps or is off, nothing is sent and replies queue in Cloudflare.

### The settings file

```
DATABASE_URL=postgres://kompleto:<postgres password>@localhost:5432/kompleto
BIND_ADDR=127.0.0.1:3000
PUBLIC_BASE_URL=https://app.getkompleto.com
MAIL_PROVIDER=resend
RESEND_API_KEY=<from step 3>
RESEND_WEBHOOK_SECRET=<from step 3>
MAIL_FROM_ADDRESS=reminders@getkompleto.com
INBOUND_SECRET=<from step 1>
FILE_MASTER_KEY=<from step 1>
UPLOAD_DIR=/var/lib/kompleto/uploads          # ~/kompleto/uploads on the Mac
SUPPORT_EMAIL=help@getkompleto.com
# ANTHROPIC_API_KEY=   only once paid for; drafts are templates until then
```

For backups, `/etc/kompleto/backup.env` (path A) or the same file (path B):

```
BACKUP_PASSPHRASE=/etc/kompleto/backup-passphrase   # a file, mode 600
BACKUP_REMOTE=gdrive:kompleto-backups
```

## 6. The tunnel

```
cloudflared tunnel login
cloudflared tunnel create kompleto
cloudflared tunnel route dns kompleto app.getkompleto.com
```

Copy `deploy/cloudflared.yml` to the cloudflared config folder, set `credentials-file` to the JSON file `create` made, then `sudo cloudflared service install`. In Cloudflare, add a rate limiting rule for `/login` (the free plan allows one rule).

## 7. Backups to Google Drive

1. `rclone config` → new remote `gdrive`, type Google Drive, scope `drive.file`. Run it as the `kompleto` user on the server.
2. Run `deploy/backup.sh` once by hand (with the settings loaded), and check that the file appears in Drive.
3. Practise a restore into a scratch database: `deploy/restore.sh <file>` with `DATABASE_URL` pointing at an empty database. `deploy/test-backup.sh` checks the same round trip on a development machine.

## 8. Watching it

Better Stack Free → a monitor on `https://app.getkompleto.com/health`, every 3 minutes, alerting your email. "ok" means the app and its database answer.

## 9. First firm, and checks before the first pilot

1. Add the firm with its settings loaded. Path A:
   ```
   sudo -u kompleto bash -c 'set -a; . /etc/kompleto/kompleto.env; /opt/kompleto/kompleto firm add --name "<Firm>" --email <their login address>'
   ```
   Path B: the same with `~/kompleto/kompleto.env` and `~/kompleto/kompleto`.
2. Make a test firm with your own address as a Client, approve one Reminder, and check:
   - it arrives; in Gmail, **Show original** says SPF, DKIM and DMARC **PASS** (E2);
   - replying to it shows the reply in Kompleto and emails the firm a notice (G1);
   - the upload link opens on a phone and takes a photo (F1);
   - the Activity page lists all of it (J2).
3. Close the test firm's account, to be sure it's deleted 7 days later (I2).
