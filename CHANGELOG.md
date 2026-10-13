# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-13

### Added

- Cargo workspace with the `terminal`, `network`, `platform`, and `mosh` crates, the release profile, and the Windows cross-compile target.
- `mosh-terminal`: the terminal emulator and `Display::new_frame`, ported from mosh master (decd9b7), cell-exact against upstream on the session corpus and differential fuzzing; a TERM capability table generated from the terminfo database (PLAN.md 9.6).
- `mosh-network`: AES-128-OCB3 crypto, protobuf messages, and the SSP transport, byte-exact against upstream; malformed input from a keyed peer is dropped instead of aborting (PLAN.md 9.8).
- `mosh-server`, `mosh-client` with the prediction overlay, and the `mosh` launcher (mosh.pl's options and `--fake-proxy` address discovery), interoperating with mosh 1.4.0 in every client/server pair.
- Scrollback (`mosh --scrollback`, `mosh-server new --scrollback MIB`): history of the session in the local terminal's own scrollback, with full-screen programs forwarded to the local terminal's alternate screen; old peers are unaffected.
- Windows client: `mosh.exe` and `mosh-client.exe` for Windows Terminal, cross-compiled from Linux, defaulting TERM to `xterm-256color` (PLAN.md 9.12).
- Static musl Linux build (`scripts/build-release.sh`) and a release workflow for Linux x86_64 and arm64, macOS arm64 and Windows x86_64.
- Test suite: reference oracles in `ref/`, differential fuzzing (`vt-fuzz`), the interop matrix, upstream's `make check` with mosh-rs binaries, the scrollback oracle, and `tools/bench`.

[Unreleased]: https://github.com/Arunjay4213/mosh-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.0
