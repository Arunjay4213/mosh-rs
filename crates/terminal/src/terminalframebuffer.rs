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
// Ported from: upstream/src/terminal/terminalframebuffer.h, upstream/src/terminal/terminalframebuffer.cc

//! Screen contents: renditions, cells, rows, draw state and framebuffer. Port of upstream `src/terminal/terminalframebuffer.{h,cc}`.

/* Terminal framebuffer */

use std::io::{self, Write};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::parseraction::wchar_t;

/// A color number as upstream stores it.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
pub type color_type = u32;

/// Mask for a 25-bit bitfield (`uint64_t foreground_color : 25`).
const COLOR_FIELD_MASK: u32 = (1 << 25) - 1;

/// Bit positions in `Renditions::attributes`.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum attribute_type {
    /// SGR 1.
    bold = 0,
    /// Faint (never set by upstream).
    faint = 1,
    /// SGR 3.
    italic = 2,
    /// SGR 4.
    underlined = 3,
    /// SGR 5.
    blink = 4,
    /// SGR 7.
    inverse = 5,
    /// SGR 8.
    invisible = 6,
}

/// Character attributes and colors, with upstream's bitfield widths.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Renditions {
    /// 25-bit field
    foreground_color: u32,
    /// 25-bit field
    background_color: u32,
    /// 8-bit field
    attributes: u8,
}

impl Renditions {
    const TRUE_COLOR_MASK: u32 = 0x1000000;

    /// Default renditions with the given background color.
    pub fn new(s_background: color_type) -> Renditions {
        Renditions {
            foreground_color: 0,
            background_color: s_background & COLOR_FIELD_MASK,
            attributes: 0,
        }
    }

    /// Packs an RGB color with the true-color flag (bit 24); components are not range-checked, as upstream.
    pub fn make_true_color(r: u32, g: u32, b: u32) -> u32 {
        Self::TRUE_COLOR_MASK | (r << 16) | (g << 8) | b
    }

    /// Whether bit 24 (true color) is set.
    pub fn is_true_color(color: u32) -> bool {
        (color & Self::TRUE_COLOR_MASK) != 0
    }

    /// The background color number.
    pub fn get_background_rendition(&self) -> u32 {
        self.background_color
    }

    /// Debug accessor (not upstream; upstream comments its getter out): the
    /// raw 25-bit foreground field, for the ref-term/vt-check dump.
    pub fn debug_foreground_color(&self) -> u32 {
        self.foreground_color
    }

    /// Debug accessor (not upstream): the raw attribute bits, for the dump.
    pub fn debug_attributes(&self) -> u8 {
        self.attributes
    }

    /// Sets or clears one attribute bit.
    pub fn set_attribute(&mut self, attr: attribute_type, val: bool) {
        self.attributes = if val {
            self.attributes | (1 << attr as u8)
        } else {
            self.attributes & !(1 << attr as u8)
        };
    }

    /// Reads one attribute bit.
    pub fn get_attribute(&self, attr: attribute_type) -> bool {
        (self.attributes & (1 << attr as u8)) != 0
    }

    /// Clears all attribute bits.
    pub fn clear_attributes(&mut self) {
        self.attributes = 0;
    }

    /* This routine cannot be used to set a color beyond the 16-color set. */
    /// Applies one SGR parameter (attributes, 8/16-color, defaults).
    pub fn set_rendition(&mut self, num: color_type) {
        if num == 0 {
            self.clear_attributes();
            self.foreground_color = 0;
            self.background_color = 0;
            return;
        }

        if num == 39 {
            self.foreground_color = 0;
            return;
        } else if num == 49 {
            self.background_color = 0;
            return;
        }

        if (30..=37).contains(&num) {
            /* foreground color in 8-color set */
            self.foreground_color = num;
            return;
        } else if (40..=47).contains(&num) {
            /* background color in 8-color set */
            self.background_color = num;
            return;
        } else if (90..=97).contains(&num) {
            /* foreground color in 16-color set */
            self.foreground_color = num - 90 + 38;
            return;
        } else if (100..=107).contains(&num) {
            /* background color in 16-color set */
            self.background_color = num - 100 + 48;
            return;
        }

        let value = num < 9;
        match num {
            1 | 22 => self.set_attribute(attribute_type::bold, value),
            3 | 23 => self.set_attribute(attribute_type::italic, value),
            4 | 24 => self.set_attribute(attribute_type::underlined, value),
            5 | 25 => self.set_attribute(attribute_type::blink, value),
            7 | 27 => self.set_attribute(attribute_type::inverse, value),
            8 | 28 => self.set_attribute(attribute_type::invisible, value),
            _ => {} /* ignore unknown rendition */
        }
    }

    /// Sets a 256-color index (0-255) or a packed true color; anything else is ignored.
    pub fn set_foreground_color(&mut self, num: i32) {
        if (0..=255).contains(&num) {
            self.foreground_color = (30 + num) as u32;
        } else if Self::is_true_color(num as u32) {
            // num is negative when upstream's unsigned true color (red up to 65535)
            // was passed as int; `as u32` keeps the same bits, and the 25-bit field
            // keeps the low 25 of them, as the C++ bitfield assignment does.
            self.foreground_color = (num as u32) & COLOR_FIELD_MASK;
        }
    }

    /// Sets a 256-color index (0-255) or a packed true color; anything else is ignored.
    pub fn set_background_color(&mut self, num: i32) {
        if (0..=255).contains(&num) {
            self.background_color = (40 + num) as u32;
        } else if Self::is_true_color(num as u32) {
            // num is negative when upstream's unsigned true color (red up to 65535)
            // was passed as int; `as u32` keeps the same bits, and the 25-bit field
            // keeps the low 25 of them, as the C++ bitfield assignment does.
            self.background_color = (num as u32) & COLOR_FIELD_MASK;
        }
    }

