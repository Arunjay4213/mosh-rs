# STATUS

Plain-English progress log for mosh-rs, written by the orchestrator at each gate.
Entries are in time order: newest at the bottom.

## 2026-10-13 Phase 0 started

What is happening: creating the empty Cargo workspace, four crates, release profile, and installing the Windows cross target.

Findings before starting:
- The repo had no commits. Only PLAN.md, CLAUDE.md, HANDOFF.md, .gitignore and the upstream symlink existed.
- The Windows cross target (`x86_64-pc-windows-gnu`) was NOT installed in rustup, and the mingw cross compiler was NOT installed from apt. CLAUDE.md said it was. Phase 0 installs both.
- Upstream was built with its internal OCB implementation (`ocb_internal.o`), not OpenSSL's. All upstream static libraries (`libmoshcrypto.a`, `libmoshterminal.a`, ...) exist, so the reference oracles can link against them directly.

Agent spawns and why:
- Phase 0 agent (Opus, medium): one agent, because the work is small, sequential, and must finish before any Phase 1 agent can build. Owns the workspace root and the four empty crates.

## 2026-10-13 Phase 0 gate: PASSED

What passed (verified by the orchestrator rerunning the commands, not from the agent's report):
- `cargo build --workspace` succeeds. Six packages: mosh-terminal, mosh-network, mosh-platform, mosh, mosh-tests (tests/), mosh-tools (tools/).
- `cargo build -p mosh-platform --target x86_64-pc-windows-gnu` succeeds. The mingw cross compiler and the rustup Windows target are now installed.
- Release profile is opt-level 3, fat LTO, one codegen unit, panic = unwind, as PLAN section 5 requires.

Decisions made in Phase 0:
- nix has no `pty` feature; its pty functions come with the `term` feature. Dropped `pty`.
- libc stays at 0.2.x. The 1.0 alpha would pull two libc versions into the build because nix depends on 0.2.
- `tests/` and `tools/` are their own packages so Phase 1 agents never edit a shared Cargo.toml. Helper binaries go in `tests/src/bin/` and `tools/src/bin/` (auto-discovered).
- `ref/ref-crypto/` and `ref/ref-term/` each get their own Makefile, owned by agent A and B respectively.
- `.gitignore` had `upstream/`, which does not match the `upstream` symlink. Changed to `upstream`.

Next: Phase 1 agents A (network), B (terminal), C (platform + launcher), H (tools) in parallel.

## 2026-10-13 Phase 1 started: four agents in parallel

Agent spawns and why (all Opus, medium effort):
- A, network crate. Owns `crates/network`, `ref/ref-crypto`, and the crypto and transport tests. Independent of the other three, so it runs in parallel. Its gate needs `udp-impair` from H; the orchestrator reruns the sync test through the proxy once H delivers.
- B, terminal crate. Owns `crates/terminal`, `ref/ref-term`, `vt-check`, `vt-fuzz`, and the emulator conformance tests. The biggest module (4,704 upstream lines) and the one most likely to need rework, so it starts first and runs alone on its crate.
- C, platform crate and launcher. Owns `crates/platform`, `crates/mosh/src/launcher`, the `mosh` binary. Independent of A and B because it only touches ttys, ptys, processes and `ssh -G`.
- H, tools. Owns `tools/`, the recorded session corpus, and runs upstream's own `make check` to prove the test harness is good. A and B both need its output (udp-impair, corpus) at their gates, which is why it runs now rather than later.

Decision for B: character widths come from a table generated from glibc's `wcwidth` on devbox, not from the `unicode-width` crate.
Reason: upstream calls glibc `wcwidth`, and the client's emulator must agree with the server's emulator about every character width or the screen mirror drifts. Different Unicode tables would make the fuzz oracle fail on emoji and East Asian characters.

## 2026-10-13 Phase 1: all four oracles pass, gate pending review and CI

Every number below comes from the orchestrator rerunning the command, not from an agent's report.

What passed:
- A, network. 2000 random packets encrypt byte-identically in Rust and in upstream's C++ (`ref-crypto`), decrypt cross-ways, and both reject tampered packets; 14 malformed keys rejected by both. RFC 7253 vectors pass. Fragment, compressor and protobuf round trips pass, including a message with an unknown field. Two Rust endpoints synced a toy state over loopback in 10 s, and through `udp-impair` with 30% loss, 100-150 ms delay, 10% reordering and a client address change at 5 s: the server re-attached to the new address and the state converged in 10.5 s. Windows cross-build compiles.
- B, terminal. 27 corpus files (17 extracted from upstream's own emulation tests, 10 recorded sessions) are cell-exact against upstream at 80x24 and 132x43 (54 dumps). 2268 `new_frame` redraws are byte-exact. Differential fuzzing: 1,000,000 cases (seed 1) and 200,000 (seed 2) by the agent, 200,000 (seed 11) by the orchestrator, all zero mismatches, and 300,000 random frame pairs zero mismatches. Planted bugs were caught and minimized (wrong 16-colour mapping -> `ESC[97m`, broken UTF-8 lead byte -> single byte `f8`).
- C, platform and launcher. Raw mode on a real pty shows `-icanon -echo -isig`, a resize to 120x40 is detected through SIGWINCH, 1 MB written through the pty arrives complete, raw mode restores. The `mosh` launcher starts a session end to end over `ssh localhost` with real mosh 1.4.0 binaries, including the `user@host` form and an IP literal; a missing server exits 10 with upstream's message. `mosh.exe` cross-compiles for Windows.
- H, tools. `udp-impair` drops (1012 of 2000 pass at 50%), delays (median RTT 450 ms at 200 ms + jitter each way), reorders (568 of 2000), rebinds (server sees a new source port), and blacks out (0 replies for 2 s, then resumes). Ten sessions recorded, 752 KB. Upstream's own `make check`: 32 tests, 30 PASS, 2 XFAIL (expected), 0 FAIL.
- Code quality bar: `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean at commit d9c8289.

What failed:
- CI on d9c8289: the GitHub runner has no upstream C++ tree, so `ref-crypto` cannot build. Not a code failure. One agent is making CI build mosh 1.4.0 from source and install mosh, tmux and ssh on the runner so the same oracles run there.

Gate not yet declared: waiting on the Fable line-by-line review of the terminal and transport ports (`reviews/phase1-*.md`) and on green CI, as CLAUDE.md requires.

Next: B is porting statesync (Phase 2 first step). Then agents D (server) and E (client).

## 2026-10-13 Phase 2: statesync port (agent B)

What changed: `crates/mosh/src/statesync` ports upstream `user.{h,cc}` and `completeterminal.{h,cc}` (`UserStream`, `Complete`, both implementing the transport `State` trait).
`ref-term` and `vt-check` gained a `steps` mode (byte records plus resize records), so a test can replay exactly what a `Complete` was fed through upstream's C++ emulator.

Measured (`cargo test --release -p mosh-tests --test statesync_roundtrip -- --nocapture`):
- `files=27 steps=2214 upstream_identical_divergences=1 mismatches=0`. Every corpus file at 80x24 and 132x43, 41 chunks each, one resize mid-run.
- After every step the client shows what the server shows (every field `new_frame` transmits).
- At the end, the server's full dump equals ref-term replaying the server's input, and the client's full dump equals ref-term replaying exactly the host bytes and resizes the client applied.
- The one divergence (bash-colors 80x24 step 18, client cursor one column left of the server's) is upstream behavior: ref-term replays show the same server and client states. Its cause is `new_frame` backspacing from a believed column 80 while the real cursor waits at column 79 to wrap.
- UserStream: diff, apply, subtract and init_diff round-trip; an instruction carrying only an unknown field 4 is ignored.

Facts for later phases:
- `Complete == Complete` compares framebuffer row pointers, as upstream: a client built from diffs never equals the server object; only copies compare equal.
- Full dumps of client and server differ by design (tabs, scroll region, saved cursor and the bell count are not transmitted).

Re-run after the move of the dump code into `tests/src/dump.rs`: terminal_conformance passes; vt-fuzz 100,000 cases (seed 21) and 100,000 `--frames` pairs (seed 22), 0 mismatches.

## 2026-10-13 Impaired transport driver

`tests/transport-impaired.sh` runs `tests/tests/transport_sync.rs` through `udp-impair` (listen 60700, server 60701, control 60702, seed 7, 100 ms delay + 0-50 ms jitter, 10% reordering, each way), and `tests/run-all.sh` now runs it.
It closes the review's "Oracle wiring" gap: before it, only the clean-loopback sync ran.

Command: `tests/transport-impaired.sh` (exit 0).
- 30% loss, rebind at 5 s: converged in 10.95 s; the server logged two different "Server now attached to client at" addresses (51475, then 33722 after the rebind).
- 5% loss, no rebind: converged in 10.40 s.
- 30% loss, `blackout 3000` at 4 s: converged in 10.62 s; the proxy dropped 48 client-to-server packets (15 in the 30% run without a blackout).

Control actions are timed from the moment the test starts its session, and the driver runs the prebuilt test executable rather than `cargo test`.
A first version timed them from script start; `cargo test` took about 7 s to start, so the rebind landed before any traffic and the server saw only one address.

Checks that it can fail: with `MOSH_SYNC_SERVER_PORT` pointed at 60799 (proxy still targets 60701) all three runs time out at 60 s and the script exits 1; with the test executable replaced by `false` it exits 1 in 8 s, naming the unacknowledged control commands, instead of hanging.

## 2026-10-13 Phase 1 review fixes: terminal (agent B)

Review findings for the terminal crate are fixed or answered in `reviews/2026-10-13-phase1-terminal-network.md` (F1, F2, F4-F8, F18, F21, F22, F24-F26), plus T-1 from the local `reviews/phase1-terminal.md`.
Main changes: framebuffer fields have upstream's visibility (F7), the TERM capability table is copied from infocmp (F25, T-1), and dispatch looks up its key without allocating (F18).

New oracle coverage: `new_frame` across a resize (`frame-resize-batch` in ref-term and vt-check).

Measured:
- `cargo test --release -p mosh-tests --test terminal_conformance`: `corpus files=27 dumps compared=54 frames compared=3024 (resize frames 756) mismatches=0`.
- `vt-fuzz --cases 200000 --jobs 8 --seed 31`: `cases=200000 mismatches=0`.
- `vt-fuzz --cases 200000 --jobs 8 --seed 32 --frames` (half the batches resize between old and new): `cases=200000 mismatches=0`.
- Planted-bug check: breaking the size-change branch of `new_frame` gave 1,997 mismatches in 4,000 `--frames` cases, so the resize frames reach that code.

Found while testing: when the size changes, upstream `new_frame` forces `initialized = false` and never compares old cells. The contents of its row-extension code therefore cannot affect output; a planted bug there was correctly not reported.

## 2026-10-13 Phase 1 review fixes: network (agent A)

Review findings for the network crate are fixed or answered in `reviews/2026-10-13-phase1-terminal-network.md` (F3, F6 network part, F9-F17, F19 network part, F20, F23, F24 and F26 network parts).

What changed:
- Malformed input from a peer holding the key (short fragment, conflicting duplicate, final fragment below a stored index, bad protobuf inside valid zlib, `throwaway_num` above every state, oversized ciphertext) now returns an error and drops the packet instead of aborting (PLAN section 9 item 8). Commit 77311f8.
- Socket system calls and core-dump control moved to `crates/platform` (`socket.rs`, `coredump.rs`); `mosh-network` has no `unsafe` (`#![forbid(unsafe_code)]`) and no `libc`. Commit 14e65f2.
- No whole-state copy when rationalizing sent and received states, except the front state's self-subtraction through the new `State::subtract_self` default, which `Complete` and `UserStream` still need to override (agent B). Commit cc19759.
- AES round keys and the session key copy are wiped on drop; casts annotated; `myatoi` documented. Commit e29a4cc.
- Every module names its upstream file in `//!`, every `pub` item has a doc comment, `#![deny(missing_docs)]`. Commit cb37d80.

Measured (devbox, worktree at origin/main):
- `cargo test --release -p mosh-tests --test crypto_conformance`: 2000 random cases identical between `ref-crypto` and `crypto-check`, 14 bad keys rejected by both, RFC 7253 vectors ok on both. 1 passed.
- `cargo test --release -p mosh-tests --test transport_roundtrip`: 5 passed, including `malformed_fragments_are_errors` and `malformed_packets_through_transport` (each malformed case is an `Err` from a live server `Transport`, which then accepts a valid instruction).
- `cargo test --release -p mosh-tests --test transport_sync`: 1 passed; 300 changes synced in 10.0 s on loopback; through `udp-impair` with 30% loss, 20 ms delay and jitter, 5% reordering and a rebind at 5 s, the server re-attached to the new port and the state converged in 13.4 s.
- `cargo build -p mosh-network -p mosh-platform --target x86_64-pc-windows-gnu`: builds.
- `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`: clean.

Open: CI is red only in `tests/tests/launcher.rs` (agent C, real apt mosh on the runner: `TERM environment variable not set`); the network tests pass in CI.

## 2026-10-13 Launcher: CI failures fixed, dual-stack bug fixed (agent C)

What changed:
- The `mosh` launcher no longer resolves the server name on its own (`ssh -G`). Like mosh.pl, it is ssh's ProxyCommand (`--fake-proxy`): it connects to the first address that accepts TCP, reports it as `MOSH IP`, and relays the ssh session, so mosh-server and mosh-client always use the same address. PLAN 9.1 withdrawn, section 8 rewritten.
- `tests/tests/launcher.rs` sets `TERM=xterm-256color` for every process it starts, and has a new dual-stack case (`mosh-dualstack` resolving to `::1` and `127.0.0.1`, run with no flag, `-4` and `-6`; skipped with a printed reason where that /etc/hosts entry is absent, e.g. CI, where `localhost` covers it).

Measured on devbox:
- Before the fix, `mosh mosh-dualstack -- true` with real 1.4.0 binaries: "mosh did not make a successful connection to 127.0.0.1:60001" (server bound `[::1]:60001`), in both /etc/hosts orders; `/usr/bin/mosh` succeeded. After: exit 0 for no flag, `-4`, `-6`, both orders; client argv `127.0.0.1 60001` and `ss -ulnp` shows the server on `127.0.0.1:60001`.
- `cargo test --release -p mosh-tests --test launcher`, 5 runs in a row: 4 passed, 0 failed each time (4.1-8.4 s).
- The dual-stack test fails against the previous launcher (timed out waiting for LAUNCH_OK).
- `--local`, `--ssh='ssh -p 2222'` and an unreachable ssh port behave as `/usr/bin/mosh` (same messages, exit 10 on the last).
- CI on 816b79d: result success (fmt, clippy, test), https://github.com/Arunjay4213/mosh-rs/actions/runs/38360828483.

Terminfo (review T-1, T-2, PLAN 9.6), commit 13f6466: the TERM capability table is generated from the devbox terminfo database (ncurses 6.4+20240113-1ubuntu2.2 with ncurses-term, kitty-terminfo 0.32.2): 2793 names (1830 `toe -a` entries plus aliases), 115 distinct tuples.
`cargo test --release -p mosh-tests --test terminfo_table -- --nocapture`: `terms=2830 (toe -a 1830 plus aliases) compared=5660 upstream_errors_fell_back_to_xterm=74 mismatches=0`.
The comparison runs every name with `MOSH_NO_TERM_INIT` unset and set. The 74 fallbacks are 36 hardcopy entries plus one unknown name, times two runs: upstream throws on them, and the port uses xterm's answers.
Planted-bug check: flipping `bce` in kitty's tuple gave 12 mismatches.

## 2026-10-13 Phase 2 started: server, client, interop harness

Before starting, the orchestrator reran: statesync round trip (27 files, 2214 steps, 0 mismatches; one screen difference that upstream's own client/server pair reproduces byte for byte), terminal conformance (3024 redraws including 756 across a resize, 0 mismatches), terminfo table (2830 terminal names, 0 mismatches against upstream), launcher (4 of 4), and the Windows cross-builds of mosh-network, mosh-platform and mosh.

CI is red only on the terminfo test because the runner lacks two packages; the fix is on branch ci/terminfo for the laptop to push (INTERVENTIONS.md). Phase 1 is not declared passed until CI is green.

Agent spawns and why (all Opus, medium effort, each in its own git worktree):
- M, interop harness. Owns `tests/interop.sh` and `tests/upstream-check.sh`. Needed because the gates for D and E are defined as "the interop matrix column passes" and no matrix driver exists. M proves the harness on C++ client x C++ server first, so a later failure points at our code, not the harness.
- D, server. Ports `mosh-server.cc`. Independent of E: it is tested first against the real C++ client.
- E, client. Ports `mosh-client.cc` and `stmclient.cc`, then the prediction overlay `terminaloverlay.cc`. Independent of D: it is tested first against the real C++ server.
- B and C get small follow-ups (state subtraction without a full-screen copy, malformed diffs return errors; the proxy ignores SIGHUP like mosh.pl).

## 2026-10-13 Launcher: --fake-proxy ignores SIGHUP as mosh.pl does (agent C)

- The proxy relay now ignores SIGHUP at the point mosh.pl does (line 305) via `mosh_platform::process::ignore_sighup()` (no-op on Windows), so the copy finishes after ssh exits. Before: a running `mosh --fake-proxy -- localhost 22` died on `kill -HUP`; after: alive, as `/usr/bin/mosh --fake-proxy` is.
- `cargo test --release -p mosh-tests --test launcher`, 3 runs: 4 passed, 0 failed each (4.44 s, 4.25 s, 4.20 s), including the dual-stack case with no flag, `-4` and `-6`.

## 2026-10-13 statesync: subtract_self and errors instead of aborts (agent B)

What changed:
- `Complete` and `UserStream` override `State::subtract_self` with upstream's `subtract(this)` semantics: `Complete` does nothing (upstream `subtract` is an empty const function) and `UserStream` clears (upstream's `this == prefix` branch). The trait default cloned the whole state on every packet.
- PLAN 9.8 extended: `State::apply_string` returns `Result<(), CryptoError>`, and `Transport::recv` returns the error and drops the packet.
  `Complete` and `UserStream` return non-fatal `dos_detected` errors where upstream `fatal_assert`s or `assert`s: undecodable diff, resize below 1x1, host bytes that make the client reply, and a decreasing echo ack.

Measured:
- `subtract_self`, 10,000 calls, before and after (temporary bench, not committed): `Complete` holding the vim session at 132x43, 4.02 ms before and 53 ns after; the state is unchanged both times (asserted).
  `UserStream` with 1,000 events (clone included in each iteration), 67.3 ms before and 6.14 ms after; the clone alone is 6.17 ms. The stream is empty afterwards in both cases.
- `statesync_roundtrip`: `files=27 steps=2214 upstream_identical_divergences=1 mismatches=0` (unchanged), plus the new `malformed_diffs_through_transport`.
  A keyed peer sends 3 malformed `UserMessage` diffs and 4 malformed `HostMessage` diffs to live `Transport`s: each is an `Err` from `recv` with no state added, and the next valid diff is accepted.
- `transport_roundtrip`: 5 passed. `transport_sync`: 1 passed.

## 2026-10-13 mosh-server ported; gate 1 (server_e2e) passes (agent D)

What changed:
- `crates/mosh/src/server/mod.rs` ports `mosh-server.cc` top to bottom with the same function names: option parsing (a small glibc-compatible `getopt`, so error messages and their order match), the locale fallback through `-l`, `MOSH CONNECT` with the `\r\n` bodge, the detach fork, the /dev/null redirect, the pty child (pipe wait, IUTF8, TERM, motd, `warn_unattached`, `execvp`), and `serve` with the same constants, order and exception handling (an error from `recv`, `apply_string` or `tick` skips the rest of the loop iteration, as a throw does).
- Not ported, as in the devbox upstream build: syslog (`HAVE_SYSLOG` off), utempter records (PLAN 9.2), `pledge`.
- `crates/platform/src/server.rs` (new, unix only): the system calls mosh-server makes directly. `crates/platform/src/pty.rs` gained `forkpty_winsize`, `get_winsize`, `set_winsize` (whole struct, so a resize keeps the pixel fields as upstream) and `execvp` (path separate from argv[0], for login shells).
- `mosh-server` on Windows prints "mosh-server is not supported on Windows" and exits 1.

Measured on devbox (`cargo test --release -p mosh-tests --test server_e2e -- --nocapture`, each case run against /usr/bin/mosh-server first, then ours, with the apt C++ client):
- Launcher session `sh -c 'printf "SERVER_OK\n"; sleep 1; seq 1 300; echo DONE; sleep 2'`: SERVER_OK, 300 and DONE on screen, `[mosh is exiting.]`, exit 0, for both servers.
- `bash --norc`: typed `echo typed-through-mosh` shows its output; `tmux resize-window -x 100 -y 30` then `tput cols; tput lines` shows 100 and 30.
- `MOSH_SERVER_NETWORK_TMOUT=5`, client killed with SIGKILL: upstream exits 4.759 s after the kill, ours 4.756 s.
- `MOSH_SERVER_SIGNAL_TMOUT=5`: SIGUSR1 1 s after the kill is ignored by both; SIGUSR1 7 s after the kill: upstream exits 4.056 s later, ours 4.056 s.
- Planted bug (resize width off by one): the resize case fails.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo build -p mosh -p mosh-platform -p mosh-network -p mosh-terminal --target x86_64-pc-windows-gnu`: builds. `cargo build --workspace` for that target fails only in `tools` (`pty-record`, `udp-impair`) and `tests/src/bin/toy-sync.rs`, which are unix-only programs not gated by `cfg(unix)`, unchanged by this work.

## 2026-10-13 mosh-client, landing 1: client against the C++ server (agent E)

What changed:
- `mosh-client` is a port of `mosh-client.cc` (`client/mod.rs`), `stmclient.cc` (`client/stmclient.rs`) and all of `terminaloverlay.cc` (`client/terminaloverlay.rs`, including the prediction engine, which the default adaptive mode only shows on slow links; its own gate is landing 2).
- PLAN 9.5: each non-empty frame is wrapped in `\e[?2026h` ... `\e[?2026l`. Empty frames stay empty, so an idle client writes nothing.
- A panic leaves the alternate screen and restores the tty from the panic hook before the message is printed, then prints `[mosh is exiting.]` and exits 1 (checked once with a temporary panic on Ctrl-G: `STTY:icanon STTY:echo` afterwards).
- Platform: `tty::suspend_self()` (`kill(0, SIGSTOP)`, no-op on Windows) and a Windows no-op `coredump::disable_dumping_core()`.
- INTERIM: `mosh-client -c` asks `tput colors`, because the generated terminfo table (PLAN 9.6) has no `colors` capability yet. Upstream's messages and exit status for unset and unknown TERM are kept. Open question to the orchestrator.

Measured (`cargo test --release -p mosh-tests --test client_e2e`, every test runs `/usr/bin/mosh-client` first, then ours, against `/usr/bin/mosh-server` via our launcher in tmux): 5 passed.
- Command output (`seq 1 300`, `DONE`), `[mosh is exiting.]`, exit 0; interactive `bash --norc`; resize to 100x30 gives `COLS=100 LINES=30` from `tput`; `Ctrl-^ .` exits 0, the pre-mosh screen comes back and `stty -a` shows `icanon echo`.
- `kill -9` of the server: `mosh: Last contact 9 seconds ago. [To quit: Ctrl-^ .]` after 9.1 s for both clients.
- Idle CPU, `pidstat 1 60`: 0.00% for both clients.
- Argument errors, `-c`, unknown prediction mode, no MOSH_KEY, no tty: same messages and exit codes as `/usr/bin/mosh-client`.
- `cargo build -p mosh --bin mosh-client --target x86_64-pc-windows-gnu`: builds.
- Not yet run: `tests/interop.sh` and `tests/upstream-check.sh` (agent M's harness is not on main yet).

