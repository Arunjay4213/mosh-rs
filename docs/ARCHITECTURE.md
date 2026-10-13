# mosh-rs architecture

This document is for contributors.
It explains what a mosh session is made of, how mosh-rs splits that into crates, and how the two new features (scrollback and the Windows client) fit in without breaking wire compatibility with mosh 1.4.0.
The code is ported from mosh master (decd9b7, March 2026).
The authoritative plan is [PLAN.md](../PLAN.md); section numbers below refer to it.

mosh-rs is pre-alpha, and v0.1.0 is its first pre-release.
Everything described here is built and checked by the test suite, and the results are recorded in [STATUS.md](../STATUS.md).

## The big picture

```text
 local machine                                              remote host
+-------------------+                                   +-------------------+
| local terminal    |                                   | user's shell      |
+---------^---------+                                   +---------^---------+
          | raw bytes                                             | bytes
+---------v---------+                                   +---------v---------+
| mosh-client       |                                   | pseudoterminal    |
|  poll loop        |                                   +---------^---------+
|  prediction       |                                             |
|  mirror emulator  |                                   +---------v---------+
+---------^---------+                                   | mosh-server       |
          |                                             |  poll loop        |
          |                                             |  emulator         |
          |                                             +---------^---------+
          |     UDP datagrams, AES-128-OCB3, SSP              |
          +<--------------------------------------------------+
            client -> server: UserStream (keys, resizes)
            server -> client: Complete (screen diffs as escape sequences)

 setup, once:  mosh --ssh--> mosh-server new --prints--> MOSH CONNECT <port> <key>
```

## The four layers of a mosh session

A mosh session has four layers, bottom to top (PLAN.md section 1).
Upstream mosh is about 17,000 lines of C++ and protobuf.
The terminal emulator (about 4,700 lines) and the prediction overlay (about 1,240 lines) are the hard parts.

### 1. Crypto (upstream `src/crypto`)

Every UDP packet is encrypted and authenticated with AES-128 in OCB3 mode (RFC 7253), with a 16-byte tag.
The server generates the 128-bit key, prints it once over ssh as 22 base64 characters, and never sends it again.
The nonce is 12 bytes: 4 zero bytes, then a big-endian u64 whose top bit is the direction (1 means "to the client") and whose low 63 bits are the packet sequence number.
On the wire, a packet is the low 8 bytes of the nonce followed by the OCB ciphertext.
Inside the plaintext are a 16-bit timestamp, a 16-bit timestamp echo used to measure round-trip time, and then the payload.

### 2. Transport, called SSP (upstream `src/network`)

SSP is the State Synchronization Protocol.
Neither side streams bytes.
Instead, each side keeps numbered snapshots of a state object and sends a `TransportBuffers::Instruction` with `old_num`, `new_num`, `ack_num`, `throwaway_num`, and `diff`, the bytes that turn state `old_num` into state `new_num`.
Diffs are zlib-compressed and split into fragments (id, fragment number, final flag) that each fit in one UDP packet.
The sender picks its send interval from the measured round-trip time and resends when the other side has not acknowledged.
Because it always sends the diff from the newest state the receiver has acknowledged, a lost packet is replaced by fresher data instead of being retransmitted.
The server adopts a new client address whenever a valid packet arrives from one; that single rule is what makes roaming work.

### 3. State objects (upstream `src/statesync`)

Client-to-server state is `UserStream`: an ordered list of keystroke and resize events (`userinput.proto`).
Server-to-client state is `CompleteTerminal`: a full terminal framebuffer plus an echo-ack counter (`hostinput.proto`).

The server-to-client diff is not the program's raw output.
It is an ANSI escape string computed by `Display::new_frame` that redraws the client's mirror framebuffer from the old snapshot to the new one.
The client feeds that string into its own copy of the emulator.
So both ends run the same emulator, and the client's emulator must interpret that redraw subset exactly as upstream's does.
If it does not, the client's mirror drifts from the server's screen and never recovers.
This is why the terminal crate is tested cell for cell against upstream's own code.

