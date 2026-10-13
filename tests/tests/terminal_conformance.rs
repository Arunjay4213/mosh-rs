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
// Ported from: (none; conformance test for the port of upstream/src/terminal)
//
// Terminal emulator conformance: the Rust port (vt-check) must produce
// byte-identical framebuffer dumps and Display::new_frame output to upstream
// mosh's own C++ code (ref/ref-term/ref-term) on every corpus file:
// upstream's emulation test streams (tests/corpus/emulation) and recorded
// real sessions (tests/corpus/sessions, when present).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SIZES: [(usize, usize); 2] = [(80, 24), (132, 43)];
const PREFIXES: usize = 40;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn run(prog: &Path, args: &[String], stdin: &[u8]) -> Vec<u8> {
    let mut child = Command::new(prog)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {}: {}", prog.display(), e));
    let mut sin = child.stdin.take().expect("stdin");
    let input = stdin.to_vec();
    let writer = std::thread::spawn(move || {
        let _ = sin.write_all(&input);
    });
    let out = child.wait_with_output().expect("wait");
    writer.join().expect("stdin writer");
    assert!(
        out.status.success(),
        "{} {:?} exited with {}",
        prog.display(),
        args,
        out.status
    );
    out.stdout
}

fn corpus_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in ["tests/corpus/emulation", "tests/corpus/sessions"] {
        let dir = root().join(dir);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in entries {
            let p = e.expect("dir entry").path();
            if p.extension().is_some_and(|x| x == "bin") {
                files.push(p);
            }
        }
    }
    files.sort();
    files
}

fn record(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}

fn split_records(mut b: &[u8]) -> Vec<&[u8]> {
    let mut v = Vec::new();
    while !b.is_empty() {
        let n = u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize;
        v.push(&b[4..4 + n]);
        b = &b[4 + n..];
    }
    v
}

/// splitmix64, so the prefix offsets are the same on every run.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

fn first_difference(a: &[u8], b: &[u8]) -> String {
    let (la, lb): (Vec<&[u8]>, Vec<&[u8]>) = (
        a.split(|&c| c == b'\n').collect(),
        b.split(|&c| c == b'\n').collect(),
    );
    for i in 0..la.len().max(lb.len()) {
        let x = la.get(i).copied().unwrap_or(b"<eof>");
        let y = lb.get(i).copied().unwrap_or(b"<eof>");
        if x != y {
            return format!(
                "line {}:\n    ref-term: {}\n    vt-check: {}",
                i + 1,
                String::from_utf8_lossy(x).escape_debug(),
                String::from_utf8_lossy(y).escape_debug()
            );
        }
    }
    "identical".to_string()
}

#[test]
fn terminal_conformance() {
    let status = Command::new("make")
        .arg("-C")
        .arg(root().join("ref/ref-term"))
        .status()
        .expect("run make");
    assert!(status.success(), "make -C ref/ref-term FAILED");
    let ref_term = root().join("ref/ref-term/ref-term");
    let vt_check = PathBuf::from(env!("CARGO_BIN_EXE_vt-check"));

    let files = corpus_files();
    assert!(!files.is_empty(), "no corpus files");
    let mut failures = 0usize;
    let mut dumps = 0usize;
    let mut frames = 0usize;

    for file in &files {
        let name = file
            .strip_prefix(root())
            .unwrap_or(file)
            .display()
            .to_string();
        let data = std::fs::read(file).expect("read corpus file");

        // Cumulative prefixes at seeded random offsets, plus the whole file.
        let seed = data.iter().fold(0xcbf29ce484222325u64, |h, &b| {
            (h ^ b as u64).wrapping_mul(0x100000001b3)
        });
        let mut rng = Rng(seed);
        let mut offsets: Vec<usize> = (0..PREFIXES)
            .map(|_| (rng.next() % (data.len() as u64 + 1)) as usize)
            .collect();
        offsets.sort();
        offsets.push(data.len());

        // (old, new) pairs: old = bytes up to offset i, new = bytes from offset i to offset i+1.
        let mut pairs = Vec::new();
        let mut prev = 0usize;
        for &o in &offsets {
            record(&mut pairs, &data[..prev]);
            record(&mut pairs, &data[prev..o]);
            prev = o;
        }
        let npairs = offsets.len();
        // One uninitialized frame: empty -> middle prefix.
        let mid = offsets[PREFIXES / 2];
        let mut uninit = Vec::new();
        record(&mut uninit, &data[..0]);
        record(&mut uninit, &data[..mid]);

        for (w, h) in SIZES {
            let size = [w.to_string(), h.to_string()];
            let args = |mode: &str, extra: &[&str]| {
                let mut a = vec![mode.to_string()];
                a.extend(size.iter().cloned());
                a.extend(extra.iter().map(|s| s.to_string()));
                a
            };

            let a = run(&ref_term, &args("dump", &[]), &data);
            let b = run(&vt_check, &args("dump", &[]), &data);
            dumps += 1;
            if a != b {
                failures += 1;
                println!(
                    "DUMP MISMATCH {} {}x{}: {}",
                    name,
                    w,
                    h,
                    first_difference(&a, &b)
                );
            }

            for (input, init, count) in [(&pairs, "1", npairs), (&uninit, "0", 1)] {
                let a = run(&ref_term, &args("frame-batch", &[init]), input);
                let b = run(&vt_check, &args("frame-batch", &[init]), input);
                let (ra, rb) = (split_records(&a), split_records(&b));
                assert_eq!(ra.len(), count);
                assert_eq!(rb.len(), count);
                frames += count;
                for (i, (x, y)) in ra.iter().zip(rb.iter()).enumerate() {
                    if x != y {
                        failures += 1;
                        let p = x
                            .iter()
                            .zip(y.iter())
                            .position(|(p, q)| p != q)
                            .unwrap_or(x.len().min(y.len()));
                        println!(
                            "FRAME MISMATCH {} {}x{} pair {} initialized={}: first difference at byte {}\n    ref-term: {}\n    vt-check: {}",
                            name,
                            w,
                            h,
                            i,
                            init,
                            p,
                            String::from_utf8_lossy(&x[p.saturating_sub(20)..x.len().min(p + 40)]).escape_debug(),
                            String::from_utf8_lossy(&y[p.saturating_sub(20)..y.len().min(p + 40)]).escape_debug()
                        );
                    }
                }
            }
        }
    }

    println!(
        "corpus files={} dumps compared={} frames compared={} mismatches={}",
        files.len(),
        dumps,
        frames,
        failures
    );
    assert_eq!(failures, 0, "terminal conformance FAILED");
}
