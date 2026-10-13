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
// Ported from: upstream/src/terminal/terminaldisplay.h, upstream/src/terminal/terminaldisplay.cc

use std::rc::Rc;

use crate::parseraction::wchar_t;
use crate::terminalframebuffer::{color_type, rows_type, Cell, Framebuffer, Hyperlink, Renditions, Row};
use crate::terminalframebuffer::{MOUSE_ENCODING_DEFAULT, MOUSE_REPORTING_NONE};

/* Print a new "frame" to the terminal, using ANSI/ECMA-48 escape codes. */

fn initial_rendition() -> Renditions {
    Renditions::new(0)
}

/* variables used within a new_frame */
pub struct FrameState<'a> {
    pub str: String,

    pub cursor_x: i32,
    pub cursor_y: i32,
    pub current_rendition: Renditions,
    pub current_hyperlink: Hyperlink,
    pub cursor_visible: bool,

    pub last_frame: &'a Framebuffer,
}

impl<'a> FrameState<'a> {
    pub fn new(s_last: &'a Framebuffer) -> FrameState<'a> {
        FrameState {
            /* Preallocate for better performance.  Make a guess-- doesn't matter for correctness */
            str: String::with_capacity((s_last.ds.get_width() * s_last.ds.get_height() * 4) as usize),
            cursor_x: 0,
            cursor_y: 0,
            current_rendition: Renditions::new(0),
            current_hyperlink: Hyperlink::default(),
            cursor_visible: s_last.ds.cursor_visible,
            last_frame: s_last,
        }
    }

    pub fn append_char(&mut self, c: char) {
        self.str.push(c);
    }
    pub fn append_n(&mut self, s: usize, c: char) {
        for _ in 0..s {
            self.str.push(c);
        }
    }
    pub fn append_wchar(&mut self, wc: wchar_t) {
        Cell::append_to_str(&mut self.str, wc);
    }
    pub fn append(&mut self, s: &str) {
        self.str.push_str(s);
    }
    pub fn append_string(&mut self, append: &str) {
        self.str.push_str(append);
    }

    pub fn append_cell(&mut self, cell: &Cell) {
        cell.print_grapheme(&mut self.str);
    }

    pub fn append_silent_move(&mut self, y: i32, x: i32) {
        if self.cursor_x == x && self.cursor_y == y {
            return;
        }
        /* turn off cursor if necessary before moving cursor */
        if self.cursor_visible {
            self.append("\x1b[?25l");
            self.cursor_visible = false;
        }
        self.append_move(y, x);
    }

    pub fn append_move(&mut self, y: i32, x: i32) {
        let last_x = self.cursor_x;
        let last_y = self.cursor_y;
        self.cursor_x = x;
        self.cursor_y = y;
        // Only optimize if cursor pos is known
        if last_x != -1 && last_y != -1 {
            // Can we use CR and/or LF?  They're cheap and easier to trace.
            if x == 0 && y - last_y >= 0 && y - last_y < 5 {
                if last_x != 0 {
                    self.append_char('\r');
                }
                self.append_n((y - last_y) as usize, '\n');
                return;
            }
            // Backspaces are good too.
            if y == last_y && x - last_x < 0 && x - last_x > -5 {
                self.append_n((last_x - x) as usize, '\x08');
                return;
            }
            // More optimizations are possible.
        }
        let tmp = format!("\x1b[{};{}H", y + 1, x + 1);
        self.append(&tmp);
    }

    pub fn update_rendition(&mut self, r: &Renditions, force: bool) {
        if force || !(self.current_rendition == *r) {
            /* print renditions */
            let s = r.sgr();
            self.append_string(&s);
            self.current_rendition = *r;
        }
    }

    pub fn update_hyperlink(&mut self, h: &Hyperlink, force: bool) {
        if force || self.current_hyperlink != *h {
            /* print hyperlink */
            let s = h.osc8();
            self.append_string(&s);
            self.current_hyperlink = h.clone();
        }
    }
}

#[derive(Clone, Debug)]
pub struct Display {
    /// erase character is part of vt200 but not supported by tmux
    /// (or by "screen" terminfo entry, which is what tmux advertises)
    pub(crate) has_ech: bool,