    /// The SGR sequence that selects exactly these renditions, starting from a reset.
    pub fn sgr(&self) -> String {
        let mut ret = String::new();

        ret.push_str("\x1b[0");
        if self.get_attribute(attribute_type::bold) {
            ret.push_str(";1");
        }
        if self.get_attribute(attribute_type::italic) {
            ret.push_str(";3");
        }
        if self.get_attribute(attribute_type::underlined) {
            ret.push_str(";4");
        }
        if self.get_attribute(attribute_type::blink) {
            ret.push_str(";5");
        }
        if self.get_attribute(attribute_type::inverse) {
            ret.push_str(";7");
        }
        if self.get_attribute(attribute_type::invisible) {
            ret.push_str(";8");
        }

        let fg = self.foreground_color;
        if fg != 0 {
            if Self::is_true_color(fg) {
                ret.push_str(&format!(
                    ";38;2;{};{};{}",
                    (fg >> 16) & 0xff,
                    (fg >> 8) & 0xff,
                    fg & 0xff
                ));
            } else if fg > 37 {
                /* use 256-color set */
                ret.push_str(&format!(";38;5;{}", fg as i32 - 30));
            } else {
                /* ANSI foreground color */
                ret.push_str(&format!(";{}", fg));
            }
        }
        let bg = self.background_color;
        if bg != 0 {
            if Self::is_true_color(bg) {
                ret.push_str(&format!(
                    ";48;2;{};{};{}",
                    (bg >> 16) & 0xff,
                    (bg >> 8) & 0xff,
                    bg & 0xff
                ));
            } else if bg > 47 {
                /* use 256-color set */
                ret.push_str(&format!(";48;5;{}", bg as i32 - 40));
            } else {
                /* ANSI background color */
                ret.push_str(&format!(";{}", bg));
            }
        }
        ret.push('m');

        ret
    }
}

/// The contents of an OSC 8 hyperlink.
#[derive(Clone, Debug, PartialEq, Eq)]
struct HyperlinkRep {
    /// OSC 8 parameters (e.g. "id=x").
    params: String,
    /// The URL.
    url: String,
}

/// OSC 8 hyperlink. `rep` is null (None) when the URL is empty.
/// Equality compares contents (upstream compares pointers first, then contents).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Hyperlink {
    /// None when there is no link.
    rep: Option<Rc<HyperlinkRep>>,
}

impl Hyperlink {
    /// A hyperlink; an empty URL means no link.
    pub fn new(params: String, url: String) -> Hyperlink {
        Hyperlink {
            rep: if url.is_empty() {
                None
            } else {
                Some(Rc::new(HyperlinkRep { params, url }))
            },
        }
    }

    /// Whether there is no link.
    pub fn empty(&self) -> bool {
        self.rep.is_none()
    }

    /// Debug accessor (not upstream): (params, url), or None for no link.
    pub fn debug_parts(&self) -> Option<(&str, &str)> {
        self.rep
            .as_ref()
            .map(|r| (r.params.as_str(), r.url.as_str()))
    }

    /// The OSC 8 sequence that selects this link (or ends any link).
    pub fn osc8(&self) -> String {
        let mut ret = String::new();

        ret.push_str("\x1b]8;");

        if let Some(rep) = &self.rep {
            ret.push_str(&rep.params);
        }
        ret.push(';');
        if let Some(rep) = &self.rep {
            ret.push_str(&rep.url);
        }

        ret.push_str("\x1b\\");
        ret
    }
}

/// One screen cell: grapheme bytes, renditions, link and flags.
#[derive(Clone, Debug)]
pub struct Cell {
    /// UTF-8 bytes of the grapheme (base character plus combining characters).
    contents: String,
    /// The cell's renditions.
    renditions: Renditions,
    /// The cell's hyperlink.
    hyperlink: Hyperlink,
    /// 0 = narrow, 1 = wide
    wide: bool,
    /// first character is combining character
    fallback: bool,
    /// Set on a row's last cell when the row wrapped into the next one.
    wrap: bool,
}

impl PartialEq for Cell {
    fn eq(&self, x: &Cell) -> bool {
        (self.contents == x.contents)
            && (self.fallback == x.fallback)
            && (self.wide == x.wide)
            && (self.renditions == x.renditions)
            && (self.hyperlink == x.hyperlink)
            && (self.wrap == x.wrap)
    }
}

/// UTF-8 encoding of a code point, as glibc `wcrtomb()` gives it in a UTF-8
/// locale. The parser only ever produces Unicode scalar values.
fn push_wchar(dest: &mut String, c: wchar_t) {
    dest.push(char::from_u32(c).expect("wcrtomb: not a Unicode scalar value"));
}

impl Cell {
    /// An empty cell with the given background color.
    pub fn new(background_color: color_type) -> Cell {
        Cell {
            contents: String::new(),
            renditions: Renditions::new(background_color),
            hyperlink: Hyperlink::default(),
            wide: false,
            fallback: false,
            wrap: false,
        }
    }

    /// Empties the cell and resets everything but the background color.
    pub fn reset(&mut self, background_color: color_type) {
        self.contents.clear();
        self.renditions = Renditions::new(background_color);
        self.hyperlink = Hyperlink::default();
        self.wide = false;
        self.fallback = false;
        self.wrap = false;
    }

    /// Whether the cell has no contents.
    pub fn empty(&self) -> bool {
        self.contents.is_empty()
    }

    /* 32 seems like a reasonable limit on combining characters */
    /// Whether the contents reached the 32-byte combining limit.
    pub fn full(&self) -> bool {
        self.contents.len() >= 32
    }

    /// Removes the contents.
    pub fn clear(&mut self) {
        self.contents.clear();
    }

    /// Debug accessor (not upstream): the raw UTF-8 bytes of the contents.
    pub fn debug_contents_bytes(&self) -> &[u8] {
        self.contents.as_bytes()
    }

    /// Empty, a space or a no-break space.
    pub fn is_blank(&self) -> bool {
        // XXX fix.
        self.contents.is_empty() || self.contents == " " || self.contents == "\u{A0}"
    }

