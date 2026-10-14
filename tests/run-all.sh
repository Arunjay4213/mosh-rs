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
# Runs every test: each cargo integration test, the workspace's lib, bin and
# doc tests, and every executable tests/*.sh driver. Exits non-zero if
# anything fails.
#
# Environment:
#   RUN_ALL_ONLY=PATTERNS  comma-separated names or globs; run only the
#       matching items. Names are cargo integration test names (client_e2e),
#       driver file names (scrollback.sh), and lib-bin-doc (the workspace's
#       lib, bin and doc tests). A pattern that matches nothing is an error,
#       so a typo in a CI shard cannot silently skip tests.
#       Example: RUN_ALL_ONLY='client_e2e,transport-*.sh' tests/run-all.sh
#   MOSH_CI_FULL  exported as 1 unless it is already 0, so tests that guard
#       their slow cases behind it run in full here.
#
# Scheduling. Everything is built first (cargo build plus cargo test
# --no-run), so nothing compiles while tests run. Then:
#   - The cargo lane runs the cargo test items one after another, as cargo
#     does, because several of them bind the same fixed ports (61200 is used
#     by server_e2e and scrollback_server). After them it runs the drivers
#     listed in AFTER_CARGO.
#   - Every other driver starts at once, in parallel with the cargo lane.
#     interop.sh, interop-setup-loss.sh and scrollback.sh serialize among
#     themselves through /tmp/mosh-rs-interop.lock; transport-impaired.sh
#     holds its own lock for its fixed ports 60700-60702.
#
# Each item's output goes to target/run-all/NAME.log and is printed as one
# block when the item finishes: the last lines when it passed, all of it when
# it failed. A PASS/FAIL line per item and the wall time come at the end.

set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root" || exit 1
self=$(basename "$0")

if [ "${MOSH_CI_FULL-}" != 0 ]; then
    export MOSH_CI_FULL=1
fi

# Drivers that run after the cargo tests. scrollback.sh owns 61500-61599
# and SIGKILLs whatever is bound there; no cargo test binds in that range
# (detached_warning.rs uses 61400-61409, scrollback_hostile.rs 61410-61419),
# and keeping it after them also keeps its sessions off the cargo tests'
# timing.
AFTER_CARGO=" scrollback.sh "

PASS_TAIL=15
start_all=$(date +%s)

# All items, in display order. cargo integration tests come from cargo
# metadata so tests added to any workspace package are picked up.
mapfile -t cargo_items < <(cargo metadata --format-version 1 --no-deps | python3 -c '
import json, sys
for p in json.load(sys.stdin)["packages"]:
    for t in p["targets"]:
        if t["kind"] == ["test"]:
            print(t["name"])
' | sort)
if [ ${#cargo_items[@]} -eq 0 ]; then
    echo "run-all: cargo metadata listed no integration tests"
    exit 1
fi
cargo_items=(lib-bin-doc "${cargo_items[@]}")
driver_items=()
for t in tests/*.sh; do
    [ -x "$t" ] || continue
    [ "$(basename "$t")" = "$self" ] && continue
    driver_items+=("$(basename "$t")")
done

selected() {
    [ -z "${RUN_ALL_ONLY-}" ] && return 0
    local pat
    local IFS=,
    for pat in $RUN_ALL_ONLY; do
        # Unquoted on the right of == so the pattern is a glob.
        # shellcheck disable=SC2053
        [[ $1 == $pat ]] && return 0
    done
    return 1
}

if [ -n "${RUN_ALL_ONLY-}" ]; then
    IFS=, read -ra pats <<< "$RUN_ALL_ONLY"
    for pat in "${pats[@]}"; do
        hit=no
        for name in "${cargo_items[@]}" "${driver_items[@]}"; do
            # shellcheck disable=SC2053
            [[ $name == $pat ]] && hit=yes
        done
        if [ $hit = no ]; then
            echo "run-all: RUN_ALL_ONLY pattern '$pat' matches nothing; names are: ${cargo_items[*]} ${driver_items[*]}"
            exit 2
        fi
    done
fi

lane_cargo=()
lane_after=()
lane_parallel=()
for name in "${cargo_items[@]}"; do selected "$name" && lane_cargo+=("$name"); done
for name in "${driver_items[@]}"; do
    selected "$name" || continue
    if [[ $AFTER_CARGO == *" $name "* ]]; then lane_after+=("$name"); else lane_parallel+=("$name"); fi
done
all_items=("${lane_cargo[@]}" "${lane_after[@]}" "${lane_parallel[@]}")
if [ ${#all_items[@]} -eq 0 ]; then
    echo "run-all: nothing selected"
    exit 2
fi
echo "run-all: ${#all_items[@]} item(s): ${all_items[*]}"
echo "run-all: MOSH_CI_FULL=$MOSH_CI_FULL"

out=$root/target/run-all
rm -rf "$out"
mkdir -p "$out"

echo "run-all: building"
if ! cargo build --release --workspace || ! cargo test --release --workspace --no-run; then
    echo "run-all: FAILED: build"
    exit 1
fi

# run_item NAME: run one item, record "EXIT SECONDS" in NAME.result, print its block.
run_item() {
    local name=$1 log="$out/$1.log" t0 rc
    t0=$(date +%s)
    case $name in
        lib-bin-doc)
            # Both halves run even if the first fails, as one cargo test would.
            {
                cargo test --release --workspace --no-fail-fast --lib --bins
                local first=$?
                cargo test --release --workspace --no-fail-fast --doc && [ $first -eq 0 ]
            } > "$log" 2>&1 ;;
        *.sh)
            "tests/$name" > "$log" 2>&1 ;;
        *)
            cargo test --release --workspace --no-fail-fast --test "$name" > "$log" 2>&1 ;;
    esac
    rc=$?
    local secs=$(($(date +%s) - t0))
    echo "$rc $secs" > "$out/$name.result"
    local verdict=PASS
    [ $rc -ne 0 ] && verdict="FAIL (exit $rc)"
    {
        flock 8
        echo
        echo "==== $name: $verdict in ${secs}s"
        if [ $rc -eq 0 ]; then
            local n
            n=$(wc -l < "$log")
            [ "$n" -gt $PASS_TAIL ] && echo "[last $PASS_TAIL of $n lines; full log: target/run-all/$name.log]"
            tail -n $PASS_TAIL "$log"
        else
            cat "$log"
        fi
        echo "==== end $name"
    } 8> "$out/.print.lock"
}

cargo_lane() {
    local name
    for name in "${lane_cargo[@]}" "${lane_after[@]}"; do run_item "$name"; done
}

trap 'kill $(jobs -p) 2> /dev/null; exit 130' INT TERM

cargo_lane &
for name in "${lane_parallel[@]}"; do
    run_item "$name" &
done
wait

echo
echo "run-all: results"
failed=0
for name in "${all_items[@]}"; do
    if [ -f "$out/$name.result" ]; then
        read -r rc secs < "$out/$name.result"
    else
        rc=missing secs='?'
    fi
    if [ "$rc" = 0 ]; then
        printf 'PASS %-28s %5ss\n' "$name" "$secs"
    else
        printf 'FAIL %-28s %5ss  exit %s\n' "$name" "$secs" "$rc"
        failed=$((failed + 1))
    fi
done
wall=$(($(date +%s) - start_all))
echo "run-all: ${#all_items[@]} item(s), $failed failed, wall time ${wall}s"
if [ $failed -ne 0 ]; then
    echo "run-all: FAILED"
    exit 1
fi
echo "run-all: all passed"
