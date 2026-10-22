#!/usr/bin/env bash
# Restores one backup made by backup.sh: `restore.sh kompleto-<stamp>.tar.gpg`.
# Stop the app first (it must not send while this runs), and restore into
# an empty database. Same settings as backup.sh.
#
# A backup can be up to a day old, so emails sent after it was made would
# look unsent. Every message that was approved but not confirmed sent is
# marked "unconfirmed": it shows on the Drafts page and goes out only if a
# person checks and re-approves it (E1: never send twice).
set -euo pipefail
: "${DATABASE_URL:?}" "${UPLOAD_DIR:?}" "${BACKUP_PASSPHRASE:?}" "${BACKUP_REMOTE:?}"
name="${1:?usage: restore.sh kompleto-<stamp>.tar.gpg (rclone ls \$BACKUP_REMOTE lists them)}"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
chmod 700 "$work"

rclone copyto "$BACKUP_REMOTE/$name" "$work/$name"
gpg --batch --yes --quiet --pinentry-mode loopback --passphrase-file "$BACKUP_PASSPHRASE" \
    --decrypt --output "$work/backup.tar" "$work/$name"
tar -C "$work" -xf "$work/backup.tar"

pg_restore --no-owner --no-privileges --exit-on-error --dbname="$DATABASE_URL" "$work/database.dump"
mkdir -p "$UPLOAD_DIR"
tar -C "$UPLOAD_DIR" -xf "$work/files.tar"
psql "$DATABASE_URL" --quiet --command="
    update outbox_messages
    set status = 'unconfirmed', may_have_sent = true, retry_at = null,
        last_error = 'restored from a backup: check whether it went out before resending'
    where status in ('approved', 'sending', 'failed')"
echo "restored $name"
