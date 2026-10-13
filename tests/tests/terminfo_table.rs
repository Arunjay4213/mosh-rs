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
// Ported from: (none; conformance test for upstream/src/terminal/terminaldisplayinit.cc)
//
// The generated terminfo table (crates/terminal/src/terminfo.rs, PLAN.md 9.6)
// must give the decisions upstream's Display(true) makes. For every terminal
// type name in `toe -a`, every alias in the table, and an unknown name, with
// MOSH_NO_TERM_INIT unset and set: ref-term `caps` (upstream's constructor,
// real setupterm) and vt-check `caps` must print the same line. Where upstream
// throws (hardcopy or unknown types), the port must fall back to upstream's
// xterm answers (title is not compared there: upstream throws before deciding it).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use mosh_tests::oracle::build_ref_term;

fn caps(prog: &Path, names: &str, no_term_init: bool) -> Vec<String> {
    let mut cmd = Command::new(prog);
    cmd.arg("caps").stdin(Stdio::piped()).stdout(Stdio::piped());
    if no_term_init {
        cmd.env("MOSH_NO_TERM_INIT", "1");
    } else {
        cmd.env_remove("MOSH_NO_TERM_INIT");
    }
    let mut child = cmd.spawn().expect("spawn caps");
    let mut sin = child.stdin.take().expect("stdin");
    let input = names.to_string();
    let writer = std::thread::spawn(move || {
        let _ = sin.write_all(input.as_bytes());
    });
    let out = child.wait_with_output().expect("wait");
    writer.join().expect("writer");
    assert!(out.status.success(), "{} caps failed", prog.display());
    String::from_utf8(out.stdout)
        .expect("caps output is ASCII")
        .lines()
        .map(str::to_string)
        .collect()
}

/// The terminfo fields of a caps line (everything but name and title).
fn terminfo_fields(line: &str) -> Vec<&str> {
    let f: Vec<&str> = line.split(' ').collect();
    // name ech E bce B title T smcup S rmcup R
    vec![f[1], f[2], f[3], f[4], f[7], f[8], f[9], f[10]]
}

#[test]
fn terminfo_table() {
    let ref_term = build_ref_term();
    let vt_check = PathBuf::from(env!("CARGO_BIN_EXE_vt-check"));

    // The table records the terminfo packages it was generated from; upstream
    // reads the local database, so the comparison is only meaningful when the
    // local packages are the same.
    let table =
        std::fs::read_to_string(mosh_tests::oracle::root().join("crates/terminal/src/terminfo.rs"))
            .expect("read terminfo.rs");
    let recorded = table
        .lines()
        .find(|l| l.starts_with("// ncurses "))
        .expect("terminfo.rs header names its packages")
        .to_string();
    let version = |pkg: &str| {
        let o = Command::new("dpkg-query")
            .args(["-W", "-f=${Version}", pkg])
            .output()
            .expect("run dpkg-query");
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    let local = format!(
        "// ncurses {} (ncurses-base, ncurses-term), kitty-terminfo {}",
        version("ncurses-term"),
        version("kitty-terminfo")
    );
    assert_eq!(
        local, recorded,
        "local terminfo packages differ from the ones terminfo.rs was generated from; \
         install them (apt-get install ncurses-term kitty-terminfo) or regenerate with \
         `make -C ref/ref-term terminfo`"
    );

    let toe = Command::new("toe").arg("-a").output().expect("run toe -a");
    assert!(toe.status.success(), "toe -a failed");
    let mut names: Vec<String> = String::from_utf8_lossy(&toe.stdout)
        .lines()
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .collect();
    let from_toe = names.len();
    names.extend(
        mosh_terminal::terminfo::NAMES
            .iter()
            .map(|(n, _)| n.to_string()),
    );
    names.push("no-such-term".to_string());
    names.sort();
    names.dedup();
    let input = names.join("\n") + "\nxterm\n";

    let (mut compared, mut fallbacks, mut mismatches) = (0usize, 0usize, 0usize);
    for no_term_init in [false, true] {
        let a = caps(&ref_term, &input, no_term_init);
        let b = caps(&vt_check, &input, no_term_init);
        assert_eq!(a.len(), names.len() + 1);
        assert_eq!(b.len(), names.len() + 1);
        let xterm = a.last().expect("xterm line").clone();
        assert!(!xterm.contains(" error "), "upstream has no xterm entry");
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate().take(names.len()) {
            compared += 1;
            let ok = if x.contains(" error ") {
                fallbacks += 1;
                terminfo_fields(y) == terminfo_fields(&xterm)
            } else {
                x == y
            };
            if !ok {
                mismatches += 1;
                println!(
                    "MISMATCH TERM={} MOSH_NO_TERM_INIT={}\n    ref-term: {}\n    vt-check: {}",
                    names[i], no_term_init as u8, x, y
                );
            }
        }
    }

    println!(
        "terms={} (toe -a {} plus aliases) compared={} upstream_errors_fell_back_to_xterm={} mismatches={}",
        names.len(),
        from_toe,
        compared,
        fallbacks,
        mismatches
    );
    assert!(
        names.len() > 1800,
        "too few terminal types: {}",
        names.len()
    );
    assert_eq!(mismatches, 0, "terminfo table FAILED");
}
