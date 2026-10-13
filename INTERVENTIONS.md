# INTERVENTIONS

Every time the run needed outside help, was nudged after going idle, or deviated from PLAN.md.
Newest entry first.

## 2026-10-13 NEEDS THE USER: push branch ci/run-all

- NEEDS THE USER: push branch ci/run-all (commit a36d03d, one line in `.github/workflows/ci.yml`). The test job runs `tests/run-all.sh` instead of `cargo build && cargo test`, so shell drivers such as `tests/transport-impaired.sh` run in CI too. `run-all.sh` itself now passes `--no-fail-fast` (on main, 8636f95), so one failing test binary no longer hides the rest. This replaces the "low priority" note in the entry below.
- Docs now describe the port as "ported from mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0" per the CLAUDE.md reference decision (539f9ed). Mentions of 1.4.0 that refer to the release binaries used for interop and benchmarks are unchanged.

## 2026-10-13 CI now builds upstream; launcher fails on the runner

- The user pushed the CI fix (a3f29e9). CI now builds upstream and the oracle tests run there. It is still red because two launcher tests fail on the GitHub runners (first seen on cc19759):
  - `missing_server_is_reported`: the runner has no TERM, and `mosh-client -c` needs it. Test environment problem; the test will set TERM itself.
  - `launches_session_on_localhost_with_command`: a real launcher bug on dual-stack hosts. ssh reached the server over one address family, mosh-server bound that address, and our launcher resolved the name separately and sent the client to the other one. A side effect of PLAN 9.1 (`ssh -G` instead of mosh.pl's proxy). Assigned to agent C.
- NEEDS THE USER (low priority). CI runs `cargo test`, not `tests/run-all.sh`, so the shell drivers (`tests/transport-impaired.sh` and later the interop matrix) never run in CI. Changing `.github/workflows/ci.yml` needs a push with `workflow` scope, which devbox does not have. The orchestrator will batch this with any other workflow change and hand it to the user as a ready branch.

## 2026-10-13 Fable review of terminal and transport, PLAN 9.6 widened

- The line-by-line Fable review (`reviews/phase1-terminal.md`, `reviews/phase1-transport.md`) found 1 high and 6 low items. The five transport items were already fixed or covered by PLAN section 9 items 8 and 9; the orchestrator answered them in the files.
- High item T-2: kitty sets `TERM=xterm-kitty`, which the hand-written TERM table did not list, so it got xterm answers (BCE on) where its terminfo says BCE off. Devbox does not even have the `xterm-kitty` entry installed, so the table's kitty row had no checkable source. Decision: replace the hand-written table with one generated from every terminfo entry on devbox (1829 entries, plus `apt install kitty-terminfo`). PLAN 9.6 now says so. Reason: hand-picked rows and prefix matching keep producing wrong answers for real terminals (the review also found `screen.xterm-256color`), while a generated table matches upstream exactly for every TERM terminfo knows.

## 2026-10-13 Phase 1 review fixes: network (agent A)

- Review F3: the socket system calls (`new_socket`, `resolve`, `bind`, `recv_raw`, `strerror`, the TOS/ECN read) moved from `crates/network/src/transport/network.rs` to `crates/platform/src/socket.rs`, and `disable_dumping_core`/`reenable_dumping_core` to `crates/platform/src/coredump.rs`. `crates/network` now depends on `mosh-platform`, no longer on `libc`, and carries `#![forbid(unsafe_code)]`. PLAN.md section 2 updated to say sockets live in the platform crate. Two signatures changed to fit the new boundary: `bind` takes the one socket instead of the socket deque (on Windows it replaces the socket with a newly bound one), and `recv_raw` takes the maximum datagram length; errors come back as `mosh_platform::socket::SysError`, which converts to `NetworkError`.
- Review F19: removing the whole-state clone in `rationalize_states` and `get_remote_diff` with `split_first_mut` leaves upstream's last step, the front state subtracted from itself (`UserStream::subtract` clears on `this == prefix`; skipping it would make the next `subtract` assert). A `&mut` and `&` to the same state cannot coexist in Rust, so the `State` trait gained `subtract_self` with a default that subtracts a copy (identical to upstream for every implementation). `UserStream` (clear) and `Complete` (nothing) should override it so no copy is made; that is agent B's file. PLAN.md section 2 updated.
- Review F23: `ocb3` 0.2 has no `zeroize` feature (the review assumed one). Enabled `aes`'s `zeroize` feature instead, which wipes the AES round keys inside the OCB context on drop, and `Session` now wipes its key copy in `Drop` using the `zeroize` crate (already in the build through `aes`; added as a direct dependency of `mosh-network` so no `unsafe` is needed). The OCB offset table derived from the key (`ocb3`'s private `ll` values) is not wiped; `ocb3` exposes no way to.
- CI on cc19759 (and its predecessors since a3f29e9) is red only in `tests/tests/launcher.rs` (agent C's test against apt mosh on the runner: `TERM environment variable not set`, then no UDP connection to 127.0.0.1:60500). The network tests pass there (`crypto_conformance` ok). Not a network change; reported to the coordinator, not fixed here.
- The shared tree `/home/ubuntu/mosh-rs` could not `git pull --rebase` because agent B's statesync files were untracked there but already committed on origin; agent A finished the review fixes from a separate worktree on origin/main instead of touching B's files.
- PLAN.md section 2 also now lists `reset_input` in the `State` trait (added at the coordinator's request; upstream calls it in `set_current_state`).

## 2026-10-13 CI push blocked and upstream version corrected

- NEEDS THE USER. The CI fix (local branch `ci-upstream-build`, commit a3f29e9) cannot be pushed: GitHub rejects workflow changes from the `github-moshrs` deploy key ("refusing to allow an OAuth App to create or update workflow .github/workflows/ci.yml without workflow scope"). No `gh` CLI or token exists on devbox. To land it, run from `~/mosh-rs` with credentials that have the `workflow` scope: `git rebase origin/main ci-upstream-build && git push <https-url-with-token> ci-upstream-build:main`, or grant the key workflow permission. CI stays red until then; work continues because the fix is ready and verified locally (both oracle tests pass against a from-scratch upstream build).
- The upstream tree on devbox (`~/src/mosh`) is mosh master at commit decd9b7 (2026-03-22), 39 commits after the `mosh-1.4.0` tag, not the 1.4.0 release. The Phase 1 ports and oracles were therefore built against master, which among other things has OSC 8 hyperlink support that 1.4.0 lacks. Decision by the orchestrator: the port target is upstream commit decd9b7, and compatibility is proven against the apt mosh 1.4.0 binaries in the interop matrix. PLAN.md now says so. Re-targeting to the 1.4.0 tag would mean redoing verified work for an older emulator.
## 2026-10-13 CI

- Failure: CI on d9c8289 could not build `ref/ref-crypto` because the GitHub runner had no built upstream tree at `upstream/`, and also lacked apt mosh and `ssh localhost` for the launcher test.
- Fix: `scripts/build-upstream.sh` builds upstream (cached by `actions/cache`) and links `upstream/`, and the CI test job installs mosh, tmux, sshd and build deps, sets `LANG=C.UTF-8`, and authorizes a key for `ssh localhost`.
- Deviation found: devbox's `~/src/mosh` is not the mosh-1.4.0 tag but master commit decd9b7 (39 commits later, includes OSC 8 hyperlinks), and `ref/ref-term` does not compile against the 1.4.0 tag, so the script pins decd9b7; PLAN.md still says 1.4.0 and needs the orchestrator's decision.

## 2026-10-13 Phase 1 review and PLAN section 9 additions

- An independent review of commit 451b2ce arrived from the user's other machine (`reviews/2026-10-13-phase1-terminal-network.md`, 26 findings). Blocking items F1, F4, F5, F8 and F17 were already fixed in d9c8289. The remaining network items went to agent A, terminal items to agent B, and the missing impaired-transport driver to agent H.
- PLAN.md section 9 gained items 7-10 (clock base offset, errors instead of aborts on malformed authenticated input, two iterator-UB repairs, `compare` omitted from the `State` trait). Reason: the review found these deliberate differences documented only in code comments, and section 9 is the one place deviations may live.
- CI was red on d9c8289 because the GitHub runner has no upstream C++ tree. One agent is making CI build mosh 1.4.0 from source.

## 2026-10-13 Phase 0

- CLAUDE.md said the mingw linker and Windows target were installed. They were not. The Phase 0 agent installed both (`apt-get install gcc-mingw-w64-x86-64`, `rustup target add x86_64-pc-windows-gnu`).
- PLAN section 2 lists `unicode-width` as a dependency. Replaced by a width table generated from glibc `wcwidth` so the emulator matches upstream on every character. Not a semantic deviation from upstream; it is the opposite.
- PLAN section 2 says `tests/` holds cargo integration tests. A bare directory cannot hold them, so `tests/` and `tools/` are workspace packages (`mosh-tests`, `mosh-tools`).
