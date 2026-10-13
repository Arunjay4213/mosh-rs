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
# Session setup under loss. The interop matrix (tests/interop.sh) switches
# loss on only after the session is up, because at 30% loss from the first
# packet a client sometimes gives up by chance (upstream stmclient.cc: no
# state from the server for 15 s, "Timed out waiting for server..."). This
# driver covers setup itself, statistically, against the C++ pair.
#
# Part 1, setups: for the pairs cxx client x cxx server (the baseline), rust
# client x cxx server and cxx client x rust server, SETUP_TRIALS (40) setups
# each, through udp-impair with --loss 0.30 --delay-ms 200 --jitter-ms 50
# --reorder 0.10 from the first packet, each with a 20 s budget. A setup
# succeeds when the session command's READY line is on the client's screen.
# Recorded per setup: success, time from client start to that screen, time
# to the first server packet delivered to the client ("first contact"), and
# client packets sent (forwarded plus dropped by the proxy) before it.
#
# Part 2, send rate with no contact: each client, BLACKHOLE_TRIALS (5)
# times, against a proxy that drops everything (--loss 1.0) for 12 s.
# Counts the packets the client sends while it has never heard from the
# server, the same window for both clients. Measured on devbox, both
# clients send one empty packet every 3 s before contact (upstream
# ACK_INTERVAL), and at 15 s give up and send a burst of shutdown packets.
# The window ends before that give-up so the burst cannot land in some
# runs and not others.
#
# Pass rule:
# - Setup success. For each rust pair, a one-sided Fisher exact test asks
#   whether its failures out of 40 are more than the baseline's by more
#   than chance. FAIL if p < 0.05. Fisher's test is exact for small counts
#   and needs no assumed failure rate, which is unknown here (it depends on
#   the loss draw). When both pairs truly fail equally often it fails the
#   gate at most 5% of the time. For scale: with 0 baseline failures it
#   allows up to 4 rust failures (p = 0.058 at 4, 0.027 at 5); with 2
#   baseline failures, up to 7.
# - Send rate. The median packet count of the rust client in part 2 must be
#   at least the C++ client's. The client's timers decide this count, so
#   it hardly varies between runs.
# Per-pair medians of setup time and packets before first contact are
# printed as well.
#
# UDP ports 61000-61199 (4 per parallel slot) and the same lock file as
# tests/interop.sh. Runs with `--run`, or with no arguments when
# INTEROP_FULL=1 (so tests/run-all.sh runs it only then).
#
# Environment: SETUP_TRIALS (default 40), BLACKHOLE_TRIALS (default 5),
# INTEROP_JOBS (default 10, max 50).

set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root" || exit 1

# tests/run-all.sh runs every tests/*.sh with no arguments; this one takes a
# few minutes, so it runs only with --run or INTEROP_FULL=1.
case ${1:-} in
    --run) ;;
    "")
        if [ "${INTEROP_FULL:-0}" != 1 ]; then
            echo "interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)"
            exit 0
        fi
        ;;
    *)
        echo "usage: tests/interop-setup-loss.sh [--run]" >&2
        exit 2
        ;;
esac

TRIALS=${SETUP_TRIALS:-40}
BH_TRIALS=${BLACKHOLE_TRIALS:-5}
JOBS=${INTEROP_JOBS:-10}
BUDGET=20
BH_WINDOW=12
PORT_FIRST=61000
PORT_LAST=61199
SLOT_PORTS=4
LOSS_FLAGS=(--loss 0.30 --delay-ms 200 --jitter-ms 50 --reorder 0.10)
impair=$root/target/release/udp-impair
work=$root/target/interop-setup-loss
# upstream_bin NAME: the real mosh 1.4.0 program NAME from PATH (apt puts it
# in /usr/bin, Homebrew in /opt/homebrew/bin), never a mosh-rs build; falls
# back to /usr/bin/NAME so a missing install fails visibly.
upstream_bin() {
    local dir
    local IFS=:
    for dir in $PATH; do
        if [ -x "$dir/$1" ] && ! "$dir/$1" --version 2>/dev/null | grep -q mosh-rs; then
            echo "$dir/$1"
            return 0
        fi
    done
    echo "/usr/bin/$1"
}

