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
// Ported from: (new; no upstream test). The behavior checked is upstream
// src/frontend/mosh-server.cc `warn_unattached`.
//
// The detached-session warning. The apt mosh 1.4.0 `mosh-server` (built
// with utempter) leaves a detached session with a `mosh [PID]` utmp record.
// Then each server under test starts a login-shell session, and the
// screen a client sees must carry upstream's warning naming that session:
// first the real mosh-server from PATH itself, then ours from target/release
// (glibc), then the static musl build when it has been built
// (scripts/build-release.sh). Our warning text must equal the C++ one.
//
// Each server is started on a pseudoterminal (`script`), as under
// `ssh -t`, which is how the `mosh` launcher runs it. That matters for the
// C++ server: it prints the motd and the warning with stdio in the session
// child and then execs the shell without flushing. When `mosh-server new`
// started with stdout not a terminal (a pipe or file), glibc made stdout
// fully buffered, and that buffer is lost at the exec: no motd, no warning.
//
// Every mosh session of this user that is waiting for its client counts
// as detached, so the test holds /tmp/mosh-rs-interop.lock (the lock of
// tests/interop.sh and tools/bench) to keep their sessions away.
//
// Linux only (utmp layout, utempter). UDP ports 61400-61409.

#![cfg(target_os = "linux")]

use std::fs::OpenOptions;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn release_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let status = Command::new(env!("CARGO"))
            .args(["build", "--release", "-p", "mosh", "--bin", "mosh-server"])
            .current_dir(root())
            .status()
            .expect("run cargo build");
        assert!(status.success(), "cargo build failed");
        let target = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root().join("target"));
        target
            .join("release")
            .canonicalize()
            .expect("target/release")
    })
    .clone()
}

/// The pid of a detached server from `[mosh-server detached, pid = N]`.
fn detached_pid(output: &str) -> Option<i32> {
    output
        .lines()
        .find_map(|l| l.trim().strip_prefix("[mosh-server detached, pid = "))
        .and_then(|r| r.trim_end_matches(']').parse().ok())
}

fn connect_key(output: &str) -> Option<String> {
    output
        .lines()
        .find_map(|l| l.trim().strip_prefix("MOSH CONNECT "))
        .and_then(|r| r.split_whitespace().nth(1))
        .map(str::to_string)
}

/// SIGTERM, then SIGKILL after `grace`. The C++ server removes its utmp
/// record on SIGTERM; SIGKILL would leave a stale `mosh [PID]` record.
fn stop(pid: i32, grace: Duration) {
    // SAFETY: kill(2) on a pid this test started.
    unsafe { libc::kill(pid, libc::SIGTERM) };
    let start = Instant::now();
    while Path::new(&format!("/proc/{pid}")).exists() && start.elapsed() < grace {
        sleep(Duration::from_millis(100));
    }
    // SAFETY: as above.
    unsafe { libc::kill(pid, libc::SIGKILL) };
}

/// A detached C++ session (no client ever attaches) that stops on drop.
struct Detached {
    pid: i32,
}

impl Drop for Detached {
    fn drop(&mut self) {
        stop(self.pid, Duration::from_secs(5));
    }
}

