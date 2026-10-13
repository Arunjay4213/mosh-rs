# STATUS

Plain-English progress log for mosh-rs, written by the orchestrator at each gate.
Newest entry first.

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