    /// Same contents, treating all blanks as equal.
    pub fn contents_match(&self, other: &Cell) -> bool {
        (self.is_blank() && other.is_blank()) || (self.contents == other.contents)
    }

    // Is this a printing ISO 8859-1 character?
    /// Whether `c` is a printing ISO 8859-1 character (always width 1).
    pub fn isprint_iso8859_1(c: wchar_t) -> bool {
        (0xa0..=0xff).contains(&c) || (0x20..=0x7e).contains(&c)
    }

    /// Appends `c` as UTF-8, as glibc `wcrtomb` does.
    pub fn append_to_str(dest: &mut String, c: wchar_t) {
        /* ASCII?  Cheat. */
        if c <= 0x7f {
            dest.push(c as u8 as char);
            return;
        }
        push_wchar(dest, c);
    }

    /// Appends a code point to the contents.
    pub fn append(&mut self, c: wchar_t) {
        /* ASCII?  Cheat. */
        if c <= 0x7f {
            self.contents.push(c as u8 as char);
            return;
        }
        push_wchar(&mut self.contents, c);
    }

    /// Appends what to print for this cell (a space if empty; a no-break space before a leading combining character).
    pub fn print_grapheme(&self, output: &mut String) {
        if self.contents.is_empty() {
            output.push(' ');
            return;
        }
        /*
         * cells that begin with combining character get combiner
         * attached to no-break space
         */
        if self.fallback {
            output.push('\u{A0}');
        }
        output.push_str(&self.contents);
    }

    /// Debug form of the contents: `'_' ()` when empty, else the grapheme and its bytes.
    pub fn debug_contents(&self) -> String {
        if self.contents.is_empty() {
            return "'_' ()".to_string();
        }
        let mut chars = String::from("'");
        self.print_grapheme(&mut chars);
        chars.push_str("' [");
        let mut lazycomma = "";
        for b in self.contents.bytes() {
            chars.push_str(&format!("{}0x{:02x}", lazycomma, b));
            lazycomma = ", ";
        }
        chars.push(']');
        chars
    }

    /// Diagnostics: prints differences to stderr; returns true if the
    /// grapheme, width, renditions or wrap differ (contents and fallback
    /// differences are printed but, as upstream, do not count). Failed
    /// writes are ignored, as C's `fprintf` ignores them (this crate has no
    /// platform dependency, so it writes to stderr directly rather than
    /// through `mosh_platform::errln!`).
    pub fn compare(&self, other: &Cell) -> bool {
        let mut ret = false;

        let mut grapheme = String::new();
        let mut other_grapheme = String::new();

        self.print_grapheme(&mut grapheme);
        other.print_grapheme(&mut other_grapheme);

        if grapheme != other_grapheme {
            ret = true;
            let _ = writeln!(
                io::stderr(),
                "Graphemes: '{}' vs. '{}'",
                grapheme,
                other_grapheme
            );
        }

        if !self.contents_match(other) {
            // ret = true;
            let _ = writeln!(
                io::stderr(),
                "Contents: {} ({}) vs. {} ({})",
                self.debug_contents(),
                self.contents.len(),
                other.debug_contents(),
                other.contents.len()
            );
        }

        if self.fallback != other.fallback {
            // ret = true;
            let _ = writeln!(
                io::stderr(),
                "fallback: {} vs. {}",
                self.fallback as i32,
                other.fallback as i32
            );
        }

        if self.wide != other.wide {
            ret = true;
            let _ = writeln!(
                io::stderr(),
                "width: {} vs. {}",
                self.wide as i32,
                other.wide as i32
            );
        }

        if !(self.renditions == other.renditions) {
            ret = true;
            let _ = writeln!(io::stderr(), "renditions differ");
        }

        if self.wrap != other.wrap {
            ret = true;
            let _ = writeln!(
                io::stderr(),
                "wrap: {} vs. {}",
                self.wrap as i32,
                other.wrap as i32
            );
        }

        ret
    }

    /* Other accessors */
    /// The hyperlink.
    pub fn get_hyperlink(&self) -> &Hyperlink {
        &self.hyperlink
    }
    /// Sets the hyperlink.
    pub fn set_hyperlink(&mut self, l: Hyperlink) {
        self.hyperlink = l;
    }
    /// The renditions.
    pub fn get_renditions(&self) -> &Renditions {
        &self.renditions
    }
    /// The renditions, mutable.
    pub fn get_renditions_mut(&mut self) -> &mut Renditions {
        &mut self.renditions
    }
    /// Sets the renditions.
    pub fn set_renditions(&mut self, r: Renditions) {
        self.renditions = r;
    }
    /// Whether the cell is double width.
    pub fn get_wide(&self) -> bool {
        self.wide
    }
    /// Sets double width.
    pub fn set_wide(&mut self, w: bool) {
        self.wide = w;
    }
    /// 1 or 2 columns.
    pub fn get_width(&self) -> u32 {
        self.wide as u32 + 1
    }
    /// Whether the contents start with a combining character.
    pub fn get_fallback(&self) -> bool {
        self.fallback
    }
    /// Sets the fallback flag.
    pub fn set_fallback(&mut self, f: bool) {
        self.fallback = f;
    }
    /// The wrap flag.
    pub fn get_wrap(&self) -> bool {
        self.wrap
    }
    /// Sets the wrap flag.
    pub fn set_wrap(&mut self, f: bool) {
        self.wrap = f;
    }
}

/// Index for a row, column or tab position. Upstream indexes with `int`; the
/// value is never negative here because the cursor, the scrolling region and
/// every row/column argument are kept inside the screen (snap_cursor_to_border,
/// set_scrolling_region, and callers passing cursor positions). A negative
/// value would wrap to a huge index and panic on the bounds check, where
/// upstream's `at()` throws.
fn idx(i: i32) -> usize {
    i as usize
}

/// Process-wide row generation counter (upstream: a function-local static).
static GEN_COUNTER: AtomicU64 = AtomicU64::new(0);