/// Starts `server new -i 127.0.0.1 -p PORT` (login shell, so motd and the
/// warning) on a pty, attaches the real `mosh-client` in a 120x60 tmux pane,
/// and returns the warning block from the screen: the lines from
/// "Mosh: You have" up to the first empty line.
fn warning_seen(name: &str, server: &str, port: u16) -> Vec<String> {
    let out = Command::new("script")
        .args([
            "-qec",
            &format!("{server} new -i 127.0.0.1 -p {port}"),
            "/dev/null",
        ])
        .env("LANG", "C.UTF-8")
        .stdin(Stdio::null())
        .output()
        .expect("run script");
    let text = String::from_utf8_lossy(&out.stdout).replace('\r', "");
    let key = connect_key(&text).unwrap_or_else(|| panic!("[{name}] no MOSH CONNECT: {text}"));
    let pid = detached_pid(&text).unwrap_or_else(|| panic!("[{name}] no detached pid: {text}"));

    let socket = std::env::temp_dir().join(format!(
        "mosh-rs-detached-{name}-{}.sock",
        std::process::id()
    ));
    let status = Command::new("tmux")
        .args(["-f", "/dev/null", "-S"])
        .arg(&socket)
        .args(["new-session", "-d", "-x", "120", "-y", "60"])
        .args([
            "env",
            &format!("MOSH_KEY={key}"),
            "TERM=xterm-256color",
            "LANG=C.UTF-8",
            &mosh_tests::upstream::get().client,
            "127.0.0.1",
            &port.to_string(),
        ])
        .status()
        .expect("run tmux");
    assert!(status.success(), "tmux new-session failed");
    let capture = || {
        let o = Command::new("tmux")
            .arg("-S")
            .arg(&socket)
            .args(["capture-pane", "-p"])
            .output()
            .expect("run tmux");
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    // The login shell's prompt comes after the warning; wait for it.
    let start = Instant::now();
    let mut screen = capture();
    while !screen.contains("$ ") && start.elapsed() < Duration::from_secs(15) {
        sleep(Duration::from_millis(100));
        screen = capture();
    }
    sleep(Duration::from_millis(500));
    screen = capture();

    // Client first (SIGKILL: no shutdown packets), then the server.
    let pane = Command::new("tmux")
        .arg("-S")
        .arg(&socket)
        .args(["list-panes", "-F", "#{pane_pid}"])
        .output()
        .expect("run tmux");
    let pane = String::from_utf8_lossy(&pane.stdout).trim().to_string();
    let _ = Command::new("pkill").args(["-KILL", "-P", &pane]).status();
    let _ = Command::new("tmux")
        .arg("-S")
        .arg(&socket)
        .arg("kill-server")
        .status();
    stop(pid, Duration::from_secs(10));

    eprintln!("[{name}] screen:\n{screen}");
    screen
        .lines()
        .skip_while(|l| !l.starts_with("Mosh: You have"))
        .take_while(|l| !l.trim().is_empty())
        .map(|l| l.trim_end().to_string())
        .collect()
}

#[test]
fn detached_session_is_reported() {
    let Some(up) = mosh_tests::upstream::require() else {
        return;
    };
    let rust = release_dir().join("mosh-server").display().to_string();
    let lock = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/mosh-rs-interop.lock")
        .expect("open /tmp/mosh-rs-interop.lock");
    // SAFETY: flock on a descriptor we own; released when `lock` drops.
    assert_eq!(unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX) }, 0);
    let musl = root().join(format!(
        "target/{}-unknown-linux-musl/release/mosh-server",
        std::env::consts::ARCH
    ));

    // The detached session. utempter writes its record shortly after start.
    let out = Command::new(&up.server)
        .args([
            "new",
            "-i",
            "127.0.0.1",
            "-p",
            "61500",
            "--",
            "sleep",
            "600",
        ])
        .env("LANG", "C.UTF-8")
        .stdin(Stdio::null())
        .output()
        .expect("run the real mosh-server");
    let detached = Detached {
        pid: detached_pid(&String::from_utf8_lossy(&out.stderr)).expect("detached pid"),
    };
    let entry = format!("mosh [{}]", detached.pid);
    let start = Instant::now();
    loop {
        // Our own reader of the utmp file sees the record.
        let found = mosh_platform::server::getutxent_all()
            .into_iter()
            .any(|e| e.user_process && e.host == entry);
        if found {
            break;
        }
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "no utmp record {entry:?} from the C++ server (is libutempter0 installed?)"
        );
        sleep(Duration::from_millis(100));
    }

    let mut servers = vec![("cxx", up.server.clone()), ("rust", rust)];
    if musl.exists() {
        servers.push(("rust-musl", musl.display().to_string()));
    } else {
        eprintln!(
            "{} not built (scripts/build-release.sh); the static build is not checked",
            musl.display()
        );
    }
    let singular = format!("Mosh: You have a detached Mosh session on this server ({entry}).");
    let mut blocks = Vec::new();
    for (i, (name, server)) in servers.iter().enumerate() {
        let block = warning_seen(name, server, 61401 + i as u16);
        // Other detached sessions of this user may exist on the machine;
        // then upstream prints the plural form with one line per session.
        let names_ours = block.first() == Some(&singular)
            || (block
                .first()
                .is_some_and(|l| l.contains("detached Mosh sessions"))
                && block.iter().any(|l| l.trim() == format!("- {entry}")));
        assert!(
            names_ours,
            "[{name}] no warning about {entry}; warning lines: {block:?}"
        );
        blocks.push((name, block));
    }
    for (name, block) in &blocks[1..] {
        assert_eq!(
            block, &blocks[0].1,
            "[{name}] warning differs from the real mosh-server's"
        );
    }
    drop(detached);
    drop(lock);
}
