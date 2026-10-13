# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - unreleased

### Fixed

- macOS: `mosh` could not connect to any host ("Could not connect to HOST, last tried :", then "Did not find remote IP address"), because it read the address family at the Linux position of a socket address. It now connects; verified with a real session from an Apple silicon Mac to a Linux server.
- Windows: if the console input could not be set up, `mosh-client` crashed with a Rust panic; it now prints the error and exits 1 (P1).
- Windows: Ctrl+Break no longer leaves the console without echo and line editing; it shuts the session down cleanly and restores the console, and `mosh.exe` waits for the client to finish (P2).
- Internal: a lint exception without a stated reason was removed (P3).
- Linux `mosh-server`: reading the login records for the "detached session" warning now ignores a file that is not in the expected format instead of misreading it; the direct read is documented as a deliberate difference from upstream (P4).
- Windows: a failure to read the console's code page is reported at startup instead of leaving the wrong code page after exit (P5).
- Windows: escape sequences render even if a parent program turned off processed output (P6).
- Windows: an internal wait with nothing to watch no longer spins the CPU (P7).
- Windows: a console read error is reported as `read: <message>` instead of ending the session silently (P8).
- Error messages now read like upstream's (`setlocale: No such file or directory`, without Rust's "(os error 2)") on every platform (P9).
- Internal: the two address decoders agree on IPv6 flow labels and scope ids; now one shared decoder (P10).
- Cygwin: the terminal is put in UTF-8 input mode as on Linux and macOS (P11).
- Internal: undocumented public items in the platform crate are now a build error (P12).
- Windows: IPv6 link-local addresses with a zone (`fe80::1%3`) are accepted, as on unix (P13).

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
[0.1.1]: https://github.com/Arunjay4213/mosh-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.0
