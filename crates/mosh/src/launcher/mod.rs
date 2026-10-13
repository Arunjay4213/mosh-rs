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
// Ported from: upstream/scripts/mosh.pl
//
// Deviation (PLAN 9.1): instead of acting as its own ProxyCommand
// (`--fake-proxy`) to learn which address ssh connected to, the launcher asks
// `ssh -G` for the resolved hostname and resolves it with the system
// resolver before starting ssh. Everything else follows mosh.pl: the same
// options, the same `mosh-server new` command line, the same output parsing
// and the same messages.

use std::ffi::OsString;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::process::{exit, Command, Stdio};

use mosh_platform::locale::LOCALE_VARS;
use mosh_platform::process;

/// Perl `die` exits with `$!` when it is set, else 255. Before ssh runs that
/// is 255 in practice (checked against /usr/bin/mosh: bad -p gives 255).
const DIE: i32 = 255;
/// After `close $pipe; waitpid $pid, 0` the child is already reaped, so
/// waitpid fails with ECHILD and `$!` is 10. Real mosh 1.4.0 exits with 10
/// for "Did not find mosh server startup message".
const DIE_AFTER_WAIT: i32 = 10;

fn die(msg: &str, code: i32) -> ! {
    let _ = std::io::stdout().flush();
    eprint!("{msg}");
    exit(code)
}

fn usage(argv0: &str) -> String {
    format!(
        r#"Usage: {argv0} [options] [--] [user@]host [command...]
        --client=PATH        mosh client on local machine
                                (default: "mosh-client")
        --server=COMMAND     mosh server on remote machine
                                (default: "mosh-server")

        --predict=adaptive      local echo for slower links [default]
-a      --predict=always        use local echo even on fast links
-n      --predict=never         never use local echo
        --predict=experimental  aggressively echo even when incorrect

-o      --predict-overwrite     prediction overwrites instead of inserting

-4      --family=inet        use IPv4 only
-6      --family=inet6       use IPv6 only
        --family=auto        autodetect network type for single-family hosts only
        --family=all         try all network types
        --family=prefer-inet use all network types, but try IPv4 first [default]
        --family=prefer-inet6 use all network types, but try IPv6 first
-p PORT[:PORT2]
        --port=PORT[:PORT2]  server-side UDP port or range
                                (No effect on server-side SSH port)
        --bind-server={{ssh|any}}  ask the server to reply from an IP address
                                       (default: "ssh")

        --ssh=COMMAND        ssh command to run when setting up session
                                (example: "ssh -p 2222")
                                (default: "ssh")

        --no-ssh-pty         do not allocate a pseudo tty on ssh connection

        --no-init            do not send terminal initialization string

        --local              run mosh-server locally without using ssh

        --help               this message
        --version            version and copyright information

Please report bugs to mosh-devel@mit.edu.
Mosh home page: https://mosh.org
"#
    )
}

fn version_message() -> String {
    let v = env!("CARGO_PKG_VERSION");
    format!(
        "mosh-rs {v} [build mosh-rs {v}]
Copyright 2012 Keith Winstein <mosh-devel@mit.edu>
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
"
    )
}

fn predict_check(argv0: &str, predict: &str, env_set: bool) {
    if !["adaptive", "always", "never", "experimental"].contains(&predict) {
        let explanation = if env_set { " (MOSH_PREDICTION_DISPLAY in environment)" } else { "" };
        eprint!("{argv0}: Unknown mode \"{predict}\"{explanation}.\n\n");
        die(&usage(argv0), DIE);
    }
}

