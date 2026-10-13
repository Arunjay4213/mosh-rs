#!/bin/sh
# Extract the byte streams that upstream mosh's emulation tests send to the
# terminal, by running each test script's "baseline" hook directly.
#
# The scripts normally run under upstream's e2e-test harness (tmux + mosh)
# and sleep between chunks. Here fake `sleep`, `stty` and `tmux` commands go
# first on PATH: sleep and stty do nothing, and `tmux -V` reports 3.4 (the
# real tmux on devbox) without starting a server that would print control-mode
# output into the captured bytes.
#
# Usage: tests/corpus/emulation/extract.sh   (from anywhere)
set -eu

here=$(cd "$(dirname "$0")" && pwd)
tests=$(cd "$here/../../../upstream/src/tests" && pwd)
fake=$(mktemp -d)
trap 'rm -rf "$fake"' EXIT

printf '#!/bin/sh\nexit 0\n' > "$fake/sleep"
printf '#!/bin/sh\nexit 0\n' > "$fake/stty"
cat > "$fake/tmux" <<'TMUX'
#!/bin/sh
# -V: report the real devbox version. -S SOCKET: create the socket file the
# test's tmux_check removes afterwards.
if [ "${1:-}" = -V ]; then echo "tmux 3.4"; exit 0; fi
while [ $# -gt 0 ]; do
    if [ "$1" = -S ]; then : > "$2"; fi
    shift
done
exit 0
TMUX
chmod +x "$fake/sleep" "$fake/stty" "$fake/tmux"

run() {
    # $1 = script name, $2 = hook argument
    (cd "$tests" && PATH="$fake:$PATH" sh "./$1" "$2" < /dev/null)
}

rm -f "$here"/*.bin
empty=""
for script in "$tests"/emulation-*.test prediction-unicode.test unicode-combine-fallback-assert.test \
              unicode-later-combining.test window-resize.test; do
    name=$(basename "$script" .test)
    # emulation-attributes.test is only run through its symlinked names
    # (emulation-attributes-16color.test etc.); run directly it fails.
    [ "$name" = emulation-attributes ] && continue
    out="$here/$name.baseline.bin"
    run "$name.test" baseline > "$out"
    if [ ! -s "$out" ]; then
        rm -f "$out"
        empty="$empty $name(baseline)"
        continue
    fi
    # "direct" runs the same function as "baseline" in every script that
    # accepts it; keep it only if the bytes differ.
    if grep -q 'baseline|direct)' "$tests/$name.test"; then
        tmp="$fake/direct.bin"
        run "$name.test" direct > "$tmp"
        if ! cmp -s "$tmp" "$out"; then
            cp "$tmp" "$here/$name.direct.bin"
        fi
    fi
done

# unicode-later-combining has a second, different stream ("variant").
run unicode-later-combining.test variant > "$here/unicode-later-combining.variant.bin"

echo "produced no bytes:${empty:- none}"
ls "$here"/*.bin | wc -l
