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
# Ported from: (new; test tooling)
#
# Runs the transport sync test (tests/tests/transport_sync.rs) through
# udp-impair, three times. Control actions are timed from the start of the
# test's session:
#   1. 30% loss, 100 ms + 0-50 ms jitter, 10% reordering, and a client address
#      change (proxy rebind) at 5 s. The server must attach to two different
#      client addresses, and the state must still converge.
#   2. 5% loss, same delay and reordering, no address change.
#   3. Same as 1 without the rebind, plus a 3 s blackout starting at 4 s.
# Prints one summary line per run; exits non-zero if any run fails.
# Holds /tmp/mosh-rs-transport-impaired.lock while it runs.

set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

LISTEN=127.0.0.1:60700
SERVER_PORT=60701
CONTROL=127.0.0.1:60702

# The ports above are fixed, so one run at a time per machine (another
# checkout or worktree may run this script too).
exec 8> /tmp/mosh-rs-transport-impaired.lock
if ! flock -n 8; then
    echo "transport-impaired: another run holds /tmp/mosh-rs-transport-impaired.lock, waiting"
    flock 8
fi

cargo build --release -p mosh-tools --bin udp-impair
# Build once, then run the test executable directly. Going through
# `cargo test` each time adds seconds of startup (more when the tree is being
# edited), which would shift the control actions relative to the session.
test_bin=$(cargo test --release -p mosh-tests --test transport_sync --no-run --message-format=json \
    | python3 -c '
import json, sys
for line in sys.stdin:
    m = json.loads(line)
    if m.get("reason") == "compiler-artifact" and m["target"]["name"] == "transport_sync" and m.get("executable"):
        print(m["executable"])
')
[ -x "$test_bin" ] || { echo "transport-impaired: could not build transport_sync"; exit 1; }

work=$(mktemp -d)
proxy_pid=
cleanup() {
    if [ -n "$proxy_pid" ]; then kill "$proxy_pid" 2>/dev/null || true; wait "$proxy_pid" 2>/dev/null || true; fi
    jobs -p | xargs -r kill 2>/dev/null || true
    rm -rf "$work"
}
trap cleanup EXIT

# Send one control datagram and print the reply.
control() {
    python3 -c '
import socket, sys
host, port = sys.argv[1].rsplit(":", 1)
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(2)
s.sendto(sys.argv[2].encode(), (host, int(port)))
print(s.recv(1024).decode())
' "$CONTROL" "$1"
}

failures=0

# run NAME AT_SECONDS CONTROL_COMMAND MIN_ATTACH PROXY_ARGS...
# AT_SECONDS/CONTROL_COMMAND may be "-" for no control action.
run() {
    local name=$1 at=$2 cmd=$3 min_attach=$4
    shift 4
    local log="$work/$name.log"
    target/release/udp-impair --listen $LISTEN --target 127.0.0.1:$SERVER_PORT \
        --control $CONTROL "$@" > "$work/$name.proxy" 2>&1 &
    proxy_pid=$!
    for _ in $(seq 50); do [ -s "$work/$name.proxy" ] && break; sleep 0.1; done

    local ctl_pid=
    if [ "$at" != - ]; then
        # Count from the moment the test starts its session.
        (until grep -q '^transport_sync seed=' "$log" 2>/dev/null; do sleep 0.05; done
         sleep "$at"; echo "control at ${at}s: $cmd -> $(control "$cmd")" > "$work/$name.ctl") &
        ctl_pid=$!
    fi

    local status=0
    MOSH_IMPAIR_PROXY=$LISTEN MOSH_SYNC_SERVER_PORT=$SERVER_PORT \
        "$test_bin" --nocapture > "$log" 2>&1 || status=$?
    # If the test died before its session started, the control job is still
    # waiting for it; stop it so this script cannot hang.
    if [ -n "$ctl_pid" ]; then kill "$ctl_pid" 2>/dev/null || true; wait "$ctl_pid" 2>/dev/null || true; fi

    local stats
    stats=$(control stats 2>/dev/null || echo "no stats reply")
    kill "$proxy_pid" 2>/dev/null || true
    wait "$proxy_pid" 2>/dev/null || true
    proxy_pid=

    local attach elapsed ok=yes why=""
    # A failed run may lack these lines; no match must not trip `set -e`.
    attach=$( (grep -o 'Server now attached to client at [0-9.:]*' "$log" || true) | sort -u | wc -l)
    elapsed=$( (grep -o 'elapsed [0-9.]*s' "$log" || true) | head -1 | cut -d' ' -f2)
    if [ "$status" -ne 0 ] || ! grep -q 'test result: ok' "$log"; then
        ok=no; why="$why test failed (exit $status);"
    fi
    if [ "$at" != - ] && ! grep -q -- '-> ok' "$work/$name.ctl" 2>/dev/null; then
        ok=no; why="$why control command '$cmd' was not acknowledged during the session;"
    fi
    if [ "$attach" -lt "$min_attach" ]; then
        ok=no; why="$why $attach distinct attach address(es), need $min_attach;"
    fi

    echo "--- $name: proxy $*"
    [ -f "$work/$name.ctl" ] && cat "$work/$name.ctl"
    (grep 'Server now attached to client at' "$log" || true) | awk '!seen[$0]++'
    grep -E '^elapsed |^test result' "$log" || true
    echo "proxy stats: $stats"
    if [ $ok = yes ]; then
        echo "PASS $name: converged in ${elapsed:-?}, $attach client address(es)"
    else
        echo "FAIL $name:$why log follows"
        tail -40 "$log"
        failures=$((failures + 1))
    fi
}

common=(--delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7)
run loss30-rebind 5 rebind 2 --loss 0.30 "${common[@]}"
run loss5 - - 1 --loss 0.05 "${common[@]}"
run loss30-blackout 4 "blackout 3000" 1 --loss 0.30 "${common[@]}"

if [ $failures -ne 0 ]; then
    echo "transport-impaired: $failures run(s) FAILED"
    exit 1
fi
echo "transport-impaired: all 3 runs passed"
