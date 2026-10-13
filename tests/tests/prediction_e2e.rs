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
// Ported from: (new; no upstream equivalent; the unicode case follows
// upstream/src/tests/prediction-unicode.test)
//
// End-to-end tests of the prediction overlay (local echo). The real mosh
// 1.4.0 `mosh-server` runs a shell; the client talks to it through
// `udp-impair` with 150 ms delay each way (300 ms round trip), inside a tmux
// pane. Keys go in with `tmux send-keys` and the pane is read every 5 ms,
// so the times below are what a user would see. Every case runs with
// the real (C++) `mosh-client` from PATH first, so the numbers can be compared and a
// failure of ours is not a harness problem.
//
// UDP ports 61350-61397.

#![cfg(unix)]

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

const DELAY_MS: &str = "150";

fn release_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let status = Command::new(env!("CARGO"))
            .args(["build", "--release", "-p", "mosh", "--bin", "mosh-client"])
            .args(["-p", "mosh-tools", "--bin", "udp-impair"])
            .current_dir(&root)
            .status()
            .expect("run cargo build");
        assert!(status.success(), "cargo build failed");
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

/// The clients under test: upstream's first.
fn clients() -> Vec<(&'static str, String)> {
    vec![
        ("cxx", mosh_tests::upstream::get().client.clone()),
        (
            "rust",
            release_dir().join("mosh-client").display().to_string(),
        ),
    ]
}

/// A detached tmux server on its own socket running `bash --norc`, killed on drop.
struct Tmux {
    socket: PathBuf,
}

