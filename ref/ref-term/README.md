# ref-term

Differential-testing oracle for the mosh-rs terminal emulator (`crates/terminal`).
`ref-term` links upstream mosh's own `libmoshterminal.a` and `libmoshutil.a` from `upstream/`, so its output is by definition upstream's behavior.
Its Rust twin is `vt-check` (`tests/src/bin/vt-check.rs`), which uses only the `mosh_terminal` crate.
Both programs take the same arguments and must print byte-identical output.

Build: `make -C ref/ref-term` (needs the built upstream tree; links `-ltinfo`).
Both programs set the locale to `C.UTF-8` first, because upstream requires a UTF-8 locale for `mbrtowc` and `wcwidth`.

## Feeding

Input bytes are fed exactly as `Terminal::Complete::act` does: one byte at a time into `Parser::UTF8Parser::input`, and every resulting action is applied with `act_on_terminal` before the next byte.
Replies the emulator generates (`read_octets_to_host`, e.g. for `CSI 6 n`) are collected and shown in the dump.
The emulator size is fixed; no resize is applied.

## CLI

- `dump W H < input`: one `Terminal::Emulator(W, H)` is fed all of stdin, then the dump below is printed.
- `batch W H < records`: stdin is a sequence of records, each a big-endian `u32` length followed by that many bytes.
  For each record a fresh emulator is fed the bytes, the dump is printed, then a line `=====`.
- `steps W H < records`: one emulator; each record starts with a tag byte.
  `B` + bytes: feed the bytes as above.
  `R` + big-endian `u32` width + big-endian `u32` height (9 bytes in all): apply a `Parser::Resize` action, as `Terminal::Complete::act(const Action&)` does.
  After the last record the dump is printed.
  This lets a test replay exactly what a `Complete` was fed, resizes included.
- `frame W H OLDFILE NEWFILE [initialized]`: one emulator is fed the bytes of OLDFILE; a copy of its framebuffer is `last`.
  The same emulator (same parser state) is then fed the bytes of NEWFILE, giving `f`.
  The raw bytes of `Display(false).new_frame(initialized, last, f)` are written to stdout.
  `initialized` is 1 (default) or 0.
  Note that NEWFILE holds only the bytes that come after OLDFILE, not a cumulative prefix.
- `frame-batch W H [initialized] < records`: records come in pairs (old, new), each handled like `frame`.
  Output per pair is a big-endian `u32` length followed by the `new_frame` bytes.

- `frame-resize-batch W H [initialized] < records`: records come in threes (size, old, new).
  The size record is 8 bytes: big-endian `u32` width then big-endian `u32` height (both at least 1).
  Like `frame`, but after feeding old and copying `last`, a `Parser::Resize` action resizes the emulator to that size before new is fed, so `new_frame` compares framebuffers of different sizes.
  Output per triple is a big-endian `u32` length followed by the `new_frame` bytes.

- `caps < names`: for each terminal type name on stdin (one per line), set `TERM` to it and print upstream's `Display(true)` decisions on one line: `<hex name> ech E bce B title T smcup <hex>|none rmcup <hex>|none`, or `<hex name> error <message>` when upstream's constructor throws (hardcopy or unknown types).
  `MOSH_NO_TERM_INIT` is honored as upstream does. No W H arguments.
  vt-check prints the same line from the generated table (it never errors: unknown types fall back to xterm, PLAN.md 9.6).

Exit status is 0 on success and 2 on usage or input errors.
An upstream assertion failure aborts ref-term; a Rust `assert!` panics vt-check.

## Dump format

Text, one item per line, every row and every cell, nothing elided.
Integers are decimal unless noted.
`<hex>` is lowercase hex of the raw bytes with no separators (empty when there are no bytes).
`<wlist>` is a comma-separated list of lowercase hex code points (upstream stores titles as `std::vector<wchar_t>`).
Booleans are `0` or `1`.

```
size W H
cursor ROW COL visible B next_print_will_wrap B
combining ROW COL
origin B autowrap B insert B cursor_keys_app B reverse_video B bracketed_paste B
mouse mode N focus B altscroll B encoding N
scroll_region TOP BOTTOM
default_tabs B tabs <comma list of columns with a tab stop>
renditions ATTRS FG BG
hyperlink none | <hex params>|<hex url>
saved ROW COL autowrap B origin B renditions ATTRS FG BG
title_initialized B
icon <wlist>
title <wlist>
clipboard <wlist>
bell_count N
to_host <hex>
row R gen G wrap B
  cell C contents=<hex> wide B fallback B wrap B renditions ATTRS FG BG link=none | link=<hex params>|<hex url>
  ... one line per cell ...
... one block per row ...
```

Field sources (all from `src/terminal/terminalframebuffer.h` unless noted):

- `size`, `cursor`, `combining`, mode flags, `mouse`, `scroll_region`, `default_tabs`/`tabs`, `renditions`, `hyperlink`, `saved`: `DrawState` (`saved` is `DrawState::save`, a `SavedCursor`).
  `mouse mode` and `encoding` are the numeric enum values (0, 9, 1000-1003; 0, 1005, 1006, 1015).
- `renditions ATTRS FG BG`: the raw `Renditions` bitfields: `attributes` (bit 0 bold, 1 faint, 2 italic, 3 underlined, 4 blink, 5 inverse, 6 invisible), `foreground_color` and `background_color` (25-bit fields; 30-37/40-47 for ANSI colors, 38+n/48+n for 256-color index n above 7, bit 24 set for true color).
- `title_initialized`, `icon`, `title`, `clipboard`, `bell_count`: `Framebuffer`.
- `to_host`: the concatenated replies from `Emulator::read_octets_to_host`.
- `row R gen G wrap B`: `Row::gen` (the process-wide generation counter, compared by `Row::operator==` and therefore by `new_frame`'s scroll detection) and `Row::get_wrap()`.
  Because the counter is process-wide, in `batch` mode it keeps counting across records; both programs must create rows in the same order to agree.
- `cell`: every field `Cell::operator==` compares: `contents` (UTF-8 bytes), `wide`, `fallback`, `wrap`, `renditions`, `hyperlink`.

`Framebuffer::operator==` compares rows by `shared_ptr` identity, which a dump cannot show; `new_frame` output (the `frame` modes) covers row sharing.
The dispatcher's partial-sequence state and the parser state are not dumped; they show up through their effect on later input.
Upstream has no keypad application mode in `DrawState`, so the dump has no such field.

## gen-wcwidth

`make -C ref/ref-term wcwidth` regenerates `crates/terminal/src/wcwidth.rs`: glibc `wcwidth()` for every code point 0..=0x10FFFF under `C.UTF-8`, as a sorted range table.
The file header records the glibc version that produced it.
On devbox (glibc 2.39) `C.UTF-8` and `en_US.UTF-8` give identical widths for every code point.

## gen-terminfo

`make -C ref/ref-term terminfo` regenerates `crates/terminal/src/terminfo.rs`: for every name from `toe -a` and every alias in each entry's names line, the answers upstream reads after `setupterm` (`ech` present, `bce`, `smcup`, `rmcup`), deduplicated into a tuple table plus a sorted name index.
The header records the package versions (ncurses-base/ncurses-term and kitty-terminfo) and glibc.
`tests/tests/terminfo_table.rs` checks it against ref-term's `caps` for every name and refuses to run if the local packages differ from the recorded ones.
