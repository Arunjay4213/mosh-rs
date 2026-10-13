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

//! pty-record: run a command under a pseudoterminal, drive it from a script,
//! and record every byte it writes.
//!
//! pty-record --cols 80 --rows 24 --out FILE --script SCRIPT -- COMMAND ARGS...
//!
//! The child gets TERM=xterm-256color and LANG=C.UTF-8, with COLUMNS and
//! LINES removed, so programs size themselves from the pty. FILE receives the
//! raw output bytes. FILE.meta receives command, size, byte count, duration
//! and exit status.
//!
//! Script lines (blank lines and lines starting with '#' are ignored):
//!   wait MS             keep recording for MS milliseconds
//!   send TEXT           write TEXT to the child; escapes \n \r \t \\ \xHH
//!   resize COLSxROWS    TIOCSWINSZ on the pty, then SIGWINCH to the
//!                       foreground process group
//!   expect TEXT MS      wait until TEXT (escapes allowed) appears in the
//!                       output after the previous expect's match; fail if it
//!                       does not appear within MS milliseconds
//!   end                 send nothing more; wait up to 10 s for the child to
//!                       exit, then SIGHUP it
//! A script that runs out of lines behaves as if it ended with `end`.

use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::process::exit;
use std::time::{Duration, Instant};

