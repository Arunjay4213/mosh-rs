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
// Ported from: (none; differential fuzzer for the port of upstream/src/terminal)
//
// vt-fuzz: generates random terminal input biased toward escape sequences,
// runs it through upstream mosh's emulator (ref/ref-term/ref-term batch) and
// the Rust port (vt-check batch), and compares the framebuffer dumps.
// A mismatching input is minimized and written to tests/corpus/fuzz-failures.
//
// usage: vt-fuzz --cases N --jobs J --seed S [--size WxH] [--frames]
//
// With --frames, each case is a pair (old, new) of random inputs and the
// programs' `frame-batch` output (Display::new_frame bytes) is compared
// instead; every other batch also resizes the emulator to a random size
// between old and new (`frame-resize-batch`), and one batch in four of each
// kind uses initialized=0.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

const BATCH: usize = 2000;
const MAX_SAVED: usize = 20;
const SIZES: [(usize, usize); 5] = [(80, 24), (132, 43), (40, 10), (10, 3), (1, 1)];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

/// splitmix64
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi - lo + 1)
    }
    fn chance(&mut self, num: u64, den: u64) -> bool {
        self.below(den) < num
    }
    fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[self.below(v.len() as u64) as usize]
    }
}

fn push_char(out: &mut Vec<u8>, c: u32) {
    if let Some(ch) = char::from_u32(c) {
        let mut b = [0u8; 4];
        out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
    }
}

/// A CSI parameter value: mostly small, sometimes defaults, large, or out of range.
fn param(r: &mut Rng) -> String {
    match r.below(20) {
        0 => String::new(),
        1 => "0".into(),
        2 => r.range(100, 300).to_string(),
        3 => r.range(1000, 70000).to_string(),
        4 => r
            .pick(&[
                "65535",
                "65536",
                "99999999999999999999",
                "00000000000000000001",
                "2147483648",
            ])
            .to_string(),
        _ => r.range(1, 30).to_string(),
    }
}

fn params(r: &mut Rng, n: usize) -> String {
    (0..n).map(|_| param(r)).collect::<Vec<_>>().join(";")
}

const DEC_MODES: [u32; 25] = [
    1, 3, 5, 6, 7, 9, 25, 47, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1015, 1047, 1049,
    2004, 12, 4, 0, 2, 66,
];

fn text(r: &mut Rng, out: &mut Vec<u8>, n: usize) {
    for _ in 0..n {
        match r.below(10) {
            0 => push_char(out, r.range(0xA0, 0x24F) as u32),
            1 => push_char(out, r.range(0x4E00, 0x4E40) as u32),
            2 => out.push(*r.pick(&[0x07u8, 0x1b, 0x9c, 0x0a, 0x00, 0x7f])),
            _ => out.push(r.range(0x20, 0x7e) as u8),
        }
    }
}

