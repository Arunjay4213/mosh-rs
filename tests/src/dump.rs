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
// Ported from: ref/ref-term/ref-term.cc (dump format shared by vt-check and the tests)

//! The framebuffer dump printed by `ref-term` and `vt-check`, as a library so
//! tests can dump any `Framebuffer`. Format: `ref/ref-term/README.md`.

use std::fmt::Write as _;

use mosh_terminal::terminalframebuffer::{Framebuffer, Hyperlink, Renditions};

/// Lowercase hex of raw bytes, no separators.
pub fn hex(s: &[u8]) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s {
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn wlist(v: &[u32]) -> String {
    let mut out = String::new();
    for (i, c) in v.iter().enumerate() {
        let _ = write!(out, "{}{:x}", if i > 0 { "," } else { "" }, c);
    }
    out
}

fn link(h: &Hyperlink) -> String {
    match h.debug_parts() {
        None => "none".to_string(),
        Some((params, url)) => format!("{}|{}", hex(params.as_bytes()), hex(url.as_bytes())),
    }
}

fn rend(r: &Renditions) -> String {
    format!(
        "{} {} {}",
        r.debug_attributes(),
        r.debug_foreground_color(),
        r.get_background_rendition()
    )
}

/// Appends the dump of `fb` (format: ref/ref-term/README.md) to `o`.
/// With `with_gen` false the `gen G` field of each row line is left out, for
/// comparing framebuffers built in the same process, whose row generation
/// numbers come from one shared counter.
pub fn dump(fb: &Framebuffer, to_host: &[u8], with_gen: bool, o: &mut String) {
    let ds = &fb.ds;
    let b = |x: bool| x as i32;
    let _ = writeln!(o, "size {} {}", ds.get_width(), ds.get_height());
    let _ = writeln!(
        o,
        "cursor {} {} visible {} next_print_will_wrap {}",
        ds.get_cursor_row(),
        ds.get_cursor_col(),
        b(ds.cursor_visible),
        b(ds.next_print_will_wrap)
    );
    let _ = writeln!(
        o,
        "combining {} {}",
        ds.get_combining_char_row(),
        ds.get_combining_char_col()
    );
    let _ = writeln!(
        o,
        "origin {} autowrap {} insert {} cursor_keys_app {} reverse_video {} bracketed_paste {}",
        b(ds.origin_mode),
        b(ds.auto_wrap_mode),
        b(ds.insert_mode),
        b(ds.application_mode_cursor_keys),
        b(ds.reverse_video),
        b(ds.bracketed_paste)
    );
    let _ = writeln!(
        o,
        "mouse mode {} focus {} altscroll {} encoding {}",
        ds.mouse_reporting_mode as i32,
        b(ds.mouse_focus_event),
        b(ds.mouse_alternate_scroll),
        ds.mouse_encoding_mode as i32
    );
    let _ = writeln!(
        o,
        "scroll_region {} {}",
        ds.get_scrolling_region_top_row(),
        ds.get_scrolling_region_bottom_row()
    );
    let tabs: Vec<String> = ds
        .debug_tabs()
        .iter()
        .enumerate()
        .filter(|(_, &t)| t)
        .map(|(i, _)| i.to_string())
        .collect();
    let _ = writeln!(
        o,
        "default_tabs {} tabs {}",
        b(ds.debug_default_tabs()),
        tabs.join(",")
    );
    let _ = writeln!(o, "renditions {}", rend(ds.get_renditions()));
    let _ = writeln!(o, "hyperlink {}", link(ds.get_hyperlink()));
    let _ = writeln!(
        o,
        "saved {} {} autowrap {} origin {} renditions {}",
        ds.debug_saved_cursor().cursor_row,
        ds.debug_saved_cursor().cursor_col,
        b(ds.debug_saved_cursor().auto_wrap_mode),
        b(ds.debug_saved_cursor().origin_mode),
        rend(&ds.debug_saved_cursor().renditions)
    );
    let _ = writeln!(o, "title_initialized {}", b(fb.is_title_initialized()));
    let _ = writeln!(o, "icon {}", wlist(fb.get_icon_name()));
    let _ = writeln!(o, "title {}", wlist(fb.get_window_title()));
    let _ = writeln!(o, "clipboard {}", wlist(fb.get_clipboard()));
    let _ = writeln!(o, "bell_count {}", fb.get_bell_count());
    let _ = writeln!(o, "to_host {}", hex(to_host));
    for (r, row) in fb.get_rows().iter().enumerate() {
        if with_gen {
            let _ = writeln!(o, "row {} gen {} wrap {}", r, row.gen, b(row.get_wrap()));
        } else {
            let _ = writeln!(o, "row {} wrap {}", r, b(row.get_wrap()));
        }
        for (c, cell) in row.cells.iter().enumerate() {
            let _ = writeln!(
                o,
                "  cell {} contents={} wide {} fallback {} wrap {} renditions {} link={}",
                c,
                hex(cell.debug_contents_bytes()),
                b(cell.get_wide()),
                b(cell.get_fallback()),
                b(cell.get_wrap()),
                rend(cell.get_renditions()),
                link(cell.get_hyperlink())
            );
        }
    }
}

/// Removes the `gen G` field from every row line of a dump, so a dump from
/// another process can be compared with `dump(.., with_gen = false, ..)`.
pub fn strip_gen(d: &str) -> String {
    let mut out = String::with_capacity(d.len());
    for line in d.split_inclusive('\n') {
        if let (true, Some(g)) = (line.starts_with("row "), line.find(" gen ")) {
            let rest = &line[g + 5..];
            let w = rest.find(' ').unwrap_or(rest.len());
            out.push_str(&line[..g]);
            out.push_str(&rest[w..]);
        } else {
            out.push_str(line);
        }
    }
    out
}
