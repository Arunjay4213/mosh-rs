# Phase 1 review: crates/terminal and crates/network against upstream mosh 1.4.0

Date: 2026-10-13.
Commit reviewed: 451b2ce.
Scope: every file in `crates/terminal/src` against `upstream/src/terminal`, and every file in `crates/network/src` against `upstream/src/crypto` and `upstream/src/network`, walked function by function.
Tests, `ref/` and `tools/` were read only as far as needed to judge the oracle wiring.
Method notes are at the end of each section where a claim was checked by running code rather than by reading.

## Summary

Semantic deviations: 1 confirmed and observable (F1, UTF-8 decoding of 0xF8-0xFB lead bytes, already caught by the fuzz oracle), plus 3 intentional behavior differences that are explained in code comments but not listed in PLAN.md section 9 (F14, F15, F16).
Panics reachable from a received UDP packet: 0 from an unauthenticated packet, 6 sites from a peer holding the session key (F9 to F13), every one of them a place where upstream also aborts via `fatal_assert` or `assert`.
`#[allow(...)]` without the required one-line justification above it: 57 item-level allows (56 with no comment, 1 with a same-line comment) plus 3 crate-level blanket allows, all in `crates/terminal` except one in `crates/network/src/crypto/mod.rs`.
`unsafe` outside `crates/platform`: 20 blocks in `crates/network` (2 in `crypto/mod.rs`, 18 in `transport/network.rs`), and neither crate carries `#![forbid(unsafe_code)]`; `crates/terminal` has 0.
`cargo fmt --all --check` fails (255 hunks in 36 files, 111 of them in these two crates); `cargo clippy --workspace --all-targets -- -D warnings` fails on F1 and stops there; `cargo clippy -W clippy::pedantic` reports 243 warnings in `mosh-terminal` and 100 or so in `mosh-network` (counts below); 486 `pub` items lack doc comments; `vt-fuzz` does not compile at HEAD (F8).

## Blocking

### F1. 5-byte UTF-8 lead bytes are rejected where glibc accepts them

`crates/terminal/src/parser.rs:169` ports glibc `mbrtowc()` as called from `upstream/src/terminal/parser.cc:99`.
The test `(c & 0xFC) == 0xF9` can never be true (clippy `bad_bit_mask`), so lead bytes 0xF8 to 0xFB fall through to `Invalid`, while glibc 2.39 treats them as the start of a 5-byte form.
Observable effect, taken from the recorded fuzz failures `tests/corpus/fuzz-failures/3-4003.bin` and four identical ones: input byte 0xF8 alone makes upstream buffer it (incomplete), while the Rust prints U+FFFD and advances the cursor.
Fix: change the condition to `(c & 0xFC) == 0xF8`.
Verification: I compiled the Rust `mbrtowc` body as a standalone program and compared it with glibc 2.39 `mbrtowc` in `C.UTF-8` on every 1-, 2- and 3-byte sequence (16.8 million inputs) and on 4- to 6-byte sequences with leads 0xE0 to 0xFF and 13 sampled continuation values (4.5 million inputs); with the current mask there are 87,060 mismatches, all for leads 0xF8 to 0xFB, and with `0xF8` the two outputs are byte-identical.

### F2. Blanket crate-level allows, and 57 item-level allows without justification

`crates/terminal/src/lib.rs:38-40` carries `#![allow(non_snake_case)]`, `#![allow(non_camel_case_types)]`, `#![allow(non_upper_case_globals)]`, which CLAUDE.md forbids ("blanket allows at module or crate level are forbidden").
In addition 57 item-level allows exist: `terminalfunctions.rs` (41), `terminalframebuffer.rs` (5), `terminal.rs` (3), `parserstate.rs` (3), `terminaldispatcher.rs` (2), `parseraction.rs` (2), `crypto/mod.rs:335` (1, comment on the same line instead of the line above).
None of the 56 naming allows has the required comment, and with the crate-level allows in place they are also redundant.
Fix: pick one of two options and record it. Either delete the three crate-level lines and give every item-level allow a one-line reason above it (for example `// upstream name: Terminal::CSI_EL`), or amend CLAUDE.md to permit exactly the three naming lints at crate level in crates that mirror upstream identifiers and then delete all 57 item-level allows.

