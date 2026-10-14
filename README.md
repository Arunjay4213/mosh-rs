<div align="center">

<h1>mosh-rs</h1>

<p><strong>The mobile shell, rewritten in Rust: wire-compatible with mosh 1.4, with a native Windows client and real scrollback.</strong></p>

[![CI](https://github.com/Arunjay4213/mosh-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Arunjay4213/mosh-rs/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![MSRV 1.89](https://img.shields.io/badge/MSRV-1.89-orange.svg)](https://blog.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/Arunjay4213/mosh-rs?include_prereleases)](https://github.com/Arunjay4213/mosh-rs/releases)

<img src="docs/assets/demo.gif" alt="A real mosh-rs session recorded in tmux: the Rust client connects to the Rust server through a UDP proxy, runs uname -a and ls --color, prints seq 1 300 and scrolls back through it in the local terminal history, keeps the session through a 12-second network blackout while the blue &quot;Last contact N seconds ago&quot; bar counts up, catches up, and quits with Ctrl-^ ." width="720">

[Install](#install) · [Known limitations](#known-limitations) · [Usage](#usage) · [Compatibility](#compatibility) · [How it works](#how-it-works) · [How this was built](#how-this-was-built) · [Contributing](CONTRIBUTING.md)

</div>

> mosh-rs is an independent project.
> It is not affiliated with MIT, Keith Winstein, or the mosh maintainers.
> It exists because of their work: see [License and attribution](#license-and-attribution).

## What it is

[mosh](https://mosh.org) is a remote terminal that survives what kills ssh: Wi-Fi to cellular handoffs, a laptop that sleeps overnight, and links with high latency or packet loss.
It runs over UDP, keeps the screen in sync instead of streaming bytes, and echoes your keystrokes locally before the server answers.

mosh-rs is a Rust port of mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0, so that it works with the `mosh-server` already installed on your hosts.
It adds two things upstream mosh never had: a native Windows client, and scrollback that lands in your local terminal's history.
Scrollback is opt-in: run `mosh --scrollback host` against a mosh-rs `mosh-server`; plain `mosh host` behaves like upstream and keeps no history.
The Linux release binaries are single static binaries.

**Status: pre-alpha.**
The current pre-release is v0.1.2: download it from the [Releases page](https://github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.2) and see [Install](#install).
The [Known limitations](#known-limitations) list says what has been tested and what has not.
Every number and every compatibility result in this README was measured, and anything not yet tested is marked as such.

Found a bug or something that behaves differently from mosh?
Please [open an issue](https://github.com/Arunjay4213/mosh-rs/issues).
For security problems, follow [SECURITY.md](SECURITY.md) instead.

## Known limitations

- Pre-alpha: expect rough edges, and report them in the issue tracker.
- The v0.1.0 macOS client cannot connect (an address lookup bug on macOS); use v0.1.1 or later on a Mac.
- Windows: there is no native ARM64 build; the x86_64 build runs on Windows on Arm through emulation.
- Windows: there is no job control, so Ctrl-^ Ctrl-Z does not suspend the client; the Ctrl-Z goes to the remote shell (use `fg` there to resume a stopped program). Sleep and wake, network loss, and idle CPU were tested on Windows 11 (see [Windows](#windows)).
- `mosh-server` is Unix-only, as in upstream.
- No X forwarding, port forwarding, or SOCKS; mosh never had them.

## Why a rewrite

mosh is excellent software, and its protocol design is the reason this project exists.
Its last release, 1.4.0, shipped in October 2022.
As of October 2026 the upstream repository has 170 open issues and 62 open pull requests.
There has never been a native Windows client; Windows users run mosh inside WSL, Cygwin, or a browser extension.
Scrollback has been missing since the start: [mobile-shell/mosh#122](https://github.com/mobile-shell/mosh/issues/122), "mosh prevents the use of scrollback", has been open since April 2012.

A port keeps everything that works (the protocol, the emulator semantics, the prediction behavior) and makes the two missing features possible without forking the protocol.
New features are additive protobuf fields that existing peers ignore, so mixed old and new installs keep working.

## Install

Pre-built binaries are attached to the [v0.1.2 release](https://github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.2).
Each archive has a `.sha256` file next to it for checking the download.
v0.1.2 is a pre-release; the [Known limitations](#known-limitations) list says what has been tested.

mosh-rs works with the `mosh-server` already on your hosts (mosh 1.4), so you can install only the client side.
The `--scrollback` feature needs the mosh-rs `mosh-server` on the remote host: install the same archive there (see [Scrollback](#scrollback)).

### Homebrew (macOS on Apple silicon, Linux)

```sh
brew install Arunjay4213/mosh-rs/mosh-rs
```

The [tap](https://github.com/Arunjay4213/homebrew-mosh-rs) installs the pre-built `mosh`, `mosh-client`, and `mosh-server` from the release archives.
It conflicts with the `mosh` formula because both install the same three programs, so `brew unlink mosh` first if you have it.

### Scoop (Windows)

```powershell
scoop bucket add mosh-rs https://github.com/Arunjay4213/scoop-mosh-rs
scoop install mosh-rs
```

The [bucket](https://github.com/Arunjay4213/scoop-mosh-rs) installs `mosh.exe` and `mosh-client.exe` from the release zip.
A winget manifest is submitted ([microsoft/winget-pkgs#452393](https://github.com/microsoft/winget-pkgs/pull/452393)); `winget install Arunjay4213.mosh-rs` works once it is merged.

### Linux (x86_64 and arm64)

The archives hold static binaries for `mosh`, `mosh-client`, and `mosh-server`.
`uname -m` prints `x86_64` or `aarch64`, which picks the matching archive.

```sh
v=v0.1.2; t=$(uname -m)-unknown-linux-musl
curl -LO https://github.com/Arunjay4213/mosh-rs/releases/download/$v/mosh-rs-$v-$t.tar.gz
curl -LO https://github.com/Arunjay4213/mosh-rs/releases/download/$v/mosh-rs-$v-$t.tar.gz.sha256
sha256sum -c mosh-rs-$v-$t.tar.gz.sha256
tar xzf mosh-rs-$v-$t.tar.gz
sudo install mosh-rs-$v-$t/mosh mosh-rs-$v-$t/mosh-client mosh-rs-$v-$t/mosh-server /usr/local/bin/
mosh --version   # prints "mosh-rs 0.1.2"
```

Without sudo, install into a directory on your `PATH` that you own instead, such as `~/.local/bin`.
If an archive for the wrong CPU is installed, every program fails with "cannot execute binary file: Exec format error".

### macOS (Apple silicon)

Download `mosh-rs-v0.1.2-aarch64-apple-darwin.tar.gz` and install it the same way, with `shasum -a 256 -c` in place of `sha256sum -c`.
The v0.1.0 macOS `mosh` launcher could not connect ("Did not find remote IP address"), because it read the address family at the Linux position of a C socket address.
v0.1.1 fixes this: the v0.1.1 release archive was used for a real session from an Apple silicon Mac to a Linux server ([INTERVENTIONS.md](INTERVENTIONS.md)).
The v0.1.0 `mosh-server` was verified on macOS separately: a Linux client ran a full session against it.

### Windows (x86_64)

There is one Windows build, for x86_64.
It also runs on Windows on Arm through the x64 emulation built into Windows 11; the hand tests below were run that way, on an ARM64 Windows 11 laptop.

In PowerShell, download the zip and its checksum, compare the two hashes, and unzip:

```powershell
$f = "mosh-rs-v0.1.2-x86_64-pc-windows-msvc"
Invoke-WebRequest "https://github.com/Arunjay4213/mosh-rs/releases/download/v0.1.2/$f.zip" -OutFile "$f.zip"
Invoke-WebRequest "https://github.com/Arunjay4213/mosh-rs/releases/download/v0.1.2/$f.zip.sha256" -OutFile "$f.zip.sha256"
(Get-FileHash "$f.zip" -Algorithm SHA256).Hash.ToLower() -eq (Get-Content "$f.zip.sha256").Split(" ")[0]   # must print True
Expand-Archive "$f.zip" -DestinationPath .
cd $f
.\mosh.exe --version   # prints "mosh-rs 0.1.2"
```

The zip unpacks into a folder named `mosh-rs-v0.1.2-x86_64-pc-windows-msvc` that holds `mosh.exe` and `mosh-client.exe`.
Keep the two together in one folder: `mosh.exe` starts the `mosh-client.exe` next to it.
In Windows Terminal, run `.\mosh.exe user@host` from that folder, or add the folder to your `PATH` and run `mosh user@host`.

The launcher uses the OpenSSH client that ships with Windows 10 and later.
There is no Windows server; the remote host needs `mosh-server` from mosh-rs or mosh 1.4.
The escape key is Ctrl-^, which you type as Ctrl+6 in Windows Terminal, so Ctrl+6 then `.` quits a session.
See [Windows](#windows) for what has been tested.

### With cargo

The package is published on crates.io as [`mosh`](https://crates.io/crates/mosh).
Cargo builds it with Rust 1.89 or later and installs `mosh`, `mosh-client`, and `mosh-server` into `~/.cargo/bin`:

```sh
cargo install --locked mosh
```

The crate named `mosh-rs` on crates.io is a different, unrelated project, so do not run `cargo install mosh-rs`.
To build a specific commit instead, use `cargo install --locked --git https://github.com/Arunjay4213/mosh-rs --tag v0.1.2 mosh`.
On Windows the installed `mosh-server.exe` only prints that the server is not supported there, as described under [Windows](#windows).

### From source

Build on Linux or macOS with Rust 1.89 or later (the minimum, `rust-version` in `Cargo.toml`):

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
mosh user@host                      # connect; type Ctrl-^ then . to quit
mosh host -- tmux attach            # run a command instead of a login shell
mosh --ssh="ssh -p 2222" host       # custom ssh command
mosh -p 60001 host                  # pin the server's UDP port (or a range: 60000:60010)
mosh --predict=always host          # local echo even on fast links
mosh --local localhost              # run mosh-server locally, without ssh (a host argument is still required)
mosh --scrollback host              # keep the session's output in your terminal's history (needs a mosh-rs server)
```

The escape key is Ctrl-^, as in upstream: Ctrl-^ then `.` quits, and Ctrl-^ then Ctrl-Z suspends the client on Unix.
In most terminals you type Ctrl-^ as Ctrl+6 or Ctrl+Shift+6.
Set `MOSH_ESCAPE_KEY` to change it.

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

✅ verified by the interop test (`INTEROP_FULL=1 tests/interop.sh`), six screen scenarios per cell (seq, colors, vim, wide characters, resize, typing), 180 runs in all (120 for the first four rows, 60 for the two `--scrollback` rows), on an 8-core arm64 Linux VM at commit 728e468 (results in [STATUS.md](STATUS.md)).
In the loss columns, loss starts once the session is up; session setup with 30% loss from the first packet is checked separately by `tests/interop-setup-loss.sh` (40 of 40 setups for the mosh-rs client with the 1.4 server and for the 1.4 client with the mosh-rs server).
Upstream's own test suite (`make check`, 32 tests, from the decd9b7 tree) also passes with mosh-rs binaries in every client/server combination (mosh-rs server, mosh-rs client, both): 30 pass and the same 2 expected failures as upstream.

## Performance

Targets from [PLAN.md](PLAN.md) section 5, measured by `tools/bench` against C++ mosh 1.4.0 (Ubuntu 24.04 package) on the same machine, an 8-core arm64 Linux VM (Neoverse-V1), mosh-rs commit 7cab26f, with `target/release/bench --hour`.
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
Against an upstream server, `--scrollback` falls back to normal mosh behavior and shows the notice "mosh: Server does not support scrollback; using the alternate screen."
Without `--scrollback`, the session keeps no local history, even with a mosh-rs server.

To use it, install the mosh-rs `mosh-server` on the remote host with the same Linux or macOS steps, and check it with `ssh host mosh-server --version`, which prints `mosh-server (mosh-rs ...)`.
If the mosh-rs server is not the first `mosh-server` on that host's `PATH`, point the launcher at it: `mosh --scrollback --server=/path/to/mosh-server host`.

How this is checked: `tests/scrollback.sh` runs `seq 1 50000` through mosh in tmux with a 200,000-line history and compares the captured history with `seq` line by line, also through 30% packet loss, around vim, less and `clear`, across resizes, and against an upstream server.

## Windows

The Windows client puts the console into VT mode, so keys arrive as the same escape-sequence bytes a Unix terminal produces and output is plain VT bytes, with no key translation layer.
It starts the server with the `ssh.exe` that ships with Windows 10 and later, and assumes `TERM=xterm-256color` when TERM is unset, as it normally is on Windows.
`mosh.exe` and `mosh-client.exe` cross-compile from Linux and need only system DLLs.
`mosh-server` stays Unix-only, as in upstream.

The escape key is Ctrl-^, as in upstream; in Windows Terminal you type it as Ctrl+6.
So Ctrl+6 then `.` quits the session.

Tested by hand in Windows Terminal on an ARM64 Windows 11 laptop (the x86_64 build under emulation) against a Linux server ([INTERVENTIONS.md](INTERVENTIONS.md)): connecting, typing, `ls --color`, vim, UTF-8 output, Ctrl-C, resizing and quitting all worked, and a session survived turning Wi-Fi off and on.
Losing and regaining the network (Wi-Fi off and on) kept the session live, and so did sleeping and waking the machine.
Sleeping and waking the Windows machine was tested on 2026-10-13 with `top` running: after a minute asleep the session was live and `top` kept updating.
Idle CPU of `mosh-client.exe`, measured over 60 seconds while `top` redrew: 0.1% of one core and 14.7 MB of memory.
Ctrl-^ Ctrl-Z does not suspend the client on Windows, which has no job control; the Ctrl-Z is delivered to the remote shell instead.

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

## How this was built

The port was produced by AI coding agents working from the upstream mosh source under a written plan, [PLAN.md](PLAN.md).
Every module was checked against upstream's own code by differential testing, as described in [How correctness is checked](#how-correctness-is-checked), and reviewed line by line against the upstream code it was ported from.
The complete record of the work is in this repository:

- [PLAN.md](PLAN.md): the plan the port follows, including the allowed differences from upstream.
- [STATUS.md](STATUS.md): the status log, with the commands run and their measured results.
- [INTERVENTIONS.md](INTERVENTIONS.md): every time the work needed outside help or departed from the plan.
- [reviews/](reviews/): the code reviews and how each finding was resolved.

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