/// Text::ParseWords::shellwords: split on whitespace, honoring single
/// quotes, double quotes and backslash escapes.
fn shellwords(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_word = false;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                in_word = true;
                for c in chars.by_ref() {
                    if c == '\'' {
                        break;
                    }
                    word.push(c);
                }
            }
            '"' => {
                in_word = true;
                while let Some(c) = chars.next() {
                    match c {
                        '"' => break,
                        '\\' => word.extend(chars.next()),
                        _ => word.push(c),
                    }
                }
            }
            '\\' => {
                in_word = true;
                word.extend(chars.next());
            }
            c if c.is_whitespace() => {
                if in_word {
                    words.push(std::mem::take(&mut word));
                    in_word = false;
                }
            }
            c => {
                in_word = true;
                word.push(c);
            }
        }
    }
    if in_word {
        words.push(word);
    }
    words
}

/// `sub shell_quote { join ' ', map {(my $a = $_) =~ s/'/'\\''/g; "'$a'"} @_ }`
fn shell_quote(args: &[String]) -> String {
    args.iter().map(|a| format!("'{}'", a.replace('\'', "'\\''"))).collect::<Vec<_>>().join(" ")
}

fn locale_vars() -> Vec<String> {
    LOCALE_VARS
        .iter()
        .filter_map(|name| std::env::var_os(name).map(|v| format!("{name}={}", v.to_string_lossy())))
        .collect()
}

/// The checks of mosh.pl lines 213-234 on `PORT[:PORT2]`.
fn check_port_request(argv0: &str, port_request: &str) {
    let (low, high) = match port_request.split_once(':') {
        Some((low, high)) => (low, Some(high)),
        None => (port_request, None),
    };
    let is_number = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !is_number(low) || !high.is_none_or(is_number) {
        die(&format!("{argv0}: Server-side port or range ({port_request}) is not valid.\n"), DIE);
    }
    // Perl compares numerically; f64 behaves like Perl for over-long digit strings.
    let low_n: f64 = low.parse().unwrap();
    if !(0.0..=65535.0).contains(&low_n) {
        die(&format!("{argv0}: Server-side (low) port ({low}) must be within valid range [0..65535].\n"), DIE);
    }
    if let Some(high) = high {
        let high_n: f64 = high.parse().unwrap();
        if high_n <= 0.0 || high_n > 65535.0 {
            die(&format!("{argv0}: Server-side high port ({high}) must be within valid range [1..65535].\n"), DIE);
        }
        if low_n == 0.0 {
            die(
                &format!("{argv0}: Server-side port ranges may not be used with starting port 0 ({port_request}).\n"),
                DIE,
            );
        }
        if low_n > high_n {
            die(&format!("{argv0}: Server-side port range ({port_request}): low port greater than high port.\n"), DIE);
        }
    }
}

/// Deviation 9.1, first half: the hostname ssh would connect to, after
/// `~/.ssh/config` (HostName, aliases), from `ssh [opts] -G [user@]host`.
fn ssh_resolved_hostname(argv0: &str, ssh: &[String], userhost: &str) -> String {
    let output = Command::new(&ssh[0])
        .args(&ssh[1..])
        .arg("-G")
        .arg(userhost)
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|e| die(&format!("Cannot exec ssh: {e}\n"), DIE));
    if !output.status.success() {
        let _ = std::io::stderr().write_all(&output.stderr);
        die(&format!("{argv0}: `{} -G {userhost}` failed ({}).\n", ssh.join(" "), output.status), DIE);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(hostname) = line.strip_prefix("hostname ") {
            return hostname.trim().to_string();
        }
    }
    // ssh -G always prints hostname; fall back to what the user typed.
    userhost.rsplit_once('@').map_or(userhost, |(_, host)| host).to_string()
}