    /// erases result in cell filled with background color
    pub(crate) has_bce: bool,

    /// supports window title and icon name
    pub(crate) has_title: bool,

    /// enter and exit alternate screen mode
    pub(crate) smcup: Option<&'static str>,
    pub(crate) rmcup: Option<&'static str>,
}

impl Display {
    pub fn open(&self) -> String {
        format!("{}{}", self.smcup.unwrap_or(""), "\x1b[?1h")
    }

    pub fn close(&self) -> String {
        format!(
            "{}{}",
            "\x1b[?1l\x1b[0m\x1b[?25h\
             \x1b[?1003l\x1b[?1002l\x1b[?1001l\x1b[?1000l\
             \x1b[?1015l\x1b[?1006l\x1b[?1005l",
            self.rmcup.unwrap_or("")
        )
    }

    pub fn new_frame(&self, mut initialized: bool, last: &Framebuffer, f: &Framebuffer) -> String {
        let mut frame = FrameState::new(last);

        /* has bell been rung? */
        if f.get_bell_count() != frame.last_frame.get_bell_count() {
            frame.append_char('\x07');
        }

        /* has icon name or window title changed? */
        if self.has_title
            && f.is_title_initialized()
            && ((!initialized)
                || (f.get_icon_name() != frame.last_frame.get_icon_name())
                || (f.get_window_title() != frame.last_frame.get_window_title()))
        {
            /* set icon name and window title */
            if f.get_icon_name() == f.get_window_title() {
                /* write combined Icon Name and Window Title */
                frame.append("\x1b]0;");
                for &i in f.get_window_title() {
                    frame.append_wchar(i);
                }
                frame.append_char('\x07');
                /* ST is more correct, but BEL more widely supported */
            } else {
                /* write Icon Name */
                frame.append("\x1b]1;");
                for &i in f.get_icon_name() {
                    frame.append_wchar(i);
                }
                frame.append_char('\x07');

                frame.append("\x1b]2;");
                for &i in f.get_window_title() {
                    frame.append_wchar(i);
                }
                frame.append_char('\x07');
            }
        }

        /* has clipboard changed? */
        if f.get_clipboard() != frame.last_frame.get_clipboard() {
            frame.append("\x1b]52;c;");
            for &i in f.get_clipboard() {
                frame.append_wchar(i);
            }
            frame.append_char('\x07');
        }

        /* has reverse video state changed? */
        if (!initialized) || (f.ds.reverse_video != frame.last_frame.ds.reverse_video) {
            /* set reverse video */
            let tmp = format!("\x1b[?5{}", if f.ds.reverse_video { 'h' } else { 'l' });
            frame.append(&tmp);
        }

        /* has size changed? */
        if (!initialized)
            || (f.ds.get_width() != frame.last_frame.ds.get_width())
            || (f.ds.get_height() != frame.last_frame.ds.get_height())
        {
            /* reset scrolling region */
            frame.append("\x1b[r");

            /* clear screen */
            frame.append("\x1b[0m\x1b[H\x1b[2J");
            initialized = false;
            frame.cursor_x = 0;
            frame.cursor_y = 0;
            frame.current_rendition = initial_rendition();
            frame.current_hyperlink = Hyperlink::default();
        } else {
            frame.cursor_x = frame.last_frame.ds.get_cursor_col();
            frame.cursor_y = frame.last_frame.ds.get_cursor_row();
            frame.current_rendition = *frame.last_frame.ds.get_renditions();
            frame.current_hyperlink = frame.last_frame.ds.get_hyperlink().clone();
        }

        /* is cursor visibility initialized? */
        if !initialized {
            frame.cursor_visible = false;
            frame.append("\x1b[?25l");
        }

        let mut frame_y: i32 = 0;
        let mut blank_row: Option<Rc<Row>> = None;
        let mut rows: rows_type = frame.last_frame.get_rows().clone();
        /* Extend rows if we've gotten a resize and new is wider than old */
        if frame.last_frame.ds.get_width() < f.ds.get_width() {
            for p in rows.iter_mut() {
                let mut r = (**p).clone();
                r.cells.resize(f.ds.get_width() as usize, Cell::new(f.ds.get_background_rendition() as color_type));
                *p = Rc::new(r);
            }
        }
        /* Add rows if we've gotten a resize and new is taller than old */
        if (rows.len() as i32) < f.ds.get_height() {
            // get a proper blank row
            let w = f.ds.get_width() as usize;
            let c: color_type = 0;
            let b = Rc::new(Row::new(w, c));
            rows.resize(f.ds.get_height() as usize, b.clone());
            blank_row = Some(b);
        }

        /* shortcut -- has display moved up by a certain number of lines? */
        if initialized {
            let mut lines_scrolled: i32 = 0;
            let mut scroll_height: i32 = 0;

            for row in 0..f.ds.get_height() {
                let new_row: &Row = f.get_row(0);
                let old_row: &Row = &rows[row as usize];
                if !(std::ptr::eq(new_row, old_row) || *new_row == *old_row) {
                    continue;
                }
                /* if row 0, we're looking at ourselves and probably didn't scroll */
                if row == 0 {
                    break;
                }
                /* found a scroll */
                lines_scrolled = row;
                scroll_height = 1;

                /* how big is the region that was scrolled? */
                let mut region_height = 1;
                while lines_scrolled + region_height < f.ds.get_height() {
                    if *f.get_row(region_height) == *rows[(lines_scrolled + region_height) as usize] {
                        scroll_height = region_height + 1;
                    } else {
                        break;
                    }
                    region_height += 1;
                }

                break;
            }

            if scroll_height != 0 {
                frame_y = scroll_height;

                if lines_scrolled != 0 {
                    /* Now we need a proper blank row. */
                    if blank_row.is_none() {
                        let w = f.ds.get_width() as usize;
                        let c: color_type = 0;
                        blank_row = Some(Rc::new(Row::new(w, c)));
                    }
                    frame.update_rendition(&initial_rendition(), true);
                    frame.update_hyperlink(&Hyperlink::default(), true);

                    let top_margin: i32 = 0;
                    let bottom_margin: i32 = top_margin + lines_scrolled + scroll_height - 1;

                    assert!(bottom_margin < f.ds.get_height());

                    /* Common case:  if we're already on the bottom line and we're scrolling the whole
                     * screen, just do a CR and LFs.
                     */
                    if scroll_height + lines_scrolled == f.ds.get_height() && frame.cursor_y + 1 == f.ds.get_height() {
                        frame.append_char('\r');
                        frame.append_n(lines_scrolled as usize, '\n');
                        frame.cursor_x = 0;
                    } else {
                        /* set scrolling region */
                        let tmp = format!("\x1b[{};{}r", top_margin + 1, bottom_margin + 1);
                        frame.append(&tmp);

                        /* go to bottom of scrolling region */
                        frame.cursor_x = -1;
                        frame.cursor_y = -1;
                        frame.append_silent_move(bottom_margin, 0);

                        /* scroll */
                        frame.append_n(lines_scrolled as usize, '\n');

                        /* reset scrolling region */
                        frame.append("\x1b[r");
                        /* invalidate cursor position after unsetting scrolling region */
                        frame.cursor_x = -1;
                        frame.cursor_y = -1;
                    }

                    /* do the move in our local index */
                    for i in top_margin..=bottom_margin {
                        if i + lines_scrolled <= bottom_margin {
                            rows[i as usize] = rows[(i + lines_scrolled) as usize].clone();
                        } else {
                            rows[i as usize] = blank_row.clone().expect("blank row");
                        }
                    }
                }
            }
        }

        /* Now update the display, row by row */
        let mut wrap = false;
        while frame_y < f.ds.get_height() {
            wrap = self.put_row(initialized, &mut frame, f, frame_y, &rows[frame_y as usize], wrap);
            frame_y += 1;
        }

        /* has cursor location changed? */
        if (!initialized) || (f.ds.get_cursor_row() != frame.cursor_y) || (f.ds.get_cursor_col() != frame.cursor_x) {
            frame.append_move(f.ds.get_cursor_row(), f.ds.get_cursor_col());
        }

        /* has cursor visibility changed? */
        if (!initialized) || (f.ds.cursor_visible != frame.cursor_visible) {
            if f.ds.cursor_visible {
                frame.append("\x1b[?25h");
            } else {
                frame.append("\x1b[?25l");
            }
        }

        /* have renditions changed? */
        frame.update_rendition(f.ds.get_renditions(), !initialized);
        /* has hyperlink changed? */
        frame.update_hyperlink(f.ds.get_hyperlink(), !initialized);

        /* has bracketed paste mode changed? */
        if (!initialized) || (f.ds.bracketed_paste != frame.last_frame.ds.bracketed_paste) {
            frame.append(if f.ds.bracketed_paste { "\x1b[?2004h" } else { "\x1b[?2004l" });
        }

        /* has mouse reporting mode changed? */
        if (!initialized) || (f.ds.mouse_reporting_mode != frame.last_frame.ds.mouse_reporting_mode) {
            if f.ds.mouse_reporting_mode == MOUSE_REPORTING_NONE {
                frame.append("\x1b[?1003l");
                frame.append("\x1b[?1002l");
                frame.append("\x1b[?1001l");
                frame.append("\x1b[?1000l");
            } else {
                if frame.last_frame.ds.mouse_reporting_mode != MOUSE_REPORTING_NONE {
                    let tmp = format!("\x1b[?{}l", frame.last_frame.ds.mouse_reporting_mode as i32);
                    frame.append(&tmp);
                }
                let tmp = format!("\x1b[?{}h", f.ds.mouse_reporting_mode as i32);
                frame.append(&tmp);
            }
        }

        /* has mouse focus mode changed? */
        if (!initialized) || (f.ds.mouse_focus_event != frame.last_frame.ds.mouse_focus_event) {
            frame.append(if f.ds.mouse_focus_event { "\x1b[?1004h" } else { "\x1b[?1004l" });
        }

        /* has mouse encoding mode changed? */
        if (!initialized) || (f.ds.mouse_encoding_mode != frame.last_frame.ds.mouse_encoding_mode) {
            if f.ds.mouse_encoding_mode == MOUSE_ENCODING_DEFAULT {
                frame.append("\x1b[?1015l");
                frame.append("\x1b[?1006l");
                frame.append("\x1b[?1005l");
            } else {
                if frame.last_frame.ds.mouse_encoding_mode != MOUSE_ENCODING_DEFAULT {
                    let tmp = format!("\x1b[?{}l", frame.last_frame.ds.mouse_encoding_mode as i32);
                    frame.append(&tmp);
                }
                let tmp = format!("\x1b[?{}h", f.ds.mouse_encoding_mode as i32);
                frame.append(&tmp);
            }
        }

        frame.str
    }

