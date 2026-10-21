#!/usr/bin/env bash
# SessionStart hook: prepares a Claude Code on the web session (Postgres,
# Mailpit, pinned Rust). Does nothing on a developer's own machine.
set -uo pipefail
[ -n "${CLAUDE_CODE_REMOTE:-}" ] || exit 0
cd "${CLAUDE_PROJECT_DIR:-.}" || exit 0

# Installs the toolchain pinned in rust-toolchain.toml, if rustup is present.
command -v rustup >/dev/null && rustup toolchain install >/dev/null 2>&1

[ -f .env ] || cp .env.example .env
if ! docker compose up -d --wait >/dev/null 2>&1; then
    echo "cloud-session-start: docker compose failed; database tests will not run" >&2
fi
exit 0