fn piece(r: &mut Rng, out: &mut Vec<u8>) {
    match r.below(30) {
        // CSI with 0-6 params, optional private prefix and intermediate, any final byte.
        0..=4 => {
            if r.chance(1, 12) {
                out.extend_from_slice("\u{9b}".as_bytes()); /* C1 CSI */
            } else {
                out.extend_from_slice(b"\x1b[");
            }
            match r.below(6) {
                0 => out.push(b'?'),
                1 => out.push(b'>'),
                2 => out.push(*r.pick(b"<=!:")),
                _ => {}
            }
            let n = r.below(7) as usize;
            out.extend_from_slice(params(r, n).as_bytes());
            if r.chance(1, 8) {
                out.push(r.range(0x20, 0x2f) as u8);
            }
            out.push(r.range(0x40, 0x7e) as u8);
        }
        // SGR with 0-9 params including 38/48 sub-forms.
        5..=7 => {
            out.extend_from_slice(b"\x1b[");
            let n = r.below(10);
            let mut p: Vec<String> = Vec::new();
            for _ in 0..n {
                match r.below(8) {
                    0 => p.push(format!("{};5;{}", r.pick(&["38", "48"]), param(r))),
                    1 => p.push(format!(
                        "{};2;{};{};{}",
                        r.pick(&["38", "48"]),
                        r.below(300),
                        r.below(256),
                        param(r)
                    )),
                    2 => p.push(r.pick(&["38", "48", "38;5", "48;2;1", "38;2"]).to_string()),
                    3 => p.push(r.range(90, 107).to_string()),
                    4 => p.push(param(r)),
                    _ => p.push(r.below(50).to_string()),
                }
            }
            out.extend_from_slice(p.join(";").as_bytes());
            out.push(b'm');
        }
        // OSC 0/1/2/8/52 with random text and terminators.
        8..=9 => {
            if r.chance(1, 10) {
                out.extend_from_slice("\u{9d}".as_bytes());
            } else {
                out.extend_from_slice(b"\x1b]");
            }
            match r.below(8) {
                0 => out.extend_from_slice(b"0;"),
                1 => out.extend_from_slice(b"1;"),
                2 => out.extend_from_slice(b"2;"),
                3 => {
                    out.extend_from_slice(b"8;");
                    if r.chance(1, 2) {
                        out.extend_from_slice(b"id=x");
                        let n = r.below(3) as usize;
                        text(r, out, n);
                    }
                    out.push(b';');
                    if r.chance(3, 4) {
                        out.extend_from_slice(b"http://example.com/");
                    }
                }
                4 => out.extend_from_slice(b"52;c;"),
                5 => out.extend_from_slice(r.pick(&[&b";"[..], b"8", b"8;", b"52;", b"3;", b"0"])),
                _ => out.extend_from_slice(r.pick(&[&b"0;"[..], b"2;", b"8;;", b"52;c;"])),
            }
            let n = r.below(20) as usize;
            text(r, out, n);
            match r.below(6) {
                0 | 1 => out.push(0x07),
                2 | 3 => out.extend_from_slice(b"\x1b\\"),
                4 => out.extend_from_slice("\u{9c}".as_bytes()),
                _ => {}
            }
        }
        // ESC sequences.
        10..=11 => {
            out.push(0x1b);
            match r.below(4) {
                0 => out.extend_from_slice(r.pick(&[
                    &b"7"[..],
                    b"8",
                    b"c",
                    b"#8",
                    b"D",
                    b"E",
                    b"H",
                    b"M",
                    b"(B",
                    b"(0",
                    b")0",
                    b"=",
                    b">",
                ])),
                1 => {
                    out.push(r.range(0x20, 0x2f) as u8);
                    out.push(r.range(0x20, 0x7e) as u8);
                }
                2 => push_char(out, r.range(0xA0, 0x4FF) as u32),
                _ => out.push(r.range(0x20, 0x7f) as u8),
            }
        }
        // C0 controls.
        12..=13 => {
            let n = r.range(1, 4);
            for _ in 0..n {
                out.push(*r.pick(&[
                    0x08u8, 0x09, 0x0a, 0x0d, 0x07, 0x0e, 0x0f, 0x0b, 0x0c, 0x00, 0x18, 0x1a, 0x1b,
                    0x05, 0x7f,
                ]));
            }
        }
        // DEC private modes.
        14 => {
            out.extend_from_slice(b"\x1b[?");
            let n = r.range(1, 3);
            let m: Vec<String> = (0..n).map(|_| r.pick(&DEC_MODES).to_string()).collect();
            out.extend_from_slice(m.join(";").as_bytes());
            out.push(*r.pick(b"hl"));
        }
        // Scroll region, insert/delete lines and chars, erase variants, insert mode.
        15..=16 => {
            let s = match r.below(9) {
                0 => format!("\x1b[{};{}r", r.below(30), r.below(50)),
                1 => format!("\x1b[{}L", param(r)),
                2 => format!("\x1b[{}M", param(r)),
                3 => format!("\x1b[{}@", param(r)),
                4 => format!("\x1b[{}P", param(r)),
                5 => format!("\x1b[{}X", param(r)),
                6 => format!("\x1b[{}{}", r.below(4), r.pick(&["J", "K"])),
                7 => format!(
                    "\x1b[{}{}",
                    r.pick(&["4", "4;20", "20"]),
                    r.pick(&["h", "l"])
                ),
                _ => format!("\x1b[{}{}", param(r), r.pick(&["S", "T"])),
            };
            out.extend_from_slice(s.as_bytes());
        }
        // Cursor movement, save/restore, tabs, reports.
        17..=18 => {
            let s = match r.below(10) {
                0 => format!("\x1b[{};{}{}", param(r), param(r), r.pick(&["H", "f"])),
                1 => format!(
                    "\x1b[{}{}",
                    param(r),
                    r.pick(&["A", "B", "C", "D", "G", "d", "`", "I", "Z", "E", "F"])
                ),
                2 => r.pick(&["\x1b7", "\x1b8", "\x1b[s", "\x1b[u"]).to_string(),
                3 => r
                    .pick(&["\x1bH", "\x1b[g", "\x1b[3g", "\x1b[0g", "\u{88}"])
                    .to_string(),
                4 => r
                    .pick(&[
                        "\x1b[5n", "\x1b[6n", "\x1b[c", "\x1b[>c", "\x1b[0c", "\x1b[!p",
                    ])
                    .to_string(),
                5 => r
                    .pick(&["\u{84}", "\u{85}", "\u{8d}", "\x1bD", "\x1bE", "\x1bM"])
                    .to_string(),
                _ => format!(
                    "\x1b[{}{}",
                    r.range(1, 50),
                    r.pick(&["H", "G", "d", "C", "D", "A", "B"])
                ),
            };
            out.extend_from_slice(s.as_bytes());
        }
        // Printable ASCII.
        19..=21 => {
            let n = r.range(1, 20);
            for _ in 0..n {
                out.push(r.range(0x20, 0x7e) as u8);
            }
        }
        // UTF-8: wide, emoji, combining, ZWJ, variation selectors, C1, anything.
        22..=24 => {
            let n = r.range(1, 8);
            for _ in 0..n {
                let c = match r.below(14) {
                    0 => r.range(0x4E00, 0x9FFF),
                    1 => r.range(0x1F300, 0x1F6FF),
                    2 | 3 => r.range(0x300, 0x36F),
                    4 => 0x200D,
                    5 => *r.pick(&[0xFE0F, 0xFE0E, 0x20E3, 0x1F3FB]),
                    6 => r.range(0xAC00, 0xD7A3),
                    7 => r.range(0x80, 0x9F),
                    8 => r.range(0xA0, 0xFF),
                    9 => r.range(0x1100, 0x11FF),
                    10 => r.range(0xFF00, 0xFFFF),
                    11 => r.range(0x0, 0x10FFFF),
                    12 => r.range(0x2000, 0x2BFF),
                    _ => r.range(0x20, 0x7E),
                } as u32;
                push_char(out, c);
            }
        }
        // Invalid bytes.
        25..=26 => {
            let n = r.range(1, 3);
            for _ in 0..n {
                let b: &[u8] = match r.below(12) {
                    0 => &[0x80],
                    1 => &[0xBF],
                    2 => &[0xC0, 0x80],
                    3 => &[0xE0, 0x80, 0x80],
                    4 => &[0xF0, 0x80, 0x80, 0x80],
                    5 => &[0xED, 0xA0, 0x80],
                    6 => &[0xFF],
                    7 => &[0xFE],
                    8 => &[0xF4, 0x90, 0x80, 0x80],
                    9 => &[0xF8, 0x88, 0x80, 0x80, 0x80],
                    10 => &[0xFC, 0x84, 0x80, 0x80, 0x80, 0x80],
                    _ => &[0xE4, 0xB8],
                };
                // Sometimes truncate the sequence.
                let len = if r.chance(1, 3) {
                    r.range(1, b.len() as u64) as usize
                } else {
                    b.len()
                };
                out.extend_from_slice(&b[..len]);
            }
            if r.chance(1, 3) {
                out.push(r.range(0x80, 0xFF) as u8);
            }
        }
        // Long runs that wrap and scroll.
        27 => {
            let n = r.range(50, 400);
            let c = *r.pick(b"xX#=");
            for i in 0..n {
                out.push(if r.chance(1, 40) {
                    b'\n'
                } else {
                    c.wrapping_add((i % 3) as u8)
                });
            }
        }
        28 => {
            let n = r.range(5, 60);
            for i in 0..n {
                out.extend_from_slice(format!("line {}\r\n", i).as_bytes());
            }
        }
        // DCS / SOS / PM / APC strings, random raw bytes.
        _ => {
            if r.chance(1, 2) {
                out.extend_from_slice(r.pick(&[
                    &b"\x1bP"[..],
                    b"\x1bX",
                    b"\x1b^",
                    b"\x1b_",
                    "\u{90}".as_bytes(),
                ]));
                if r.chance(1, 2) {
                    out.extend_from_slice(params(r, 2).as_bytes());
                }
                let n = r.below(10) as usize;
                text(r, out, n);
                out.extend_from_slice(r.pick(&[&b"\x1b\\"[..], "\u{9c}".as_bytes(), b"\x07", b""]));
            } else {
                let n = r.range(1, 8);
                for _ in 0..n {
                    out.push(r.below(256) as u8);
                }
            }
        }
    }
}

