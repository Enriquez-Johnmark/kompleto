#!/usr/bin/env bash
# SessionStart hook: prepares a Claude Code on the web session (Postgres,
# Mailpit, pinned Rust, sqlx-cli, migrations). Does nothing on a developer's
# own machine.
set -uo pipefail
[ -n "${CLAUDE_CODE_REMOTE:-}" ] || exit 0
cd "${CLAUDE_PROJECT_DIR:-.}" || exit 0

log() { echo "cloud-session-start: $*" >&2; }

# Installs the toolchain pinned in rust-toolchain.toml, if rustup is present.
command -v rustup >/dev/null && rustup toolchain install >/dev/null 2>&1

[ -f .env ] || cp .env.example .env

# The cloud VM has Docker installed but doesn't always start the daemon.
if ! docker info >/dev/null 2>&1; then
    nohup dockerd >/tmp/dockerd.log 2>&1 &
    for _ in $(seq 1 30); do docker info >/dev/null 2>&1 && break; sleep 1; done
fi

# Docker Hub rate-limits anonymous pulls (429). mirror.gcr.io serves the same
# official images, so pull from there and give them their usual names.
for image in $(docker compose config --images 2>/dev/null); do
    docker image inspect "$image" >/dev/null 2>&1 && continue
    docker pull -q "$image" >/dev/null 2>&1 && continue
    case "$image" in */*) mirror="mirror.gcr.io/$image" ;; *) mirror="mirror.gcr.io/library/$image" ;; esac
    for _ in 1 2 3; do
        docker pull -q "$mirror" >/dev/null 2>&1 && docker tag "$mirror" "$image" && break
        sleep 5
    done
done

if ! docker compose up -d --wait >/dev/null 2>&1; then
    log "docker compose failed; database tests will not run"
    exit 0
fi

# Slow steps run in the background so the session isn't held up. Migrations
# go first: the sqlx query macros check against the dev database, and a test
# run started meanwhile waits on cargo's build lock.
(
    SQLX_OFFLINE=true cargo run --quiet -- migrate || log "migrations failed"
    # `cargo sqlx prepare` needs sqlx-cli, matching the sqlx in Cargo.toml.
    command -v sqlx >/dev/null || cargo install sqlx-cli --version '^0.9' \
        --no-default-features --features postgres,rustls --locked \
        || log "installing sqlx-cli failed"
) >/tmp/cloud-session-start.log 2>&1 &
exit 0
