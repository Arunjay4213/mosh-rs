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
        // Canonical, so it is the same path the launcher reports for itself
        // (`current_exe`) in its ProxyCommand; `target/../` or a symlink in it
        // would otherwise defeat the `$0` normalization below.
        let bin = target.join("release").join("mosh");
        std::fs::canonicalize(&bin).unwrap_or(bin)
    })
    .clone()
}

/// `--client=<real mosh-client> --server=<real mosh-server>`, from PATH
/// (`mosh_tests::upstream`); tests check `upstream::require()` first.
fn real_client_server() -> String {
    let up = mosh_tests::upstream::get();
    format!("--client={} --server={}", up.client, up.server)
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

    /// Wait for the shell's `MOSH_EXIT=<status>` line. The typed command
    /// line (`...; echo MOSH_EXIT=$?`) holds `MOSH_EXIT=` too, and is back
    /// on screen as soon as the client leaves the alternate screen, before
    /// the client has exited and the shell has printed the status; waiting
    /// for the text alone could capture that moment (CI, 565684a).
    fn wait_exit(&self, timeout: Duration) -> String {
        let deadline = Instant::now() + timeout;
        loop {
            let screen = self.capture();
            if screen.lines().any(|l| {
                l.strip_prefix("MOSH_EXIT=")
                    .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
            }) {
                return screen;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for a MOSH_EXIT=<status> line; screen:\n{screen}"
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
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
    require_ssh_localhost();
    let tmux = Tmux::start(
        "localhost",
        &session(&format!(
            "{} -p 60500 localhost -- sh -c 'echo LAUNCH_OK; sleep 1'",
            real_client_server()
        )),
    );
    let live = tmux.wait_for("LAUNCH_OK", Duration::from_secs(30));
    println!("live session screen:\n{}", live.trim_end());
    let done = tmux.wait_exit(Duration::from_secs(30));
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
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
    require_ssh_localhost();
    let user = std::env::var("USER").expect("USER");
    let tmux = Tmux::start(
        "userip",
        &session(&format!(
            "{} {user}@127.0.0.1 -- true",
            real_client_server()
        )),
    );
    let done = tmux.wait_exit(Duration::from_secs(30));
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
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
    require_ssh_localhost();
    let out = Command::new(mosh_binary())
        .args([
            &format!("--client={}", mosh_tests::upstream::get().client),
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
    let Some(_) = mosh_tests::upstream::require() else {
        return;
    };
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
                "{family} {} mosh-dualstack -- sh -c 'echo LAUNCH_OK; sleep 1'",
                real_client_server()
            )),
        );
        tmux.wait_for("LAUNCH_OK", Duration::from_secs(30));
        let done = tmux.wait_exit(Duration::from_secs(30));
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

/// The ProxyCommand words on Windows, where ssh.exe runs no shell and the
/// command line is split by the Windows rules (see
/// `mosh_platform::process::proxy_command_quote`). Single quotes there made
/// ssh look for a file named `'C:\...\mosh.exe'` (CreateProcessW error 2,
/// found in the Phase 3 laptop test). Devbox cannot run ssh.exe, so this
/// checks the exact strings the Windows launcher puts into ProxyCommand.
#[test]
fn windows_proxy_command_quoting() {
    use mosh_platform::process::windows_command_line_quote as q;
    let cases = [
        // Plain path: no quotes, as verified by hand on the laptop.
        (
            r"C:\Users\arunj\mosh-rs-test\mosh.exe",
            r"C:\Users\arunj\mosh-rs-test\mosh.exe",
        ),
        // Path with a space: double quotes, backslashes untouched.
        (
            r"C:\Program Files\mosh-rs\mosh.exe",
            r#""C:\Program Files\mosh-rs\mosh.exe""#,
        ),
        // Trailing backslash with a space: doubled so it does not escape the closing quote.
        (r"C:\dir with space\", r#""C:\dir with space\\""#),
        // Forward slashes: unchanged.
        (
            "C:/Users/arunj/mosh-rs-test/mosh.exe",
            "C:/Users/arunj/mosh-rs-test/mosh.exe",
        ),
        ("--family=prefer-inet", "--family=prefer-inet"),
        // Embedded quotes and backslashes before them.
        (r#"a"b"#, r#""a\"b""#),
        (r#"a\"b"#, r#""a\\\"b""#),
        ("tab\there", "\"tab\there\""),
        ("", r#""""#),
    ];
    for (arg, want) in cases {
        println!("{arg:<40} -> {}", q(arg));
        assert_eq!(q(arg), want, "quoting {arg:?}");
    }
    // The whole ProxyCommand value for an install path with a space. It starts
    // with a double quote, so Win32-OpenSSH's build_commandline_string passes
    // it to CreateProcessW unchanged.
    let proxy = format!(
        "{} {} --fake-proxy -- %h %p",
        q(r"C:\Program Files\mosh-rs\mosh.exe"),
        q("--family=prefer-inet")
    );
    println!("ProxyCommand={proxy}");
    assert_eq!(
        proxy,
        r#""C:\Program Files\mosh-rs\mosh.exe" --family=prefer-inet --fake-proxy -- %h %p"#
    );
}

/// A scratch directory holding a fake `ssh` and a fake `mosh-client`, so
/// both launchers can be run with the same arguments and compared without a
/// network. The fake ssh records its argv and prints `FAKE_SSH_OUT`; the
/// fake client answers `-c` with `FAKE_COLORS` and otherwise records its
/// argv and `MOSH_*` environment.
fn fake_tools() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("mosh-rs-launcher-fakes-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ssh = r#"#!/bin/sh
for a in "$@"; do printf '%s\n' "$a"; done > "$FAKE_DIR/ssh.log"
printf '%b' "${FAKE_SSH_OUT-MOSH IP 192.0.2.7\nMOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n}"
exit "${FAKE_SSH_EXIT:-0}"
"#;
        let client = r#"#!/bin/sh
if [ "$1" = "-c" ]; then printf '%b' "${FAKE_COLORS-256\n}"; exit "${FAKE_COLORS_EXIT:-0}"; fi
{ for a in "$@"; do printf 'arg %s\n' "$a"; done; env | grep '^MOSH_' | grep -v '^MOSH_CLIENT_PID=' | sort; } > "$FAKE_DIR/client.log"
"#;
        for (name, text) in [("fakessh", ssh), ("fakeclient", client)] {
            let path = dir.join(name);
            std::fs::write(&path, text).unwrap();
            Command::new("chmod").arg("+x").arg(&path).status().unwrap();
        }
        dir
    })
    .clone()
}

/// Replace `$0` paths, drop Perl's `at SCRIPT line N.` suffix (which ours
/// does not print, PLAN 9.16) and the usage text (compared separately) so
/// the two launchers' outputs can be diffed.
fn normalize(text: &str, bin: &str) -> String {
    let mut out = String::new();
    for line in text.replace(bin, "$0").lines() {
        if line.starts_with("Usage: ") {
            out.push_str("[usage]\n");
            break;
        }
        let line = match line.rfind(" at $0 line ") {
            Some(i) if line.ends_with('.') => line[..i].to_string(),
            _ => line.to_string(),
        };
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// Run one launcher with the fakes and return everything observable:
/// exit status, stdout, stderr, the ssh argv and the client argv and env.
fn observe(bin: &str, args: &[&str], env: &[(&str, &str)], tag: &str) -> String {
    let tools = fake_tools();
    let out_dir = tools.join(tag);
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir).unwrap();
    let mut cmd = Command::new(bin);
    cmd.arg(format!("--ssh={}", tools.join("fakessh").display()))
        .arg(format!("--client={}", tools.join("fakeclient").display()))
        .args(args)
        .env("TERM", "xterm-256color")
        .env("LANG", "C.UTF-8")
        .env("FAKE_DIR", &out_dir)
        .env_remove("MOSH_PREDICTION_DISPLAY")
        .stdin(Stdio::null());
    for name in ["LANGUAGE", "LC_ALL", "LC_CTYPE", "LC_MESSAGES"] {
        cmd.env_remove(name);
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("run launcher");
    let read = |name: &str| {
        std::fs::read_to_string(out_dir.join(name)).unwrap_or_else(|_| "(none)\n".into())
    };
    format!(
        "exit {:?}\n--- stdout\n{}--- stderr\n{}--- ssh argv\n{}--- client\n{}",
        out.status.code(),
        normalize(&String::from_utf8_lossy(&out.stdout), bin),
        normalize(&String::from_utf8_lossy(&out.stderr), bin),
        normalize(&read("ssh.log"), bin),
        normalize(&read("client.log"), bin),
    )
}

/// The lines where two observations differ, numbered, for a failure
/// message that shows every difference even when only the end of the log
/// is kept.
fn differing_lines(want: &str, got: &str) -> String {
    let (want, got): (Vec<&str>, Vec<&str>) = (want.lines().collect(), got.lines().collect());
    let mut out = String::new();
    for i in 0..want.len().max(got.len()) {
        let a = want.get(i).copied().unwrap_or("(no line)");
        let b = got.get(i).copied().unwrap_or("(no line)");
        if a != b {
            out.push_str(&format!(
                "  line {}: mosh.pl {a:?}\n          ours    {b:?}\n",
                i + 1
            ));
        }
    }
    out
}

/// One differential case: launcher arguments and extra environment.
type Case<'a> = (Vec<&'a str>, Vec<(&'a str, &'a str)>);

/// Option parsing (Getopt::Long: unique prefixes, case-insensitive names,
/// no bundling, negations, values from the next argument), the color count,
/// `--experimental-remote-ip`, `--local`, the `MOSH ...` line parsing and
/// every error message and exit code, compared with mosh.pl case by case:
/// same arguments, same fake ssh and client, identical observable results.
#[test]
fn options_and_messages_match_mosh_pl() {
    let Some(up) = mosh_tests::upstream::require() else {
        return;
    };
    let mosh_pl = &up.mosh;
    let ours = mosh_binary().display().to_string();
    let connect = "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n";
    let fake_server = fake_tools().join("fakessh").display().to_string();
    let plain: &[&[&str]] = &[
        &["host"],
        &["--pred=never", "host"],
        &["--Predict=Always", "host"],
        &["-an", "host"],
        &["-AN", "host"],
        &["-A", "host"],
        &["-N", "host"],
        &["-P", "60001", "host"],
        &["-p60001", "host"],
        &["-port=60001", "host"],
        &["--po", "6", "host"],
        &["--port", "--local", "host"],
        &["--port=", "host"],
        &["--port"],
        &["--local=1", "host"],
        &["--LOC=1", "host"],
        &["-A=1", "host"],
        &["--no-in", "host"],
        &["--NO-INIT", "host"],
        &["--no-INIT=1", "host"],
        &["--nos", "host"],
        &["--No-Ssh-Pty", "host"],
        &["--no-pred", "host"],
        &["--noo", "host"],
        &["--no-a", "host"],
        &["--no", "host"],
        &["--o", "host"],
        &["--4", "host"],
        &["--4=1", "host"],
        &["--fam=INET6", "host"],
        &["--f=inet", "host"],
        &["--ss=x", "host"],
        &["--b=any", "host"],
        &["--fake-p=x", "host"],
        &["host", "-a"],
        &["host", "cmd", "--foo"],
        &["host", "--", "cmd", "--foo"],
        &["--", "--local"],
        &["-", "host"],
        &["--=x", "host"],
        &["---local", "host"],
        &["--bogus", "--other", "host"],
        &["-p", "1", "-p", "2", "host"],
        &["host", "--server"],
        &["--family=bogus", "host"],
        &["--experimental-remote-ip=bogus", "host"],
        &["-p", "99999", "host"],
        &["-p", "0:5", "host"],
        &["-p", "6:5", "host"],
        &["-p", "6a", "host"],
        &["user@host", "--", "sh", "-c", "echo \"a b\""],
        &["--client=/nonexistent/client", "host"],
        &["--ssh=/nonexistent/ssh", "host"],
        &["--ssh=ssh -p 2222 -o Foo=\"a b\"", "host"],
    ];
    let mut cases: Vec<Case> = plain.iter().map(|a| (a.to_vec(), Vec::new())).collect();
    let with_env: Vec<Case> = vec![
        (vec!["host"], vec![("FAKE_COLORS_EXIT", "1")]),
        (vec!["host"], vec![("FAKE_COLORS", "256\\n\\n")]),
        (vec!["host"], vec![("FAKE_COLORS", "abc\\n")]),
        (vec!["host"], vec![("FAKE_COLORS", "")]),
        (vec!["host"], vec![("FAKE_SSH_OUT", "hello\\nMOSH CONNECT x y\\n")]),
        (vec!["host"], vec![("FAKE_SSH_OUT", "MOSH IP a b\\n")]),
        (vec!["host"], vec![("FAKE_SSH_OUT", "MOSH IP 192.0.2.7\\nMOSH IP 192.0.2.8\\n")]),
        (vec!["host"], vec![("FAKE_SSH_OUT", connect)]),
        (vec!["host"], vec![("FAKE_SSH_OUT", "MOSH IP 192.0.2.7\\n")]),
        (vec!["host"], vec![("MOSH_PREDICTION_DISPLAY", "bogus")]),
        (vec!["host"], vec![("MOSH_PREDICTION_DISPLAY", "never")]),
        (
            vec!["-p", "60001:60010", "host"],
            vec![("FAKE_SSH_OUT", "MOSH IP 192.0.2.7\\nBad UDP port range\\n")],
        ),
        (
            vec!["--experimental-remote-ip=remote", "host"],
            vec![("FAKE_SSH_OUT", "MOSH SSH_CONNECTION 1 2 3\\n")],
        ),
        (
            vec!["--experimental-remote-ip=remote", "-6", "host"],
            vec![(
                "FAKE_SSH_OUT",
                "\\r\\nMOSH SSH_CONNECTION 192.0.2.1 5555 192.0.2.9 22\\r\\nMOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\\r\\n",
            )],
        ),
        (vec!["--experimental-remote-ip=local", "me@127.0.0.1", "--", "ls"], vec![("FAKE_SSH_OUT", connect)]),
        (vec!["--e=local", "a@b@127.0.0.1"], vec![("FAKE_SSH_OUT", connect)]),
        (vec!["--experimental-remote-ip=local", "-6", "127.0.0.1"], vec![("FAKE_SSH_OUT", connect)]),
        (vec!["--experimental-remote-ip=local", "nosuch.invalid"], vec![("FAKE_SSH_OUT", connect)]),
    ];
    cases.extend(with_env);
    let local_server = format!("--server={fake_server}");
    cases.push((
        vec!["--local", &local_server, "127.0.0.1"],
        vec![("FAKE_SSH_OUT", connect)],
    ));

    let mut failures = 0;
    let mut report = String::new();
    for (args, env) in &cases {
        let want = observe(mosh_pl, args, env, "real");
        let got = observe(&ours, args, env, "ours");
        if want == got {
            println!(
                "same: {args:?} {env:?} -> {}",
                want.lines().next().unwrap_or("")
            );
        } else {
            failures += 1;
            println!("DIFFERENT: {args:?} {env:?}\n=== mosh.pl\n{want}=== ours\n{got}");
            report.push_str(&format!(
                "case {args:?} env {env:?}: mosh.pl {}, ours {}\n{}",
                want.lines().next().unwrap_or(""),
                got.lines().next().unwrap_or(""),
                differing_lines(&want, &got)
            ));
        }
    }
    println!("{} cases, {failures} different", cases.len());
    assert!(
        failures == 0,
        "{failures} of {} cases differ from mosh.pl:\n{report}",
        cases.len()
    );

    // The usage text differs only where PLAN allows: `--bind-server` takes
    // ssh or any (PLAN 9.3), `--scrollback` exists (PLAN section 6), and bug
    // reports go to this repository (PLAN 9.16).
    let help = |bin: &str| {
        let out = Command::new(bin).arg("--help").output().unwrap();
        String::from_utf8_lossy(&out.stdout).replace(bin, "$0")
    };
    let want = help(mosh_pl)
        .replace("--bind-server={ssh|any|IP}", "--bind-server={ssh|any}")
        .replace(
            "Please report bugs to mosh-devel@mit.edu.",
            "Please report bugs to https://github.com/Arunjay4213/mosh-rs/issues.",
        )
        .replace(
            "\n\n        --local ",
            "\n\n        --scrollback         keep the session's output in this terminal's\n                                scrollback (needs a mosh-rs server)\n\n        --local ",
        );
    assert_eq!(
        help(&ours),
        want,
        "usage text differs from mosh.pl beyond PLAN 9.3 and section 6"
    );
}

/// `resolvename` (mosh.pl lines 484-538) asks for the canonical name of a
/// host that is not a numeric address and resolves that name again. Devbox
/// has `127.0.0.1 mosh-canon mosh-alias` and `::1 mosh-canon` in /etc/hosts
/// (/etc/hosts cannot hold a CNAME; glibc reports the first name of the
/// line as the canonical name): `mosh-alias` alone resolves to 127.0.0.1
/// only, its canonical name to both families. The proxy must choose from
/// the canonical name's addresses, as mosh.pl does.
#[test]
fn proxy_resolves_canonical_name_like_mosh_pl() {
    let hosts = std::fs::read_to_string("/etc/hosts").unwrap_or_default();
    let Some(up) = mosh_tests::upstream::require() else {
        return;
    };
    let mosh_pl = &up.mosh;
    if !hosts.contains("mosh-alias") {
        println!("SKIPPED: needs the mosh-alias/mosh-canon lines in /etc/hosts");
        return;
    }
    let ours = mosh_binary().display().to_string();
    for family in ["auto", "prefer-inet6", "prefer-inet", "all", "inet6"] {
        let run = |bin: &str| {
            let out = Command::new(bin)
                .args([
                    &format!("--family={family}"),
                    "--fake-proxy",
                    "--",
                    "mosh-alias",
                    "22",
                ])
                .stdin(Stdio::null())
                .output()
                .unwrap();
            format!(
                "exit {:?}: {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr)
                    .replace(bin, "$0")
                    .trim_end()
            )
        };
        let (want, got) = (run(mosh_pl), run(&ours));
        println!("--family={family}: mosh.pl {want} | ours {got}");
        assert_eq!(got, want, "--family={family}");
    }
}

/// The two places mosh resolves numeric addresses must agree: the network
/// code's `socket::resolve` (upstream network.cc, used by mosh-client and
/// mosh-server for the address on their command line) and the launcher's
/// `netdb::getaddrinfo` (mosh.pl `resolvename`). Review P10 found they
/// decoded `sockaddr_in6` differently; a scoped IPv6 literal such as a
/// link-local server address goes through both, so both must give the same
/// address, port, scope id and flow information.
#[test]
fn numeric_addresses_resolve_the_same_in_both_paths() {
    // The loopback interface, index 1 on Linux (`lo`) and macOS (`lo0`).
    const LOOPBACK: &str = if cfg!(target_os = "linux") {
        "lo"
    } else {
        "lo0"
    };
    // `lo` is interface 1 on Linux; the numeric zone works everywhere.
    let cases = [
        ("fe80::1%1", "60001"),
        (&*format!("fe80::1%{LOOPBACK}"), "60001"),
        ("fe80::abcd:1234%1", "22"),
        ("::1", "60001"),
        ("127.0.0.1", "60001"),
    ];
    for (host, port) in cases {
        let via_socket = mosh_platform::socket::resolve(Some(host), port, false)
            .unwrap_or_else(|e| panic!("socket::resolve({host}): {e:?}"));
        let via_netdb = mosh_platform::netdb::getaddrinfo(
            host,
            port,
            None,
            mosh_platform::netdb::Flags::NumericHost,
        )
        .unwrap_or_else(|e| panic!("netdb::getaddrinfo({host}): {}", e.message));
        println!(
            "{host:<20} socket.rs {via_socket:?}  netdb.rs {:?}",
            via_netdb.addrs.first()
        );
        assert_eq!(via_netdb.addrs.first(), Some(&via_socket), "{host}");
        if let std::net::SocketAddr::V6(a) = via_socket {
            if host.contains('%') {
                assert_eq!(
                    a.scope_id(),
                    1,
                    "{host}: the zone must become scope id 1 (the loopback interface)"
                );
            }
            // getaddrinfo never sets a flow label; both paths read the raw
            // `sin6_flowinfo` field, as std's SocketAddrV6 does.
            assert_eq!(a.flowinfo(), 0, "{host}");
        }
    }
}