fn gen_case(r: &mut Rng) -> Vec<u8> {
    let n = if r.chance(1, 10) {
        r.range(80, 300)
    } else {
        r.range(1, 60)
    };
    gen_pieces(r, n)
}

fn gen_pieces(r: &mut Rng, n: u64) -> Vec<u8> {
    let mut out = Vec::new();
    for _ in 0..n {
        piece(r, &mut out);
    }
    out
}

fn records(cases: &[Vec<u8>]) -> Vec<u8> {
    let mut v = Vec::new();
    for c in cases {
        v.extend_from_slice(&(c.len() as u32).to_be_bytes());
        v.extend_from_slice(c);
    }
    v
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    /// `batch`: compare framebuffer dumps.
    Dump,
    /// `frame-batch` with this `initialized` flag: compare new_frame bytes.
    Frame(bool),
    /// `frame-resize-batch`: like Frame, with a resize between old and new.
    FrameResize(bool),
}

/// One fuzz case: one input for Dump, an (old, new) pair for Frame, and
/// (old, new, 8-byte target size) for FrameResize.
type Case = Vec<Vec<u8>>;

struct Progs {
    ref_term: PathBuf,
    vt_check: PathBuf,
}

fn gen(r: &mut Rng, mode: Mode) -> Case {
    match mode {
        Mode::Dump => vec![gen_case(r)],
        Mode::Frame(_) | Mode::FrameResize(_) => {
            let old = gen_case(r);
            // Half the time a small update, like successive server snapshots.
            let new = if r.chance(1, 2) {
                let n = r.range(1, 5);
                gen_pieces(r, n)
            } else {
                gen_case(r)
            };
            let mut case = vec![old, new];
            if let Mode::FrameResize(_) = mode {
                // Random target, biased to small sizes and to 1-wide/1-tall.
                let w2 = if r.chance(1, 8) { 1 } else { r.range(1, 160) } as u32;
                let h2 = if r.chance(1, 8) { 1 } else { r.range(1, 60) } as u32;
                let mut z = w2.to_be_bytes().to_vec();
                z.extend_from_slice(&h2.to_be_bytes());
                case.push(z);
            }
            case
        }
    }
}

