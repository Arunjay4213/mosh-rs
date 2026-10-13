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
// Ported from: (new; no upstream equivalent). The timeout cases follow
// upstream/src/tests/server-network-timeout.test and server-signal-timeout.test.
//
// End-to-end tests of mosh-server with the real C++ mosh-client (apt
// mosh 1.4.0) inside tmux. Every case runs first against
// /usr/bin/mosh-server, so the test proves itself on upstream, then against
// ours. A user would notice each of these: command output and exit, typing
// into a shell, a window resize reaching the program, and the server going
// away after the client vanished.
//
// Needs `ssh -o BatchMode=yes localhost true` to succeed (the launcher case).
// UDP ports 61200-61299.

#![cfg(unix)]

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

const CXX_CLIENT: &str = "/usr/bin/mosh-client";
const CXX_SERVER: &str = "/usr/bin/mosh-server";

fn release_binary(name: &str) -> PathBuf {
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    let dir = BUILT.get_or_init(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let status = Command::new(env!("CARGO"))
            .args([
                "build",
                "--release",
                "-p",
                "mosh",
                "--bin",
                "mosh",
                "--bin",
                "mosh-server",
            ])
            .current_dir(&root)
            .status()
            .expect("run cargo build");
        assert!(status.success(), "cargo build of mosh, mosh-server failed");
        let target = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("target"));
        std::fs::canonicalize(target.join("release")).expect("target/release")
    });
    dir.join(name)
}

/// The two servers every case runs against: upstream first, then ours.
/// The number is added to the case's base port so the two never share one.
fn servers() -> Vec<(&'static str, String, u16)> {
    vec![
        ("cxx", CXX_SERVER.to_string(), 0),
        (
            "rust",
            release_binary("mosh-server").display().to_string(),
            50,
        ),
    ]
}

/// A detached tmux server on its own socket, killed on drop.
struct Tmux {
    socket: PathBuf,
}

impl Tmux {
    fn start(name: &str, shell_command: &str) -> Tmux {
        let socket =
            std::env::temp_dir().join(format!("mosh-rs-server-{name}-{}.sock", std::process::id()));
        let status = Command::new("tmux")
            .args(["-f", "/dev/null", "-S"])
            .arg(&socket)
            .args(["new-session", "-d", "-x", "80", "-y", "24", shell_command])
            .status()
            .expect("run tmux");
        assert!(status.success(), "tmux new-session failed");
        Tmux { socket }
    }

