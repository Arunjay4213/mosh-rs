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
# Runs every test: the cargo workspace tests, then every executable
# tests/*.sh driver. Exits non-zero if anything fails.

set -e

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"
self=$(basename "$0")

cargo build --release --workspace

failed=()
if ! cargo test --release --workspace --no-fail-fast; then
    failed+=("cargo test --release --workspace --no-fail-fast")
fi

drivers=0
for t in "$root"/tests/*.sh; do
    [ -e "$t" ] || continue
    [ "$(basename "$t")" = "$self" ] && continue
    [ -x "$t" ] || continue
    drivers=$((drivers + 1))
    echo "== $(basename "$t")"
    if ! "$t"; then
        failed+=("$(basename "$t")")
    fi
done

echo
echo "run-all: cargo test plus $drivers shell driver(s)"
if [ ${#failed[@]} -ne 0 ]; then
    echo "run-all: FAILED: ${failed[*]}"
    exit 1
fi
echo "run-all: all passed"
