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
# Scenario wide: double-width characters (CJK, Hangul, fullwidth Latin,
# emoji), combining marks, and wide characters at the right margin.

[ -n "${INTEROP_SOURCED:-}" ] && return 0

. "$(dirname "$0")/session.sh"
interop_ready wide

echo 'CJK: 日本語のテキスト 中文字符 한국어 텍스트'
echo 'Fullwidth: ＡＢＣ１２３ｘｙｚ'
echo 'Emoji: 😀 🎉 🚀 🐍 ✅ 👍'
printf 'Combining: e\xcc\x81 a\xcc\x88 n\xcc\x83 o\xcc\x82\xcc\x81 x\xcd\xa1y\n'
echo 'Mixed: abc日本def😀ghi'
# 79 narrow cells, then a wide character: it does not fit in column 80 and
# wraps to the next line.
printf '%079d世界\n' 0
# A wide character exactly filling columns 79-80.
printf '%078d世\n' 0
echo 'Box: ┌──┬──┐ ├──┼──┤ └──┴──┘'

interop_done wide
