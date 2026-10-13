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
# Scenario typing: the harness types 300 keystrokes (six lines of 49
# characters plus Enter) at about 30 per second, and a cat-like program
# echoes them: with the tty in non-canonical mode and its own echo off,
# `dd bs=1 count=300` copies each byte to the screen as it arrives and stops
# after 300. Under mosh every keystroke goes client to server and every echo
# comes back, and under latency the client's prediction overlay is active.
#
# Why not plain `cat` with the tty's echo: then each line is shown twice,
# once by the tty echo and once by cat, and how the two interleave depends
# on how keystrokes are batched in transit. Under loss and delay mosh
# delivers several keystrokes at once and the screen differs from the
# direct run for a reason that has nothing to do with mosh's correctness.
# Here the screen depends only on the bytes typed.

# The six lines, 49 characters each.
typing_lines() {
    local i
    for i in 1 2 3 4 5 6; do
        printf 'line %d: the quick brown fox jumps over a lazy dog\n' "$i"
    done
}

# Harness side: called by tests/interop.sh with the helpers it defines.
drive_typing() {
    wait_text 'type 6 lines:' || return 1
    local line i
    while IFS= read -r line; do
        for ((i = 0; i < ${#line}; i++)); do
            keys -l "${line:i:1}"
            sleep 0.033
        done
        keys Enter
        sleep 0.033
    done < <(typing_lines)
}

[ -n "${INTEROP_SOURCED:-}" ] && return 0

. "$(dirname "$0")/session.sh"
interop_ready typing
echo 'type 6 lines:'
stty -icanon -echo min 1
dd bs=1 count="$(typing_lines | wc -c)" status=none
stty icanon echo
interop_done typing
