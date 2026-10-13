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
// Ported from: (new; no upstream equivalent)
//
// Runs the tty-probe helper under a real pseudoterminal made by
// mosh_platform::pty::forkpty and checks what a user of mosh-client relies
// on: raw mode really turns off canonical input, echo and signals; leaving
// raw mode really restores them; a window resize reaches the event loop as
// SIGWINCH with the new size; a large write arrives complete.

#![cfg(unix)]

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use mosh_platform::pty::{self, Fork};

/// Wait until the collected output contains `needle`, return everything so far.
fn wait_for(out: &Arc<Mutex<Vec<u8>>>, needle: &str, timeout: Duration) -> String {
    let deadline = Instant::now() + timeout;
    loop {
        let text = String::from_utf8_lossy(&out.lock().unwrap()).into_owned();
        if text.contains(needle) {
            return text;
        }
        assert!(Instant::now() < deadline, "timed out waiting for {needle:?}; output so far:\n{text}");
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn between<'a>(text: &'a str, start: &str, end: &str) -> &'a str {
    let a = text.find(start).unwrap_or_else(|| panic!("missing {start:?}")) + start.len();
    let b = a + text[a..].find(end).unwrap_or_else(|| panic!("missing {end:?}"));
    &text[a..b]
}

fn has_token(stty: &str, token: &str) -> bool {
    stty.split_whitespace().any(|t| t.trim_end_matches(';') == token)
}

#[test]
fn raw_mode_resize_and_large_write_through_a_pty() {
    let probe = env!("CARGO_BIN_EXE_tty-probe").to_string();
    let (master, child) = match pty::forkpty(100, 30).expect("forkpty") {
        Fork::Child => {
            let err = pty::exec(&[probe], &[("LC_ALL".into(), "C".into())]);
            eprintln!("exec tty-probe: {err}");
            unsafe { libc::_exit(127) };
        }
        Fork::Parent { master, child } => (master, child),
    };

    let out = Arc::new(Mutex::new(Vec::new()));
    let reader = {
        let out = out.clone();
        let mut file = File::from(master.try_clone().unwrap());
        std::thread::spawn(move || {
            let mut buf = [0u8; 65536];
            // Linux returns EIO on the master once the child side is closed.
            while let Ok(n) = file.read(&mut buf) {
                if n == 0 {
                    break;
                }
                out.lock().unwrap().extend_from_slice(&buf[..n]);
            }
        })
    };

    let text = wait_for(&out, "waiting for SIGWINCH", Duration::from_secs(10));
    assert!(text.contains("size 100x30"), "initial size not 100x30:\n{text}");
    let raw_stty = between(&text, "raw on", "waiting for SIGWINCH");
    println!("stty -a in raw mode:\n{}", raw_stty.trim());
    for token in ["-icanon", "-echo", "-isig"] {
        assert!(has_token(raw_stty, token), "raw-mode stty lacks {token}:\n{raw_stty}");
    }

    pty::set_window_size(&master, 120, 40).expect("TIOCSWINSZ");
    wait_for(&out, "size 120x40", Duration::from_secs(10));

    let status = pty::wait_child(child).expect("waitpid");
    reader.join().unwrap();
    let text = String::from_utf8_lossy(&out.lock().unwrap()).into_owned();

    let big = between(&text, "big start\n", "\nbig end");
    let x_count = big.bytes().filter(|&b| b == b'x').count();
    println!("1 MB write: {} bytes received, {} of them 'x'", big.len(), x_count);
    assert_eq!(big.len(), 1 << 20);
    assert_eq!(x_count, 1 << 20);

    let cooked_stty = &text[text.find("raw off").expect("missing raw off")..];
    println!("stty -a after raw_mode_off:\n{}", cooked_stty.trim());
    for token in ["icanon", "echo", "isig"] {
        assert!(has_token(cooked_stty, token), "restored stty lacks {token}:\n{cooked_stty}");
    }
    for token in ["-icanon", "-echo", "-isig"] {
        assert!(!has_token(cooked_stty, token), "restored stty still has {token}:\n{cooked_stty}");
    }
    assert!(status.success(), "tty-probe exited with {status:?}:\n{text}");
}
