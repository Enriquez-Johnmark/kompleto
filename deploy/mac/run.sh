#!/usr/bin/env bash
# Starts Kompleto (`run.sh serve`) or a backup (`run.sh backup`) on the
# owner's Mac with the settings in ~/kompleto/kompleto.env. launchd agents
# call this; copy it to ~/kompleto/run.sh.
set -euo pipefail
cd "$HOME/kompleto"
set -a
# shellcheck disable=SC1091
. ./kompleto.env
set +a
export PATH="/opt/homebrew/bin:/usr/local/bin:$PATH"
case "${1:-serve}" in
  serve)
    exec ./kompleto serve ;;
  backup)
    exec ./backup.sh ;;
  *)
    echo "usage: run.sh serve|backup" >&2; exit 2 ;;
esac