/// One screen row.
#[derive(Clone, Debug)]
pub struct Row {
    /// The cells, one per column.
    pub cells: Vec<Cell>,
    // gen is a generation counter.  It can be used to quickly rule
    // out the possibility of two rows being identical; this is useful
    // in scrolling.
    /// Generation number from a process-wide counter; part of row equality, used by scroll detection.
    pub gen: u64,
}

impl PartialEq for Row {
    fn eq(&self, x: &Row) -> bool {
        self.gen == x.gen && self.cells == x.cells
    }
}

impl Row {
    /// A blank row with a fresh generation number.
    pub fn new(s_width: usize, background_color: color_type) -> Row {
        Row {
            cells: vec![Cell::new(background_color); s_width],
            gen: Row::get_gen(),
        }
    }

    /// Inserts a blank cell at `col`, dropping the last cell.
    pub fn insert_cell(&mut self, col: i32, background_color: color_type) {
        self.cells.insert(idx(col), Cell::new(background_color));
        self.cells.pop();
    }

    /// Deletes the cell at `col`, appending a blank cell.
    pub fn delete_cell(&mut self, col: i32, background_color: color_type) {
        self.cells.push(Cell::new(background_color));
        self.cells.remove(idx(col));
    }

    /// Blanks every cell and takes a fresh generation number.
    pub fn reset(&mut self, background_color: color_type) {
        self.gen = Row::get_gen();
        for i in self.cells.iter_mut() {
            i.reset(background_color);
        }
    }

    /// The last cell's wrap flag.
    pub fn get_wrap(&self) -> bool {
        self.cells.last().expect("row has cells").get_wrap()
    }
    /// Sets the last cell's wrap flag.
    pub fn set_wrap(&mut self, w: bool) {
        self.cells.last_mut().expect("row has cells").set_wrap(w);
    }

    /// Takes the next generation number.
    pub fn get_gen() -> u64 {
        GEN_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

/// State saved by DECSC and restored by DECRC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SavedCursor {
    /// Saved column.
    pub cursor_col: i32,
    /// Saved row.
    pub cursor_row: i32,
    /// Saved renditions.
    pub renditions: Renditions,
    /* not implemented: character set shift state */
    /// Saved autowrap mode.
    pub auto_wrap_mode: bool,
    /// Saved origin mode.
    pub origin_mode: bool,
    /* not implemented: state of selective erase */
}

impl Default for SavedCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl SavedCursor {
    /// The state DECRC restores when nothing was saved.
    pub fn new() -> SavedCursor {
        SavedCursor {
            cursor_col: 0,
            cursor_row: 0,
            renditions: Renditions::new(0),
            auto_wrap_mode: true,
            origin_mode: false,
        }
    }
}

/// Mouse reporting mode, numbered as its DEC private mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
// Upstream variant names (DrawState::MOUSE_*), kept for line-by-line comparison.
#[allow(non_camel_case_types)]
pub enum MouseReportingMode {
    /// Off.
    MOUSE_REPORTING_NONE = 0,
    /// Mode 9.
    MOUSE_REPORTING_X10 = 9,
    /// Mode 1000.
    MOUSE_REPORTING_VT220 = 1000,
    /// Mode 1001.
    MOUSE_REPORTING_VT220_HILIGHT = 1001,
    /// Mode 1002.
    MOUSE_REPORTING_BTN_EVENT = 1002,
    /// Mode 1003.
    MOUSE_REPORTING_ANY_EVENT = 1003,
}

/// Mouse report encoding, numbered as its DEC private mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
// Upstream variant names (DrawState::MOUSE_*), kept for line-by-line comparison.
#[allow(non_camel_case_types)]
pub enum MouseEncodingMode {
    /// X10 encoding.
    MOUSE_ENCODING_DEFAULT = 0,
    /// Mode 1005.
    MOUSE_ENCODING_UTF8 = 1005,
    /// Mode 1006.
    MOUSE_ENCODING_SGR = 1006,
    /// Mode 1015.
    MOUSE_ENCODING_URXVT = 1015,
}

pub use MouseEncodingMode::*;
pub use MouseReportingMode::*;

/// Cursor, modes, tabs, scrolling region and current renditions.
#[derive(Clone, Debug)]
pub struct DrawState {
    /// Columns.
    width: i32,
    /// Rows.
    height: i32,

    /// Cursor column.
    cursor_col: i32,
    /// Cursor row.
    cursor_row: i32,
    /// Column of the cell a following combining character attaches to (-1 after a resize made it invalid).
    combining_char_col: i32,
    /// Row of that cell.
    combining_char_row: i32,

    /// Whether tab stops are still the default every-8-columns (cleared by TBC 3).
    default_tabs: bool,
    /// Tab stop per column.
    tabs: Vec<bool>,

    /// Top row of the scrolling region.
    scrolling_region_top_row: i32,
    /// Bottom row of the scrolling region.
    scrolling_region_bottom_row: i32,

    /// Renditions for newly printed characters.
    renditions: Renditions,
    /// Hyperlink for newly printed characters.
    hyperlink: Hyperlink,

    /// DECSC state.
    save: SavedCursor,

    /// The cursor is past the last column: the next printed character wraps first.
    pub next_print_will_wrap: bool,
    /// DECOM.
    pub origin_mode: bool,
    /// DECAWM.
    pub auto_wrap_mode: bool,
    /// IRM.
    pub insert_mode: bool,
    /// DECTCEM.
    pub cursor_visible: bool,
    /// DECSCNM.
    pub reverse_video: bool,
    /// Mode 2004.
    pub bracketed_paste: bool,

    /// Mouse reporting mode.
    pub mouse_reporting_mode: MouseReportingMode,

    /// Mode 1004.
    pub mouse_focus_event: bool, // 1004
    /// Mode 1007.
    pub mouse_alternate_scroll: bool, // 1007

    /// Mouse encoding mode.
    pub mouse_encoding_mode: MouseEncodingMode,

