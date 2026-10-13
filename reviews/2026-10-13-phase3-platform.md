# Phase 3 review: crates/platform (Windows console, unix tty, pty, signals, sockets) against upstream mosh master (decd9b7)

Date: 2026-10-13.
Commit reviewed: 026eadb.
Scope: every file in `crates/platform/src` (`tty.rs`, `select.rs`, `pty.rs`, `server.rs`, `socket.rs`, `netdb.rs`, `locale.rs`, `coredump.rs`, `process.rs`, `stdio.rs`, `lib.rs`), the `#[cfg(windows)]` halves against the Win32 API contracts and PLAN.md section 7, and the `#[cfg(unix)]` halves against `upstream/src/frontend/stmclient.cc`, `upstream/src/frontend/mosh-server.cc`, `upstream/src/util/select.h`, `upstream/src/util/pty_compat.cc`, `upstream/src/util/swrite.cc`, `upstream/src/network/network.cc`, `upstream/src/util/locale_utils.cc`, `upstream/src/crypto/crypto.cc` and `upstream/scripts/mosh.pl`, walked function by function.
The Phase 1 and Phase 2 to 4 reviews cover the other crates; `crates/mosh/src/client/stmclient.rs` and `crates/network/src/transport/network.rs` were read only where they call into this crate (the raw-mode guard, the panic hook, the stdin fd, `Select`, `bind`).
Windows code cannot run here, so every Windows claim below is reasoned from the documented Win32 contracts and from the `windows-sys 0.61.2` source in the local registry; the section "Not verifiable from this machine" lists what remains open.

## Summary

Semantic deviations from upstream in the unix code: 0 found; termios handling, the signal setup, `pselect`, `forkpty`, the socket options, `recvmsg` with the ECN mark, the locale checks and the core-dump limit match upstream line by line (details under "Verified correct").
Windows console path: the mode flags, code pages, save and restore on exit, early return and panic, the blocking input thread and the 50 ms resize poll are as PLAN section 7 says, with one hole in the restore guarantee (P2, Ctrl+Break) and one panic on a runtime-fallible setup (P1).
Unlisted deliberate differences: 1 (P4, reading `/run/utmp` directly instead of `getutxent` on Linux) that PLAN.md section 9 must list.
`unsafe` blocks: 90 (tty 19, socket 19, netdb 14, server 14, select 10, pty 7, coredump 4, locale 2, process 1), every one with a `// SAFETY:` comment within the preceding lines, and each invariant checked by hand; the one `unsafe fn` (`socket.rs:274`, `from_sockaddr`) carries a `# Safety` section.
`.expect()`: 2, one with a provably local invariant (`socket.rs:140`) and one on a runtime-fallible value (P1); `.unwrap()`: 0 (`unwrap_or*` forms only).
`assert!`: 10 in `select.rs`, each mirroring an upstream `fatal_assert` at the same place.
`#[allow(...)]`: 2, one with the required reason (`server.rs:281`) and one without (P3).
`pub` items without a doc comment: 0 (`cargo clippy -p mosh-platform --lib -- -W missing_docs` reports none), but the crate does not deny them (P12).
`cargo clippy -p mosh-platform --all-targets -- -D warnings`: passes on this laptop (rustc stable, aarch64).
`cargo fmt --check -p mosh-platform`: passes.
`cargo check -p mosh-platform --target x86_64-pc-windows-gnu`: not run, the target is not installed here (`rustup target list --installed` shows only `aarch64-unknown-linux-gnu`) and the brief says not to install it; STATUS.md records the Windows clippy as clean on devbox for the same sources.
Blocking: 0.
Should fix: 4 (P1 to P4).
Nice to have: 9 (P5 to P13).

## Blocking

None.

## Should fix

### P1. Windows console input setup panics instead of returning an error

