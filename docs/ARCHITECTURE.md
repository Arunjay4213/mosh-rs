# mosh-rs architecture

This document is for contributors.
It explains what a mosh session is made of, how mosh-rs splits that into crates, and how the two new features (scrollback and the Windows client) fit in without breaking wire compatibility with mosh 1.4.0. The code is ported from mosh master (decd9b7, March 2026).
The authoritative plan is [PLAN.md](../PLAN.md); section numbers below refer to it.

mosh-rs is under active development and most of what is described here is not built yet.
This document describes the design the code is being ported to, not a finished product.

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
| `crates/platform` | local terminal, pseudoterminal, process spawning | platform code from `src/util` and `src/frontend` |
| `crates/mosh` | `statesync`, the client (`stmclient`, `terminaloverlay`), the server, the launcher | `src/statesync`, `src/frontend`, `scripts/mosh.pl` |

Notes for each crate:

- **terminal** is pure Rust with no I/O and no platform code.
  Type and method names mirror upstream (`Complete`, `Framebuffer`, `DrawState`, `Cell`, `Row`, `Display`) so the frontend port can follow upstream line by line.
  Character widths come from a table generated from glibc `wcwidth`, because upstream calls `wcwidth` and both emulators must agree on every width.
- **network** uses hand-written `prost` structs: no `.proto` compilation and no build script.
  The transport is generic over a `State` type with exactly the methods upstream's C++ template requires: `diff_from`, `init_diff`, `apply_string`, `subtract`, and equality.
- **platform** is the only crate with `#[cfg(unix)]` and `#[cfg(windows)]` code.
  It provides raw mode on and off, size query, resize detection, blocking byte reads on a thread, and byte writes for the local terminal; forkpty, window size, and child wait for the pseudoterminal (Unix only); and spawning `ssh` and reading its output (both platforms).
- **mosh** produces three executables with upstream's names and flags: `mosh`, `mosh-client`, and `mosh-server`.
  Upstream's test harness and existing user scripts work with them unchanged.

Not shipped:

- `ref/`: small C++ programs linked against the upstream build, used as oracles (`ref-term`, `ref-crypto`).
- `tools/`: `udp-impair` (a UDP proxy that adds loss, delay, reordering, and client-address changes), the session corpus recorder, and the benchmark.
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

Everything is gated on negotiation, so old peers see byte-identical behavior.

1. **Negotiation.**
   A new `ClientBuffers::Instruction` field 4, `ClientCapabilities { optional bool scrollback = 1; }`, is the first event in the client's `UserStream`.
   Upstream `UserStream::apply_string` ignores unknown extensions, so an old server sees nothing.
   A new server that receives it turns scrollback mode on for that session.
2. **Server capture.**
   When the scroll region is the full screen and the remote program has not switched to the alternate screen (DEC modes 1049, 1047, or 47, tracked as a new flag in `DrawState`), `Framebuffer::scroll` records each row that leaves the top.
   Rows are rendered to ANSI strings with the same row renderer `new_frame` uses, and appended to a ring buffer (default 10,000 lines, server flag `--scrollback N`).
   The ring lives outside the state object and is shared by `Rc`, so copying `Complete` for the sender's state history stays cheap; `Complete` itself carries only `scrolled_total: u64`.
3. **Wire.**
   A new `HostBuffers::Instruction` field 9, `ScrolledLines { optional uint64 first_index = 1; repeated bytes line = 2; optional uint64 dropped = 3; }`.
   `Complete::diff_from(existing)` includes the lines with index in `(existing.scrolled_total, self.scrolled_total]` that are still in the ring, and `dropped` for any that were evicted.
   `Complete` equality includes `scrolled_total` in scrollback mode; otherwise `yes | head` would scroll without the framebuffer changing, and nothing would be sent.
4. **Client rendering.**
   In scrollback mode the client does not send smcup, and runs `new_frame` with scroll detection off.
   Before each render, for each pending scrolled line L, it moves the cursor to row 1 column 1, writes L, moves to the last row, and writes `\n`.
   The local terminal moves row 1 (now L) into its history and shifts the screen up one row.
   After injecting S lines, the client's copy of the last frame is shifted up S rows with S blank rows appended, and `new_frame` runs as usual.
   For a one-line scroll this costs one newline plus one row repaint, the same as today.
   The whole render is wrapped in synchronized output (`\e[?2026h` ... `\e[?2026l`), which terminals without support ignore.
5. **Compatibility.**
   Old client with a new server: no capability arrives, scrollback mode stays off, and behavior is identical to upstream.
   New client with an old server: no `ScrolledLines` ever arrive, the screen is repainted in place, and the client prints a one-line notice that the server does not support scrollback.
6. **Stretch goal, only after the scrollback oracle passes.**
   Forward the alternate-screen flag to the local terminal (emit 1049h/l on transitions, then a full repaint), so the mouse wheel inside vim behaves as it does over ssh and the pre-vim screen comes back on exit.

## Windows client (PLAN.md section 7)

- **Console setup.**
  `SetConsoleMode` enables `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on stdout and `ENABLE_VIRTUAL_TERMINAL_INPUT` on stdin, with processed input off, and `SetConsoleCP` and `SetConsoleOutputCP` set UTF-8.
  After that, keys arrive as the same escape-sequence bytes a Unix tty produces, and output is plain VT bytes.
  There is no key-event decoding and re-encoding layer.
- **Resize.**
  The client polls `GetConsoleScreenBufferInfo` on every loop iteration; the loop already wakes at least every 50 ms.
- **Ctrl+C** arrives as byte 0x03 and goes to the server like any other key.
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

Upstream's Perl launcher finds the server's IP address by acting as its own ssh `ProxyCommand` (`--fake-proxy`), so it sees the hostname ssh resolved from `~/.ssh/config`.
mosh-rs replaces that with `ssh -G <host>`, which prints the resolved `hostname`, `port`, and `user`.
The launcher then resolves the hostname with the system resolver, so names like Tailscale MagicDNS names resolve the same way they do for ssh.

Flags supported for v0.1:

- `--server=COMMAND` and `--ssh=COMMAND`
- `-p`, `--port=PORT[:PORT2]`
- `--predict=adaptive|always|never|experimental`
- `--no-init`
- `--local`
- `-4` and `-6`
- `--scrollback=N` (new)
- `[user@]host`, and a trailing command after the host

## Deviations from upstream

The complete list of allowed differences from upstream is PLAN.md section 9.
Anything else that differs from upstream is a bug.
