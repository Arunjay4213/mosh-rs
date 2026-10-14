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
- CI on d9c8289: the GitHub runner has no upstream C++ tree, so `ref-crypto` cannot build. Not a code failure. One agent is making CI build mosh 1.4.0 from source and install mosh, tmux and ssh on the runner so the same oracles run there. (Correction: CI builds upstream master at decd9b7, the tree the oracles use, not the 1.4.0 tag; see INTERVENTIONS.md "CI push blocked and upstream version corrected".)

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

## 2026-10-13 mosh-server: stdio write errors ignored as in C (agent D)

Found by running upstream `make check` with our server in a private copy of the upstream tree (`e2e-success` server stderr): when stdout was a pipe whose reader had gone, the final `[mosh-server is exiting.]` made the server panic with "failed printing to stdout: Broken pipe", because Rust's `print!` panics on a failed write and C's `fputs` does not.
Every message in `crates/mosh/src/server` now goes through `write!` with the error ignored.
The network crate's verbose diagnostics (`-v`) still use `eprintln!` and would panic the same way if stderr broke; reported to the orchestrator, not changed here.

Measured: private `make check` copy with our `mosh-server`: TOTAL 32, PASS 30, XFAIL 2, FAIL 0, XPASS 0, ERROR 0, and no `panicked` in any test output.

## 2026-10-13 mosh-client, landing 2: prediction overlay gate (agent E)

What changed:
- `tests/tests/prediction_e2e.rs`: the client and the C++ `mosh-server` talk through `udp-impair --delay-ms 150` (300 ms round trip); keys go in with `tmux send-keys`, the pane is read every 5 ms. Each case runs `/usr/bin/mosh-client` first.
- The client's messages no longer use Rust's `print!` family, which panics on a failed write; like C stdio, a failed write is ignored (same issue agent D found in the server). `mosh-client --help >/dev/full` now exits 0, as upstream.
- The overlay code itself is unchanged since landing 1 (0504a2d); it still needs the line-by-line review.

Measured (`cargo test --release -p mosh-tests --test prediction_e2e`): 3 passed.
- Key to screen at 300 ms RTT: C++ always 2 ms, adaptive 2 ms, never 312 ms; ours always 2 ms, adaptive 2 ms, never 316 ms.
- Wrong prediction (the shell loop stops echoing at `q`): `q` shows after 2 ms and is gone after 369 ms (C++) and 364 ms (ours); the screen stays `ab`.
- Upstream's prediction-unicode input (`gl`, `ü`, `ck fa`, `ĩ`, `l ` into `cat`): no garbled frame seen, final screen is three lines of `glück faĩl` x5, both clients.
- Planted bugs: overlay cells never drawn: the latency and wrong-prediction cases fail (316 ms; `q` never shows). Wrong confirmed predictions never killed (cell and cursor checks both disabled; either one alone still removes `q`, as upstream has both): the wrong-prediction case fails.
- `client_e2e` still 5 passed, idle CPU 0.00% for both clients.

## 2026-10-13 mosh-client: more e2e coverage (agent E)

