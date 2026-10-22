#!/usr/bin/env bash
# Round trip of backup.sh and restore.sh against the local dev database,
# with a stand-in for rclone that copies to a local folder. Run from the
# repository root: deploy/test-backup.sh. Leaves the dev database as it was.
set -euo pipefail
admin="postgres://kompleto:kompleto@localhost:5432"
source_db="$admin/kompleto"
restored_db="$admin/kompleto_restore_test"
work=$(mktemp -d)
trap 'rm -rf "$work"; psql "$admin/postgres" -qc "drop database if exists kompleto_restore_test" >/dev/null' EXIT

# rclone stand-in: "copyto SRC DST" and "delete --min-age Nd DIR" on local paths.
mkdir -p "$work/bin" "$work/remote" "$work/files"
cat > "$work/bin/rclone" <<'SHIM'
#!/usr/bin/env bash
case "$1" in
  copyto) cp "$2" "$3" ;;
  delete) find "$4" -type f -mtime "+${3%d}" -delete ;;
esac
SHIM
chmod +x "$work/bin/rclone"
# The server is Postgres 17 in Docker; use its own pg_dump and pg_restore,
# whatever version the machine has.
if docker exec kompleto-postgres-1 true 2>/dev/null; then
    cat > "$work/bin/pg_dump" <<'SHIM'
#!/usr/bin/env bash
args=(); out=
for arg in "$@"; do case "$arg" in --file=*) out="${arg#--file=}" ;; *) args+=("$arg") ;; esac; done
docker exec -i kompleto-postgres-1 pg_dump "${args[@]}" > "$out"
SHIM
    cat > "$work/bin/pg_restore" <<'SHIM'
#!/usr/bin/env bash
args=("${@:1:$#-1}"); file="${!#}"
docker exec -i kompleto-postgres-1 pg_restore "${args[@]}" < "$file"
SHIM
    chmod +x "$work/bin/pg_dump" "$work/bin/pg_restore"
fi
export PATH="$work/bin:$PATH"

echo "a stored file" > "$work/files/one"
printf 'correct horse battery staple\n' > "$work/passphrase"
export UPLOAD_DIR="$work/files" BACKUP_PASSPHRASE="$work/passphrase" BACKUP_REMOTE="$work/remote"

DATABASE_URL="$source_db" deploy/backup.sh
name=$(ls "$work/remote")
grep -q "a stored file" "$work/remote/$name" && { echo "FAIL: backup not encrypted"; exit 1; }

psql "$admin/postgres" -qc "create database kompleto_restore_test" >/dev/null
rm -rf "$work/files"
DATABASE_URL="$restored_db" UPLOAD_DIR="$work/files" deploy/restore.sh "$name"

count() { psql "$1" -Atc "select (select count(*) from firms), (select count(*) from clients), (select count(*) from audit_events), (select count(*) from outbox_messages)"; }
[ "$(count "$source_db")" = "$(count "$restored_db")" ] || { echo "FAIL: row counts differ"; exit 1; }
[ "$(cat "$work/files/one")" = "a stored file" ] || { echo "FAIL: files not restored"; exit 1; }
waiting=$(psql "$restored_db" -Atc "select count(*) from outbox_messages where status in ('approved', 'sending', 'failed')")
[ "$waiting" = 0 ] || { echo "FAIL: $waiting messages could send again"; exit 1; }
echo "backup round trip OK: $(count "$restored_db") (firms, clients, audit, messages)"
