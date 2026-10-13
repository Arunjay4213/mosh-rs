# Session corpus

Raw output of real programs running under a pseudoterminal, recorded on an arm64 Ubuntu 24.04 machine by `tools/pty-record`.
Each `.bin` holds every byte the program wrote to the pty, with no framing.
Each `.bin.meta` holds the command, starting size, byte count, duration and exit status.
Recordings start at 80x24 with `TERM=xterm-256color` and `LANG=C.UTF-8`.
The scripted keystrokes for each file are in `tools/corpus/NAME.script`.
To re-record: `tools/corpus/record-all.sh [NAME...]` (this rewrites this README).
Content depends on the machine (file listings, process lists, dates), so re-recording changes the bytes.

These files are conformance inputs for the terminal emulator oracle (PLAN.md section 3, oracle 3).

| file | command | size (bytes) | duration (ms) |
| --- | --- | ---: | ---: |
| `bash-colors.bin` | `bash --norc --noprofile -i` | 5538 | 367 |
| `claude-tui.bin` | `bash --norc --noprofile -i` | 2053 | 10471 |
| `colors256-truecolor.bin` | `bash --norc --noprofile -i` | 14896 | 369 |
| `htop.bin` | `htop` | 6686 | 5050 |
| `less.bin` | `bash -c 'man less \| less'` | 15574 | 3078 |
| `resize.bin` | `bash --norc --noprofile -i` | 2750 | 1359 |
| `seq100k.bin` | `seq 1 100000` | 688895 | 153 |
| `tmux-inside.bin` | `tmux -f /dev/null -L corpus new-session` | 4713 | 4015 |
| `vim.bin` | `vim -u NONE -N /tmp/tmp.s7AJt2kfjx/vim-corpus.txt` | 9100 | 4752 |
| `wide-emoji.bin` | `bash --norc --noprofile -i` | 2138 | 357 |

Total: 752343 bytes.

`$TMP` in a command is a fresh temporary directory, which is also the working directory, except for `claude-tui` (see below).

## Notes per file

- `bash-colors.bin`: `ls --color=always -la /usr/bin | head -40`, the 16 foreground and background colors via `echo -e`, bold/underline/inverse, two `grep --color=always` runs.
- `vim.bin`: 30 lines of mixed text (tabs, Latin accents, Greek, Cyrillic, CJK, Hangul, emoji, box drawing, Thai, Hebrew, fullwidth) typed in insert mode, then `gg`, `:set number`, motions, `G`, Ctrl-U/D/E/Y scrolling, `14G`, `:q!`.
- `less.bin`: `man less | less`. Because man writes into a pipe it emits plain text without overstrike bold. Ten page-downs, `/pattern`, `n` twice, `q`.
- `htop.bin`: real htop (no fallback to `top` was needed), 5 seconds, then F10.
- `tmux-inside.bin`: tmux exits with status 1 because `tmux kill-server` makes the attached client print `[server exited]` and exit 1. That is normal tmux behavior.
- `claude-tui.bin`: `claude --version`, then the full-screen claude interface for 8 seconds (alternate screen, logo, input box, footer), Ctrl-C twice. It runs in the repo root because in a new directory claude only shows its "trust this folder?" dialog. Environment variables from a parent Claude Code session are removed first. Claude's terminal queries (DA1, XTVERSION, kitty keyboard) go unanswered, as no terminal is attached; it renders anyway.
- `seq100k.bin`: well under the 2 MB cap, so it is complete (no truncation).
- `colors256-truecolor.bin`: 256 foreground and 256 background colors with `38;5;N` / `48;5;N`, three 80-column 24-bit gradients (`38;2`, `48;2`, both combined), SGR 1 2 3 4 5 7 8 9 21 53 alone, combinations, and the matching reset codes 22-29.
- `wide-emoji.bin`: CJK, precomposed and conjoining Hangul, emoji with and without VS15/VS16, ZWJ family, flag, skin tone, combining accents, Devanagari conjuncts, ZWSP and ZWNJ, a line of 40 wide characters (exactly 80 columns), a line of 80 wide characters (wraps), 79 digits then a wide character (must wrap at column 80), Arabic and Hebrew.
- `resize.bin`: `seq 1 100` at 80x24, resize to 40x12 (TIOCSWINSZ plus SIGWINCH), `seq 1 50`, resize to 120x40, `ls -la /etc | head -30`.
