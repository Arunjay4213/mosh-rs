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
//
// Ported from: (new; no upstream equivalent) test helper for
// tests/tests/platform_tty.rs. Exercises mosh_platform::tty and ::select on
// the terminal it runs in.

use std::io::Write;
use std::process::{exit, Command};

use mosh_platform::select::{Select, SIGWINCH};
use mosh_platform::tty;

fn say(line: &str) {
    let mut out = std::io::stdout();
    out.write_all(line.as_bytes()).unwrap();
    out.write_all(b"\n").unwrap();
    out.flush().unwrap();
}

fn size_line() -> String {
    let (cols, rows) = tty::window_size().expect("window_size");
    format!("size {cols}x{rows}")
}

fn stty() {
    let status = Command::new("stty").arg("-a").status().expect("run stty");
    assert!(status.success(), "stty -a failed");
}

fn main() {
    let mut sel = Select::new();
    sel.add_signal(SIGWINCH);

    say(&size_line());

    let guard = tty::raw_mode_on().expect("raw_mode_on");
    say("raw on");
    stty();
    say("waiting for SIGWINCH");

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let left = deadline.saturating_duration_since(std::time::Instant::now());
        if left.is_zero() {
            say("no SIGWINCH within 5 s");
            exit(1);
        }
        sel.select(left.as_millis() as i32).expect("select");
        if sel.signal(SIGWINCH) {
            break;
        }
    }
    say(&size_line());

    say("big start");
    tty::write_all_stdout(&vec![b'x'; 1 << 20]).expect("write_all_stdout");
    say("\nbig end");

    tty::raw_mode_off(guard).expect("raw_mode_off");
    say("raw off");
    stty();
    exit(0);
}