### F3. `unsafe` and platform code live in `crates/network`

CLAUDE.md allows `unsafe` only in `crates/platform`, and PLAN.md section 2 says `crates/platform` is the only crate with `#[cfg(unix)]` and `#[cfg(windows)]` code.
`crates/network/src/transport/network.rs:693-1077` is two `#[cfg]` modules named `sys` with 18 `unsafe` blocks (lines 91, 710, 725, 730, 772, 776, 796, 824, 863, 873, 883, 889, 912, 924, 925, 950, 968, 979), and `crates/network/src/crypto/mod.rs:328` and `:353` wrap `getrlimit`/`setrlimit`.
`crates/network/Cargo.toml` depends on `libc` for this.
The code itself is careful (every block has a `SAFETY:` comment and `from_sockaddr` checks the length before the cast), so this is a placement problem, not a soundness problem I could find.
Fix: move the socket layer (`new_socket`, `resolve`, `bind`, `recv_raw`, `strerror`) and `disable_dumping_core`/`reenable_dumping_core` into `crates/platform` as plain functions with the same signatures, have `Connection` call them, drop `libc` from the network crate, and add `#![forbid(unsafe_code)]` to both `crates/network/src/lib.rs` and `crates/terminal/src/lib.rs`.
The alternative is to amend PLAN.md section 2 to list UDP sockets as a platform-crate responsibility implemented inside `crates/network`; that still needs the `forbid` attribute on `crates/terminal`.

### F4. `cargo fmt --all --check` fails

`rustfmt.toml` sets `max_width = 100`, but 68 lines in the two crates exceed 100 columns and `cargo fmt --all --check` reports 111 hunks in these crates (`network.rs` 29, `terminaldisplay.rs` 12, `terminalframebuffer.rs` 12, `networktransport.rs` 11, `transportsender.rs` 9, `transportfragment.rs` 7, `terminalfunctions.rs` 6, `crypto/mod.rs` 5, `terminaldispatcher.rs` 4, `terminal.rs` 3, `terminaldisplayinit.rs` 3, `compressor.rs` 2, `parser.rs` 2, `parseraction.rs` 2, and one each in `ae.rs`, `lib.rs`, `parsertransition.rs`, `terminaluserinput.rs`).
The code was evidently formatted with a wider width.
Fix: run `cargo fmt --all` and commit, or change `rustfmt.toml` first if the wider width is wanted, then run it.

### F5. `cargo clippy --workspace --all-targets -- -D warnings` fails

The run stops at `crates/terminal/src/parser.rs:110` with the `bad_bit_mask` error from F1, so no later crate is checked.
Fix: F1, then rerun; the pedantic run (which does not stop) found no further default-level warnings in `mosh-network`, but `mosh` and `mosh-tests` were not reached by either run and must be rechecked.

### F6. 486 `pub` items have no doc comment

Counted with `cargo clippy -p mosh-terminal -p mosh-network --lib -- -W missing_docs` (184 methods, 99 struct fields, 55 variants, 39 structs, 38 associated functions, 18 modules, 11 constants, 7 type aliases, 7 functions, 6 enums, 4 associated constants).
Per file: `terminalframebuffer.rs` 175, `transport/network.rs` 32, `parseraction.rs` 29, `proto.rs` 26, `crypto/mod.rs` 26, `terminaldispatcher.rs` 25, `terminaldisplay.rs` 23, `networktransport.rs` 20, `parserstate.rs` 17, `transportfragment.rs` 17, `transportsender.rs` 16, `terminal/lib.rs` 12, `parser.rs` 9, `terminaluserinput.rs` 7, `transportstate.rs` 6, `transport/mod.rs` 6, `prng.rs` 6, `terminal.rs` 4, `ae.rs` 4, `parsertransition.rs` 2, `timestamp.rs` 2, `network/lib.rs` 2, `terminalfunctions.rs` 1, `terminaldisplayinit.rs` 1.
18 of the 27 modules also lack the `//!` module doc comment that CLAUDE.md requires (every `crates/terminal/src` file except `lib.rs`, plus `crypto/mod.rs`, `transport/mod.rs`, `network.rs`, `networktransport.rs`, `transportfragment.rs`, `transportstate.rs`); they carry only the `// Ported from:` line comment, and `terminaldisplayinit.rs` has its explanation as plain `//` lines.
Fix: add the comments; the upstream header comments can be carried over nearly verbatim for most items, and `#![warn(missing_docs)]` in both `lib.rs` files keeps it from regressing.

