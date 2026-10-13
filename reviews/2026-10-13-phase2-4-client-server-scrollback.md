# Phase 2 to 4 review: client, overlay, statesync, server, scrollback and launcher against upstream mosh master (decd9b7)

Date: 2026-10-13.
Commit reviewed: 0e8dd6d (the source files are identical to 1cb0eb6; the only change between the two is INTERVENTIONS.md).
Scope: `crates/mosh/src/client/terminaloverlay.rs` against `upstream/src/frontend/terminaloverlay.{h,cc}`, `crates/mosh/src/client/stmclient.rs` and `client/mod.rs` against `stmclient.{h,cc}` and `mosh-client.cc`, `crates/mosh/src/statesync/*` against `upstream/src/statesync/*`, `crates/mosh/src/server/mod.rs` against `mosh-server.cc`, the scrollback code in those files plus the scrollback hooks in `crates/terminal` against PLAN.md section 6, and `crates/mosh/src/launcher/mod.rs` against `upstream/scripts/mosh.pl`, walked function by function.
The Phase 1 review (`reviews/2026-10-13-phase1-terminal-network.md`) covers the terminal and network crates; they were read here only where the client or the scrollback code calls into them.
Method notes are at the end of each section where a claim was checked by running code rather than by reading.

## Summary

Semantic deviations from upstream in the ported code: 0 found in the overlay, 0 in the client loop, 0 in statesync, 0 in the server; every timing constant, epoch rule, cursor and underline decision, escape-key path, resize path, shutdown path and notification string matches upstream line by line (details under "Verified correct").
Scrollback: the design in PLAN.md section 6 is implemented as written (field numbers, capture conditions, ring outside the state, equality, announcement, injection, old-peer behavior), but one gap is blocking: a server holding the session key can make the client allocate without bound from a single UDP packet (G1, measured: 1 MB of newlines in one history line costs the client 4.1 GB and 11.6 s).
Escape-sequence injection through history lines is not possible: every line is re-rendered by the client's own emulator before it reaches the host terminal (details in the scrollback section).
Unlisted deliberate differences: 2 in the launcher (G7, G8) that PLAN.md section 9 must list or the code must drop.
`#[allow(...)]`: 1 in `crates/mosh`, with the required comment.
`unsafe`: 0 in `crates/mosh`, but the crate root lacks `#![forbid(unsafe_code)]` and `#![deny(missing_docs)]` and has no crate doc comment (G4).
`.unwrap()`: 2 (launcher, both provably safe, G12); `.expect()`: 10, each with a local invariant or mirroring an upstream `fatal_assert` (judged under "Quality bar").
`cargo fmt --all --check`: passes.
`cargo clippy --workspace --all-targets -- -D warnings`: fails on this laptop's rustc 1.91.1 with 2 `non_snake_case` errors in `crates/terminal` and 10 type errors in `tests/src/bin/vt-fuzz.rs`, all of which the devbox toolchain (rustc 1.99.0) accepts; with those two lints allowed, `mosh`, `mosh-platform` and `mosh-network` are clippy-clean on 1.91.1 as well (G3).

## Blocking

### G1. One history line can make the client allocate gigabytes

