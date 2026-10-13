# INTERVENTIONS

Every time the run needed outside help, was nudged after going idle, or deviated from PLAN.md.
Newest entry first.

## 2026-10-13 Phase 1 review fixes: network (agent A)

- Review F3: the socket system calls (`new_socket`, `resolve`, `bind`, `recv_raw`, `strerror`, the TOS/ECN read) moved from `crates/network/src/transport/network.rs` to `crates/platform/src/socket.rs`, and `disable_dumping_core`/`reenable_dumping_core` to `crates/platform/src/coredump.rs`. `crates/network` now depends on `mosh-platform`, no longer on `libc`, and carries `#![forbid(unsafe_code)]`. PLAN.md section 2 updated to say sockets live in the platform crate. Two signatures changed to fit the new boundary: `bind` takes the one socket instead of the socket deque (on Windows it replaces the socket with a newly bound one), and `recv_raw` takes the maximum datagram length; errors come back as `mosh_platform::socket::SysError`, which converts to `NetworkError`.
- PLAN.md section 2 also now lists `reset_input` in the `State` trait (added at the coordinator's request; upstream calls it in `set_current_state`).

## 2026-10-13 CI push blocked and upstream version corrected

- NEEDS THE USER. The CI fix (local branch `ci-upstream-build`, commit a3f29e9) cannot be pushed: GitHub rejects workflow changes from the `github-moshrs` deploy key ("refusing to allow an OAuth App to create or update workflow .github/workflows/ci.yml without workflow scope"). No `gh` CLI or token exists on devbox. To land it, run from `~/mosh-rs` with credentials that have the `workflow` scope: `git rebase origin/main ci-upstream-build && git push <https-url-with-token> ci-upstream-build:main`, or grant the key workflow permission. CI stays red until then; work continues because the fix is ready and verified locally (both oracle tests pass against a from-scratch upstream build).
- The upstream tree on devbox (`~/src/mosh`) is mosh master at commit decd9b7 (2026-03-22), 39 commits after the `mosh-1.4.0` tag, not the 1.4.0 release. The Phase 1 ports and oracles were therefore built against master, which among other things has OSC 8 hyperlink support that 1.4.0 lacks. Decision by the orchestrator: the port target is upstream commit decd9b7, and compatibility is proven against the apt mosh 1.4.0 binaries in the interop matrix. PLAN.md now says so. Re-targeting to the 1.4.0 tag would mean redoing verified work for an older emulator.

## 2026-10-13 Phase 1 review and PLAN section 9 additions

- An independent review of commit 451b2ce arrived from the user's other machine (`reviews/2026-10-13-phase1-terminal-network.md`, 26 findings). Blocking items F1, F4, F5, F8 and F17 were already fixed in d9c8289. The remaining network items went to agent A, terminal items to agent B, and the missing impaired-transport driver to agent H.
- PLAN.md section 9 gained items 7-10 (clock base offset, errors instead of aborts on malformed authenticated input, two iterator-UB repairs, `compare` omitted from the `State` trait). Reason: the review found these deliberate differences documented only in code comments, and section 9 is the one place deviations may live.
- CI was red on d9c8289 because the GitHub runner has no upstream C++ tree. One agent is making CI build mosh 1.4.0 from source.

## 2026-10-13 Phase 0

- CLAUDE.md said the mingw linker and Windows target were installed. They were not. The Phase 0 agent installed both (`apt-get install gcc-mingw-w64-x86-64`, `rustup target add x86_64-pc-windows-gnu`).
- PLAN section 2 lists `unicode-width` as a dependency. Replaced by a width table generated from glibc `wcwidth` so the emulator matches upstream on every character. Not a semantic deviation from upstream; it is the opposite.
- PLAN section 2 says `tests/` holds cargo integration tests. A bare directory cannot hold them, so `tests/` and `tools/` are workspace packages (`mosh-tests`, `mosh-tools`).