- `prediction_e2e` now times three edits per mode (key at the end of the line, backspace, insert after a left arrow) in always, adaptive, always with `MOSH_PREDICTION_OVERWRITE=yes` (the insert shows as an overwrite, `ax`, and is corrected to bash's `axb`), and never.
  C++: 2/4/2, 2/2/2, 2/2/2 and 316/312/310 ms. Ours: 2/2/2, 3/3/2, 3/2/2 and 312/316/318 ms.
- `client_e2e` gains `Ctrl-^ Ctrl-Z` suspend (`[mosh is suspended.]`, the shell reports the job stopped and works, `fg` repaints the session) and a client with no server (the "Nothing received" bar after 0.3 s (C++) and 0.5 s (ours), "Timed out waiting for server..." at 15.0 s, exit 1 after 19 s with the four-line stderr explanation, same for both): 7 passed.
- `idle_cpu` prints SKIPPED where `pidstat` is missing (the CI runner has no `sysstat`).

## 2026-10-13 Interop harness (agent M)

What landed (8edd8d8, 5fafd9c, 9d41baf):
- `tests/interop.sh`: the interop matrix (PLAN 3.4). Each cell runs `mosh-server new` with a scenario script, `udp-impair` with the profile's impairment, and the client in its own 80x24 tmux server, then polls `capture-pane` until the screen equals a direct (no mosh) tmux run of the same scenario. Scenarios: seq, colors (compared with `-e`), vim, wide, resize, typing. Profiles: clean, loss5, loss30, blackout (then typed keys must echo within 10 s), rebind (then typed keys must echo, and the proxy must forward server-to-client packets to its new port).
- `tests/upstream-check.sh`: upstream `make check` (PLAN 3.5) in a copy of the upstream tree with any client/server pair; the upstream tree is never written.
- `tests/interop/README.md`: usage, the gate commands for D and E, and four mosh 1.4.0 limits the scenarios avoid (no alternate screen, SGR 2 dropped, 256-color 0-7 and bright 90-107 redrawn differently, echo interleaving under batching).
- `tests/run-all.sh` picks both up unchanged. With no arguments `interop.sh` runs cxx x cxx with a 10 s blackout (45 s wall); `upstream-check.sh` runs cxx x cxx (about 4 min).

Measured on devbox:
- `tests/interop.sh --client cxx --server cxx --profile all --scenario all` (120 s blackout): 30 cells, 30 passed, wall time 2m15s (10 cells in parallel).
- `tests/upstream-check.sh`: TOTAL 32, PASS 30, XFAIL 2, FAIL 0, XPASS 0, ERROR 0 in 242 s. `make -q` confirmed no rebuild; sha256 of all 640 files in `~/src/mosh`, and their mtimes, identical before and after; `git status --short` empty.
- Can fail: the stub Rust client (before D and E landed) failed every cell; `INTEROP_CORRUPT_DIRECT=wide` failed the wide cells with a one-line diff; `--server /bin/false` gave FAIL 1, ERROR 27, exit 1.
- After D and E landed: `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 cells, 119 passed. The failure was rust x rust, loss30, vim: the client printed `[mosh is exiting.]` after 15.8 s without ever showing the session; the proxy saw 8 client packets (6 dropped) and 1 server packet in that time. Three reruns of rust x rust loss5 and loss30 passed (36 cells). This is upstream's give-up path (`stmclient.cc` line 539: no state from the server for 15 s, "Timed out waiting for server...", clean shutdown), which 30% loss from the first packet can trigger by chance. Whether chance explains this case (only 8 client packets in 15.8 s) is open, for E.
- Changed so the matrix does not flake on that: the loss profiles now switch loss on through the proxy's control socket once READY is on screen, so connection setup is not lossy; delay, jitter and reordering still apply from the start. Connection setup under loss is therefore not covered by the matrix. After the change, loss5 and loss30 for all four pairs: 48 cells, 48 passed; the full cxx x cxx matrix again 30 of 30 (135 s).

Open: CI's test job installs no `vim`, which the vim scenario needs; `interop.sh` then fails naming it. The fix is adding `vim` to the apt line in `.github/workflows/ci.yml`, which devbox cannot push.

## 2026-10-13 mosh-server: gates 2, 3, 4 pass (agent D)

Measured on devbox at origin/main 9d41baf plus the server commits:
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client cxx --server rust`: 30 cells, 30 passed, 0 failed, wall time 40s.
- `INTEROP_FULL=1 tests/interop.sh --client cxx --server rust` (120 s blackouts): 30 cells, 30 passed, 0 failed, wall time 135s.
- `tests/upstream-check.sh --client cxx --server rust`: total=32 pass=30 xfail=2 fail=0 error=0 xpass=0, 241 s; the test logs show our server's banner and no `panicked`.
- Idle CPU, `pidstat -p <pid> 1 60` on a connected idle session with the C++ client: `/usr/bin/mosh-server` Average 0.00 %CPU, ours Average 0.00 %CPU.
- Earlier manual check, before the harness landed: our server with `-v` behind `udp-impair` at 30% loss and 200 ms delay, a rebind and a 20 s blackout; typed lines echoed after each, and the server log shows a second `Server now attached to client at 127.0.0.1:<new port>` after the rebind.
- `cargo test --release -p mosh-tests --test platform_tty --test launcher` after the `pty.rs` additions: 1 and 4 passed.

## 2026-10-13 mosh-client: interop and upstream gates pass (agent E)

Measured on devbox at origin/main ab9562e:
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client rust --server cxx`: 30 cells, 30 passed, 0 failed, wall time 40s.
- `INTEROP_FULL=1 tests/interop.sh --client rust --server cxx` (120 s blackouts): 30 cells, 30 passed, 0 failed, wall time 135s.
- `tests/upstream-check.sh --client rust --server cxx`: total=32 pass=30 xfail=2 fail=0 error=0 xpass=0, 238 s.
- About agent M's one rust x rust loss30 vim failure (no contact within 15 s, 8 client packets seen): before first contact both clients send few packets. Through `udp-impair` to a port with nothing behind it, `/usr/bin/mosh-client` sent 1, 2, 4, 5 packets by 2, 5, 10, 15 s and ours 1, 2, 4, 6. With 30% loss each way, a cold start can lose all of them, so that cell can fail now and then with any client; it does not point at a client bug.

## 2026-10-13 mosh-client -c from the generated terminfo table (agent B)

What changed: the generated terminfo table also holds `colors`, plus the entries `setupterm` refuses as hardcopy (34) or generic (2).
`terminfo::colors(TERM)` reproduces upstream's `mosh-client -c`, including ncurses' messages and exit status 1 for an unset or empty TERM, a hardcopy type or a generic type. Upstream's `tigetnum("colors")` is -1 when the entry has no `colors`.
The client's interim `tput colors` call is replaced, so `-c` works on Windows.
PLAN 9.6 difference: an unknown TERM prints xterm's count (8) where upstream says "unknown terminal type".

Measured:
- `cargo test --release -p mosh-tests --test terminfo_table -- --nocapture`:
  - `terms=2830 (toe -a 1830 plus aliases) compared=5660 upstream_errors_fell_back_to_xterm=74 mismatches=0` (the caps lines now include `colors`).
  - `colors: cases=2832 unknown_name_fallbacks=1 mismatches=0`. That is every name plus an unknown, an empty and an unset TERM, each compared with upstream's built `mosh-client -c` on stdout, stderr and exit status.
- Manual comparison with `/usr/bin/mosh-client -c`:
  - identical for `xterm-256color` (256), `xterm` (8), `xterm-kitty` (256), `dumb` (-1), `xerox1720` (hardcopy message), `unknown` (generic message), and empty and unset TERM (message, exit 1)
  - `no-such-term` gives 8, the PLAN 9.6 fallback
- `client_e2e`: 7 passed. `launcher`: 4 passed. `cargo build -p mosh --bin mosh-client --target x86_64-pc-windows-gnu`: builds.

## 2026-10-13 Client (agent E) verified by the orchestrator; overlay review pending

Rerun by the orchestrator at 058508b, not taken from the agent's report:
- `client_e2e`: 7 passed (command output, interactive shell, resize to 100x30, quit with Ctrl-^ . restoring the terminal, suspend, "Last contact" after the server is killed, idle CPU 0%), each also run with the C++ client first.
- `prediction_e2e` at 300 ms round trip: a typed key, a backspace and a mid-line insert appear in 2 ms with prediction on (C++ client: 2 ms); with prediction off they take 311-317 ms (C++: 313-316 ms). A wrong prediction is removed after 373 ms (C++: 368 ms).
- `tests/interop.sh --client rust --server cxx` (quick mode, 10 s blackout): 30 of 30 cells passed in 42 s.
- `tests/upstream-check.sh --client rust --server cxx`: upstream's own `make check` with our client, 32 tests, 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR, 0 XPASS.
- `mosh-client -c` matches upstream for every terminal type devbox knows (2832 cases); an unknown TERM prints xterm's 8 per PLAN 9.6.

Not yet passed: the line-by-line review of the prediction overlay, which now runs from the laptop (NEEDS THE USER in INTERVENTIONS.md).

## 2026-10-13 Setup under loss (agent M)

`tests/interop-setup-loss.sh --run` (opt-in; `tests/run-all.sh` runs it only with `INTEROP_FULL=1`): 40 setups per pair through 30% loss, 200 ms delay, 50 ms jitter, 10% reordering from the first packet, 20 s budget each. Result, 73 s wall:
- cxx client x cxx server: 40/40, median first frame 1867 ms, first contact 724 ms, 1 client packet before contact.
- rust client x cxx server: 40/40, 1398 ms, 686 ms, 1 packet. Fisher p=1.000 against the baseline.
- cxx client x rust server: 40/40, 1347 ms, 716 ms, 1 packet. Fisher p=1.000.
- With no contact for 12 s: both clients sent 4 packets in each of 5 runs (one every 3 s, upstream ACK_INTERVAL; both give up at 15 s with a burst of shutdown packets).
Harness bug found and fixed in `tests/interop.sh` too: teardown killed the client's tmux, the SIGHUPed mosh-client kept sending shutdown packets for seconds, and they reached the next cell's proxy on the same port, which then sent the server's replies to the dead client. Before the fix the same driver gave 1 rust-client failure in 40 and median packets before contact of 3, and blackhole counts swung between 5 and 21. The rust x rust loss30 vim failure reported above (proxy saw 1 server packet in 15.8 s) fits this bug but is not proven to be it. After the fix: `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all` 120/120, and the full cxx x cxx matrix with the 120 s blackout 30/30.

## 2026-10-13 Interop harness (agent M) verified by the orchestrator

Rerun by the orchestrator at e697c96:
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 cells (C++ and Rust clients x C++ and Rust servers x clean, 5% loss, 30% loss with 200 ms delay and reordering, blackout, address change x six screen scenarios), 120 passed, 0 failed, 97 s.
- `tests/interop-setup-loss.sh --run` (30% loss from the first packet, 40 setups per pair): C++ client with Rust server 40/40, Rust client with C++ server 40/40, C++ with C++ 39/40. With no server reachable both clients send exactly 4 packets in 12 s.
- Open question sent to agent D: with the Rust server the client's first reply arrives when the session is ready (median 2074 ms), with the C++ server much earlier (780 ms). D is checking whether our server skips an early acknowledgement that upstream sends.
- Not yet run by the orchestrator: the full matrix with 2-minute blackouts for the Rust pairs (agent E ran it for the Rust client: 30/30).

## 2026-10-13 mosh-server: upstream-check log greps, first-contact timing, loss30 failure localized (agent D)

Upstream's `src/tests/e2e-test` (lines 242-255) fails a test when the server's `-vv` stderr contains "round-trip Instruction verification failed", "polls, rate limiting" or "consecutive polls", or "assertion.*failed".
- Select diagnostics: `crates/platform/src/select.rs` prints upstream's text ("select: got 10 polls, rate limiting."). Planted `sel.select(0)` in the server loop (reverted, not committed): `tests/upstream-check.sh --client cxx --server rust` gave pass=5 error=27, every error "select() with zero timeout called too often on server during baseline".
- Assertions: a Rust `assert!` prints "assertion failed: <expr>", which the grep matches. Planted a caught `assert!` in the server loop (the server keeps running, reverted): pass=5 error=27, every error "assertion during baseline". Upstream's `fatal_assert` text ("Fatal assertion failure ... / Failed test: ...") does not match this grep in upstream either. Rust `debug_assert!` is compiled out of release builds, where upstream's `assert` is not (no NDEBUG).
- Round-trip check: OPEN. The Rust sender never prints "round-trip Instruction verification failed": `crates/network/src/transport/transportsender.rs:227` skips upstream's `current_state.compare( newstate )` (transportsender-impl.h:162) because the `State` trait has no `compare` (PLAN 9.10). So this make-check check passes silently for both Rust peers. `Complete::compare` and `UserStream::compare` exist; putting `compare` back into the trait would restore it. Question for the orchestrator (PLAN 9.10, network crate).
- After reverting both plants: upstream-check total=32 pass=30 xfail=2 fail=0 error=0 xpass=0.

First contact without loss (`udp-impair --delay-ms 200`, tcpdump on the server port, 10 runs per server, times from the first client packet at the server): both servers answer the first packet at once (C++ 5 ms, Rust under 1 ms), then send at 250 ms, the client sends at 751 ms, the server at 3253 ms, the client at 3752 ms, in all 20 runs. Packet sizes differ only by the 0-16 bytes of random chaff.
`tests/interop-setup-loss.sh --run` here: 40/40 for all three pairs; median contact cxx-cxx 498 ms, rust-cxx 686 ms, cxx-rust 727 ms; median setup 1866, 1866, 1349 ms.
The contact times are bimodal in every pair: about 440-830 ms when the first exchange survives the 30% loss, about 3.4 s or later when it does not and the client's 3 s ACK_INTERVAL retransmit is needed (contact under 1 s: cxx-cxx 27/40, cxx-rust 25/40, rust-cxx 26/40). With about half the trials in each mode, the median moves between the modes from run to run, which explains 780 versus 2074 ms in the orchestrator's run.

Agent M's rust x rust loss30 vim failure: repeating `--profile loss30` 5 times per pair (30 cells each): rust x rust 30/30, cxx client x rust server 30/30, cxx x cxx 30/30, rust client x cxx server 29/30. The one failure (seq, "client exited while waiting for READY-seq", status 0 after 15.5 s) has the C++ server, so it is on the client side (agent E), or chance that the C++ client would also hit; the server is not involved.

## 2026-10-13 server_e2e: network timeout check does not depend on runner speed (agent D)

The `network_timeout` case asserted the server lived at least 4 s after the client was killed. The server times out 5 s after it last heard from the client, which can be up to 3 s (the client's ACK_INTERVAL) before the kill on a slow runner, so that bound could fail without a bug. It now checks the server is alive 1 s after the kill and gone within 10 s.
Measured: `network_timeout` and `signal_timeout` passed; exit 4.755 s after the kill for both servers.

## 2026-10-13 Spawn: agent F, Phase 2 cleanup cluster

Agent F (Opus, medium effort, own worktree) takes three small items found by agent D, because they cut across crates whose agents have finished (PLAN 10: "fresh small agents per bug cluster"):
- Restore upstream's `compare` in the `State` trait and the verbose round-trip check that prints "round-trip Instruction verification failed", so upstream's `make check` can catch that failure in our binaries (it currently passes silently). Withdraws PLAN 9.10.
- `cargo build --workspace --target x86_64-pc-windows-gnu` fails because unix-only test tools are not gated; gate them.
- Diagnostic messages in the network and platform crates use `eprintln!`, which panics if stderr is closed (for example after ssh exits); C's stdio ignores that error.

## 2026-10-13 Server (agent D) verified by the orchestrator

Rerun by the orchestrator at afa5b47:
- `server_e2e`: 4 passed, each case run against the C++ server first. Network timeout: both servers exit 4.76 s after the client is killed (`MOSH_SERVER_NETWORK_TMOUT=5`). Signal timeout: both ignore an early SIGUSR1 and exit 4.06 s after a late one (`MOSH_SERVER_SIGNAL_TMOUT=5`).
- `tests/upstream-check.sh --client cxx --server rust`: upstream's own `make check` with our server, 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR, 0 XPASS.
- Interop with our server was already covered by the 120-cell run (all four pairs, 120/120).
- Agent D measured first contact with 10 runs per server at 200 ms delay: both servers reply to the client's first packet at once (Rust under 1 ms, C++ 5 ms), and the following packets come at the same times. The jump in median contact time seen under loss comes from two groups of trials (first exchange survives, or the client waits for its 3 s retry), not from the server.
- Agent D saw one failure of the Rust client with the C++ server under 30% loss. The orchestrator repeated that profile 10 times at current main: 60 of 60 cells passed. Most likely it was the test-teardown bug agent M fixed (e697c96).
- Idle CPU over 60 s: 0.00% for both servers (agent D's measurement).

Answers to agent D's questions: the version banner stays `mosh-rs 0.1.0` (it must not claim to be upstream); reading utmp to warn about unattached sessions stays (PLAN 9.2 excludes writing records only); no `setsid` in the server's own fork, as upstream.
Open, assigned to agent F: upstream's "round-trip Instruction verification failed" check cannot fire for our binaries until `compare` is restored (PLAN 9.10).

## 2026-10-13 Round-trip check restored, PLAN 9.10 withdrawn (agent F)

`State` has `compare` again (`Complete` and `UserStream` moved their existing ports into the trait impl; the toy states return false like `UserStream`), and `TransportSender::tick` runs upstream's verbose block in upstream's order (transportsender-impl.h:158-171), printing "Warning, round-trip Instruction verification failed!" when the receiver's reconstructed screen differs from the sender's.
- Planted bug (not committed): `Complete::diff_from` appends a `Z` to the `new_frame` bytes when `MOSH_PLANT_BAD_DIFF` is set. `MOSH_PLANT_BAD_DIFF=1 tests/upstream-check.sh --client cxx --server rust`: total=32 pass=7 error=25, every one of the 25 logs "Round-trip Instruction verification failed on server during baseline"; the server stderr shows `Graphemes: ' ' vs. 'Z'`, `Cell (0, 23) differs.`, `Cursor mismatch: (0, 23) vs. (0, 24).`, then the warning.
- Without the plant: `tests/upstream-check.sh --client cxx --server rust`: total=32 pass=30 xfail=2 fail=0 error=0 xpass=0.
- Client side not testable this way, as upstream: the client runs the block (stmclient.cc:271 sets the transport verbose) but its state is `UserStream`, whose `compare` always returns false (user.h:93), and e2e-test greps only the server's stderr (e2e-test:243).


## 2026-10-13 Windows workspace build (agent F)

The unix-only test programs `udp-impair`, `pty-record` (tools) and `toy-sync` (tests) are now each a directory: `main.rs` keeps the doc comment and, on unix, calls the unchanged code in `unix.rs`; elsewhere it prints "<name> is unix-only" and exits 1. The integration tests that run them (`udp_impair`, `transport_sync`) carry `#![cfg(unix)]` like the other unix-only tests. No shipped crate changed.
- `cargo build --workspace --target x86_64-pc-windows-gnu`: builds (before: 19 errors in `udp-impair` and `pty-record`, then 11 in `toy-sync`, 3 in `tests/tests/udp_impair.rs`).
- `cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu -- -D warnings`: clean.
- Native: `udp_impair` 6 passed, `transport_sync` 1 passed.

## 2026-10-13 Phase 2: all oracles the orchestrator can run pass, gate pending

Rerun by the orchestrator:
- `INTEROP_FULL=1 tests/interop.sh --client all --server all` at 31837bc: 120 cells (C++ and Rust clients x C++ and Rust servers x clean, 5% loss, 30% loss with 200 ms delay and reordering, 2-minute blackout, client address change x six screen scenarios), 120 passed, 0 failed, 402 s. Every blackout cell resumed after 120 s of total silence.
- Upstream's own `make check` (32 tests) with our server and the C++ client: 30 PASS, 2 XFAIL, 0 FAIL. With our client and the C++ server: 30 PASS, 2 XFAIL, 0 FAIL.
- Setup under 30% loss from the first packet, 40 tries per pair: Rust client 40/40, Rust server 40/40, C++ pair 39/40.
- Prediction at 300 ms round trip: typed characters appear in 2 ms, as with the C++ client; wrong predictions are removed after 373 ms (C++ 368 ms).
- Idle CPU of our client and server over 60 s: 0.00%, as the C++ ones.

Still needed before "gate: PASSED":
- Upstream's `make check` with both of our binaries together, after agent F restores the round-trip comparison check (without it one of the suite's log checks cannot fire for our binaries).
- The line-by-line review of the prediction overlay, from the laptop (INTERVENTIONS.md, NEEDS THE USER).
- Green CI, which needs the laptop to push branch `ci/terminfo`.