`crates/mosh/src/client/stmclient.rs:151-169` (`host_rows`) feeds a received `ScrolledLines.line` into a one-row emulator with scroll capture on and keeps every row that scrolls off as an `Rc<Row>` (80 `Cell`s each), then renders all of them (`:166-168`) and `inject_history` (`:385-391`) writes all of them to the host in one synchronized frame.
PLAN.md section 6.4 says a line produces more than one row only when it is wider than the window, but nothing enforces that: a line holding `\n` bytes produces one captured row per byte, and `Complete::apply_string` (`crates/mosh/src/statesync/completeterminal.rs:694-709`) accepts any line length up to the transport's 4 MiB inflate cap.
Measured with the `host_rows` body copied verbatim into a standalone program linked against `crates/terminal` (release build, width 80): 100,000 newline bytes give 100,001 host rows, 300 KB written to the terminal, RSS from 1.9 MB to 420 MB in 0.54 s; 1,000,000 newline bytes give 1,000,001 rows, 3 MB written, RSS 4.08 GB in 11.6 s; 4,000,000 letters (the wrap case the design expects) give 50,000 rows and 326 MB.
A 1 MB run of newlines deflates to about 1 KB, so this is one UDP packet from a peer that holds the key, and the 4 MiB cap allows four times the measured 4 GB, which kills the client through the OOM killer.
Upstream has nothing comparable: a 4 MiB `hostbytes` diff of newlines only scrolls the client's fixed-size framebuffer.
Only clients started with `MOSH_SCROLLBACK=1` are exposed.
Fix, both parts: in `host_rows`, stop feeding the line once the captured rows reach a bound (the server's own row renderer never emits a line feed, so a legitimate line wraps into at most `ceil(server_width / host_width) + 1` rows; a constant such as 64, or that formula using the current frame width, is safe) and replace the rest with the existing dim notice line; in `Complete::apply_string`, treat a `line` longer than a fixed limit (for example 256 KiB, far above any row the server renderer can produce for a 65535-column terminal with 16 KiB OSC 8 links per cell being the only way to get close) as `dos_detected`, as PLAN 9.8 does for other malformed input from a keyed peer.
Record the bound in PLAN.md section 6.4.

## Should fix

### G2. Scrollback line numbers from the server are added without overflow protection

`crates/mosh/src/statesync/completeterminal.rs:303-317` (`ReceivedScrollback::receive`) computes `index + 1` and `index += 1`, and `crates/mosh/src/client/stmclient.rs:368, 375` (`inject_history`) computes `index + 1` and `self.next_line += n`, where `index` comes from the server's `first_index` (`:697`) which is only checked to be non-zero and not behind `scrolled_total`.
`first_index = u64::MAX` wraps these to 0 in release (the workspace release profile has no `overflow-checks`) and panics in debug and test builds.
After a wrap the client silently drops every later line as "already received".
`end` on `:698` is already computed with `saturating_add`; do the same in the four places above, or reject `first_index` above some sane bound in `apply_string` (a session cannot produce 2^63 lines).

### G3. The workspace compiles only on a newer stable than it declares

No `rust-toolchain.toml` and no `rust-version` field exist, and CI uses `dtolnay/rust-toolchain@stable`, so the required compiler is whatever is current on the day.
On rustc 1.91.1 (stable, 2025-11-07), `cargo clippy --workspace --all-targets -- -D warnings` fails: `crates/terminal/src/terminaldispatcher.rs:74, 134` (`CSI`, `OSC_string`) carry the item-level `#[allow(non_snake_case)]` the Phase 1 review asked for, but that rustc reports field names at the struct's lint level, so the allow does not apply; and `tests/src/bin/vt-fuzz.rs:203, 204, 220, 389, 400` write `&[&b";"[..], b"8", ...]`, whose unsized coercion of the later elements older rustc rejects (10 errors, E0308 and E0277).
Both pass on devbox's rustc 1.99.0.
Fix: pick one, either add `rust-toolchain.toml` with the channel the gates are run on and `rust-version` in the workspace `Cargo.toml`, or move the two allows to the struct (`#[allow(non_snake_case)]` on `DispatchRegistry` with the one-line reason, which is not a blanket allow) and write the five arrays as `let v: [&[u8]; N] = [...]`, so the workspace builds on any stable since the edition's MSRV.
Either way, say in CLAUDE.md which toolchain the quality bar is measured with.

### G4. `crates/mosh` lacks the crate-level attributes CLAUDE.md requires

`crates/mosh/src/lib.rs:30-35` has no `//!` crate doc comment, no `#![forbid(unsafe_code)]` and no `#![deny(missing_docs)]`, which `crates/terminal` and `crates/network` gained for Phase 1 findings F3 and F6.
The crate has no `unsafe` and every `pub` item is documented (`cargo clippy -p mosh --lib -- -W missing_docs` reports only "missing documentation for the crate"), so this is three lines, but without them the next edit can regress silently.

### G5. `swprintf` truncation copied where upstream's behavior is undefined

`crates/mosh/src/client/terminaloverlay.rs:324-329` (`swprintf128`) keeps 127 characters, as the comment says glibc does, and is used for `set_network_error` (`:384`), the notification bar text (`:505`), the "Nothing received" string (`stmclient.rs:496`) and the crypto message (`stmclient.rs:926`).
Upstream (`terminaloverlay.h:200-207`, `terminaloverlay.cc:202-247`, `stmclient.cc:199-201, 576-578`) calls `swprintf( tmp, 128, ... )` into an uninitialized `wchar_t tmp[128]`; when the result does not fit glibc returns -1 and does not write the terminator, so `std::wstring( tmp )` reads whatever follows the 127 characters.
Keeping exactly 127 characters is the sensible reading, but it is a decision, not a port, so it belongs in PLAN.md section 9 (one line next to item 11, which already covers the client exiting cleanly where upstream misbehaves).

### G6. `print_version` of the client prints the package string twice

`crates/mosh/src/client/mod.rs:52-62` formats `[build {PACKAGE_STRING}]`, so `mosh-client --version` prints `mosh-client (mosh-rs 0.1.0) [build mosh-rs 0.1.0]`.
`crates/mosh/src/server/mod.rs:75-77` defines `BUILD_VERSION` separately (with the same value today) and `launcher/mod.rs:115-125` hard-codes the pair.
Upstream `mosh-client.cc:76` prints `PACKAGE_STRING` and `BUILD_VERSION`, where the second is the git describe output.
Fix: one `BUILD_VERSION` in `lib.rs` (set from `git describe` by a build script, or the package version), used by all three.

### G7. The launcher prefers a sibling `mosh-client`, which PLAN does not list

`crates/mosh/src/launcher/mod.rs:375-385` (`default_client`) runs `mosh-client` next to the launcher's own executable when it exists, else `mosh-client` from `PATH`.
`mosh.pl:64` always uses `mosh-client` from `PATH`.
The sibling lookup is right for the Windows zip (INTERVENTIONS.md step 1 relies on it), but PLAN.md section 8 and section 9 do not mention it, and on unix it changes which client runs when two installations coexist.
Fix: add it to PLAN.md section 8 (and section 9 if it stays on unix), or limit it to Windows.

### G8. Launcher option parsing differs from `Getopt::Long` in four small ways, none listed

`crates/mosh/src/launcher/mod.rs:414-461` uses `lexopt`.
Compared with `mosh.pl:155-174`: `Getopt::Long` accepts any unique prefix of a long option (`--pred=never`, `--loc`), matches long options case-insensitively (`--Local`), treats `-an` as the unknown option `an` rather than `-a -n`, and accepts `--experimental-remote-ip=(local|remote|proxy)` (`mosh.pl:127-129, 195-197, 335-351, 364-376`), which the Rust rejects as an unknown option and omits from the usage text.
Also `:547-548` strips up to two trailing newlines from the color count where `chomp` (`mosh.pl:322`) strips one, so `256\n\n` yields 256 here and 0 upstream.
PLAN.md section 8 lists the flags supported "for v0.1" but says nothing about these differences in how the listed flags are parsed; add a sentence there, or port the prefix matching (a 20-line loop over the known names) since users of `mosh` have had it since 2012.

## Nice to have

### G9. `panic!` for an impossible `wcwidth` result

`crates/mosh/src/client/terminaloverlay.rs:556` panics on a width other than -1, 0, 1, 2, porting `assert( !"unexpected character width" )` at `terminaloverlay.cc:296`.
The Phase 1 review (F21) had the same site in `terminal.rs` changed to `unreachable!()` with the reason; do the same here.

### G10. `fake_proxy` relay stops on an interrupted read

`crates/mosh/src/launcher/mod.rs:330-337` (`cat`) ends the relay on any `read` error, while `mosh.pl:290-297` retries `EINTR`.
`std::io::copy` retries `ErrorKind::Interrupted` and does the same job in one call.

### G11. `resolvename` skips the canonical-name step

`crates/mosh/src/launcher/mod.rs:247-281` resolves `(host, port)` once with `to_socket_addrs`.
`mosh.pl:484-516` first tries the host as a numeric address, then asks for the canonical name (`AI_CANONNAME`) and resolves that, so for a CNAME whose target has a different record set the Perl connects to the target's addresses.
The difference is rare in practice; either port it with `getaddrinfo` from the platform crate or note it in PLAN.md section 8.

### G12. Two `unwrap()` calls without the reason CLAUDE.md asks for

`crates/mosh/src/launcher/mod.rs:220, 230` parse digit-only strings as `f64`, which cannot fail (an over-long digit string parses to infinity).
Use `expect("digits only, checked above")` so the invariant is stated, as the rule requires.

### G13. Row equality marker `Row::alternate` is now mostly redundant

`crates/terminal/src/terminalframebuffer.rs:632-636, 1399` keep the per-row `alternate` mark from the capture-on-entry design, but since PLAN 6.6 the alternate screen is a separate row set that is discarded on leave, and `scroll` already refuses to capture while the flag is on.
It still protects the case of a program that enters the alternate screen and exits without leaving it, so keeping it is defensible; say so in the field's doc comment, which still describes the old design ("rows drawn on the alternate screen never become history").

## Verified correct

These were walked line by line against upstream and found to match, so they need no second look unless the files change.

- `terminaloverlay.rs` constants: `SRTT_TRIGGER_LOW/HIGH` 20/30, `FLAG_TRIGGER_LOW/HIGH` 50/80, `GLITCH_THRESHOLD` 250, `GLITCH_REPAIR_COUNT` 10, `GLITCH_REPAIR_MININTERVAL` 150, `GLITCH_FLAG_THRESHOLD` 5000, `send_interval` initial 250, `prediction_epoch` 1, `confirmed_epoch` 0, `wait_time` 50 or `INT_MAX`, notification `server_late` 6500, `reply_late` 10000, countup 1000 then `ACK_INTERVAL` after 60 s, message expiry 1000 ms, network error expiry `ACK_INTERVAL + 100`.
- `ConditionalOverlay::reset` (`-1` as `u64::MAX`), `tentative`, `expire`; `ConditionalOverlayCell::reset`, `reset_with_orig` (inactive or unknown resets without saving), `apply` (blank-on-blank clears the flag, unknown underlines except in the last column, replacement only when the cell differs), `get_validity` (pending until `late_ack >= expiration_frame`, unknown and blank give no credit, original-contents check); `ConditionalCursorMove::get_validity` and `apply` with upstream's three asserts (origin mode can never be set on the client's framebuffer because `Display::new_frame` never emits DECOM, checked with `rg '\?6' crates/terminal/src/terminaldisplay.rs upstream/src/terminal/terminaldisplay.cc`).
- `NotificationEngine::apply`: cursor hidden on row 0, bar of `fg 7 bg 4` spaces, "contact" versus "reply" choice, the three message forms (`Last %s %s ago.`, `mosh: %ls%s`, `mosh: %ls (%s without %s.)%s`) with `human_readable_duration` (`%d %s`, `%d:%02d`, `%d:%02d:%02d`), the `(int)` truncation of the elapsed time, the grapheme loop (bold, wide, combining into the previous cell with the fallback rule, unprintable skipped), `adjust_message`, `wait_time` with wrapping `uint64_t` arithmetic.
- `PredictionEngine::cull`: reset on size change, `srtt_trigger` hysteresis gated on `!active()`, `flagging` hysteresis, glitch underline above `GLITCH_REPAIR_COUNT`, off-screen rows erased, `IncorrectOrExpired` tentative -> `kill_epoch(tentative_until_epoch)` else `reset()` and return (both `Experimental` variants), `Correct` confirms the epoch, glitch repair with `last_quick_confirmation`, renditions of the rest of the row copied from the actual cell inclusive of the confirmed cell, fallthrough to `reset()`, `Pending` glitch thresholds (`2 * GLITCH_REPAIR_COUNT` and `GLITCH_REPAIR_COUNT`), cursor validity check with `reset()` and return, non-pending cursors erased.
- `kill_epoch` (cursors tentative at `epoch - 1` erased, new cursor at the frame's cursor with the current `prediction_epoch`, cells tentative at `epoch - 1` reset, then `become_tentative`), `init_cursor` both branches, `get_or_make_row` (cells constructed with expiration 0 and the current epoch), `become_tentative` (no increment in `Experimental`), `active`, `apply` (display-preference gate, cursors then rows), `reset`.
- `new_user_byte`: `Never` returns, `Experimental` sets `prediction_epoch = confirmed_epoch`, `cull` first, `ESC O` -> `ESC [` translation, `Print` with backspace (`predict_overwrite` single-cell path reading the cursor cell via `get_cell( -1, -1 )`, and the shift-left loop reading `i + 1` before writing `i`, unknown when `i + 2 >= width`), control or non-width-1 characters -> tentative, last column -> tentative, insert shift-right loop from `rightmost_column` down to `col + 1` reading `i - 1` before writing `i`, unknown in the last column, renditions from the drawing state then from the left neighbour, `original_contents` push, cursor expire and wrap into `newline_carriage_return`; `Execute` CR -> newline else tentative; `Esc_Dispatch` tentative; `CSI_Dispatch` `C`/`D` moving within `[0, width - 1]` else tentative; other action types ignored.
- `newline_carriage_return`: column 0, on the last row every cell of the row made active, tentative at the current epoch, expiring at `local_frame_sent + 1` and cleared (without touching `unknown` or `original_contents`, as upstream), else row + 1.
- `TitleEngine`, `OverlayManager::apply` order (cull, predictions, adjust_message, notifications, title), `wait_time` minimum.
- `stmclient.rs` constructor: prediction mode names and the exit on an unknown one, `MOSH_PREDICTION_OVERWRITE=yes`, `display( true )`; `init`: locale check and `system( "locale" )`, raw mode with `IUTF8` and `cfmakeraw` (in `crates/platform/src/tty.rs:111-131`), `display.open()`, `MOSH_TITLE_NOPREFIX`, the whole `MOSH_ESCAPE_KEY` table (single byte 1..127, `@`-shifted pass key for control characters, lower-case alternative for `A..Z`, empty disables with `-1`, anything else resets to `Ctrl-^`), the forbidden set `03 04 0A 0C 0D`, `escape_requires_lf` for printable escape keys, the help text and `set_escape_key_string( "Ctrl-^" )`, the "Nothing received from server on UDP port" string.
- `shutdown`: empty notification, `server_heard( now )`, empty title prefix, last frame, `display.close()`, termios restore, the two diagnostics with their exact text and conditions (`still_connecting`, `network && !clean_shutdown`).
- `main_init`: six signals, `TIOCGWINSZ` (early return where upstream dereferences null, PLAN 9.11), framebuffers, initial frame with `new_frame( false, fb, fb )`, `set_send_delay( 1 )`, initial `Resize`, verbosity.
- `output_new_frame`, `process_network_input` (`server_heard`, `server_acked`, `set_local_frame_acked`, `set_send_interval`, `set_local_frame_late_acked` from the echo ack), `process_user_input` (16384-byte reads, EOF and error -> false, shutdown-in-progress swallow, `set_local_frame_sent`, paste threshold `> 100` resets predictions, per-byte prediction, quit sequence: `.` with the "Exiting on user request..." permanent message and `start_shutdown`, `0x1a` suspend with `display.close()`, termios restore, the `[mosh is suspended.]` line, `kill( 0, SIGSTOP )` (`tty.rs:43-49`) and `resume`, pass keys sending the escape key once, anything else sending escape key plus byte, help cleared when it is the current message, `quit_sequence_started` rule with `lf_entered` and `escape_requires_lf`, help shown permanent without the quit hint, `lf_entered` on `0A`/`0D`, `0C` requests a repaint), `process_resize` (`TIOCGWINSZ` failure -> `main` returns false, `Resize` pushed unless shutting down, predictions reset), `resume` (raw termios, `display.open()`, repaint).
- Main loop: `wait_time` minimum with 250 ms while connecting, fd list rebuilt each pass, `select` failure -> break, one `recv` per pass, user EOF -> break or "Exiting..." and `start_shutdown`, `SIGWINCH` -> resize, `SIGCONT` -> resume, `SIGTERM`/`SIGINT`/`SIGHUP`/`SIGPIPE` -> "Signal received, shutting down..." (upstream's `Select::signal` consumes the flag, `select.h:196-205`, as the Rust does), the three shutdown exits with `clean_shutdown` set in two of them, "Timed out waiting for server..." after 15000 ms and the connecting notification after 250 ms, clearing it once a state arrived, `tick`, send-error -> `set_network_error` else `clear_network_error`, `NetworkException` -> error message unless shutting down, 200 ms sleep and `freeze_timestamp`, `CryptoException` -> rethrow if fatal else notification.
- `client/mod.rs`: `--help`/`--version` first, `getopt( "#:cv" )` with permutation (`-#` eats its argument, `-c` prints the color count and exits inside the loop, `-v` counts), exactly two operands, digit-only port check with the `Bad UDP port` text, `MOSH_KEY` required and removed from the environment, `MOSH_PREDICTION_DISPLAY` and `MOSH_PREDICTION_OVERWRITE` optional, `set_native_locale`, `init`, `main`, `shutdown` also on error, the `Network exception:` and `Crypto exception:` lines with `\r\n`, `[mosh is exiting.]`, exit status `!success`.
- `statesync/user.rs`: `UserEvent` placeholders (`UserByte( 0 )`, `Resize( -1, -1 )` as `usize::MAX`), equality over all fields, `subtract` asserts, `subtract_self` clears, `diff_from` prefix asserts and keystroke coalescing into the last `Keystroke` instruction, resize `size_t -> int32`, `init_diff`, `apply_string` arm order (keystroke, resize, then the mosh-rs capability), `reset_input` no-op (upstream never instantiates it for `UserStream`), `compare` false.
- `statesync/completeterminal.rs` upstream part: `act` byte loop and `read_octets_to_host`, `act( Action )`, `diff_from` (echo ack first with the `>=` assert, resize before host bytes only when the size changed, empty `new_frame` string omitted), `init_diff` against a blank terminal of the same size, `apply_string` arm order (hostbytes, resize, echoack) with the three upstream asserts turned into `dos_detected` errors (PLAN 9.8), equality (`terminal`, `echo_ack`), `set_echo_ack` (`now - ECHO_TIMEOUT` wrapping, erase below the newest ack, change flag), `register_input_frame`, `wait_time` (second entry plus `ECHO_TIMEOUT`, `int` truncation), `compare` output text, `ECHO_TIMEOUT` 50.
- `server/mod.rs`: `--help`/`-h`/`--version`/`--` scan, `new` syntax with `getopt( "@:i:p:c:svl:" )` including glibc's permutation and `POSIXLY_CORRECT`, legacy 1/2/3-argument forms, `parse_portrange` check text, shell lookup (`SHELL`, `getpwuid`, empty -> `/bin/sh`, `-` prefix, `with_motd`), locale re-check with `putenv` semantics (no `=` removes the variable) and the four-line diagnostic, `MOSH_SERVER_NETWORK_TMOUT`/`MOSH_SERVER_SIGNAL_TMOUT` with upstream's exact `strtol` quirks (invalid value reported but kept, negative reset to 0), window size defaults 80x24 on failure or zero, `\r\n` bodge on a tty, `MOSH CONNECT %s %s`, `SIGHUP`/`SIGPIPE` ignored, fork with the parent's banner on stderr, `[mosh-server detached, pid = %d]`, `tcdrain`, `/dev/null` redirection when not verbose, `utmp_entry` text, `pipe` and `forkpty` with the window size, child: signals to default, network dropped, `IUTF8`, `TERM` xterm or xterm-256color on exactly 256 colors, `NCURSES_NO_UTF8_ACS`, `STY` removed, `chdir_homedir` with `PWD`, motd files in order and `.hushlogin`, `warn_unattached` text for one and for several sessions with `device_exists`, wait on the pipe with `EINTR` retry and `err( 1, "parent signal" )`, core dumps re-enabled, `execvp` failure `warn`, 3 s sleep, exit 1; parent: pipe close, `serve`, exception text, master close, `[mosh-server is exiting.]`.
- `serve`: `SIGTERM`/`SIGINT`/`SIGUSR1`, timeout minimum with 5000 ms while unconnected or shutting down and the `network_timeout_ms` sleep clamp (`INT_MAX`), one network fd asserted, host fd only when not shutting down, `select` failure -> break, `recv`, new remote state -> `UserStream::apply_string` on `get_remote_diff`, only the last of consecutive `Resize` actions applied with `TIOCGWINSZ`/`TIOCSWINSZ` (failures `start_shutdown`), `register_input_frame` when the stream is non-empty, `set_current_state` unless shutting down, child released by closing the pipe once, host read of 16384 bytes with `<= 0` -> `start_shutdown`, `swrite` to the host, idle and `SIGUSR1` timeouts with their messages, `any_signal` -> `start_shutdown` or break, the three shutdown exits, `set_echo_ack` -> `set_current_state`, 60 s no-client exit text, `tick`, `NetworkException` -> message and `spin()` (0.1 s after 10 spins), fatal crypto rethrown.
- Launcher: usage text (plus `--scrollback`, minus the unsupported `--bind-server=IP` and `--experimental-remote-ip`), version text, `predict_check` message with the environment explanation, `MOSH_PREDICTION_DISPLAY` fallback and removal before the fork, `MOSH_PREDICTION_OVERWRITE=yes`, `-p` validation messages and limits (`0..65535`, high `1..65535`, no range from 0, low > high), `shellwords` (single quotes, double quotes with backslash escapes, bare backslash, empty quoted word), `shell_quote`, `locale_vars` list, `bind_arguments` (`-s`, or `-i host` with `--local`, nothing for `any`), color count via `mosh-client -c` with non-digits -> 0, `MOSH_CLIENT_PID`, server argument order (`new`, `-c`, bind, `-p`, `-l` pairs, `--` command), `--local` through `sh -c` with `SSH_CONNECTION` removed and `chdir $HOME`, ssh argument order (`-n`, `-tt`, `-S none -o ProxyCommand=... --fake-proxy -- %h %p`, host, `--`, command) with `SHELL=/bin/sh`, merged stdout and stderr pipe, `MOSH IP` redefinition check, `MOSH CONNECT` regex (`\d+` then exactly one space then 22 base64 characters then optional whitespace), `Bad UDP port` detection only with a port range, pass-through of other lines, exit codes 255 and 10 matched against real mosh, client exec with `-# cmdline |`, `MOSH_KEY`, `MOSH_PREDICTION_DISPLAY`, `MOSH_NO_TERM_INIT`.
- `fake_proxy`: family filter and `prefer-*` reorder (stable sort), dual-stack refusal for `auto`, numeric host with IPv6 scope, first address that accepts TCP, `MOSH IP` on stderr, bidirectional relay with half-close in each direction and `SIGHUP` ignored, exit 0.
- The Windows ProxyCommand quoting failure found by the laptop on 2026-10-13 (INTERVENTIONS.md, commit 0e8dd6d) is already logged with its fix and is not repeated here.

## Scrollback against PLAN.md section 6

Checked item by item; everything below matches the plan and the two logged decisions (windowed history, INTERVENTIONS.md "history is windowed"; alternate screen forwarded, STATUS.md "PLAN 6.6").

- 6.1 Negotiation: `ClientBuffers::Instruction` field 4 `ClientCapabilities { scrollback = 1 }` (`crates/network/src/proto.rs:98-107`), pushed as the first event before the initial `Resize` (`stmclient.rs:645-654`), applied after upstream's arms (`user.rs:284-292`), turns capture on once and only when the server's limit is above 0 (`completeterminal.rs:468-477`); the encoding `0a 04 22 02 08 01` follows from the tags.
- 6.2 Capture: only when `scrolled_rows` is `Some`, the region is the full screen and `alternate_screen` is off (`terminalframebuffer.rs:1393-1400`); `capture_screen` on ED 2, or ED 0 from the home position, with a full-screen region (`terminalfunctions.rs:82-88`) and never on the alternate screen (`:1336`); DEC 47/1047/1049 switch a real second screen only while capturing (`:1356-1365`, `:1369-1379`), otherwise only the flag is set (`terminalfunctions.rs:313`); the saved rows follow resizes (`:1652, 1675`) and RIS drops them (`:1586`); `Row::alternate` is not part of `Row` equality (`:640-644`); `Cell` equality matches upstream including `wrap` (`terminalframebuffer.h:150`).
- 6.2 Ring: `ScrolledRing` is shared by `Rc` and holds only undelivered lines (`publish` frees entries at or below `acked`), capped in bytes with a dropped run replacing the oldest unpublished lines and a sixteenth of slack (`completeterminal.rs:115-179`), `--scrollback MIB` with `K` suffix, default 32 MiB, 0 disables (`server/mod.rs:252, 279-349`), `Complete::act` renders every captured row with `Display::render_scrolled_row` (`completeterminal.rs:431-443`), `apply_string` discards rows a diff scrolls (`:693`), the server never stops reading the pty, and after host EOF shutdown waits for the backlog or 60 s without ack progress (`server/mod.rs:1017-1023, 1068-1077`); with no capability the ring is `None` and all of this is inert.
- 6.3 Wire: `HostBuffers::Instruction` field 9 `ScrolledLines { first_index = 1, line = 2, dropped = 3 }` and field 10 `ScreenMode { alternate_screen = 1 }` (`proto.rs:153-190`); instruction order echo ack, scrolled lines, screen mode, resize, host bytes (`completeterminal.rs:545-632`); `scrolled_total` counts published lines and is always part of equality, `scrollback_on` is part of equality, the alternate flag only in scrollback mode (`:638-655`); the announcement instruction with no line when none is due (`:580-592`) and `init_diff` sending only the count (`:535-544, 668-677`); `W = 4 * 1214` (`:102`) and `publish` admitting at least one line when nothing is outstanding and dropped runs whole (`:202-220`); `get_sent_state_acked_state` is read-only and returns the front sent state (`transportsender.rs:549-551`); the publish points are after host output and after every received packet (`server/mod.rs:906-913, 1027`); the client queue skips numbers it has and counts jumps as lost (`completeterminal.rs:274-288`).
- 6.4 Client: `MOSH_SCROLLBACK=1` exactly (`client/mod.rs:212`); start in the alternate screen as upstream; decision on `get_remote_state_num() != 0` with `close()`, `set_host_history()`, `open()` and one CR LF per row (`stmclient.rs:302-322`); host-history mode has no `smcup`/`rmcup` (`terminaldisplay.rs:201-205`), erases with `CSI H CSI K CSI 2;1H CSI J CSI H` (`:296-298`) and disables the scroll shortcut (`:347`); frames of another size than the window are held (`stmclient.rs:716-723`); injection is `CSI H`, row, `CSI <rows>;1H`, LF per row after hiding the cursor and resetting renditions and the hyperlink (`:410-419`), through a one-row emulator of the host's width (`:151-169`), with the dim notice line written directly (`:384-388`), then `host_scrolled` (`terminalframebuffer.rs:1298-1311`); the whole render is in `CSI ?2026h ... CSI ?2026l` (`:115-124`); alternate screen forwarded with the line ordering of 6.6 (`:691-712`); exit leaves the final screen and parks the cursor below the text (`:544-556, 586-600`); suspend erases without entering history and resume pushes the local shell's output into history (`:813-827, 307-313`).
- 6.5 Compatibility: no announcement -> `Scrollback::Off`, the 5 s notice without the quit hint (`:303-311, 971-983`), nothing sent or drawn differently afterwards.
- Hostile row strings: a line never reaches the host unparsed, because `host_rows` runs it through mosh's own parser and emulator and emits only what `render_scrolled_row` produces (SGR, OSC 8 whose URI the OSC parser has already stripped of C0 and escape bytes, `CSI K`, `CSI 1;<col>H`, printable cells); a newline or cursor movement inside a line therefore cannot leave the injection procedure, it can only add rows, which is G1.

Method note: the allocation numbers in G1 come from a 40-line program in the scratchpad that copies `host_rows` verbatim and links `crates/terminal` (release profile), reading `VmRSS` before and after one call.

## Quality bar

`rg -n '#\[allow|unsafe|unwrap\(\)|expect\(' crates/mosh` gives 14 hits.
- `server/mod.rs:550` `#[allow(clippy::too_many_arguments)]` has the one-line reason above it: acceptable.
- `server/mod.rs:603, 604, 670, 672` `.expect("fatal_assert: sigaction( ... )")` port `fatal_assert` on `sigaction` with valid arguments, which cannot fail at runtime: acceptable.
- `server/mod.rs:912` `.expect("one fd")` follows `assert!(fd_list.len() == 1)` as upstream: acceptable.
- `statesync/completeterminal.rs:173` `.expect("entry")` after `p + 1 < self.entries.len()`: acceptable.
- `client/terminaloverlay.rs:666` `.expect("cursors not empty")` ports upstream's `assert( !cursors.empty() )`; every caller either checks `is_empty()` or has just pushed: acceptable.
- `client/stmclient.rs:676, 694, 742, 759` `.expect("network initialized")`: `network` is set at the end of `main_init` before the loop runs and `output_new_frame` returns early on `None`: acceptable, though a single `fn net()` already exists and the two in `output_new_frame` could use the binding taken on `:636`.
- `launcher/mod.rs:220, 230` `.unwrap()`: G12.
- `unsafe`: none in `crates/mosh`; the crate lacks the `forbid` attribute (G4).

## Resolution log

| Finding | Status | Commit | Note |
|---------|--------|--------|------|
| G3 | fixed | 077968f | portable code, not a pinned toolchain: struct-level `non_snake_case` allows (with reason) in terminaldispatcher.rs, typed byte-string consts in vt-fuzz; `rust-version = "1.89"` (aes 0.9.3 needs it; 1.88.0 refused); clippy -D warnings and `test --release --workspace --no-run` pass on 1.89.0, 1.91.1 and stable 1.99; CLAUDE.md states it |