`crates/platform/src/tty.rs:356-360` (`stdin_socket`) initializes the forwarding socket with `start_stdin_thread().expect("cannot start console input thread")`.
`start_stdin_thread` (`:362-389`) can fail at run time: `TcpListener::bind("127.0.0.1:0")`, `TcpStream::connect`, `accept`, `set_nodelay` and `GetStdHandle` all return errors on a machine with a broken loopback adapter, a Winsock layered provider, port exhaustion or a missing stdin handle.
The first caller is `stmclient.rs:1022` (`self.sel.add_fd(tty::stdin_fd())`), which runs after `raw_mode_on`, so the panic happens with the console in raw mode; the panic hook does restore it, but CLAUDE.md says runtime errors are returned, never panicked, and the message the user sees is a Rust panic instead of upstream's `perror` style line.
Fix: start the input thread inside `raw_mode_on()` (which already returns `io::Result`) and store the receiver in the `OnceLock` there, so `stdin_fd()` and `read_stdin()` only ever see an initialized value; or make `stdin_fd()` return `io::Result<Fd>` and let the client print the error and exit 1 as it does for `tcgetattr`.

### P2. Ctrl+Break exits without restoring the console

`crates/platform/src/tty.rs:293-341` restores the console modes and code pages through `Drop` and `raw_mode_off`, which covers normal exit, early returns and panics.
It does not cover console control events: the `SetConsoleMode` documentation says `ENABLE_PROCESSED_INPUT` controls only Ctrl+C, and the `SetConsoleCtrlHandler` documentation says `CTRL_BREAK_EVENT` is delivered regardless of the input mode; with no handler installed the default handler calls `ExitProcess` from the control thread, so no `Drop` runs.
After that the parent shell's console keeps `ENABLE_VIRTUAL_TERMINAL_INPUT` on, `ENABLE_PROCESSED_INPUT`, `ENABLE_LINE_INPUT` and `ENABLE_ECHO_INPUT` off and both code pages at 65001; cmd.exe then reads keys without echo or line editing until the user runs something that resets the mode (PowerShell with PSReadLine happens to reset it at the next prompt).
On unix the same key, Ctrl+\, reaches the client as byte 0x1C because `cfmakeraw` clears `ISIG` (stmclient.cc:115), so nothing can kill the client from the keyboard except through the server.
Fix: in `raw_mode_on` keep a copy of the saved modes and code pages in a `static` and call `SetConsoleCtrlHandler` with a handler that, for `CTRL_BREAK_EVENT`, returns `TRUE` so the event is swallowed like `ISIG` off swallows SIGQUIT (closest to upstream), or restores the saved state and returns `FALSE` so the default exit follows; `CTRL_CLOSE_EVENT`, logoff and shutdown need nothing because the console is destroyed with the process.
Record the choice in PLAN.md section 7.

### P3. `#[allow]` without the required reason

`crates/platform/src/select.rs:54` has `#[allow(clippy::declare_interior_mutable_const)]` with no comment on the line above, which CLAUDE.md requires for every allow.
The const exists only to initialize the `GOT_SIGNAL` array on `:58`.
Fix: write the array initializer as `[const { AtomicBool::new(false) }; MAX_SIGNAL_NUMBER + 1]` (inline const blocks are stable since 1.79, below the 1.89 floor) and delete both the const and the allow; or add the one-line reason above it.

### P4. Reading `/run/utmp` directly is a deviation PLAN section 9 does not list

