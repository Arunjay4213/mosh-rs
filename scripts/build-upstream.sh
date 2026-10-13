#!/usr/bin/env sh
# Build upstream mosh and point the repo's `upstream` symlink at it.
# Pinned to the commit devbox's ~/src/mosh is checked out at: master decd9b7 (2026-03-22),
# 39 commits after the mosh-1.4.0 tag. The oracles need it: ref/ref-term does not compile
# against the 1.4.0 headers (src/util/shared.h), and the corpus has master's OSC 8 test.
# The ref/ oracles link against this tree (src/*/libmosh*.a, src/include/config.h, src/tests/*.cc).
# Usage: scripts/build-upstream.sh [DIR]   (default DIR: $HOME/src/mosh)
# Idempotent: the build is skipped when DIR/src/frontend/mosh-server already exists.
# Configure flags match the devbox tree (see upstream/config.log there).
set -eu
commit=decd9b705eb81626f694335b8d5940538beb06da
dir=${1:-$HOME/src/mosh}
repo=$(cd "$(dirname "$0")/.." && pwd)

if [ ! -x "$dir/src/frontend/mosh-server" ]; then
  if [ ! -f "$dir/configure.ac" ]; then
    mkdir -p "$dir"
    git -C "$dir" init -q
    git -C "$dir" fetch -q --depth 1 https://github.com/mobile-shell/mosh "$commit"
    git -C "$dir" checkout -q FETCH_HEAD
  fi
  (cd "$dir" && ./autogen.sh && ./configure --enable-examples=no && make -j"$(nproc)")
else
  echo "upstream already built at $dir"
fi

if [ -e "$repo/upstream" ] && [ ! -L "$repo/upstream" ]; then
  echo "$repo/upstream exists and is not a symlink; refusing to replace it" >&2
  exit 1
fi
ln -sfn "$dir" "$repo/upstream"
echo "upstream -> $dir"
