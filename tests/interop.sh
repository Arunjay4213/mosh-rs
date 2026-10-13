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
# Ported from: (new; test tooling). The direct-versus-mosh screen comparison follows upstream src/tests/e2e-test.
#
# Interop matrix (PLAN.md section 3, item 4).
#
# tests/interop.sh [--client cxx|rust|all|PATH] [--server cxx|rust|all|PATH]
#                  [--profile clean|loss5|loss30|blackout|rebind|all]
#                  [--scenario seq|colors|vim|wide|resize|typing|all]
#
# Each option may be given more than once. cxx is mosh 1.4.0 from apt
# (/usr/bin/mosh-client, /usr/bin/mosh-server), rust is target/release/
# (built here with `cargo build --release -p mosh`), all is both.
#
# One cell = one client, one server, one network profile, one scenario:
#   1. `mosh-server new -c 256 -p PORTS -- bash tests/interop/<scenario>.sh`,
#      reading MOSH CONNECT from its output. The server pid comes from
#      `ss -ulpn` on that port so it can be killed at the end.
#   2. udp-impair between client and server, with the profile's impairment.
#   3. The client in a fresh 80x24 tmux server, pointed at the proxy.
#   4. The scenario prints READY-<name>; the harness sends Enter, fires the
#      profile's event (blackout, rebind), runs the scenario's keystrokes,
#      and polls `tmux capture-pane` until the screen equals the expected
#      one (`-e`, with attributes, for colors).
# The expected screens come from running every scenario once directly in a
# tmux pane of the same size, without mosh (like upstream's e2e "direct"
# run). Profiles:
#   clean    no impairment (still through the proxy)
#   loss5    5% loss each way
#   loss30   30% loss, 200 ms delay + 0-50 ms jitter, 10% reordering
#            Loss starts once READY is on screen, so the session is up
#            and everything after (Enter, output, keystrokes) is lossy.
#            Connection setup is not tested under loss: like upstream
#            (stmclient.cc, "Timed out waiting for server..."), a client
#            that hears nothing for 15 s gives up, and at 30% loss that
#            happens now and then by chance alone.
#   blackout clean, plus all traffic dropped for INTEROP_BLACKOUT_MS just
#            after the scenario starts; afterwards typed keys must echo
#            within 10 s
#   rebind   clean, plus the proxy changes its source port just after the
#            scenario starts; afterwards typed keys must echo within 10 s,
#            and the proxy must have forwarded server-to-client packets
#            after the rebind. The proxy closes the old port, so those
#            packets prove the server sends to the new address.
#
# Output: one PASS/FAIL line per cell and a summary; exit 1 if any FAIL. On
# FAIL, target/interop/<cell>/ keeps the expected and actual captures, their
# unified diff, and the server and proxy output.
#
# With no arguments (what tests/run-all.sh does) it runs the quick subset:
# cxx client x cxx server, every profile and scenario, 10 s blackout. With
# arguments, or with INTEROP_FULL=1, the blackout is PLAN's 120 s.
#
# Environment:
#   INTEROP_FULL=1          120 s blackout even with no arguments
#   INTEROP_BLACKOUT_MS=N   blackout length (overrides the above)
#   INTEROP_JOBS=N          cells run in parallel (default 10, max 50)
#   INTEROP_CORRUPT_DIRECT=SCENARIO
#                           corrupt that scenario's expected screen, to show
#                           the harness fails
#
# UDP ports 61000-61199 only: 4 per parallel slot (2 for the server's -p
# range, the proxy's listen and control ports). A lock file serializes
# concurrent runs on one machine so they cannot collide on those ports.

set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root" || exit 1
scen_dir=$root/tests/interop
run_dir=$root/target/interop
impair=$root/target/release/udp-impair

ALL_PROFILES=(blackout clean loss5 loss30 rebind)
ALL_SCENARIOS=(seq colors vim wide resize typing)
PORT_FIRST=61000
PORT_LAST=61199
SLOT_PORTS=4