Phase 3: Windows binaries are staged on devbox with a laptop checklist (INTERVENTIONS.md). Phase 4 (scrollback) starts once agent F's change to the state interface has landed.

## 2026-10-13 No panic on a failing stdout or stderr (agent F)

Every `print!`, `println!`, `eprint!` and `eprintln!` in the shipped crates now goes through `mosh_platform::{outp, outln, errp, errln}` (new `crates/platform/src/stdio.rs`), which ignore a failed write as C's `printf`/`fprintf` do. The server's and client's own copies of these macros (agents D and E) are replaced by the shared ones. `crates/terminal` has no platform dependency, so `Cell::compare` writes with `let _ = writeln!(io::stderr(), ...)` directly.
Proof (driver: `mosh-server new -v -i 127.0.0.1 -- sh -c 'sleep 1; echo hello-from-session; sleep 3'`, then `mosh-client -v` in tmux, both under strace, both with `2>/dev/full` so every stderr write fails with ENOSPC):
- Before (binaries built at 0ccd5b6): the server child wrote "Server now attached to client at", got ENOSPC, panicked, `exit_group(101)`; the client failed on the "[" of its first verbose "Sent" line and exited 1 at once; nothing reached the screen.
- After: every stderr write still fails (`write(2, "[", 1) = -1 ENOSPC`), the session output appears on screen, the client exits 0 after 4.1 s when the command ends, all server processes `exit_group(0)`.
- `2>&-` (closed stderr) never panicked, before or after: Rust's runtime reopens a closed fd 0, 1 or 2 on `/dev/null` at startup (strace: `openat("/dev/null", O_RDWR) = 2`), so writes succeed. The failing case in practice is an stderr that errors (full device, a pipe or pty whose other end is gone).
- `client_e2e` 7, `server_e2e` 4, `launcher` 4, `transport_roundtrip` 3, `statesync_roundtrip` 5: all passed.

## 2026-10-13 Phase 4 started: scrollback, server side first

Agent S1 (Opus, medium effort, own worktree) implements the server half of PLAN section 6: the capability negotiation, the alternate-screen flag, capture of scrolled-off rows into a ring, and the new wire field. One agent, because the client half depends on the server half's wire format, and PLAN 10 says the two run in sequence. Its gate proves old peers are untouched (full interop matrix, upstream's `make check`) and that a capable client receives every scrolled line exactly once.

## 2026-10-13 Default TERM on Windows, PLAN 9.12 (agent F)

`mosh_platform::tty::default_term_if_unset()`, called first in `mosh` and `mosh-client` `main`: on Windows sets `TERM=xterm-256color` when TERM is unset or empty; on unix does nothing.
- Unix unchanged: `env -u TERM target/release/mosh-client -c` and `TERM= ... -c` print "TERM environment variable not set." and exit 1, the same as upstream's `mosh-client -c`. `client_e2e` 7, `launcher` 4, `terminfo_table` 2: all passed.
- `cargo build --workspace --target x86_64-pc-windows-gnu` and the Windows clippy: clean.
- Not run on Windows (devbox has no Windows or wine): the laptop check is in INTERVENTIONS.md.

## 2026-10-13 Phase 2 cleanup (agent F) verified by the orchestrator

Rerun by the orchestrator at 6589529:
- `tests/upstream-check.sh --client rust --server rust`: upstream's own `make check` with both of our binaries, 32 tests, 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR, 0 XPASS. Upstream's round-trip comparison check is restored (PLAN 9.10 withdrawn); agent F showed it now fails the suite (25 ERROR) when the server sends a wrong screen update.
- `cargo build --workspace --target x86_64-pc-windows-gnu` succeeds: unix-only test tools are gated.
- On unix, `mosh-client -c` without TERM still prints upstream's "TERM environment variable not set." and exits 1. On Windows both binaries default TERM to xterm-256color (PLAN 9.12); only the laptop test can confirm that.
- Output to a closed or failing stdout/stderr no longer panics (one shared helper in the platform crate).
- Windows binaries rebuilt from 6589529 with new checksums in INTERVENTIONS.md.