use nix::errno::Errno;
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use nix::pty::{forkpty, ForkptyResult, Winsize};
use nix::sys::signal::{kill, killpg, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{execvp, Pid};

const END_WAIT: Duration = Duration::from_secs(10);

fn die(msg: &str) -> ! {
    eprintln!("pty-record: {msg}");
    exit(1);
}

fn usage() -> ! {
    eprintln!("usage: pty-record --cols N --rows N --out FILE --script SCRIPT -- COMMAND ARGS...");
    exit(2);
}

fn unescape(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 1 < b.len() {
            match b[i + 1] {
                b'n' => out.push(b'\n'),
                b'r' => out.push(b'\r'),
                b't' => out.push(b'\t'),
                b'\\' => out.push(b'\\'),
                b'x' if i + 4 <= b.len() => {
                    let hex = std::str::from_utf8(&b[i + 2..i + 4]).unwrap_or("");
                    match u8::from_str_radix(hex, 16) {
                        Ok(v) => out.push(v),
                        Err(_) => die(&format!("bad \\x escape in {s:?}")),
                    }
                    i += 4;
                    continue;
                }
                _ => die(&format!("unknown escape in {s:?}")),
            }
            i += 2;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    out
}

fn winsize(cols: u16, rows: u16) -> Winsize {
    Winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    }
}

fn parse_size(s: &str) -> (u16, u16) {
    let (c, r) = s
        .split_once('x')
        .unwrap_or_else(|| die(&format!("bad size {s:?}")));
    match (c.parse(), r.parse()) {
        (Ok(c), Ok(r)) => (c, r),
        _ => die(&format!("bad size {s:?}")),
    }
}

struct Session {
    master: OwnedFd,
    child: Pid,
    out: File,
    recorded: Vec<u8>,
    /// Output position where the next `expect` starts searching.
    expect_from: usize,
    /// Set once the child is reaped.
    status: Option<WaitStatus>,
    /// Set once the pty reports EOF/EIO (no writer left on the slave side).
    eof: bool,
}

impl Session {
    /// Read whatever is available, waiting at most `timeout`.
    fn pump(&mut self, timeout: Duration) {
        if self.eof {
            self.reap(false);
            std::thread::sleep(timeout.min(Duration::from_millis(50)));
            return;
        }
        let ms = timeout.as_millis().min(i32::MAX as u128) as i32;
        let mut fds = [PollFd::new(self.master.as_fd(), PollFlags::POLLIN)];
        match poll(
            &mut fds,
            PollTimeout::try_from(ms).unwrap_or(PollTimeout::MAX),
        ) {
            Ok(0) | Err(Errno::EINTR) => return,
            Ok(_) => {}
            Err(e) => die(&format!("poll: {e}")),
        }
        let mut buf = [0u8; 65536];
        match nix::unistd::read(&self.master, &mut buf) {
            Ok(0) | Err(Errno::EIO) => self.eof = true,
            Ok(n) => {
                self.recorded.extend_from_slice(&buf[..n]);
                self.out
                    .write_all(&buf[..n])
                    .unwrap_or_else(|e| die(&format!("write output: {e}")));
            }
            Err(Errno::EINTR) | Err(Errno::EAGAIN) => {}
            Err(e) => die(&format!("read pty: {e}")),
        }
    }

    fn pump_for(&mut self, d: Duration) {
        let deadline = Instant::now() + d;
        while let Some(left) = deadline.checked_duration_since(Instant::now()) {
            if left.is_zero() {
                break;
            }
            self.pump(left);
        }
    }

    fn reap(&mut self, block: bool) {
        if self.status.is_some() {
            return;
        }
        let flags = if block {
            None
        } else {
            Some(WaitPidFlag::WNOHANG)
        };
        match waitpid(self.child, flags) {
            Ok(WaitStatus::StillAlive) => {}
            Ok(s) => self.status = Some(s),
            Err(e) => die(&format!("waitpid: {e}")),
        }
    }

    fn send(&mut self, bytes: &[u8]) {
        let mut rest = bytes;
        while !rest.is_empty() {
            match nix::unistd::write(&self.master, rest) {
                Ok(n) => rest = &rest[n..],
                Err(Errno::EINTR) | Err(Errno::EAGAIN) => self.pump(Duration::from_millis(10)),
                Err(e) => die(&format!("write to child: {e}")),
            }
        }
    }

    fn expect(&mut self, text: &[u8], timeout: Duration) {
        let deadline = Instant::now() + timeout;
        let mut from = self.expect_from;
        loop {
            if let Some(pos) = self.recorded[from..]
                .windows(text.len())
                .position(|w| w == text)
            {
                self.expect_from = from + pos + text.len();
                return;
            }
            from = self
                .recorded
                .len()
                .saturating_sub(text.len() - 1)
                .max(self.expect_from);
            let now = Instant::now();
            if now >= deadline || (self.eof && self.status.is_some()) {
                let tail = &self.recorded[self.recorded.len().saturating_sub(300)..];
                eprintln!(
                    "pty-record: expect {:?} failed after {:?}; last output: {:?}",
                    String::from_utf8_lossy(text),
                    timeout,
                    String::from_utf8_lossy(tail)
                );
                let _ = kill(self.child, Signal::SIGKILL);
                exit(1);
            }
            self.pump(deadline - now);
        }
    }

    /// Wait for the child to exit and the pty to drain; SIGHUP after END_WAIT.
    fn finish(&mut self) {
        let mut deadline = Instant::now() + END_WAIT;
        let mut hup_sent = false;
        loop {
            self.reap(false);
            if self.status.is_some() && self.eof {
                return;
            }
            if self.status.is_some() {
                // Child gone; drain output still buffered or written by leftovers.
                let before = self.recorded.len();
                self.pump(Duration::from_millis(200));
                if self.recorded.len() == before && !self.eof {
                    return;
                }
                continue;
            }
            if Instant::now() >= deadline {
                if hup_sent {
                    eprintln!("pty-record: child ignored SIGHUP; sending SIGKILL");
                    let _ = kill(self.child, Signal::SIGKILL);
                    self.reap(true);
                    return;
                }
                eprintln!("pty-record: child still running after {END_WAIT:?}; sending SIGHUP");
                let _ = kill(self.child, Signal::SIGHUP);
                hup_sent = true;
                deadline = Instant::now() + Duration::from_secs(2);
                continue;
            }
            self.pump(Duration::from_millis(50));
        }
    }
}

fn main() {
    let mut cols = 80u16;
    let mut rows = 24u16;
    let mut out_path = None;
    let mut script_path = None;
    let mut args = std::env::args().skip(1);
    let mut command = Vec::new();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--cols" => {
                cols = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| usage())
            }
            "--rows" => {
                rows = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| usage())
            }
            "--out" => out_path = Some(args.next().unwrap_or_else(|| usage())),
            "--script" => script_path = Some(args.next().unwrap_or_else(|| usage())),
            "--" => {
                command = args.by_ref().collect();
                break;
            }
            _ => usage(),
        }
    }
    let (Some(out_path), Some(script_path)) = (out_path, script_path) else {
        usage()
    };
    if command.is_empty() {
        usage();
    }
    let script = std::fs::read_to_string(&script_path)
        .unwrap_or_else(|e| die(&format!("read {script_path}: {e}")));
    let out = File::create(&out_path).unwrap_or_else(|e| die(&format!("create {out_path}: {e}")));
    let argv: Vec<CString> = command
        .iter()
        .map(|a| CString::new(a.as_str()).unwrap_or_else(|_| die("NUL in command")))
        .collect();

    let start = Instant::now();
    let ws = winsize(cols, rows);
    let result = unsafe { forkpty(&ws, None) }.unwrap_or_else(|e| die(&format!("forkpty: {e}")));
    let (master, child) = match result {
        ForkptyResult::Child => {
            // The child is single-threaded here: set the environment and exec.
            unsafe {
                std::env::set_var("TERM", "xterm-256color");
                std::env::set_var("LANG", "C.UTF-8");
                std::env::remove_var("COLUMNS");
                std::env::remove_var("LINES");
            }
            let err = execvp(&argv[0], &argv).unwrap_err();
            eprintln!("pty-record: exec {}: {err}", command[0]);
            exit(127);
        }
        ForkptyResult::Parent { child, master } => (master, child),
    };

    let mut s = Session {
        master,
        child,
        out,
        recorded: Vec::new(),
        expect_from: 0,
        status: None,
        eof: false,
    };

    for (lineno, raw) in script.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let (cmd, rest) = line.split_once(' ').unwrap_or((line, ""));
        let bad = || -> ! { die(&format!("{script_path}:{}: bad line {line:?}", lineno + 1)) };
        match cmd {
            "wait" => s.pump_for(Duration::from_millis(
                rest.trim().parse().unwrap_or_else(|_| bad()),
            )),
            "send" => s.send(&unescape(rest)),
            "resize" => {
                let (c, r) = parse_size(rest.trim());
                let ws = winsize(c, r);
                if unsafe { libc::ioctl(s.master.as_raw_fd(), libc::TIOCSWINSZ, &ws) } != 0 {
                    die(&format!("TIOCSWINSZ: {}", Errno::last()));
                }
                if let Ok(pgrp) = nix::unistd::tcgetpgrp(&s.master) {
                    let _ = killpg(pgrp, Signal::SIGWINCH);
                }
            }
            "expect" => {
                let (text, ms) = rest.rsplit_once(' ').unwrap_or_else(|| bad());
                let ms: u64 = ms.trim().parse().unwrap_or_else(|_| bad());
                s.expect(&unescape(text), Duration::from_millis(ms));
            }
            "end" => break,
            _ => bad(),
        }
    }
    s.finish();
    let duration = start.elapsed();
    s.out.flush().ok();
    let status = match s.status {
        Some(WaitStatus::Exited(_, code)) => format!("exited {code}"),
        Some(WaitStatus::Signaled(_, sig, _)) => format!("killed by {sig:?}"),
        Some(other) => format!("{other:?}"),
        None => "unknown".to_string(),
    };
    let meta = format!(
        "command: {}\nsize: {cols}x{rows}\nbytes: {}\nduration_ms: {}\nexit: {status}\n",
        command
            .iter()
            .map(|a| if a.contains(' ') {
                format!("'{a}'")
            } else {
                a.clone()
            })
            .collect::<Vec<_>>()
            .join(" "),
        s.recorded.len(),
        duration.as_millis()
    );
    std::fs::write(format!("{out_path}.meta"), meta)
        .unwrap_or_else(|e| die(&format!("write meta: {e}")));
    eprintln!(
        "pty-record: {out_path}: {} bytes, {} ms, {status}",
        s.recorded.len(),
        duration.as_millis()
    );
}