### F7. Many `pub` items that upstream keeps private

`Framebuffer`, `DrawState`, `Cell`, `Row`, `Renditions`, `Hyperlink` expose every field as `pub` (`terminalframebuffer.rs:58-65, 216-226, 256-265, 408-414, 503-539, 851-861`), while upstream keeps them private behind accessors that the Rust also provides.
`Dispatcher::terminal_to_host` is `pub` as upstream has it, but `OSC_string` is `pub(crate)` for `OSC_dispatch`, which is fine.
This is a correctness risk for Phase 2 because `Framebuffer::operator==` compares row pointers (correctly ported at `terminalframebuffer.rs:863-873`), so any outside code that rebuilds `rows` from cells silently breaks state equality.
Fix: make the fields private (or `pub(crate)`) and keep the accessor methods, matching the upstream headers; `vt-check` reads private state for its dump, so give it a `pub fn debug_*` on `Framebuffer`/`DrawState` instead, which `ref-term.cc:46` does on the C++ side by a `#define private public` hack.

### F8. `vt-fuzz` does not compile, so `cargo test -p mosh-tests` cannot build

`tests/src/bin/vt-fuzz.rs:179, 180, 195, 319, 325` call `r.pick(&[&b"\x1b\\"[..], "\u{9c}".as_bytes(), b"\x07", b""])`, mixing array and slice types; rustc reports 10 errors (E0308, E0277).
Because cargo builds every bin in the package before running integration tests, `terminal_conformance` and `crypto_conformance` cannot run either, and `tests/run-all.sh` fails at `cargo test`.
Fix: write the arrays as `&[&[u8]]` values (for example `let v: [&[u8]; 4] = [b"\x1b\\", "\u{9c}".as_bytes(), b"\x07", b""]; r.pick(&v)`), or relax `Rng::pick` to `T: ?Sized` and pass `&[&[u8]]`.
This is listed as blocking because the Phase 1 gate for the terminal crate is defined by this harness.

## Should fix

### F9. Short fragment aborts the process

`crates/network/src/transport/transportfragment.rs:501` (`Fragment::from_bytes`) asserts `x.len() >= FRAG_HEADER_LEN`, porting `upstream/src/network/transportfragment.cc:355` which is a `fatal_assert`.
A peer that holds the key can send a 4- to 13-byte plaintext (passes `Packet::from_message`) and crash us; upstream dies the same way.
Fix: return `Result<Fragment, CryptoError>` using `dos_detected`, and make `Transport::recv` (`networktransport.rs:109`) propagate it; record in PLAN.md section 9 that malformed authenticated input returns an error where upstream aborts, because this is a deliberate difference.

### F10. Conflicting duplicate fragment aborts the process

`transportfragment.rs:568` asserts the resent fragment equals the stored one (upstream `transportfragment.cc:381`, a plain `assert`).
Fix: as F9, return a `dos_detected` error from `add_fragment`.

### F11. Final fragment with a lower index than one already stored aborts the process

`transportfragment.rs:580` asserts `fragments.len() <= fragments_total` and `:585` asserts `fragments_arrived <= fragments_total` (upstream `transportfragment.cc:393, 398`).
A peer sending fragment 5 (not final) and then fragment 2 (final) with the same id trips the first; fix as F9.

### F12. Undecodable protobuf after a valid zlib stream aborts the process

`transportfragment.rs:601-602` calls `Instruction::decode(...).expect(...)`, porting `fatal_assert(ret.ParseFromString(...))` at `transportfragment.cc:417`.
Fix: map the prost error to `dos_detected("ret.ParseFromString(...)", "get_assembly")` and return it; the function already returns `Result`.

### F13. `throwaway_num` above every received state aborts the process