cxx_client=$(upstream_bin mosh-client)
cxx_server=$(upstream_bin mosh-server)
rust_client=$root/target/release/mosh-client
rust_server=$root/target/release/mosh-server

[ "$JOBS" -ge 1 ] && [ "$JOBS" -le $(((PORT_LAST - PORT_FIRST + 1) / SLOT_PORTS)) ] || {
    echo "interop-setup-loss: bad INTEROP_JOBS" >&2
    exit 2
}
for cmd in tmux ss python3 flock cargo; do
    command -v $cmd > /dev/null || { echo "FAIL interop-setup-loss: missing $cmd"; exit 1; }
done
cargo build --release -q -p mosh-tools --bin udp-impair -p mosh || exit 1

exec 9> /tmp/mosh-rs-interop.lock
if ! flock -n 9; then
    echo "interop-setup-loss: another interop run holds /tmp/mosh-rs-interop.lock, waiting"
    flock 9
fi

rm -rf "$work"
mkdir -p "$work"
sock_dir=$(mktemp -d /tmp/setuploss.XXXXXX)

cleanup() {
    local pids s
    pids=$(jobs -p)
    [ -n "$pids" ] && kill $pids 2> /dev/null
    for s in "$sock_dir"/*; do
        [ -S "$s" ] && tmux -S "$s" kill-server 2> /dev/null
    done
    # Anything left in our port range is ours (we hold the lock).
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
# and the next trial on the same slot would find the port busy.
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
# reach the next trial's proxy on the same port, inflate its packet counts,
# and make it send the server's replies to that dead client.
kill_client() {
    local pane
    pane=$(tmux -S "$1" list-panes -a -F '#{pane_pid}' 2> /dev/null)
    [ -n "$pane" ] && pkill -KILL -P "$pane"
    tmux -S "$1" kill-server 2> /dev/null
}

# monitor CTL_PORT SOCKET BUDGET_S MODE CLIENT_COMMAND
# Starts the client in a fresh tmux server and watches the proxy counters
# (every 20 ms) and the screen (every 100 ms). MODE setup stops at READY on
# screen; MODE window runs for the whole budget. Prints one line of fields.
monitor() {
    python3 - "$@" << 'EOF'
import socket, subprocess, sys, time
ctl, sock, budget, mode, cmd = int(sys.argv[1]), sys.argv[2], float(sys.argv[3]), sys.argv[4], sys.argv[5]
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(1)
def stats():
    s.sendto(b"stats", ("127.0.0.1", ctl))
    f = s.recv(1024).decode().split()
    # c2s forwarded=A dropped=B reordered=C s2c forwarded=D dropped=E reordered=F
    v = {}
    d = None
    for w in f:
        if w in ("c2s", "s2c"):
            d = w
        elif "=" in w:
            k, n = w.split("=")
            v[d + "_" + k] = int(n)
    return v
def screen():
    return subprocess.run(["tmux", "-S", sock, "capture-pane", "-p", "-t", "t"],
                          capture_output=True, text=True).stdout
subprocess.run(["tmux", "-f", "/dev/null", "-S", sock, "new-session", "-d", "-s", "t",
                "-x", "80", "-y", "24", cmd], check=True)
t0 = time.monotonic()
contact_ms = c2s_at_contact = setup_ms = None
last_screen = 0.0
v = {}
while True:
    now = time.monotonic()
    el = now - t0
    if el > budget:
        break
    v = stats()
    if contact_ms is None and v["s2c_forwarded"] > 0:
        contact_ms = round(el * 1000)
        c2s_at_contact = v["c2s_forwarded"] + v["c2s_dropped"]
    if mode == "setup" and now - last_screen >= 0.1:
        last_screen = now
        if "READY-setup" in screen():
            setup_ms = round(el * 1000)
            if contact_ms is None:
                # Contact and the first frame fell between two samples.
                v = stats()
                contact_ms = setup_ms
                c2s_at_contact = v["c2s_forwarded"] + v["c2s_dropped"]
            break
    time.sleep(0.02)
c2s_total = v["c2s_forwarded"] + v["c2s_dropped"]
print("ok=%d setup_ms=%s contact_ms=%s c2s_before_contact=%s c2s_total=%d"
      % (setup_ms is not None, setup_ms if setup_ms is not None else "-",
         contact_ms if contact_ms is not None else "-",
         c2s_at_contact if c2s_at_contact is not None else "-", c2s_total))
EOF
}

# trial KIND PAIR CLIENT_BIN SERVER_BIN SLOT INDEX
# KIND setup: real server, lossy proxy. KIND blackhole: no server needed
# (the proxy drops everything), a random key.
trial() {
    local kind=$1 pair=$2 client_bin=$3 server_bin=$4 slot=$5 idx=$6
    local base=$((PORT_FIRST + slot * SLOT_PORTS))
    local listen=$((base + 2)) ctl=$((base + 3)) sock=$sock_dir/s$slot
    local dir=$work/$pair-$idx port key spid='' ppid='' result
    mkdir -p "$dir"
    if [ "$kind" = setup ]; then
        LANG=C.UTF-8 timeout 20 "$server_bin" new -c 256 -p "$base:$((base + 1))" \
            -- bash -c 'echo READY-setup; exec sleep 1000000' > "$dir/server.out" 2>&1
        read -r _ _ port key < <(grep -m1 '^MOSH CONNECT ' "$dir/server.out")
        if [ -z "${key:-}" ]; then
            echo "pair=$pair i=$idx error=server-no-connect" >> "$work/results"
            return
        fi
        spid=$(ss -Hulpn "sport = :$port" | grep -o 'pid=[0-9]*' | head -1 | cut -d= -f2)
        "$impair" --listen "127.0.0.1:$listen" --target "127.0.0.1:$port" \
            --control "127.0.0.1:$ctl" "${LOSS_FLAGS[@]}" > "$dir/proxy.out" 2>&1 &
    else
        key=$(head -c 16 /dev/urandom | base64 | tr -d '=')
        "$impair" --listen "127.0.0.1:$listen" --target "127.0.0.1:$((base + 1))" \
            --control "127.0.0.1:$ctl" --loss 1.0 > "$dir/proxy.out" 2>&1 &
    fi
    ppid=$!
    for _ in $(seq 50); do grep -qs '^udp-impair listen=' "$dir/proxy.out" && break; sleep 0.1; done
    local budget=$BUDGET mode=setup
    [ "$kind" = blackhole ] && budget=$BH_WINDOW mode=window
    result=$(monitor "$ctl" "$sock" "$budget" "$mode" \
        "env MOSH_KEY='$key' TERM=xterm-256color LANG=C.UTF-8 '$client_bin' 127.0.0.1 $listen; echo \"[client exited with status \$?]\"; exec sleep 1000000")
    tmux -S "$sock" capture-pane -p -t t > "$dir/screen.txt" 2> /dev/null
    kill_client "$sock"
    stop_pids $ppid $spid
    wait $ppid 2> /dev/null
    echo "kind=$kind pair=$pair i=$idx ${result:-error=monitor}" >> "$work/results"
}

# Interleave the pairs so all of them see the same machine load.
tasks=()
for ((i = 1; i <= TRIALS; i++)); do
    tasks+=("setup cxx-cxx $cxx_client $cxx_server $i")
    tasks+=("setup rust-cxx $rust_client $cxx_server $i")
    tasks+=("setup cxx-rust $cxx_client $rust_server $i")
done
for ((i = 1; i <= BH_TRIALS; i++)); do
    tasks+=("blackhole cxx-client $cxx_client - $i")
    tasks+=("blackhole rust-client $rust_client - $i")
done

echo "interop-setup-loss: $TRIALS setups per pair with ${LOSS_FLAGS[*]} from the first packet, ${BUDGET} s budget; $BH_TRIALS x ${BH_WINDOW} s blackhole per client; jobs=$JOBS"
start=$(date +%s)
touch "$work/results"
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
        if [ -z "$pid" ] && [ $next -lt ${#tasks[@]} ]; then
            read -r kind pair cbin sbin idx <<< "${tasks[next]}"
            trial "$kind" "$pair" "$cbin" "$sbin" "$slot" "$idx" &
            slot_pid[slot]=$!
            pid=$!
            next=$((next + 1))
        fi
        [ -n "$pid" ] && running=$((running + 1))
    done
    [ $running -eq 0 ] && [ $next -ge ${#tasks[@]} ] && break
    sleep 0.2
done
echo "interop-setup-loss: wall time $(($(date +%s) - start)) s, raw results in $work/results"

python3 - "$work/results" "$TRIALS" "$BH_WINDOW" << 'EOF'
import math, statistics, sys
rows = []
for line in open(sys.argv[1]):
    rows.append(dict(w.split("=", 1) for w in line.split()))
n = int(sys.argv[2])
window = int(sys.argv[3])
def med(xs):
    return statistics.median(xs) if xs else float("nan")
def fisher_greater(a_fail, b_fail, n):
    # P(rust failures >= a_fail | total failures, equal rates), hypergeometric.
    k = a_fail + b_fail
    total = math.comb(2 * n, k)
    return sum(math.comb(n, x) * math.comb(n, k - x) for x in range(a_fail, min(k, n) + 1)) / total
ok = True
fails = {}
for pair in ("cxx-cxx", "rust-cxx", "cxx-rust"):
    r = [x for x in rows if x.get("pair") == pair]
    succ = [x for x in r if x.get("ok") == "1"]
    errors = [x for x in r if "error" in x]
    fails[pair] = len(r) - len(succ)
    setup = [int(x["setup_ms"]) for x in succ]
    pkts = [int(x["c2s_before_contact"]) for x in r if x.get("c2s_before_contact", "-") != "-"]
    contact = [int(x["contact_ms"]) for x in r if x.get("contact_ms", "-") != "-"]
    print("pair=%-8s client x server, trials=%d success=%d/%d (%.0f%%) errors=%d median_setup_ms=%.0f "
          "median_contact_ms=%.0f median_packets_before_contact=%.1f (over %d setups with contact)"
          % (pair, len(r), len(succ), len(r), 100.0 * len(succ) / max(len(r), 1), len(errors),
             med(setup), med(contact), med(pkts), len(pkts)))
    if len(r) != n or errors:
        print("FAIL %s: %d of %d trials ran, %d errors" % (pair, len(r), n, len(errors)))
        ok = False
for pair in ("rust-cxx", "cxx-rust"):
    p = fisher_greater(fails[pair], fails["cxx-cxx"], n)
    verdict = "PASS" if p >= 0.05 else "FAIL"
    if p < 0.05:
        ok = False
    print("%s setup-success %s: %d failures vs %d for cxx-cxx, one-sided Fisher p=%.3f (fail if < 0.05)"
          % (verdict, pair, fails[pair], fails["cxx-cxx"], p))
bh = {}
for c in ("cxx-client", "rust-client"):
    r = [int(x["c2s_total"]) for x in rows if x.get("pair") == c and "c2s_total" in x]
    bh[c] = med(r)
    print("blackhole %s: packets sent in %d s with no contact: %s (median %.1f)" % (c, window, sorted(r), bh[c]))
verdict = "PASS" if bh["rust-client"] >= bh["cxx-client"] else "FAIL"
if verdict == "FAIL":
    ok = False
print("%s send-rate: rust client median %.1f packets vs cxx client %.1f in the same %d s with no contact (need >=)"
      % (verdict, bh["rust-client"], bh["cxx-client"], window))
print("interop-setup-loss: %s" % ("PASS" if ok else "FAIL"))
sys.exit(0 if ok else 1)
EOF
