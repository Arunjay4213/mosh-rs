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
// Ported from: (none; shared driver for the ref/ oracles in the tests)

//! Helpers shared by the conformance tests: workspace paths, running the
//! `ref-term` oracle, corpus discovery, record framing, seeded offsets.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The workspace root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

/// Runs `prog args` with `stdin` as input; returns stdout; panics if it fails.
pub fn run(prog: &Path, args: &[String], stdin: &[u8]) -> Vec<u8> {
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

/// Every `*.bin` file directly inside the given directories (relative to the
/// workspace root), sorted. Missing directories are skipped.
pub fn corpus_files(dirs: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in dirs {
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

/// Appends one record: a big-endian u32 length, then the bytes.
pub fn record(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}

/// Splits length-prefixed records (see `record`).
pub fn split_records(mut b: &[u8]) -> Vec<&[u8]> {
    let mut v = Vec::new();
    while !b.is_empty() {
        let n = u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize;
        v.push(&b[4..4 + n]);
        b = &b[4 + n..];
    }
    v
}

/// splitmix64, so the prefix offsets are the same on every run.
pub struct Rng(pub u64);
impl Rng {
    /// The next pseudo-random value.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// The first differing line of two dumps, for failure messages.
pub fn first_difference(a: &[u8], b: &[u8]) -> String {
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

/// Builds `ref/ref-term` (panics loudly if make fails) and returns its path.
pub fn build_ref_term() -> PathBuf {
    let status = Command::new("make")
        .arg("-s")
        .arg("-C")
        .arg(root().join("ref/ref-term"))
        .status()
        .expect("run make");
    assert!(status.success(), "make -C ref/ref-term FAILED");
    root().join("ref/ref-term/ref-term")
}

/// A seed derived from the file contents (FNV-1a), so offsets are stable.
pub fn seed_of(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf29ce484222325u64, |h, &b| {
        (h ^ b as u64).wrapping_mul(0x100000001b3)
    })
}

/// `n` sorted pseudo-random offsets into `data` plus `data.len()` at the end.
pub fn prefix_offsets(data: &[u8], n: usize) -> Vec<usize> {
    let mut rng = Rng(seed_of(data));
    let mut offsets: Vec<usize> = (0..n)
        .map(|_| (rng.next_u64() % (data.len() as u64 + 1)) as usize)
        .collect();
    offsets.sort();
    offsets.push(data.len());
    offsets
}
