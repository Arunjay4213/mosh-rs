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
