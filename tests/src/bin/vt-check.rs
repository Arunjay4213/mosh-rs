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
// Ported from: ref/ref-term/ref-term.cc (the Rust twin of the C++ oracle)
//
// vt-check: feeds bytes to the mosh-rs terminal emulator and prints the same
// dump / new_frame bytes as ref/ref-term/ref-term. See ref/ref-term/README.md.

use std::fmt::Write as _;
use std::io::{Read, Write};

use mosh_terminal::terminalframebuffer::{Framebuffer, Hyperlink, Renditions};
use mosh_terminal::{Actions, Display, Emulator, UTF8Parser};

fn hex(s: &[u8]) -> String {
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
    match &h.rep {
        None => "none".to_string(),
        Some(rep) => format!("{}|{}", hex(rep.params.as_bytes()), hex(rep.url.as_bytes())),
    }
}

fn rend(r: &Renditions) -> String {
    format!("{} {} {}", r.attributes, r.foreground_color, r.background_color)
}

fn dump(fb: &Framebuffer, to_host: &[u8], o: &mut String) {
    let ds = &fb.ds;
    let b = |x: bool| x as i32;
    let _ = writeln!(o, "size {} {}", ds.width, ds.height);
    let _ = writeln!(
        o,
        "cursor {} {} visible {} next_print_will_wrap {}",
        ds.cursor_row,
        ds.cursor_col,
        b(ds.cursor_visible),
        b(ds.next_print_will_wrap)
    );
    let _ = writeln!(o, "combining {} {}", ds.combining_char_row, ds.combining_char_col);
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
    let _ = writeln!(o, "scroll_region {} {}", ds.scrolling_region_top_row, ds.scrolling_region_bottom_row);
    let tabs: Vec<String> =
        ds.tabs.iter().enumerate().filter(|(_, &t)| t).map(|(i, _)| i.to_string()).collect();
    let _ = writeln!(o, "default_tabs {} tabs {}", b(ds.default_tabs), tabs.join(","));
    let _ = writeln!(o, "renditions {}", rend(&ds.renditions));
    let _ = writeln!(o, "hyperlink {}", link(&ds.hyperlink));
    let _ = writeln!(
        o,
        "saved {} {} autowrap {} origin {} renditions {}",
        ds.save.cursor_row,
        ds.save.cursor_col,
        b(ds.save.auto_wrap_mode),
        b(ds.save.origin_mode),
        rend(&ds.save.renditions)
    );
    let _ = writeln!(o, "title_initialized {}", b(fb.title_initialized));
    let _ = writeln!(o, "icon {}", wlist(&fb.icon_name));
    let _ = writeln!(o, "title {}", wlist(&fb.window_title));
    let _ = writeln!(o, "clipboard {}", wlist(&fb.clipboard));
    let _ = writeln!(o, "bell_count {}", fb.bell_count);
    let _ = writeln!(o, "to_host {}", hex(to_host));
    for (r, row) in fb.rows.iter().enumerate() {
        let _ = writeln!(o, "row {} gen {} wrap {}", r, row.gen, b(row.get_wrap()));
        for (c, cell) in row.cells.iter().enumerate() {
            let _ = writeln!(
                o,
                "  cell {} contents={} wide {} fallback {} wrap {} renditions {} link={}",
                c,
                hex(cell.contents.as_bytes()),
                b(cell.wide),
                b(cell.fallback),
                b(cell.wrap),
                rend(&cell.renditions),
                link(&cell.hyperlink)
            );
        }
    }
}

/// Feed bytes exactly as Terminal::Complete::act does.
fn feed(parser: &mut UTF8Parser, emu: &mut Emulator, s: &[u8], to_host: &mut Vec<u8>) {
    let mut actions = Actions::new();
    for &c in s {
        parser.input(c, &mut actions);
        for act in actions.iter() {
            act.act_on_terminal(emu);
        }
        actions.clear();
    }
    to_host.extend_from_slice(&emu.read_octets_to_host());
}

fn records(input: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut p = 0;
    while p < input.len() {
        if input.len() - p < 4 {
            die("vt-check: truncated record header");
        }
        let len = u32::from_be_bytes([input[p], input[p + 1], input[p + 2], input[p + 3]]) as usize;
        p += 4;
        if input.len() - p < len {
            die("vt-check: truncated record");
        }
        out.push(&input[p..p + len]);
        p += len;
    }
    out
}

fn do_dump(w: usize, h: usize, input: &[u8]) -> String {
    let mut emu = Emulator::new(w, h);
    let mut parser = UTF8Parser::new();
    let mut to_host = Vec::new();
    let mut o = String::new();
    feed(&mut parser, &mut emu, input, &mut to_host);
    dump(emu.get_fb(), &to_host, &mut o);
    o
}

fn do_frame(w: usize, h: usize, oldb: &[u8], newb: &[u8], initialized: bool) -> String {
    let mut emu = Emulator::new(w, h);
    let mut parser = UTF8Parser::new();
    let mut to_host = Vec::new();
    feed(&mut parser, &mut emu, oldb, &mut to_host);
    let last = emu.get_fb().clone();
    feed(&mut parser, &mut emu, newb, &mut to_host);
    let display = Display::new(false);
    display.new_frame(initialized, &last, emu.get_fb())
}

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(2);
}

fn usage() -> ! {
    die("usage: vt-check dump W H < input\n       vt-check batch W H < records\n       vt-check frame W H OLDFILE NEWFILE [initialized]\n       vt-check frame-batch W H [initialized] < record pairs")
}

fn read_stdin() -> Vec<u8> {
    let mut v = Vec::new();
    std::io::stdin().lock().read_to_end(&mut v).unwrap_or_else(|e| die(&format!("vt-check: stdin: {}", e)));
    v
}

fn read_file(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| die(&format!("vt-check: cannot open {}: {}", path, e)))
}

fn atoi(s: &str) -> i64 {
    s.trim().parse().unwrap_or(0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        usage();
    }
    let (w, h) = (atoi(&args[2]), atoi(&args[3]));
    if w < 1 || h < 1 {
        usage();
    }
    let (w, h) = (w as usize, h as usize);
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    match (args[1].as_str(), args.len()) {
        ("dump", 4) => {
            let _ = out.write_all(do_dump(w, h, &read_stdin()).as_bytes());
        }
        ("batch", 4) => {
            let input = read_stdin();
            for r in records(&input) {
                let _ = out.write_all(do_dump(w, h, r).as_bytes());
                let _ = out.write_all(b"=====\n");
            }
        }
        ("frame", 6) | ("frame", 7) => {
            let initialized = if args.len() == 7 { atoi(&args[6]) != 0 } else { true };
            let f = do_frame(w, h, &read_file(&args[4]), &read_file(&args[5]), initialized);
            let _ = out.write_all(f.as_bytes());
        }
        ("frame-batch", 4) | ("frame-batch", 5) => {
            let initialized = if args.len() == 5 { atoi(&args[4]) != 0 } else { true };
            let input = read_stdin();
            let recs = records(&input);
            if recs.len() % 2 != 0 {
                die("vt-check: frame-batch needs an even number of records");
            }
            for pair in recs.chunks(2) {
                let f = do_frame(w, h, pair[0], pair[1], initialized);
                let _ = out.write_all(&(f.len() as u32).to_be_bytes());
                let _ = out.write_all(f.as_bytes());
            }
        }
        _ => usage(),
    }
    let _ = out.flush();
}
