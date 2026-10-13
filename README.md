<div align="center">

<h1>mosh-rs</h1>

<p><strong>The mobile shell, rewritten in Rust: wire-compatible with mosh 1.4, with a native Windows client and real scrollback.</strong></p>

[![CI](https://github.com/Arunjay4213/mosh-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Arunjay4213/mosh-rs/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![MSRV 1.89](https://img.shields.io/badge/MSRV-1.89-orange.svg)](https://blog.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/Arunjay4213/mosh-rs?include_prereleases)](https://github.com/Arunjay4213/mosh-rs/releases)

<!--
  Demo GIF slot. The recording is made at v0.1.0, once the client and server work end to end.
  Until then this stays commented out so the README never shows a mock-up.
  <img src="docs/assets/demo.gif" alt="mosh-rs keeping a session alive across a network change, with scrollback" width="720">
-->

[Status](#status) · [Install](#install) · [Usage](#usage) · [Compatibility](#compatibility) · [How it works](#how-it-works) · [Contributing](CONTRIBUTING.md)

</div>

> mosh-rs is an independent project.
> It is not affiliated with MIT, Keith Winstein, or the mosh maintainers.
> It exists because of their work: see [License and attribution](#license-and-attribution).

## What it is

[mosh](https://mosh.org) is a remote terminal that survives what kills ssh: Wi-Fi to cellular handoffs, a laptop that sleeps overnight, and links with high latency or packet loss.
It runs over UDP, keeps the screen in sync instead of streaming bytes, and echoes your keystrokes locally before the server answers.

mosh-rs is a port of mosh 1.4.0 to Rust that speaks the same wire protocol, so that it works with the `mosh-server` already installed on your hosts.
It is being built to add two things upstream mosh never had: a native Windows client, and scrollback that lands in your local terminal's history.
Linux releases will be single static binaries.

**Status: pre-alpha, under active development.**
Nothing below works yet unless the [Status](#status) list says so.
Every number and every compatibility result in this README will be measured, and until it is, the README says so.

## Status

Progress follows the phases in [PLAN.md](PLAN.md) section 10.
A box is checked only when its gate has passed and the result is recorded in [STATUS.md](STATUS.md).

### Phase 0: setup

- [x] Cargo workspace with four crates, release profile, and Windows cross target

### Phase 1: foundations

- [ ] `crates/network`: AES-128-OCB3 crypto, protobuf messages, and SSP transport, differential-tested against upstream
- [ ] `crates/terminal`: emulator and `Display::new_frame`, cell-exact against upstream on fuzz and corpus inputs
- [ ] `crates/platform` and the `mosh` launcher: raw mode, resize, pty, `ssh -G` host resolution
- [ ] `tools/`: `udp-impair` network impairment proxy, session corpus recorder

### Phase 2: state sync, server, client

- [ ] State objects (`UserStream`, `Complete`) round-trip against upstream
- [ ] `mosh-server`: upstream `mosh-client` connects and survives loss and roaming
- [ ] `mosh-client`: connects to upstream `mosh-server`, with the prediction overlay
- [ ] Full interop matrix and upstream `make check` pass with mosh-rs binaries

### Phase 3: Windows

- [ ] `mosh.exe` runs in Windows Terminal against a Linux server and survives sleep and wake

### Phase 4: scrollback

- [ ] Server-side capture and the `ScrolledLines` wire extension
- [ ] Client-side injection into the local terminal's history, with old peers unaffected

### Phase 5: performance and release

- [ ] Benchmarks against C++ mosh 1.4.0 published below
- [ ] v0.1.0 release with static Linux (x86_64, arm64), macOS arm64, and Windows x86_64 binaries

## Why a rewrite

mosh is excellent software, and its protocol design is the reason this project exists.
Its last release, 1.4.0, shipped in October 2022.
As of October 2026 the upstream repository has 170 open issues and 62 open pull requests.
There has never been a native Windows client; Windows users run mosh inside WSL, Cygwin, or a browser extension.
Scrollback has been missing since the start: [mobile-shell/mosh#122](https://github.com/mobile-shell/mosh/issues/122), "mosh prevents the use of scrollback", has been open since April 2012.

A port keeps everything that works (the protocol, the emulator semantics, the prediction behavior) and makes the two missing features possible without forking the protocol.
New features are additive protobuf fields that existing peers ignore, so mixed old and new installs keep working.

## Install

Pre-built binaries for Linux, macOS, and Windows will be attached to the v0.1.0 release.
Until then, build from source on Linux or macOS with Rust 1.89 or later:

```sh
git clone https://github.com/Arunjay4213/mosh-rs
cd mosh-rs
cargo build --release
# binaries: target/release/mosh, target/release/mosh-client, target/release/mosh-server
```

To build the Windows client, see [Building for Windows](docs/ARCHITECTURE.md#building-for-windows).

## Usage

mosh-rs builds the same three programs as mosh, with the same names and flags, so existing scripts keep working.

```sh
mosh user@host                      # connect
mosh host -- tmux attach            # run a command instead of a login shell
mosh --ssh="ssh -p 2222" host       # custom ssh command
mosh -p 60001 host                  # pin the server's UDP port (or a range: 60000:60010)
mosh --predict=always host          # local echo even on fast links
mosh --local                        # run mosh-server locally, without ssh
```

Launcher flags planned for v0.1 ([PLAN.md](PLAN.md) section 8):

| Flag | Meaning |
| --- | --- |
| `--server=COMMAND` | mosh-server command on the remote host (default `mosh-server`) |
| `--ssh=COMMAND` | ssh command used to start the server (default `ssh`) |
| `-p`, `--port=PORT[:PORT2]` | server-side UDP port or range |
| `--predict=adaptive\|always\|never\|experimental` | local echo mode (`-a` is `always`, `-n` is `never`) |
| `--no-init` | do not send the terminal initialization string |
| `--local` | run mosh-server locally without ssh |
| `-4`, `-6` | IPv4 only, IPv6 only |
| `--scrollback=N` | lines of scrollback the server keeps (new in mosh-rs) |

Environment variables, as in upstream:

| Variable | Meaning |
| --- | --- |
| `MOSH_KEY` | session key passed from the launcher to `mosh-client`; removed from the environment once read |
| `MOSH_PREDICTION_DISPLAY` | default prediction mode (`adaptive`, `always`, `never`, `experimental`) |

## Compatibility

Each cell is one run of the interop test ([PLAN.md](PLAN.md) section 3, oracle 4): a scripted session over loopback through the `udp-impair` proxy, with the final screen compared against a direct run of the same script.

| Client / server | Clean | 5% loss | 30% loss, 200 ms, reorder | 2 min blackout | Client roams |
| --- | :---: | :---: | :---: | :---: | :---: |
| mosh-rs client / mosh-rs server | 🚧 | 🚧 | 🚧 | 🚧 | 🚧 |
| mosh-rs client / mosh 1.4 server | 🚧 | 🚧 | 🚧 | 🚧 | 🚧 |
| mosh 1.4 client / mosh-rs server | 🚧 | 🚧 | 🚧 | 🚧 | 🚧 |
| mosh 1.4 client / mosh 1.4 server (baseline) | 🚧 | 🚧 | 🚧 | 🚧 | 🚧 |

✅ verified by the interop test · 🚧 not yet verified

## Performance

Targets from [PLAN.md](PLAN.md) section 5, measured by `tools/bench` against C++ mosh 1.4.0 on the same machine.
No number appears here until it has been measured.

| Metric | Target | mosh-rs | mosh 1.4.0 |
| --- | --- | --- | --- |
| Idle CPU, client and server, 60 s | 0.0% | measured at v0.1.0 | measured at v0.1.0 |
| Resident memory after a 1 hour session | no more than mosh 1.4.0 | measured at v0.1.0 | measured at v0.1.0 |
| Static binary size | under 4 MB | measured at v0.1.0 | measured at v0.1.0 |
| Startup to first frame, loopback | under 20 ms | measured at v0.1.0 | measured at v0.1.0 |
| Keystroke to screen with prediction, 300 ms RTT | one render, no visible delay | measured at v0.1.0 | measured at v0.1.0 |
| `seq 1 100000` over loopback | no slower than mosh 1.4.0 | measured at v0.1.0 | measured at v0.1.0 |

## How it works

A mosh session has four layers, and mosh-rs ports each one from upstream.

1. **Crypto.** Every UDP packet is AES-128 in OCB3 mode with a 16-byte tag, keyed by a secret the server prints once over ssh.
2. **Transport (SSP).** Each side keeps numbered snapshots of a state object and sends only the diff from the snapshot the other side last acknowledged, so a lost packet is replaced by a newer diff instead of being resent, and the server adopts a new client address on the first valid packet from it (roaming).
3. **State objects.** The client sends keystrokes and resizes; the server sends its terminal screen, encoded as the escape sequences that redraw the client's copy.
4. **Frontend.** `mosh` starts `mosh-server` over ssh, `mosh-server` runs your shell in a pseudoterminal, and `mosh-client` draws the mirrored screen plus predicted local echo.

```text
 local machine                                                 remote host
+--------------+     UDP: AES-128-OCB3 + SSP diffs     +--------------+     +-----+     +-------+
| mosh-client  | <-----------------------------------> | mosh-server  | <-> | pty | <-> | shell |
| (prediction) |   keystrokes ->     <- screen diffs   | (emulator)   |     +-----+     +-------+
+--------------+                                       +--------------+
       ^                                                      ^
       +------- mosh launcher: ssh host -- mosh-server new ---+  (once, to exchange the key)
```

The full design is in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Scrollback

Upstream mosh has no scrollback because the client switches the local terminal to the alternate screen and the server discards rows that scroll off.
In mosh-rs, the server will record rows that leave the top of a full-screen scroll region (but not while a full-screen program like vim is on the alternate screen) and send them as a new protobuf field.
The client will write those lines into the local terminal's normal history, so the mouse wheel and your terminal's search work as they do over ssh, and an upstream peer on either end simply sees normal mosh behavior.

## Windows

The Windows client will put the console into VT mode, so keys arrive as the same escape-sequence bytes a Unix terminal produces and output is plain VT bytes, with no key translation layer.
It will start the server with the `ssh.exe` that ships with Windows 10 and later.
`mosh-server` stays Unix-only, as in upstream.

## How correctness is checked

Correctness is proven against upstream's own C++ code, not by unit tests.
Each check is a program that can be run, and the results go into [STATUS.md](STATUS.md).

- **Reference oracles.** Small C++ harnesses linked against the upstream build dump the emulator's screen, `new_frame` output, and encrypted packets; the Rust versions must produce byte-identical output.
- **Differential fuzzing.** Millions of generated escape-sequence inputs run through both emulators; any mismatch is minimized and fixed.
- **Recorded session corpus.** Real sessions (bash, vim, less, htop, tmux, 256-color and truecolor, wide characters and emoji) replayed through both emulators.
- **Interop matrix.** Rust and C++ clients and servers in every combination, under loss, delay, reordering, blackouts, and roaming.
- **Upstream's test suite.** `make check` in upstream mosh, with mosh-rs binaries swapped in.
- **Scrollback oracle.** `seq 1 50000` through mosh-rs inside tmux must appear in history exactly, with no gaps, duplicates, or blank lines, including under 30% loss, a vim excursion, and a resize.
- **Benchmarks.** Side by side with C++ mosh 1.4.0 in the same harness.

## Project layout

| Path | What it holds | Ported from |
| --- | --- | --- |
| `crates/terminal` | VT parser, framebuffer, `Display` and `new_frame`; pure Rust, no I/O | `src/terminal` |
| `crates/network` | OCB3 crypto, protobuf messages, SSP transport | `src/crypto`, `src/network`, `src/protobufs` |
| `crates/platform` | local tty, pseudoterminal, process spawning; the only platform-specific crate | `src/util`, `src/frontend` |
| `crates/mosh` | state sync, client, prediction overlay, server, launcher; builds `mosh`, `mosh-client`, `mosh-server` | `src/statesync`, `src/frontend`, `scripts/mosh.pl` |
| `ref/` | C++ oracle harnesses linked against the upstream build (not shipped) | |
| `tools/` | `udp-impair`, corpus recorder, bench (not shipped) | |
| `tests/` | end-to-end and conformance tests; `tests/run-all.sh` runs them all | |

## Contributing

Contributions are welcome, with one strong rule: behavior matches upstream mosh 1.4.0 exactly, except for the deviations listed in [PLAN.md](PLAN.md) section 9.
Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request, and the [Code of Conduct](CODE_OF_CONDUCT.md) before joining a discussion.

## Security

Please do not report vulnerabilities in public issues.
See [SECURITY.md](SECURITY.md) for private reporting.

## License and attribution

mosh-rs is licensed under the [GNU General Public License v3.0 or later](LICENSE), because it is a port of mosh, which is GPLv3.

mosh was created by **Keith Winstein** and the mosh authors.
The protocol, the emulator design, the prediction overlay, and the test suite that mosh-rs is checked against are their work.
See [mosh.org](https://mosh.org), the upstream repository [mobile-shell/mosh](https://github.com/mobile-shell/mosh), and the paper ["Mosh: An Interactive Remote Shell for Mobile Clients"](https://mosh.org/mosh-paper.pdf) (Winstein and Balakrishnan, USENIX ATC 2012).

mosh-rs is an independent project and is not affiliated with or endorsed by MIT, Keith Winstein, or the mosh maintainers.
Please report mosh-rs bugs here, not upstream.
