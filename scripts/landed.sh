#!/usr/bin/env sh
# Exit 0 if COMMIT is on origin/main (its branch was merged), 1 if not yet.
# Auto-merge fast-forwards main to the tested commit, so the sha you pushed is the sha on main.
# Usage: scripts/landed.sh COMMIT
set -eu
[ $# -eq 1 ] || { echo "usage: scripts/landed.sh COMMIT"; exit 2; }
sha=$(git rev-parse --verify "$1^{commit}")
git fetch -q origin main
if git merge-base --is-ancestor "$sha" origin/main; then
  echo "landed: $sha is on origin/main"; exit 0
else
  echo "not landed: $sha is not on origin/main"; exit 1
fi
