#!/usr/bin/env sh
# Print the CI result for a commit, read from the ci-status branch that the CI workflow writes.
# Usage: scripts/ci-status.sh [--wait MINUTES] [COMMIT]   (default COMMIT: HEAD)
# Exit 0 if CI passed, 1 if it failed, 2 if no result yet (or timed out).
set -eu
wait_min=0
if [ "${1:-}" = "--wait" ]; then wait_min=$2; shift 2; fi
sha=$(git rev-parse "${1:-HEAD}")
short=$(printf %s "$sha" | cut -c1-7)
deadline=$(( $(date +%s) + wait_min * 60 ))
while :; do
  git fetch -q origin ci-status 2>/dev/null || true
  if git cat-file -e "origin/ci-status:runs/$short.md" 2>/dev/null; then
    git show "origin/ci-status:runs/$short.md"
    if git show "origin/ci-status:runs/$short.md" | grep -q '^result: success'; then exit 0; else exit 1; fi
  fi
  if [ "$(date +%s)" -ge "$deadline" ]; then
    echo "no CI result yet for $short (CI takes a few minutes after a push; use --wait 10)"; exit 2
  fi
  sleep 30
done
