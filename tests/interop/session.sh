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
# Session-side helpers for the interop scenarios (tests/interop/*.sh).
# Sourced by each scenario when it runs as the session command, either under
# mosh-server or directly in a tmux pane (the "direct" run that gives the
# expected screen).
#
# Every scenario starts the same way: it prints READY-<name> and waits for
# one Enter from the harness. mosh-server already holds the session command
# back until the first client arrives, but the harness also needs to fire
# profile events (blackout, rebind) while output is flowing, and this
# handshake gives it that moment. It also proves that keystrokes reach the
# server in every cell. Then the scenario clears the screen and runs.

interop_ready() {
    stty -echo
    echo "READY-$1"
    IFS= read -r _
    stty echo
    printf '\033[H\033[2J'
    # Deliberate-failure switch for proving the harness can fail: the harness
    # passes INTEROP_CORRUPT only to the direct run, so the expected screen
    # gets one extra line the mosh run never shows.
    if [ "${INTEROP_CORRUPT:-}" = "$1" ]; then
        echo "corrupted expected screen (INTEROP_CORRUPT=$1)"
    fi
}

# Print the end marker and keep the screen as it is until the harness kills
# the session. The tty still echoes typed keys, which the blackout and rebind
# profiles use to check that the session is live.
interop_done() {
    echo "DONE-$1"
    exec sleep 1000000
}
