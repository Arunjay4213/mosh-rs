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