    fn tmux(&self, args: &[&str]) -> String {
        let out = Command::new("tmux")
            .arg("-S")
            .arg(&self.socket)
            .args(args)
            .output()
            .expect("run tmux");
        assert!(out.status.success(), "tmux {args:?} failed");
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn capture(&self) -> String {
        self.tmux(&["capture-pane", "-p"])
    }

    /// Poll the pane every 50 ms until `test` accepts it.
    fn wait_until(&self, what: &str, timeout: Duration, test: impl Fn(&str) -> bool) -> String {
        let deadline = Instant::now() + timeout;
        loop {
            let screen = self.capture();
            if test(&screen) {
                return screen;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for {what}; screen:\n{screen}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn wait_for(&self, needle: &str, timeout: Duration) -> String {
        self.wait_until(needle, timeout, |s| s.contains(needle))
    }

    /// Wait for a line that is exactly `line` (trailing blanks ignored), so
    /// typed or predicted command lines do not count as output.
    fn wait_for_line(&self, line: &str, timeout: Duration) -> String {
        self.wait_until(&format!("line {line:?}"), timeout, |s| {
            s.lines().any(|l| l.trim_end() == line)
        })
    }

    fn send_keys(&self, keys: &[&str]) {
        let mut args = vec!["send-keys", "-t", "0"];
        args.extend_from_slice(keys);
        self.tmux(&args);
    }
}

impl Drop for Tmux {
    fn drop(&mut self) {
        let _ = Command::new("tmux")
            .arg("-S")
            .arg(&self.socket)
            .arg("kill-server")
            .status();
        let _ = std::fs::remove_file(&self.socket);
    }
}

fn require_ssh_localhost() {
    let ok = Command::new("ssh")
        .args(["-o", "BatchMode=yes", "localhost", "true"])
        .stdin(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    assert!(
        ok,
        "`ssh -o BatchMode=yes localhost true` failed; authorize your key for localhost first"
    );
}

/// The pane runs the launcher with the C++ client and the given server, then
/// prints the exit status and stays open so the final screen can be read.
fn launcher_session(server: &str, port: u16, command: &str) -> String {
    format!(
        "export TERM=xterm-256color; {} --client={CXX_CLIENT} --server={server} -p {port} localhost -- {command}; echo MOSH_EXIT=$?; sleep 60",
        release_binary("mosh").display()
    )
}

#[test]
fn command_output_and_exit() {
    require_ssh_localhost();
    for (name, server, offset) in servers() {
        let port = 61200 + offset;
        // SERVER_OK gets a second on screen of its own: mosh sends screen
        // states, not every byte, so a line that scrolls away within one
        // frame interval is never shown by any mosh.
        let tmux = Tmux::start(
            &format!("cmd-{name}"),
            &launcher_session(
                &server,
                port,
                r#"sh -c 'printf "SERVER_OK\n"; sleep 1; seq 1 300; echo DONE; sleep 2'"#,
            ),
        );
        tmux.wait_for_line("SERVER_OK", Duration::from_secs(30));
        let live = tmux.wait_for_line("DONE", Duration::from_secs(30));
        println!("[{name}] live screen:\n{}", live.trim_end());
        assert!(
            live.lines().any(|l| l.trim_end() == "300"),
            "[{name}] no 300 line:\n{live}"
        );
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        println!("[{name}] final screen:\n{}", done.trim_end());
        assert!(
            done.contains("[mosh is exiting.]"),
            "[{name}] no exit message:\n{done}"
        );
        assert!(
            done.contains("MOSH_EXIT=0"),
            "[{name}] mosh exited non-zero:\n{done}"
        );
    }
}

#[test]
fn typing_and_resize() {
    require_ssh_localhost();
    for (name, server, offset) in servers() {
        let port = 61201 + offset;
        let tmux = Tmux::start(
            &format!("type-{name}"),
            &launcher_session(&server, port, "bash --norc"),
        );
        tmux.wait_for("$", Duration::from_secs(30));
        tmux.send_keys(&["echo typed-through-mosh", "Enter"]);
        let typed = tmux.wait_for_line("typed-through-mosh", Duration::from_secs(10));
        println!("[{name}] after typing:\n{}", typed.trim_end());

        tmux.tmux(&["resize-window", "-x", "100", "-y", "30"]);
        // The client notices SIGWINCH and sends the size; give it a moment
        // before asking the program.
        std::thread::sleep(Duration::from_millis(500));
        tmux.send_keys(&["tput cols; tput lines", "Enter"]);
        let resized = tmux.wait_for_line("100", Duration::from_secs(10));
        println!("[{name}] after resize:\n{}", resized.trim_end());
        assert!(
            resized.lines().any(|l| l.trim_end() == "30"),
            "[{name}] tput lines is not 30:\n{resized}"
        );

        tmux.send_keys(&["exit", "Enter"]);
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        assert!(
            done.contains("MOSH_EXIT=0"),
            "[{name}] mosh exited non-zero:\n{done}"
        );
    }
}

/// A server started directly (no ssh), as upstream's e2e tests do, with
/// extra environment. Returns (port, key, pid).
fn start_server(server: &str, port: u16, env: &[(&str, &str)]) -> (String, String, i32) {
    let out = Command::new(server)
        .args(["new", "-p", &port.to_string(), "--"])
        .args(["sh", "-c", "echo CONNECTED; exec sleep 1000"])
        .envs(env.iter().copied())
        .env("LANG", "C.UTF-8")
        .stdin(Stdio::null())
        .output()
        .expect("run mosh-server");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "server failed: {stdout}{stderr}");
    let connect = stdout
        .lines()
        .find_map(|l| l.strip_prefix("MOSH CONNECT "))
        .unwrap_or_else(|| panic!("no MOSH CONNECT in {stdout:?}"));
    let mut words = connect.split(' ');
    let (port, key) = (words.next().unwrap(), words.next().unwrap());
    let pid = stderr
        .lines()
        .find_map(|l| l.strip_prefix("[mosh-server detached, pid = "))
        .and_then(|l| l.strip_suffix(']'))
        .unwrap_or_else(|| panic!("no detached message in {stderr:?}"))
        .parse()
        .unwrap();
    (port.to_string(), key.to_string(), pid)
}

fn alive(pid: i32) -> bool {
    // SAFETY: signal 0 only checks that the process exists.
    unsafe { libc::kill(pid, 0) == 0 }
}

fn pgrep(pattern: &str) -> i32 {
    let out = Command::new("pgrep")
        .args(["-f", pattern])
        .output()
        .expect("run pgrep");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()
        .unwrap_or_else(|| panic!("no process matches {pattern:?}"))
        .parse()
        .unwrap()
}

/// Start a server with `env`, connect the C++ client in tmux, then kill the
/// client with SIGKILL. Returns the server pid, the tmux session, and when
/// the client died.
fn connect_then_kill_client(
    name: &str,
    server: &str,
    port: u16,
    env: &[(&str, &str)],
) -> (i32, Tmux, Instant) {
    let (port, key, server_pid) = start_server(server, port, env);
    let tmux = Tmux::start(
        name,
        &format!(
            "export TERM=xterm-256color; MOSH_KEY={key} {CXX_CLIENT} 127.0.0.1 {port}; echo MOSH_EXIT=$?; sleep 60"
        ),
    );
    tmux.wait_for_line("CONNECTED", Duration::from_secs(20));
    // Anchored: the tmux server and the pane shell carry the same text.
    let client_pid = pgrep(&format!("^{CXX_CLIENT} 127.0.0.1 {port}$"));
    // SAFETY: plain kill of the client process found above.
    assert_eq!(0, unsafe { libc::kill(client_pid, libc::SIGKILL) });
    (server_pid, tmux, Instant::now())
}

fn wait_for_exit(pid: i32, timeout: Duration) -> Option<Duration> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if !alive(pid) {
            return Some(start.elapsed());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    None
}

#[test]
fn network_timeout() {
    for (name, server, offset) in servers() {
        let port = 61202 + offset;
        let (pid, _tmux, killed) = connect_then_kill_client(
            &format!("ntmo-{name}"),
            &server,
            port,
            &[("MOSH_SERVER_NETWORK_TMOUT", "5")],
        );
        // Still alive right after the kill: the client, not the tmux server,
        // was killed, and the server did not just follow it.
        std::thread::sleep(Duration::from_secs(1));
        assert!(alive(pid), "[{name}] server exited within 1 s of the kill");
        let gone = wait_for_exit(pid, Duration::from_secs(20));
        let after_kill = killed.elapsed();
        println!(
            "[{name}] MOSH_SERVER_NETWORK_TMOUT=5: server exited {:?} after the client was killed",
            gone.map(|_| after_kill)
        );
        assert!(gone.is_some(), "[{name}] server {pid} still alive");
        assert!(
            after_kill <= Duration::from_secs(10),
            "[{name}] server exited {after_kill:?} after the client was killed"
        );
    }
}

#[test]
fn signal_timeout() {
    for (name, server, offset) in servers() {
        let port = 61203 + offset;
        let (pid, _tmux, killed) = connect_then_kill_client(
            &format!("stmo-{name}"),
            &server,
            port,
            &[("MOSH_SERVER_SIGNAL_TMOUT", "5")],
        );
        // Too early: the client was heard from less than 5 s ago, so
        // upstream consumes the signal and does nothing.
        std::thread::sleep(Duration::from_secs(1));
        // SAFETY: plain kill of the server found above.
        assert_eq!(0, unsafe { libc::kill(pid, libc::SIGUSR1) });
        std::thread::sleep(Duration::from_secs(2));
        assert!(alive(pid), "[{name}] early SIGUSR1 killed the server");

        // After the timeout the signal shuts the server down.
        std::thread::sleep(
            Duration::from_secs(7)
                .checked_sub(killed.elapsed())
                .unwrap_or_default(),
        );
        // SAFETY: as above.
        assert_eq!(0, unsafe { libc::kill(pid, libc::SIGUSR1) });
        let gone = wait_for_exit(pid, Duration::from_secs(10));
        println!(
            "[{name}] MOSH_SERVER_SIGNAL_TMOUT=5: early SIGUSR1 ignored; late SIGUSR1 at {:?} after the kill, server exited {gone:?} after it",
            Duration::from_secs(7)
        );
        assert!(
            gone.is_some_and(|d| d <= Duration::from_secs(5)),
            "[{name}] server {pid} still alive after the late SIGUSR1"
        );
    }
}