/// Deviation 9.1, second half: `sub resolvename` (mosh.pl lines 484-538)
/// with the system resolver, then the first address, as the proxy would
/// have connected to first.
fn resolvename(argv0: &str, host: &str, family: &str) -> String {
    let mut res: Vec<SocketAddr> = match (host, 22).to_socket_addrs() {
        Ok(res) => res.collect(),
        Err(err) => die(&format!("{argv0}: could not get canonical name for {host}: {err}\n"), DIE),
    };
    let is_family = |a: &SocketAddr, fam: &str| if fam == "inet" { a.is_ipv4() } else { a.is_ipv6() };
    if family == "inet" || family == "inet6" {
        /* If v4 or v6 was specified, reduce the host list. */
        res.retain(|a| is_family(a, family));
    } else if let Some(prefer) = family.strip_prefix("prefer-") {
        /* If prefer-* was specified, reorder the host list to put that family first. */
        if prefer != "inet" && prefer != "inet6" {
            die(&format!("{argv0}: Invalid preferred family {family}\n"), DIE);
        }
        res.sort_by_key(|a| !is_family(a, prefer));
    } else if family != "all" {
        /* If v4/v6/all were not specified, verify that this host only has one address family available. */
        if res.iter().any(|a| a.is_ipv4()) && res.iter().any(|a| a.is_ipv6()) {
            die(&format!("{argv0}: host has both IPv4 and IPv6 addresses, use --family to specify behavior\n"), DIE);
        }
    }
    match res.first() {
        None => die(&format!("{argv0}: could not find address for {host}\n"), DIE),
        Some(SocketAddr::V6(a)) if a.scope_id() != 0 => format!("{}%{}", a.ip(), a.scope_id()),
        Some(a) => IpAddr::to_string(&a.ip()),
    }
}

/// `m{^MOSH CONNECT (\d+?) ([A-Za-z0-9/+]{22})\s*$}`
fn parse_mosh_connect(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("MOSH CONNECT ")?;
    let (port, rest) = rest.split_once(' ')?;
    if port.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let key = rest.trim_end();
    let key_ok = key.len() == 22 && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'/' || b == b'+');
    key_ok.then(|| (port.to_string(), key.to_string()))
}

/// `m{^MOSH IP (\S+)\s*$}`
fn parse_mosh_ip(line: &str) -> Option<String> {
    let rest = line.strip_prefix("MOSH IP ")?;
    let ip = rest.trim_end();
    (!ip.is_empty() && !ip.contains(char::is_whitespace)).then(|| ip.to_string())
}

/// `mosh-client` next to this executable, else `mosh-client` from PATH.
fn default_client() -> String {
    let name = format!("mosh-client{}", std::env::consts::EXE_SUFFIX);
    if let Ok(exe) = std::env::current_exe() {
        let sibling = exe.with_file_name(&name);
        if sibling.is_file() {
            return sibling.to_string_lossy().into_owned();
        }
    }
    "mosh-client".to_string()
}

fn os(v: OsString) -> String {
    v.to_string_lossy().into_owned()
}

