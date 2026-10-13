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
# Ported from: (new; test tooling).
#
# Scrollback oracle (PLAN.md section 3 item 6, section 6).
#
# tests/scrollback.sh [VARIANT...]
# tests/scrollback.sh --list
# SCROLLBACK_VARIANTS=clean,vim tests/scrollback.sh
#
# VARIANT is one of clean loss30 vim clear altscreen resize shrink grow oldserver dropped (default:
# all of them, in that order). Variants come from the arguments, else from
# SCROLLBACK_VARIANTS (comma- or space-separated); `--list` prints the names,
# one per line. Each variant sets up and tears down its own session, so any
# subset runs alone (a CI shard per variant or group). Each runs a mosh-rs session in a fresh tmux
# server (80x24, history-limit 200000), which plays the host terminal, and
# then reads the host's history plus screen with `capture-pane -S -200000`:
#   clean     `mosh --scrollback localhost` (our launcher over ssh, our client
#             and server), `seq 1 50000`: every number exactly once, in
#             order, on consecutive lines.
#   loss30    the same through udp-impair (30% loss each way, 100 ms delay),
#             with `mosh-server new` started directly and the client given
#             MOSH_KEY and MOSH_SCROLLBACK=1 (the launcher's ssh step has no
#             loss to test).
#   vim       `seq 1 3000; vim; seq 3001 6000` on one command line: exact,
#             and nothing typed in vim reaches history (the server keeps the
#             rows on screen when vim enters the alternate screen, PLAN 6.2).
#   clear     `seq 1 100; clear; seq 101 3000`: exact (the server keeps the
#             rows `clear` erases, as tmux does).
#   resize    `seq 1 3000`, the window made wider and narrower, `seq 3001
#             6000`: exact.
#   shrink    `seq 1 3000` (the screen holds 2978-3000 and the prompt), the
#             window made 6 rows shorter, `seq 3001 6000`. Not exact, and
#             checked to be exactly this (PLAN 6.4): tmux itself moves the
#             top 6 rows (2978-2983) into its history when it shrinks, and
#             they come again when they scroll off; the server's emulator
#             cuts the bottom 6 rows (2996-3000 and the prompt), as
#             upstream's does, and puts the cursor on the row holding 2995,
#             column 3, where the next command line is echoed
#             ("29seq 3001 ..."). Everything else is exact.
#   grow      the same, the window made 6 rows taller: tmux pulls its last 6
#             history lines (2972-2977) back onto the screen, and the
#             repaint overwrites them, so exactly those are missing.
#   altscreen `seq 1 3000`, then vim (text typed, `:q!`), then less on a
#             3000-line file (`q`), then `seq 3001 6000`: while vim and less
#             run the host terminal is in its alternate screen; after each
#             exits the visible screen equals the one a plain tmux pane shows
#             for the same keys (the last rows of `seq 1 3000` and the
#             prompts, PLAN 6.6); history is exactly 1-6000 plus the command
#             lines, nothing from vim or less.
#   oldserver `mosh --scrollback --server=/usr/bin/mosh-server`: the client
#             stays in the alternate screen, shows the notice, and nothing
#             reaches the host's history.
#   dropped   a server with a 16 KiB backlog (`--scrollback 16K`), the
#             client cut off for 8 s during `seq`: the dropped lines are
#             replaced by one dim notice line with their count, in place, and
#             every other line is exact.
# Every variant also fails if mosh's notification bar text ("mosh:",
# "[To quit", ...) is found in the host's history.
#
# Output (stdout, so CI logs show it): one PASS/FAIL line per variant with
# the checker's summary; on FAIL also the reason, the checker's first
# unexpected lines, any screen diff, and the screen at the time; exit 1 if
# any failed. On FAIL, target/scrollback/<variant>/ keeps the capture.
# Waits are generous (a shared CI runner is several times slower than
# devbox) and history waits are by progress: they give up only after the
# captured history has stopped growing for a while.

set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root" || exit 1
out_dir=$root/target/scrollback
bin=$root/target/release
PORT_FIRST=61500
PORT_LAST=61599
ALL=(clean loss30 vim clear altscreen resize shrink grow oldserver dropped)

if [ "${1:-}" = --list ]; then
    printf '%s\n' "${ALL[@]}"
    exit 0
