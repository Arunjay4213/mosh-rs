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
> It is built on their work (see [License and attribution](#license-and-attribution)).

## What it is

[mosh](https://mosh.org) is a remote terminal that keeps a session alive through things that drop an ssh connection, such as a switch from Wi-Fi to cellular, a laptop that sleeps overnight, or a link with high latency or packet loss.
It runs over UDP and keeps the client's copy of the screen in sync with the server's instead of streaming bytes.
It also echoes your keystrokes locally before the server answers.

mosh-rs is a Rust port of mosh master (decd9b7, March 2026).
It is wire-compatible with mosh 1.4.0, so it works with the `mosh-server` already installed on your hosts.
It also adds two things upstream mosh never had, a native Windows client and scrollback that lands in your local terminal's history.
Scrollback is opt-in.
You get it by running `mosh --scrollback host` against a mosh-rs `mosh-server`, while plain `mosh host` behaves like upstream and keeps no history.
On Linux, each release program is a single static binary.

The project is pre-alpha.
The current pre-release is v0.1.3, which you can download from the [Releases page](https://github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.3) (see [Install](#install)).
[Known limitations](#known-limitations) lists what has and has not been tested.
The numbers and compatibility results in this README all come from real measurements, and anything not yet tested is marked.

Found a bug or something that behaves differently from mosh?
Please [open an issue](https://github.com/Arunjay4213/mosh-rs/issues).
For security problems, follow [SECURITY.md](SECURITY.md) instead.

## Known limitations

- Pre-alpha: expect rough edges, and report them in the issue tracker.
- The v0.1.0 macOS client cannot connect (an address lookup bug on macOS); use v0.1.1 or later on a Mac.
- Windows: there is no native ARM64 build; the x86_64 build runs on Windows on Arm through emulation.
- Windows: there is no job control, so Ctrl-^ Ctrl-Z does not suspend the client; the Ctrl-Z goes to the remote shell (use `fg` there to resume a stopped program).
  Sleep and wake, network loss, and idle CPU were tested on Windows 11 (see [Windows](#windows)).
- `mosh-server` is Unix-only, as in upstream.
- No X forwarding, port forwarding, or SOCKS; mosh never had them.

## Why a rewrite

mosh is excellent software, and this project exists because of its protocol design.
But its last release, 1.4.0, shipped in October 2022, and as of October 2026 the upstream repository has 170 open issues and 62 open pull requests.
There has never been a native Windows client, so Windows users run mosh inside WSL, Cygwin, or a browser extension.
Scrollback has been missing from the start: [mobile-shell/mosh#122](https://github.com/mobile-shell/mosh/issues/122), "mosh prevents the use of scrollback", has been open since April 2012.

A port can keep the parts that work (the protocol, the emulator semantics and the prediction behavior) and still add the two missing features without forking the protocol.
New features go in additive protobuf fields that existing peers ignore, which is why mixed old and new installs keep working.

## Verification at a glance

mosh-rs is checked against upstream's own C++ code.
The commands are in `tests/` and the numbers in [STATUS.md](STATUS.md).
These are the main results at v0.1.3, unchanged since v0.1.2.

- Every packet is encrypted with AES-128-OCB3 from the RustCrypto [`aes`](https://crates.io/crates/aes) and [`ocb3`](https://crates.io/crates/ocb3) crates, and the key schedule is wiped on drop as in upstream.
  `tests/tests/crypto_conformance.rs` encrypts and decrypts 2000 random packets with mosh-rs and with upstream's C++ linked into a harness.
  The output is byte-identical in both directions, both reject tampered packets and the same 14 malformed keys, and the RFC 7253 test vectors pass.
- In the interop test, C++ and Rust clients run against C++ and Rust servers under clean, 5% loss, 30% loss with 200 ms delay and reordering, a 2-minute blackout, and a client address change, with six screen scenarios each; 120 of 120 cells pass.
- mosh's own `make check`, run with the mosh-rs client, the mosh-rs server, and both swapped in, gives 30 PASS, 2 XFAIL, 0 FAIL.
- Differential fuzzing against upstream's terminal emulator found 0 mismatches over 1,000,000 cases plus four runs of 200,000 and 200,000 frame cases.
  27 recorded sessions are cell-exact at two sizes, and terminfo answers for 2830 terminal names are identical.
- The terminal, network, and mosh crates use `#![forbid(unsafe_code)]`.
  `unsafe` appears only in the platform crate, and each block has a stated invariant.
  A hostile server that sends a 1,000,000-line-feed history line is rejected while the client peaks at 6.2 MB of memory.

There has been no independent security audit.
The crypto and transport code is a line-by-line port of upstream's, backed by the conformance tests above (the full list is in [How correctness is checked](#how-correctness-is-checked)).

## Install

Pre-built binaries are attached to the [v0.1.3 release](https://github.com/Arunjay4213/mosh-rs/releases/tag/v0.1.3), each archive with a `.sha256` file next to it for checking the download.
v0.1.3 is a pre-release, and [Known limitations](#known-limitations) lists what has been tested.

Since mosh-rs works with the `mosh-server` already on your hosts (mosh 1.4), you can install only the client side.
The `--scrollback` feature is the exception: it needs the mosh-rs `mosh-server` on the remote host, so install the same archive there too (see [Scrollback](#scrollback)).

### Homebrew (macOS on Apple silicon, Linux)

```sh
brew install Arunjay4213/mosh-rs/mosh-rs
```

The [tap](https://github.com/Arunjay4213/homebrew-mosh-rs) installs the pre-built `mosh`, `mosh-client`, and `mosh-server` from the release archives.
Both it and the `mosh` formula install the same three programs, so they conflict; if you have the formula installed, run `brew unlink mosh` first.

### Scoop (Windows)

```powershell
scoop bucket add mosh-rs https://github.com/Arunjay4213/scoop-mosh-rs
scoop install mosh-rs
```

The [bucket](https://github.com/Arunjay4213/scoop-mosh-rs) installs `mosh.exe` and `mosh-client.exe` from the release zip.
A winget manifest has been submitted ([microsoft/winget-pkgs#452393](https://github.com/microsoft/winget-pkgs/pull/452393)), and once it is merged `winget install Arunjay4213.mosh-rs` will work too.

### Linux (x86_64 and arm64)

The archives hold static binaries for `mosh`, `mosh-client`, and `mosh-server`.
The script below picks the archive from `uname -m`, which prints `x86_64` or `aarch64`.

```sh
v=v0.1.3; t=$(uname -m)-unknown-linux-musl
curl -LO https://github.com/Arunjay4213/mosh-rs/releases/download/$v/mosh-rs-$v-$t.tar.gz
curl -LO https://github.com/Arunjay4213/mosh-rs/releases/download/$v/mosh-rs-$v-$t.tar.gz.sha256
sha256sum -c mosh-rs-$v-$t.tar.gz.sha256
tar xzf mosh-rs-$v-$t.tar.gz
sudo install mosh-rs-$v-$t/mosh mosh-rs-$v-$t/mosh-client mosh-rs-$v-$t/mosh-server /usr/local/bin/
mosh --version   # prints "mosh-rs 0.1.3"
```

If you don't have sudo, install into a directory on your `PATH` that you own, such as `~/.local/bin`.
An archive built for the wrong CPU fails on every program with "cannot execute binary file: Exec format error".

### macOS (Apple silicon)

Download `mosh-rs-v0.1.3-aarch64-apple-darwin.tar.gz` and install it the same way, with `shasum -a 256 -c` in place of `sha256sum -c`.
The v0.1.0 macOS `mosh` launcher could not connect ("Did not find remote IP address"), because it read the address family at the Linux position of a C socket address.
v0.1.1 fixes this, and the v0.1.1 release archive was used for a real session from an Apple silicon Mac to a Linux server ([INTERVENTIONS.md](INTERVENTIONS.md)).
The v0.1.0 `mosh-server` was verified on macOS separately, with a Linux client running a full session against it.

### Windows (x86_64)

The only Windows build is for x86_64.
It also runs on Windows on Arm through the x64 emulation built into Windows 11, and that is how the hand tests below were run, on an ARM64 Windows 11 laptop.

In PowerShell, download the zip and its checksum, compare the hashes, and unzip:

```powershell
$f = "mosh-rs-v0.1.3-x86_64-pc-windows-msvc"
Invoke-WebRequest "https://github.com/Arunjay4213/mosh-rs/releases/download/v0.1.3/$f.zip" -OutFile "$f.zip"
Invoke-WebRequest "https://github.com/Arunjay4213/mosh-rs/releases/download/v0.1.3/$f.zip.sha256" -OutFile "$f.zip.sha256"
(Get-FileHash "$f.zip" -Algorithm SHA256).Hash.ToLower() -eq (Get-Content "$f.zip.sha256").Split(" ")[0]   # must print True
Expand-Archive "$f.zip" -DestinationPath .
cd $f
.\mosh.exe --version   # prints "mosh-rs 0.1.3"
```

The zip unpacks into a folder named `mosh-rs-v0.1.3-x86_64-pc-windows-msvc` that holds `mosh.exe` and `mosh-client.exe`.
Keep the two in the same folder, because `mosh.exe` starts the `mosh-client.exe` next to it.
In Windows Terminal, run `.\mosh.exe user@host` from that folder, or add the folder to your `PATH` and run `mosh user@host`.

The launcher uses the OpenSSH client that ships with Windows 10 and later.
There is no Windows server, so the remote host needs `mosh-server` from mosh-rs or mosh 1.4.
The escape key is Ctrl-^, which you type as Ctrl+6 in Windows Terminal (Ctrl+6 then `.` quits a session).
[Windows](#windows) describes what has been tested.

### With cargo

The package is on crates.io as [`mosh`](https://crates.io/crates/mosh).
With Rust 1.89 or later, this builds it and installs `mosh`, `mosh-client`, and `mosh-server` into `~/.cargo/bin`:

```sh
cargo install --locked mosh
```

Do not run `cargo install mosh-rs`: the crate named `mosh-rs` on crates.io is an unrelated project.
To build a specific commit, use `cargo install --locked --git https://github.com/Arunjay4213/mosh-rs --tag v0.1.3 mosh`.
On Windows the installed `mosh-server.exe` only prints that the server is not supported there (see [Windows](#windows)).

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

mosh-rs builds the same three programs as mosh with the same names and flags, so existing scripts still work.

```sh
mosh user@host                      # connect; type Ctrl-^ then . to quit
mosh host -- tmux attach            # run a command instead of a login shell
mosh --ssh="ssh -p 2222" host       # custom ssh command
mosh -p 60001 host                  # pin the server's UDP port (or a range: 60000:60010)
mosh --predict=always host          # local echo even on fast links
mosh --local localhost              # run mosh-server locally, without ssh (a host argument is still required)
mosh --scrollback host              # keep the session's output in your terminal's history (needs a mosh-rs server)
```

As in upstream, the escape key is Ctrl-^.
Ctrl-^ then `.` quits, and on Unix Ctrl-^ then Ctrl-Z suspends the client.
Most terminals produce Ctrl-^ when you press Ctrl+6 or Ctrl+Shift+6, and `MOSH_ESCAPE_KEY` changes it.

These are the most used launcher flags ([PLAN.md](PLAN.md) section 8), and `mosh --help` lists all of mosh.pl's options:

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

On the server, `mosh-server new --scrollback MIB` caps how much undelivered history it holds (default 32 MiB, and `0` turns capture off).

### ssh ControlMaster

In the default mode the launcher learns the server's address through a fake `ProxyCommand`.
That needs a fresh ssh connection, so it runs ssh with `-S none`, an open ControlMaster socket goes unused (upstream [mobile-shell/mosh#24](https://github.com/mobile-shell/mosh/issues/24)), and every session authenticates again.
With `--experimental-remote-ip=remote` the launcher asks the server for the address instead (`$SSH_CONNECTION`), ssh runs without `-S none`, and a ControlMaster socket from `~/.ssh/config` is reused, so later sessions open without logging in:

```
Host myhost
    ControlMaster auto
    ControlPersist 10m
    ControlPath ~/.ssh/cm-%C
```

```sh
mosh --experimental-remote-ip=remote myhost
```

This was measured against devbox with a master socket open and `ssh -v -E log`.
The default mode made a full new connection and authenticated, `remote` mode reused the socket (`mux_client_request_session`), and both sessions worked.
One catch is that `remote` reports the address the server sees, which is wrong behind port-forwarding NAT.
`--experimental-remote-ip=local` resolves the name on your side and also runs ssh without `-S none`.
If a master socket is open and you use the default mode, mosh-rs prints a one-line hint naming the option, where upstream prints nothing; `tests/tests/launcher.rs` checks both modes.

Environment variables, as in upstream:

| Variable | Meaning |
| --- | --- |
| `MOSH_KEY` | session key passed from the launcher to `mosh-client`; removed from the environment once read |
| `MOSH_PREDICTION_DISPLAY` | default prediction mode (`adaptive`, `always`, `never`, `experimental`) |
| `MOSH_ESCAPE_KEY` | the client's escape key (default Ctrl-^) |

## Compatibility

Each cell is six runs of the interop test ([PLAN.md](PLAN.md) section 3, oracle 4), one per screen scenario.
A run is a scripted session over loopback through the `udp-impair` proxy, and its final screen is compared against a direct run of the same script.
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

The targets come from [PLAN.md](PLAN.md) section 5.
`tools/bench` measured mosh-rs commit 7cab26f against C++ mosh 1.4.0 (Ubuntu 24.04 package) on the same machine, an 8-core arm64 Linux VM (Neoverse-V1), with `target/release/bench --hour`.
Both implementations run the same scripted workload, taking turns run by run, and [tools/bench/README.md](tools/bench/README.md) describes it.

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
- Part of the memory difference is shared-library pages, since the C++ binaries load several shared libraries and the static mosh-rs binaries load none.
- Where several runs were taken, the number is the median.
  `tools/bench` prints the full output, including p90 values and per-run times.

## How it works

A mosh session has four layers, and mosh-rs ports each one from upstream.

1. Crypto: every UDP packet is AES-128 in OCB3 mode with a 16-byte tag, keyed by a secret the server prints once over ssh.
2. Transport (SSP): each side keeps numbered snapshots of a state object and sends only the diff from the snapshot the other side last acknowledged.
   A lost packet is never resent; a newer diff replaces it.
   The server also switches to a new client address on the first valid packet from it, which is how roaming works.
3. State objects: the client sends keystrokes and resizes, and the server sends its terminal screen, encoded as the escape sequences that redraw the client's copy.
4. Frontend: `mosh` starts `mosh-server` over ssh, `mosh-server` runs your shell in a pseudoterminal, and `mosh-client` draws the mirrored screen plus predicted local echo.

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

Upstream mosh has no scrollback because the client switches the local terminal to the alternate screen and the server throws away rows that scroll off.
With `mosh --scrollback` and a mosh-rs server, the server records every row that leaves the top of the screen, plus the rows on screen when a program clears it.
It sends them in a new protobuf field that upstream peers ignore, and the client writes them into your terminal's normal history, so the mouse wheel and your terminal's search work as they do over ssh.
Full-screen programs like vim and less run in your terminal's own alternate screen.
Nothing they draw enters history, and the screen from before them comes back when they exit.

The screen is always current, but on a large burst history can lag a few seconds behind (the history of `seq 1 50000` arrives in about 9 seconds on loopback).
That is because the server sends at most a few packets of history per round trip, so that history survives packet loss.
The remote program is never paused to wait for history.
If more than 32 MiB of history is waiting (say your laptop sleeps during a long build), the oldest undelivered lines are dropped and a dim line in your history says how many.
There is one known limit: changing the window height can lose or repeat a few rows, because terminals keep the bottom of the screen on resize while mosh's emulator keeps the top.
Against an upstream server, `--scrollback` falls back to normal mosh behavior and shows the notice "mosh: Server does not support scrollback; using the alternate screen."
Without `--scrollback`, the session keeps no local history, even with a mosh-rs server.

To use it, install the mosh-rs `mosh-server` on the remote host with the same Linux or macOS steps.
`ssh host mosh-server --version` should then print `mosh-server (mosh-rs ...)`.
If the mosh-rs server is not the first `mosh-server` on that host's `PATH`, point the launcher at it: `mosh --scrollback --server=/path/to/mosh-server host`.

`tests/scrollback.sh` checks this by running `seq 1 50000` through mosh in tmux with a 200,000-line history and comparing the captured history with `seq` line by line.
It repeats the check through 30% packet loss, around vim, less and `clear`, across resizes, and against an upstream server.

## Windows

The Windows client puts the console into VT mode.
Keys then arrive as the same escape-sequence bytes a Unix terminal produces, and output is plain VT bytes, so there is no key translation layer.
It starts the server with the `ssh.exe` that ships with Windows 10 and later, and assumes `TERM=xterm-256color` when TERM is unset, which it normally is on Windows.
`mosh.exe` and `mosh-client.exe` cross-compile from Linux and need only system DLLs.
`mosh-server` stays Unix-only, as in upstream.

The escape key is Ctrl-^, as in upstream, and in Windows Terminal you type it as Ctrl+6, so Ctrl+6 then `.` quits the session.

The client was tested by hand in Windows Terminal on an ARM64 Windows 11 laptop (the x86_64 build under emulation) against a Linux server ([INTERVENTIONS.md](INTERVENTIONS.md)).
Connecting, typing, `ls --color`, vim, UTF-8 output, Ctrl-C, resizing and quitting all worked.
The session stayed live when Wi-Fi was turned off and back on.
Sleep and wake was tested on 2026-10-13 with `top` running, and after a minute asleep the session was live and `top` kept updating.
While `top` redrew, `mosh-client.exe` used 0.1% of one core and 14.7 MB of memory, measured over 60 seconds.
Windows has no job control, so Ctrl-^ Ctrl-Z does not suspend the client and the Ctrl-Z goes to the remote shell instead.

## How correctness is checked

The correctness checks compare mosh-rs with upstream's own C++ code instead of relying on unit tests.
Each check is a program you can run, and the results go into [STATUS.md](STATUS.md).

- Reference oracles: small C++ harnesses linked against the upstream build dump the emulator's screen, `new_frame` output, and encrypted packets, and the Rust versions must produce byte-identical output.
- Differential fuzzing runs generated escape-sequence inputs (one run of 1,000,000 and several of 200,000, plus random screen pairs for `new_frame`) through both emulators, and any mismatch is minimized and fixed.
- A corpus of ten recorded sessions (bash, vim, less, htop, tmux, the Claude Code TUI, `seq`, a resize, 256-color and truecolor, wide characters and emoji) and 17 inputs from upstream's own emulation tests is replayed through both emulators.
- The interop matrix runs Rust and C++ clients and servers in every combination, under loss, delay, reordering, blackouts, and roaming.
- Upstream's `make check` runs with mosh-rs binaries swapped in.
- End-to-end tests drive the real binaries, mostly in tmux, and most cases also run the C++ program for comparison.
  They cover the client, the server, prediction timing at 300 ms round trip, the launcher's options and messages compared with `/usr/bin/mosh` (73 cases), terminfo answers for every terminal type, and a hostile scrollback server.
- The scrollback oracle requires `seq 1 50000` run through mosh-rs inside tmux to appear in history exactly, with no gaps, duplicates, or blank lines.
  That includes runs under 30% loss, a vim excursion, `clear`, and a width resize (height resizes have the known limit above).
- Benchmarks run side by side with C++ mosh 1.4.0 in the same harness.

## How this was built

AI coding agents wrote the port, working from the upstream mosh source under a written plan, [PLAN.md](PLAN.md).
Every module was checked against upstream's own code by differential testing (see [How correctness is checked](#how-correctness-is-checked)) and reviewed line by line against the upstream code it came from.
The full record of the work is in this repository:

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

Contributions are welcome.
The one firm rule is that behavior must match upstream mosh master (decd9b7, March 2026) exactly, apart from the deviations listed in [PLAN.md](PLAN.md) section 9.
Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request, and the [Code of Conduct](CODE_OF_CONDUCT.md) before joining a discussion.

## Security

Please do not report vulnerabilities in public issues.
See [SECURITY.md](SECURITY.md) for private reporting.

## License and attribution

mosh-rs is a port of mosh, which is GPLv3, so it is licensed under the [GNU General Public License v3.0 or later](LICENSE).

mosh was created by **Keith Winstein** and the mosh authors.
The protocol, the emulator design, the prediction overlay, and the test suite that mosh-rs is checked against are their work.
See [mosh.org](https://mosh.org), the upstream repository [mobile-shell/mosh](https://github.com/mobile-shell/mosh), and the paper ["Mosh: An Interactive Remote Shell for Mobile Clients"](https://mosh.org/mosh-paper.pdf) (Winstein and Balakrishnan, USENIX ATC 2012).

mosh-rs is an independent project and is not affiliated with or endorsed by MIT, Keith Winstein, or the mosh maintainers.
Please report mosh-rs bugs here, not upstream.