pub fn main() -> ! {
    let mut args = std::env::args_os();
    let argv0 = args.next().map(os).unwrap_or_else(|| "mosh".to_string());
    let cmdline: Vec<String> = args.map(os).collect();

    let mut client = default_client();
    let mut server = "mosh-server".to_string();
    let mut predict: Option<String> = None;
    let mut overwrite = false;
    let mut bind_ip: Option<String> = None;
    let mut family = "prefer-inet".to_string();
    let mut port_request: Option<String> = None;
    let mut ssh = vec!["ssh".to_string()];
    let mut term_init = true;
    let mut localhost = false;
    let mut ssh_pty = true;
    let mut help = false;
    let mut version = false;
    let mut positional: Vec<String> = Vec::new();

    // Getopt::Long in its default (permute) mode: options may follow the
    // host; everything after "--" is positional.
    let mut parser = lexopt::Parser::from_env();
    let parsed: Result<(), lexopt::Error> = (|| {
        use lexopt::Arg::{Long, Short, Value};
        while let Some(arg) = parser.next()? {
            match arg {
                Long("client") => client = os(parser.value()?),
                Long("server") => server = os(parser.value()?),
                Long("predict") => predict = Some(os(parser.value()?)),
                Long("predict-overwrite") | Short('o') | Long("o") => overwrite = true,
                Long("no-predict-overwrite") | Long("nopredict-overwrite") => overwrite = false,
                Long("port") | Short('p') | Long("p") => port_request = Some(os(parser.value()?)),
                Short('a') | Long("a") => predict = Some("always".to_string()),
                Short('n') | Long("n") => predict = Some("never".to_string()),
                Long("family") => family = os(parser.value()?),
                Short('4') | Long("4") => family = "inet".to_string(),
                Short('6') | Long("6") => family = "inet6".to_string(),
                Long("ssh") => ssh = shellwords(&os(parser.value()?)),
                Long("ssh-pty") => ssh_pty = true,
                Long("no-ssh-pty") | Long("nossh-pty") => ssh_pty = false,
                Long("init") => term_init = true,
                Long("no-init") | Long("noinit") => term_init = false,
                Long("local") => localhost = true,
                Long("help") => help = true,
                Long("version") => version = true,
                Long("bind-server") => bind_ip = Some(os(parser.value()?)),
                // Accepted and ignored until scrollback lands (PLAN phase 4).
                Long("scrollback") => {
                    parser.optional_value();
                }
                Value(v) => positional.push(os(v)),
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(())
    })();
    if let Err(err) = parsed {
        match err {
            lexopt::Error::UnexpectedOption(o) => eprintln!("Unknown option: {}", o.trim_start_matches('-')),
            lexopt::Error::MissingValue { option: Some(o) } => {
                eprintln!("Option {} requires an argument", o.trim_start_matches('-'))
            }
            other => eprintln!("{other}"),
        }
        die(&usage(&argv0), DIE);
    }

    if help {
        print!("{}", usage(&argv0));
        exit(0);
    }
    if version {
        print!("{}", version_message());
        exit(0);
    }

    let predict = match predict {
        Some(p) => {
            predict_check(&argv0, &p, false);
            p
        }
        None => match std::env::var("MOSH_PREDICTION_DISPLAY") {
            Ok(p) => {
                predict_check(&argv0, &p, true);
                p
            }
            Err(_) => "adaptive".to_string(),
        },
    };

    family = family.to_lowercase();
    if overwrite {
        std::env::set_var("MOSH_PREDICTION_OVERWRITE", "yes");
    }

    if let Some(port_request) = &port_request {
        check_port_request(&argv0, port_request);
    }

    std::env::remove_var("MOSH_PREDICTION_DISPLAY");

    if positional.is_empty() || ssh.is_empty() {
        die(&usage(&argv0), DIE);
    }
    let userhost = positional.remove(0);
    let command = positional;
    let mut bind_arguments: Vec<String> = Vec::new();
    match bind_ip.as_deref() {
        None => {}
        Some(b) if b.eq_ignore_ascii_case("ssh") => {}
        Some(b) if b.eq_ignore_ascii_case("any") => {}
        // PLAN 9.3: only ssh and any.
        Some(b) => die(&format!("{argv0}: --bind-server={b} is not supported (use ssh or any).\n"), DIE),
    }
    if bind_ip.as_deref().is_none_or(|b| b.eq_ignore_ascii_case("ssh")) {
        if !localhost {
            bind_arguments.push("-s".to_string());
        } else {
            bind_arguments.extend(["-i".to_string(), userhost.clone()]);
        }
    }

    /* Count colors */
    let colors = match Command::new(&client).arg("-c").stderr(Stdio::inherit()).output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).into_owned(),
        Ok(out) => die(&format!("Can't count colors: {client} -c exited with {}\n", out.status), DIE),
        Err(e) => die(&format!("Can't count colors: {e}\n"), DIE),
    };
    let colors = colors.strip_suffix('\n').unwrap_or(&colors);
    let colors = colors.strip_suffix('\n').unwrap_or(colors);
    let colors =
        if !colors.is_empty() && colors.bytes().all(|b| b.is_ascii_digit()) { colors.to_string() } else { "0".to_string() };

    std::env::set_var("MOSH_CLIENT_PID", std::process::id().to_string()); /* We don't support this, but it's useful for test and debug. */

    /* Deviation 9.1: resolve the server address before starting ssh. */
    let mut ip: Option<String> = None;
    if !localhost {
        let hostname = ssh_resolved_hostname(&argv0, &ssh, &userhost);
        ip = Some(resolvename(&argv0, &hostname, &family));
    }

    let mut server_args = vec!["new".to_string(), "-c".to_string(), colors];
    server_args.extend(bind_arguments);
    if let Some(port_request) = &port_request {
        server_args.extend(["-p".to_string(), port_request.clone()]);
    }
    for l in locale_vars() {
        server_args.extend(["-l".to_string(), l]);
    }
    if !command.is_empty() {
        server_args.push("--".to_string());
        server_args.extend(command);
    }
    let server_command = format!("{server} {}", shell_quote(&server_args));

    let mut cmd;
    if localhost {
        cmd = Command::new("sh");
        cmd.arg("-c").arg(&server_command).env_remove("SSH_CONNECTION");
        if let Some(home) = std::env::var_os("HOME") {
            cmd.current_dir(home); /* chdir; # $HOME */
        }
        /* print "MOSH IP ${userhost}\n" in the child; the parent reads it back. */
        ip = Some(userhost.clone());
    } else {
        cmd = Command::new(&ssh[0]);
        cmd.args(&ssh[1..]).arg("-n");
        if ssh_pty {
            cmd.arg("-tt");
        }
        cmd.arg(&userhost).arg("--").arg(&server_command);
    }

    let mut port_key: Option<(String, String)> = None;
    let mut bad_udp_port_warning = false;
    match process::spawn_merged(&mut cmd) {
        Err(e) => {
            // The forked Perl child dies with this message into the pipe.
            println!("Cannot exec {}: {e}", if localhost { server.as_str() } else { "ssh" });
        }
        Ok(mut child) => {
            let stdout = std::io::stdout();
            while let Ok(Some(raw)) = child.read_line() {
                let line = String::from_utf8_lossy(&raw);
                if line.starts_with("MOSH IP ") {
                    if ip.is_some() {
                        die(&format!("{argv0} error: detected attempt to redefine MOSH IP.\n"), DIE);
                    }
                    ip = Some(parse_mosh_ip(&line).unwrap_or_else(|| die(&format!("Bad MOSH IP string: {line}\n"), DIE)));
                } else if line.starts_with("MOSH CONNECT ") {
                    match parse_mosh_connect(&line) {
                        Some(pk) => {
                            port_key = Some(pk);
                            break;
                        }
                        None => die(&format!("Bad MOSH CONNECT string: {line}\n"), DIE),
                    }
                } else {
                    if port_request.as_deref().is_some_and(|p| p.contains(':')) && line.contains("Bad UDP port") {
                        bad_udp_port_warning = true;
                    }
                    let mut out = stdout.lock();
                    let _ = out.write_all(&raw);
                    let _ = out.write_all(b"\n");
                    let _ = out.flush();
                }
            }
            let _ = child.finish();
        }
    }

    let Some(ip) = ip else {
        die(&format!("{argv0}: Did not find remote IP address (is SSH ProxyCommand disabled?).\n"), DIE_AFTER_WAIT);
    };
    let Some((port, key)) = port_key else {
        if bad_udp_port_warning {
            die(&format!("{argv0}: Server does not support UDP port range option.\n"), DIE_AFTER_WAIT);
        }
        die(
            &format!("{argv0}: Did not find mosh server startup message. (Have you installed mosh on your server?)\n"),
            DIE_AFTER_WAIT,
        );
    };

    /* Now start real mosh client */
    let mut client_cmd = Command::new(&client);
    client_cmd.arg(format!("-# {} |", cmdline.join(" "))).arg(&ip).arg(&port);
    client_cmd.env("MOSH_KEY", key).env("MOSH_PREDICTION_DISPLAY", &predict);
    if !term_init {
        client_cmd.env("MOSH_NO_TERM_INIT", "1");
    }
    let err = process::exec(&mut client_cmd);
    die(&format!("{argv0}: Cannot exec {client}: {err}\n"), DIE);
}