impl Tmux {
    fn start(name: &str) -> Tmux {
        let socket = std::env::temp_dir().join(format!(
            "mosh-rs-predict-{name}-{}.sock",
            std::process::id()
        ));
        let status = Command::new("tmux")
            .args(["-f", "/dev/null", "-S"])
            .arg(&socket)
            .args(["new-session", "-d", "-x", "80", "-y", "24"])
            .args([
                "env",
                "TERM=xterm-256color",
                "LANG=C.UTF-8",
                "PS1=$ ",
                "bash",
                "--norc",
            ])
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

    fn keys(&self, text: &str) {
        self.tmux(&["send-keys", "-l", text]);
    }

    fn type_line(&self, text: &str) {
        self.keys(text);
        self.tmux(&["send-keys", "Enter"]);
    }

    /// Poll the pane every `every` until `pred` holds; returns the screen
    /// and the time it took, or None after `timeout`.
    fn poll(
        &self,
        every: Duration,
        timeout: Duration,
        pred: impl Fn(&str) -> bool,
    ) -> Option<(String, Duration)> {
        let start = Instant::now();
        loop {
            let screen = self.capture();
            if pred(&screen) {
                return Some((screen, start.elapsed()));
            }
            if start.elapsed() > timeout {
                return None;
            }
            std::thread::sleep(every);
        }
    }

    fn wait_until(&self, what: &str, timeout: Duration, pred: impl Fn(&str) -> bool) -> String {
        match self.poll(Duration::from_millis(50), timeout, pred) {
            Some((screen, _)) => screen,
            None => panic!("timed out waiting for {what}; screen:\n{}", self.capture()),
        }
    }

    fn wait_line(&self, line: &str, timeout: Duration) -> String {
        self.wait_until(&format!("line {line:?}"), timeout, |s| {
            s.lines().any(|l| l == line)
        })
    }
}

impl Drop for Tmux {
    fn drop(&mut self) {
        self.tmux(&["kill-server"]);
        let _ = std::fs::remove_file(&self.socket);
    }
}

/// `mosh-server new` on `port` (it detaches), and the session key from `MOSH CONNECT`.
fn start_server(port: u16, command: &[&str]) -> String {
    let mut child = Command::new(&mosh_tests::upstream::get().server)
        .args(["new", "-c", "256", "-p", &port.to_string(), "--"])
        .args(command)
        .env("LANG", "C.UTF-8")
        .env("TERM", "xterm-256color")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("run mosh-server");
    let stdout = child.stdout.take().unwrap();
    let mut key = None;
    for line in BufReader::new(stdout).lines() {
        let line = line.unwrap();
        if let Some(rest) = line.strip_prefix("MOSH CONNECT ") {
            let mut parts = rest.split_whitespace();
            assert_eq!(parts.next(), Some(port.to_string().as_str()));
            key = parts.next().map(str::to_string);
            break;
        }
    }
    let _ = child.wait();
    key.expect("MOSH CONNECT line")
}

/// The pid of the process listening on UDP `port`.
fn kill_port(port: u16) {
    let out = Command::new("ss")
        .args(["-H", "-u", "-l", "-n", "-p"])
        .arg(format!("sport = :{port}"))
        .output()
        .expect("run ss");
    let text = String::from_utf8_lossy(&out.stdout);
    if let Some(at) = text.find("pid=") {
        let pid: String = text[at + 4..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let _ = Command::new("kill").args(["-9", &pid]).status();
    }
}

/// A mosh session through udp-impair: server on `base`, proxy on `base + 1`
/// (control `base + 2`), client in a tmux pane. Cleans up on drop.
struct Session {
    tmux: Tmux,
    proxy: Child,
    base: u16,
}

impl Session {
    /// `env` holds the client's variables, e.g. `MOSH_PREDICTION_DISPLAY=always`.
    fn start(name: &str, client: &str, base: u16, env: &str, command: &[&str]) -> Session {
        kill_port(base);
        let key = start_server(base, command);
        let proxy = Command::new(release_dir().join("udp-impair"))
            .args(["--listen", &format!("127.0.0.1:{}", base + 1)])
            .args(["--target", &format!("127.0.0.1:{base}")])
            .args(["--control", &format!("127.0.0.1:{}", base + 2)])
            .args(["--delay-ms", DELAY_MS])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("run udp-impair");
        let tmux = Tmux::start(name);
        tmux.type_line(&format!(
            "MOSH_KEY={key} {env} {client} 127.0.0.1 {}; echo MOSH_EXIT=$?",
            base + 1
        ));
        Session { tmux, proxy, base }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        kill_port(self.base);
        let _ = self.proxy.kill();
        let _ = self.proxy.wait();
    }
}

/// The shell in the session: `R$ ` prompt, no rc files.
const SHELL: &[&str] = &["env", "PS1=R$ ", "bash", "--norc"];

/// Key-to-screen times for three edits of a shell command line: typing at
/// the end, backspace, and inserting in the middle (after a left arrow).
/// A few keys are typed and echoed first, so the prediction epoch is
/// confirmed (upstream shows nothing of an unconfirmed epoch), and each
/// edit waits until the previous one is confirmed by the server.
/// With `MOSH_PREDICTION_OVERWRITE=yes` the insert is predicted as an
/// overwrite (`ax` instead of `axb`), and corrected to bash's `axb`.
fn editing_latencies(name: &str, client: &str, base: u16, env: &str) -> [Duration; 3] {
    let label = env.replace(['=', ' '], "-");
    let s = Session::start(&format!("{label}-{name}"), client, base, env, SHELL);
    let t = &s.tmux;
    t.wait_line("R$", Duration::from_secs(30));
    t.keys("echo ab");
    t.wait_line("R$ echo ab", Duration::from_secs(10));
    std::thread::sleep(Duration::from_millis(1000));

    let measure = |what: &str, expect: &str| {
        let (_, took) = t
            .poll(Duration::from_millis(5), Duration::from_secs(5), |sc| {
                sc.lines().any(|l| l == expect)
            })
            .unwrap_or_else(|| panic!("[{name}] {env}: {what} never showed {expect:?}"));
        // Let the server confirm the edit before the next one.
        std::thread::sleep(Duration::from_millis(1000));
        took
    };

    t.keys("c");
    let typed = measure("typing", "R$ echo abc");
    t.tmux(&["send-keys", "BSpace"]);
    let backspace = measure("backspace", "R$ echo ab");
    t.tmux(&["send-keys", "Left"]);
    std::thread::sleep(Duration::from_millis(1000));
    t.keys("x");
    let overwrite = env.contains("MOSH_PREDICTION_OVERWRITE=yes");
    let (_, inserted) = t
        .poll(Duration::from_millis(5), Duration::from_secs(5), |sc| {
            sc.lines().any(|l| {
                l == if overwrite {
                    "R$ echo ax"
                } else {
                    "R$ echo axb"
                }
            })
        })
        .unwrap_or_else(|| panic!("[{name}] {env}: insert never showed"));
    // The server's truth is bash's insert in every mode.
    t.wait_line("R$ echo axb", Duration::from_secs(5));
    t.tmux(&["send-keys", "Enter"]);
    t.wait_line("axb", Duration::from_secs(10));
    [typed, backspace, inserted]
}

#[test]
fn keystroke_shows_at_once_with_prediction() {
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
    let modes = [
        "MOSH_PREDICTION_DISPLAY=always",
        "MOSH_PREDICTION_DISPLAY=adaptive",
        "MOSH_PREDICTION_DISPLAY=always MOSH_PREDICTION_OVERWRITE=yes",
        "MOSH_PREDICTION_DISPLAY=never",
    ];
    for (i, (name, client)) in clients().into_iter().enumerate() {
        for (k, env) in modes.iter().enumerate() {
            let base = 61350 + 16 * i as u16 + 4 * k as u16;
            let [typed, backspace, inserted] = editing_latencies(name, &client, base, env);
            println!(
                "[{name}] 300 ms RTT, {env}: key {} ms, backspace {} ms, insert {} ms",
                typed.as_millis(),
                backspace.as_millis(),
                inserted.as_millis()
            );
            for (what, took) in [
                ("key", typed),
                ("backspace", backspace),
                ("insert", inserted),
            ] {
                if env.ends_with("never") {
                    assert!(
                        took > Duration::from_millis(250),
                        "[{name}] without prediction the {what} took only {took:?}: the test does not measure prediction"
                    );
                } else {
                    // Adaptive: at 300 ms RTT the send interval is far above SRTT_TRIGGER_HIGH.
                    assert!(
                        took < Duration::from_millis(50),
                        "[{name}] {env}: predicted {what} took {took:?}"
                    );
                }
            }
        }
    }
}

/// The server does not echo `q`: a shell loop echoes every typed character
/// except `q`. The prediction of `q` shows, then goes away when the
/// server's frame arrives without it.
///
/// A prediction is only shown in a confirmed epoch, and a prediction whose
/// echo is not on the server's screen 50 ms after the keystroke reached it
/// (upstream's `ECHO_TIMEOUT`) counts as wrong and makes the epoch
/// tentative again. On a loaded machine the shell loop's echo can take
/// longer than that, so the `q` typed after it may legitimately not be
/// shown. Then the attempt is repeated (two more echoed keys, which
/// confirm a new epoch, then `q` again), up to 10 times; `q` never comes
/// from the server, so any `q` on screen is the prediction.
#[test]
fn wrong_prediction_is_removed() {
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
    const KEYS: &str = "abcdefghijklmnoprstuvwxyz";
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let s = Session::start(
            &format!("wrong-{name}"),
            &client,
            61382 + 4 * i as u16,
            "MOSH_PREDICTION_DISPLAY=always",
            SHELL,
        );
        s.tmux.wait_line("R$", Duration::from_secs(30));
        s.tmux.type_line(
            "clear; stty -icanon -echo; while read -rsn1 c; do [ \"$c\" = q ] || printf %s \"$c\"; done",
        );
        s.tmux.wait_until("clear", Duration::from_secs(30), |sc| {
            sc.lines().all(|l| l.is_empty())
        });
        let mut text = String::new();
        let mut keys = KEYS.chars().cycle();
        let mut attempt = 0;
        let (shown, removed, screen) = loop {
            attempt += 1;
            assert!(
                attempt <= 10,
                "[{name}] the prediction of q never showed in 10 attempts; screen:\n{}",
                s.tmux.capture()
            );
            for _ in 0..2 {
                let c = keys.next().unwrap();
                s.tmux.keys(&c.to_string());
                text.push(c);
                s.tmux.wait_line(&text, Duration::from_secs(30));
            }
            // Let the server's echo of the last key arrive and be judged.
            std::thread::sleep(Duration::from_millis(1000));

            let typed = Instant::now();
            s.tmux.keys("q");
            let with_q = format!("{text}q");
            let Some((_, shown)) =
                s.tmux
                    .poll(Duration::from_millis(5), Duration::from_secs(2), |sc| {
                        sc.lines().any(|l| l == with_q)
                    })
            else {
                println!("[{name}] attempt {attempt}: q was not predicted (epoch not confirmed)");
                continue;
            };
            let (screen, _) = s
                .tmux
                .poll(Duration::from_millis(5), Duration::from_secs(30), |sc| {
                    sc.lines().any(|l| l == text)
                })
                .expect("the wrong prediction was never removed");
            break (shown, typed.elapsed(), screen);
        };
        println!(
            "[{name}] attempt {attempt}: q predicted after {} ms, removed after {} ms",
            shown.as_millis(),
            removed.as_millis()
        );
        assert!(!screen.contains('q'), "[{name}] screen:\n{screen}");
        // It stays removed.
        std::thread::sleep(Duration::from_millis(1000));
        let screen = s.tmux.capture();
        assert_eq!(
            screen.lines().find(|l| !l.is_empty()),
            Some(text.as_str()),
            "[{name}] screen:\n{screen}"
        );
    }
}

/// Upstream's prediction-unicode.test: with predictions always shown,
/// typing text with non-ASCII characters into `cat` must never show a
/// garbled glyph (that test greps for `)` and a raw `\374`), and the final
/// screen is what was typed.
#[test]
fn unicode_input_with_prediction() {
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
    for (i, (name, client)) in clients().into_iter().enumerate() {
        let s = Session::start(
            &format!("unicode-{name}"),
            &client,
            61390 + 4 * i as u16,
            "MOSH_PREDICTION_DISPLAY=always",
            SHELL,
        );
        s.tmux.wait_line("R$", Duration::from_secs(30));
        s.tmux.type_line("clear; cat > /dev/null");
        s.tmux.wait_until("clear", Duration::from_secs(10), |sc| {
            sc.lines().all(|l| l.is_empty())
        });
        let mut garbled = Vec::new();
        let pieces = ["gl", "ü", "ck fa", "ĩ", "l "];
        for _ in 0..3 {
            for _ in 0..5 {
                for p in pieces {
                    s.tmux.keys(p);
                    for _ in 0..4 {
                        std::thread::sleep(Duration::from_millis(10));
                        let sc = s.tmux.capture();
                        if sc.contains(')') || sc.contains('\u{fc}') && !sc.contains("gl\u{fc}") {
                            garbled.push(sc);
                        }
                    }
                }
            }
            s.tmux.tmux(&["send-keys", "Enter"]);
        }
        let line = "glück faĩl ".repeat(5);
        let line = line.trim_end();
        let screen = s
            .tmux
            .wait_until("all typed lines", Duration::from_secs(10), |sc| {
                sc.lines().filter(|l| *l == line).count() == 3
            });
        println!("[{name}] final screen:\n{}", screen.trim_end());
        assert!(
            garbled.is_empty(),
            "[{name}] garbled screens:\n{}",
            garbled.join("\n----\n")
        );
    }
}