### 4. Frontend (upstream `src/frontend`, `scripts/mosh.pl`)

- `mosh-server` reads `SSH_CONNECTION`, binds a UDP port in 60000-61000, prints `MOSH CONNECT <port> <key>`, forks, detaches from the ssh session, and runs the user's shell under a pseudoterminal.
- `mosh-client` puts the local terminal in raw mode, runs the poll loop, renders the mirror framebuffer to the real terminal, shows the status line, and runs the prediction overlay: local echo of typed characters, later confirmed or corrected by the server.
- `mosh` (a Perl script upstream) runs `ssh host -- mosh-server new ...`, reads the `MOSH CONNECT` line, and starts the client with the key in the `MOSH_KEY` environment variable.

## Crates (PLAN.md section 2)

One Cargo workspace, four shipped crates, plus tooling that is not shipped.

| Crate | Contents | Ported from |
| --- | --- | --- |
| `crates/terminal` | parser state machine, framebuffer, dispatcher, terminal functions, `Display` (render and `new_frame`) | `src/terminal` |
| `crates/network` | `crypto` (OCB3 session, key, nonce), `proto` (protobuf structs), `transport` (fragments, compression, sender, receiver, connection, timing, roaming) | `src/crypto`, `src/network`, `src/protobufs` |
| `crates/platform` | local terminal, pseudoterminal, process spawning, UDP sockets, the select loop, stdio writes that ignore errors | platform code from `src/util`, `src/frontend` and `src/network` |
| `crates/mosh` | `statesync`, the client (`stmclient`, `terminaloverlay`), the server, the launcher | `src/statesync`, `src/frontend`, `scripts/mosh.pl` |

Notes for each crate:

- **terminal** is pure Rust with no I/O and no platform code.
  Type and method names mirror upstream (`Complete`, `Framebuffer`, `DrawState`, `Cell`, `Row`, `Display`) so the frontend port can follow upstream line by line.
  Character widths come from a table generated from glibc `wcwidth`, because upstream calls `wcwidth` and both emulators must agree on every width.
- **network** uses hand-written `prost` structs: no `.proto` compilation and no build script.
  The transport is generic over a `State` trait with the methods upstream's C++ template requires: `diff_from`, `init_diff`, `apply_string` (returning an error for malformed input, PLAN.md 9.8), `subtract`, `reset_input`, `compare` (the sender's verbose round-trip check), and equality, plus `subtract_self` for upstream's `subtract( this )`, which Rust borrows cannot express.
  It has no `unsafe` and no platform code; sockets come from the platform crate.
- **platform** is the only crate with `#[cfg(unix)]` and `#[cfg(windows)]` code (apart from the `#[cfg(unix)]` that builds the server only on Unix), and the only shipped crate with `unsafe` (the others carry `#![forbid(unsafe_code)]`).
  It provides raw mode on and off, size query, resize detection and byte reads and writes for the local terminal; forkpty, window size, and child wait for the pseudoterminal (Unix only); spawning `ssh` and reading its output (both platforms); UDP sockets and name resolution; the select loop with signals; and `outp!`/`errln!` style printing that ignores a failed write, as C stdio does.
- **mosh** produces three executables with upstream's names and flags: `mosh`, `mosh-client`, and `mosh-server`.
  Upstream's test harness and existing user scripts work with them unchanged.

Not shipped:

- `ref/`: small C++ programs linked against the upstream build, used as oracles (`ref-term`, `ref-crypto`); the matching Rust programs are `vt-check` and `crypto-check` in `tests/src/bin`.
- `tools/`: `udp-impair` (a UDP proxy that adds loss, delay, reordering, blackouts and client-address changes), `pty-record` (the session corpus recorder), and `bench`.
- `tests/`: end-to-end and conformance tests, run with `cargo test --release` and `tests/run-all.sh`.

Design constraints that apply everywhere:

- No async runtime.
  Both programs run one poll loop with a computed timeout, exactly like upstream's select loop.
- No trait unless there are at least two real implementations.
- The protocol version stays 2.
  New features are additive protobuf fields that old peers ignore.

