# Interop matrix and upstream test suite

Two shell drivers prove that a mosh client and server work together, whichever implementation each one is (PLAN.md section 3, items 4 and 5).
Both print one PASS or FAIL line per case and exit non-zero on any failure.

## tests/interop.sh

```text
tests/interop.sh [--client cxx|rust|all|PATH] [--server cxx|rust|all|PATH]
                 [--profile clean|loss5|loss30|blackout|rebind|all]
                 [--scenario seq|colors|vim|wide|resize|typing|all]
```

`cxx` is mosh 1.4.0 from apt (`/usr/bin/mosh-client`, `/usr/bin/mosh-server`).
`rust` is `target/release/mosh-client` and `mosh-server`; the script builds them with `cargo build --release -p mosh`.
`all` is both.
Options can be repeated, and missing ones default to `cxx`, `cxx`, every profile, every scenario.

Each cell starts `mosh-server new -c 256 -p PORTS -- bash tests/interop/<scenario>.sh`, puts `udp-impair` between client and server, runs the client in its own 80x24 tmux server, and polls `tmux capture-pane` until the screen equals the expected one.
The expected screen comes from running the same scenario directly in an 80x24 tmux pane, without mosh, the way upstream's `e2e-test` makes its "direct" run.
The colors scenario is compared with `capture-pane -e`, so colors and attributes count.

Profiles:

- `clean`: no impairment, still through the proxy.
- `loss5`: 5% loss each way.
- `loss30`: 30% loss, 200 ms delay plus 0-50 ms jitter, 10% reordering.
- In both loss profiles the loss starts once `READY-<name>` is on screen, so connection setup is not lossy, and everything after it is. Reason: like upstream (`stmclient.cc`, "Timed out waiting for server..."), a client that has heard nothing from the server for 15 s gives up, and at 30% loss from the first packet that can happen by chance alone. One Rust-client, Rust-server cell failed this way in a 120-cell run before this change; whether that was chance or a bug is still open.
- `blackout`: all traffic dropped for `INTEROP_BLACKOUT_MS` (120 s by default) right after the scenario starts. Afterwards the screen must still match, and typed keys must echo within 10 s.
- `rebind`: the proxy moves to a new source port right after the scenario starts. Afterwards typed keys must echo within 10 s, and the proxy must have forwarded server-to-client packets after the rebind. The proxy closes the old port, so those packets prove the server now sends to the new address. This check reads the proxy's counters, not the server's logs, so it works for both servers.

Scenarios live in `tests/interop/<name>.sh`.
Each prints `READY-<name>`, waits for one Enter from the harness, clears the screen, runs, and ends with `DONE-<name>`.
The scenarios that need keystrokes (`vim`, `resize`, `typing`) also define a `drive_<name>` function that the harness runs on its side; the vim and resize scenarios also compare an intermediate screen.

On FAIL, `target/interop/<cell>/` keeps the expected and actual captures, their unified diff, the reason, and the server and proxy output.
Cells run in parallel (`INTEROP_JOBS`, default 10), with 4 UDP ports each from 61000-61199.
A lock file (`/tmp/mosh-rs-interop.lock`) makes a second run on the same machine wait for the first.

With no arguments, as `tests/run-all.sh` runs it, the script runs the quick subset: cxx client by cxx server, every profile and scenario, with a 10 s blackout.
With any argument, or with `INTEROP_FULL=1`, the blackout is the 120 s that PLAN.md asks for.
`INTEROP_BLACKOUT_MS` overrides both.
`INTEROP_CORRUPT_DIRECT=<scenario>` adds a line to the top of that scenario's expected screen, which shows the harness fails when screens differ.
It has no effect on `seq`, whose 2000 lines scroll the extra line away.

Prerequisites: tmux, vim, ss, python3, flock, cargo, and the C.UTF-8 locale.
A missing one fails the run and is named.

### Known mosh 1.4.0 limits the scenarios avoid

These differ between mosh and a direct run for every mosh pair, so they cannot be part of an expected screen:

- No alternate screen: after vim exits, a direct tmux pane restores the old screen and mosh keeps vim's last frame. The vim scenario clears the screen after vim exits.
- SGR 2 (faint) is dropped.
- 256-color indices 0-7 are redrawn as SGR 30-37, and bright colors 90-97 and 100-107 as 256-color indices 8-15. tmux keeps these apart in `capture-pane -e`. Upstream's own `emulation-attributes-256color8` test is XFAIL for the first one.
- Text echoed by the tty and output printed by a program interleave differently when keystrokes arrive in batches, which happens under loss and delay. The typing scenario turns the tty echo off and lets a cat-like program (`dd bs=1`) echo each byte, so the screen depends only on what was typed.

## tests/upstream-check.sh

```text
tests/upstream-check.sh [--client cxx|rust|PATH] [--server cxx|rust|PATH]
```

Runs upstream's own `make check` with the chosen binaries.
Here `cxx` means the binaries of the upstream build (`upstream/src/frontend/`, mosh decd9b7), and that is the default for both.
The upstream tree is never written.
It is copied with `rsync -a` (mtimes kept, so make sees everything as built) to `target/upstream-check/mosh`.
The chosen binaries replace `src/frontend/mosh-client` and `mosh-server` there and are touched, and `make -q` must confirm that make will not rebuild them.
Then `make check` runs in the copy.
This works because upstream's `src/tests/e2e-test` sets `MOSH_CLIENT=$PWD/../frontend/mosh-client` and `MOSH_SERVER=$PWD/../frontend/mosh-server` itself, so it always tests the binaries of the tree it runs in.
The tests start their sessions with the Perl launcher (`scripts/mosh --local`), which is kept.

The script prints the automake summary and passes only when FAIL, ERROR and XPASS are 0.
With upstream's binaries the result is 30 PASS, 2 XFAIL (`e2e-failure.test`, which is meant to fail, and `emulation-attributes-256color8.test`).
The full log is `target/upstream-check/make-check.log`, and per-test logs are in `target/upstream-check/mosh/src/tests/`.

## Commands that check each part

- Server: `tests/interop.sh --client cxx --server rust` and `tests/upstream-check.sh --client cxx --server rust`.
- Client: `tests/interop.sh --client rust --server cxx` and `tests/upstream-check.sh --client rust --server cxx`.
- Full matrix: `INTEROP_FULL=1 tests/interop.sh --client all --server all`, plus `tests/upstream-check.sh --client rust --server rust`.

For a quicker loop while developing, add `--profile clean` or `--scenario NAME`, or set `INTEROP_BLACKOUT_MS=10000`.

## tests/interop-setup-loss.sh

Session setup under loss, which the matrix leaves out.
Runs with `--run`, or with no arguments only when `INTEROP_FULL=1`, so `tests/run-all.sh` runs it only then.

- 40 setups for each of cxx client x cxx server (the baseline), rust client x cxx server and cxx client x rust server, through `udp-impair --loss 0.30 --delay-ms 200 --jitter-ms 50 --reorder 0.10` from the first packet, 20 s budget each. Records success (READY on screen), time to that screen, time to first contact, and client packets sent before first contact.
- 5 runs per client against a proxy that drops everything, counting packets sent in 12 s with no contact. Both clients send one packet every 3 s, then give up at 15 s with a burst of shutdown packets, so the window ends before 15 s.
- Pass: for each Rust pair a one-sided Fisher exact test of its failures against the baseline's gives p >= 0.05, and the Rust client's median no-contact packet count is at least the C++ client's.
