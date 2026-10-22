#!/usr/bin/env bash
# Nightly encrypted backup to the owner's Google Drive, kept 30 days, so a
# closed account (I2) is gone from backups within 30 days too.
#
# Holds the database (pg_dump) and the stored files. The files are already
# encrypted with FILE_MASTER_KEY; the whole backup is encrypted again with
# its own passphrase (gpg, AES-256), so Google never sees anything readable.
#
# Needs: pg_dump, gpg, rclone (with a Google Drive remote set up, see
# docs/deploy.md). Settings, from the environment:
#   DATABASE_URL       the production database
#   UPLOAD_DIR         where the app keeps stored files
#   BACKUP_PASSPHRASE  a file holding the backup passphrase (keep a copy in
#                      the password manager; without it no backup restores)
#   BACKUP_REMOTE      the rclone folder, e.g. gdrive:kompleto-backups
#   BACKUP_KEEP_DAYS   optional, default 30
set -euo pipefail
: "${DATABASE_URL:?}" "${UPLOAD_DIR:?}" "${BACKUP_PASSPHRASE:?}" "${BACKUP_REMOTE:?}"
keep="${BACKUP_KEEP_DAYS:-30}"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
chmod 700 "$work"
name="kompleto-$(date -u +%Y-%m-%dT%H%M%SZ).tar.gpg"

pg_dump --format=custom --no-owner --no-privileges --file="$work/database.dump" "$DATABASE_URL"
tar -C "$UPLOAD_DIR" -cf "$work/files.tar" .
tar -C "$work" -cf - database.dump files.tar |
    gpg --batch --yes --quiet --pinentry-mode loopback --passphrase-file "$BACKUP_PASSPHRASE" \
        --symmetric --cipher-algo AES256 --output "$work/$name"

rclone copyto "$work/$name" "$BACKUP_REMOTE/$name"
rclone delete --min-age "${keep}d" "$BACKUP_REMOTE"
echo "backed up $name"
