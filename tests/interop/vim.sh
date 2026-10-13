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
# Scenario vim: a full-screen editor driven by keystrokes that travel through
# mosh. The harness types three lines, swaps two with `ddp`, appends one,
# writes the file, compares the vim screen against the direct run, and quits
# with :q!. The scenario then clears the screen and prints the file, which
# shows the edits arrived in order.
#
# The clear after vim is needed because mosh 1.4.0 has no alternate screen:
# in the direct run tmux restores the pre-vim screen when vim exits, while
# under mosh vim's last frame stays.

# Harness side: called by tests/interop.sh with the helpers it defines.
drive_vim() {
    wait_text '"notes.txt" [New]' || return 1
    keys i
    wait_text '-- INSERT --' || return 1
    keys -l 'alpha line'
    keys Enter
    keys -l 'bravo line'
    keys Enter
    keys -l 'charlie line'
    wait_text 'charlie line' || return 1
    keys Escape
    wait_gone '-- INSERT --' || return 1
    keys g g d d p
    wait_line 2 'alpha line' || return 1
    keys G o
    wait_text '-- INSERT --' || return 1
    keys -l 'delta added at the end'
    wait_text 'delta added at the end' || return 1
    keys Escape
    wait_gone '-- INSERT --' || return 1
    keys : w Enter
    wait_text 'written' || return 1
    checkpoint written || return 1
    keys : q ! Enter
}

[ -n "${INTEROP_SOURCED:-}" ] && return 0

. "$(dirname "$0")/session.sh"
interop_ready vim
dir=$(mktemp -d)
cd "$dir" || exit 1
vim -u NONE -N notes.txt
status=$?
printf '\033[H\033[2J'
echo "vim exited with status $status; notes.txt:"
cat -n notes.txt
cd / && rm -rf "$dir"
interop_done vim
