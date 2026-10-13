# Contributing to mosh-rs

Thank you for helping.
mosh-rs is a faithful port of mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0, so the bar for a change is simple to state: it must behave exactly like upstream, or be one of the deviations listed in [PLAN.md](PLAN.md) section 9.
Read [PLAN.md](PLAN.md) before writing code; this guide refers to its section numbers.

## Development setup

You need:

- Rust 1.89 or later (`rustup toolchain install stable`). 1.89 is the minimum (`rust-version` in `Cargo.toml`), set by the `aes` crate.
- Linux.
  The tests are run on Ubuntu 24.04 (x86_64 and arm64, locally and in CI); running them on macOS has not been tried.
  The server and the test drivers are Unix-only; the Windows client is cross-compiled (see [Building for Windows](docs/ARCHITECTURE.md#building-for-windows)).
- `tmux` 3.2 or later.
  The end-to-end tests use tmux as the host terminal and compare `tmux capture-pane` output.
- mosh 1.4.0 on `PATH` (for example `apt install mosh`), for the interop tests, the launcher comparison and the benchmarks.
- `vim`, `less`, `python3`, `flock`, `ss`, and passwordless `ssh localhost` (and `ssh 127.0.0.1`) for the end-to-end tests; `sysstat` for the idle-CPU checks, which print SKIPPED without it.
- `ncurses-term` and `kitty-terminfo` at the versions recorded in `crates/terminal/src/terminfo.rs`: `terminfo_table` refuses to run against a different terminfo database.
- A built upstream mosh tree, symlinked as `upstream/` at the repository root, for the reference oracles in `ref/` and for running upstream's own `make check`.

First get the code:

```sh
git clone https://github.com/Arunjay4213/mosh-rs
cd mosh-rs
```

Then install the packages and build the upstream tree once:

```sh
sudo apt install mosh tmux vim less sysstat ncurses-term kitty-terminfo openssh-server \
  build-essential autoconf automake pkg-config protobuf-compiler libprotobuf-dev \
  libncurses-dev zlib1g-dev libssl-dev locales
scripts/build-upstream.sh
```

This is the package list CI installs.
The script fetches mosh at the reference commit decd9b7 into `~/src/mosh` (or the directory given as its argument), builds it with `./configure --enable-examples=no` (the configuration the oracles and CI use), and links it as `upstream`.
Do not install `libutempter-dev` first: configure would then build upstream with utempter, which the reference build does not have.

That configure uses OpenSSL's AES with mosh's internal OCB implementation, which is what the `ref-crypto` oracle links against.

Then build and test mosh-rs:

```sh
cargo build --release --workspace
cargo test --release --workspace
tests/run-all.sh        # every end-to-end and conformance test, including the oracles
```

`tests/run-all.sh` takes no options: it builds the workspace, runs `cargo test --release --workspace --no-fail-fast`, then every executable `tests/*.sh` driver, and prints `run-all: all passed` or the list of what failed.
It runs everything serially (about 15 minutes on a CI runner, per `.github/workflows/ci.yml`); [tests/README.md](tests/README.md) shows how to run the long scrollback tests in parts.
Before pushing, also run:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo +1.89.0 clippy --workspace --all-targets -- -D warnings   # the minimum supported Rust
```

## Porting rules

These come from [PLAN.md](PLAN.md) sections 0, 4, and 11.
They are what keeps a port reviewable against its source.

1. **Port file by file.** Read the upstream file before porting it. Keep its names, its constants, and its order of operations, so a reviewer can read the two side by side.
2. **Do not improve semantics.** If upstream does something odd, do the same odd thing. The only allowed differences are the ones in PLAN.md section 9; anything else that differs from upstream is a bug.
3. **When upstream behavior is unclear, run upstream.** Use the `ref/` harnesses or the real `mosh-client` and `mosh-server` and match what they do. Do not guess.
4. **Keep the code plain.** Plain functions and structs. No trait without at least two real implementations, no single-caller abstractions, no async runtime (one poll loop, as upstream).
5. **Platform code lives in `crates/platform` only.** It is the only crate with `#[cfg(unix)]` or `#[cfg(windows)]`, apart from the `#[cfg(unix)]` that builds `mosh-server` only on Unix.
6. **License header.** Every source file starts with the GPLv3 header and names the upstream file it was ported from.
7. **Proposing a deviation.** Open an issue first. A deviation is implemented only after it is added to PLAN.md section 9.

## Tests

Tests prove features a user would notice: correct screens, survival under loss and roaming, scrollback integrity, prediction under latency, exit codes, resource use.
They live in `tests/` as cargo integration tests or shell drivers, and `tests/run-all.sh` runs them all.

- A change to `crates/terminal` must stay cell-exact against `ref-term` on the corpus and the fuzzer.
- A change to `crates/network` must stay byte-exact against `ref-crypto` and pass the transport tests through `udp-impair`.
- A change to the client or server must keep the interop matrix green (README, Compatibility).
- A test that cannot fail for a real reason is deleted.

## How changes land

main is protected: nothing is pushed to it directly, and it only moves to a commit whose CI is green.
CI (`.github/workflows/ci.yml`: rustfmt, clippy, and four test shards on x86_64 and arm64) runs on every push to a branch of this repository, not on `pull_request` events.

To contribute from a fork:

1. Fork the repository on GitHub and clone your fork.
2. Create a branch, make your change, and run the checks above.
3. Push the branch to your fork and open a pull request against `main`.
4. Your pull request shows no CI at first, because CI does not run on pull requests.
   A maintainer reviews it and pushes its commits to a branch in this repository, which runs CI.
5. When CI is green, main is fast-forwarded to the tested commit, and your pull request shows as merged because its commits are now on main.

Maintainers push branches named `agent/<topic>` or `laptop/<topic>` to this repository directly.
Such a push opens a pull request by itself (`.github/workflows/pr-open.yml`), and when CI is green `.github/workflows/auto-merge.yml` fast-forwards main to the tested commit.
If main moved while CI ran, the pull request gets the comment "Rebase onto main and push again".

## Commit messages

Plain imperative subject line, under about 72 characters, for example `Port Framebuffer::scroll from terminalframebuffer.cc`.
Add a body when the why is not obvious.
No attribution trailers (`Co-Authored-By`, `Signed-off-by` added by tools, or similar).

## Pull request checklist

- [ ] Ported code names its upstream file and keeps upstream names, constants, and order of operations.
- [ ] No behavior differs from upstream except deviations already listed in PLAN.md section 9.
- [ ] The relevant oracle was run and its command and output are pasted in the PR.
- [ ] `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass, on stable and on 1.89.
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
