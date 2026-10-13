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

use std::path::PathBuf;

use mosh_tests::oracle::{
    build_ref_term, corpus_files, first_difference, prefix_offsets, record, run, split_records,
};

const SIZES: [(usize, usize); 2] = [(80, 24), (132, 43)];
const PREFIXES: usize = 40;

#[test]
fn terminal_conformance() {
    let ref_term = build_ref_term();
    let vt_check = PathBuf::from(env!("CARGO_BIN_EXE_vt-check"));

    let files = corpus_files(&["tests/corpus/emulation", "tests/corpus/sessions"]);
    assert!(!files.is_empty(), "no corpus files");
    let mut failures = 0usize;
    let mut dumps = 0usize;
    let mut frames = 0usize;

    for file in &files {
        let name = file
            .strip_prefix(mosh_tests::oracle::root())
            .unwrap_or(file)
            .display()
            .to_string();
        let data = std::fs::read(file).expect("read corpus file");

        // Cumulative prefixes at seeded random offsets, plus the whole file.
        let offsets = prefix_offsets(&data, PREFIXES);

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
