# Contributing to mosh-rs

Thank you for helping.
mosh-rs is a faithful port of mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0, so the bar for a change is simple to state: it must behave exactly like upstream, or be one of the deviations listed in [PLAN.md](PLAN.md) section 9.
Read [PLAN.md](PLAN.md) before writing code; this guide refers to its section numbers.

## Development setup

You need:

- Rust 1.89 or later (`rustup toolchain install stable`). The `aes` crate sets this minimum.
- Linux or macOS. The server and the test drivers are Unix-only; the Windows client is cross-compiled (see [Building for Windows](docs/ARCHITECTURE.md#building-for-windows)).
- `tmux` 3.x. The end-to-end tests use tmux as the host terminal and compare `tmux capture-pane` output.
- mosh 1.4.0 on `PATH` (for example `apt install mosh`), for the interop tests.
- A built upstream mosh tree, symlinked as `upstream/` at the repository root, for the reference oracles in `ref/` and for running upstream's own `make check`.

Build the upstream tree once:

```sh
sudo apt install build-essential autoconf automake pkg-config protobuf-compiler \
  libprotobuf-dev libncurses-dev zlib1g-dev libssl-dev libutempter-dev perl tmux
scripts/build-upstream.sh
```

The script clones mosh, checks out the reference commit decd9b7, builds it with the same configure line devbox uses, and links it as `upstream`.

The default `./configure` uses OpenSSL's AES with mosh's internal OCB implementation, which is what the `ref-crypto` oracle links against.

Then build and test mosh-rs:

```sh
cargo build --release --workspace
cargo test --release --workspace
tests/run-all.sh        # every end-to-end and conformance test, including the oracles
```

`tests/run-all.sh` is the single entry point for the full suite and grows as each phase lands.
Before pushing, also run:

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
```

## Porting rules

These come from [CLAUDE.md](CLAUDE.md) and [PLAN.md](PLAN.md) sections 0, 4, and 11.
They are what keeps a port reviewable against its source.

1. **Port file by file.** Read the upstream file before porting it. Keep its names, its constants, and its order of operations, so a reviewer can read the two side by side.
2. **Do not improve semantics.** If upstream does something odd, do the same odd thing. The only allowed differences are the ones in PLAN.md section 9; anything else that differs from upstream is a bug.
3. **When upstream behavior is unclear, run upstream.** Use the `ref/` harnesses or the real `mosh-client` and `mosh-server` and match what they do. Do not guess.
4. **Keep the code plain.** Plain functions and structs. No trait without at least two real implementations, no single-caller abstractions, no async runtime (one poll loop, as upstream).
5. **Platform code lives in `crates/platform` only.** It is the only crate with `#[cfg(unix)]` or `#[cfg(windows)]`.
6. **License header.** Every source file starts with the GPLv3 header and names the upstream file it was ported from.
7. **Proposing a deviation.** Open an issue first. A deviation is implemented only after it is added to PLAN.md section 9.

## Tests

Tests prove features a user would notice: correct screens, survival under loss and roaming, scrollback integrity, prediction under latency, exit codes, resource use.
They live in `tests/` as cargo integration tests or shell drivers, and `tests/run-all.sh` runs them all.

- A change to `crates/terminal` must stay cell-exact against `ref-term` on the corpus and the fuzzer.
- A change to `crates/network` must stay byte-exact against `ref-crypto` and pass the transport tests through `udp-impair`.
- A change to the client or server must keep the interop matrix green (README, Compatibility).
- A test that cannot fail for a real reason is deleted.

## Commit messages

Plain imperative subject line, under about 72 characters, for example `Port Framebuffer::scroll from terminalframebuffer.cc`.
Add a body when the why is not obvious.
No attribution trailers (`Co-Authored-By`, `Signed-off-by` added by tools, or similar).

## Pull request checklist

- [ ] Ported code names its upstream file and keeps upstream names, constants, and order of operations.
- [ ] No behavior differs from upstream except deviations already listed in PLAN.md section 9.
- [ ] The relevant oracle was run and its command and output are pasted in the PR.
- [ ] `cargo fmt --all --check` and `cargo clippy --all-targets -- -D warnings` pass.
- [ ] `cargo test --release --workspace` and `tests/run-all.sh` pass.
- [ ] Any new dependency is justified in the PR description.
- [ ] CHANGELOG.md has an entry under Unreleased for user-visible changes.

## What we do not accept

- **Unit tests of single functions, mocks, or test-only abstractions in shipped code.** Tests exercise features end to end.
- **New dependencies without a written reason.** The dependency list is in PLAN.md section 2.
- **Deviations from upstream behavior that PLAN.md section 9 does not list,** including well-meant fixes to upstream quirks. Open an issue to propose one.
- **Async runtimes, or new traits with a single implementation.**

## Code of Conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md).
By participating you agree to uphold it.

## License

By contributing, you agree that your contributions are licensed under the [GNU General Public License v3.0 or later](LICENSE), the same license as upstream mosh.
