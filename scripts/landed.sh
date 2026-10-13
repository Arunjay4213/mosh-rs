#!/usr/bin/env sh
# Exit 0 when the given commit (or every commit of the given branch) has landed on origin/main.
# Main is updated by "rebase and merge", which rewrites shas, so this checks patch equivalence
# with `git cherry` instead of ancestry.
# Usage: scripts/landed.sh <sha-or-branch>
set -eu
ref=${1:?usage: scripts/landed.sh <sha-or-branch>}
git fetch -q origin main
if git merge-base --is-ancestor "$ref" origin/main 2>/dev/null; then echo "landed (ancestor)"; exit 0; fi
# `git cherry upstream head` prints "- sha" for commits already in upstream, "+ sha" for missing ones.
if git rev-parse --verify -q "$ref^{commit}" >/dev/null; then
  pending=$(git cherry origin/main "$ref" | grep -c '^+' || true)
else
  echo "unknown ref $ref"; exit 2
fi
if [ "$pending" -eq 0 ]; then echo "landed (patch-equivalent)"; exit 0; fi
echo "$pending commit(s) of $ref not on main yet"; exit 1