    /// DECCKM.
    pub application_mode_cursor_keys: bool,
}

impl PartialEq for DrawState {
    fn eq(&self, x: &DrawState) -> bool {
        /* only compare fields that affect display */
        (self.width == x.width)
            && (self.height == x.height)
            && (self.cursor_col == x.cursor_col)
            && (self.cursor_row == x.cursor_row)
            && (self.cursor_visible == x.cursor_visible)
            && (self.reverse_video == x.reverse_video)
            && (self.renditions == x.renditions)
            && (self.bracketed_paste == x.bracketed_paste)
            && (self.mouse_reporting_mode == x.mouse_reporting_mode)
            && (self.mouse_focus_event == x.mouse_focus_event)
            && (self.mouse_alternate_scroll == x.mouse_alternate_scroll)
            && (self.mouse_encoding_mode == x.mouse_encoding_mode)
            && self.hyperlink == x.hyperlink
    }
}

impl DrawState {
    fn reinitialize_tabs(&mut self, start: u32) {
        assert!(self.default_tabs);
        // start is an old width (>= 1), so the u32 to usize cast is lossless.
        for i in start as usize..self.tabs.len() {
            self.tabs[i] = (i % 8) == 0;
        }
    }

    /// Initial state for a `s_width` x `s_height` screen.
    pub fn new(s_width: i32, s_height: i32) -> DrawState {
        let mut ds = DrawState {
            width: s_width,
            height: s_height,
            cursor_col: 0,
            cursor_row: 0,
            combining_char_col: 0,
            combining_char_row: 0,
            default_tabs: true,
            tabs: vec![false; idx(s_width)],
            scrolling_region_top_row: 0,
            scrolling_region_bottom_row: s_height - 1,
            renditions: Renditions::new(0),
            hyperlink: Hyperlink::default(),
            save: SavedCursor::new(),
            next_print_will_wrap: false,
            origin_mode: false,
            auto_wrap_mode: true,
            insert_mode: false,
            cursor_visible: true,
            reverse_video: false,
            bracketed_paste: false,
            mouse_reporting_mode: MOUSE_REPORTING_NONE,
            mouse_focus_event: false,
            mouse_alternate_scroll: false,
            mouse_encoding_mode: MOUSE_ENCODING_DEFAULT,
            application_mode_cursor_keys: false,
        };
        ds.reinitialize_tabs(0);
        ds
    }

    fn new_grapheme(&mut self) {
        self.combining_char_col = self.cursor_col;
        self.combining_char_row = self.cursor_row;
    }

    fn snap_cursor_to_border(&mut self) {
        if self.cursor_row < self.limit_top() {
            self.cursor_row = self.limit_top();
        }
        if self.cursor_row > self.limit_bottom() {
            self.cursor_row = self.limit_bottom();
        }
        if self.cursor_col < 0 {
            self.cursor_col = 0;
        }
        if self.cursor_col >= self.width {
            self.cursor_col = self.width - 1;
        }
    }

    /// Moves the cursor to row `n` (relative to the origin, or by `n` if `relative`), clamped.
    pub fn move_row(&mut self, n: i32, relative: bool) {
        if relative {
            self.cursor_row += n;
        } else {
            self.cursor_row = n + self.limit_top();
        }

        self.snap_cursor_to_border();
        self.new_grapheme();
        self.next_print_will_wrap = false;
    }

    /// Moves the cursor to column `n` (or by `n`); `implicit` is a move caused by printing, which may set `next_print_will_wrap`.
    pub fn move_col(&mut self, n: i32, relative: bool, implicit: bool) {
        if implicit {
            self.new_grapheme();
        }

        if relative {
            self.cursor_col += n;
        } else {
            self.cursor_col = n;
        }

        if implicit {
            self.next_print_will_wrap = self.cursor_col >= self.width;
        }

        self.snap_cursor_to_border();
        if !implicit {
            self.new_grapheme();
            self.next_print_will_wrap = false;
        }
    }

    /// Cursor column.
    pub fn get_cursor_col(&self) -> i32 {
        self.cursor_col
    }
    /// Cursor row.
    pub fn get_cursor_row(&self) -> i32 {
        self.cursor_row
    }
    /// Combining target column.
    pub fn get_combining_char_col(&self) -> i32 {
        self.combining_char_col
    }
    /// Combining target row.
    pub fn get_combining_char_row(&self) -> i32 {
        self.combining_char_row
    }
    /// Columns.
    pub fn get_width(&self) -> i32 {
        self.width
    }
    /// Rows.
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Debug accessor (not upstream): the tab stop flag of every column.
    pub fn debug_tabs(&self) -> &[bool] {
        &self.tabs
    }

    /// Debug accessor (not upstream): whether tab stops are still the defaults.
    pub fn debug_default_tabs(&self) -> bool {
        self.default_tabs
    }

    /// Debug accessor (not upstream): the DECSC state.
    pub fn debug_saved_cursor(&self) -> &SavedCursor {
        &self.save
    }

    /// Sets a tab stop at the cursor column.
    pub fn set_tab(&mut self) {
        self.tabs[idx(self.cursor_col)] = true;
    }

    /// Clears the tab stop at `col`.
    pub fn clear_tab(&mut self, col: i32) {
        self.tabs[idx(col)] = false;
    }

    /// Marks tab stops as no longer the defaults.
    pub fn clear_default_tabs(&mut self) {
        self.default_tabs = false;
    }
    /* Default tabs can't be restored without resetting the draw state. */

    /// Column of the `count`th tab stop after the cursor (-1 if none), or before it when `count` is negative (0 if none).
    pub fn get_next_tab(&self, mut count: i32) -> i32 {
        if count >= 0 {
            let mut i = self.cursor_col + 1;
            while i < self.width {
                if self.tabs[idx(i)] {
                    count -= 1;
                    if count == 0 {
                        return i;
                    }
                }
                i += 1;
            }
            return -1;
        }
        let mut i = self.cursor_col - 1;
        while i > 0 {
            if self.tabs[idx(i)] {
                count += 1;
                if count == 0 {
                    return i;
                }
            }
            i -= 1;
        }
        0
    }

