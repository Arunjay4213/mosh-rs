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
# Records the session corpus in tests/corpus/sessions/ with tools/pty-record
# and regenerates tests/corpus/sessions/README.md from the .meta sidecars.
# Usage: tools/corpus/record-all.sh [SCENARIO...]   (default: all)

set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$root"
cargo build --release -p mosh-tools --bin pty-record
rec="$root/target/release/pty-record"
scripts="$root/tools/corpus"
out="$root/tests/corpus/sessions"
mkdir -p "$out"

# Recordings must not depend on the terminal this script runs in.
# Also drop markers of a parent Claude Code session (when an agent runs this),
# so the claude recording looks like a user starting it from a plain shell.
unset TMUX TMUX_PANE
for v in $(env | sed -n 's/^\(CLAUDECODE\|CLAUDE_[A-Z_]*\)=.*/\1/p'); do unset "$v"; done
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# Runs in $work unless $dir is set.
record() {
    local name=$1; shift
    echo "== $name"
    (cd "${dir:-$work}" && "$rec" --cols 80 --rows 24 --out "$out/$name.bin" \
        --script "$scripts/$name.script" -- "$@")
}

all=(bash-colors vim less htop tmux-inside claude-tui seq100k colors256-truecolor wide-emoji resize)
wanted=("${@:-${all[@]}}")

for name in "${wanted[@]}"; do
    case $name in
    bash-colors|colors256-truecolor|wide-emoji|resize)
        record "$name" bash --norc --noprofile -i ;;
    claude-tui)
        # In a new directory claude only shows its "trust this folder?" dialog.
        # The repo root is already trusted, so the main interface comes up.
        dir=$root record claude-tui bash --norc --noprofile -i ;;
    vim)
        rm -f "$work/vim-corpus.txt"
        record vim vim -u NONE -N "$work/vim-corpus.txt" ;;
    less)
        record less bash -c 'man less | less' ;;
    htop)
        record htop htop ;;
    tmux-inside)
        record tmux-inside tmux -f /dev/null -L corpus new-session ;;
    seq100k)
        record seq100k seq 1 100000
        cap=$((2 * 1024 * 1024))
        if [ "$(stat -c %s "$out/seq100k.bin")" -gt $cap ]; then
            truncate -s $cap "$out/seq100k.bin"
            echo "seq100k.bin capped at $cap bytes" >> "$out/seq100k.bin.meta"
        fi ;;
    *)
        echo "unknown scenario $name" >&2; exit 2 ;;
    esac
done

# README table from the sidecars.
{
    cat <<'HEAD'
# Session corpus

Raw output of real programs running under a pseudoterminal, recorded on devbox (arm64 Ubuntu 24.04) by `tools/pty-record`.
Each `.bin` holds every byte the program wrote to the pty, with no framing.
Each `.bin.meta` holds the command, starting size, byte count, duration and exit status.
Recordings start at 80x24 with `TERM=xterm-256color` and `LANG=C.UTF-8`.
The scripted keystrokes for each file are in `tools/corpus/NAME.script`.
To re-record: `tools/corpus/record-all.sh [NAME...]` (this rewrites this README).
Content depends on the machine (file listings, process lists, dates), so re-recording changes the bytes.

These files are conformance inputs for the terminal emulator oracle (PLAN.md section 3, oracle 3).

| file | command | size (bytes) | duration (ms) |
|------|---------|-------------:|--------------:|
HEAD
    for meta in "$out"/*.bin.meta; do
        f=$(basename "$meta" .meta)
        cmd=$(sed -n 's/^command: //p' "$meta" | sed "s#$work#\$TMP#g; s/|/\\\\|/g")
        bytes=$(stat -c %s "$out/$f")
        dur=$(sed -n 's/^duration_ms: //p' "$meta")
        echo "| \`$f\` | \`$cmd\` | $bytes | $dur |"
    done
    total=$(cat "$out"/*.bin | wc -c)
    echo
    echo "Total: $total bytes."
    cat <<'NOTES'

`$TMP` in a command is a fresh temporary directory, which is also the working directory, except for `claude-tui` (see below).

## Notes per file

- `bash-colors.bin`: `ls --color=always -la /usr/bin | head -40`, the 16 foreground and background colors via `echo -e`, bold/underline/inverse, two `grep --color=always` runs.
- `vim.bin`: 30 lines of mixed text (tabs, Latin accents, Greek, Cyrillic, CJK, Hangul, emoji, box drawing, Thai, Hebrew, fullwidth) typed in insert mode, then `gg`, `:set number`, motions, `G`, Ctrl-U/D/E/Y scrolling, `14G`, `:q!`.
- `less.bin`: `man less | less`. Because man writes into a pipe it emits plain text without overstrike bold. Ten page-downs, `/pattern`, `n` twice, `q`.
- `htop.bin`: real htop (no fallback to `top` was needed), 5 seconds, then F10.
- `tmux-inside.bin`: tmux exits with status 1 because `tmux kill-server` makes the attached client print `[server exited]` and exit 1. That is normal tmux behavior.
- `claude-tui.bin`: `claude --version`, then the full-screen claude interface for 8 seconds (alternate screen, logo, input box, footer), Ctrl-C twice. It runs in the repo root because in a new directory claude only shows its "trust this folder?" dialog. Environment variables from a parent Claude Code session are removed first. Claude's terminal queries (DA1, XTVERSION, kitty keyboard) go unanswered, as no terminal is attached; it renders anyway.
- `seq100k.bin`: well under the 2 MB cap, so it is complete (no truncation).
- `colors256-truecolor.bin`: 256 foreground and 256 background colors with `38;5;N` / `48;5;N`, three 80-column 24-bit gradients (`38;2`, `48;2`, both combined), SGR 1 2 3 4 5 7 8 9 21 53 alone, combinations, and the matching reset codes 22-29.
- `wide-emoji.bin`: CJK, precomposed and conjoining Hangul, emoji with and without VS15/VS16, ZWJ family, flag, skin tone, combining accents, Devanagari conjuncts, ZWSP and ZWNJ, a line of 40 wide characters (exactly 80 columns), a line of 80 wide characters (wraps), 79 digits then a wide character (must wrap at column 80), Arabic and Hebrew.
- `resize.bin`: `seq 1 100` at 80x24, resize to 40x12 (TIOCSWINSZ plus SIGWINCH), `seq 1 50`, resize to 120x40, `ls -la /etc | head -30`.
NOTES
} > "$out/README.md"
echo "wrote $out/README.md"
