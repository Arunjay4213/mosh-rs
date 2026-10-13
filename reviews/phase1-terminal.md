# Phase 1 review: terminal emulator and display

Reviewed tree: commits d9c8289 through 14e65f2 (agent A pushed 77311f8 and 14e65f2 while the review ran; agent B pushed f01fc0d and c182b85 after it).

Scope: `crates/terminal/src/*.rs` read function by function against `upstream/src/terminal/*.cc` and `*.h`.
Method: for every function, constants, comparison boundaries, order of side effects, integer types and conversions, early returns, size 0/1/max behavior and error paths were compared.
Anything whose reachability was in doubt was fed to both `ref/ref-term/ref-term` and `target/release/vt-check` (`dump` and `frame`, the latter with `initialized` 1 and 0) and the outputs were diffed.
Allowed differences (PLAN.md section 9, `Result` for exceptions, indices for pointers) are not listed.

## Files reviewed

| Rust | Upstream | Result |
| --- | --- | --- |
| parser.rs | parser.cc, parser.h | no deviation; the hand-written `mbrtowc` was checked against glibc on 5/6-byte leads, overlongs, surrogates, >0x10FFFF, 0xC0/0xC1/0xFE leads: all identical |
| parseraction.rs | parseraction.cc, parseraction.h | no deviation |
| parserstate.rs | parserstate.cc, parserstate.h, parserstatefamily.h | no deviation (every range in every state rule, enter/exit actions, anywhere rule) |
| parsertransition.rs | parsertransition.h | no deviation |
| terminal.rs | terminal.cc, terminal.h | no deviation; `print` keeps the early `get_mutable_cell()` unshare on the cursor row, the wide-wrap branch, insert mode, the overlapped-cell erase and the combining-cell fallback |
| terminaldispatcher.rs | terminaldispatcher.cc, terminaldispatcher.h | no deviation; `parse_params` reproduces strtol/ERANGE handling (values above 65535 or above i64 become -1), the 100-byte param limit, the 8-byte dispatch_chars limit, the 16 KiB OSC limit, and `std::map::insert` first-wins registration |
| terminaldisplay.rs | terminaldisplay.cc, terminaldisplay.h | no deviation; `open`/`close` strings byte-identical, scroll detection (lines 168-243) including the CR/LF shortcut, region form and local row shift, `put_row` ECH/EL rules, wrap handling, `append_move` CR/LF and BS optimizations, mouse/paste/focus/encoding transitions |
| terminaldisplayinit.rs | terminaldisplayinit.cc | PLAN 9.6 replacement; one gap, T-1 |
| terminalframebuffer.rs | terminalframebuffer.cc, terminalframebuffer.h | no deviation; `Framebuffer ==` compares row pointers (`Rc::ptr_eq`) as `vector<shared_ptr<Row>>::operator==` does, `Row ==` is `gen && cells`, copy-on-write via `Rc::make_mut` matches `use_count() > 1`, resize keeps old `width` for `reinitialize_tabs`, 25-bit color field truncation kept |
| terminalfunctions.rs | terminalfunctions.cc | no deviation; every CSI/ESC/control function, the `clears_wrap_state` flags, the DEC and ANSI mode tables, SGR 38/48 with 5 and 2 sub-forms including the unsigned-to-int true color pass-through, OSC 0/1/2/8/52 and the 256-char title cut |
| terminaluserinput.rs | terminaluserinput.cc, terminaluserinput.h | no deviation |
| wcwidth.rs | glibc `wcwidth()` via ref/ref-term/gen-wcwidth.c | generator and ref-term.cc both `setlocale(LC_ALL, "C.UTF-8")`; regenerating the table reproduces the committed file byte for byte except the version comment; the table is identical under `en_US.UTF-8` |

Oracle evidence for the paths the fuzzer reaches only partially (all `SAME`):

```
$ for each input: ref/ref-term/ref-term dump 10 4 < in > a; target/release/vt-check dump 10 4 < in > b; cmp a b
SAME CUF huge param / CUF 65535 / CUF 65536 / 100+ param chars SGR / truecolor edge (65535;0;0)
SAME OSC8 forms / title 300 / OSC 52/0/1/2/3 / DECALN DECSTBM origin / DECSTBM invalid
SAME CBT CHT / ECH edges / ICH DCH 65535 / insert mode wide wrap / wide at last col
SAME DEC modes (3,5,25,1004,1007,2004,1,9,1005,1003,1015,1001,1049,47,1047) / no autowrap
SAME TBC HTS / DECSC DECRC DECSTR RIS / DSR DA / SU SD / IL DL / combining after erase
SAME combining at wrap col / ED EL params / size 1x1 / size 2x2 wide / DCS APC PM SOS / mixed CSI ESC
$ for each pair: ref-term frame 10 4 old new {1,0}; vt-check frame 10 4 old new {1,0}; cmp
SAME full screen scroll (CR/LF shortcut) / scroll 2 with cursor not at bottom (region form)
SAME region scroll / DL at top / wrapped row / wide at end wrapping / ECH>4 run / colored blank run
SAME title clip bell modes / mouse mode switch / mouse mode off / distinct icon and title
SAME hyperlink and sgr / cursor moves (BS, CRLF) / scroll with wrapped rows / clear then bottom
SAME wide row cursor / combining fallback cell / region scroll leaving row 0 / scroll more than height
```

