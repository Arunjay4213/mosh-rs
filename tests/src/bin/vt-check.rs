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

use std::io::{Read, Write};

use mosh_terminal::{Actions, Display, Emulator, Resize, UTF8Parser};
use mosh_tests::dump::{dump, hex};

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
    dump(emu.get_fb(), &to_host, true, &mut o);
    o
}

/// `steps`: one emulator, records tagged 'B' (bytes to feed) or 'R' (u32 BE
/// width, u32 BE height: a Resize action, as Terminal::Complete::act(Action)).
fn do_steps(w: usize, h: usize, recs: &[&[u8]]) -> String {
    let mut emu = Emulator::new(w, h);
    let mut parser = UTF8Parser::new();
    let mut to_host = Vec::new();
    for r in recs {
        match r.first() {
            Some(b'B') => feed(&mut parser, &mut emu, &r[1..], &mut to_host),
            Some(b'R') if r.len() == 9 => {
                let nw = u32::from_be_bytes([r[1], r[2], r[3], r[4]]) as usize;
                let nh = u32::from_be_bytes([r[5], r[6], r[7], r[8]]) as usize;
                if nw < 1 || nh < 1 {
                    die("vt-check: resize to zero");
                }
                Resize::new(nw, nh).act_on_terminal(&mut emu);
                to_host.extend_from_slice(&emu.read_octets_to_host());
            }
            _ => die("vt-check: bad steps record"),
        }
    }
    let mut o = String::new();
    dump(emu.get_fb(), &to_host, true, &mut o);
    o
}

/// `frame-resize`: like `do_frame`, but the emulator is resized to `w2` x `h2`
/// (a Resize action) between the old and the new bytes.
fn do_frame_resize(
    w: usize,
    h: usize,
    w2: usize,
    h2: usize,
    oldb: &[u8],
    newb: &[u8],
    initialized: bool,
) -> String {
    let mut emu = Emulator::new(w, h);
    let mut parser = UTF8Parser::new();
    let mut to_host = Vec::new();
    feed(&mut parser, &mut emu, oldb, &mut to_host);
    let last = emu.get_fb().clone();
    Resize::new(w2, h2).act_on_terminal(&mut emu);
    feed(&mut parser, &mut emu, newb, &mut to_host);
    let display = Display::new(false);
    display.new_frame(initialized, &last, emu.get_fb())
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
    die("usage: vt-check dump W H < input\n       vt-check caps < terminal type names\n       vt-check steps W H < tagged records\n       vt-check batch W H < records\n       vt-check frame W H OLDFILE NEWFILE [initialized]\n       vt-check frame-batch W H [initialized] < record pairs\n       vt-check frame-resize-batch W H [initialized] < (size, old, new) triples")
}

fn read_stdin() -> Vec<u8> {
    let mut v = Vec::new();
    std::io::stdin()
        .lock()
        .read_to_end(&mut v)
        .unwrap_or_else(|e| die(&format!("vt-check: stdin: {}", e)));
    v
}

fn read_file(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| die(&format!("vt-check: cannot open {}: {}", path, e)))
}

fn atoi(s: &str) -> i64 {
    s.trim().parse().unwrap_or(0)
}

/// `caps`: for each name on stdin, `Display::new(true)`'s decisions under
/// TERM=<name>, in ref-term's format.
fn do_caps() {
    let capstr = |c: Option<&[u8]>| c.map_or("none".to_string(), hex);
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let input = read_stdin();
    for name in input.split(|&c| c == b'\n') {
        if name.is_empty() {
            continue;
        }
        let name_str = String::from_utf8_lossy(name).into_owned();
        std::env::set_var("TERM", &name_str);
        let caps = Display::new(true).debug_caps();
        let _ = writeln!(
            out,
            "{} ech {} bce {} title {} smcup {} rmcup {} colors {}",
            hex(name),
            caps.ech as u8,
            caps.bce as u8,
            caps.title as u8,
            capstr(caps.smcup),
            capstr(caps.rmcup),
            match mosh_terminal::terminfo::colors(Some(&name_str)) {
                Ok(n) => n.to_string(),
                Err(_) => "error".to_string(),
            }
        );
    }
    let _ = out.flush();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "caps" {
        do_caps();
        return;
    }
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
        ("steps", 4) => {
            let input = read_stdin();
            let _ = out.write_all(do_steps(w, h, &records(&input)).as_bytes());
        }
        ("frame", 6) | ("frame", 7) => {
            let initialized = if args.len() == 7 {
                atoi(&args[6]) != 0
            } else {
                true
            };
            let f = do_frame(
                w,
                h,
                &read_file(&args[4]),
                &read_file(&args[5]),
                initialized,
            );
            let _ = out.write_all(f.as_bytes());
        }
        ("frame-resize-batch", 4) | ("frame-resize-batch", 5) => {
            let initialized = if args.len() == 5 {
                atoi(&args[4]) != 0
            } else {
                true
            };
            let input = read_stdin();
            let recs = records(&input);
            if !recs.len().is_multiple_of(3) {
                die("vt-check: frame-resize-batch needs records in threes");
            }
            for t in recs.chunks(3) {
                let z = t[0];
                if z.len() != 8 {
                    die("vt-check: size record must be 8 bytes");
                }
                let w2 = u32::from_be_bytes([z[0], z[1], z[2], z[3]]) as usize;
                let h2 = u32::from_be_bytes([z[4], z[5], z[6], z[7]]) as usize;
                if w2 < 1 || h2 < 1 {
                    die("vt-check: resize to zero");
                }
                let f = do_frame_resize(w, h, w2, h2, t[1], t[2], initialized);
                let _ = out.write_all(&(f.len() as u32).to_be_bytes());
                let _ = out.write_all(f.as_bytes());
            }
        }
        ("frame-batch", 4) | ("frame-batch", 5) => {
            let initialized = if args.len() == 5 {
                atoi(&args[4]) != 0
            } else {
                true
            };
            let input = read_stdin();
            let recs = records(&input);
            if !recs.len().is_multiple_of(2) {
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
