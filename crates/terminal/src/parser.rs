// Mosh: the mobile shell
// Copyright 2012 Keith Winstein
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// In addition, as a special exception, the copyright holders give
// permission to link the code of portions of this program with the
// OpenSSL library under certain conditions as described in each
// individual source file, and distribute linked combinations including
// the two.
//
// You must obey the GNU General Public License in all respects for all
// of the code used other than OpenSSL. If you modify file(s) with this
// exception, you may extend this exception to your version of the
// file(s), but you are not obligated to do so. If you do not wish to do
// so, delete this exception statement from your version. If you delete
// this exception statement from all source files in the program, then
// also delete it here.
// Ported from: upstream/src/terminal/parser.h, upstream/src/terminal/parser.cc

/* Based on Paul Williams's parser,
http://www.vt100.net/emu/dec_ansi_parser */

use crate::parseraction::{wchar_t, Action, Actions};
use crate::parserstate::State;

fn append_or_delete(act: Action, vec: &mut Actions) {
    if !act.ignore() {
        vec.push(act);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parser {
    state: State,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser { state: State::Ground }
    }

    pub fn input(&mut self, ch: wchar_t, ret: &mut Actions) {
        let tx = self.state.input(ch);

        if tx.next_state.is_some() {
            append_or_delete(self.state.exit(), ret);
        }

        append_or_delete(tx.action, ret);

        if let Some(next_state) = tx.next_state {
            append_or_delete(next_state.enter(), ret);
            self.state = next_state;
        }
    }

    pub fn reset_input(&mut self) {
        self.state = State::Ground;
    }
}

pub const BUF_SIZE: usize = 8;

/// Result of glibc `mbrtowc()` on a UTF-8 buffer with a fresh state.
enum Mbrtowc {
    /// `(size_t)-1`, errno = EILSEQ
    Invalid,
    /// `(size_t)-2`
    Incomplete,
    /// bytes consumed (0 for NUL), and the decoded value
    Parsed(usize, u32),
}

/// glibc's UTF-8 `mbrtowc()` (C.UTF-8 / en_US.UTF-8, glibc 2.39), for the
/// buffers upstream's UTF8Parser can hand it. Matches glibc on every
/// reachable buffer: 0xC0/0xC1/0x80-0xBF/0xFE/0xFF leads are invalid,
/// 0xF5-0xFD leads are accepted as 4-, 5- and 6-byte forms (values above
/// 0x10FFFF are returned, not rejected), a short buffer is incomplete only
/// when every byte after the lead is a continuation byte, and overlong forms
/// and surrogates are invalid once the sequence is complete.
fn mbrtowc(b: &[u8]) -> Mbrtowc {
    let c = b[0] as u32;
    if c < 0x80 {
        return Mbrtowc::Parsed(if c == 0 { 0 } else { 1 }, c);
    }
    let (cnt, mut v): (usize, u32) = if (0xC2..0xE0).contains(&c) {
        (2, c & 0x1F)
    } else if (c & 0xF0) == 0xE0 {
        (3, c & 0x0F)
    } else if (c & 0xF8) == 0xF0 {
        (4, c & 0x07)
    } else if (c & 0xFC) == 0xF9 {
        (5, c & 0x03)
    } else if (c & 0xFE) == 0xFC {
        (6, c & 0x01)
    } else {
        return Mbrtowc::Invalid;
    };
    if b.len() < cnt {
        if b[1..].iter().all(|&x| (x & 0xC0) == 0x80) {
            return Mbrtowc::Incomplete;
        }
        return Mbrtowc::Invalid;
    }
    for &x in &b[1..cnt] {
        if (x & 0xC0) != 0x80 {
            return Mbrtowc::Invalid;
        }
        v = (v << 6) | (x as u32 & 0x3F);
    }
    if cnt > 2 && (v >> (5 * cnt - 4)) == 0 {
        return Mbrtowc::Invalid;
    }
    if (0xD800..=0xDFFF).contains(&v) {
        return Mbrtowc::Invalid;
    }
    Mbrtowc::Parsed(cnt, v)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UTF8Parser {
    parser: Parser,

    buf: [u8; BUF_SIZE],
    buf_len: usize,
}

impl Default for UTF8Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl UTF8Parser {
    pub fn new() -> UTF8Parser {
        UTF8Parser { parser: Parser::new(), buf: [0; BUF_SIZE], buf_len: 0 }
    }

    pub fn input(&mut self, c: u8, ret: &mut Actions) {
        assert!(self.buf_len < BUF_SIZE);

        /* 1-byte UTF-8 character, aka ASCII?  Cheat. */
        if self.buf_len == 0 && c <= 0x7f {
            self.parser.input(c as wchar_t, ret);
            return;
        }

        self.buf[self.buf_len] = c;
        self.buf_len += 1;

        /* This function will only work in a UTF-8 locale. */
        let mut pwc: wchar_t;

        let mut total_bytes_parsed: usize = 0;
        let orig_buf_len = self.buf_len;

        /* this routine is somewhat complicated in order to comply with
        Unicode 6.0, section 3.9, "Best Practices for using U+FFFD" */

        while total_bytes_parsed != orig_buf_len {
            assert!(total_bytes_parsed < orig_buf_len);
            assert!(self.buf_len > 0);
            let bytes_parsed: usize;

            match mbrtowc(&self.buf[..self.buf_len]) {
                /* this returns 0 when n = 0! */
                Mbrtowc::Parsed(0, _) => {
                    /* character was NUL, accept and clear buffer */
                    assert!(self.buf_len == 1);
                    self.buf_len = 0;
                    pwc = 0;
                    bytes_parsed = 1;
                }
                Mbrtowc::Invalid => {
                    /* invalid sequence, use replacement character and try again with last char */
                    if self.buf_len > 1 {
                        self.buf[0] = self.buf[self.buf_len - 1];
                        bytes_parsed = self.buf_len - 1;
                        self.buf_len = 1;
                    } else {
                        self.buf_len = 0;
                        bytes_parsed = 1;
                    }
                    pwc = 0xFFFD;
                }
                Mbrtowc::Incomplete => {
                    /* can't parse incomplete multibyte character */
                    total_bytes_parsed += self.buf_len;
                    continue;
                }
                Mbrtowc::Parsed(n, wc) => {
                    /* parsed into pwc, accept */
                    bytes_parsed = n;
                    pwc = wc;
                    assert!(bytes_parsed <= self.buf_len);
                    self.buf.copy_within(bytes_parsed..self.buf_len, 0);
                    self.buf_len -= bytes_parsed;
                }
            }

            let pwcheck: u32 = pwc;

            if pwcheck > 0x10FFFF {
                /* outside Unicode range */
                pwc = 0xFFFD;
            }

            if (0xD800..=0xDFFF).contains(&pwcheck) {
                /* surrogate code point */
                /*
                  OS X unfortunately allows these sequences without EILSEQ, but
                  they are ill-formed UTF-8 and we shouldn't repeat them to the
                  user's terminal.
                */
                pwc = 0xFFFD;
            }

            self.parser.input(pwc, ret);

            total_bytes_parsed += bytes_parsed;
        }
    }

    pub fn reset_input(&mut self) {
        self.parser.reset_input();
        self.buf[0] = 0;
        self.buf_len = 0;
    }
}
