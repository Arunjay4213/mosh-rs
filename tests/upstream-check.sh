#!/bin/bash
# Mosh: the mobile shell
# Copyright 2012 Keith Winstein
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# In addition, as a special exception, the copyright holders give
# permission to link the code of portions of this program with the
# OpenSSL library under certain conditions as described in each
# individual source file, and distribute linked combinations including
# the two.
#
# You must obey the GNU General Public License in all respects for all
# of the code used other than OpenSSL. If you modify file(s) with this
# exception, you may extend this exception to your version of the
# file(s), but you are not obligated to do so. If you do not wish to do
# so, delete this exception statement from your version. If you delete
# this exception statement from all source files in the program, then
# also delete it here.
#
# Ported from: (new; test tooling). Runs upstream's own make check (src/tests/) with chosen binaries.
#
# Upstream's own test suite with any client/server pair (PLAN.md section 3,
# item 5).
#
# tests/upstream-check.sh [--client cxx|rust|PATH] [--server cxx|rust|PATH]
#
# cxx is the upstream build's own binaries (upstream/src/frontend/, mosh
# decd9b7), rust is target/release/ (built here with
# `cargo build --release -p mosh`). Default: cxx and cxx.
#
# The upstream tree (the `upstream` symlink, else ~/src/mosh) is never
# written. It is copied with rsync -a (which keeps mtimes, so make sees
# everything as built) to target/upstream-check/mosh, the chosen binaries
# replace src/frontend/mosh-client and mosh-server there and are touched
# newer than everything they are built from, `make -q` confirms make will
# not rebuild them, and `make check` runs in the copy.
#
# This works because upstream's e2e driver (src/tests/e2e-test) sets
#   MOSH_CLIENT="$PWD/../frontend/mosh-client"
#   MOSH_SERVER="$PWD/../frontend/mosh-server"
# itself, so it always tests the binaries inside the tree it runs in. The
# tests start sessions with the Perl launcher (scripts/mosh --local), which
# stays as it is.
#
# Prints the automake summary (# TOTAL, # PASS, ...) and exits non-zero
# unless FAIL, ERROR and XPASS are all 0. The full log is
# target/upstream-check/make-check.log, per-test logs are under
# target/upstream-check/mosh/src/tests/.

set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root" || exit 1

usage() {
    sed -n '/^# tests\/upstream-check.sh/,/^# cxx is/p' "$0" | sed 's/^# \{0,1\}//' >&2
    exit 2
}

client=cxx
server=cxx
while [ $# -gt 0 ]; do
    [ $# -ge 2 ] || usage
    case $1 in
        --client) client=$2 ;;
        --server) server=$2 ;;
        *) usage ;;
    esac
    shift 2
done

# The `upstream` symlink (made by scripts/build-upstream.sh, and present in
# the main checkout on devbox), else devbox's ~/src/mosh (an agent's
# separate working copy has no symlink).
upstream=$(readlink -f "$root/upstream")
[ -d "$upstream" ] || upstream=$HOME/src/mosh
if [ ! -x "$upstream/src/frontend/mosh-server" ] || [ ! -f "$upstream/src/tests/Makefile" ]; then
    echo "FAIL upstream-check: no built upstream tree at $root/upstream or $HOME/src/mosh (scripts/build-upstream.sh makes one)"
    exit 1
fi

# bin_path cxx|rust|PATH client|server
bin_path() {
    case $1 in
        cxx) echo "$upstream/src/frontend/mosh-$2" ;;
        rust) echo "$root/target/release/mosh-$2" ;;
        *) realpath -m -- "$1" ;;
    esac
}

if [ "$client" = rust ] || [ "$server" = rust ]; then
    cargo build --release -q -p mosh || exit 1
fi
client_bin=$(bin_path "$client" client)
server_bin=$(bin_path "$server" server)
for b in "$client_bin" "$server_bin"; do
    [ -x "$b" ] || { echo "FAIL upstream-check: $b is not executable"; exit 1; }
done

work=$root/target/upstream-check
copy=$work/mosh
mkdir -p "$work"
echo "upstream-check: client=$client ($client_bin) server=$server ($server_bin)"
echo "upstream-check: copying $upstream to $copy"
rsync -a --delete "$upstream/" "$copy/" || exit 1

# Copy to a temporary name first: a binary may be the destination's own
# source (cxx), and cp onto itself would fail.
for which in client server; do
    src=$client_bin
    [ $which = server ] && src=$server_bin
    cp "$src" "$copy/src/frontend/.mosh-$which.new" || exit 1
    mv -f "$copy/src/frontend/.mosh-$which.new" "$copy/src/frontend/mosh-$which" || exit 1
    touch "$copy/src/frontend/mosh-$which"
done

# make must consider both binaries up to date, or `make check` would
# rebuild them from upstream sources and test those instead.
echo "upstream-check: make -n -C src/frontend mosh-client mosh-server:"
make -n -C "$copy/src/frontend" mosh-client mosh-server 2>&1 | sed 's/^/  /'
if ! make -q -C "$copy/src/frontend" mosh-client mosh-server; then
    echo "FAIL upstream-check: make would rebuild mosh-client or mosh-server in the copy"
    exit 1
fi
echo "upstream-check: make -q says both binaries are up to date"

log=$work/make-check.log
echo "upstream-check: running make check (log: $log)"
start=$(date +%s)
make -C "$copy" check > "$log" 2>&1
make_status=$?
secs=$(($(date +%s) - start))

# Keep the summary of src/tests, the directory with the e2e tests.
summary=$(grep -E '^# (TOTAL|PASS|SKIP|XFAIL|FAIL|XPASS|ERROR):' "$log" | tail -7)
if [ -z "$summary" ]; then
    echo "FAIL upstream-check: no test summary in the make check output (make exit $make_status); last lines:"
    tail -20 "$log"
    exit 1
fi
echo "$summary"
grep -E '^(FAIL|ERROR|XPASS):' "$log"

count() { echo "$summary" | sed -n "s/^# $1: *\([0-9]*\)$/\1/p"; }
fail=$(count FAIL)
error=$(count ERROR)
xpass=$(count XPASS)
total=$(count TOTAL)
line="client=$client server=$server total=$total pass=$(count PASS) xfail=$(count XFAIL) fail=$fail error=$error xpass=$xpass seconds=$secs"
if [ "${fail:-1}" = 0 ] && [ "${error:-1}" = 0 ] && [ "${xpass:-1}" = 0 ] && [ "${total:-0}" -gt 0 ]; then
    echo "PASS upstream-check $line"
else
    echo "FAIL upstream-check $line (per-test logs in $copy/src/tests/*.log)"
    exit 1
fi