/// Run one program on a batch of cases; return each case's output, or None
/// if the program failed (crash, assertion) before printing every case.
fn run_batch(prog: &Path, mode: Mode, w: usize, h: usize, cases: &[Case]) -> Option<Vec<Vec<u8>>> {
    let mut args = vec![w.to_string(), h.to_string()];
    match mode {
        Mode::Dump => args.insert(0, "batch".into()),
        Mode::Frame(init) => {
            args.insert(0, "frame-batch".into());
            args.push(if init { "1" } else { "0" }.into());
        }
        Mode::FrameResize(init) => {
            args.insert(0, "frame-resize-batch".into());
            args.push(if init { "1" } else { "0" }.into());
        }
    }
    // The programs read (size, old, new) triples; cases keep the size last.
    let parts: Vec<Vec<u8>> = match mode {
        Mode::FrameResize(_) => cases
            .iter()
            .flat_map(|c| [c[2].clone(), c[0].clone(), c[1].clone()])
            .collect(),
        _ => cases.iter().flatten().cloned().collect(),
    };
    let input = records(&parts);
    let mut child = Command::new(prog)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {}: {}", prog.display(), e));
    let mut sin = child.stdin.take().expect("stdin");
    let writer = std::thread::spawn(move || {
        let _ = sin.write_all(&input);
    });
    let mut out = BufReader::with_capacity(1 << 20, child.stdout.take().expect("stdout"));
    let mut results = Vec::with_capacity(cases.len());
    match mode {
        Mode::Dump => {
            let mut cur = Vec::new();
            let mut line = Vec::new();
            loop {
                line.clear();
                if out.read_until(b'\n', &mut line).unwrap_or(0) == 0 {
                    break;
                }
                if line == b"=====\n" {
                    results.push(std::mem::take(&mut cur));
                } else {
                    cur.extend_from_slice(&line);
                }
            }
        }
        Mode::Frame(_) | Mode::FrameResize(_) => loop {
            let mut hdr = [0u8; 4];
            if out.read_exact(&mut hdr).is_err() {
                break;
            }
            let mut f = vec![0u8; u32::from_be_bytes(hdr) as usize];
            if out.read_exact(&mut f).is_err() {
                break;
            }
            results.push(f);
        },
    }
    let _ = out.read_to_end(&mut Vec::new());
    let status = child.wait().expect("wait");
    let _ = writer.join();
    if status.success() && results.len() == cases.len() {
        Some(results)
    } else {
        None
    }
}