Answers to agent F: the extra "generated diff does not apply" warning stays (it is what replaces upstream's abort under PLAN 9.8); the one direct stderr write in the terminal crate stays (the terminal crate must not depend on the platform crate).

## 2026-10-13 Spawn: agent P, performance bench and static release build

Agent P (Opus, medium effort, own worktree) builds `tools/bench` (every PLAN section 5 target, measured side by side with C++ mosh 1.4.0) and the static musl Linux build. Started during Phase 4 because it shares no files with scrollback and the 1-hour memory run is slow; see INTERVENTIONS.md.

## 2026-10-13 Scrollback, server side (agent S1)

PLAN section 6 items 1-3 and 5: the capability event (`ClientBuffers::Instruction` field 4), the alternate-screen flag (DEC 47/1047/1049, plus a per-row mark so rows drawn on the alternate screen never become history), capture of rows leaving a full-screen region into a 10,000-line ring (`mosh-server --scrollback N`, 0 disables), and `ScrolledLines` (`HostBuffers::Instruction` field 9) between the echo ack and the resize and host bytes.
PLAN section 6 now records the settled details (capture interface, line format, numbering, instruction order, client-side duplicate handling) and one open question.
- Old peers: per-step server diffs for every corpus file (27 files, 2 sizes, 40 and 400 chunks, 23,868 diffs, 1,166,280 bytes) are byte-identical to those of b5393e7 (`cmp` of the full dumps, and the digests in `tests/scrollback-server-baseline.txt`). With the capability, every diff minus `ScrolledLines` is the same bytes.
- Upstream's own `UserStream::apply_string`, fed our capability plus a keystroke and a resize, returned only the keystroke and the resize; the C++ `mosh-server` ran a live session with our capability-sending probe and sent no lines.
- `scrollback_server`: seq 1 50000 through a lossy simulated link (86 deliveries, 37,931 repeated lines skipped) arrives with no gap and no duplicate; also with resizes, a vim-like alternate-screen excursion (0 lines containing VIM), a partial scrolling region (nothing captured), and a 100-line ring (`dropped` exactly 377 and 4400). 374 colored, linked, wide and wrapped rendered lines match upstream's cells in ref-term.
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 of 120. `tests/upstream-check.sh --client cxx --server rust`: 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR. vt-fuzz 200,000 cases and 200,000 `--frames` cases: 0 mismatches. `cargo test --release --workspace`: all pass.
- Open, blocks the client agent's oracle 6: one diff can carry any number of lines. Live `seq 1 50000` with the default ring delivered 10,000 lines and reported 39,977 dropped; with `--scrollback 60000` the single 87-fragment diff got through in 5 of 7 runs; through 30% loss and 100 ms delay nothing arrived in 30 s or 120 s. Two live tests are `#[ignore]` until this is decided.

## 2026-10-13 Agent P: tools/bench and the static musl release build

- `scripts/build-release.sh [--target T]...` builds `mosh`, `mosh-client`, `mosh-server` (release profile, nothing extra stripped) and checks each Linux binary with `file`, `ldd` and `readelf` (no NEEDED entry, no interpreter). aarch64 musl: 797824, 1225240, 1100656 bytes, "statically linked". x86_64 musl (linked on devbox with gcc-x86-64-linux-gnu): 773640, 1249592, 1072824 bytes, "static-pie linked". Windows gnu still builds. The installs are in INTERVENTIONS.md.
- The aarch64 musl client and server pass `tests/interop.sh --profile clean`: 6 cells, 6 passed. The x86_64 ones were only run with `--version` under qemu-x86_64.
- `target/release/bench [--only NAME] [--hour] [--json] [--rust-dir DIR]` (tools/bench/README.md). `bench --hour` at 111139f, devbox (8 cpus, Neoverse-V1):
  - binary_size: PASS, every binary under 1.2 MiB, static.
  - startup to first frame, median of 20: mosh-rs 252.32 ms, mosh 1.4.0 257.08 ms, target 20 ms: FAIL for both. To the client's first write: 1.50 ms and 3.05 ms.
  - keystroke at 300 ms RTT: median 0.10 ms and 0.08 ms, exactly 1 write per key before the echo in all 30 runs for both: PASS.
  - seq 1 100000, median of 5: 149.26 ms and 149.78 ms: PASS.
  - idle_cpu over 60 s: 0 ticks for the client and the server of both: PASS.
  - rss after 1 h of the same workload: client VmRSS 3884 kB and 14584 kB, server 2540 kB and 10124 kB: PASS. VmRSS counts shared library pages, which the dynamic C++ binaries have and ours do not.
  - The short run (5 min rss) gave the same verdicts.
- Deliberate failure, `--rust-dir` with a client wrapper that sleeps 50 ms and sets `MOSH_PREDICTION_DISPLAY=never`, a server wrapper that runs the server under qemu-aarch64 emulation, and a launcher padded to 4.8 MB: binary_size mosh FAIL, startup 307.77 ms FAIL, keystroke 311.42 ms with 0 renders FAIL, seq 788.05 ms against 130.18 ms FAIL.
- Finding for the orchestrator (PLAN, not code): the 20 ms startup target cannot be met while keeping upstream's timing. The server sends its first frame after its send interval, `ceil(SRTT/2)` clamped to 20-250 ms, and SRTT starts at 1000 ms (upstream `transportsender-impl.h` `send_interval`, `network.cc` `SRTT( 1000 )`), so the first frame waits about 250 ms. Both implementations measure 252-257 ms. Either the target or the measured quantity has to change.
- Finding (product, not fixed): in the static musl build `getutxent` is a musl stub that always returns NULL, so `warn_unattached` never sees detached sessions. With a detached C++ session in utmp, the glibc `mosh-server` printed "Mosh: You have a detached Mosh session on this server (mosh [2300237])." and the musl one printed nothing. The musl build also warns that `libc::time_t` is deprecated (`crates/platform/src/select.rs:254`).

## 2026-10-13 Bench and static build (agent P) verified by the orchestrator

Rerun by the orchestrator at ef1c4e7 (`scripts/build-release.sh`, then `target/release/bench` short mode, side by side with C++ mosh 1.4.0 on devbox):
- Static aarch64 musl binaries: mosh 0.76 MiB, mosh-client 1.17 MiB, mosh-server 1.05 MiB; `file` says statically linked, `readelf` shows no shared libraries and no interpreter. Target under 4 MB: met.
- Client startup to its first terminal write: 1.49 ms median (C++ 3.08 ms). First frame: about 252 ms for both, set by upstream's initial 250 ms send interval (PLAN 5 target updated, INTERVENTIONS.md).
- Keystroke at 300 ms round trip with prediction: 0.09 ms median, one render per key (C++ 0.08 ms, one render).
- `seq 1 100000` until done on screen: 129.88 ms median (C++ 149.66 ms).
- Idle CPU over 60 s: 0.00% for client and server (C++ the same).
- Memory after a 5-minute active session: client 3.7 MB (C++ 14.5 MB), server 2.6 MB (C++ 10.1 MB). Part of the gap is shared-library pages the C++ binaries load and the static ones do not.
- Agent P's 1-hour run: client 3.9 MB (C++ 14.6 MB), server 2.5 MB (C++ 10.1 MB). Not rerun by the orchestrator; it will be rerun once for the README after scrollback lands.

## 2026-10-13 Scrollback server side reworked to the paced contract (agent S1)

Per the orchestrator's decision on the diff-size finding: the server never blocks the program; `scrolled_total` counts published lines; the server publishes at most W = 4856 bytes of rendered lines (four 1214-byte fragment payloads at upstream's IPv4 MTU) beyond the newest state the client acknowledged, read with a new read-only `Transport::get_sent_state_acked_state`; at least one line is published when none is outstanding.
The backlog holds only undelivered lines, capped in bytes (`mosh-server --scrollback MIB`, default 32, now in the usage line); on overflow the oldest unpublished lines become a dropped count.
One diff may carry several `ScrolledLines` instructions, one per run of held lines.
When the program exits with lines undelivered, shutdown waits until they are acknowledged, or until acknowledgements have stopped for 60 s.
`init_diff` sends only the scrolled count, because delivered lines are freed (the verbose round-trip check compares init diffs of the current state and of a copy of an older one).
PLAN 6.2, 6.3 and 9.4 rewritten.
- Live probe, `seq 1 50000`, our server with default settings: identical to seq in 10 of 10 clean runs (history complete in 9.06-9.10 s), 3 of 3 through 30% loss and 100 ms delay (123-134 s), and 3 of 3 with a 10 s blackout 2 s in (21.1 s). The server's `-v` log shows at most 2 fragments per instruction in every run (bound 7); no round-trip verification warning.
- Timing: the final screen arrives in 0.25 s on loopback and 0.35 s at 100 ms round trip, the same with and without the capability. History completes in 9.1 s on loopback and 17.3 s at 100 ms round trip with no loss.
- C++ server with the capability: 0 lines, correct final screen.
- `scrollback_server`: old-peer diffs `digests_differing=0`, `capture_on_frame_mismatches=0`; simulated lossy link with partial acks: seq 1 50000 with no gap or duplicate, no diff over the window (largest 4856 bytes); also resizes, alternate screen (0 VIM lines), scroll region, a 16 KiB backlog overflowing while the client is away (27,800 lines dropped and counted, the newest 1,404 kept in order), 374 rendered lines cell-exact in ref-term.
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 of 120. `tests/upstream-check.sh --client cxx --server rust`: 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR. vt-fuzz 200,000 and 200,000 `--frames`: 0 mismatches. `cargo test --release --workspace`: all pass. fmt and clippy (native and Windows) clean.

## 2026-10-13 Scrollback server half (agent S1) verified by the orchestrator

Rerun by the orchestrator at 43c20ec or later:
- Old peers untouched: replaying every corpus file (27 files, 108 cells, 23,868 steps) with scrollback off gives diff bytes identical to the commit before scrollback (`digests_differing=0`); with scrollback on, every diff minus the history instruction is the same bytes (`capture_on_frame_mismatches=0`).
- Live `seq 1 50000` against our server with default settings, received by a probe client and compared with `seq`: identical on clean loopback (83 instructions), through a 10 s blackout (146 instructions), and through 30% loss with 100 ms delay (442 instructions). No instruction needed more than 2 packets (bound 7).
- Rendered history lines checked cell by cell in upstream's emulator: 374 lines (wrapped, wide, colored, hyperlinked), 0 mismatches.
- Quick interop matrix, all four client/server pairs: 120 of 120. Upstream's `make check` with our server: 30 PASS, 2 XFAIL, 0 FAIL.

Known limit, for later: history arrives one 4,856-byte window per round trip plus the client's 100 ms delayed ack, so 50,000 lines take 9 s on loopback and about 17 s at 100 ms round trip (the screen itself is not delayed). Allowing more than one window in flight would speed this up; not needed for the PLAN gates.

Spawn: agent S2 (Opus, medium effort, own worktree) builds the client half (PLAN 6.4 and 6.5) and oracle 6. One agent, because it depends on the server half's wire format, which is now fixed.

## 2026-10-13 Agent P: musl detached-session warning, time_t, startup verdicts

- The static musl `mosh-server` now warns about detached sessions. `getutxent` is a stub in musl that always returns NULL, so `mosh_platform::server::getutxent_all` now reads `/run/utmp` (falling back to `/var/run/utmp`) directly on Linux x86_64 and aarch64, the same code for glibc and musl; other unix systems keep `getutxent`. The layout was measured with `offsetof`/`sizeof` against glibc `<utmpx.h>`: `ut_type` at 0, `ut_line` at 8 (32 bytes), `ut_user` at 44 (32), `ut_host` at 76 (256) on both. The record is 384 bytes on x86_64, where glibc keeps 32-bit `ut_session` and `ut_tv`, but 400 bytes on aarch64, not 384: devbox's `/run/utmp` parses as 48 records of 400 bytes, all with a valid `ut_type`, and as 384-byte records only 45 of 50 are valid. `warn_unattached` in the server is unchanged.
- New test `tests/tests/detached_warning.rs`: the apt `mosh-server` leaves a detached session, which writes a `mosh [PID]` utmp record through utempter. Then `/usr/bin/mosh-server`, our glibc server and our musl server (when built) each start a login-shell session on a pty. Each must show upstream's warning naming that session, and ours must match the C++ text. All three printed "Mosh: You have a detached Mosh session on this server (mosh [2495245])." With the musl server built from the old code the test failed: "[rust-musl] no warning about mosh [2414787]; warning lines: []". The test holds `/tmp/mosh-rs-interop.lock`: during an interop run, each session that is still waiting for its client counts as detached. In one unlocked run the C++ server saw 2 detached sessions and ours saw 1.
- Why `/usr/bin/mosh-server` printed nothing in the earlier check: its session child prints the motd and the warning with stdio and then execs the shell without flushing. My harness had redirected `mosh-server new`'s stdout to a file, so glibc made stdout fully buffered, and the buffer was lost at the exec. Under strace the child opened `/run/motd.dynamic` and `/var/run/utmp` but never wrote. Through `ssh -t` (the launcher) stdout is a terminal, the output is line-buffered, and the warning appears. Our server writes without buffering, so it prints in both cases. This is a behavior difference only when stdout is not a terminal; it was not changed (`crates/mosh/src/server/mod.rs` is agent S1's).
- `select.rs` no longer uses the `libc::time_t` alias, which is deprecated on musl (`From` conversions into the `timespec` field types). Release builds for aarch64 musl and x86_64 musl: 0 warnings. clippy `-D warnings` is clean for aarch64 gnu, both musl targets and x86_64-pc-windows-gnu.
- bench startup verdicts now follow the revised PLAN 5. The first frame must be no slower than C++, with the same tolerance as seq. The first write must be under 20 ms. Results: first frame 252.39 ms vs 257.10 ms PASS, first write 1.50 ms PASS. With the 50 ms wrapper: 308.33 ms FAIL and 52.49 ms FAIL.

## 2026-10-13 Detached-session warning (agent P) verified by the orchestrator