`networktransport.rs:218` asserts `received_states` is non-empty after `retain`, porting `fatal_assert` at `networktransport-impl.h:191`.
A peer can send `throwaway_num` greater than all our received state numbers (the `old_num` lookup passes if `old_num` is lower, and the throwaway then removes it too).
Fix: return a `dos_detected` error instead of asserting; the code at `networktransport.rs:166-169` already handles the reference state vanishing, so only the assert needs to change.

### F14. Clock base offset is an unlisted deviation

`crates/network/src/timestamp.rs:181` adds `CLOCK_BASE_MS = 1_000_000_000` to the monotonic clock so that `now - 0` comparisons in `Connection::new_packet` and `TransportSender::calculate_timers` behave as they do upstream, where the clock is milliseconds since boot (`upstream/src/util/timestamp.cc`).
The reasoning is sound and written in the module doc, but PLAN.md section 9 says every deliberate difference is listed there and nowhere else.
Fix: add it to section 9, or drop the base and accept upstream's own first-minute behavior.

### F15. Reference-state re-lookup after `process_throwaway_until` is an unlisted deviation

`networktransport.rs:163-169` looks the reference state up again after throwaway processing and silently drops the instruction if it is gone; upstream `networktransport-impl.h:135` dereferences the earlier iterator, which is undefined behavior when the state was erased.
This is the right thing to do, but it is a behavior difference and belongs in PLAN.md section 9 next to F13.

### F16. `add_sent_state` index repair is an unlisted deviation

`transportsender.rs:265-270` moves `assumed_receiver_state` to the previous element when the erased middle element was the assumed one; upstream `transportsender-impl.h:220` leaves a dangling iterator in that case.
Same treatment as F15.

### F17. `State` trait omits `reset_input` and `compare`