/// Output for one case from one program; Err(()) if the program failed.
fn run_one(prog: &Path, mode: Mode, w: usize, h: usize, case: &Case) -> Result<Vec<u8>, ()> {
    run_batch(prog, mode, w, h, std::slice::from_ref(case))
        .and_then(|mut d| d.pop())
        .ok_or(())
}

fn differs(p: &Progs, mode: Mode, w: usize, h: usize, case: &Case) -> bool {
    run_one(&p.ref_term, mode, w, h, case) != run_one(&p.vt_check, mode, w, h, case)
}

/// Remove chunks of each part while the mismatch persists (delta debugging,
/// chunk halving), one part at a time.
fn minimize(p: &Progs, mode: Mode, w: usize, h: usize, mut case: Case) -> Case {
    // The size record of a FrameResize case is not minimized.
    let nparts = if let Mode::FrameResize(_) = mode {
        2
    } else {
        case.len()
    };
    for part in 0..nparts {
        let mut chunk = case[part].len().div_ceil(2).max(1);
        loop {
            let mut i = 0;
            while i < case[part].len() {
                let end = (i + chunk).min(case[part].len());
                let mut cand = case.clone();
                cand[part].drain(i..end);
                if differs(p, mode, w, h, &cand) {
                    case = cand;
                } else {
                    i += chunk;
                }
            }
            if chunk == 1 {
                break;
            }
            chunk = chunk.div_ceil(2);
        }
    }
    case
}

fn describe(r: &Result<Vec<u8>, ()>) -> Vec<u8> {
    match r {
        Ok(d) => d.clone(),
        Err(()) => b"<program failed (crash or assertion)>\n".to_vec(),
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|b| format!("{:02x}", b)).collect()
}

fn record_failure(p: &Progs, mode: Mode, seed: u64, n: usize, w: usize, h: usize, case: Case) {
    let min = minimize(p, mode, w, h, case);
    let dir = root().join("tests/corpus/fuzz-failures");
    std::fs::create_dir_all(&dir).expect("create fuzz-failures");
    let a = describe(&run_one(&p.ref_term, mode, w, h, &min));
    let b = describe(&run_one(&p.vt_check, mode, w, h, &min));
    let (base, what) = match mode {
        Mode::Dump => {
            let base = dir.join(format!("{}-{}", seed, n));
            std::fs::write(base.with_extension("bin"), &min[0]).expect("write failure");
            (base, format!("input {}", hex(&min[0])))
        }
        Mode::Frame(init) | Mode::FrameResize(init) => {
            let base = dir.join(format!("frame-{}-{}", seed, n));
            std::fs::write(base.with_extension("old.bin"), &min[0]).expect("write failure");
            std::fs::write(base.with_extension("new.bin"), &min[1]).expect("write failure");
            let resize = match min.get(2) {
                Some(z) => {
                    std::fs::write(base.with_extension("size.bin"), z).expect("write failure");
                    format!(" resize-to {}", hex(z))
                }
                None => String::new(),
            };
            (
                base,
                format!(
                    "initialized={} old {} new {}{}",
                    init as u8,
                    hex(&min[0]),
                    hex(&min[1]),
                    resize
                ),
            )
        }
    };
    std::fs::write(base.with_extension("ref-term.txt"), &a).expect("write output");
    std::fs::write(base.with_extension("vt-check.txt"), &b).expect("write output");
    println!(
        "MISMATCH case {} size {}x{}: {} ({})",
        n,
        w,
        h,
        what,
        base.display()
    );
}

fn usage() -> ! {
    eprintln!("usage: vt-fuzz --cases N --jobs J --seed S [--size WxH] [--frames]");
    std::process::exit(2);
}

