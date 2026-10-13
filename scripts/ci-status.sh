#!/usr/bin/env sh
# Print the CI result for a commit or a PR, read from the ci-status branch that the CI workflow writes.
# Usage: scripts/ci-status.sh [--wait MINUTES] [COMMIT]          (default COMMIT: HEAD)
#        scripts/ci-status.sh [--wait MINUTES] --pr BRANCH|COMMIT
# --pr BRANCH prints the newest report for the PR opened from BRANCH (reports name their branch);
# --pr COMMIT is the same as plain COMMIT, since every run is stored by commit.
# Exit 0 if CI passed, 1 if it failed, 2 if no result yet (or timed out).
set -eu
wait_min=0
pr=""
while [ $# -gt 0 ]; do
  case "$1" in
    --wait) wait_min=$2; shift 2 ;;
    --pr) pr=$2; shift 2 ;;
    *) break ;;
  esac
done
target=${pr:-${1:-HEAD}}
# A --pr argument that resolves to a commit (and is not a branch name) is looked up by commit.
branch=""
if [ -n "$pr" ] && ! git rev-parse -q --verify "refs/heads/$pr" >/dev/null && git rev-parse -q --verify "$pr^{commit}" >/dev/null; then
  :
elif [ -n "$pr" ]; then
  branch=$pr
fi

# Print the report path for this lookup on origin/ci-status, or nothing.
find_report() {
  if [ -n "$branch" ]; then
    git ls-tree --name-only origin/ci-status prs/ 2>/dev/null | sed 's|^prs/||; s|\.md$||' | sort -rn | while read -r n; do
      if git show "origin/ci-status:prs/$n.md" | grep -qx "branch: $branch"; then echo "prs/$n.md"; break; fi
    done
  else
    short=$(git rev-parse "$target" | cut -c1-7)
    if git cat-file -e "origin/ci-status:runs/$short.md" 2>/dev/null; then echo "runs/$short.md"; fi
  fi
}

deadline=$(( $(date +%s) + wait_min * 60 ))
while :; do
  git fetch -q origin ci-status 2>/dev/null || true
  path=$(find_report)
  if [ -n "$path" ]; then
    git show "origin/ci-status:$path"
    if git show "origin/ci-status:$path" | grep -q '^result: success'; then exit 0; else exit 1; fi
  fi
  if [ "$(date +%s)" -ge "$deadline" ]; then
    echo "no CI result yet for $target (CI takes about 20 minutes after a push; use --wait 25)"; exit 2
  fi
  sleep 30
done