fi
variants=("$@")
if [ ${#variants[@]} -eq 0 ] && [ -n "${SCROLLBACK_VARIANTS:-}" ]; then
    read -r -a variants <<< "${SCROLLBACK_VARIANTS//,/ }"
fi
[ ${#variants[@]} -eq 0 ] && variants=("${ALL[@]}")
for v in "${variants[@]}"; do
    [[ " ${ALL[*]} " == *" $v "* ]] || { echo "scrollback: unknown variant $v (one of: ${ALL[*]})" >&2; exit 2; }
done

echo "scrollback: variants: ${variants[*]}"
missing=()
for cmd in tmux vim less ss ssh python3 flock cargo; do
    command -v $cmd > /dev/null || missing+=("$cmd")
done
ssh -o BatchMode=yes localhost true 2> /dev/null || missing+=("passwordless ssh to localhost")
# The shrink and grow expectations are tmux's resize behavior, measured with
# tmux 3.4; tmux before 3.2 resizes differently.
if command -v tmux > /dev/null; then
    tmux_version=$(tmux -V | grep -o '[0-9]*\.[0-9]*' | head -1)
    echo "scrollback: $(tmux -V)"
    if ! python3 -c 'import sys; sys.exit(0 if tuple(map(int, sys.argv[1].split("."))) >= (3, 2) else 1)' "${tmux_version:-0.0}" 2> /dev/null; then
        missing+=("tmux 3.2 or later (found $(tmux -V))")
    fi
fi
if [ ${#missing[@]} -ne 0 ]; then
    echo "FAIL scrollback: missing prerequisites: ${missing[*]}"
    exit 1
fi
cargo build --release -q -p mosh -p mosh-tools --bin mosh --bin mosh-client --bin mosh-server --bin udp-impair || exit 1

# Sessions use the shared lock and their own port range.
exec 9> /tmp/mosh-rs-interop.lock
if ! flock -n 9; then
    echo "scrollback: another run holds /tmp/mosh-rs-interop.lock, waiting"
    flock 9
fi

rm -rf "$out_dir"
mkdir -p "$out_dir"
# tmux socket paths must stay under 108 bytes, so they live in /tmp.
tmp=$(mktemp -d /tmp/scrollback.XXXXXX)
sock=$tmp/tmux-0.sock
printf 'set -g history-limit 200000\n' > "$tmp/tmux.conf"
proxy_pid=

kill_ours() {
    # Anything bound in our port range is ours (the lock guarantees it).
    ss -Hulpn 2> /dev/null | while read -r _ _ _ local _ users; do
        local port=${local##*:}
        if [ "$port" -ge $PORT_FIRST ] && [ "$port" -le $PORT_LAST ]; then
            grep -o 'pid=[0-9]*' <<< "$users" | cut -d= -f2
        fi
    done | sort -u | xargs -r kill -KILL 2> /dev/null
}
end_session() {
    local pane
    pane=$(tmux -S "$sock" list-panes -a -F '#{pane_pid}' 2> /dev/null)
    [ -n "$pane" ] && pkill -KILL -P "$pane"
    tmux -S "$sock" kill-server 2> /dev/null
    [ -n "$proxy_pid" ] && kill "$proxy_pid" 2> /dev/null
    proxy_pid=
    kill_ours
}
cleanup() {
    end_session
    rm -rf "$tmp"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

now_ms() { echo $((${EPOCHREALTIME/./} / 1000)); }
keys() { tmux -S "$sock" send-keys -t sb "$@"; }
screen() { tmux -S "$sock" capture-pane -p -t sb; }
history() { tmux -S "$sock" capture-pane -p -t sb -S -200000; }
control() {
    python3 -c '
import socket, sys
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(2)
s.sendto(sys.argv[2].encode(), ("127.0.0.1", int(sys.argv[1])))
print(s.recv(1024).decode())
' "$ctl_port" "$1"
}

# wait_line SECONDS TEXT: until a screen line is exactly TEXT.
wait_line() {
    local end=$(($(now_ms) + $1 * 1000))
    while [ "$(now_ms)" -lt $end ]; do
        screen | grep -qxF -- "$2" && return 0
        sleep 0.2
    done
    why="no line \"$2\" on screen within $1 s"
    return 1
}
# wait_text SECONDS TEXT: until TEXT is somewhere on the screen.
wait_text() {
    local end=$(($(now_ms) + $1 * 1000))
    while [ "$(now_ms)" -lt $end ]; do
        screen | grep -qF -- "$2" && return 0
        sleep 0.2
    done
    why="\"$2\" not on screen within $1 s"
    return 1
}

# The host's history check. check FILE LAST [exact|shrink|grow|dropped]: the run
# is the lines from the first line "1" to the line "DONE-SEQ"; it may hold
# echoed command lines ("$ ...") and, for dropped, notice lines. Prints a
# one-line summary; exits 1 if the variant's rule fails.
check() {
    python3 - "$@" << 'PY'
import re, sys
path, last, mode = sys.argv[1], int(sys.argv[2]), (sys.argv[3] if len(sys.argv) > 3 else "exact")
lines = [l.rstrip() for l in open(path, encoding="utf-8", errors="replace").read().split("\n")]
notice_re = re.compile(r"^\[mosh: (\d+) lines of history were dropped\]$")
forbidden = ["mosh:", "[To quit", "Nothing received", "Last contact", "Connecting",
             "Exiting", "does not support scrollback", "VIMTEXT"]
problems = []
for i, l in enumerate(lines):
    if notice_re.match(l):
        continue
    for f in forbidden:
        if f in l:
            problems.append("line %d holds %r: %r" % (i + 1, f, l))
try:
    start = lines.index("1")
    end = lines.index("DONE-SEQ", start)
except ValueError:
    print("no run from a line '1' to a line 'DONE-SEQ'; %d lines captured" % len(lines))
    sys.exit(1)
run = lines[start:end]
nums, prompts, notices, other = [], 0, [], []
expect = 1
for k, l in enumerate(run):
    if re.fullmatch(r"\d+", l):
        nums.append(int(l))
    elif l.startswith("$ "):
        prompts += 1
    elif notice_re.match(l):
        notices.append((len(nums), int(notice_re.match(l).group(1))))
    else:
        other.append((start + k + 1, l))
seen = set()
dups = sorted({n for n in nums if n in seen or seen.add(n)})
gaps = []
walk = 1
notice_at = dict(notices)
for k, n in enumerate(nums + [last + 1]):
    if k in notice_at:
        walk += notice_at[k]
    if n > walk:
        gaps.append((walk, n - 1))
    walk = max(walk, n + 1)
order_ok = all(a < b for a, b in zip(nums, nums[1:]))
def ranges(r):
    return ",".join("%d-%d" % g if g[0] != g[1] else "%d" % g[0] for g in r) or "none"
summary = ("lines=%d numbers=%d first=%s last=%s in_order=%s duplicates=%d%s gaps=%s notices=%s prompts=%d other=%d"
           % (len(lines), len(nums), nums[0] if nums else "-", nums[-1] if nums else "-", order_ok,
              len(dups), (" (" + ranges([(d, d) for d in dups[:20]]) + ")") if dups else "",
              ranges(gaps), ["%d before #%d" % (n, k) for k, n in notices] or "none", prompts, len(other)))
print(summary)
for p in problems[:5] + ["unexpected line %d: %r" % o for o in other[:5]]:
    print("  " + p)
ok = not problems and not other
if mode == "shrink":
    ok = not problems and dups == list(range(2978, 2984)) and gaps == [(2995, 3000)] \
        and not notices and [l for _, l in other] == ["29seq 3001 6000; echo DONE-SEQ"]
elif mode == "exact":
    ok = ok and not dups and not gaps and order_ok and not notices
elif mode == "grow":
    ok = ok and not dups and order_ok and not notices and gaps == [(2972, 2977)]
elif mode == "dropped":
    ok = ok and not dups and not gaps and order_ok and len(notices) >= 1
sys.exit(0 if ok else 1)
PY
}

# wait_history STALL LAST MODE: poll the history until the check passes.
# Gives up when the captured history has not grown for STALL seconds (or
# after 30 minutes in all); keeps the last capture in $cap.
wait_history() {
    local start now grew lines last=-1
    start=$(now_ms)
    grew=$start
    while :; do
        history > "$cap"
        summary=$(check "$cap" "$2" "$3") && return 0
        now=$(now_ms)
        lines=$(wc -l < "$cap")
        if [ "$lines" -ne "$last" ]; then
            last=$lines
            grew=$now
        fi
        [ $((now - grew)) -ge $(($1 * 1000)) ] && break
        [ $((now - start)) -ge 1800000 ] && break
        sleep 1
    done
    why="history not right after $(((now - start) / 1000)) s (no growth for $(((now - grew) / 1000)) s, $lines lines)"
    return 1
}

# new_tmux COMMAND: a fresh tmux server (80x24) running COMMAND, on a new
# socket each time: a server just killed may still hold the old socket, and
# a new session there fails with "server exited unexpectedly".
tmux_count=0
new_tmux() {
    local err
    tmux_count=$((tmux_count + 1))
    sock=$tmp/tmux-$tmux_count.sock
    if ! err=$(tmux -f "$tmp/tmux.conf" -S "$sock" new-session -d -s sb -x 80 -y 24 "$1" 2>&1); then
        why="tmux new-session failed: $err"
        return 1
    fi
}

# Starts the client through our launcher (over ssh to localhost).
start_launcher() {
    new_tmux \
        "env LANG=C.UTF-8 '$bin/mosh' --scrollback --server='$bin/mosh-server' $* --client='$bin/mosh-client' -p $PORT_FIRST:$((PORT_FIRST + 9)) localhost -- sh; echo '[client exited with status '\$?']'; exec sleep 1000000"
}

# Starts `mosh-server new` directly and the client through udp-impair.
# start_direct 'SERVER ARGS' 'PROXY ARGS'
start_direct() {
    local port key
    env LANG=C.UTF-8 "$bin/mosh-server" new $1 -p $((PORT_FIRST + 10)):$((PORT_FIRST + 19)) -- sh > "$tmp/server.out" 2>&1
    read -r _ _ port key < <(grep -m1 '^MOSH CONNECT ' "$tmp/server.out")
    [ -n "${key:-}" ] || { why="server printed no MOSH CONNECT: $(head -c 200 "$tmp/server.out")"; return 1; }
    listen_port=$((PORT_FIRST + 20))
    ctl_port=$((PORT_FIRST + 21))
    "$bin/udp-impair" --listen "127.0.0.1:$listen_port" --target "127.0.0.1:$port" \
        --control "127.0.0.1:$ctl_port" $2 > "$tmp/proxy.out" 2>&1 &
    proxy_pid=$!
    for _ in $(seq 50); do grep -qs '^udp-impair listen=' "$tmp/proxy.out" && break; sleep 0.1; done
    new_tmux \
        "env MOSH_KEY='$key' MOSH_SCROLLBACK=1 LANG=C.UTF-8 '$bin/mosh-client' 127.0.0.1 $listen_port; echo '[client exited with status '\$?']'; exec sleep 1000000"
}

# The remote shell is sh, whose prompt is "$ ".
v_clean() {
    start_launcher || return 1
    wait_line 120 '$' || return 1
    keys 'seq 1 50000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    # The escape key's help bar on the top row while history is injected
    # there; then any key ends it (sh gets Ctrl-^ and a backspace).
    keys C-^
    wait_text 60 'Commands: Ctrl-Z suspends' || return 1
    sleep 3
    keys BSpace
    wait_history 60 50000 exact
}

v_loss30() {
    start_direct "" "--loss 0.30 --delay-ms 100" || return 1
    wait_line 180 '$' || return 1
    keys 'seq 1 50000; echo DONE-SEQ' Enter
    wait_line 600 DONE-SEQ || return 1
    wait_history 120 50000 exact
}

v_vim() {
    start_launcher || return 1
    wait_line 120 '$' || return 1
    keys 'seq 1 3000; vim -u NONE -N; seq 3001 6000; echo DONE-SEQ' Enter
    wait_text 120 '~' || return 1
    keys i 'VIMTEXT one' Enter 'VIMTEXT two' Escape
    wait_text 60 'VIMTEXT two' || return 1
    keys ':q!' Enter
    wait_line 120 DONE-SEQ || return 1
    wait_history 60 6000 exact
}

v_clear() {
    start_launcher || return 1
    wait_line 120 '$' || return 1
    keys 'seq 1 100; clear; seq 101 3000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    wait_history 60 3000 exact
}

v_resize() {
    start_launcher || return 1
    wait_line 120 '$' || return 1
    keys 'seq 1 3000' Enter
    wait_line 120 3000 || return 1
    tmux -S "$sock" resize-window -t sb -x 100 -y 24
    sleep 1
    tmux -S "$sock" resize-window -t sb -x 60 -y 24
    sleep 1
    keys 'seq 3001 6000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    wait_history 60 6000 exact
}

v_shrink() {
    start_launcher || return 1
    wait_line 120 '$' || return 1
    keys 'seq 1 3000' Enter
    wait_line 120 3000 || return 1
    wait_last 120 '$' || return 1
    wait_caught_up 60 2978 || return 1
    tmux -S "$sock" resize-window -t sb -x 80 -y 18
    # the server applied the resize: its screen ends at 2995, the cursor's row
    wait_last 120 2995 || return 1
    keys 'seq 3001 6000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    wait_history 60 6000 shrink
}

v_grow() {
    start_launcher || return 1
    wait_line 120 '$' || return 1
    keys 'seq 1 3000' Enter
    wait_line 120 3000 || return 1
    wait_last 120 '$' || return 1
    wait_caught_up 60 2978 || return 1
    tmux -S "$sock" resize-window -t sb -x 80 -y 30
    # the repaint after the server applied the resize starts at 2978 (tmux
    # first shows the history lines it pulled back)
    wait_first 120 2978 || return 1
    keys 'seq 3001 6000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    wait_history 60 6000 grow
}

# wait_last SECONDS TEXT: until the screen's last non-empty line is TEXT.
wait_last() {
    local end=$(($(now_ms) + $1 * 1000))
    while [ "$(now_ms)" -lt $end ]; do
        [ "$(screen | sed '/^$/d' | tail -1)" = "$2" ] && return 0
        sleep 0.2
    done
    why="last screen line is not \"$2\" within $1 s: \"$(screen | sed '/^$/d' | tail -1)\""
    return 1
}
# wait_screen SECONDS FILE: until the screen equals FILE.
wait_screen() {
    local end=$(($(now_ms) + $1 * 1000))
    while [ "$(now_ms)" -lt $end ]; do
        screen | cmp -s - "$2" && return 0
        sleep 0.2
    done
    screen > "$2.got"
    why="screen differs from $(basename "$2"): $(diff "$2" "$2.got" | head -6 | tr '\n' '|')"
    return 1
}
# wait_caught_up STALL N: until the host's history holds 1 .. N-1 right
# before the screen's N, i.e. history has caught up with the screen (it
# trickles behind it). Gives up when the history stops growing for STALL s.
wait_caught_up() {
    local before lines last=-1 grew
    grew=$(now_ms)
    while :; do
        history > "$cap"
        before=$(grep -x -B1 -m1 -- "$2" "$cap" | head -1)
        [ "$before" = $(($2 - 1)) ] && grep -qx 1 "$cap" && return 0
        lines=$(wc -l < "$cap")
        if [ "$lines" -ne "$last" ]; then
            last=$lines
            grew=$(now_ms)
        fi
        [ $(($(now_ms) - grew)) -ge $(($1 * 1000)) ] && break
        sleep 0.5
    done
    why="history did not catch up with the screen (line before $2 is \"$before\")"
    return 1
}

# wait_alt SECONDS WANT WHAT: until the pane's alternate_on is WANT (the
# program, or mosh for it, has entered or left the alternate screen).
wait_alt() {
    local end=$(($(now_ms) + $1 * 1000)) a
    while [ "$(now_ms)" -lt $end ]; do
        a=$(tmux -S "$sock" display -p -t sb '#{alternate_on}')
        [ "$a" = "$2" ] && return 0
        sleep 0.2
    done
    why="pane alternate_on=$a, expected $2 ($3) within $1 s"
    return 1
}
# wait_first SECONDS TEXT: until the screen's first line is TEXT.
wait_first() {
    local end=$(($(now_ms) + $1 * 1000))
    while [ "$(now_ms)" -lt $end ]; do
        [ "$(screen | head -1)" = "$2" ] && return 0
        sleep 0.2
    done
    why="first screen line is not \"$2\" within $1 s: \"$(screen | head -1)\""
    return 1
}
# The keys of the altscreen variant. mode=direct saves the screens after
# vim and after less in $dir/direct-*.txt; mode=mosh waits for the same.
alt_steps() {
    wait_line 120 '$' || return 1
    keys 'seq 1 3000' Enter
    # The prompt after seq, not the first one: a command typed while seq
    # still runs is echoed into seq's output and the shell reads it ahead.
    wait_line 120 3000 || return 1
    wait_last 120 '$' || return 1
    keys 'vim -u NONE -N' Enter
    wait_alt 120 1 "vim started" || return 1
    wait_text 120 '~' || return 1
    keys i 'VIMTEXT one' Escape
    wait_text 60 'VIMTEXT one' || return 1
    keys ':q!' Enter
    if [ "$mode" = direct ]; then
        wait_last 120 '$' || return 1
        sleep 0.5
        screen > "$dir/direct-vim.txt"
    else
        wait_screen 120 "$dir/direct-vim.txt" || return 1
    fi
    wait_alt 120 0 "vim exited" || return 1
    keys "less $tmp/less.txt" Enter
    # less discards keys typed before it starts: wait for its first page
    wait_alt 120 1 "less started" || return 1
    wait_first 120 1 || return 1
    wait_last 120 "$tmp/less.txt" || return 1
    keys q
    if [ "$mode" = direct ]; then
        wait_last 120 '$' || return 1
        sleep 0.5
        screen > "$dir/direct-less.txt"
    else
        wait_screen 120 "$dir/direct-less.txt" || return 1
    fi
    wait_alt 120 0 "less exited"
}

v_altscreen() {
    seq 1 3000 > "$tmp/less.txt"
    # The reference: the same keys in a plain tmux pane (as over ssh), with
    # the TERM mosh-server gives the remote shell.
    mode=direct
    new_tmux "env TERM=xterm-256color LANG=C.UTF-8 sh; exec sleep 1000000" || return 1
    alt_steps || { why="direct run: $why"; return 1; }
    end_session
    mode=mosh
    start_launcher || return 1
    alt_steps || return 1
    keys 'seq 3001 6000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    wait_history 60 6000 exact
}

v_oldserver() {
    start_launcher --server=/usr/bin/mosh-server || return 1
    wait_text 120 'does not support scrollback' || return 1
    local alt
    alt=$(tmux -S "$sock" display -p -t sb '#{alternate_on}')
    [ "$alt" = 1 ] || { why="client is not in the alternate screen (alternate_on=$alt)"; return 1; }
    wait_line 120 '$' || return 1
    keys 'seq 1 3000; echo DONE-SEQ' Enter
    wait_line 120 DONE-SEQ || return 1
    screen > "$dir/screen.txt"
    keys exit Enter
    wait_text 120 '[client exited with status 0]' || return 1
    history > "$cap"
    # Nothing of the session may be in the host's main screen or history.
    if grep -qxE '[0-9]+|DONE-SEQ' "$cap" || grep -qF -e 'mosh:' -e 'scrollback' "$cap"; then
        why="the session reached the host's main screen: $(grep -cxE '[0-9]+' "$cap") number lines"
        return 1
    fi
    summary="alternate_on=1, notice shown, $(wc -l < "$cap") lines on the main screen after exit, none from the session"
}

v_dropped() {
    start_direct "--scrollback 16K" "" || return 1
    wait_line 120 '$' || return 1
    # The blackout must cover the flood: it starts at most a few seconds
    # after 1000 shows, the flood 10 s after that, and it lasts 20 s.
    keys 'seq 1 1000; sleep 10; seq 1001 200000; echo DONE-SEQ' Enter
    wait_line 120 1000 || return 1
    local reply
    reply=$(control "blackout 20000")
    [ "$reply" = ok ] || { why="proxy refused blackout: $reply"; return 1; }
    sleep 20
    wait_line 180 DONE-SEQ || return 1
    wait_history 60 200000 dropped || return 1
    # The notice is dim (SGR 2), so it reads as mosh's, not the program's.
    local notice
    notice=$(tmux -S "$sock" capture-pane -p -e -t sb -S -200000 | grep -F 'lines of history were dropped')
    if [[ $notice != *$'\e[2m'* ]]; then
        why="the notice line is not dim: $(cat -v <<< "$notice")"
        return 1
    fi
    summary="$summary; notice: $(cat -v <<< "$notice")"
}

failed=0
for v in "${variants[@]}"; do
    dir=$out_dir/$v
    mkdir -p "$dir"
    cap=$dir/capture.txt
    why=
    summary=
    start=$(now_ms)
    if "v_$v"; then
        echo "PASS $v ($(( ($(now_ms) - start) / 1000 )) s): $summary"
        rm -rf "$dir"
    else
        failed=1
        [ -s "$cap" ] || history > "$cap" 2> /dev/null
        [ -n "$why" ] || why="a step failed without a recorded reason (screen below)"
        echo "FAIL $v ($(( ($(now_ms) - start) / 1000 )) s): $why"
        [ -n "$summary" ] && [[ $why != *"$summary"* ]] && sed 's/^/  /' <<< "$summary"
        for got in "$dir"/*.got; do
            [ -f "$got" ] || continue
            echo "  screen diff, expected ($(basename "${got%.got}")) vs shown:"
            diff "${got%.got}" "$got" | head -20 | sed 's/^/  | /'
        done
        echo "  screen at the failure:"
        screen 2> /dev/null | sed 's/^/  | /'
        echo "  (history capture: $cap, $(wc -l < "$cap" 2> /dev/null || echo 0) lines)"
    fi
    end_session
done
exit $failed
