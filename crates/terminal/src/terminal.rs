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
// Ported from: upstream/src/terminal/terminal.h, upstream/src/terminal/terminal.cc

use crate::parseraction::Action;
use crate::terminaldispatcher::{Dispatcher, Function_Type};
use crate::terminalframebuffer::{Cell, Framebuffer};
use crate::terminaluserinput::UserInput;
use crate::wcwidth::wcwidth;

#[derive(Clone, Debug)]
pub struct Emulator {
    pub(crate) fb: Framebuffer,
    pub(crate) dispatch: Dispatcher,
    pub(crate) user: UserInput,
}

impl PartialEq for Emulator {
    fn eq(&self, x: &Emulator) -> bool {
        /* dispatcher and user are irrelevant for us */
        self.fb == x.fb
    }
}

impl Emulator {
    pub fn new(s_width: usize, s_height: usize) -> Emulator {
        Emulator {
            fb: Framebuffer::new(s_width as i32, s_height as i32),
            dispatch: Dispatcher::new(),
            user: UserInput::new(),
        }
    }

    pub fn read_octets_to_host(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.dispatch.terminal_to_host)
    }

    pub fn get_fb(&self) -> &Framebuffer {
        &self.fb
    }

    pub(crate) fn execute(&mut self, act: &Action) {
        self.dispatch.dispatch(Function_Type::CONTROL, act, &mut self.fb);
    }

    pub(crate) fn print(&mut self, act: &Action) {
        assert!(act.char_present);

        let ch = act.ch;

        /*
         * Check for printing ISO 8859-1 first, it's a cheap way to detect
         * some common narrow characters.
         */
        let chwidth: i32 = if ch == 0 {
            -1
        } else if Cell::isprint_iso8859_1(ch) {
            1
        } else {
            wcwidth(ch)
        };

        /* Cell* this_cell = fb.get_mutable_cell(); -- unshares the cursor row */
        let fb = &mut self.fb;
        fb.get_mutable_cell(-1, -1);

        match chwidth {
            1 | 2 => {
                /* normal character, wide character */
                if fb.ds.auto_wrap_mode && fb.ds.next_print_will_wrap {
                    fb.get_mutable_row(-1).set_wrap(true);
                    fb.ds.move_col(0, false, false);
                    fb.move_rows_autoscroll(1);
                } else if fb.ds.auto_wrap_mode
                    && (chwidth == 2)
                    && (fb.ds.get_cursor_col() == fb.ds.get_width() - 1)
                {
                    /* wrap 2-cell chars if no room, even without will-wrap flag */
                    fb.reset_cell(-1, -1);
                    fb.get_mutable_row(-1).set_wrap(false);
                    /* There doesn't seem to be a consistent way to get the
                    downstream terminal emulator to set the wrap-around
                    copy-and-paste flag on a row that ends with an empty cell
                    because a wide char was wrapped to the next line. */
                    fb.ds.move_col(0, false, false);
                    fb.move_rows_autoscroll(1);
                }

                if fb.ds.insert_mode {
                    for _ in 0..chwidth {
                        let (r, c) = (fb.ds.get_cursor_row(), fb.ds.get_cursor_col());
                        fb.insert_cell(r, c);
                    }
                }

                /* this_cell is the cell at the cursor in every branch above */
                fb.reset_cell(-1, -1);
                let this_cell = fb.get_mutable_cell(-1, -1);
                this_cell.append(ch);
                this_cell.set_wide(chwidth == 2); /* chwidth had better be 1 or 2 here */
                fb.apply_renditions_to_cell(-1, -1);
                fb.apply_hyperlink_to_cell(-1, -1);

                if chwidth == 2 && fb.ds.get_cursor_col() + 1 < fb.ds.get_width() {
                    /* erase overlapped cell */
                    let (r, c) = (fb.ds.get_cursor_row(), fb.ds.get_cursor_col() + 1);
                    fb.reset_cell(r, c);
                }

                fb.ds.move_col(chwidth, true, true);
            }
            0 => {
                /* combining character */
                let (r, c) = match fb.get_combining_cell() {
                    /* can be null if we were resized */
                    None => return, /* character is now offscreen */
                    Some(rc) => rc,
                };

                if fb.get_cell(r, c).empty() {
                    /* cell starts with combining character */
                    /* ... but isn't necessarily the target for a new
                    base character [e.g. start of line], if the
                    combining character has been cleared with
                    a sequence like ED ("J") or EL ("K") */
                    assert!(!fb.get_cell(r, c).get_wide());
                    fb.get_mutable_cell(r, c).set_fallback(true);
                    fb.ds.move_col(1, true, true);
                }
                let combining_cell = fb.get_mutable_cell(r, c);
                if !combining_cell.full() {
                    combining_cell.append(ch);
                }
            }
            -1 => { /* unprintable character */ }
            _ => panic!("unexpected character width from wcwidth()"),
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn CSI_dispatch(&mut self, act: &Action) {
        self.dispatch.dispatch(Function_Type::CSI, act, &mut self.fb);
    }

    #[allow(non_snake_case)]
    pub(crate) fn OSC_end(&mut self, act: &Action) {
        self.dispatch.OSC_dispatch(act, &mut self.fb);
    }

    #[allow(non_snake_case)]
    pub(crate) fn Esc_dispatch(&mut self, act: &Action) {
        /* handle 7-bit ESC-encoding of C1 control characters */
        if self.dispatch.get_dispatch_chars().is_empty() && (0x40 <= act.ch) && (act.ch <= 0x5F) {
            let mut act2 = *act;
            act2.ch += 0x40;
            self.dispatch.dispatch(Function_Type::CONTROL, &act2, &mut self.fb);
        } else {
            self.dispatch.dispatch(Function_Type::ESCAPE, act, &mut self.fb);
        }
    }

    pub(crate) fn resize(&mut self, s_width: usize, s_height: usize) {
        self.fb.resize(s_width as i32, s_height as i32);
    }
}