`crates/platform/src/server.rs:235-271` (`getutxent_all`, Linux x86_64 and aarch64) parses `/run/utmp` or `/var/run/utmp` with a hand-measured glibc record layout (384 or 400 bytes, `ut_type` at 0, `ut_line` at 8, `ut_user` at 44, `ut_host` at 76) where upstream calls `getutxent()` (mosh-server.cc:1046).
The doc comment gives a good reason (musl's `getutxent` is a stub, so a static musl `mosh-server` would never warn about detached sessions) and the offsets check out against `struct utmpx` in glibc's `<utmpx.h>` (`__WORDSIZE_TIME64_COMPAT32` makes the x86_64 record 384 bytes, aarch64 400), but CLAUDE.md allows only the differences listed in PLAN.md section 9, and 9.2 covers only writing records.
Two robustness points come with the same fix: a file whose length is not a multiple of the record size is parsed anyway up to the last full record, which is fine for a truncated tail but hides a layout mismatch (a different libc or a future glibc change), and the `detached_warning` test (`tests/tests/detached_warning.rs:254`) only shows that the parser agrees with the running system's `utmp`, not that the layout is glibc's on another distribution.
Fix: add to PLAN 9.2 that on Linux x86_64 and aarch64 the records are read from the utmp file with glibc's layout so glibc and musl builds behave the same, and `getutxent` is used elsewhere; in `getutxent_all` return no records when `data.len() % RECORD != 0` (as `getutxent` does for an unreadable file) and say so in the doc comment.

## Nice to have

### P5. `GetConsoleCP` failure is not checked

`crates/platform/src/tty.rs:299` saves `GetConsoleCP()` and `GetConsoleOutputCP()` without checking the result; both return 0 on failure per their documentation.
A saved 0 makes `restore_saved` call `SetConsoleCP(0)`, which fails with `ERROR_INVALID_PARAMETER`, so the console keeps code page 65001 after exit and `raw_mode_off` reports a confusing error.
Fix: treat 0 as `io::Error::last_os_error()` in `raw_mode_on`, like `console_mode` does for `GetConsoleMode`.

### P6. `ENABLE_PROCESSED_OUTPUT` is not forced on

`crates/platform/src/tty.rs:286-288` (`raw_output_mode`) ORs in `ENABLE_VIRTUAL_TERMINAL_PROCESSING` and `DISABLE_NEWLINE_AUTO_RETURN` but leaves `ENABLE_PROCESSED_OUTPUT` as inherited; the `SetConsoleMode` documentation says VT processing requires it ("ENABLE_VIRTUAL_TERMINAL_PROCESSING ... should be used with ENABLE_PROCESSED_OUTPUT").
It is on by default, so this only matters when a parent program cleared it, and then every escape sequence would print as text.
Fix: OR `ENABLE_PROCESSED_OUTPUT` into the raw output mode.

### P7. Windows `select` with no sockets and an infinite timeout returns at once

`crates/platform/src/select.rs:356-359` sleeps only when `timeout > 0`; a negative timeout (wait forever) with an empty fd list returns immediately where upstream's `pselect(0, ..., NULL, ...)` blocks until a signal.
No caller does this today (the client always adds the stdin socket, `stmclient.rs:1022`, and `mosh-server` is unix only), so the busy loop is latent.
Fix: sleep 50 ms when `timeout < 0` and the list is empty (the same cap the resize poll uses), or return an error.

### P8. The console input thread drops read errors silently

`crates/platform/src/tty.rs:378-379` breaks out of the loop on any `ReadConsoleW` or `ReadFile` error, which the main loop sees as end of file, so the client exits with no message where unix prints `read: <strerror>` (`stmclient.rs:795`).
Fix: `errln!("read: {e}")` before the `break`, or pass the error through a shared slot that `read_stdin` returns once.

### P9. Error text carries " (os error N)" where upstream prints `strerror` only

`crates/platform/src/coredump.rs:56-59` and `:66-69`, `locale.rs:136`, `socket.rs:202-205` and `:363-366` print an `io::Error` with `{}`, which formats as `No such file or directory (os error 2)`, where upstream's `perror` prints `getrlimit(RLIMIT_CORE): No such file or directory`.
The client already has a `perror()` (`stmclient.rs:100-105`) that uses `mosh_platform::socket::strerror` to get upstream's text, and the Windows `strerror` (`socket.rs:120-122`) has the same suffix problem.
Fix: move that `perror` into `stdio.rs`, use it at the five sites, and format the Windows `strerror` without the suffix (format the `io::Error` and strip the trailing ` (os error N)`, or call `FormatMessageW`).

### P10. `netdb.rs` and `socket.rs` disagree on the byte order of `sin6_flowinfo`

`crates/platform/src/netdb.rs:91` reads the flow label with `u32::from_be_bytes` while `socket.rs:292` passes `sin6.sin6_flowinfo` unchanged, as `std` does in its own `sockaddr_in6` conversion.
The launcher only uses `.ip()` from these results and flow labels are 0 in practice, so nothing observable changes, but one of the two is wrong for a non-zero label.
Fix: `u32::from_ne_bytes` in `netdb.rs`, as the scope id on the next line.

### P11. `IUTF8` is skipped on Cygwin

`crates/platform/src/tty.rs:124` and `server.rs:159` set `IUTF8` only for linux, android and macos; upstream's `HAVE_IUTF8` is set by configure wherever the header defines it, and the `libc` crate defines `IUTF8` also for `target_os = "cygwin"` (upstream names Cygwin in select.h:169, so it is a supported host).
Fix: add `target_os = "cygwin"` to both `cfg` lists.

### P12. The crate does not deny missing docs

`crates/platform/src/lib.rs` has no `#![deny(missing_docs)]`, which the three other crates gained for findings F3, F6 and G4; every `pub` item is documented today, so the attribute costs nothing and stops the next edit from regressing.
`#![forbid(unsafe_code)]` is correctly absent, this is the crate that is allowed `unsafe`.
Fix: add `#![deny(missing_docs)]` after the crate doc comment.

### P13. Windows `resolve` rejects IPv6 literals with a zone

`crates/platform/src/socket.rs:527-531` parses the host with `IpAddr::from_str`, which rejects `fe80::1%3`; `getaddrinfo` with `AI_NUMERICHOST` accepts it on both systems (the unix half does, `socket.rs:331-338`), so a link-local server address works from a unix client and not from Windows.
Fix: split on `%`, parse the zone as the interface index into `SocketAddrV6::scope_id`, or call WinSock's `getaddrinfo` as `netdb.rs:177-258` already does.

## Verified correct

- `tty.rs:112-137` (`raw_mode_on`) matches stmclient.cc:99-118: `tcgetattr` on stdin, copy to `raw_termios`, set `IUTF8` when missing, `cfmakeraw`, `tcsetattr(TCSANOW)`; `cfmakeraw` clears `ISIG`, `ICANON`, `ECHO`, `IEXTEN`, `OPOST`, `IXON`, `ICRNL` and the rest exactly as glibc's does because the libc function is called, and `tests/tests/platform_tty.rs` shows `-icanon -echo -isig` on a real pty.
- `raw_mode_off` and `RawModeGuard::restore_saved` (`tty.rs:142-156`) are stmclient.cc:215 and :357; `apply_raw` (`:147`) is `STMClient::resume`; `window_size` (`:167-176`) is the `TIOCGWINSZ` ioctl at :247 and :413 with `(ws_col, ws_row)` order preserved by the caller; `read_stdin` (`:180-187`) is the `read(STDIN_FILENO, ...)` of `process_user_input`; `suspend_self` (`:43-49`) is `kill(0, SIGSTOP)` at :367.
- `write_all_stdout` (`tty.rs:191-208`) is swrite.cc: loop until all bytes are written, `bytes_written <= 0` is an error, no EINTR retry.
- The `Drop` guard restores the terminal on early return and on unwinding; the client stores the guard in a thread local the panic hook reaches (`stmclient.rs:80-88`), so PLAN 9.11 holds on both systems.
- Windows `raw_mode_on` (`tty.rs:293-311`) sets exactly PLAN section 7's flags: `ENABLE_VIRTUAL_TERMINAL_INPUT` on, `ENABLE_PROCESSED_INPUT`, `ENABLE_LINE_INPUT`, `ENABLE_ECHO_INPUT` off, `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on, and both code pages 65001; `DISABLE_NEWLINE_AUTO_RETURN` matches `OPOST` off from `cfmakeraw` (LF is not turned into CR LF).
- If the second `SetConsoleMode` or `SetConsoleCP` fails inside `apply_raw`, the `?` drops the half-applied guard, whose `Drop` restores both handles and both code pages, so a partial setup never leaks.
- With `ENABLE_PROCESSED_INPUT` off, Ctrl+C is placed in the input buffer as 0x03 and no `CTRL_C_EVENT` is raised (SetConsoleMode documentation), which is the same byte a raw unix tty delivers; the user's second Phase 3 pass confirmed Ctrl+C interrupting `sleep` and Ctrl+^ `.` quitting in Windows Terminal (INTERVENTIONS.md, 2026-10-13 second pass), so 0x1E reaches the client there.
- Console input is read on its own thread in a blocking `ReadConsoleW` (`tty.rs:419-427`) and forwarded over a loopback TCP connection; the main loop blocks in `WSAPoll` on that socket plus the UDP sockets (`select.rs:371`), so there is no busy loop; the only periodic wakeup is the 50 ms cap while SIGWINCH is watched (`select.rs:351-353`), which PLAN section 7 asks for.
- The surrogate handling (`tty.rs:432-435`) does not split a pair across reads, lone surrogates become U+FFFD, and the buffer bound holds: 4096 UTF-16 units become at most 12288 UTF-8 bytes in a 16384 byte buffer.
- The loopback listener only accepts the connection whose peer address is the sender's own local address (`tty.rs:365-371`) and is closed when `start_stdin_thread` returns, so another local process cannot inject keystrokes.
- `window_size` on Windows (`tty.rs:344-354`) uses the visible window (`srWindow`), not the screen buffer height, which is what the terminal size means for a VT program.
- Resize detection (`select.rs:386-392`) compares the console size after every `select` and raises SIGWINCH into the same `GOT_SIGNAL` array, so `stmclient.rs:1058` handles it with the unix code path unchanged.
- `windows-sys 0.61.2` types match the code: `HANDLE` is `*mut c_void` (so `is_null()` and the `usize` round trip through the thread are valid), `CONSOLE_MODE` is `u32`, `POLLRDNORM` is `i16` as `WSAPOLLFD.events` is, `AI_NUMERICHOST` and `AI_CANONNAME` are `u32` (so the `as i32` casts at `netdb.rs:209-210` are needed and commented), `SOCK_STREAM` and `IPPROTO_TCP` are `i32`, `ADDRINFOA` implements `Default`; the workspace `windows-sys` entry supplies `Win32_System_Console`, `Win32_Foundation` and `Win32_System_Threading`, which union with the crate's three features, so the imports resolve.
- `select.rs` is select.h:62-215 line by line: `FD_ZERO` and `sigemptyset` in the constructor, `add_fd` with the `FD_SETSIZE` check, `add_signal` blocking the signal with `sigprocmask(SIG_BLOCK)` and installing the handler with `SA_RESTART` and a full `sa_mask`, the `MAX_POLLS` rate limiter with the three verbose messages, the `pselect` branch (upstream's `HAVE_PSELECT` path), the EINTR and `ret == 0` handling with the Cygwin check and `FD_ZERO`, and `any_signal` scanning `MAX_SIGNAL_NUMBER` entries (upstream's `i < MAX_SIGNAL_NUMBER`, which leaves signal 64 unseen; kept on purpose).
- The signal handler (`select.rs:100-106`) only stores to an atomic and has no assert, which is right for async-signal safety; the asserts in `signal_index` mirror upstream's `fatal_assert` at select.h:99-100 and :199-200.
- `pty.rs:77-87` calls libc's `forkpty(&master, NULL, NULL, &window_size)` through `nix::pty::forkpty`, which gives the child `setsid`, the slave as stdin, stdout and stderr and the controlling tty, as mosh-server.cc:526 and the `pty_compat.cc:139-156` fallback do; `set_winsize`, `get_winsize` and `set_window_size` are the `TIOCSWINSZ` and `TIOCGWINSZ` ioctls at mosh-server.cc:785-792; `execvp` passes a NUL-terminated argv with a separate `file`, so `-bash` as `argv[0]` works; `wait_child` retries EINTR.
- `server.rs:47-61` is mosh-server.cc:453-458 and :541-546 (`sa_flags = 0`, full `sa_mask`, `SIG_IGN` or `SIG_DFL`); `redirect_stdio_to_dev_null` is :496-513 with the same three error names; `pipe`, `isatty`, `tcdrain` are :521, :447, :482-486; `set_iutf8_stdin` is :557-568; `getpwuid` is :300 and :706 with `errno` cleared first so a NULL result reports the right error; the utmp fields (`ut_type == USER_PROCESS`, `ut_user`, `ut_host`, `ut_line`) are exactly what `warn_unattached` at :1046-1052 reads.
- `socket.rs:163-215` is network.cc:150-176 in order: `socket(family, SOCK_DGRAM, 0)`, `IP_MTU_DISCOVER = IP_PMTUDISC_DONT` (fatal) on Linux, `IP_TOS = 0x02` with the failure ignored, `IP_RECVTOS` with the `family == IPPROTO_IP` comparison kept as upstream wrote it (it compares an address family with a protocol number, so the message never prints; a faithful port of an upstream quirk); non-blocking mode replaces `MSG_DONTWAIT` per call, which is the same thing for `sendto` and `recvmsg`.
- `resolve` (`socket.rs:304-353`) uses upstream's hints at network.cc:280-282 and :352-354 (`AF_UNSPEC`, `SOCK_DGRAM`, `AI_NUMERICHOST | AI_NUMERICSERV`, plus `AI_PASSIVE` for binding) and the `Bad IP address (node): gai_strerror` message of :218.
- `bind` (`socket.rs:357-378`) turns `IPV6_V6ONLY` off for the IPv6 wildcard before `bind`, as network.cc:312-316, and returns errno for the caller's `Failed binding to` message; on Windows the replacement socket is created before any handle is read (`network.rs:369-376` creates, binds, and `fds()` at `:631` reads handles fresh each loop), so the swap is safe.
- `recv_raw` (`socket.rs:383-461`) is network.cc:431-470: `sockaddr_storage` name, one iovec, a 2048 byte control buffer (`RECEIVE_MTU`), `MSG_DONTWAIT`, `MSG_TRUNC` as "Received oversize datagram", `CMSG_FIRSTHDR` only, `IPPROTO_IP` level with `IP_TOS` or `IP_RECVTOS` type, and `(octet & 0x03) == 0x03` for congestion experienced; EAGAIN and EWOULDBLOCK become `Ok(None)`, which the caller treats as upstream's `continue` at :410.
- `from_sockaddr` checks the family and the length before each cast, so a short `msg_namelen` cannot read past the buffer; `to_sockaddr` sets `sin_len` and `sin6_len` on the BSDs.
- Windows sockets: `UdpSocket::bind` to the unspecified address and port 0 is what the first `sendto` on an unbound socket does, `WSAEWOULDBLOCK` maps to `Ok(None)` and `WSAEMSGSIZE` to the oversize message; Windows has no `IP_MTU_DISCOVER` (fragmentation is allowed by default, which is what `IP_PMTUDISC_DONT` asks for) and no usable `IP_TOS` or ECN reception, as the module doc says.
- `locale.rs` is locale_utils.cc:30-77: `get_ctype` checks `LC_ALL`, `LC_CTYPE`, `LANG` in that order, `locale_charset` renames `ANSI_X3.4-1968`, `is_utf8_locale` accepts `UTF-8` and `utf-8`, `set_native_locale` prints the two ENOENT messages with the double newline; `LOCALE_VARS` is mosh.pl:471 in the same order.
- `coredump.rs` is crypto.cc:284-312 exactly: `getrlimit` then `exit(1)` on failure, `rlim_cur` saved, `rlim_cur = 0`, and `reenable_dumping_core` re-reads the limit and restores only `rlim_cur` with the failure ignored, as upstream does.
- `process.rs`: the merged pipe matches mosh.pl's `open(STDERR, ">&STDOUT")` child, the write ends are released from the `Command` so the pipe reaches EOF, `ignore_sighup` is mosh.pl:305, `windows_command_line_quote` implements the `CommandLineToArgvW` rules (2n+1 backslashes before an inner quote, 2n before the closing quote).
- `stdio.rs`: the four macros swallow write errors as C's `printf` does, which the client relies on after ssh exits.
- Every `unsafe` block has a `// SAFETY:` comment stating the invariant, and each invariant is true at the call site: pointers to live locals with the right size, NUL-terminated `CString`s outliving the call, `getaddrinfo` results freed exactly once, `OwnedFd::from_raw_fd` on fresh descriptors, static strings from `strerror`, `gai_strerror` and `nl_langinfo` copied out at once, `fd_set` indices checked against `FD_SETSIZE` in `add_fd` before `FD_SET` and `FD_ISSET`.
- No `.unwrap()`; the one `.expect()` with a local invariant (`socket.rs:140`, `size_of` of a sockaddr fits `socklen_t`) states its reason.
- Signed to unsigned casts are checked or commented where the value could be out of range (`read`, `write` results after the `< 0` test; `msg_controllen`, `sin_len`, `sa_family` with comments).

## Not verifiable from this machine

- Nothing Windows was executed: the Windows target is not installed here and devbox has no Windows or wine; the user's Phase 3 second pass (INTERVENTIONS.md) is the only run-time evidence and it covers connect, typing, colors, vim, UTF-8, Ctrl+C, resize and Ctrl+^ quit in Windows Terminal.
- Whether legacy conhost (a cmd.exe window, not Windows Terminal) sends 0x1E for Ctrl+6 with `ENABLE_VIRTUAL_TERMINAL_INPUT` on; Windows Terminal does, per the user's test.
- Idle CPU of `mosh-client.exe` with the 50 ms poll (20 wakeups a second, each a `WSAPoll`, a `GetStdHandle` and a `GetConsoleScreenBufferInfo`); the checklist item in INTERVENTIONS.md asking for the Task Manager number has no answer yet.
- Whether conhost renders a UTF-8 sequence split across two `WriteFile` calls correctly with code page 65001; `write_all_stdout` writes each frame in one call, so a split only happens on a short write, which the console does not produce.
- `cargo check -p mosh-platform --target x86_64-pc-windows-gnu` (target not installed; STATUS.md says the Windows clippy is clean on devbox).

## Resolution log

| Finding | Status | Commit | Note |
|---------|--------|--------|------|
| P1 | fixed | eb4c13d | the input thread starts in `raw_mode_on` (after line input is off), errors return through its `io::Result`, the client prints them and exits 1 as for `tcgetattr`; `stdin_fd` is INVALID_SOCKET and `read_stdin` an error before that; no `.expect`. Not run on Windows here: laptop checklist (INTERVENTIONS.md) |
| P2 | fixed | eb4c13d | `Select::add_signal(SIGINT)` installs a console control handler that turns Ctrl+Break (and a generated Ctrl+C event) into SIGINT, the clean shutdown upstream does for SIGINT, which restores the console; a pending flag moved into the signal set after each poll, 50 ms cap while installed; the waiting `mosh.exe` ignores the event; PLAN 7. Not run on Windows here: laptop checklist |
| P3 | fixed | eb4c13d | `[const { AtomicBool::new(false) }; N]`, const and allow removed |
| P4 | fixed | eb4c13d | PLAN 9 item 15 (13 was taken); a file that is not whole records yields no records; devbox `/run/utmp` 19200 bytes = 48 x 400, `detached_warning` passes |
| P5 | fixed | eb4c13d | 0 from `GetConsoleCP`/`GetConsoleOutputCP` is an error from `raw_mode_on`. Not run on Windows here: laptop checklist |
| P6 | fixed | eb4c13d | `ENABLE_PROCESSED_OUTPUT` ORed into the raw output mode. Not run on Windows here: laptop checklist |
| P7 | fixed | eb4c13d | no sockets and a negative timeout sleep 50 ms and return 0 (callers loop and see signals) |
| P8 | fixed | eb4c13d | the input thread prints `read: <strerror>` with the shared `perror` before ending the input. Not run on Windows here: laptop checklist |
| P9 | fixed | eb4c13d | `mosh_platform::stdio::perror` (strerror text), used at the five sites and by the client in place of its own; Windows `strerror` strips " (os error N)" |
| P10 | fixed | eb4c13d | `netdb.rs` reads `sin6_flowinfo` with native order, the raw field as `socket.rs` and std's `SocketAddrV6` keep it (the field holds network order; neither path swaps it). New test `platform_resolve`: scoped IPv6 literals (`fe80::1%1`, `fe80::1%lo`) give identical `SocketAddr`s through both files with scope id 1; a planted wrong scope byte order fails it. getaddrinfo never returns a non-zero flow label, so the flow label itself cannot be exercised this way |
| P11 | fixed | eb4c13d | `target_os = "cygwin"` added to both IUTF8 lists (libc defines `IUTF8` for cygwin, `unix/cygwin/mod.rs`); the cygwin target is not built here |
| P12 | fixed | eb4c13d | `#![deny(missing_docs)]` in `lib.rs` |
| P13 | fixed | eb4c13d | Windows `resolve` accepts `addr%index`, the zone becoming `scope_id` (numeric zones, which is what WinSock's getaddrinfo accepts). Not run on Windows here: laptop checklist |
