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
// Ported from: upstream/src/terminal/terminalfunctions.cc

//! Every CSI, ESC, control and OSC function the emulator implements. Port of upstream `src/terminal/terminalfunctions.cc`.

use crate::parseraction::{wchar_t, Action};
use crate::terminaldispatcher::Function_Type::{CONTROL, CSI, ESCAPE};
use crate::terminaldispatcher::{DispatchRegistry, Dispatcher, Function_Type};
use crate::terminalframebuffer::{title_type, Framebuffer, Hyperlink, Renditions};
use crate::terminalframebuffer::{MouseEncodingMode, MouseReportingMode};
use crate::terminalframebuffer::{MOUSE_ENCODING_DEFAULT, MOUSE_REPORTING_NONE};

/* Terminal functions -- routines activated by CSI, escape or a control char */

fn clearline(fb: &mut Framebuffer, row: i32, start: i32, end: i32) {
    for col in start..=end {
        fb.reset_cell(row, col);
    }
}

/* erase in line */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_EL(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    match dispatch.getparam(0, 0) {
        0 => {
            /* default: active position to end of line, inclusive */
            let (c, w) = (fb.ds.get_cursor_col(), fb.ds.get_width());
            clearline(fb, -1, c, w - 1);
        }
        1 => {
            /* start of screen to active position, inclusive */
            let c = fb.ds.get_cursor_col();
            clearline(fb, -1, 0, c);
        }
        2 => {
            /* all of line */
            fb.reset_row(-1);
        }
        _ => {}
    }
}

/* erase in display */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_ED(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    match dispatch.getparam(0, 0) {
        0 => {
            /* active position to end of screen, inclusive */
            let (c, w) = (fb.ds.get_cursor_col(), fb.ds.get_width());
            clearline(fb, -1, c, w - 1);
            for y in fb.ds.get_cursor_row() + 1..fb.ds.get_height() {
                fb.reset_row(y);
            }
        }
        1 => {
            /* start of screen to active position, inclusive */
            for y in 0..fb.ds.get_cursor_row() {
                fb.reset_row(y);
            }
            let c = fb.ds.get_cursor_col();
            clearline(fb, -1, 0, c);
        }
        2 => {
            /* entire screen */
            for y in 0..fb.ds.get_height() {
                fb.reset_row(y);
            }
        }
        _ => {}
    }
}

/* cursor movement -- relative and absolute */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_cursormove(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let num = dispatch.getparam(0, 1);

    match dispatch.get_dispatch_chars()[0] {
        b'A' => fb.ds.move_row(-num, true),
        b'B' => fb.ds.move_row(num, true),
        b'C' => fb.ds.move_col(num, true, false),
        b'D' => fb.ds.move_col(-num, true, false),
        b'H' | b'f' => {
            fb.ds.move_row(dispatch.getparam(0, 1) - 1, false);
            fb.ds.move_col(dispatch.getparam(1, 1) - 1, false, false);
        }
        _ => {}
    }
}

/* device attributes */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DA(_fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    dispatch.terminal_to_host.extend_from_slice(b"\x1b[?62c"); /* plain vt220 */
}

/* secondary device attributes */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_SDA(_fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    dispatch
        .terminal_to_host
        .extend_from_slice(b"\x1b[>1;10;0c"); /* plain vt220 */
}

/* screen alignment diagnostic */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Esc_DECALN(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    for y in 0..fb.ds.get_height() {
        for x in 0..fb.ds.get_width() {
            fb.reset_cell(y, x);
            fb.get_mutable_cell(y, x).append(b'E' as wchar_t);
        }
    }
}

/* line feed */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_LF(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.move_rows_autoscroll(1);
}

/* carriage return */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_CR(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ds.move_col(0, false, false);
}

/* backspace */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_BS(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ds.move_col(-1, true, false);
}

/* reverse index -- like a backwards line feed */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_RI(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.move_rows_autoscroll(-1);
}

/* newline */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_NEL(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ds.move_col(0, false, false);
    fb.move_rows_autoscroll(1);
}

