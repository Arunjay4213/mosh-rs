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
