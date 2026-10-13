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
# Scenario colors: SGR 16-color, 256-color and truecolor foreground and
# background, and text attributes. The harness compares this one with
# `capture-pane -e`, so attributes are part of the expected screen.
#
# Left out on purpose, because mosh 1.4.0 itself does not reproduce them,
# so the direct run could never match any mosh pair:
# - SGR 2 (faint): mosh drops it (seen on devbox with apt mosh 1.4.0).
# - 256-color indices 0-7 (38;5;N, N < 8): mosh stores them as the 8 basic
#   colors and redraws them as 30-37, which tmux records as a different
#   color. Upstream marks its own test of this,
#   emulation-attributes-256color8, XFAIL.
# - Bright colors 90-97 and 100-107: the reverse, mosh redraws them as
#   38;5;8-15 and 48;5;8-15 (seen on devbox). Upstream's 16color test also
#   uses only 30-37 and 40-47. Palette entries 8-15 themselves round-trip
#   and are included below.
# Every line ends with SGR 0 before the newline so no colored background
# spills into blank cells (tmux has no BCE; mosh clears lines with spaces).

[ -n "${INTEROP_SOURCED:-}" ] && return 0

. "$(dirname "$0")/session.sh"
interop_ready colors

line=
for c in 30 31 32 33 34 35 36 37; do line+=$(printf '\033[%dmfg%d\033[0m ' $c $c); done
echo "$line"
line=
for c in 40 41 42 43 44 45 46 47; do line+=$(printf '\033[%dmbg%d\033[0m ' $c $c); done
echo "$line"

# 256-color palette entries 8-255, foreground then background.
for sgr in 38 48; do
    for row in 0 1 2 3 4 5 6; do
        line=
        for i in $(seq 0 39); do
            n=$((8 + row * 40 + i))
            [ $n -gt 255 ] && break
            line+=$(printf '\033[%d;5;%dm#' $sgr $n)
        done
        printf '%s\033[0m\n' "$line"
    done
done

# Truecolor gradients.
line=
for i in $(seq 0 63); do line+=$(printf '\033[38;2;%d;%d;128m*' $((i * 4)) $((255 - i * 4))); done
printf '%s\033[0m\n' "$line"
line=
for i in $(seq 0 63); do line+=$(printf '\033[48;2;40;%d;%dm ' $((i * 4)) $((255 - i * 4))); done
printf '%s\033[0m\n' "$line"

# Attributes alone and combined with colors.
printf '\033[1mbold\033[0m \033[3mitalic\033[0m \033[4munderline\033[0m \033[5mblink\033[0m '
printf '\033[7minverse\033[0m \033[8minvisible\033[0m \033[1;4;38;5;208mbold-ul-208\033[0m '
printf '\033[3;7;38;2;200;100;50mital-inv-rgb\033[0m\n'
printf '\033[1;31;43mbold red on yellow\033[22m normal-weight\033[39m default-fg\033[0m\n'

interop_done colors
