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
// End-to-end tests of the `mosh` launcher against the real mosh 1.4.0
// `mosh-client` and `mosh-server` from apt, over `ssh localhost`. A pass
// proves: the proxy's name resolution, the `mosh-server new` command
// line, `MOSH CONNECT` parsing, and the exec of the client with MOSH_KEY.
// Since the launcher is its own ssh ProxyCommand (`--fake-proxy`, as
// mosh.pl), a pass also proves that path.
//
// Every process gets TERM=xterm-256color explicitly: `mosh-client -c` (the
// color count) fails without TERM, and tmux would otherwise give the pane
// `tmux-256color`, so the result would depend on the runner.
//
// Needs `ssh -o BatchMode=yes localhost true` to succeed.

#![cfg(unix)]

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

fn mosh_binary() -> PathBuf {
    static BIN: OnceLock<PathBuf> = OnceLock::new();
    BIN.get_or_init(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let status = Command::new(env!("CARGO"))
            .args(["build", "--release", "-p", "mosh", "--bin", "mosh"])
            .current_dir(&root)
            .status()
            .expect("run cargo build");
        assert!(status.success(), "cargo build of mosh failed");
        let target = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("target"));
        target.join("release").join("mosh")
    })
    .clone()
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

/// A detached tmux server on its own socket, killed on drop.
struct Tmux {
    socket: PathBuf,
}

impl Tmux {
    fn start(name: &str, shell_command: &str) -> Tmux {
        let socket = std::env::temp_dir().join(format!(
            "mosh-rs-launcher-{name}-{}.sock",
            std::process::id()
        ));
        let status = Command::new("tmux")
            .args(["-f", "/dev/null", "-S"])
            .arg(&socket)
            .args(["new-session", "-d", "-x", "80", "-y", "24", shell_command])
            .status()
            .expect("run tmux");
        assert!(status.success(), "tmux new-session failed");
        Tmux { socket }
    }

    fn capture(&self) -> String {
        let out = Command::new("tmux")
            .arg("-S")
            .arg(&self.socket)
            .args(["capture-pane", "-p"])
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// Poll the pane every 50 ms until it contains `needle`.
    fn wait_for(&self, needle: &str, timeout: Duration) -> String {
        let deadline = Instant::now() + timeout;
        loop {
            let screen = self.capture();
            if screen.contains(needle) {
                return screen;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for {needle:?}; screen:\n{screen}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
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

/// The pane runs mosh, then prints its exit status and stays open so the
/// final screen can be read.
fn session(mosh_args: &str) -> String {
    format!(
        "export TERM=xterm-256color; {} {mosh_args}; echo MOSH_EXIT=$?; sleep 60",
        mosh_binary().display()
    )
}

#[test]
fn launches_session_on_localhost_with_command() {
    require_ssh_localhost();
    let tmux = Tmux::start(
        "localhost",
        &session(
            "--client=/usr/bin/mosh-client --server=/usr/bin/mosh-server -p 60500 localhost -- sh -c 'echo LAUNCH_OK; sleep 1'",
        ),
    );
    let live = tmux.wait_for("LAUNCH_OK", Duration::from_secs(30));
    println!("live session screen:\n{}", live.trim_end());
    let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
    println!("final screen:\n{}", done.trim_end());
    assert!(
        done.contains("[mosh is exiting.]"),
        "no exit message:\n{done}"
    );
    assert!(
        done.contains("MOSH_EXIT=0"),
        "mosh exited non-zero:\n{done}"
    );
}

#[test]
fn launches_session_with_user_at_ip_literal() {
    require_ssh_localhost();
    let user = std::env::var("USER").expect("USER");
    let tmux = Tmux::start(
        "userip",
        &session(&format!(
            "--client=/usr/bin/mosh-client --server=/usr/bin/mosh-server {user}@127.0.0.1 -- true"
        )),
    );
    let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
    println!("final screen:\n{}", done.trim_end());
    assert!(
        done.contains("[mosh is exiting.]"),
        "no exit message:\n{done}"
    );
    assert!(
        done.contains("MOSH_EXIT=0"),
        "mosh exited non-zero:\n{done}"
    );
}

#[test]
fn missing_server_is_reported() {
    require_ssh_localhost();
    let out = Command::new(mosh_binary())
        .args([
            "--client=/usr/bin/mosh-client",
            "--server=/bin/false",
            "localhost",
        ])
        .env("TERM", "xterm-256color")
        .stdin(Stdio::null())
        .output()
        .expect("run mosh");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    println!("status: {}\nstdout: {stdout}stderr: {stderr}", out.status);
    assert!(
        stderr.contains(
            "Did not find mosh server startup message. (Have you installed mosh on your server?)"
        ),
        "missing mosh.pl error message"
    );
    // Real mosh 1.4.0 exits with 10 here (ECHILD left in $! by waitpid).
    assert_eq!(out.status.code(), Some(10));
}

/// A name with both an IPv6 and an IPv4 address. ssh may connect over
/// either; mosh-server binds the address from SSH_CONNECTION, so the client
/// must be started with the same one or it never hears from the server.
/// Needs `::1 mosh-dualstack` and `127.0.0.1 mosh-dualstack` in /etc/hosts
/// (set up on devbox; on CI `localhost` resolves to both families and the
/// first test covers this).
#[test]
fn launches_session_on_dual_stack_name() {
    let addrs = Command::new("getent")
        .args(["ahosts", "mosh-dualstack"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    if !(addrs.contains("::1") && addrs.contains("127.0.0.1")) {
        println!("SKIPPED: /etc/hosts has no ::1 and 127.0.0.1 entries for mosh-dualstack");
        return;
    }
    require_ssh_localhost();
    for family in ["", "-4", "-6"] {
        let tmux = Tmux::start(
            &format!("dualstack{family}"),
            &session(&format!(
                "{family} --client=/usr/bin/mosh-client --server=/usr/bin/mosh-server mosh-dualstack -- sh -c 'echo LAUNCH_OK; sleep 1'"
            )),
        );
        tmux.wait_for("LAUNCH_OK", Duration::from_secs(30));
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        println!(
            "mosh {family} mosh-dualstack, final screen:\n{}",
            done.trim_end()
        );
        assert!(
            done.contains("[mosh is exiting.]"),
            "no exit message:\n{done}"
        );
        assert!(
            done.contains("MOSH_EXIT=0"),
            "mosh exited non-zero:\n{done}"
        );
    }
}