usage() {
    sed -n '/^# tests\/interop.sh/,/^# Environment/p' "$0" | sed 's/^# \{0,1\}//' >&2
    exit 2
}

clients=()
servers=()
profiles=()
scenarios=()
quick=0
[ $# -eq 0 ] && [ "${INTEROP_FULL:-0}" != 1 ] && quick=1
while [ $# -gt 0 ]; do
    [ $# -ge 2 ] || usage
    case $1 in
        --client) [ "$2" = all ] && clients+=(cxx rust) || clients+=("$2") ;;
        --server) [ "$2" = all ] && servers+=(cxx rust) || servers+=("$2") ;;
        --profile) [ "$2" = all ] && profiles+=("${ALL_PROFILES[@]}") || profiles+=("$2") ;;
        --scenario) [ "$2" = all ] && scenarios+=("${ALL_SCENARIOS[@]}") || scenarios+=("$2") ;;
        *) usage ;;
    esac
    shift 2
done
[ ${#clients[@]} -eq 0 ] && clients=(cxx)
[ ${#servers[@]} -eq 0 ] && servers=(cxx)
[ ${#profiles[@]} -eq 0 ] && profiles=("${ALL_PROFILES[@]}")
[ ${#scenarios[@]} -eq 0 ] && scenarios=("${ALL_SCENARIOS[@]}")
for p in "${profiles[@]}"; do
    [[ " ${ALL_PROFILES[*]} " == *" $p "* ]] || { echo "interop: unknown profile $p" >&2; usage; }
done
for s in "${scenarios[@]}"; do
    [ -f "$scen_dir/$s.sh" ] || { echo "interop: unknown scenario $s" >&2; usage; }
done

if [ $quick = 1 ]; then
    BLACKOUT_MS=${INTEROP_BLACKOUT_MS:-10000}
else
    BLACKOUT_MS=${INTEROP_BLACKOUT_MS:-120000}
fi
JOBS=${INTEROP_JOBS:-10}
max_jobs=$(((PORT_LAST - PORT_FIRST + 1) / SLOT_PORTS))
[ "$JOBS" -ge 1 ] && [ "$JOBS" -le $max_jobs ] || { echo "interop: INTEROP_JOBS must be 1-$max_jobs" >&2; exit 2; }

# Prerequisites. Missing ones fail the run with their names; nothing is
# skipped silently.
missing=()
for cmd in tmux vim ss python3 flock cargo; do
    command -v $cmd > /dev/null || missing+=("$cmd")
done
locale -a 2> /dev/null | grep -qiE '^c\.utf-?8$' || missing+=("C.UTF-8 locale")
if [ ${#missing[@]} -ne 0 ]; then
    echo "FAIL interop: missing prerequisites: ${missing[*]}"
    exit 1
fi

# bin_path cxx|rust|PATH client|server
bin_path() {
    case $1 in
        cxx) echo "/usr/bin/mosh-$2" ;;
        rust) echo "$root/target/release/mosh-$2" ;;
        *) realpath -m -- "$1" ;;
    esac
}
# A name for the cell directory: cxx, rust, or the path's file name.
bin_label() {
    case $1 in
        cxx | rust) echo "$1" ;;
        *) basename -- "$1" ;;
    esac
}

cargo build --release -q -p mosh-tools --bin udp-impair || exit 1
if [[ " ${clients[*]} ${servers[*]} " == *" rust "* ]]; then
    cargo build --release -q -p mosh || exit 1
fi
for c in "${clients[@]}"; do [ -x "$(bin_path "$c" client)" ] || { echo "FAIL interop: client $c: $(bin_path "$c" client) is not executable"; exit 1; }; done
for s in "${servers[@]}"; do [ -x "$(bin_path "$s" server)" ] || { echo "FAIL interop: server $s: $(bin_path "$s" server) is not executable"; exit 1; }; done

# One run at a time per machine: the port range is fixed.
exec 9> /tmp/mosh-rs-interop.lock
if ! flock -n 9; then
    echo "interop: another tests/interop.sh holds /tmp/mosh-rs-interop.lock, waiting"
    flock 9
fi

rm -rf "$run_dir"
mkdir -p "$run_dir/expected"
# tmux socket paths must stay under 108 bytes, so they live in /tmp.
sock_dir=$(mktemp -d /tmp/interop.XXXXXX)

cleanup() {
    local pids f s
    pids=$(jobs -p)
    [ -n "$pids" ] && kill $pids 2> /dev/null
    for f in "$run_dir"/*/pids; do
        [ -f "$f" ] && xargs -r kill < "$f" 2> /dev/null
    done
    for s in "$sock_dir"/*; do
        [ -S "$s" ] && tmux -S "$s" kill-server 2> /dev/null
    done
    # Anything still bound in our port range is ours (the lock guarantees no
    # other run uses it): servers that detached before their pid was recorded.
    ss -Hulpn 2> /dev/null | while read -r _ _ _ local _ users; do
        local port=${local##*:}
        if [ "$port" -ge $PORT_FIRST ] && [ "$port" -le $PORT_LAST ]; then
            grep -o 'pid=[0-9]*' <<< "$users" | cut -d= -f2
        fi
    done | sort -u | xargs -r kill 2> /dev/null
    rm -rf "$sock_dir"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

# stop_pids PID...: SIGTERM, then SIGKILL whatever is still alive after 2 s.
# mosh-server answers SIGTERM by sending its client up to 16 shutdown
# packets and waiting for an ack (upstream mosh-server.cc, SHUTDOWN_RETRIES),
# holding its UDP port meanwhile; with the client gone that takes seconds,
# and the next cell on the same slot would find the port busy.
stop_pids() {
    local p alive
    [ $# -eq 0 ] && return 0
    kill "$@" 2> /dev/null
    for _ in $(seq 20); do
        alive=
        for p in "$@"; do kill -0 "$p" 2> /dev/null && alive=1; done
        [ -z "$alive" ] && return 0
        sleep 0.1
    done
    kill -KILL "$@" 2> /dev/null
}

# kill_client SOCKET: SIGKILL the client in the tmux pane, then the tmux
# server. Killing only tmux would SIGHUP the client, and mosh-client answers
# SIGHUP by sending shutdown packets for seconds (stmclient.cc). Those would
# reach the next cell's proxy on the same port, which would then send the
# server's replies to that dead client instead of the new one.
kill_client() {
    local pane
    pane=$(tmux -S "$1" list-panes -a -F '#{pane_pid}' 2> /dev/null)
    [ -n "$pane" ] && pkill -KILL -P "$pane"
    tmux -S "$1" kill-server 2> /dev/null
}

now_ms() { echo $((${EPOCHREALTIME/./} / 1000)); }

# Send one control datagram to the proxy and print the reply.
control() {
    python3 -c '
import socket, sys
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(2)
s.sendto(sys.argv[2].encode(), ("127.0.0.1", int(sys.argv[1])))
print(s.recv(1024).decode())
' "$ctl_port" "$1"
}
s2c_forwarded() {
    control stats | sed -n 's/.*s2c forwarded=\([0-9]*\).*/\1/p'
}

# ---- Helpers for the scenario drive functions and the cell logic. They use
# the globals of the run in progress: sock, mode (direct|mosh), scen,
# cap_flags, expect_dir, cell_dir, blackout_end, why.

screen() { tmux -S "$sock" capture-pane -p -t interop "$@"; }
keys() { tmux -S "$sock" send-keys -t interop "$@"; }
resize() { tmux -S "$sock" resize-window -t interop -x "$1" -y "$2"; }

# Deadline for a wait: SECONDS from now, or from the end of a blackout that
# is still going on (nothing can arrive before it ends).
deadline() {
    local start
    start=$(now_ms)
    [ "$blackout_end" -gt "$start" ] && start=$blackout_end
    echo $((start + $1 * 1000))
}

# wait_for SECONDS DESCRIPTION COMMAND...: poll COMMAND until it succeeds.
wait_for() {
    local limit
    limit=$(deadline "$1")
    local what=$2
    shift 2
    until "$@"; do
        if [ $mode = mosh ] && has_text '[client exited with status'; then
            why="client exited while waiting for $what: $(screen | grep -v '^$' | tail -3 | tr '\n' ' ')"
            screen > "$cell_dir/screen-at-exit.txt" 2> /dev/null
            return 1
        fi
        if [ "$(now_ms)" -gt "$limit" ]; then
            why="timed out waiting for $what"
            screen > "$cell_dir/screen-at-timeout.txt" 2> /dev/null
            return 1
        fi
        sleep 0.1
    done
}
has_text() { screen | grep -qF -- "$1"; }
lacks_text() { ! screen | grep -qF -- "$1"; }
line_has() { screen | sed -n "$1p" | grep -qF -- "$2"; }
wait_text() { wait_for 30 "\"$1\" on screen" has_text "$1"; }
wait_gone() { wait_for 30 "\"$1\" to leave the screen" lacks_text "$1"; }
wait_line() { wait_for 30 "\"$2\" on line $1" line_has "$1" "$2"; }

# Expected-screen bookkeeping. In the direct run, checkpoint NAME saves the
# settled screen; in a mosh run it waits until the screen equals that save.
settled_capture() {
    local a b
    a=$(screen $cap_flags)
    for _ in $(seq 50); do
        sleep 0.3
        b=$(screen $cap_flags)
        [ "$a" = "$b" ] && break
        a=$b
    done
    printf '%s\n' "$b"
}
# Both sides go through $(...), which drops trailing blank lines the same way.
screen_matches() {
    printf '%s\n' "$(screen $cap_flags)" > "$cell_dir/$1.actual.txt"
    cmp -s "$cell_dir/$1.actual.txt" "$expect_dir/$scen.$1.txt"
}
# match NAME SECONDS
match() {
    if [ $mode = direct ]; then
        settled_capture > "$expect_dir/$scen.$1.txt"
        return 0
    fi
    if wait_for "$2" "the $1 screen to match the direct run" screen_matches "$1"; then
        rm -f "$cell_dir/$1.actual.txt"
        return 0
    fi
    why="screen '$1' differs from the direct run"
    cp "$expect_dir/$scen.$1.txt" "$cell_dir/$1.expected.txt"
    diff -u --label "$1.expected (direct)" --label "$1.actual (mosh)" \
        "$cell_dir/$1.expected.txt" "$cell_dir/$1.actual.txt" > "$cell_dir/$1.diff"
    return 1
}
checkpoint() { match "$1" 30; }

drive() {
    if declare -F "drive_$scen" > /dev/null; then
        "drive_$scen" || { [ -n "$why" ] || why="drive_$scen failed"; return 1; }
    fi
}

# The scenario files define drive_<name> for the harness side.
for s in "${ALL_SCENARIOS[@]}"; do
    # shellcheck disable=SC1090
    INTEROP_SOURCED=1 . "$scen_dir/$s.sh"
done

cap_flags_for() { [ "$1" = colors ] && printf '%s' -e; }

# ---- Direct runs: the expected screens.

run_direct() {
    scen=$1 mode=direct sock=$sock_dir/d-$1 cell_dir=$run_dir/direct-$1
    expect_dir=$run_dir/expected blackout_end=0 why=
    cap_flags=$(cap_flags_for "$scen")
    mkdir -p "$cell_dir"
    tmux -f /dev/null -S "$sock" new-session -d -s interop -x 80 -y 24 \
        "cd && env TERM=xterm-256color LANG=C.UTF-8 INTEROP_CORRUPT='${INTEROP_CORRUPT_DIRECT:-}' bash '$scen_dir/$scen.sh'"
    local ok=1
    wait_text "READY-$scen" && keys Enter && drive && wait_text "DONE-$scen" && match final 0 || ok=0
    tmux -S "$sock" kill-server 2> /dev/null
    if [ $ok = 1 ]; then
        rm -rf "$cell_dir"
    else
        echo "FAIL direct run of $scen: $why (see $cell_dir)"
        return 1
    fi
}

# ---- One matrix cell, run in a background subshell.

run_cell() {
    local client=$1 server=$2 profile=$3 slot=$5
    scen=$4 mode=mosh sock=$sock_dir/m$slot expect_dir=$run_dir/expected
    blackout_end=0 why=
    cap_flags=$(cap_flags_for "$scen")
    cell_dir=$run_dir/$(bin_label "$client")-$(bin_label "$server")-$profile-$scen
    mkdir -p "$cell_dir"
    local start base listen_port
    start=$(now_ms)
    base=$((PORT_FIRST + slot * SLOT_PORTS))
    listen_port=$((base + 2))
    ctl_port=$((base + 3))

    cell_body "$client" "$server" "$profile"
    local rc=$?

    # Tear down: client, its tmux, proxy, server (its session gets SIGHUP).
    kill_client "$sock"
    if [ -f "$cell_dir/pids" ]; then
        # shellcheck disable=SC2046
        stop_pids $(cat "$cell_dir/pids")
        rm -f "$cell_dir/pids"
    fi
    local secs
    secs=$(awk -v ms=$(($(now_ms) - start)) 'BEGIN { printf "%.1f", ms / 1000 }')
    local line="client=$client server=$server profile=$profile scenario=$scen seconds=$secs"
    if [ $rc = 0 ]; then
        echo "PASS $line" | tee -a "$run_dir/results"
        rm -rf "$cell_dir"
    else
        echo "$why" > "$cell_dir/reason"
        echo "FAIL $line: $why" | tee -a "$run_dir/results"
    fi
}

cell_body() {
    local client_bin server_bin profile=$3 port key pid
    client_bin=$(bin_path "$1" client)
    server_bin=$(bin_path "$2" server)

    # Server. The parent prints MOSH CONNECT and exits; the detached server
    # keeps running. INTEROP_CORRUPT must never reach the mosh side.
    env -u INTEROP_CORRUPT LANG=C.UTF-8 timeout 20 "$server_bin" new -c 256 \
        -p "$base:$((base + 1))" -- bash "$scen_dir/$scen.sh" > "$cell_dir/server.out" 2>&1
    local status=$?
    read -r _ _ port key < <(grep -m1 '^MOSH CONNECT ' "$cell_dir/server.out")
    if [ -z "${key:-}" ]; then
        why="server printed no MOSH CONNECT line (exit status $status): $(head -c 200 "$cell_dir/server.out" | tr '\n' ' ')"
        return 1
    fi
    pid=$(ss -Hulpn "sport = :$port" | grep -o 'pid=[0-9]*' | head -1 | cut -d= -f2)
    if [ -z "$pid" ]; then
        why="nothing is bound to the server's UDP port $port"
        return 1
    fi
    echo "$pid" >> "$cell_dir/pids"

    # Proxy. Delay, jitter and reordering apply from the start; loss is
    # switched on below once the session is up (see the header comment).
    local flags=() loss=
    case $profile in
        loss5) loss=0.05 ;;
        loss30)
            loss=0.30
            flags=(--delay-ms 200 --jitter-ms 50 --reorder 0.10)
            ;;
    esac
    "$impair" --listen "127.0.0.1:$listen_port" --target "127.0.0.1:$port" \
        --control "127.0.0.1:$ctl_port" "${flags[@]}" > "$cell_dir/proxy.out" 2>&1 &
    echo $! >> "$cell_dir/pids"
    for _ in $(seq 50); do grep -qs '^udp-impair listen=' "$cell_dir/proxy.out" && break; sleep 0.1; done
    if ! grep -q '^udp-impair listen=' "$cell_dir/proxy.out"; then
        why="udp-impair did not start: $(head -c 200 "$cell_dir/proxy.out")"
        return 1
    fi

    # Client, in its own tmux server. If it exits, the pane says so and stays.
    tmux -f /dev/null -S "$sock" new-session -d -s interop -x 80 -y 24 \
        "env MOSH_KEY='$key' TERM=xterm-256color LANG=C.UTF-8 '$client_bin' 127.0.0.1 $listen_port; echo \"[client exited with status \$?]\"; exec sleep 1000000"

    wait_text "READY-$scen" || return 1
    local reply s2c_before=0
    if [ -n "$loss" ]; then
        reply=$(control "loss $loss")
        [ "$reply" = ok ] || { why="proxy refused loss $loss: $reply"; return 1; }
    fi
    keys Enter

    case $profile in
        blackout)
            sleep 0.2
            reply=$(control "blackout $BLACKOUT_MS")
            blackout_end=$(($(now_ms) + BLACKOUT_MS))
            [ "$reply" = ok ] || { why="proxy refused blackout: $reply"; return 1; }
            ;;
        rebind)
            sleep 0.2
            reply=$(control rebind)
            [[ $reply == "ok rebind"* ]] || { why="proxy refused rebind: $reply"; return 1; }
            s2c_before=$(s2c_forwarded)
            echo "$reply; s2c forwarded at rebind: $s2c_before" > "$cell_dir/rebind.txt"
            ;;
    esac

    drive || return 1
    match final 60 || return 1

    if [ "$profile" = blackout ] || [ "$profile" = rebind ]; then
        # The session must still be live: typed keys come back (tty echo).
        local token="alive-$profile"
        keys -l "$token"
        wait_for 10 "typed \"$token\" to echo after the $profile" has_text "$token" || return 1
    fi
    if [ "$profile" = rebind ]; then
        local s2c_after
        s2c_after=$(s2c_forwarded)
        if ! [ "${s2c_after:-0}" -gt "$s2c_before" ]; then
            why="no server-to-client packets reached the new proxy port after the rebind ($s2c_before before, ${s2c_after:-?} after)"
            return 1
        fi
    fi
    return 0
}

# ---- Main.

wall_start=$(now_ms)
echo "interop: clients=(${clients[*]}) servers=(${servers[*]}) profiles=(${profiles[*]}) scenarios=(${scenarios[*]}) blackout_ms=$BLACKOUT_MS jobs=$JOBS"

direct_pids=()
for s in "${scenarios[@]}"; do
    run_direct "$s" &
    direct_pids+=($!)
done
direct_failed=0
for p in "${direct_pids[@]}"; do wait "$p" || direct_failed=1; done
if [ $direct_failed = 1 ]; then
    echo "FAIL interop: a direct run failed, so there is no expected screen; the harness or this machine is broken"
    exit 1
fi

cells=()
for c in "${clients[@]}"; do
    for sv in "${servers[@]}"; do
        for p in "${profiles[@]}"; do
            for s in "${scenarios[@]}"; do
                cells+=("$c|$sv|$p|$s")
            done
        done
    done
done

touch "$run_dir/results"
slot_pid=()
next=0
while :; do
    running=0
    for ((slot = 0; slot < JOBS; slot++)); do
        pid=${slot_pid[slot]:-}
        if [ -n "$pid" ] && ! kill -0 "$pid" 2> /dev/null; then
            wait "$pid"
            pid=
            slot_pid[slot]=
        fi
        if [ -z "$pid" ] && [ $next -lt ${#cells[@]} ]; then
            IFS='|' read -r c sv p s <<< "${cells[next]}"
            run_cell "$c" "$sv" "$p" "$s" "$slot" &
            slot_pid[slot]=$!
            next=$((next + 1))
            pid=$!
        fi
        [ -n "$pid" ] && running=$((running + 1))
    done
    [ $running -eq 0 ] && [ $next -ge ${#cells[@]} ] && break
    sleep 0.2
done

pass=$(grep -c '^PASS' "$run_dir/results")
fail=$(grep -c '^FAIL' "$run_dir/results")
total=${#cells[@]}
wall=$(awk -v ms=$(($(now_ms) - wall_start)) 'BEGIN { printf "%.0f", ms / 1000 }')
echo "interop: $total cells, $pass passed, $fail failed, wall time ${wall}s"
if [ "$pass" -ne "$total" ]; then
    [ "$fail" -gt 0 ] && echo "interop: failure details in $run_dir/<cell>/"
    exit 1
fi
