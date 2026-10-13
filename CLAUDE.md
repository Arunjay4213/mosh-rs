# mosh-rs

Rust reimplementation of mosh 1.4.0. Read `PLAN.md` first. Section numbers below refer to it.

## Rules

- Port from `upstream/` file by file. Keep names, constants, and order of operations. The only allowed differences are listed in PLAN.md section 9.
- Plain functions and structs. No trait without two real implementations. No single-caller abstractions. No async runtime.
- No unit tests of individual functions, no mocks, no test-only abstractions. Tests exercise features end to end (PLAN.md section 4) and live in `tests/`.
- Verify by running the oracle for your module (PLAN.md section 3) and paste the real output in your report.
- When upstream behavior is unclear, run upstream (`ref/` harnesses or the real `mosh-client`/`mosh-server`) and match it. Never guess.
- Never edit `upstream/` or another agent's directory.
- Commit messages are plain. No Co-Authored-By or other attribution lines.
- Every source file starts with the GPLv3 header and names the upstream file it was ported from.

## Layout

- `crates/terminal` emulator and display (upstream `src/terminal`)
- `crates/network` crypto, proto, transport (upstream `src/crypto`, `src/network`, `src/protobufs`)
- `crates/platform` tty, pty, process spawning; the only platform-specific crate
- `crates/mosh` statesync, client, server, launcher; builds `mosh`, `mosh-client`, `mosh-server`
- `ref/` C++ oracles linked against the upstream build
- `tools/` udp-impair, corpus recorder, bench
- `tests/` end-to-end and conformance tests, `tests/run-all.sh` runs everything

## Devbox facts

- Upstream built tree: `~/src/mosh` (symlink `upstream/`). Binaries at `upstream/src/frontend/mosh-client` and `mosh-server`. `make check` works there.
- Real mosh 1.4.0 from apt is on PATH.
- tmux 3.4 is on PATH and is the host-terminal oracle.
- Windows cross target: `x86_64-pc-windows-gnu` with the mingw linker from apt.

## Status, interventions, and GitHub

- Append to STATUS.md at every gate and to INTERVENTIONS.md every time the run needed outside help, was nudged after going idle, or deviated from PLAN.md.
- The remote is origin (private repo github.com/Arunjay4213/mosh-rs). Push to main after every commit. The user reviews progress on GitHub.
