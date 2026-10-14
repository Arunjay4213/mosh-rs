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
// Ported from: (new; mosh-rs scrollback extension, PLAN.md sections 6.4 and 9.8)
//
// A client in scrollback mode against a server that holds the session key
// but sends crafted history lines (tests/src/bin/hostile-scrollback-server.rs),
// from the review reviews/2026-10-13-phase2-4-client-server-scrollback.md:
// - G1: one line of 1,000,000 line feeds (about 1 KB on the wire) made the
//   client keep one row per line feed: 4.1 GB. Such a line is now rejected
//   (longer than the line limit), and a shorter one that still makes too
//   many host rows is cut to a bound with a dim notice.
// - G2: a line numbered `u64::MAX` wrapped the client's numbering.
// Each case runs the real mosh-client (target/release, MOSH_SCROLLBACK=1)
// in a tmux pane and reads its peak memory (VmHWM) from /proc.
//
// Linux only (/proc). UDP ports 61410-61419; holds /tmp/mosh-rs-interop.lock.

#![cfg(target_os = "linux")]

use std::io::{BufRead, BufReader};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

fn release_dir() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let status = Command::new(env!("CARGO"))
        .args(["build", "--release", "-p", "mosh", "--bin", "mosh-client"])
        .args(["-p", "mosh-tests", "--bin", "hostile-scrollback-server"])
        .current_dir(&root)
        .status()
        .expect("run cargo build");
    assert!(status.success(), "cargo build failed");
    std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"))
        .join("release")
}

/// What one case observed.
struct Outcome {
    /// The client's peak resident memory, kB.
    peak_kb: u64,
    /// Whether the client drew the server's frame (it accepted the diff).
    ready: bool,
    /// Seconds until the frame showed, or the whole wait.
    seconds: f64,
    /// Whether the client was still running at the end.
    alive: bool,
    /// The host's history and screen.
    capture: String,
}

fn vm_hwm_kb(pid: &str) -> Option<u64> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

fn tmux(sock: &str, args: &[&str]) -> String {
    let out = Command::new("tmux")
        .args(["-S", sock])
        .args(args)
        .output()
        .expect("run tmux");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Runs the client against `hostile-scrollback-server PORT kind size` for
/// up to `wait`; gives up early (and reports) above 2 GB.
fn run_case(dir: &Path, port: u16, kind: &str, size: u64, wait: Duration) -> Outcome {
    let mut server: Child = Command::new(dir.join("hostile-scrollback-server"))
        .args([&port.to_string(), kind, &size.to_string(), "120"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("run hostile-scrollback-server");
    let mut first = String::new();
    BufReader::new(server.stdout.take().expect("stdout"))
        .read_line(&mut first)
        .expect("MOSH CONNECT line");
    let key = first.split_whitespace().nth(3).expect("key").to_string();

    let sock = std::env::temp_dir()
        .join(format!(
            "mosh-rs-hostile-{}-{port}.sock",
            std::process::id()
        ))
        .display()
        .to_string();
    let conf = std::env::temp_dir().join(format!("mosh-rs-hostile-{}.conf", std::process::id()));
    std::fs::write(&conf, "set -g history-limit 200000\n").expect("tmux conf");
    let command = format!(
        "exec env MOSH_KEY={key} MOSH_SCROLLBACK=1 LANG=C.UTF-8 {} 127.0.0.1 {port}",
        dir.join("mosh-client").display()
    );
    let status = Command::new("tmux")
        .arg("-f")
        .arg(&conf)
        .args([
            "-S",
            &sock,
            "new-session",
            "-d",
            "-x",
            "80",
            "-y",
            "24",
            &command,
        ])
        .status()
        .expect("run tmux");
    assert!(status.success());
    let pid = tmux(&sock, &["list-panes", "-F", "#{pane_pid}"])
        .trim()
        .to_string();

    let start = Instant::now();
    let (mut peak_kb, mut ready) = (0, false);
    while start.elapsed() < wait {
        if let Some(kb) = vm_hwm_kb(&pid) {
            peak_kb = kb;
        }
        if tmux(&sock, &["capture-pane", "-p"]).contains("FAKE-READY") {
            ready = true;
            // Let the injected history settle.
            std::thread::sleep(Duration::from_millis(500));
            if let Some(kb) = vm_hwm_kb(&pid) {
                peak_kb = kb;
            }
            break;
        }
        if peak_kb > 2 << 20 {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let seconds = start.elapsed().as_secs_f64();
    let alive = vm_hwm_kb(&pid).is_some();
    let capture = tmux(&sock, &["capture-pane", "-p", "-S", "-200000"]);
    let _ = Command::new("kill").args(["-9", &pid]).status();
    tmux(&sock, &["kill-server"]);
    let _ = server.kill();
    let _ = server.wait();
    Outcome {
        peak_kb,
        ready,
        seconds,
        alive,
        capture,
    }
}

#[test]
fn hostile_history_lines_stay_bounded() {
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/mosh-rs-interop.lock")
        .expect("open /tmp/mosh-rs-interop.lock");
    // SAFETY: flock on a descriptor we own; released when `lock` drops.
    assert_eq!(unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX) }, 0);
    let dir = release_dir();
    let notice = "[mosh: ";

    // (kind, size, accepted, what)
    let cases: [(&str, u64, bool, &str); 4] = [
        (
            "newlines",
            1_000_000,
            false,
            "1,000,000 line feeds: rejected",
        ),
        (
            "newlines",
            200_000,
            true,
            "200,000 line feeds: cut to a bound",
        ),
        ("letters", 200_000, true, "200,000 letters: cut to a bound"),
        ("maxindex", 0, false, "line number u64::MAX: rejected"),
    ];
    let mut failures = Vec::new();
    for (k, (kind, size, accepted, what)) in cases.into_iter().enumerate() {
        let o = run_case(
            &dir,
            61410 + 2 * k as u16,
            kind,
            size,
            Duration::from_secs(10),
        );
        let rows = o.capture.lines().count();
        println!(
            "{what}: peak {} kB, frame shown {} after {:.2} s, client alive {}, {} host lines, notice {}",
            o.peak_kb,
            o.ready,
            o.seconds,
            o.alive,
            rows,
            o.capture.contains(notice)
        );
        if !o.alive {
            failures.push(format!("{what}: the client died"));
        }
        if o.peak_kb > 64 << 10 {
            failures.push(format!("{what}: the client used {} kB", o.peak_kb));
        }
        if o.ready != accepted {
            failures.push(format!(
                "{what}: frame shown {} (expected {accepted})",
                o.ready
            ));
        }
        if accepted && !o.capture.contains(notice) {
            failures.push(format!("{what}: no notice line for the cut rows"));
        }
        if o.capture.contains("HOSTILE") {
            failures.push(format!("{what}: the line numbered u64::MAX was shown"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
