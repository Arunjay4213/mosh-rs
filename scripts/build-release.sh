#!/usr/bin/env bash
# Build the shipped binaries (mosh, mosh-client, mosh-server) with the release
# profile (PLAN.md section 5) and check that the Linux ones are static.
#
# Usage: scripts/build-release.sh [--target TRIPLE]...
#   Default target: the host's musl target, e.g. aarch64-unknown-linux-musl.
#   Supported: aarch64-unknown-linux-musl, x86_64-unknown-linux-musl,
#   x86_64-pc-windows-gnu (built, not checked for static linking).
#
# Each *-linux-musl binary must pass three checks, or the script exits 1:
#   file      says "statically linked" or "static-pie linked"
#   ldd       says "not a dynamic executable"
#   readelf   shows no NEEDED (shared library) entry and no interpreter
# ldd alone proves little for a foreign architecture (it gives the same
# answer for any binary it cannot load), so readelf is the arch-neutral check.
#
# Nothing is stripped beyond what the release profile does.
# Prerequisites: rustup targets for each triple; musl-tools (host musl);
# gcc-x86-64-linux-gnu on a non-x86_64 host (linker for x86_64 musl, set in
# .cargo/config.toml); gcc-mingw-w64-x86-64 for Windows.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

targets=()
while [ $# -gt 0 ]; do
    case $1 in
        --target)
            [ $# -ge 2 ] || { echo "build-release: --target needs a value" >&2; exit 2; }
            targets+=("$2")
            shift 2
            ;;
        -h | --help)
            sed -n '2,/^set -euo/p' "$0" | sed '$d; s/^# \{0,1\}//'
            exit 0
            ;;
        *)
            echo "build-release: unknown argument $1" >&2
            exit 2
            ;;
    esac
done
[ ${#targets[@]} -eq 0 ] && targets=("$(uname -m)-unknown-linux-musl")

bins=(mosh mosh-client mosh-server)
fail=0
rows=()

for t in "${targets[@]}"; do
    case $t in
        *-linux-musl | x86_64-pc-windows-gnu) ;;
        *)
            echo "build-release: unsupported target $t" >&2
            exit 2
            ;;
    esac
    echo "== cargo build --release -p mosh --target $t"
    cargo build --release -p mosh --target "$t"
    ext=
    [ "$t" = x86_64-pc-windows-gnu ] && ext=.exe
    for b in "${bins[@]}"; do
        f=target/$t/release/$b$ext
        [ -f "$f" ] || { echo "FAIL $f was not built"; fail=1; continue; }
        size=$(stat -c %s "$f")
        status=ok
        echo "-- $f"
        echo "   file:    $(file -b "$f")"
        if [[ $t == *-linux-musl ]]; then
            ldd_out=$(ldd "$f" 2>&1 || true)
            echo "   ldd:     $(echo $ldd_out)"
            needed=$(readelf -d "$f" 2>/dev/null | grep -c '(NEEDED)' || true)
            interp=$(readelf -l "$f" 2>/dev/null | grep -c 'Requesting program interpreter' || true)
            echo "   readelf: NEEDED entries: $needed, interpreter: $interp"
            file -b "$f" | grep -qE 'statically linked|static-pie linked' || { echo "   FAIL: file does not say statically linked"; status=FAIL; }
            grep -q 'not a dynamic executable' <<< "$ldd_out" || { echo "   FAIL: ldd lists libraries"; status=FAIL; }
            { [ "$needed" = 0 ] && [ "$interp" = 0 ]; } || { echo "   FAIL: readelf shows dynamic linking"; status=FAIL; }
        else
            status="ok (no static check)"
        fi
        [ "$status" = FAIL ] && fail=1
        rows+=("$(printf '%-28s %-16s %10s %7s  %s' "$t" "$b$ext" "$size" "$(awk -v s="$size" 'BEGIN { printf "%.2f", s / 1048576 }')" "$status")")
    done
done

echo
printf '%-28s %-16s %10s %7s  %s\n' target binary bytes MiB static
for r in "${rows[@]}"; do echo "$r"; done
exit $fail
