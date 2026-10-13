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
# Scenario resize: the harness resizes the client's tmux window to 100x30
# and back to 80x24. Under mosh the new size has to travel from the client
# to the server, which resizes the session's pty; the scenario sees SIGWINCH
# and prints the size it now has (`tput cols`/`tput lines`).
#
# Each SIGWINCH clears the screen and redraws the list of sizes seen so far
# from the top, so the result does not depend on how tmux reflows text on
# resize (mosh does not reflow). Repeated identical sizes are listed once.

# Harness side: called by tests/interop.sh with the helpers it defines.
drive_resize() {
    wait_text 'size 80x24' || return 1
    resize 100 30
    wait_text 'size 100x30' || return 1
    checkpoint big || return 1
    resize 80 24
}

[ -n "${INTEROP_SOURCED:-}" ] && return 0

. "$(dirname "$0")/session.sh"
interop_ready resize

sizes=()
show() {
    local now
    now="$(tput cols)x$(tput lines)"
    if [ ${#sizes[@]} -eq 0 ] || [ "${sizes[-1]}" != "$now" ]; then
        sizes+=("$now")
    fi
    printf '\033[H\033[2J'
    printf 'size %s\n' "${sizes[@]}"
    if [ ${#sizes[@]} -ge 3 ]; then
        interop_done resize
    fi
}
trap show WINCH
show
# `wait` returns when a trapped signal arrives, so SIGWINCH is handled at
# once instead of after the sleep.
while :; do
    sleep 1000000 &
    wait $!
    kill $! 2>/dev/null
done