    fn put_row(
        &self,
        initialized: bool,
        frame: &mut FrameState,
        f: &Framebuffer,
        frame_y: i32,
        old_row: &Row,
        wrap: bool,
    ) -> bool {
        let mut frame_x: i32 = 0;

        let row: &Row = f.get_row(frame_y);
        let cells = &row.cells;
        let old_cells = &old_row.cells;

        /* If we're forced to write the first column because of wrap, go ahead and do so. */
        if wrap {
            let cell = &cells[0];
            frame.update_rendition(cell.get_renditions(), false);
            frame.update_hyperlink(cell.get_hyperlink(), false);
            frame.append_cell(cell);
            frame_x += cell.get_width() as i32;
            frame.cursor_x += cell.get_width() as i32;
        }

        /* If rows are the same object, we don't need to do anything at all. */
        if initialized && std::ptr::eq(row, old_row) {
            return false;
        }

        let wrap_this = row.get_wrap();
        let row_width = f.ds.get_width();
        let mut clear_count: i32 = 0;
        let mut wrote_last_cell = false;
        let mut blank_renditions = initial_rendition();
        let mut blank_hyperlink = Hyperlink::default();

        /* iterate for every cell */
        while frame_x < row_width {
            let cell = &cells[frame_x as usize];

            /* Does cell need to be drawn?  Skip all this. */
            if initialized && clear_count == 0 && (*cell == old_cells[frame_x as usize]) {
                frame_x += cell.get_width() as i32;
                continue;
            }

            /* Slurp up all the empty cells */
            if cell.empty() {
                if clear_count == 0 {
                    blank_renditions = *cell.get_renditions();
                    blank_hyperlink = cell.get_hyperlink().clone();
                }
                if *cell.get_renditions() == blank_renditions && *cell.get_hyperlink() == blank_hyperlink {
                    /* Remember run of blank cells */
                    clear_count += 1;
                    frame_x += 1;
                    continue;
                }
            }

            /* Clear or write cells within the row (not to end). */
            if clear_count != 0 {
                /* Move to the right position. */
                frame.append_silent_move(frame_y, frame_x - clear_count);
                frame.update_rendition(&blank_renditions, false);
                frame.update_hyperlink(&blank_hyperlink, false);
                if self.can_use_erase(frame) && self.has_ech && clear_count > 4 {
                    let tmp = format!("\x1b[{}X", clear_count);
                    frame.append(&tmp);
                } else {
                    frame.append_n(clear_count as usize, ' ');
                    frame.cursor_x = frame_x;
                }
                clear_count = 0;
                // If the current character is *another* empty cell in a different rendition,
                // we restart counting and continue here
                if cell.empty() {
                    blank_renditions = *cell.get_renditions();
                    blank_hyperlink = cell.get_hyperlink().clone();
                    clear_count = 1;
                    frame_x += 1;
                    continue;
                }
            }

            /* Now draw a character cell. */
            /* Move to the right position. */
            let cell_width = cell.get_width() as i32;
            /* If we are about to print the last character in a wrapping row,
            trash the cursor position to force explicit positioning.  We do
            this because our input terminal state may have the cursor on
            the autowrap column ("column 81"), but our output terminal
            states always snap the cursor to the true last column ("column
            80"), and we want to be able to apply the diff to either, for
            verification. */
            if wrap_this && frame_x + cell_width >= row_width {
                frame.cursor_x = -1;
                frame.cursor_y = -1;
            }
            frame.append_silent_move(frame_y, frame_x);
            frame.update_rendition(cell.get_renditions(), false);
            frame.update_hyperlink(cell.get_hyperlink(), false);
            frame.append_cell(cell);
            frame_x += cell_width;
            frame.cursor_x += cell_width;
            if frame_x >= row_width {
                wrote_last_cell = true;
            }
        }

        /* End of line. */

        /* Clear or write empty cells at EOL. */
        if clear_count != 0 {
            /* Move to the right position. */
            frame.append_silent_move(frame_y, frame_x - clear_count);
            frame.update_rendition(&blank_renditions, false);
            frame.update_hyperlink(&blank_hyperlink, false);

            if self.can_use_erase(frame) && !wrap_this {
                frame.append("\x1b[K");
            } else {
                frame.append_n(clear_count as usize, ' ');
                frame.cursor_x = frame_x;
                wrote_last_cell = true;
            }
        }

        if !(wrote_last_cell && (frame_y < f.ds.get_height() - 1)) {
            return false;
        }
        /* To hint that a word-select should group the end of one line
        with the beginning of the next, we let the real cursor
        actually wrap around in cases where it wrapped around for us. */
        if wrap_this {
            /* Update our cursor, and ask for wrap on the next row. */
            frame.cursor_x = 0;
            frame.cursor_y += 1;
            return true;
        }
        /* Resort to CR/LF and update our cursor. */
        frame.append("\r\n");
        frame.cursor_x = 0;
        frame.cursor_y += 1;
        false
    }

    fn can_use_erase(&self, frame: &FrameState) -> bool {
        self.has_bce || (frame.current_rendition == initial_rendition() && frame.current_hyperlink.empty())
    }
}