/* horizontal tab */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn HT_n(fb: &mut Framebuffer, count: i32) {
    let mut col = fb.ds.get_next_tab(count);
    if col == -1 {
        /* no tabs, go to end of line */
        col = fb.ds.get_width() - 1;
    }

    /* A horizontal tab is the only operation that preserves but
    does not set the wrap state. It also starts a new grapheme. */

    let wrap_state_save = fb.ds.next_print_will_wrap;
    fb.ds.move_col(col, false, false);
    fb.ds.next_print_will_wrap = wrap_state_save;
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_HT(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    HT_n(fb, 1);
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_CxT(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let mut param = dispatch.getparam(0, 1);
    if dispatch.get_dispatch_chars()[0] == b'Z' {
        param = -param;
    }
    if param == 0 {
        return;
    }
    HT_n(fb, param);
}

/* horizontal tab set */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_HTS(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ds.set_tab();
}

/* tabulation clear */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_TBC(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let param = dispatch.getparam(0, 0);
    match param {
        0 => {
            /* clear this tab stop */
            let c = fb.ds.get_cursor_col();
            fb.ds.clear_tab(c);
        }
        3 => {
            /* clear all tab stops */
            fb.ds.clear_default_tabs();
            for x in 0..fb.ds.get_width() {
                fb.ds.clear_tab(x);
            }
        }
        _ => {}
    }
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn get_DEC_mode(param: i32, fb: &mut Framebuffer) -> Option<&mut bool> {
    match param {
        1 => {
            /* cursor key mode */
            Some(&mut fb.ds.application_mode_cursor_keys)
        }
        3 => {
            /* 80/132. Ignore but clear screen. */
            /* clear screen */
            fb.ds.move_row(0, false);
            fb.ds.move_col(0, false, false);
            for y in 0..fb.ds.get_height() {
                fb.reset_row(y);
            }
            None
        }
        5 => {
            /* reverse video */
            Some(&mut fb.ds.reverse_video)
        }
        6 => {
            /* origin */
            fb.ds.move_row(0, false);
            fb.ds.move_col(0, false, false);
            Some(&mut fb.ds.origin_mode)
        }
        7 => {
            /* auto wrap */
            Some(&mut fb.ds.auto_wrap_mode)
        }
        25 => Some(&mut fb.ds.cursor_visible),
        1004 => {
            /* xterm mouse focus event */
            Some(&mut fb.ds.mouse_focus_event)
        }
        1007 => {
            /* xterm mouse alternate scroll */
            Some(&mut fb.ds.mouse_alternate_scroll)
        }
        2004 => {
            /* bracketed paste */
            Some(&mut fb.ds.bracketed_paste)
        }
        // mosh-rs scrollback (PLAN.md 6.2, 9.4): upstream ignores the
        // alternate-screen modes (default: NULL). Only a flag is recorded; the
        // screen is not switched, so nothing upstream observes changes.
        47 | 1047 | 1049 => Some(&mut fb.ds.alternate_screen),
        _ => None,
    }
}

/* helper for CSI_DECSM and CSI_DECRM */
fn set_if_available(mode: Option<&mut bool>, value: bool) {
    if let Some(mode) = mode {
        *mode = value;
    }
}

fn mouse_reporting_mode(param: i32) -> MouseReportingMode {
    match param {
        9 => MouseReportingMode::MOUSE_REPORTING_X10,
        1000 => MouseReportingMode::MOUSE_REPORTING_VT220,
        1001 => MouseReportingMode::MOUSE_REPORTING_VT220_HILIGHT,
        1002 => MouseReportingMode::MOUSE_REPORTING_BTN_EVENT,
        1003 => MouseReportingMode::MOUSE_REPORTING_ANY_EVENT,
        _ => unreachable!(),
    }
}

fn mouse_encoding_mode(param: i32) -> MouseEncodingMode {
    match param {
        1005 => MouseEncodingMode::MOUSE_ENCODING_UTF8,
        1006 => MouseEncodingMode::MOUSE_ENCODING_SGR,
        1015 => MouseEncodingMode::MOUSE_ENCODING_URXVT,
        _ => unreachable!(),
    }
}

/* set private mode */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DECSM(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let mut i = 0;
    while i < dispatch.param_count() {
        let param = dispatch.getparam(i as usize, 0);
        if param == 9 || (1000..=1003).contains(&param) {
            fb.ds.mouse_reporting_mode = mouse_reporting_mode(param);
        } else if param == 1005 || param == 1006 || param == 1015 {
            fb.ds.mouse_encoding_mode = mouse_encoding_mode(param);
        } else {
            set_if_available(get_DEC_mode(param, fb), true);
        }
        i += 1;
    }
}

/* clear private mode */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DECRM(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let mut i = 0;
    while i < dispatch.param_count() {
        let param = dispatch.getparam(i as usize, 0);
        if param == 9 || (1000..=1003).contains(&param) {
            fb.ds.mouse_reporting_mode = MOUSE_REPORTING_NONE;
        } else if param == 1005 || param == 1006 || param == 1015 {
            fb.ds.mouse_encoding_mode = MOUSE_ENCODING_DEFAULT;
        } else {
            set_if_available(get_DEC_mode(param, fb), false);
        }
        i += 1;
    }
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn get_ANSI_mode(param: i32, fb: &mut Framebuffer) -> Option<&mut bool> {
    if param == 4 {
        /* insert/replace mode */
        return Some(&mut fb.ds.insert_mode);
    }
    None
}

/* set mode */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_SM(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let mut i = 0;
    while i < dispatch.param_count() {
        if let Some(mode) = get_ANSI_mode(dispatch.getparam(i as usize, 0), fb) {
            *mode = true;
        }
        i += 1;
    }
}

/* clear mode */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_RM(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let mut i = 0;
    while i < dispatch.param_count() {
        if let Some(mode) = get_ANSI_mode(dispatch.getparam(i as usize, 0), fb) {
            *mode = false;
        }
        i += 1;
    }
}

/* set top and bottom margins */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DECSTBM(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let top = dispatch.getparam(0, 1);
    let bottom = dispatch.getparam(1, fb.ds.get_height());

    if (bottom <= top) || (top > fb.ds.get_height()) || (top == 0 && bottom == 1) {
        return; /* invalid, xterm ignores */
    }

    fb.ds.set_scrolling_region(top - 1, bottom - 1);
    fb.ds.move_row(0, false);
    fb.ds.move_col(0, false, false);
}

/* terminal bell */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Ctrl_BEL(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ring_bell();
}

/* select graphics rendition -- e.g., bold, blinking, etc. */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_SGR(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let mut i: i32 = 0;
    while i < dispatch.param_count() {
        let rendition = dispatch.getparam(i as usize, 0);
        /* We need to special-case the handling of [34]8 ; 5 ; Ps,
        because Ps of 0 in that case does not mean reset to default, even
        though it means that otherwise (as usually renditions are applied
        in order). */
        if (rendition == 38 || rendition == 48)
            && (dispatch.param_count() - i >= 3)
            && (dispatch.getparam((i + 1) as usize, -1) == 5)
        {
            let c = dispatch.getparam((i + 2) as usize, 0);
            if rendition == 38 {
                fb.ds.set_foreground_color(c);
            } else {
                fb.ds.set_background_color(c);
            }
            i += 2;
            i += 1;
            continue;
        }

        /* True color support: ESC[ ... [34]8;2;<r>;<g>;<b> ... m */
        if (rendition == 38 || rendition == 48)
            && (dispatch.param_count() - i >= 5)
            && (dispatch.getparam((i + 1) as usize, -1) == 2)
        {
            // getparam returns 0..=65535 here (default 0), so these casts are lossless.
            let red = dispatch.getparam((i + 2) as usize, 0) as u32;
            let green = dispatch.getparam((i + 3) as usize, 0) as u32;
            let blue = dispatch.getparam((i + 4) as usize, 0) as u32;

            let color = Renditions::make_true_color(red, green, blue);

            // Upstream passes the unsigned color to an int parameter: values above
            // i32::MAX (red >= 32768) become negative, keeping the same bits.
            if rendition == 38 {
                fb.ds.set_foreground_color(color as i32);
            } else {
                fb.ds.set_background_color(color as i32);
            }
            i += 4;
            i += 1;
            continue;
        }

        // rendition is 0..=65535 (getparam default 0), so the cast is lossless.
        fb.ds.add_rendition(rendition as u32);
        i += 1;
    }
}

/* save and restore cursor */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Esc_DECSC(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ds.save_cursor();
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Esc_DECRC(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.ds.restore_cursor();
}

/* device status report -- e.g., cursor position (used by resize) */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DSR(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let param = dispatch.getparam(0, 0);

    match param {
        5 => {
            /* device status report requested */
            dispatch.terminal_to_host.extend_from_slice(b"\x1b[0n");
        }
        6 => {
            /* report of active position requested */
            let cpr = format!(
                "\x1b[{};{}R",
                fb.ds.get_cursor_row() + 1,
                fb.ds.get_cursor_col() + 1
            );
            dispatch.terminal_to_host.extend_from_slice(cpr.as_bytes());
        }
        _ => {}
    }
}

/* insert line */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_IL(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let lines = dispatch.getparam(0, 1);

    let r = fb.ds.get_cursor_row();
    fb.insert_line(r, lines);

    /* vt220 manual and Ecma-48 say to move to first column */
    /* but xterm and gnome-terminal don't */
    fb.ds.move_col(0, false, false);
}

/* delete line */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DL(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let lines = dispatch.getparam(0, 1);

    let r = fb.ds.get_cursor_row();
    fb.delete_line(r, lines);

    /* same story -- xterm and gnome-terminal don't
    move to first column */
    fb.ds.move_col(0, false, false);
}

/* insert characters */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_ICH(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let cells = dispatch.getparam(0, 1);

    for _ in 0..cells {
        let (r, c) = (fb.ds.get_cursor_row(), fb.ds.get_cursor_col());
        fb.insert_cell(r, c);
    }
}

/* delete character */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DCH(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let cells = dispatch.getparam(0, 1);

    for _ in 0..cells {
        let (r, c) = (fb.ds.get_cursor_row(), fb.ds.get_cursor_col());
        fb.delete_cell(r, c);
    }
}

/* line position absolute */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_VPA(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let row = dispatch.getparam(0, 1);
    fb.ds.move_row(row - 1, false);
}

/* character position absolute */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_HPA(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let col = dispatch.getparam(0, 1);
    fb.ds.move_col(col - 1, false, false);
}

/* erase character */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_ECH(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    let num = dispatch.getparam(0, 1);
    let mut limit = fb.ds.get_cursor_col() + num - 1;
    if limit >= fb.ds.get_width() {
        limit = fb.ds.get_width() - 1;
    }

    let c = fb.ds.get_cursor_col();
    clearline(fb, -1, c, limit);
}

/* reset to initial state */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Esc_RIS(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.reset();
}

/* soft reset */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_DECSTR(fb: &mut Framebuffer, _dispatch: &mut Dispatcher) {
    fb.soft_reset();
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn Parse_OSC_8(osc8_vector: &[wchar_t]) -> Option<String> {
    let mut osc8_str = String::with_capacity(osc8_vector.len());
    for &wide_char in osc8_vector {
        // Valid char range is 32-126, per
        // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda#encodings
        if !(32..=126).contains(&wide_char) {
            return None;
        }
        osc8_str.push(wide_char as u8 as char);
    }
    Some(osc8_str)
}

// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn OSC_8(OSC_string: &str, fb: &mut Framebuffer) {
    // OSC of the form "\033]8;params;url\007"
    let s = OSC_string.as_bytes();
    assert!(s[0] == b'8');
    if s.len() <= 2 || s[1] != b';' {
        // Bail early if the string is malformed.
        return;
    }

    let second_semicolon = match s[2..].iter().position(|&c| c == b';') {
        // Missing the second semicolon, malformed.
        None => return,
        Some(p) => p + 2,
    };

    let url = OSC_string[second_semicolon + 1..].to_string();
    fb.ds.set_hyperlink(Hyperlink::new(
        OSC_string[2..second_semicolon].to_string(),
        url,
    ));
}

impl Dispatcher {
    /* xterm uses an Operating System Command to set the window title */
    /// Handles a completed OSC string: clipboard (52;c;), title and icon name (0, 1, 2), hyperlink (8).
    // Upstream name, kept so the port can be compared line by line.
    #[allow(non_snake_case)]
    pub fn OSC_dispatch(&mut self, _act: &Action, fb: &mut Framebuffer) {
        let s = &self.OSC_string;
        let c = |x: char| x as wchar_t;
        /* handle osc copy clipboard sequence 52;c; */
        if s.len() >= 5
            && s[0] == c('5')
            && s[1] == c('2')
            && s[2] == c(';')
            && s[3] == c('c')
            && s[4] == c(';')
        {
            let clipboard: title_type = s[5..].to_vec();
            fb.set_clipboard(&clipboard);
            /* handle osc terminal title sequence */
        } else if !s.is_empty() {
            let mut cmd_num: i64 = -1;
            let mut offset: usize = 0;
            if s[0] == c(';') {
                /* OSC of the form "\033];<title>\007" */
                cmd_num = 0; /* treat it as as a zero */
                offset = 1;
            } else if (s.len() >= 2) && (s[1] == c(';')) {
                /* OSC of the form "\033]X;<title>\007" where X can be:
                 * 0: set icon name and window title
                 * 1: set icon name
                 * 2: set window title */
                // wchar_t arithmetic: a character below '0' wraps to a large unsigned
                // value on the arm64 oracle (wchar_t is unsigned there); either way it
                // is never 0, 1, 2 or 8.
                cmd_num = s[0].wrapping_sub(c('0')) as i64;
                offset = 2;
            }
            if cmd_num == 8 {
                // Handle OSC8 hyperlinks separately
                let osc_8_str = match Parse_OSC_8(s) {
                    None => return,
                    Some(x) => x,
                };
                OSC_8(&osc_8_str, fb);
                return;
            }
            let set_icon = cmd_num == 0 || cmd_num == 1;
            let set_title = cmd_num == 0 || cmd_num == 2;
            if set_icon || set_title {
                fb.set_title_initialized();
                let title_length = std::cmp::min(s.len(), 256);
                let newtitle: title_type = s[offset..title_length].to_vec();
                if set_icon {
                    fb.set_icon_name(&newtitle);
                }
                if set_title {
                    fb.set_window_title(&newtitle);
                }
            }
        }
    }
}

/* scroll down or terminfo indn */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_SD(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    fb.scroll(dispatch.getparam(0, 1));
}

/* scroll up or terminfo rin */
// Upstream name, kept so the port can be compared line by line.
#[allow(non_snake_case)]
fn CSI_SU(fb: &mut Framebuffer, dispatch: &mut Dispatcher) {
    fb.scroll(-dispatch.getparam(0, 1));
}

/// The static `Function` objects of terminalfunctions.cc, in file order.
pub(crate) fn register_all(r: &mut DispatchRegistry) {
    let mut f = |ty: Function_Type,
                 chars: &[u8],
                 func: fn(&mut Framebuffer, &mut Dispatcher),
                 clears: bool| { r.register_function(ty, chars, func, clears) };
    f(CSI, b"K", CSI_EL, true);
    f(CSI, b"J", CSI_ED, true);
    f(CSI, b"A", CSI_cursormove, true);
    f(CSI, b"B", CSI_cursormove, true);
    f(CSI, b"C", CSI_cursormove, true);
    f(CSI, b"D", CSI_cursormove, true);
    f(CSI, b"H", CSI_cursormove, true);
    f(CSI, b"f", CSI_cursormove, true);
    f(CSI, b"c", CSI_DA, true);
    f(CSI, b">c", CSI_SDA, true);
    f(ESCAPE, b"#8", Esc_DECALN, true);
    /* same procedure for index, vertical tab, and form feed control codes */
    f(CONTROL, b"\x0a", Ctrl_LF, true);
    f(CONTROL, b"\x84", Ctrl_LF, true);
    f(CONTROL, b"\x0b", Ctrl_LF, true);
    f(CONTROL, b"\x0c", Ctrl_LF, true);
    f(CONTROL, b"\x0d", Ctrl_CR, true);
    f(CONTROL, b"\x08", Ctrl_BS, true);
    f(CONTROL, b"\x8D", Ctrl_RI, true);
    f(CONTROL, b"\x85", Ctrl_NEL, true);
    f(CONTROL, b"\x09", Ctrl_HT, false);
    f(CSI, b"I", CSI_CxT, false);
    f(CSI, b"Z", CSI_CxT, false);
    f(CONTROL, b"\x88", Ctrl_HTS, true);
    /* TBC preserves wrap state */
    f(CSI, b"g", CSI_TBC, false);
    /* These functions don't clear wrap state. */
    f(CSI, b"?h", CSI_DECSM, false);
    f(CSI, b"?l", CSI_DECRM, false);
    f(CSI, b"h", CSI_SM, true);
    f(CSI, b"l", CSI_RM, true);
    f(CSI, b"r", CSI_DECSTBM, true);
    f(CONTROL, b"\x07", Ctrl_BEL, true);
    f(CSI, b"m", CSI_SGR, false); /* changing renditions doesn't clear wrap flag */
    f(ESCAPE, b"7", Esc_DECSC, true);
    f(ESCAPE, b"8", Esc_DECRC, true);
    f(CSI, b"n", CSI_DSR, true);
    f(CSI, b"L", CSI_IL, true);
    f(CSI, b"M", CSI_DL, true);
    f(CSI, b"@", CSI_ICH, true);
    f(CSI, b"P", CSI_DCH, true);
    f(CSI, b"d", CSI_VPA, true);
    f(CSI, b"G", CSI_HPA, true); /* ECMA-48 name: CHA */
    f(CSI, b"\x60", CSI_HPA, true); /* ECMA-48 name: HPA */
    f(CSI, b"X", CSI_ECH, true);
    f(ESCAPE, b"c", Esc_RIS, true);
    f(CSI, b"!p", CSI_DECSTR, true);
    f(CSI, b"S", CSI_SD, true);
    f(CSI, b"T", CSI_SU, true);
}