## Scrollback (PLAN.md section 6)

### Why upstream has none

The client enters the alternate screen (`Display::open` sends smcup), so the local terminal never scrolls and never collects history.
The server's framebuffer also discards rows that scroll off, so lines that pass between two frames are lost.
Upstream's `new_frame` already detects full-screen scrolls and emits newlines at the bottom row (`terminaldisplay.cc` lines 168-243).
That is why a client patched to skip smcup gets partial scrollback, with gaps and with the status line leaking into history.

### Design

Everything depends on negotiation, so old peers see byte-identical behavior.
PLAN.md section 6 has every detail; this is the summary of what is implemented.

1. **Negotiation.**
   `mosh --scrollback` starts the client with `MOSH_SCROLLBACK=1`.
   The client then sends a new `ClientBuffers::Instruction` field 4, `ClientCapabilities { optional bool scrollback = 1; }`, as the first event in its `UserStream`.
   Upstream `UserStream::apply_string` ignores unknown extensions, so an old server sees nothing.
   A mosh-rs server that receives it turns scrollback mode on for that session and announces it in its first frame.
2. **Server capture.**
   When the scroll region is the full screen and the remote program is not on the alternate screen (DEC modes 1049, 1047, or 47, tracked as a new flag in `DrawState`), `Framebuffer::scroll` records each row that leaves the top.
   Erasing the whole display (ED 2, or ED 0 from the home position) also records the rows on screen first, as tmux does, so `clear` loses nothing.
   In scrollback mode the server keeps a real alternate screen, as tmux does: the main screen's rows are put aside while vim or less runs and come back when it exits, so they scroll into history later, once.
   Rows are rendered to ANSI strings with the same row renderer `new_frame` uses (`Display::render_scrolled_row`) and kept in a backlog shared by `Rc`, so copying `Complete` for the sender's state history stays cheap; `Complete` itself carries only `scrolled_total: u64`.
3. **Windowed history, never blocking the program.**
   The backlog holds only lines the client has not acknowledged, capped in bytes (`mosh-server --scrollback MIB`, default 32 MiB, `K` suffix for KiB, 0 turns capture off).
   The server publishes at most 4,856 bytes of history (four fragments) beyond what the client has acknowledged, so history follows the screen a few round trips behind and survives loss (the history of `seq 1 50000` takes about 9 s on loopback; the screen itself is not delayed).
   The remote program is never paused: on overflow the oldest unpublished lines become a dropped count.
   This was a decision, recorded in INTERVENTIONS.md ("history is windowed and never blocks the remote program"): pausing the program would make history exact in every case, but would freeze the screen under loss and stall a long build while the client's machine sleeps.
4. **Wire.**
   New `HostBuffers::Instruction` field 9, `ScrolledLines { optional uint64 first_index = 1; repeated bytes line = 2; optional uint64 dropped = 3; }`, and field 10, `ScreenMode { optional bool alternate_screen = 1; }`, sent only in scrollback mode.
   `Complete` equality includes `scrolled_total` and the announcement, so publishing lines without a screen change is still sent.