    /// Sets the scrolling region (clamped).
    pub fn set_scrolling_region(&mut self, top: i32, bottom: i32) {
        if self.height < 1 {
            return;
        }

        self.scrolling_region_top_row = top;
        self.scrolling_region_bottom_row = bottom;

        if self.scrolling_region_top_row < 0 {
            self.scrolling_region_top_row = 0;
        }
        if self.scrolling_region_bottom_row >= self.height {
            self.scrolling_region_bottom_row = self.height - 1;
        }

        if self.scrolling_region_bottom_row < self.scrolling_region_top_row {
            self.scrolling_region_bottom_row = self.scrolling_region_top_row;
        }
        /* real rule requires TWO-line scrolling region */

        if self.origin_mode {
            self.snap_cursor_to_border();
            self.new_grapheme();
        }
    }

    /// Top row of the scrolling region.
    pub fn get_scrolling_region_top_row(&self) -> i32 {
        self.scrolling_region_top_row
    }
    /// Bottom row of the scrolling region.
    pub fn get_scrolling_region_bottom_row(&self) -> i32 {
        self.scrolling_region_bottom_row
    }

    /// Topmost row the cursor may reach (the region top in origin mode).
    pub fn limit_top(&self) -> i32 {
        if self.origin_mode {
            self.scrolling_region_top_row
        } else {
            0
        }
    }

    /// Bottom-most row the cursor may reach (the region bottom in origin mode).
    pub fn limit_bottom(&self) -> i32 {
        if self.origin_mode {
            self.scrolling_region_bottom_row
        } else {
            self.height - 1
        }
    }

    /// Current hyperlink.
    pub fn get_hyperlink(&self) -> &Hyperlink {
        &self.hyperlink
    }
    /// Sets the current hyperlink.
    pub fn set_hyperlink(&mut self, x: Hyperlink) {
        self.hyperlink = x;
    }

    /// Sets the foreground (see `Renditions::set_foreground_color`).
    pub fn set_foreground_color(&mut self, x: i32) {
        self.renditions.set_foreground_color(x);
    }
    /// Sets the background (see `Renditions::set_background_color`).
    pub fn set_background_color(&mut self, x: i32) {
        self.renditions.set_background_color(x);
    }
    /// Applies one SGR parameter.
    pub fn add_rendition(&mut self, x: color_type) {
        self.renditions.set_rendition(x);
    }
    /// Current renditions.
    pub fn get_renditions(&self) -> &Renditions {
        &self.renditions
    }
    /// Current renditions, mutable.
    pub fn get_renditions_mut(&mut self) -> &mut Renditions {
        &mut self.renditions
    }
    /// Current background color number.
    pub fn get_background_rendition(&self) -> i32 {
        self.renditions.get_background_rendition() as i32
    }

    /// DECSC.
    pub fn save_cursor(&mut self) {
        self.save.cursor_col = self.cursor_col;
        self.save.cursor_row = self.cursor_row;
        self.save.renditions = self.renditions;
        self.save.auto_wrap_mode = self.auto_wrap_mode;
        self.save.origin_mode = self.origin_mode;
    }

    /// DECRC (clamped to the current size).
    pub fn restore_cursor(&mut self) {
        self.cursor_col = self.save.cursor_col;
        self.cursor_row = self.save.cursor_row;
        self.renditions = self.save.renditions;
        self.auto_wrap_mode = self.save.auto_wrap_mode;
        self.origin_mode = self.save.origin_mode;

        self.snap_cursor_to_border(); /* we could have resized in between */
        self.new_grapheme();
    }

    /// Forgets the saved cursor.
    pub fn clear_saved_cursor(&mut self) {
        self.save = SavedCursor::new();
    }

    /// Resizes: resets the scrolling region, extends tabs, clamps the cursor.
    pub fn resize(&mut self, s_width: i32, s_height: i32) {
        if (self.width != s_width) || (self.height != s_height) {
            /* reset entire scrolling region on any resize */
            /* xterm and rxvt-unicode do this. gnome-terminal only
            resets scrolling region if it has to become smaller in resize */
            self.scrolling_region_top_row = 0;
            self.scrolling_region_bottom_row = s_height - 1;
        }

        self.tabs.resize(idx(s_width), false);
        if self.default_tabs {
            self.reinitialize_tabs(self.width as u32);
        }

        self.width = s_width;
        self.height = s_height;

        self.snap_cursor_to_border();

        /* saved cursor will be snapped to border on restore */

        /* invalidate combining char cell if necessary */
        if (self.combining_char_col >= self.width) || (self.combining_char_row >= self.height) {
            self.combining_char_col = -1;
            self.combining_char_row = -1;
        }
    }
}

/// Window title, icon name or clipboard, as code points.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
pub type title_type = Vec<wchar_t>;
/// A shared row; rows are copied on write.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
pub type row_pointer = Rc<Row>;
/// The screen rows.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
pub type rows_type = Vec<row_pointer>;

// To minimize copying of rows and cells, we use Rc to share unchanged rows
// between multiple Framebuffers.  If we write to a row in a Framebuffer and
// it is shared with other owners, we copy it first (Rc::make_mut).
// Rc (not Arc) makes Framebuffer and Emulator !Send; upstream mosh is single
// threaded and its shared_ptr copy-on-write relies on that too
// (get_mutable_row: "only safe because mosh isn't multi-threaded").
/// Upstream `Terminal::Framebuffer`: the screen contents plus `DrawState`.
#[derive(Clone, Debug)]
pub struct Framebuffer {
    /// The rows, shared with copies of this framebuffer until written.
    rows: rows_type,
    /// Icon name (OSC 0/1).
    icon_name: title_type,
    /// Window title (OSC 0/2).
    window_title: title_type,
    /// Clipboard contents (OSC 52).
    clipboard: title_type,
    /// Number of BELs received.
    bell_count: u32,
    /// true if the window title has been set via an OSC
    title_initialized: bool,

