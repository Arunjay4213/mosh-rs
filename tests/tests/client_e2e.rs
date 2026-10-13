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
// End-to-end tests of `mosh-client` against the real mosh 1.4.0
// `mosh-server` from apt, started by our launcher over `ssh localhost`.
// The client runs in a tmux pane (the host terminal), and every assertion
// reads that pane, as a user would see it. Every test runs the same steps
// with `/usr/bin/mosh-client` first, so a failure of our client is not a
// test or environment problem.
//
// Needs `ssh -o BatchMode=yes localhost true` to succeed. UDP ports 61300-61399.

#![cfg(unix)]

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

fn release_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
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
                "mosh-client",
            ])
            .current_dir(&root)
            .status()
            .expect("run cargo build");
        assert!(status.success(), "cargo build of mosh failed");
        let target = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("target"));
        target
            .join("release")
            .canonicalize()
            .expect("target/release")
    })
    .clone()
}

/// The clients under test: upstream's first, so the test proves itself.
fn clients() -> Vec<(&'static str, String)> {
    vec![
        ("cxx", "/usr/bin/mosh-client".to_string()),
        (
            "rust",
            release_dir().join("mosh-client").display().to_string(),
        ),
    ]
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

/// A detached tmux server on its own socket running `bash --norc`, killed on drop.
struct Tmux {
    socket: PathBuf,
}

impl Tmux {
    fn start(name: &str) -> Tmux {
        let socket =
            std::env::temp_dir().join(format!("mosh-rs-client-{name}-{}.sock", std::process::id()));
        let status = Command::new("tmux")
            .args(["-f", "/dev/null", "-S"])
            .arg(&socket)
            .args(["new-session", "-d", "-x", "80", "-y", "24"])
            .args(["env", "TERM=xterm-256color", "PS1=$ ", "bash", "--norc"])
            .status()
            .expect("run tmux");
        assert!(status.success(), "tmux new-session failed");
        let t = Tmux { socket };
        t.wait_line("$", Duration::from_secs(10));
        t
    }

    fn tmux(&self, args: &[&str]) -> String {
        let out = Command::new("tmux")
            .arg("-S")
            .arg(&self.socket)
            .args(args)
            .output()
            .expect("run tmux");
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn capture(&self) -> String {
        self.tmux(&["capture-pane", "-p"])
    }

    /// Type `text` literally, then Enter.
    fn type_line(&self, text: &str) {
        self.tmux(&["send-keys", "-l", text]);
        self.tmux(&["send-keys", "Enter"]);
    }

    /// Poll the pane every 50 ms until `pred` holds.
    fn wait_until(&self, what: &str, timeout: Duration, pred: impl Fn(&str) -> bool) -> String {
        let deadline = Instant::now() + timeout;
        loop {
            let screen = self.capture();
            if pred(&screen) {
                return screen;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for {what}; screen:\n{screen}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// Wait for a line that is exactly `line` (tmux drops trailing spaces,
    /// so a prompt `R$ ` shows as `R$`).
    fn wait_line(&self, line: &str, timeout: Duration) -> String {
        self.wait_until(&format!("line {line:?}"), timeout, |s| {
            s.lines().any(|l| l == line)
        })
    }

    fn wait_for(&self, needle: &str, timeout: Duration) -> String {
        self.wait_until(&format!("{needle:?}"), timeout, |s| s.contains(needle))
    }
}

impl Drop for Tmux {
    fn drop(&mut self) {
        self.tmux(&["kill-server"]);
        let _ = std::fs::remove_file(&self.socket);
    }
}

/// The launcher command line that runs `client` against the C++ server on `port`.
fn mosh_command(client: &str, port: u16, remote: &str) -> String {
    format!(
        "{} --client={client} --server=/usr/bin/mosh-server -p {port} localhost -- {remote}; echo MOSH_EXIT=$?",
        release_dir().join("mosh").display()
    )
}

/// The pid of the mosh-server listening on UDP `port`.
fn server_pid(port: u16) -> Option<u32> {
    let out = Command::new("ss")
        .args(["-H", "-u", "-l", "-n", "-p"])
        .arg(format!("sport = :{port}"))
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let at = text.find("pid=")? + 4;
    text[at..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

/// Make sure no server from a failed run is left on `port`.
fn kill_server(port: u16) {
    if let Some(pid) = server_pid(port) {
        let _ = Command::new("kill").arg("-9").arg(pid.to_string()).status();
    }
}

#[test]
fn command_output_and_exit() {
    require_ssh_localhost();
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61300 + i as u16;
        let tmux = Tmux::start(&format!("cmd-{name}"));
        tmux.type_line(&mosh_command(
            &client,
            port,
            "sh -c 'printf \"CLIENT_OK\\n\"; seq 1 300; echo DONE; sleep 2'",
        ));
        let live = tmux.wait_line("DONE", Duration::from_secs(30));
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        println!(
            "[{name}] live screen:\n{}\n[{name}] final screen:\n{}",
            live.trim_end(),
            done.trim_end()
        );
        assert!(
            live.lines().any(|l| l == "300"),
            "[{name}] seq output missing"
        );
        assert!(
            done.contains("[mosh is exiting.]"),
            "[{name}] no exit message"
        );
        assert!(done.contains("MOSH_EXIT=0"), "[{name}] non-zero exit");
        kill_server(port);
    }
}

#[test]
fn interactive_shell_and_resize() {
    require_ssh_localhost();
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61310 + i as u16;
        let tmux = Tmux::start(&format!("shell-{name}"));
        tmux.type_line(&mosh_command(&client, port, "env PS1='R$ ' bash --norc"));
        tmux.wait_line("R$", Duration::from_secs(30));
        tmux.type_line("echo $((6*7))TYPED");
        tmux.wait_for("42TYPED", Duration::from_secs(10));

        tmux.tmux(&["resize-window", "-x", "100", "-y", "30"]);
        let size = tmux.tmux(&["display", "-p", "#{window_width}x#{window_height}"]);
        assert_eq!(size.trim(), "100x30", "[{name}] tmux did not resize");
        tmux.type_line("echo COLS=$(tput cols) LINES=$(tput lines)");
        let screen = tmux.wait_for("COLS=100 LINES=30", Duration::from_secs(10));
        println!("[{name}] after resize:\n{}", screen.trim_end());

        tmux.type_line("exit");
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        println!("[{name}] final screen:\n{}", done.trim_end());
        assert!(done.contains("MOSH_EXIT=0"), "[{name}] non-zero exit");
        kill_server(port);
    }
}

#[test]
fn escape_quit_restores_terminal() {
    require_ssh_localhost();
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61320 + i as u16;
        let tmux = Tmux::start(&format!("quit-{name}"));
        tmux.type_line("echo PRE_MOSH_MARKER");
        tmux.wait_for("PRE_MOSH_MARKER\n", Duration::from_secs(5));
        tmux.type_line(&mosh_command(&client, port, "env PS1='R$ ' bash --norc"));
        tmux.wait_line("R$", Duration::from_secs(30));
        tmux.type_line("echo INSIDE_MOSH");
        tmux.wait_for("INSIDE_MOSH\n", Duration::from_secs(10));

        tmux.tmux(&["send-keys", "C-^"]);
        let help = tmux.wait_for("Commands:", Duration::from_secs(5));
        println!(
            "[{name}] escape help:\n{}",
            help.lines().next().unwrap_or("")
        );
        tmux.tmux(&["send-keys", "."]);
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        println!("[{name}] after quit:\n{}", done.trim_end());
        assert!(
            done.contains("[mosh is exiting.]"),
            "[{name}] no exit message"
        );
        assert!(done.contains("MOSH_EXIT=0"), "[{name}] non-zero exit");
        assert!(
            done.contains("PRE_MOSH_MARKER"),
            "[{name}] the pre-mosh screen did not come back"
        );
        assert!(
            !done.contains("INSIDE_MOSH"),
            "[{name}] the alternate screen was not left"
        );

        // Each of the four flags `stty -a` may show, on its own line as
        // `STTY:<flag>`; the quotes keep the typed command from matching.
        tmux.type_line(
            "clear; stty -a | tr ' ;' '\\n\\n' | grep -x -e icanon -e echo -e -icanon -e -echo | sed 's/^/ST''TY:/'; echo ST''TY_END",
        );
        let stty = tmux.wait_line("STTY_END", Duration::from_secs(5));
        let flags: Vec<&str> = stty
            .lines()
            .filter_map(|l| l.strip_prefix("STTY:"))
            .collect();
        println!("[{name}] stty flags: {flags:?}");
        assert_eq!(flags, ["icanon", "echo"], "[{name}] terminal still raw");
        kill_server(port);
    }
}

#[test]
fn killed_server_shows_last_contact() {
    require_ssh_localhost();
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61330 + i as u16;
        let tmux = Tmux::start(&format!("kill-{name}"));
        tmux.type_line(&mosh_command(&client, port, "env PS1='R$ ' bash --norc"));
        tmux.wait_line("R$", Duration::from_secs(30));
        let pid = server_pid(port).expect("mosh-server pid");
        let killed = Instant::now();
        let status = Command::new("kill").arg("-9").arg(pid.to_string()).status();
        assert!(status.is_ok_and(|s| s.success()));
        let bar = tmux.wait_for("mosh: Last contact", Duration::from_secs(10));
        println!(
            "[{name}] notification after {:.1} s: {}",
            killed.elapsed().as_secs_f64(),
            bar.lines().next().unwrap_or("")
        );
        assert!(bar
            .lines()
            .next()
            .unwrap_or("")
            .starts_with("mosh: Last contact"));

        tmux.tmux(&["send-keys", "C-^"]);
        tmux.tmux(&["send-keys", "."]);
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        println!("[{name}] after quit:\n{}", done.trim_end());
        assert!(
            done.contains("[mosh is exiting.]"),
            "[{name}] no exit message"
        );
    }
}

/// PLAN section 5: an idle session costs no CPU. Both clients sit at a
/// shell prompt side by side while `pidstat` samples them for 60 s.
#[test]
fn idle_cpu() {
    if Command::new("pidstat").arg("-V").output().is_err() {
        println!("SKIPPED: pidstat not installed (package sysstat)");
        return;
    }
    require_ssh_localhost();
    let mut sessions = Vec::new();
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61340 + i as u16;
        let tmux = Tmux::start(&format!("idle-{name}"));
        tmux.type_line(&mosh_command(&client, port, "env PS1='R$ ' bash --norc"));
        tmux.wait_line("R$", Duration::from_secs(30));
        let out = Command::new("pgrep")
            .args(["-f", &format!("mosh-client .* {port}$")])
            .output()
            .expect("run pgrep");
        let pid: u32 = String::from_utf8_lossy(&out.stdout)
            .trim()
            .parse()
            .expect("one client pid");
        sessions.push((name, port, pid, tmux));
    }
    let pids: Vec<String> = sessions.iter().map(|s| s.2.to_string()).collect();
    let out = Command::new("pidstat")
        .args(["-p", &pids.join(","), "1", "60"])
        .env("LC_ALL", "C")
        .output()
        .expect("run pidstat");
    let report = String::from_utf8_lossy(&out.stdout);
    for (name, port, pid, tmux) in &sessions {
        let line = report
            .lines()
            .find(|l| {
                l.starts_with("Average:")
                    && l.split_whitespace().nth(2) == Some(pid.to_string().as_str())
            })
            .unwrap_or_else(|| panic!("no pidstat average for {pid}:\n{report}"));
        println!("[{name}] {line}");
        let cpu: f64 = line.split_whitespace().nth(7).unwrap().parse().unwrap();
        assert!(cpu <= 0.05, "[{name}] idle client used {cpu}% CPU");
        tmux.type_line("exit");
        tmux.wait_for("MOSH_EXIT=", Duration::from_secs(30));
        kill_server(*port);
    }
}

/// `Ctrl-^ Ctrl-Z` suspends the client (the shell reports it stopped and
/// the terminal is usable), and `fg` brings the session back.
#[test]
fn escape_suspend_and_resume() {
    require_ssh_localhost();
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61345 + i as u16;
        let tmux = Tmux::start(&format!("suspend-{name}"));
        // No `; echo` after it: a list would go on running when mosh stops.
        tmux.type_line(&format!(
            "{} --client={client} --server=/usr/bin/mosh-server -p {port} localhost -- env PS1='R$ ' bash --norc",
            release_dir().join("mosh").display()
        ));
        tmux.wait_line("R$", Duration::from_secs(30));
        tmux.type_line("echo BEFORE_SUSPEND");
        tmux.wait_line("BEFORE_SUSPEND", Duration::from_secs(10));

        tmux.tmux(&["send-keys", "C-^"]);
        tmux.tmux(&["send-keys", "C-z"]);
        let stopped = tmux.wait_until("the shell's job report", Duration::from_secs(10), |s| {
            s.contains("[mosh is suspended.]") && s.contains("Stopped")
        });
        println!("[{name}] suspended:\n{}", stopped.trim_end());
        tmux.type_line("echo SHELL_WORKS");
        tmux.wait_line("SHELL_WORKS", Duration::from_secs(5));

        tmux.type_line("fg");
        let back = tmux.wait_line("BEFORE_SUSPEND", Duration::from_secs(10));
        assert!(
            !back.contains("SHELL_WORKS"),
            "[{name}] not repainted:\n{back}"
        );
        tmux.type_line("echo AFTER_RESUME");
        let screen = tmux.wait_line("AFTER_RESUME", Duration::from_secs(10));
        println!("[{name}] resumed:\n{}", screen.trim_end());

        tmux.type_line("exit");
        tmux.wait_for("[mosh is exiting.]", Duration::from_secs(30));
        kill_server(port);
    }
}

/// Nothing answers on the port: the client says so at once, gives up after
/// 15 s, and explains on stderr, exit 1.
#[test]
fn no_server_times_out() {
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let port = 61347 + i as u16;
        kill_server(port);
        let tmux = Tmux::start(&format!("noserver-{name}"));
        let started = Instant::now();
        tmux.type_line(&format!(
            "MOSH_KEY=AAAAAAAAAAAAAAAAAAAAAA {client} 127.0.0.1 {port}; echo MOSH_EXIT=$?"
        ));
        let bar = format!("mosh: Nothing received from server on UDP port {port}.");
        tmux.wait_until("the connecting bar", Duration::from_secs(5), |s| {
            s.lines().next().is_some_and(|l| l.starts_with(&bar))
        });
        println!(
            "[{name}] connecting bar after {:.1} s",
            started.elapsed().as_secs_f64()
        );
        tmux.wait_until("the timeout bar", Duration::from_secs(25), |s| {
            s.lines()
                .next()
                .is_some_and(|l| l.starts_with("mosh: Timed out waiting for server..."))
        });
        println!(
            "[{name}] timeout bar after {:.1} s",
            started.elapsed().as_secs_f64()
        );
        let done = tmux.wait_for("MOSH_EXIT=", Duration::from_secs(60));
        println!(
            "[{name}] exit after {:.1} s:\n{}",
            started.elapsed().as_secs_f64(),
            done.trim_end()
        );
        assert!(done.contains(&format!(
            "mosh did not make a successful connection to 127.0.0.1:{port}."
        )));
        assert!(done.contains("[mosh is exiting.]"));
        assert!(done.contains("MOSH_EXIT=1"));
    }
}