`crates/network/src/transport/transportstate.rs:313-321` defines `diff_from`, `init_diff`, `apply_string`, `subtract` and equality, as PLAN.md section 2 specifies.
Upstream's template also calls `current_state.reset_input()` in `set_current_state` (`transportsender.h:152`) and `current_state.compare(newstate)` in verbose `tick` (`transportsender-impl.h:162`); the Rust notes both omissions in comments at `transportsender.rs:210` and `:467`.
`reset_input` has no effect on anything that is compared or sent (it clears the parser buffer of the sender's private copy), and `compare` is diagnostics only, so this is not a wire-visible deviation.
Fix: either add the two methods to the trait so Phase 2 ports `Complete` and `UserStream` line by line, or write the omission into PLAN.md section 9 so the Phase 2 agent does not reintroduce it differently.

### F18. Per-dispatch allocation in the parser hot path

`crates/terminal/src/terminaldispatcher.rs:240` clones `dispatch_chars` into a fresh `Vec<u8>` on every CSI and ESC dispatch to build the map key; upstream (`terminaldispatcher.cc:221`) copies into a `std::string` that stays in its inline buffer for these 1- to 8-byte keys, so it does not allocate.
Fix: `map.get(self.dispatch_chars.as_slice())` works directly because `BTreeMap<Vec<u8>, _>::get` accepts `&[u8]`; the `CONTROL` case can use a one-byte stack array the same way.

### F19. Whole-state clones in the transport hot path

`networktransport.rs:230` clones the oldest received state and `transportsender.rs:325` clones the known receiver state on every `recv` and every `calculate_timers`, because `subtract` needs `&mut` on the elements while the reference points into the same vector; upstream (`networktransport-impl.h:201`, `transportsender-impl.h:283`) uses a pointer to the front element.
For `Complete` in Phase 2 this clones a full framebuffer (rows are `Rc`, so it is the row vector, titles and `DrawState`) several times per packet.
Fix: `let (first, rest) = sent_states.split_first_mut()` and subtract `first.state` from `rest` and from `current_state`, then handle the front element itself, which upstream subtracts from itself as a no-op.

### F20. `Session::decrypt` asserts on lengths it can be handed

`crates/network/src/crypto/mod.rs:304-305` asserts `body_len <= RECEIVE_MTU` and `pt_len <= RECEIVE_MTU`, porting `crypto.cc:422-423`.
Inside `Connection` the receive buffer is 2048 bytes so this cannot fire, but `Session::decrypt` is `pub` and `crypto-check` feeds it arbitrary hex.
Fix: return `CryptoError::new("Ciphertext too long")` for lengths above `RECEIVE_MTU + 8`.

## Nice to have

### F21. `Emulator::print` default arm

`crates/terminal/src/terminal.rs:163` uses `panic!("unexpected character width from wcwidth()")` for widths other than -1, 0, 1, 2.
The generated table only ever yields those four values (checked, see verified list), so this is unreachable; `unreachable!()` with the same message states that.

### F22. `Rc` makes `Framebuffer` and `Emulator` `!Send`

`terminalframebuffer.rs:225, 843` use `Rc` for hyperlink reps and rows, matching upstream `shared_ptr` sharing.
That is correct and cheapest for a single-threaded loop; note it in the type docs so Phase 2 does not try to move a `Complete` across the platform crate's reader thread.

### F23. Key material is not zeroed on drop

Upstream `Session::~Session` calls `ae_clear` (`crypto.cc:329`); `crates/network/src/crypto/mod.rs:285-289` holds `Base64Key` and the `ocb3` context with no `Drop`.
`ocb3`'s `Ocb3` zeroizes its own state on drop when the `zeroize` feature is on; enabling it and zeroing `key` in a `Drop` impl matches upstream.

### F24. Dead and single-caller items

`crates/network/src/crypto/mod.rs:163` `myatoi` has no caller in the workspace yet (upstream uses it in the frontends); fine to keep but mark it in the doc as "used by crates/mosh".
`crates/terminal/src/terminaldisplay.rs:85` `append_string` and `:82` `append` are the same function; upstream has both overloads only because of `const char*` versus `std::string`.
`crates/terminal/src/parseraction.rs:78, 131, 153` `name()` has no caller; upstream uses it only in debugging output.

### F25. `Display::new` TERM table

`crates/terminal/src/terminaldisplayinit.rs:49-65` implements PLAN.md section 9 item 6 with two classes (xterm, screen/tmux).
This is allowed, but `rxvt` (`smcup=\E7\E[?47h`), `linux` and `vt100` (no `smcup`) get xterm's strings; if those terminals matter, add rows, and say in the doc comment which `infocmp` output each row was copied from.

### F26. Pedantic clippy counts

`cargo clippy --workspace --all-targets -- -W clippy::pedantic`, warnings by kind, workspace-wide, excluding `crates/mosh` and `mosh-tests` which were not reached (see F5): 103 `must_use` candidates, 63 `i32 -> usize` sign-loss casts, 43 `Result` functions without an `# Errors` section, 36 functions that may panic without a `# Panics` section, 27 implicit raw-pointer borrows, 23 doc items missing backticks, 19 `i32 -> u32` casts, 13 `u64 -> i32` truncations, 11 `usize -> i32` casts, 10 `u32 -> u8` truncations, 10 long literals without separators, and a tail of 1 to 9 each.
The cast warnings are the ones CLAUDE.md asks to annotate; the `as` casts at `terminalframebuffer.rs:674, 678, 690, 702, 939, 951, 959, 970` index with `i32` values that the border snapping keeps in range, and a one-line comment at each site (or a private `fn idx(i: i32) -> usize`) would satisfy the rule.

## Verified correct

These were walked line by line against upstream and found to match, so they need no second look unless the files change.

- `parserstate.rs` against `parserstate.cc`: every range in `anywhere_rule`, `C0_prime`, `GLGR` and all fourteen `input_state_rule` bodies, `enter`/`exit` actions, and the `ch >= 0xA0 -> 0x41` substitution.
- `parseraction.rs`, `parsertransition.rs`: the flattening of the class hierarchy keeps `ch = -1` (`u32::MAX`) and `char_present = false` defaults and the `ignore()` filtering.
- `parser.rs` `UTF8Parser::input` control flow (NUL, EILSEQ retry with last byte, incomplete, surrogate and > 0x10FFFF replacement) and `BUF_SIZE` bound; only the F1 mask is wrong.
- `terminal.rs` `print()` including the copy-on-write side effect of the discarded `get_mutable_cell`, wrap and wide-wrap branches, insert mode, combining-cell fallback, and `Esc_dispatch` 7-bit C1 mapping.
- `terminaldispatcher.rs`: `parse_params` is equivalent to the `strtol` loop including empty segments, `> PARAM_MAX` and `ERANGE` giving -1, `getparam` `< 1` defaulting, 100-byte param cap, 8-byte dispatch cap, 16 KiB OSC cap, `snprintf` 63-byte truncation in `str()`, and first-wins registration order.
- `terminalfunctions.rs`: all 47 registrations with the same keys and `clears_wrap_state` flags as the 47 static `Function` objects, and every handler body (`CSI_SGR` 38/48;5 and 38/48;2 lookahead, `DECSTBM` validity test, `ECH` limit, `HT_n` wrap-state preservation, `TBC`, DEC and ANSI modes, `OSC_dispatch` 52;c; clipboard, title length cap 256, OSC 8 parsing).
- `terminalframebuffer.rs`: 25-bit color truncation at the `set_*_color` entry points, `set_rendition` table, `sgr()` output, `Hyperlink` equality, `Row::gen` counter, `insert_cell`/`delete_cell`, `insert_line`/`delete_line` index arithmetic, `resize()` stopping at the first blank row by pointer identity, `DrawState::resize` tab reinitialisation from the old width, `get_next_tab` `i > 0` lower bound, `Framebuffer::operator==` by row pointer.
- `terminaldisplay.rs` `new_frame` and `put_row`: scroll detection, `append_move` CR/LF and backspace shortcuts, ECH threshold `> 4`, `can_use_erase`, wrap handling, mouse and encoding mode strings.
- `terminaluserinput.rs`: the three-state SS3 translation.
- `wcwidth.rs`: compared against glibc 2.39 `wcwidth()` in `C.UTF-8` for every code point 0 to 0x10FFFF, 0 mismatches.
- `crypto/base64.rs`, `crypto/mod.rs` (`Base64Key`, `Nonce`, `Session::encrypt` block counting and 2^47 limit, `unique()`), `crypto/ae.rs` (12-byte nonce, 16-byte tag appended), `prng.rs`.
- `proto.rs`: every field number and type against the three `.proto` files, including the extension fields encoded as plain fields.
- `transport/compressor.rs`: 4 MiB output cap on both directions, trailing bytes ignored as `uncompress()` does.
- `transportfragment.rs` `tostring`, `from_bytes` bit layout, `Fragmenter::make_fragments` id bumping and `last_mtu`; `FragmentAssembly` allocation is bounded by the 15-bit fragment number (at most 32768 entries, 64 MiB of contents before the 4 MiB inflate cap).
- `transport/network.rs` `Packet` layout and masks, `new_packet` 16-bit timestamp arithmetic, `prune_sockets`, `send` MTU fallback and port hopping, `recv_one` replay check, ECN penalty, RTT estimator, server roaming, `timeout()`, `parse_portrange` including the `strtol` edge cases.
- `transportsender.rs` `calculate_timers`, `wait_time`, `tick`, `send_empty_ack`, `send_to_receiver`, `update_assumed_receiver_state`, `process_acknowledgment_through`, `attempt_prospective_resend_optimization` (size_t wraparound mirrored with `wrapping_sub`), `make_chaff`, `shutdown_ack_timed_out`.
- `networktransport.rs` `recv` ordering (version check, ack, roundtrip success, duplicate new_num, missing old_num, throwaway, quench at 1024 states, sorted insert) and `get_remote_diff`.

Packet trace for the question in the brief, socket read to `Instruction`: `sys::recv_raw` (2048-byte stack buffer, `MSG_TRUNC` rejected) -> `Session::decrypt` (length < 24 rejected, tag failure rejected, no panic) -> `Packet::from_message` (< 4 bytes rejected) -> direction and replay checks -> `Fragment::from_bytes` (F9) -> `FragmentAssembly::add_fragment` (F10, F11; allocation bounded by 15-bit index) -> `get_assembly` (concatenation bounded by 32768 x 2048 bytes; `uncompress_str` bounded to 4 MiB; F12) -> `protocol_version` check -> `process_throwaway_until` (F13) -> `apply_string` (Phase 2 code).
Every allocation on that path has a fixed bound and nothing panics before authentication succeeds.

## Oracle wiring

What is in place:

- `ref/ref-term/ref-term.cc` links `libmoshterminal.a` and `libmoshutil.a` and offers `dump`, `batch`, `frame` and `frame-batch` modes; `tests/src/bin/vt-check.rs` offers the identical four modes and output format.
- `tests/tests/terminal_conformance.rs` runs `make -C ref/ref-term`, then for each of the 18 `tests/corpus/emulation/*.bin` inputs and the 10 `tests/corpus/sessions/*.bin` recordings, at 80x24 and 132x43, compares `dump` output byte for byte and compares `frame-batch` output over 40 pseudo-random prefix pairs per file plus one uninitialized frame.
  It asserts on failure and does not skip when the oracle is missing.
- The session corpus listed in PLAN.md section 3 item 3 is committed (bash colors, vim, less, htop, tmux inside, Claude TUI, seq 100000, 256-color and truecolor, wide and emoji, resize), 752 KB in total, with `.meta` files and re-recording scripts.
- `tests/src/bin/vt-fuzz.rs` is the differential fuzzer of section 3 item 2: a generator biased to CSI, SGR, OSC, DEC modes, UTF-8 including wide and combining characters, invalid and overlong sequences and surrogates (lines 265-299), batch runs through `ref-term batch` and `vt-check batch`, minimization, and failure capture to `tests/corpus/fuzz-failures/`.
  Its five saved failures are all F1.
- `ref/ref-crypto/ref-crypto.cc` links `libmoshnetwork.a`, `libmoshcrypto.a`, `libmoshutil.a` plus upstream's own `ocb-aes.cc`; `tests/src/bin/crypto-check.rs` is the Rust twin with the same CLI.
- `tests/tests/crypto_conformance.rs` runs `make -C ref/ref-crypto`, checks `vectors` on both, and runs random `enc`/`dec` cases in both directions with a seed.
- `tests/tests/transport_roundtrip.rs` fixes the `Instruction` wire format against literal hex, and round-trips the compressor and the fragmenter; `tests/tests/transport_sync.rs` runs two `toy-sync` processes and accepts `MOSH_IMPAIR_PROXY` so the orchestrator can insert `udp-impair`.

What is missing or broken:

- `vt-fuzz` does not compile (F8), so none of the `mosh-tests` integration tests can run at HEAD, and `tests/run-all.sh` fails.
- The oracles need the built upstream tree; this clone's `upstream/` has no `.a` files, so the conformance tests can only run on devbox.
  I could not run them here; the results claimed for the gate must come from a devbox run pasted into STATUS.md.
- There is no record in the repo of the one-million-case fuzz run required by the Phase 1 gate, and the five saved failures show the last recorded run (seed 3) failed.
- `transport_sync.rs` only exercises loss and address change when the orchestrator supplies the proxy environment; there is no committed driver script under `tests/*.sh` that sets `MOSH_IMPAIR_PROXY`, so `tests/run-all.sh` runs the clean-loopback case only.
  Resolved by tests/transport-impaired.sh, commit 60ef500.
- The `new_frame` oracle compares frames between prefixes of one input stream; it never compares a frame across a resize (different widths), which `Display::new_frame` handles in the row-extension code at `terminaldisplay.rs:270-285`.
  `resize.bin` only exercises that path if `vt-check` and `ref-term` apply the recorded resizes, which the `dump` mode (fixed W H) does not.
  Resolved in c182b85: `frame-resize-batch` in both programs; terminal_conformance compares 756 resize frames (7 targets x 2 sizes x 2 initialized values x 27 files) and `vt-fuzz --frames` resizes in every other batch.

## Resolution log

| Finding | Status | Commit | Note |
|---------|--------|--------|------|
| F1 | fixed | d9c8289 | 0xF8 lead restored (the planted mutation was in the 451b2ce snapshot); model matches glibc on 12,779,136 reachable buffers; 1M-case fuzz seed 1: 0 mismatches |
| F2 | fixed | d9c8289 | terminal: no crate-level allows; 64 item-level allows, each with a one-line reason above (`rg -n 'allow\(' crates/terminal/src | wc -l` = 64) |
| F3 | fixed | 14e65f2 | socket calls in `crates/platform/src/socket.rs`, core-dump control in `crates/platform/src/coredump.rs`; `mosh-network` depends on `mosh-platform`, not `libc`, and has `#![forbid(unsafe_code)]` (terminal side: d9c8289); `bind` takes one socket and `recv_raw` a max length (INTERVENTIONS.md) |
| F4 | fixed | d9c8289 | terminal: `cargo fmt --all --check` clean |
| F5 | fixed | d9c8289 | terminal: `cargo clippy --workspace --all-targets -- -D warnings` clean |
| F6 | fixed | d9c8289, cb37d80 | terminal: `#![deny(missing_docs)]`, `#![forbid(unsafe_code)]`, `//!` doc on every module; network done (`#![deny(missing_docs)]`, `//!` naming the upstream file on every module); terminal by agent B |
| F7 | fixed | f01fc0d | private fields as upstream (Row::cells/gen, SavedCursor, Framebuffer::ds and DrawState mode flags stay public, as upstream); dump uses getters plus `debug_*` accessors |
| F8 | fixed | 077968f | d9c8289 built only on devbox's newer rustc (the mixed array types were unchanged); 077968f makes the arrays typed `[&[u8]; N]` consts, so vt-fuzz builds on every toolchain from 1.89 (review G3); also `--frames` with resizes (c182b85) |
| F9 | fixed | 77311f8 | `Fragment::from_bytes` returns a dos_detected error under 10 bytes; tested directly and through a live server `Transport` (`transport_roundtrip` 5/5) |
| F10 | fixed | 77311f8 | conflicting duplicate is an error and the partial instruction is discarded; tested both ways |
| F11 | fixed | 77311f8 | final fragment below a stored index, and arrived above total, are errors; tested with fragment 5 then final 2 |
| F12 | fixed | 77311f8 | protobuf decode failure after valid zlib is a dos_detected error; tested with zlib around `0a 05 78` |
| F13 | fixed | 77311f8 | `process_throwaway_until` checks first and erases nothing if no state would survive; tested through a live `Transport` (remote state stays 0, next valid instruction accepted) |
| F14 | answered | f3cec0d | PLAN.md section 9 item 7 lists the 1e9 ms clock base |
| F15 | answered | f3cec0d | PLAN.md section 9 item 9 lists the reference-state re-lookup |
| F16 | answered | f3cec0d | PLAN.md section 9 item 9 lists the `add_sent_state` index repair |
| F17 | fixed | d9c8289, f3cec0d | `reset_input` added to `State` and called in `set_current_state` as upstream (d9c8289); `compare` omission is PLAN.md section 9 item 10 |
| F18 | fixed | eda88ae | map key borrowed from dispatch_chars or a 1-byte stack array; vt-fuzz 100,000 cases seed 21: 0 mismatches |
| F19 | fixed | cc19759, e625741 | `split_first_mut` in `rationalize_states` and `get_remote_diff`; upstream's self-subtraction of the front state is `State::subtract_self`, whose default still copies; `Complete` (no-op) and `UserStream` (clear) override it to remove the copy, done in e625741. transport_sync passes direct (10.0 s) and through udp-impair 30% loss + rebind (13.4 s) |
| F20 | fixed | 77311f8 | `Session::decrypt` returns "Ciphertext too long." above `RECEIVE_MTU`; tested with 2057 bytes |
| F21 | fixed | f01fc0d | `unreachable!` with the reason |
| F22 | answered | f01fc0d | Rc kept to match shared_ptr copy-on-write in a single-threaded program; doc note on Framebuffer |
| F23 | fixed | e29a4cc | `ocb3` 0.2 has no zeroize feature; `aes` `zeroize` feature wipes the round keys inside the OCB context, `Session` zeroes its key copy in `Drop`; ocb3's private offset table is not wiped (no API) |
| F24 | fixed | f01fc0d, e29a4cc | terminal: `append_string` merged into `append`, uncalled `name()` removed; network: `myatoi` doc says it is kept for crates/mosh |
| F25 | fixed | eda88ae, f01fc0d | TERM table rows copied from `infocmp -1 <name>` (ncurses 6.4), 30 entries plus screen/tmux/rxvt family fallback |
| F26 | fixed | f01fc0d, e29a4cc | terminal: index casts go through a commented `idx()`; true-color and cmd_num casts commented; network: a one-line comment at every cast that could be negative, truncate or overflow |