    /// Cursor, modes and renditions.
    pub ds: DrawState,
}

impl PartialEq for Framebuffer {
    fn eq(&self, x: &Framebuffer) -> bool {
        // rows_type is std::vector<std::shared_ptr<Row>>: its operator==
        // compares the pointers, not the rows they point to.
        (self.rows.len() == x.rows.len()
            && self
                .rows
                .iter()
                .zip(x.rows.iter())
                .all(|(a, b)| Rc::ptr_eq(a, b)))
            && (self.window_title == x.window_title)
            && (self.clipboard == x.clipboard)
            && (self.bell_count == x.bell_count)
            && (self.ds == x.ds)
    }
}

impl Framebuffer {
    fn newrow(&self) -> row_pointer {
        let w = idx(self.ds.get_width());
        let c = self.ds.get_background_rendition() as color_type;
        Rc::new(Row::new(w, c))
    }

    /// A blank `s_width` x `s_height` framebuffer.
    pub fn new(s_width: i32, s_height: i32) -> Framebuffer {
        assert!(s_height > 0);
        assert!(s_width > 0);
        let w = idx(s_width);
        let c: color_type = 0;
        let row = Rc::new(Row::new(w, c));
        Framebuffer {
            rows: vec![row; idx(s_height)],
            icon_name: Vec::new(),
            window_title: Vec::new(),
            clipboard: Vec::new(),
            bell_count: 0,
            title_initialized: false,
            ds: DrawState::new(s_width, s_height),
        }
    }

    /// The rows.
    pub fn get_rows(&self) -> &rows_type {
        &self.rows
    }

    /// Scrolls the scrolling region up by `n` rows (down if negative).
    pub fn scroll(&mut self, n: i32) {
        if n >= 0 {
            self.delete_line(self.ds.get_scrolling_region_top_row(), n);
        } else {
            self.insert_line(self.ds.get_scrolling_region_top_row(), -n);
        }
    }

    /// Moves the cursor down `rows` rows (up if negative), scrolling the region when it would leave it.
    pub fn move_rows_autoscroll(&mut self, rows: i32) {
        /* don't scroll if outside the scrolling region */
        if (self.ds.get_cursor_row() < self.ds.get_scrolling_region_top_row())
            || (self.ds.get_cursor_row() > self.ds.get_scrolling_region_bottom_row())
        {
            self.ds.move_row(rows, true);
            return;
        }

        if self.ds.get_cursor_row() + rows > self.ds.get_scrolling_region_bottom_row() {
            let n = self.ds.get_cursor_row() + rows - self.ds.get_scrolling_region_bottom_row();
            self.scroll(n);
            self.ds.move_row(-n, true);
        } else if self.ds.get_cursor_row() + rows < self.ds.get_scrolling_region_top_row() {
            let n = self.ds.get_cursor_row() + rows - self.ds.get_scrolling_region_top_row();
            self.scroll(n);
            self.ds.move_row(-n, true);
        }

        self.ds.move_row(rows, true);
    }

    /// `row == -1` means the cursor row.
    pub fn get_row(&self, mut row: i32) -> &Row {
        if row == -1 {
            row = self.ds.get_cursor_row();
        }

        &self.rows[idx(row)]
    }

    /// `row == -1` / `col == -1` mean the cursor position.
    pub fn get_cell(&self, mut row: i32, mut col: i32) -> &Cell {
        if row == -1 {
            row = self.ds.get_cursor_row();
        }
        if col == -1 {
            col = self.ds.get_cursor_col();
        }

        &self.rows[idx(row)].cells[idx(col)]
    }

    /// Row `row` (-1 = cursor row), copied first if shared.
    pub fn get_mutable_row(&mut self, mut row: i32) -> &mut Row {
        if row == -1 {
            row = self.ds.get_cursor_row();
        }
        // If the row is shared, copy it.
        Rc::make_mut(&mut self.rows[idx(row)])
    }

    /// Cell at `row`, `col` (-1 = cursor), its row copied first if shared.
    pub fn get_mutable_cell(&mut self, mut row: i32, mut col: i32) -> &mut Cell {
        if row == -1 {
            row = self.ds.get_cursor_row();
        }
        if col == -1 {
            col = self.ds.get_cursor_col();
        }

        &mut self.get_mutable_row(row).cells[idx(col)]
    }

    /// Upstream returns the cell (or NULL); here its (row, col), after
    /// making the row unshared exactly as upstream's get_mutable_cell does.
    pub fn get_combining_cell(&mut self) -> Option<(i32, i32)> {
        if (self.ds.get_combining_char_col() < 0)
            || (self.ds.get_combining_char_row() < 0)
            || (self.ds.get_combining_char_col() >= self.ds.get_width())
            || (self.ds.get_combining_char_row() >= self.ds.get_height())
        {
            return None;
        } /* can happen if a resize came in between */

        let (row, col) = (
            self.ds.get_combining_char_row(),
            self.ds.get_combining_char_col(),
        );
        self.get_mutable_cell(row, col);
        Some((row, col))
    }

    /// Upstream takes a `Cell*` (NULL = cursor cell); here (row, col) with -1 = cursor.
    pub fn apply_renditions_to_cell(&mut self, row: i32, col: i32) {
        let r = *self.ds.get_renditions();
        self.get_mutable_cell(row, col).set_renditions(r);
    }

    /// Upstream takes a `Cell*` (NULL = cursor cell); here (row, col) with -1 = cursor.
    pub fn apply_hyperlink_to_cell(&mut self, row: i32, col: i32) {
        let h = self.ds.get_hyperlink().clone();
        self.get_mutable_cell(row, col).set_hyperlink(h);
    }

