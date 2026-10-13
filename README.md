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

mosh-rs is a Rust port of mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0, so that it works with the `mosh-server` already installed on your hosts.
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

- [x] `crates/network`: AES-128-OCB3 crypto, protobuf messages, and SSP transport, differential-tested against upstream
- [x] `crates/terminal`: emulator and `Display::new_frame`, cell-exact against upstream on fuzz and corpus inputs
- [x] `crates/platform` and the `mosh` launcher: raw mode, resize, pty, `--fake-proxy` address discovery as in mosh.pl
- [x] `tools/`: `udp-impair` network impairment proxy, session corpus recorder

### Phase 2: state sync, server, client

- [x] State objects (`UserStream`, `Complete`) round-trip against upstream
- [x] `mosh-server`: upstream `mosh-client` connects and survives loss and roaming
- [x] `mosh-client`: connects to upstream `mosh-server`, with the prediction overlay
- [x] Full interop matrix and upstream `make check` pass with mosh-rs binaries

### Phase 3: Windows

- [ ] `mosh.exe` runs in Windows Terminal against a Linux server and survives sleep and wake

### Phase 4: scrollback

- [x] Server-side capture and the `ScrolledLines` wire extension
- [x] Client-side injection into the local terminal's history, with old peers unaffected

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
Until then, build from source on Linux with Rust 1.89 or later (the minimum, `rust-version` in `Cargo.toml`).
The macOS target (aarch64-apple-darwin) passes `cargo check` but has not been built or run on a Mac yet.

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
mosh --local localhost              # run mosh-server locally, without ssh (a host argument is still required)
mosh --scrollback host              # keep the session's output in your terminal's history (needs a mosh-rs server)
```

The most used launcher flags ([PLAN.md](PLAN.md) section 8); `mosh --help` lists all of mosh.pl's options:

| Flag | Meaning |
| --- | --- |
| `--server=COMMAND` | mosh-server command on the remote host (default `mosh-server`) |
| `--ssh=COMMAND` | ssh command used to start the server (default `ssh`) |
| `-p`, `--port=PORT[:PORT2]` | server-side UDP port or range |
| `--predict=adaptive\|always\|never\|experimental` | local echo mode (`-a` is `always`, `-n` is `never`) |
| `--no-init` | do not send the terminal initialization string |
| `--local` | run mosh-server locally without ssh |
| `-4`, `-6` | IPv4 only, IPv6 only |
| `--scrollback` | keep the session's output in the local terminal's history; needs a mosh-rs server, falls back to upstream behavior with a notice otherwise (new in mosh-rs) |

The server takes `mosh-server new --scrollback MIB` to cap how much undelivered history it holds (default 32 MiB, `0` turns capture off).

Environment variables, as in upstream:

| Variable | Meaning |
| --- | --- |
| `MOSH_KEY` | session key passed from the launcher to `mosh-client`; removed from the environment once read |
| `MOSH_PREDICTION_DISPLAY` | default prediction mode (`adaptive`, `always`, `never`, `experimental`) |
| `MOSH_ESCAPE_KEY` | the client's escape key (default Ctrl-^) |

## Compatibility

Each cell is six runs of the interop test ([PLAN.md](PLAN.md) section 3, oracle 4), one per screen scenario: a scripted session over loopback through the `udp-impair` proxy, with the final screen compared against a direct run of the same script.
The mosh 1.4 binaries are the Ubuntu 24.04 apt package.

| Client / server | Clean | 5% loss | 30% loss, 200 ms, reorder | 2 min blackout | Client roams |
| --- | :---: | :---: | :---: | :---: | :---: |
| mosh-rs client / mosh-rs server | ✅ | ✅ | ✅ | ✅ | ✅ |
| mosh-rs client / mosh 1.4 server | ✅ | ✅ | ✅ | ✅ | ✅ |
| mosh 1.4 client / mosh-rs server | ✅ | ✅ | ✅ | ✅ | ✅ |
| mosh 1.4 client / mosh 1.4 server (baseline) | ✅ | ✅ | ✅ | ✅ | ✅ |
| mosh-rs client with `--scrollback` / mosh-rs server | ✅ | ✅ | ✅ | ✅ | ✅ |
| mosh-rs client with `--scrollback` / mosh 1.4 server | ✅ | ✅ | ✅ | ✅ | ✅ |

✅ verified by the interop test (`INTEROP_FULL=1 tests/interop.sh`), six screen scenarios per cell (seq, colors, vim, wide characters, resize, typing), 180 runs in all (120 for the first four rows, 60 for the two `--scrollback` rows), on devbox (arm64 Linux) at commit 728e468 ([STATUS.md](STATUS.md), "Phase 4: every devbox check passes").
In the loss columns, loss starts once the session is up; session setup with 30% loss from the first packet is checked separately by `tests/interop-setup-loss.sh` (40 of 40 setups for the mosh-rs client with the 1.4 server and for the 1.4 client with the mosh-rs server).
Upstream's own test suite (`make check`, 32 tests, from the decd9b7 tree) also passes with mosh-rs binaries in every client/server combination (mosh-rs server, mosh-rs client, both): 30 pass and the same 2 expected failures as upstream.

## Performance

Targets from [PLAN.md](PLAN.md) section 5, measured by `tools/bench` against C++ mosh 1.4.0 (Ubuntu 24.04 package) on the same machine: devbox, an 8-core arm64 Linux VM (Neoverse-V1), mosh-rs commit 7cab26f, with `target/release/bench --hour`.
Both implementations run the same scripted workload, alternating run by run; [tools/bench/README.md](tools/bench/README.md) describes it.

| Metric | Target | mosh-rs | mosh 1.4.0 |
| --- | --- | --- | --- |
| Idle CPU over 60 s, client and server | 0.0% | 0.00%, 0.00% | 0.00%, 0.00% |
| Resident memory after a 1 hour session, client | no more than mosh 1.4.0 | 4.1 MiB | 14.3 MiB |
| Resident memory after a 1 hour session, server | no more than mosh 1.4.0 | 2.5 MiB | 9.9 MiB |
| Static binary size (aarch64 musl): mosh, mosh-client, mosh-server | under 4 MB | 0.8 MB, 1.2 MB, 1.1 MB | not measured (Perl script and dynamically linked binaries) |
| Client startup to its first terminal write | under 20 ms | 1.40 ms | 2.94 ms |
| Startup to the first frame, loopback | no slower than mosh 1.4.0 | 252.2 ms | 256.6 ms |
| Keystroke to screen with prediction, 300 ms RTT | one render, no visible delay | 0.09 ms, one render | 0.08 ms, one render |
| `seq 1 100000` until done on screen, loopback | no slower than mosh 1.4.0 | 110 ms | 130 ms |

Notes:

- The first frame takes about 250 ms in both implementations because upstream's round-trip estimate starts at 1 second, which sets the first send interval to its 250 ms maximum; mosh-rs keeps upstream's timing.
- Part of the memory difference is shared-library pages: the C++ binaries load several shared libraries, the static mosh-rs binaries load none.
- All numbers are medians where several runs were taken; the full output, including p90 values and per-run times, is printed by `tools/bench`.

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
With `mosh --scrollback` and a mosh-rs server, the server records every row that leaves the top of the screen, and the rows on screen when a program clears it, and sends them as a new protobuf field that upstream peers ignore.
The client writes those lines into your terminal's normal history, so the mouse wheel and your terminal's search work as they do over ssh.
Full-screen programs like vim and less run in your terminal's own alternate screen, so nothing they draw enters history and the screen from before them comes back when they exit.

The screen is always current; history follows a few seconds behind on a large burst (the history of `seq 1 50000` arrives in about 9 seconds on loopback), because the server sends at most a few packets of history per round trip so that it survives packet loss.
The remote program is never paused for history: if more than 32 MiB of history is waiting (for example while your laptop sleeps during a long build), the oldest undelivered lines are dropped and a dim line in your history says how many.
Known limit: changing the window height can lose or repeat a few rows, because terminals keep the bottom of the screen on resize while mosh's emulator keeps the top.
Against an upstream server, `--scrollback` falls back to normal mosh behavior and shows a notice.

How this is checked: `tests/scrollback.sh` runs `seq 1 50000` through mosh in tmux with a 200,000-line history and compares the captured history with `seq` line by line, also through 30% packet loss, around vim, less and `clear`, across resizes, and against an upstream server.

## Windows

The Windows client puts the console into VT mode, so keys arrive as the same escape-sequence bytes a Unix terminal produces and output is plain VT bytes, with no key translation layer.
It starts the server with the `ssh.exe` that ships with Windows 10 and later, and assumes `TERM=xterm-256color` when TERM is unset, as it normally is on Windows.
`mosh.exe` and `mosh-client.exe` cross-compile from Linux and need only system DLLs.
`mosh-server` stays Unix-only, as in upstream.

The escape key is Ctrl-^, as in upstream; in Windows Terminal you type it as Ctrl+6.
So Ctrl+6 then `.` quits the session.

Tested by hand on a Windows laptop in Windows Terminal against a Linux server ([INTERVENTIONS.md](INTERVENTIONS.md)): connecting, typing, `ls --color`, vim, UTF-8 output, Ctrl-C, resizing and quitting all worked, and a session survived turning Wi-Fi off and on.
Not tested yet: the laptop sleeping and waking during a session, Ctrl-^ Ctrl-Z, and idle CPU on Windows.

## How correctness is checked

Correctness is proven against upstream's own C++ code, not by unit tests.
Each check is a program that can be run, and the results go into [STATUS.md](STATUS.md).

- **Reference oracles.** Small C++ harnesses linked against the upstream build dump the emulator's screen, `new_frame` output, and encrypted packets; the Rust versions must produce byte-identical output.
- **Differential fuzzing.** Generated escape-sequence inputs (one run of 1,000,000 and several of 200,000, plus random screen pairs for `new_frame`) run through both emulators; any mismatch is minimized and fixed.
- **Recorded session corpus.** Ten recorded sessions (bash, vim, less, htop, tmux, the Claude Code TUI, `seq`, a resize, 256-color and truecolor, wide characters and emoji) and 17 inputs from upstream's own emulation tests, replayed through both emulators.
- **Interop matrix.** Rust and C++ clients and servers in every combination, under loss, delay, reordering, blackouts, and roaming.
- **Upstream's test suite.** `make check` in upstream mosh, with mosh-rs binaries swapped in.
- **End-to-end and comparison tests.** The real binaries, mostly driven in tmux, with most cases also run with the C++ program for comparison: client, server, prediction timing at 300 ms round trip, the launcher's options and messages compared with `/usr/bin/mosh` (73 cases), terminfo answers for every terminal type, and a hostile scrollback server.
- **Scrollback oracle.** `seq 1 50000` through mosh-rs inside tmux must appear in history exactly, with no gaps, duplicates, or blank lines, including under 30% loss, a vim excursion, `clear`, and a width resize (height resizes have the known limit above).
- **Benchmarks.** Side by side with C++ mosh 1.4.0 in the same harness.

## Project layout

| Path | What it holds | Ported from |
| --- | --- | --- |
| `crates/terminal` | VT parser, framebuffer, `Display` and `new_frame`; pure Rust, no I/O | `src/terminal` |
| `crates/network` | OCB3 crypto, protobuf messages, SSP transport | `src/crypto`, `src/network`, `src/protobufs` |
| `crates/platform` | local tty, pseudoterminal, process spawning, UDP sockets, the select loop; the only platform-specific crate | `src/util`, `src/frontend`, `src/network` (sockets) |
| `crates/mosh` | state sync, client, prediction overlay, server, launcher; builds `mosh`, `mosh-client`, `mosh-server` | `src/statesync`, `src/frontend`, `scripts/mosh.pl` |
| `ref/` | C++ oracle harnesses linked against the upstream build (not shipped) | |
| `tools/` | `udp-impair`, corpus recorder, bench (not shipped) | |
| `tests/` | end-to-end and conformance tests; `tests/run-all.sh` runs them all | |
| `scripts/` | upstream build for the oracles, static release build, CI helpers | |

## Contributing

Contributions are welcome, with one strong rule: behavior matches upstream mosh master (decd9b7, March 2026) exactly, except for the deviations listed in [PLAN.md](PLAN.md) section 9.
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