`cargo test --release -p mosh-tests --test detached_warning` at 6e0ab0b: the C++ server, our glibc server and our static musl server each print "Mosh: You have a detached Mosh session on this server (mosh [PID])." for the same detached session, identical text. The static musl platform crate builds with 0 warnings. The login-record layout is 400 bytes per record on arm64 and 384 on x86_64 (agent P measured both; the orchestrator's brief had said 384 for both).
Not a bug: when `mosh-server new`'s output goes to a file instead of a terminal, the C++ session child loses its banner in an unflushed buffer while ours prints it; in real use that output goes to the session's terminal, where both print it.

## 2026-10-13 Scrollback, client side (agent S2)

PLAN 6.4 and 6.5, opt-in with `mosh --scrollback` (the launcher starts the client with `MOSH_SCROLLBACK=1`); without it the client is unchanged.
The server now announces scrollback: `Complete::scrollback_on` makes its first diff after the capability hold a `ScrolledLines` (no line when none is due), so the client tells an old server from a quiet one on the first frame.
The client starts in the alternate screen as upstream; on the first frame it either leaves it (scrolling the host's screen down to the cursor into history) and draws on the main screen, injecting history lines at row 1 and scrolling them off with a line feed on the last row, or stays as upstream with a 5 s notice.
History lines are re-drawn through a one-row emulator at the host's width before they reach the host; lost lines become one dim `[mosh: N lines of history were dropped]` line in place.
In this mode `new_frame` has no scroll shortcut and repaints without `CSI H CSI 2J` (tmux 3.4 moves the screen into history on it); frames of another size than the window are held until the server applies the resize, then fully repainted.
`mosh-server --scrollback` takes a `K` suffix for KiB.
- Oracle 6, `tests/scrollback.sh` (tmux 80x24, history-limit 200000):
  - clean, `mosh --scrollback localhost`, `seq 1 50000`, with the escape-key help bar up while history arrives: 50000 numbers, in order, no gap, no duplicate, no overlay text (9 s).
  - loss30 (30% loss each way, 100 ms delay): the same (136 s, 165 s, 144 s).
  - resize wider and narrower between `seq 1 3000` and `seq 3001 6000`: exact.
  - oldserver (`--server=/usr/bin/mosh-server`): alternate screen, notice shown, nothing on the host's main screen after exit.
  - dropped (`--scrollback 16K`, 8 s blackout during `seq 1 200000`): 1-1672, then `[mosh: 197079 lines of history were dropped]` (dim), then 198752-200000, exact.
  - Not exact, and the script checks they are exactly this (PLAN 6.4 known limits): vim on the same command line after `seq 1 3000` erases the screen rows 2978-3000 without scrolling them, so they are missing (nothing from vim is in history); `seq 1 100; clear` loses 78-100 the same way. Height shrink 24 -> 18: tmux itself moves 2978-2983 into its history and they come again later (6 duplicates); the server cuts 2996-3000 and the prompt as upstream's emulator does, and 2995 is overwritten by the echoed command. Height growth 24 -> 30: tmux pulls 2972-2977 back from its history and the repaint overwrites them.
  - Questions for the orchestrator, all server side: capture rows that the remote erases with ED 2 or covers on entering the alternate screen (would close the clear and vim gaps without changing the screen the program sees); make the emulator's resize keep the bottom like host terminals (would close the shrink and grow cases, but changes the screen the program sees after a resize, a deviation from upstream).
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 of 120. The same with the Rust client in scrollback mode (a wrapper that sets `MOSH_SCROLLBACK=1`, passed as `--client PATH`) against both servers: 60 of 60; the same wrapper shows `alternate_on=0` with our server and the notice with the C++ server.
- `tests/upstream-check.sh --client rust --server rust`: 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR.
- `cargo test --release --workspace`: all pass; `scrollback_server` still `digests_differing=0 capture_on_frame_mismatches=0`. fmt and clippy (native and Windows) clean; Windows `mosh` and `mosh-client` build.

## 2026-10-13 Scrollback client half (agent S2) verified by the orchestrator

Rerun by the orchestrator at 1ef23f3:
- Old peers untouched: corpus replay with scrollback off is byte-identical to before scrollback (`digests_differing=0`, `capture_on_frame_mismatches=0`); screen redraws match upstream's exactly (3024 frames including 756 across a resize, 0 mismatches).
- Oracle 6 (`tests/scrollback.sh`, tmux 80x24 with a 200,000-line history, `mosh --scrollback`):
  - `seq 1 50000`, clean: all 50,000 lines exactly once, in order, no gaps, nothing from mosh's overlay (9 s).
  - Same through 30% loss and 100 ms delay: exact (140 s).
  - Width resize in the middle: exact.
  - Old C++ server: the client stays on the alternate screen as upstream, shows a notice, nothing reaches history.
  - Backlog overflow: one dim line `[mosh: 197079 lines of history were dropped]` in the right place, everything else exact.
  - Not exact yet: vim and `clear` lose the rows that were on screen when they started (2978-3000 and 78-100); agent S2 is adding capture of those rows. Height shrink and grow lose or repeat about 6 rows, because host terminals keep the bottom of the screen on resize and upstream's emulator keeps the top; this stays a documented limit (changing the emulator would change what the program sees).
- Quick interop matrix, default clients: 120 of 120. Upstream's `make check` with both of our binaries: 30 PASS, 2 XFAIL, 0 FAIL.

## 2026-10-13 Scrollback: rows covered by the alternate screen or erased by clear are history (agent S2)

Per the orchestrator's decision (1): the server now captures, through `Framebuffer::capture_screen`, the rows from the top through the last non-blank one (never rows marked alternate) in two places where they would otherwise vanish without scrolling.
On entering the alternate screen (DEC 47, 1047 or 1049 set while the flag is off) they are captured before anything draws over them and marked, so they are not captured again if they later scroll off.
On erasing the whole display with a full-screen scrolling region and the flag off (ED 2, or ED 0 from the home position, which tmux 3.4 treats the same way) they are captured before the erase.
`clear` on devbox sends `CSI H CSI 2J CSI 3J` for TERM=xterm-256color (what mosh-server sets), `CSI H CSI J CSI 3J` for tmux-256color and `CSI H CSI J` for screen. ED 3 stays ignored, as upstream: tmux over ssh erases its whole history on it (after `seq 1 30; clear` it kept nothing; with `CSI H CSI 2J` alone it kept 1-30), but mosh leaves the host's history to the host.
Decisions (2) and (3) taken: no change to the emulator's resize; PLAN 6.6 stays deferred. PLAN 6.2 and the 6.4 known limits updated.
- Oracle 6, `tests/scrollback.sh`, all variants: clean, loss30 (147 s), vim, clear and resize exact (`gaps=none`, nothing from vim in history); shrink and grow exactly as documented (6 duplicates and 2995-3000 missing; 2972-2977 missing); oldserver and dropped as before.
- `scrollback_server`: `digests_differing=0 capture_on_frame_mismatches=0`; the alternate-screen test now expects the screen captured on entry and 1-6000 exact; new test: `clear` in both forms keeps 1-3000 exact. `terminal_conformance`, `statesync_roundtrip`: pass. vt-fuzz 200,000 cases and 200,000 `--frames`: 0 mismatches.
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 of 120; scrollback client column: 60 of 60. `tests/upstream-check.sh --client rust --server rust`: 30 PASS, 2 XFAIL, 0 FAIL. `cargo test --release --workspace`: all pass. fmt and clippy (native and Windows) clean.

## 2026-10-13 Phase 4: scrollback oracle passes, gate pending CI and the full-blackout matrix

Rerun by the orchestrator at 2d9f52a:
- Oracle 6 (`tests/scrollback.sh`, tmux with a 200,000-line history, `mosh --scrollback`): `seq 1 50000` exact on a clean link (9 s) and through 30% loss (131 s); `seq 1 3000`, vim, `seq 3001 6000` exact with nothing from vim in history; `seq`, `clear`, `seq` exact; width resize exact; old C++ server falls back to upstream behavior with a notice; backlog overflow shows one dim "lines were dropped" line in place. Height shrink and grow keep their documented limit (about 6 rows lost or repeated, because host terminals keep the bottom of the screen on resize and upstream's emulator keeps the top).
- Old peers untouched: corpus replay with scrollback off byte-identical to before scrollback; 3024 screen redraws identical to upstream; statesync round trip 2214 steps 0 mismatches; vt-fuzz 200,000 frame cases 0 mismatches.
- Quick interop matrix, all four client/server pairs: 120 of 120.

Next: the live scrollback tests take the shared port lock (one collided with a concurrent run), the full matrix with 2-minute blackouts including the scrollback client, and the PLAN 6.6 stretch (forward the alternate screen to the host terminal).

## 2026-10-13 Scrollback: alternate screen forwarded to the host (agent S2, PLAN 6.6)

- First, a small commit (0a1915e): the live tests in `scrollback_server` hold `/tmp/mosh-rs-interop.lock` for their sessions (one at a time, and never alongside an interop, scrollback or bench run). `tests/scrollback.sh` already held it for its whole run.
- Design. In scrollback mode the server's emulator keeps a real alternate screen, as tmux does: DEC 47/1047/1049 set puts the main screen's rows (and for 1049 the cursor) aside and the program draws on a blank screen; reset brings them back. The saved rows follow resizes; RIS drops them. This replaces the capture-on-entry from the previous commit: the main screen's rows are no longer captured when vim starts. They come back when vim exits and scroll into history later, once, in the order a host terminal over ssh shows. Without scrollback mode only the flag is recorded, as before.
- Wire: new `HostBuffers::Instruction` field 10, `ScreenMode { alternate_screen }`, sent in scrollback mode only, when the flag changes (after scrolled lines, before resize and host bytes). `Complete` equality counts the flag in scrollback mode only.
- Client: on the flag turning on, it injects the lines received so far, writes `CSI ?1049h` and repaints; on turning off, it writes `CSI ?1049l` (the host restores its main screen), injects the lines received meanwhile and repaints, all in one synchronized-output write. While the host is in its alternate screen no line is injected. Exit and suspend leave the host's alternate screen first.
- Oracle 6, `tests/scrollback.sh`, new variant `altscreen`: `seq 1 3000`, vim (text typed, `:q!`), `less` on a 3000-line file (`q`), `seq 3001 6000`. The host is in its alternate screen while vim and less run. After each exits, the screen equals the one a plain tmux pane shows for the same keys. History is 1-6000 exactly, plus 3 command lines. A deliberately corrupted reference makes it fail ("screen differs from direct-vim.txt: 25d24|< corrupt|"). All variants: clean, loss30 (110 s), vim, clear, altscreen, resize exact; shrink and grow as documented; oldserver and dropped as before.
- `scrollback_server`: `digests_differing=0 capture_on_frame_mismatches=0`. The capture-on frame check now covers every step until the program first enters the alternate screen; after that, frames differ by design (20 of 108 cells, 4294 steps). In every cell the client that applied the diffs ends with the server's screen. The alternate-screen test checks that the main screen comes back on leaving. `transport_roundtrip`'s unknown-field probe moved from field 10 to 11. `terminal_conformance`, `statesync_roundtrip`: pass. vt-fuzz 200,000 and 200,000 `--frames`: 0 mismatches.
- `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 of 120; scrollback client column: 60 of 60. `tests/upstream-check.sh --client rust --server rust`: 30 PASS, 2 XFAIL, 0 FAIL. `cargo test --release --workspace`: all pass. fmt and clippy (native and Windows) clean; Windows `mosh` and `mosh-client` build.

## 2026-10-13 Phase 4: every devbox check passes, gate pending CI

Rerun by the orchestrator at 728e468 (alternate screen forwarded to the host terminal, PLAN 6.6):
- Oracle 6 (`tests/scrollback.sh`): all 10 variants pass. `seq 1 50000` exact on a clean link (9 s) and through 30% loss (136 s); vim, `clear`, and a vim plus less session each leave history exactly 1..6000 with nothing from the full-screen programs; while vim and less run the host terminal is in its own alternate screen, and after they exit the visible screen equals a plain tmux pane given the same keys; width resize exact; old C++ server falls back with a notice; overflow shows the dropped-lines line. Height shrink and grow keep their documented limit.
- Old peers untouched: corpus replay with scrollback off byte-identical to before scrollback; 3024 screen redraws identical to upstream; statesync 0 mismatches.
- Full interop matrix with 2-minute blackouts: default clients, all four client/server pairs, 120 of 120 (401 s); the Rust client in scrollback mode against both servers, 60 of 60 (257 s).
- Upstream's `make check` with both of our binaries (agent S2's run at 728e468): 30 PASS, 2 XFAIL, 0 FAIL.

Gate not declared: CI is red until the laptop pushes branch `ci/terminfo`. Next: Phase 5 (one-hour bench rerun for the README, macOS compile check, README compatibility matrix and measured numbers, release workflow).

## 2026-10-13 Launcher: Windows ProxyCommand quoting fixed (agent C)

- Fix for the Phase 3 laptop failure (`CreateProcessW failed error:2`, INTERVENTIONS "Phase 3 Windows test, first pass"): the ProxyCommand words are quoted by `mosh_platform::process::proxy_command_quote`, POSIX single quotes on unix (unchanged byte for byte: `ProxyCommand='.../mosh' '--family=prefer-inet' --fake-proxy -- %h %p` seen in `ps` during a session) and Windows command-line rules on Windows (`windows_command_line_quote`), because Win32-OpenSSH runs no shell for ProxyCommand: `ssh_proxy_connect` passes the whole string to its `posix_spawnp`, whose `build_commandline_string` (contrib/win32/win32compat/misc.c) hands it to CreateProcessW unchanged when it starts with `"`, else quotes only up to the first space after `.exe`.
- New check `windows_proxy_command_quoting` in `tests/tests/launcher.rs`: plain path unchanged, `C:\Program Files\mosh-rs\mosh.exe` -> `"C:\Program Files\mosh-rs\mosh.exe"`, `C:\dir with space\` -> `"C:\dir with space\\"`, forward slashes unchanged, embedded quotes escaped.
- `cargo test --release -p mosh-tests --test launcher`: 5 passed, 0 failed (dual-stack case included). fmt, clippy native and Windows clean; Windows `mosh` builds. Laptop rerun of Phase 3 steps 3-6 needs fresh Windows binaries.

## 2026-10-13 Phase 5: one-hour bench rerun by the orchestrator, README numbers

`target/release/bench --hour` at 7cab26f on devbox, side by side with C++ mosh 1.4.0. Every PLAN 5 target passes:
- Idle CPU over 60 s: 0.00% client and server (C++ the same).
- Memory after a one-hour session: client 4228 kB (C++ 14596 kB), server 2564 kB (C++ 10124 kB).
- Static aarch64 binaries: mosh 797,824 B, mosh-client 1,225,216 B, mosh-server 1,101,520 B (target under 4 MB).
- Client startup to first write 1.40 ms (target under 20 ms; C++ 2.94 ms); first frame 252.2 ms (C++ 256.6 ms).
- Keystroke at 300 ms round trip with prediction: 0.09 ms, one render (C++ 0.08 ms, one render).
- `seq 1 100000` until done on screen: 110 ms (C++ 130 ms).
The README performance table now holds these numbers. Also verified: `cargo check` for the release targets not built so far, aarch64-apple-darwin and x86_64-pc-windows-msvc, passes with no errors or warnings.

## 2026-10-13 Test robustness for CI runners (agent S2)

- `tests/scrollback.sh` prints the variant list and the tmux version, and checks that tmux is 3.2 or later and that less is installed. A failing variant now prints its reason, the checker's counts line and first unexpected lines, any screen diff, and the screen at the failure, all to stdout.
- Waits are generous (120 s for screens). History waits go by progress: they give up only after the history has stopped growing for 60 s, or 120 s for loss30. The dropped variant's blackout is 20 s, with 10 s of margin before the flood.
- CI's altscreen failure ("direct run: last screen line is not "$" ... "q"") was a race in the test: it waited for text that the typed `less` command line already held, and less discarded the early `q`. It now waits until the pane is in the alternate screen and less shows its first page.
- shrink and grow now wait until history has caught up with the screen before resizing. Done earlier, while history was still arriving, tmux's own resize moves rows into the middle of history.
- Client: an injected line's line feed now goes to `CSI 9999;1H`, the last row by clamping. Before, it used the window size, so a resize the client had not processed yet (a taller host) left the line feed above the last row, and the line was lost. PLAN 6.4 updated.
- Under load on devbox (tests and `stress-ng --cpu 12` pinned to 4 cores, load average 18): all 10 oracle variants pass. Unpinned, with `stress-ng --cpu 6` (load 6): all pass.
- `wrong_prediction_is_removed`: a prediction is shown only in a confirmed epoch, and an echo that takes more than 50 ms on the server (upstream's ECHO_TIMEOUT) makes the epoch tentative again, so on a loaded machine `q` may legitimately not be predicted. Reproduced under the same load with the old test: it failed in 6 of 18 runs ("the prediction of q never showed"), for both the C++ and our client. The shell loop now swallows `q` without ending, and the test repeats the attempt (two echoed keys, then `q`) up to 10 times. A `q` on screen can only be the prediction, so the assertion keeps its meaning. New test under that load: 10 of 10 runs passed; 1 of 20 client sessions needed a second attempt.

## 2026-10-13 Minimum Rust 1.89 (agent B, review G3)

Portable code instead of a pinned toolchain: the two field-level `non_snake_case` allows moved to their structs, and the vt-fuzz byte-string arrays are typed local consts.
`rust-version = "1.89"` (forced by `aes` 0.9.3; cargo refuses 1.88.0). Verified: `cargo +1.89.0` and `cargo +1.91.1` `clippy --workspace --all-targets -- -D warnings` clean and `test --release --workspace --no-run` build, as does stable 1.99.

## 2026-10-13 Review G6, G7, G8, G10, G11, G12: launcher and version strings (agent C)

- G6: one `mosh::BUILD_VERSION` (the package version); `mosh --version` prints `mosh-rs 0.1.0 [build 0.1.0]`, `mosh-client --version` `mosh-client (mosh-rs 0.1.0) [build 0.1.0]`, `mosh-server --version` `mosh-server (mosh-rs 0.1.0) [build 0.1.0]`.
- G7: default client is `mosh-client` from PATH on unix (mosh.pl); the sibling `mosh-client.exe` lookup is Windows-only (`mosh_platform::process::default_client_program`), PLAN section 8.
- G8: Getopt::Long's parsing ported (prefixes, case-insensitive names, no bundling, negations, next-argument values), `--experimental-remote-ip` ported, `chomp` exact, Perl's messages and `$!` exit codes reproduced. New differential test `options_and_messages_match_mosh_pl`: 73 cases run through /usr/bin/mosh and ours with a fake ssh and client, 0 different; usage text differs only by PLAN 9.3 and `--scrollback`.
- G10: the `--fake-proxy` relay retries interrupted reads. G12: `expect("digits only, checked above")`.
- G11: `resolvename` ported (numeric first, then AI_CANONNAME and the canonical name) over `mosh_platform::netdb::getaddrinfo`. New test `proxy_resolves_canonical_name_like_mosh_pl`: for `mosh-alias` (devbox /etc/hosts) the proxy's choice and messages equal mosh.pl's for `--family` auto, prefer-inet6, prefer-inet, all and inet6; the previous launcher differed in 4 of the 5.
- `cargo test --release -p mosh-tests --test launcher`: 7 passed, 3 runs in a row (dual-stack case included). `client_e2e`: 7 passed. fmt; clippy native, Windows and Rust 1.89: clean. Windows `mosh.exe` and `mosh-client.exe` build.

## 2026-10-13 Review G1, G2, G4, G5, G9, G13 (agent S2, 50a21a7)

- G1, blocking: one history line made the client allocate without bound. Reproduced with the real client against a new test server (`hostile-scrollback-server`, `tests/tests/scrollback_hostile.rs`). Before: 1,000,000 line feeds in one line took the client above 2.1 GB within 0.85 s; 200,000 took 840 MB and made 180,026 host lines.
- Fix: the client cuts a line at ceil(frame width / host width) + 1 host rows (4 to 1024) and writes a dim notice line. It rejects lines over 256 KiB as malformed (PLAN 9.8). The server sends a longer rendered row as a dropped line instead. After: 6.2 MB with the line rejected, and 3.6 MB with 30 host lines and the notice.
- G2: line numbers above 2^63 are rejected; the `u64::MAX` case no longer shows or wraps.
- G4: `crates/mosh` crate doc, `forbid(unsafe_code)`, `deny(missing_docs)`. G5: PLAN 9.11a (swprintf truncation). G9: `unreachable!`. G13: `Row::alternate` doc.
- Gates: `tests/scrollback.sh` 10 of 10 variants. Workspace tests all pass, except two failures caused by the setup, not by this change. `launcher::options_and_messages_match_mosh_pl` fails in any checkout whose target path is not canonical: the test passes `tests/../target/release/mosh` and our launcher prints the canonical path. It passes with `CARGO_TARGET_DIR` set to the absolute path. `client_e2e::idle_cpu` failed once (it uses ports 61340-61341 without the lock while another run was active) and passed on rerun. `INTEROP_BLACKOUT_MS=10000 tests/interop.sh --client all --server all`: 120 of 120. fmt and clippy (native and Windows) clean.
- `tests/scrollback.sh`, from the CI log of 1cb0eb6 ("server exited unexpectedly", then "FAIL vim (0 s):" with no reason): a new tmux session on the socket of a server that was just killed can fail. Each session now gets its own socket, a failed `tmux new-session` reports tmux's message, and a failure without a recorded reason says so and prints the screen. Under `stress-ng --cpu 12` pinned to the same 4 cores (load average 12): 10 of 10 variants pass.

## 2026-10-13 Scrollback tests split for CI shards (agent S2)

- `tests/scrollback.sh` takes variant names as arguments or in `SCROLLBACK_VARIANTS` (comma-separated), and `--list` prints them. All 10 variants pass alone. Measured alone: loss30 173 s; every other variant 23 s or less.
- `scrollback_server`: the 30%-loss live case now runs `seq 1 10000` instead of 50000: 24 s instead of 158 s. It proves the same thing: lossy link, every instruction within the fragment bound (max 2 of 7), exact history (9977 lines, 0 lost). The full 50,000 lines through the same loss are still `tests/scrollback.sh loss30`. No `MOSH_CI_FULL` guard was needed. All 13 tests pass with `--test-threads=13` in 55 s and alone with `--exact`.
- Commands for each shard: new `tests/README.md`.

## 2026-10-13 CI altscreen failure fixed in the test (agent S2)

- `tests/scrollback.sh altscreen` typed the vim command before `seq 1 3000` had finished on slow links, because the first prompt already satisfied its wait. It now waits for the line 3000 and then the prompt. Reproduced through udp-impair with 100 ms delay (4 of 4 failed before the fix, 5 of 5 passed after it). Details in INTERVENTIONS.md. All 10 variants pass on devbox.

## 2026-10-13 Phase 1 gate: PASSED

Green CI at c64e97d (run 38435551993, fmt, clippy and every test shard on x86_64 and arm64), which contains all Phase 1 work and every Phase 1 review fix. Every number below was rerun by the orchestrator (entries above give the full output):
- Crypto: `cargo test --release -p mosh-tests --test crypto_conformance`: 2000 random packets byte-identical between Rust and upstream's C++ in both directions, tampered packets rejected by both, 14 malformed keys rejected by both, RFC 7253 vectors pass.
- Transport: `transport_roundtrip` 5 passed (including every malformed fragment and diff a keyed peer can send: error, no crash); `tests/transport-impaired.sh`: sync converges with 30% loss and an address change (server re-attached to the new port), with 5% loss, and through a 3 s blackout.
- Terminal: `terminal_conformance`: 27 corpus files cell-exact at two sizes, 3024 screen redraws byte-exact including 756 across a resize, 0 mismatches. Differential fuzzing: 1,000,000 cases (seed 1), 200,000 (seeds 2, 11, 31, 77), 200,000 `--frames` cases: 0 mismatches. `terminfo_table`: 2830 terminal names, 0 mismatches; color counts for 2832 cases match upstream.
- Platform and launcher: `platform_tty` 1 passed, `launcher` 7 passed (73 option and message cases identical to the real mosh.pl, dual-stack names, canonical-name resolution); Windows cross-build of the shipped crates.
- Tools: `udp_impair` 6 passed; upstream's own `make check` with upstream's binaries 30 PASS, 2 XFAIL.
- Reviews: every item in `reviews/2026-10-13-phase1-terminal-network.md`, `reviews/phase1-terminal.md`, `reviews/phase1-transport.md` fixed or answered with its commit.

## 2026-10-13 Phase 2 gate: PASSED

Same green CI run at c64e97d. Rerun by the orchestrator:
- `INTEROP_FULL=1 tests/interop.sh --client all --server all`: 120 of 120 cells (C++ and Rust clients x C++ and Rust servers x clean, 5% loss, 30% loss with 200 ms delay and reordering, 2-minute blackout, client address change x six screen scenarios), 401 s.
- `tests/upstream-check.sh` (upstream's own `make check`, 32 tests) with our server, our client, and both: 30 PASS, 2 XFAIL, 0 FAIL, 0 ERROR each; the restored round-trip check makes it fail (25 ERROR) on a deliberately wrong server diff.
- `tests/interop-setup-loss.sh --run`: session setup with 30% loss from the first packet, 40/40 for each Rust pair (C++ pair 39/40); both clients send 4 packets in 12 s with no server.
- `statesync_roundtrip`: 27 files, 2214 steps, 0 mismatches; 7 malformed diffs rejected without a crash.
- `client_e2e` 7 passed, `server_e2e` 4 passed, `prediction_e2e` 3 passed: typed characters appear in 2 ms at 300 ms round trip with prediction (C++ 2 ms), about 313 ms without; wrong predictions removed after 373 ms (C++ 368 ms). `detached_warning`: identical text from the C++, glibc and musl servers.
- Idle CPU 0.00% for our client and server.
- Reviews: the line-by-line review of the client, prediction overlay, statesync and server (`reviews/2026-10-13-phase2-4-client-server-scrollback.md`, run from the laptop) found 0 deviations from upstream in those files; all 13 findings fixed (G1 to G13, resolution log filled).

## 2026-10-13 Phase 4 gate: PASSED

Green CI at 157a55a (PR #12; fmt, clippy and every test shard on x86_64 and arm64), which contains all scrollback work and the fixes for every review finding. Rerun by the orchestrator (entries above give the full output):
- Oracle 6, `tests/scrollback.sh` (tmux with a 200,000-line history, `mosh --scrollback`), all 10 variants: `seq 1 50000` exact on a clean link and through 30% loss; vim, `clear`, and vim plus less each leave history exact with nothing from the full-screen programs, and the host shows the pre-vim screen after vim exits; width resize exact; old C++ server falls back with a notice; backlog overflow shows the dropped-lines line in place. Height shrink and grow keep their documented limit (PLAN 6.4). The altscreen variant passed 3 of 3 on main after the test race fix (agent S2 reproduced the CI failure 4 of 4 with a 100 ms link delay: the test typed `vim` before `seq` output had arrived).
- Old peers untouched: corpus replay with scrollback off byte-identical to before scrollback (`digests_differing=0`, `capture_on_frame_mismatches=0`).
- Full interop matrix with 2-minute blackouts still green: 120 of 120 (default clients), 60 of 60 (Rust client in scrollback mode against both servers).
- Hostile server (`scrollback_hostile`): a history line of 1,000,000 line feeds is rejected with the client at 6.2 MB peak and alive (before the fix: over 2.1 GB); a line numbered u64::MAX is rejected at 3.1 MB.

## 2026-10-13 run-all.sh: subsets for CI shards, parallel drivers (agent H)

- `RUN_ALL_ONLY=<comma list of names or globs>` runs a subset; names are cargo integration tests (`client_e2e`), drivers (`scrollback.sh`) and `lib-bin-doc` (workspace lib, bin and doc tests). A pattern that matches nothing exits 2 with the list of names. `MOSH_CI_FULL=1` is exported unless it is 0.
- Everything is built first. The cargo tests run one at a time as before (server_e2e and scrollback_server share port 61200). `scrollback.sh` runs after them because it owns 61500-61599 and kills whatever is bound there, where detached_warning and scrollback_hostile bind. All other drivers start at once alongside the cargo lane; the interop drivers serialize on their lock, and `transport-impaired.sh` now holds `/tmp/mosh-rs-transport-impaired.lock` for its fixed ports.
- Each item prints one block (last 15 lines when it passed, all of it when it failed; full logs in `target/run-all/`), then one PASS/FAIL line per item and the wall time.
- Devbox wall time, warm build. Old script: 1247 s, but that run overlapped another worktree's tests (scrollback_hostile took 169 s instead of 21 s, interop waited on the lock), so it overstates the gain; the new run's per-item times add up to 885 s when run one after another. New script before rebasing onto c64e97d: 563 s, all 21 pass. After rebasing (scrollback_server is now 55 s, agent S2): 495 s, all 21 pass, with the shared checkout holding `/tmp/mosh-rs-interop.lock` for about 190 s of it (sampled with `lslocks` every 5 s). An earlier run of the same tree took 751 s because scrollback_hostile, which also takes that lock, waited about 200 s for it. Per-test times in parallel match the serial ones within 1-2 s.
- The critical path is now the cargo lane (about 290 s) plus `scrollback.sh` (about 150-250 s). Moving `scrollback.sh` or the two cargo tests off 61500-61599 would let it run alongside. On one machine, everything that takes the interop lock (interop drivers, scrollback.sh, scrollback_hostile) also waits for other checkouts' runs.
- Proofs: `RUN_ALL_ONLY=client_e2e,transport-impaired.sh` ran exactly those two (wall 70 s, items 62 s and 34 s). A throwaway failing driver printed its whole output as a FAIL block and the run exited 1.

## 2026-10-13 udp_impair blackout test made race-free (agent H)

- `tests/tests/udp_impair.rs` `blackout_silences_then_resumes` now drains in-flight replies before the blackout and judges packets by send time; see INTERVENTIONS. `cargo test --release --workspace --test udp_impair`: 6 passed. Blackout test 30 of 30 under `stress-ng --cpu 8`; fails (155 of 155 answered) with the proxy's blackout disabled.

## 2026-10-13 Phase 3 gate: PASSED (user decision)

- The user accepted the Wi-Fi off-and-on roaming test as the Phase 3 pass condition, in place of the sleep-and-wake step in PLAN section 3 item 8 (decision recorded by the laptop, commit 1e53737 "Phase 3 gate accepted by the user on the roaming result"). Sleep and wake itself was not run.
- Evidence (INTERVENTIONS.md, laptop entries): in Windows Terminal with OpenSSH_for_Windows 9.5p2, `mosh.exe devbox` connects after the ProxyCommand quoting fix (8a32d96); connecting, typing, `ls --color`, vim, UTF-8 output, Ctrl-C, resizing, and quitting with Ctrl+6 then `.` all worked; `mosh.exe --server=<our mosh-server> devbox` (Rust client on Windows, Rust server on devbox over Tailscale) kept `top` updating while Wi-Fi was turned off and on, with no reconnect and no error. A folder path with a space works with the double-quoted ProxyCommand.
- Built on devbox: `mosh.exe` and `mosh-client.exe` cross-compiled for x86_64-pc-windows-gnu, importing only system DLLs; the MSVC target passes `cargo check`.
- Not tested on Windows: Ctrl-^ Ctrl-Z, idle CPU in Task Manager, sleep and wake. The Fable review of `crates/platform` (the Windows console path) is running from the laptop; its findings will be handled as they land, and a blocking one reopens this gate.

Also in this commit: the unused `unicode-width` and `lexopt` declarations are removed (PLAN section 2 updated), the workspace `repository` URL is the GitHub repository, and the two old entries that said CI builds mosh 1.4.0 carry a correction (it builds decd9b7). Checked: release build, `cargo clippy --workspace --all-targets -- -D warnings` on stable and on 1.89.0, Windows build of `mosh`, `cargo fmt --all --check`.

## 2026-10-13 CI client_e2e "non-zero exit" fixed in the tests (agent S2)

- The exit-status waits in client_e2e, server_e2e and launcher matched the typed command line's `MOSH_EXIT=` and could capture before the shell printed the status. They now wait for a whole `MOSH_EXIT=<n>` line. Both clients exited 0 in all 100 devbox runs (50 each, with and without load). `command_output_and_exit` under load: 15 of 15. Details in INTERVENTIONS.md.

## 2026-10-13 Phase 5 gate: PASSED; v0.1.0 released

- Performance: every PLAN section 5 target measured by `tools/bench --hour` against C++ mosh 1.4.0 on devbox and published in the README (entry "Phase 5: one-hour bench rerun" above): idle CPU 0.00%, memory after one hour 4.1 MiB client and 2.5 MiB server (C++ 14.3 and 9.9), static binaries 0.8 to 1.2 MB, client startup 1.40 ms, keystroke 0.09 ms with one render, `seq 1 100000` 110 ms (C++ 130 ms).
- Release: the laptop ran the Release workflow dry run (run 38447825710, all four targets built, smoke-tested and packaged), the user made the repository public ("public if it's ready", 13:50 Central) and tagged v0.1.0 ("yes tag v0.1.0", 13:58 Central) on main at 7938fdc. The release at github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.0 carries, each with a sha256 file: aarch64-apple-darwin (996 KB), aarch64-unknown-linux-musl (1.24 MB), x86_64-unknown-linux-musl (1.32 MB), x86_64-pc-windows-msvc (573 KB).
- Open after the gate: the Phase 3 platform review (`reviews/2026-10-13-phase3-platform.md`, 0 blocking, 4 should-fix) is with agent C; its fixes land through CI as 0.1.x follow-ups.
## 2026-10-13 Platform review (Phase 3) P1-P13 (agent C)

- P1: the Windows console input thread starts inside `raw_mode_on`, so a failure is an error the client prints before exiting 1, not a panic. P2: Ctrl+Break becomes SIGINT through a console control handler (clean shutdown, console restored); the waiting `mosh.exe` ignores it (PLAN 7). P3: inline const array, the allow is gone. P4: utmp read listed as PLAN 9 item 15; a file that is not whole records gives no records. P5-P8, P11-P13 fixed as the review proposed. P9: one `perror` in `mosh_platform::stdio`, used at the five sites and by the client; Windows `strerror` without the " (os error N)" suffix. P10: `netdb.rs` keeps the raw `sin6_flowinfo` as `socket.rs` and std do; new test `numeric_addresses_resolve_the_same_in_both_paths` in `tests/tests/launcher.rs` resolves scoped IPv6 literals through both files and requires identical results (a planted wrong scope byte order fails it: `fe80::1%16777216` against `%1`).
- Gates on devbox: platform_tty 1, launcher 8 (including the resolve test), client_e2e 7, server_e2e 4, detached_warning 1 passed. fmt; clippy -D warnings on stable and 1.89.0; Windows clippy and build of `mosh-platform`, `mosh`, `mosh-client`: clean. Windows behavior of P1, P2, P5, P6, P8, P13 is on the laptop checklist (INTERVENTIONS.md).

## 2026-10-13 macOS launcher fixed; first real session from a Mac (agent C)

- The launcher's address lookup read the socket address family at the Linux offset, so on macOS it found no address for any host. `netdb.rs` now uses the same decoder as `socket.rs` (`socket::from_sockaddr`, through libc's `sockaddr`).
- On the Apple silicon Mac (macOS 15.8), built from this branch: `mosh --fake-proxy -- 127.0.0.1 22` prints `MOSH IP 127.0.0.1` (before: "Could not connect to 127.0.0.1, last tried :"); a session from the Mac to devbox over the network with our launcher, client and server ran a typed command on devbox and exited with `[mosh is exiting.]`, status 0; `mosh --local` works; `platform_tty` 1 and `launcher` 8 passed there, including 73 option cases identical to Homebrew's mosh.pl. `client_e2e` cannot run on the Mac until it finds the C++ mosh on PATH (hard-coded /usr/bin).
- Devbox: platform_tty 1, launcher 8, client_e2e 7, server_e2e 4, detached_warning 1 passed; fmt; clippy -D warnings stable and 1.89.0; Windows clippy and build: clean. README: the macOS real-session box is ticked, with the note that the v0.1.0 macOS launcher has this bug until the asset is rebuilt.

## 2026-10-13 Tests portable to macOS; CHANGELOG 0.1.1 (agent C)

- The real mosh 1.4.0 used as a comparison in the tests is found on PATH (`mosh_tests::upstream`, never a mosh-rs build), with a skip message when missing; `client_e2e` finds the server pid with `lsof` where `ss` is missing.
- Mac (Apple silicon, macOS 15.8): `client_e2e` 7 passed (`idle_cpu` without its pidstat measurement). Devbox: client_e2e 7, server_e2e 4, prediction_e2e 3, detached_warning 1, launcher 8, scrollback_server `cxx_server_ignores_capability` 1 passed; fmt; clippy -D warnings stable and 1.89.0 clean.
- CHANGELOG has `[0.1.1] - unreleased` with the macOS launcher fix and the platform review fixes P1-P13; the README's macOS section says v0.1.1 carries the fix once tagged. Nothing tagged.

## 2026-10-13 Remaining macOS findings and issue #42 fixed (laptop)

- Launcher: bug reports in `--help` go to the GitHub issue tracker (#42); `mosh-client` next to the launcher is preferred on every platform (PLAN 9.16); the Perl `at SCRIPT line N.` suffixes are gone (PLAN 9.17). `tools/bench` compiles for `aarch64-apple-darwin` (`cargo check --workspace --target aarch64-apple-darwin` clean). Tests: shared `mosh_tests::udp` probe (`ss`, else `lsof`), `scrollback.sh` without `ss`/`flock`, oracle Makefiles reading `CXX`, `CRYPTO_CFLAGS`, `CRYPTO_LIBS`, `TINFO_LIBS` from `upstream/Makefile`.
- Evidence (devbox test runs and CI) in INTERVENTIONS.md, entry of the same date.
## 2026-10-14 CI flakiness fixed in the tests (laptop)

- Root causes and fixes in INTERVENTIONS.md (same date). Devbox: server_e2e, client_e2e, prediction_e2e, udp_impair 10 of 10 each with two binaries running at once; `tests/run-all.sh` twice, 21 items, 0 failed each; CI green on both architectures.

## 2026-10-14 packaging: crates.io, Homebrew tap, Scoop bucket, winget submission

- `mosh` 0.1.2 is live on crates.io (published by `publish.yml` at 02:13 UTC); README now says `cargo install --locked mosh`.
- Homebrew tap `Arunjay4213/homebrew-mosh-rs` with `Formula/mosh-rs.rb` (macOS arm64, Linux x86_64 and arm64, from the v0.1.2 release archives; conflicts with the `mosh` formula). `ruby -c` passes; not yet installed through brew on a real machine.
- Scoop bucket `Arunjay4213/scoop-mosh-rs` with `bucket/mosh-rs.json` (x86_64 zip, `extract_dir`, autoupdate via the `.sha256` file). JSON validated; not yet installed through scoop on a real machine.
- winget manifest submitted as microsoft/winget-pkgs#452393 (Arunjay4213.mosh-rs 0.1.2, zip with two portable executables). Local `winget validate` was not run (no Windows shell here); the pipeline validates it.
- README: the Windows install block downloaded the v0.1.1 zip under a v0.1.2 folder name (fixed), and the other v0.1.1 release references were updated to v0.1.2.

## 2026-10-14 README: "Verification at a glance" (laptop)

- A short section before Install with the Phase 1, 2 and 4 gate numbers that matter to someone deciding whether to run it: crypto conformance (2000 packets, 14 malformed keys, RFC 7253), interop 120 of 120, upstream `make check` 30 PASS 2 XFAIL, fuzzing totals, forbid(unsafe_code) per crate, the hostile-server bound, and a plain statement that there has been no independent audit. Numbers copied from the gate entries above; no new runs.

## 2026-10-14 Homebrew tap verified on a real Mac (laptop)

- Fresh mac2.metal instance (macOS 15.8, Apple M1, Homebrew 6.0.22) on the still-allocated dedicated host h-0b448e31dc2a08aa9. `brew install Arunjay4213/mosh-rs/mosh-rs`: tapped, fetched the aarch64-apple-darwin v0.1.2 archive, installed 8 files, 2.1 MB in 12 s. `brew test mosh-rs` passed. `mosh --version` prints `mosh-rs 0.1.2 [build 0.1.2]`; `mosh-server --version` prints `mosh-server (mosh-rs 0.1.2)`.
- `brew audit --strict --online` found 3 problems in the first formula (`url`/`sha256` directly under `on_macos`, redundant `version`); fixed in tap commit 9490b51, after which the audit exits 0 with 0 problems.
- Real sessions from the Mac to devbox in a 100x30 tmux pane, both exit 0: the brew `mosh` against devbox's `/usr/bin/mosh-server` (mosh 1.4.0), and `mosh --scrollback --server=~/mosh-rs/target/release/mosh-server` running `seq 1 40`, after which tmux's history holds all 40 lines.
- Harness note: a session started under macOS `script` with no terminal behind it (0x0 window) panics with `assertion failed: s_height > 0` in `terminalframebuffer.rs:1272`; upstream's `Framebuffer` asserts the same (`terminalframebuffer.cc:79`), so the port matches and the harness was wrong.
- The instance was terminated and the four temporary security-group rules (ssh to the Mac from the VPC and from the laptop; ssh and UDP 60000-61000 to devbox from the VPC) were removed. The dedicated host is still allocated.