    /// Inserts `count` blank rows before `before_row` within the scrolling region.
    pub fn insert_line(&mut self, before_row: i32, count: i32) {
        if (before_row < self.ds.get_scrolling_region_top_row())
            || (before_row > self.ds.get_scrolling_region_bottom_row() + 1)
        {
            return;
        }

        let mut scroll = self.ds.get_scrolling_region_bottom_row() + 1 - before_row;
        if count < scroll {
            scroll = count;
        }

        if scroll == 0 {
            return;
        }

        // scroll >= 1 here: callers pass count >= 1 (getparam default 1), and the
        // region check above makes bottom + 1 - before_row >= 0.
        // delete old rows
        let start = (self.ds.get_scrolling_region_bottom_row() + 1 - scroll) as usize;
        self.rows.drain(start..start + scroll as usize);
        // insert new rows
        let start = before_row as usize;
        let newrow = self.newrow();
        self.rows
            .splice(start..start, std::iter::repeat_n(newrow, scroll as usize));
    }

    /// Deletes `count` rows at `row` within the scrolling region, adding blank rows at its bottom.
    pub fn delete_line(&mut self, row: i32, count: i32) {
        if (row < self.ds.get_scrolling_region_top_row())
            || (row > self.ds.get_scrolling_region_bottom_row())
        {
            return;
        }

        let mut scroll = self.ds.get_scrolling_region_bottom_row() + 1 - row;
        if count < scroll {
            scroll = count;
        }

        if scroll == 0 {
            return;
        }

        // scroll >= 1 and row >= 0 here (see insert_line).
        // delete old rows
        let start = row as usize;
        self.rows.drain(start..start + scroll as usize);
        // insert a block of dummy rows
        let start = (self.ds.get_scrolling_region_bottom_row() + 1 - scroll) as usize;
        let newrow = self.newrow();
        self.rows
            .splice(start..start, std::iter::repeat_n(newrow, scroll as usize));
    }

    /// Inserts a blank cell at `row`, `col`.
    pub fn insert_cell(&mut self, row: i32, col: i32) {
        let bg = self.ds.get_background_rendition() as color_type;
        self.get_mutable_row(row).insert_cell(col, bg);
    }

    /// Deletes the cell at `row`, `col`.
    pub fn delete_cell(&mut self, row: i32, col: i32) {
        let bg = self.ds.get_background_rendition() as color_type;
        self.get_mutable_row(row).delete_cell(col, bg);
    }

    /// RIS: blank screen and initial `DrawState`; keeps the bell count and icon name.
    pub fn reset(&mut self) {
        let width = self.ds.get_width();
        let height = self.ds.get_height();
        self.ds = DrawState::new(width, height);
        let row = self.newrow();
        self.rows = vec![row; idx(height)];
        self.window_title.clear();
        self.clipboard.clear();
        /* do not reset bell_count */
    }

    /// DECSTR.
    pub fn soft_reset(&mut self) {
        self.ds.insert_mode = false;
        self.ds.origin_mode = false;
        self.ds.cursor_visible = true; /* per xterm and gnome-terminal */
        self.ds.application_mode_cursor_keys = false;
        let h = self.ds.get_height();
        self.ds.set_scrolling_region(0, h - 1);
        self.ds.add_rendition(0);
        self.ds.set_hyperlink(Hyperlink::default());
        self.ds.clear_saved_cursor();
    }

    /// Marks the title as set by the application.
    pub fn set_title_initialized(&mut self) {
        self.title_initialized = true;
    }
    /// Whether the application has set a title.
    pub fn is_title_initialized(&self) -> bool {
        self.title_initialized
    }
    /// Sets the icon name.
    pub fn set_icon_name(&mut self, s: &title_type) {
        self.icon_name = s.clone();
    }
    /// Sets the window title.
    pub fn set_window_title(&mut self, s: &title_type) {
        self.window_title = s.clone();
    }
    /// Sets the clipboard contents.
    pub fn set_clipboard(&mut self, s: &title_type) {
        self.clipboard = s.clone();
    }
    /// The icon name.
    pub fn get_icon_name(&self) -> &title_type {
        &self.icon_name
    }
    /// The window title.
    pub fn get_window_title(&self) -> &title_type {
        &self.window_title
    }
    /// The clipboard contents.
    pub fn get_clipboard(&self) -> &title_type {
        &self.clipboard
    }

    /// Prepends `s` to the window title (and the icon name, if they were equal).
    pub fn prefix_window_title(&mut self, s: &title_type) {
        if self.icon_name == self.window_title {
            /* preserve equivalence */
            self.icon_name.splice(0..0, s.iter().copied());
        }
        self.window_title.splice(0..0, s.iter().copied());
    }

    /// Resizes the screen; rows that existed before lose their wrap flag.
    pub fn resize(&mut self, s_width: i32, s_height: i32) {
        assert!(s_width > 0);
        assert!(s_height > 0);

        let oldheight = self.ds.get_height();
        let oldwidth = self.ds.get_width();
        self.ds.resize(s_width, s_height);

        let blankrow = self.newrow();
        if oldheight != s_height {
            self.rows.resize(idx(s_height), blankrow.clone());
        }
        if oldwidth == s_width {
            return;
        }
        let bg = self.ds.get_background_rendition() as color_type;
        for i in self.rows.iter_mut() {
            if Rc::ptr_eq(i, &blankrow) {
                break;
            }
            let mut r = (**i).clone();
            r.set_wrap(false);
            r.cells.resize(idx(s_width), Cell::new(bg));
            *i = Rc::new(r);
        }
    }

    /// Upstream takes a `Cell*`; here (row, col) with -1 = cursor.
    pub fn reset_cell(&mut self, row: i32, col: i32) {
        let bg = self.ds.get_background_rendition() as color_type;
        self.get_mutable_cell(row, col).reset(bg);
    }

    /// Upstream takes a `Row*`; here a row index with -1 = cursor row.
    pub fn reset_row(&mut self, row: i32) {
        let bg = self.ds.get_background_rendition() as color_type;
        self.get_mutable_row(row).reset(bg);
    }

    /// Counts a BEL.
    pub fn ring_bell(&mut self) {
        self.bell_count = self.bell_count.wrapping_add(1);
    }
    /// Number of BELs received.
    pub fn get_bell_count(&self) -> u32 {
        self.bell_count
    }
}