Resize paths (`Framebuffer::resize`, `DrawState::resize`, `new_frame` with a size change) cannot be driven through ref-term; they were compared by reading and match line for line.
A second reviewer pass re-read `Framebuffer::resize`, `DrawState::resize`, `reset`, `soft_reset`, `Framebuffer ==` (same five members, rows by pointer, `icon_name` and `title_initialized` omitted on both sides), `Row` cloning in `resize` (derived `Clone` keeps `gen`), `Display::open`/`close`, `UserInput::input`, and the `smcup`/`rmcup` column of the TERM table against `tput -T<name> smcup|rmcup` for all 20 rows (all match, including putty).

Note, not a finding: the width table is frozen to glibc 2.39 (Unicode 15).
A C++ peer on a host with a different glibc will disagree on widths of newer code points exactly as two upstream peers on different glibcs do; this is the same exposure upstream has, not a port deviation.

## Findings

### T-1  severity: low
file: crates/terminal/src/terminaldisplayinit.rs:77
upstream: upstream/src/terminal/terminaldisplayinit.cc:108
deviation: The fixed TERM table (PLAN 9.6) falls back to xterm's answers (`ech` present, `bce` true) for every TERM not listed or not prefixed by screen/tmux/rxvt, but several terminfo entries on devbox's ncurses 6.4 answer differently: `iterm2` (no ech), `kterm` (no ech, no bce), `cygwin` (no ech, no bce), `xterm-color` (no ech, no bce), `eterm` (no ech, no bce), `ansi` (no bce), and `screen.xterm-256color` (has ech, but the "screen" prefix maps it to the no-ech entry).
Evidence:
```
$ for t in iterm2 kterm cygwin xterm-color eterm ansi screen.xterm-256color; do printf "%s: " $t; \
    echo "ech=$(tput -T$t ech 1 | od -An -c | tr -d ' \n') bce_true=$(tput -T$t bce && echo yes || echo no)"; done
iterm2: ech= bce_true=yes
kterm: ech= bce_true=no
cygwin: ech= bce_true=no
xterm-color: ech= bce_true=no
eterm: ech= bce_true=no
ansi: ech=033[1X bce_true=no
screen.xterm-256color: ech=033[1X bce_true=no
```
effect: On those terminals the client emits `CSI n X` for blank runs longer than 4 (terminaldisplay.cc:391) and `CSI K` / `CSI n X` under a non-default background (terminaldisplay.cc:443, `can_use_erase`) where upstream would have written spaces. A terminal that really lacks ECH or BCE (kterm, old cygwin console, xterm-color) shows stale cells or the wrong background after erases; iterm2 and screen.xterm-256color only differ in output bytes, not in what is shown. Unknown TERMs that upstream rejects with "Unknown terminal type." start successfully with xterm assumptions, which is the intent of 9.6.
status: fixed in 13f6466. The hand-written table and prefix matching are gone; `crates/terminal/src/terminfo.rs` is generated by `ref/ref-term/gen-terminfo.c` from all 1830 `toe -a` entries and their aliases (2793 names, 115 tuples). iterm2, kterm, cygwin, xterm-color, eterm and ansi now give their own answers, and screen.xterm-256color (ech, no bce) is an exact entry. `tests/tests/terminfo_table.rs`: `terms=2830 ... mismatches=0` against upstream Display(true).

### T-2  severity: high
file: crates/terminal/src/terminaldisplayinit.rs:132
upstream: upstream/src/terminal/terminaldisplayinit.cc:111
deviation: kitty sets `TERM=xterm-kitty`, which is not in the table and has no listed family prefix, so `term_caps` falls through to `lookup("xterm")` and answers `bce=true`; the table's own `kitty` row (line 102, `bce=false`, copied from the terminfo entry upstream would read) is never selected because no terminal sets `TERM=kitty`.
Evidence:
```
$ tput -Tkitty bce && echo yes || echo no
no
$ infocmp -1 xterm-kitty
infocmp: couldn't open terminfo file /usr/share/terminfo/x/xterm-kitty.   (devbox has the entry only under the name kitty; kitty itself installs it as xterm-kitty)
```
effect: In kitty, `can_use_erase` (terminaldisplay.cc:473) is true under a non-default background, so the client emits `CSI K` and `CSI n X` where upstream writes spaces.
Any program that sets a background color and then erases (vim, less with a color scheme, a colored prompt) reaches this path.
If kitty behaves as its terminfo declares (no BCE), the erased cells show the default background instead of the program's, so the screen differs from upstream.
This could not be tested on devbox (no kitty); the severity follows the terminfo entry, which is the same source upstream trusts, and should be downgraded only if kitty is shown to honor BCE.
status: fixed in 13f6466. kitty-terminfo installed on devbox; `xterm-kitty` (and its alias `KovIdTTY`) resolve to their own entry with bce=0, ech present, smcup/rmcup `\E[?1049h`/`\E[?1049l`, identical to what upstream reads (checked by `ref-term caps` vs `vt-check caps`).

files reviewed: 12, findings: high 1 medium 0 low 1
