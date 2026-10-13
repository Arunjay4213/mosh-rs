# Tests

`tests/run-all.sh` runs everything: each cargo integration test, `lib-bin-doc` (the workspace's lib, bin and doc tests), and every executable `tests/*.sh` driver.
The cargo tests run one after another; `scrollback.sh` runs after them (it shares ports 61500-61599 with `detached_warning` and `scrollback_hostile`); every other driver runs alongside them.
Each item prints one block (its tail when it passed, all of it when it failed, full logs in `target/run-all/`), then a PASS/FAIL line per item and the wall time.
It exports `MOSH_CI_FULL=1` unless that is already 0.

To run a subset, for example in a CI shard, name the items, comma-separated, globs allowed:

```
RUN_ALL_ONLY=client_e2e,transport-impaired.sh tests/run-all.sh
RUN_ALL_ONLY='lib-bin-doc,*_conformance' tests/run-all.sh
```

A pattern that matches nothing exits 2 and lists every name, so a typo cannot silently skip tests.
A set of shards must together cover every name, including `lib-bin-doc`.
The interop matrix and upstream's test suite are described in `tests/interop/README.md`.

## Splitting the long scrollback tests across CI shards

Both scrollback tests can run as any subset, alone or side by side with other tests.
Every subset proves the same things it proves in a full run.

### `tests/scrollback.sh` (oracle 6, about 270 s in all)

```
tests/scrollback.sh --list                       # the variant names, one per line
tests/scrollback.sh                              # all variants (what run-all.sh does)
tests/scrollback.sh clean vim                    # the named variants
SCROLLBACK_VARIANTS=clean,vim tests/scrollback.sh  # the same, from the environment
```

Arguments win over `SCROLLBACK_VARIANTS`; names are separated by commas or spaces.
Each variant starts and ends its own sessions and its own tmux server, so any subset runs alone.
Sessions on one machine take `/tmp/mosh-rs-interop.lock` and use UDP ports 61500-61599, so concurrent runs on one machine wait for each other.
On separate runners they run in parallel.
Measured alone on devbox: loss30 173 s, dropped 23 s, clean 10 s, altscreen 5 s, resize 5 s, every other variant 3 s or less.
A failing variant prints its reason, the checker's counts, any screen diff and the screen at the failure to stdout.
Needs tmux 3.2 or later, vim, less, ss, python3, flock and passwordless `ssh localhost`; a missing one is reported as a FAIL with its name.

Shards that split it evenly:

```
SCROLLBACK_VARIANTS=loss30 tests/scrollback.sh
SCROLLBACK_VARIANTS=clean,vim,clear,altscreen,resize,shrink,grow,oldserver,dropped tests/scrollback.sh
```

### `tests/tests/scrollback_server.rs` (13 tests)

```
cargo test --release -p mosh-tests --test scrollback_server -- --list
cargo test --release -p mosh-tests --test scrollback_server -- --exact live::rust_server_loss30_delay100
cargo test --release -p mosh-tests --test scrollback_server -- --skip live::
cargo test --release -p mosh-tests --test scrollback_server -- live::
```

Every test runs alone with `--exact <name>` and in parallel with the others (`--test-threads=N`).
The four `live::` tests hold `/tmp/mosh-rs-interop.lock` for their sessions, so they run one at a time on one machine.
Measured alone on devbox: `live::rust_server_loss30_delay100` 24 s (`seq 1 10000` through 30% loss; the full 50,000 lines through the same loss are `tests/scrollback.sh loss30`), `live::rust_server_blackout_10s` 21 s, `live::rust_server_clean` 9 s, every other test 2 s or less.
All 13 with `--test-threads=13`: 55 s.