fn main() {
    let mut cases: usize = 10000;
    let mut jobs: usize = 8;
    let mut seed: u64 = 1;
    let mut size: Option<(usize, usize)> = None;
    let mut frames = false;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--frames" {
            frames = true;
            i += 1;
            continue;
        }
        let v = args.get(i + 1).unwrap_or_else(|| usage());
        match args[i].as_str() {
            "--cases" => cases = v.parse().unwrap_or_else(|_| usage()),
            "--jobs" => jobs = v.parse().unwrap_or_else(|_| usage()),
            "--seed" => seed = v.parse().unwrap_or_else(|_| usage()),
            "--size" => {
                let (a, b) = v.split_once('x').unwrap_or_else(|| usage());
                size = Some((
                    a.parse().unwrap_or_else(|_| usage()),
                    b.parse().unwrap_or_else(|_| usage()),
                ));
            }
            _ => usage(),
        }
        i += 2;
    }

    let status = Command::new("make")
        .arg("-s")
        .arg("-C")
        .arg(root().join("ref/ref-term"))
        .status()
        .expect("make");
    assert!(status.success(), "make -C ref/ref-term FAILED");
    let exe = std::env::current_exe().expect("current_exe");
    let progs = Progs {
        ref_term: root().join("ref/ref-term/ref-term"),
        vt_check: exe.parent().expect("exe dir").join("vt-check"),
    };
    assert!(
        progs.vt_check.exists(),
        "vt-check not built next to vt-fuzz: {}",
        progs.vt_check.display()
    );

    let start = Instant::now();
    let nbatches = cases.div_ceil(BATCH);
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let mismatches = AtomicUsize::new(0);
    let crashes = AtomicUsize::new(0);
    let print_lock = Mutex::new(());

    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| loop {
                let b = next.fetch_add(1, Ordering::Relaxed);
                if b >= nbatches {
                    return;
                }
                let first = b * BATCH;
                let count = BATCH.min(cases - first);
                let (w, h) = size.unwrap_or(SIZES[b % SIZES.len()]);
                // --frames: even batches plain pairs, odd batches with a
                // resize between old and new; one batch in four uninitialized.
                let mode = if frames && b % 2 == 1 {
                    Mode::FrameResize(b % 4 != 3)
                } else if frames {
                    Mode::Frame(b % 4 != 2)
                } else {
                    Mode::Dump
                };
                let mut rng = Rng(seed.wrapping_mul(0x2545F4914F6CDD1D)
                    ^ (b as u64).wrapping_mul(0x9E3779B97F4A7C15));
                let batch: Vec<Case> = (0..count).map(|_| gen(&mut rng, mode)).collect();
                let ra = run_batch(&progs.ref_term, mode, w, h, &batch);
                let rb = run_batch(&progs.vt_check, mode, w, h, &batch);
                let bad: Vec<usize> = match (&ra, &rb) {
                    (Some(a), Some(b)) => (0..count).filter(|&k| a[k] != b[k]).collect(),
                    _ => {
                        // A program failed somewhere in the batch: compare case by case.
                        let mut v = Vec::new();
                        for (k, c) in batch.iter().enumerate() {
                            let x = run_one(&progs.ref_term, mode, w, h, c);
                            let y = run_one(&progs.vt_check, mode, w, h, c);
                            if x.is_err() || y.is_err() {
                                crashes.fetch_add(1, Ordering::Relaxed);
                            }
                            if x != y {
                                v.push(k);
                            }
                        }
                        v
                    }
                };
                for k in bad {
                    // Minimize and save the first MAX_SAVED; count the rest.
                    if mismatches.fetch_add(1, Ordering::Relaxed) < MAX_SAVED {
                        let _g = print_lock.lock().unwrap();
                        record_failure(&progs, mode, seed, first + k, w, h, batch[k].clone());
                    }
                }
                let d = done.fetch_add(count, Ordering::Relaxed) + count;
                if (d / BATCH).is_multiple_of(50) {
                    let _g = print_lock.lock().unwrap();
                    eprintln!(
                        "progress cases={} mismatches={} seconds={:.0}",
                        d,
                        mismatches.load(Ordering::Relaxed),
                        start.elapsed().as_secs_f64()
                    );
                }
            });
        }
    });

    println!(
        "cases={} mismatches={} crashes_in_either={} seconds={:.1}",
        done.load(Ordering::Relaxed),
        mismatches.load(Ordering::Relaxed),
        crashes.load(Ordering::Relaxed),
        start.elapsed().as_secs_f64()
    );
    if mismatches.load(Ordering::Relaxed) != 0 {
        std::process::exit(1);
    }
}
