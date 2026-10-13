# INTERVENTIONS

Every time the run needed outside help, was nudged after going idle, or deviated from PLAN.md.
Newest entry first.

## 2026-10-13 Phase 0

- CLAUDE.md said the mingw linker and Windows target were installed. They were not. The Phase 0 agent installed both (`apt-get install gcc-mingw-w64-x86-64`, `rustup target add x86_64-pc-windows-gnu`).
- PLAN section 2 lists `unicode-width` as a dependency. Replaced by a width table generated from glibc `wcwidth` so the emulator matches upstream on every character. Not a semantic deviation from upstream; it is the opposite.
- PLAN section 2 says `tests/` holds cargo integration tests. A bare directory cannot hold them, so `tests/` and `tools/` are workspace packages (`mosh-tests`, `mosh-tools`).