5. **Client rendering.**
   The client starts in the alternate screen as upstream; when the first frame carries the announcement it leaves it, scrolls the host's existing screen into history, and from then on draws on the host's main screen with `new_frame`'s scroll shortcut off.
   Before each render, for each received line it moves to row 1, writes the line (re-rendered through a one-row emulator at the host's width, so nothing from the server reaches the host unparsed), moves to the last row, and writes a line feed; the host moves row 1 into its history.
   Missing lines become one dim `[mosh: N lines of history were dropped]` line in their place.
   When the server's `ScreenMode` turns on, the client writes `CSI ?1049h` and repaints, so vim and less run in the host's own alternate screen; when it turns off, `CSI ?1049l` brings the host's main screen back.
   The whole render is wrapped in synchronized output (`\e[?2026h` ... `\e[?2026l`), which terminals without support ignore.
6. **Compatibility.**
   Old client with a new server: no capability arrives, scrollback mode stays off, and behavior is identical to upstream.
   New client with an old server (or `--scrollback 0`): the first frame has no announcement, so the client stays in the alternate screen exactly as upstream and shows a notice for 5 s.
7. **Known limit.**
   Changing the window height can lose or repeat a few rows, because host terminals keep the bottom of the screen on resize while upstream's emulator keeps the top (PLAN.md 6.4).

## Windows client (PLAN.md section 7)

- **Console setup.**
  `SetConsoleMode` enables `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on stdout and `ENABLE_VIRTUAL_TERMINAL_INPUT` on stdin, with processed input off, and `SetConsoleCP` and `SetConsoleOutputCP` set UTF-8.
  After that, keys arrive as the same escape-sequence bytes a Unix tty produces, and output is plain VT bytes.
  There is no key-event decoding and re-encoding layer.
- **Input.**
  The select loop polls sockets only, so a thread reads the console with `ReadConsoleW` and forwards the bytes over a loopback TCP connection that the loop watches.
- **Resize.**
  While the client watches for resizes, the select loop waits at most 50 ms and compares the console window size (`GetConsoleScreenBufferInfo`) after every wait.
- **Ctrl+C** arrives as byte 0x03 and goes to the server like any other key.
- **Escape key.** Ctrl-^ as upstream, typed as Ctrl+6 in Windows Terminal.
- **TERM.** When TERM is unset or empty, `mosh.exe` and `mosh-client.exe` set `TERM=xterm-256color` for themselves (PLAN.md 9.12).
- **ssh.**
  The launcher spawns `ssh.exe` from the Windows OpenSSH client that ships with Windows 10 and later.
- **No Windows server.**
  `mosh-server` is Unix-only, as upstream.

### Building for Windows

Releases are built with the MSVC target on a Windows GitHub Actions runner.
For development, cross-compile from Linux with the MinGW toolchain (the linker is configured in `.cargo/config.toml`):

```sh
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu -p mosh --bin mosh --bin mosh-client
# output: target/x86_64-pc-windows-gnu/release/mosh.exe and mosh-client.exe
```

On a Windows machine with Rust and the Visual Studio C++ build tools installed, build natively instead:

```sh
cargo build --release -p mosh --bin mosh --bin mosh-client
```

## Launcher (PLAN.md section 8)

Like upstream's Perl launcher, `mosh` finds the server's IP address by acting as its own ssh `ProxyCommand` (`mosh --fake-proxy -- %h %p`).
ssh hands it the hostname resolved from `~/.ssh/config`; the launcher resolves that with the system resolver (so Tailscale MagicDNS names work as they do for ssh), connects to the first address that accepts a TCP connection, reports it as `MOSH IP`, and relays the ssh session over that connection.
mosh-server binds the address it sees in `SSH_CONNECTION`, which is the same one the client is given, so both agree even on hosts with IPv4 and IPv6 addresses.
An earlier design that resolved the name separately (`ssh -G`) was dropped after a dual-stack bug; `--fake-proxy` is kept as in mosh.pl (PLAN.md section 8).

The ProxyCommand words are quoted for the shell that runs them: POSIX single quotes on Unix, and Windows command-line rules on Windows (double quotes only around a word with a space, tab or quote; backslashes untouched), because Windows OpenSSH hands the ProxyCommand to `CreateProcessW` without a shell (`mosh_platform::process::proxy_command_quote`).
On Windows the launcher prefers `mosh-client.exe` next to `mosh.exe`; elsewhere it runs `mosh-client` from PATH, as mosh.pl does.

Options are mosh.pl's complete set, parsed the way its Getopt::Long configuration parses them, with the same messages and exit codes (`tests/tests/launcher.rs` compares 73 cases with `/usr/bin/mosh`).
The differences are `--scrollback` (new, a flag that starts the client with `MOSH_SCROLLBACK=1`) and `--bind-server`, which takes `ssh` or `any` only (PLAN.md 9.3).
`mosh --help` prints the full list.

## Deviations from upstream

The complete list of allowed differences from upstream is PLAN.md section 9.
Anything else that differs from upstream is a bug.
