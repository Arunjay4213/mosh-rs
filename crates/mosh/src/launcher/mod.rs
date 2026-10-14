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

//! The `mosh` launcher, ported from upstream `scripts/mosh.pl`.
//!
//! Like mosh.pl, the launcher runs ssh with itself as the ProxyCommand
//! (`mosh --fake-proxy -- %h %p`). The proxy resolves the host ssh would have
//! connected to, opens the TCP connection to the first address that accepts
//! it, reports that address as `MOSH IP` on stderr and relays the ssh session
//! over it. So mosh-server sees that address in SSH_CONNECTION, binds it, and
//! the client is started with the same address: both always agree, also on
//! hosts with IPv4 and IPv6 addresses.

use mosh_platform::{errln, errp, outp};
use std::ffi::OsString;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::process::{exit, Command, Stdio};

use mosh_platform::locale::LOCALE_VARS;
use mosh_platform::netdb;
use mosh_platform::process;

/// Perl `die` exits with `$!` when it is set, else 255. Before ssh runs that
/// is 255 in practice (checked against /usr/bin/mosh: bad -p gives 255).
const DIE: i32 = 255;
/// After `close $pipe; waitpid $pid, 0` the child is already reaped, so
/// waitpid fails with ECHILD and `$!` is 10. Real mosh 1.4.0 exits with 10
/// for "Did not find mosh server startup message".
const DIE_AFTER_WAIT: i32 = 10;
/// Inside the `while ( <$pipe> )` loop Perl's `$!` is ESPIPE (29), left by
/// its read buffering on the pipe; real mosh exits with 29 for "Bad MOSH
/// CONNECT string" and the other dies in that loop.
const DIE_IN_READ_LOOP: i32 = 29;

fn die(msg: &str, code: i32) -> ! {
    let _ = std::io::stdout().flush();
    errp!("{msg}");
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

        --scrollback         keep the session's output in this terminal's
                                scrollback (needs a mosh-rs server)

        --local              run mosh-server locally without using ssh

        --experimental-remote-ip=(local|remote|proxy)  select the method for
                             discovering the remote IP address to use for mosh
                             (default: "proxy")

        --help               this message
        --version            version and copyright information

Please report bugs to https://github.com/Arunjay4213/mosh-rs/issues.
Mosh home page: https://mosh.org
"#
    )
}

fn version_message() -> String {
    let v = env!("CARGO_PKG_VERSION");
    let build = crate::BUILD_VERSION;
    format!(
        "mosh-rs {v} [build {build}]
Copyright 2012 Keith Winstein <mosh-devel@mit.edu>
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
"
    )
}

fn predict_check(argv0: &str, predict: &str, env_set: bool) {
    if !["adaptive", "always", "never", "experimental"].contains(&predict) {
        let explanation = if env_set {
            " (MOSH_PREDICTION_DISPLAY in environment)"
        } else {
            ""
        };
        errp!("{argv0}: Unknown mode \"{predict}\"{explanation}.\n\n");
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
    args.iter()
        .map(|a| format!("'{}'", a.replace('\'', "'\\''")))
        .collect::<Vec<_>>()
        .join(" ")
}

fn locale_vars() -> Vec<String> {
    LOCALE_VARS
        .iter()
        .filter_map(|name| {
            std::env::var_os(name).map(|v| format!("{name}={}", v.to_string_lossy()))
        })
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
        die(
            &format!("{argv0}: Server-side port or range ({port_request}) is not valid.\n"),
            DIE,
        );
    }
    // Perl compares numerically; f64 behaves like Perl for over-long digit strings.
    let low_n: f64 = low.parse().expect("digits only, checked above");
    if !(0.0..=65535.0).contains(&low_n) {
        die(
            &format!(
                "{argv0}: Server-side (low) port ({low}) must be within valid range [0..65535].\n"
            ),
            DIE,
        );
    }
    if let Some(high) = high {
        let high_n: f64 = high.parse().expect("digits only, checked above");
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

/// Perl's text for an OS error (`$!` in string context): the message
/// without Rust's " (os error N)" suffix.
fn perl_errstr(e: &std::io::Error) -> String {
    let e = e.to_string();
    e.split(" (os error").next().unwrap_or(&e).to_string()
}

/// `sub resolvename` (mosh.pl lines 484-538): first the host as a numeric
/// address, else its canonical name (AI_CANONNAME) resolved again, then
/// the family filter or order. A failed lookup dies with Perl's `$!` (the
/// errno the resolver left) as the exit code, as `die` does.
fn resolvename(argv0: &str, host: &str, port: &str, family: &str) -> Vec<SocketAddr> {
    let gai_die = |msg: String, e: &netdb::GaiError| -> ! {
        die(&msg, if e.errno != 0 { e.errno } else { DIE })
    };
    /* If the user selected a specific family, parse it. */
    let af = match family {
        "inet" => Some(netdb::Family::Inet),
        "inet6" => Some(netdb::Family::Inet6),
        _ => None,
    };

    /* First try the address as a numeric. */
    let mut res = match netdb::getaddrinfo(host, port, af, netdb::Flags::NumericHost) {
        Ok(res) => res.addrs,
        Err(_) => {
            /* Get canonical name for this host. */
            let canon =
                netdb::getaddrinfo(host, port, af, netdb::Flags::CanonName).unwrap_or_else(|e| {
                    gai_die(
                        format!(
                            "{argv0}: could not get canonical name for {host}: {}\n",
                            e.message
                        ),
                        &e,
                    )
                });
            /* Then get final resolution of the canonical name. */
            let canonname = canon.canonname.unwrap_or_else(|| host.to_string());
            match netdb::getaddrinfo(&canonname, port, af, netdb::Flags::None) {
                Ok(res) => res.addrs,
                Err(e) => gai_die(
                    format!(
                        "{argv0}: could not resolve canonical name {canonname} for {host}: {}\n",
                        e.message
                    ),
                    &e,
                ),
            }
        }
    };

    let is_family = |a: &SocketAddr, fam: &str| {
        if fam == "inet" {
            a.is_ipv4()
        } else {
            a.is_ipv6()
        }
    };
    if af.is_some() {
        /* If v4 or v6 was specified, reduce the host list. */
        res.retain(|a| is_family(a, family));
    } else if let Some(prefer) = family.strip_prefix("prefer-") {
        /* If prefer-* was specified, reorder the host list to put that family first. */
        if prefer != "inet" && prefer != "inet6" {
            die(
                &format!("{argv0}: Invalid preferred family {family}\n"),
                DIE,
            );
        }
        res.sort_by_key(|a| !is_family(a, prefer));
    } else if family != "all" {
        /* If v4/v6/all were not specified, verify that this host only has one address family available. */
        if res.iter().any(|a| a.is_ipv4()) && res.iter().any(|a| a.is_ipv6()) {
            die(&format!("{argv0}: host has both IPv4 and IPv6 addresses, use --family to specify behavior\n"), DIE);
        }
    }
    res
}

/// `getnameinfo( $ai->{addr}, NI_NUMERICHOST )`: the numeric address, with
/// the scope of a link-local IPv6 address.
fn numeric_host(addr: &SocketAddr) -> String {
    match addr {
        SocketAddr::V6(a) if a.scope_id() != 0 => format!("{}%{}", a.ip(), a.scope_id()),
        a => a.ip().to_string(),
    }
}

/// `sub cat` of mosh.pl: copy until end of file or an error, retrying a
/// read interrupted by a signal (`next if ( $n == -1 && $! == EINTR )`),
/// flushing every chunk so the ssh protocol is never held back.
fn cat(from: &mut impl Read, to: &mut impl Write) {
    let mut buf = [0u8; 4096];
    loop {
        match from.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if to.write_all(&buf[..n]).is_err() || to.flush().is_err() {
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
}

/// `--fake-proxy -- HOST PORT` (mosh.pl lines 259-311): ssh runs this as its
/// ProxyCommand. Connect to the first address of HOST that accepts a TCP
/// connection on PORT, report it on stderr as `MOSH IP <addr>` (stderr goes
/// into the launcher's pipe; stdout is the ssh protocol stream), then act
/// like netcat between stdin/stdout and the socket.
fn fake_proxy(argv0: &str, family: &str, host: &str, port: &str) -> ! {
    let res = resolvename(argv0, host, port, family);

    /* Now try and connect to something. */
    let mut err = String::new();
    let mut addr_string = String::new();
    let mut sock = None;
    for ai in &res {
        addr_string = numeric_host(ai);
        match TcpStream::connect(ai) {
            Ok(s) => {
                errln!("MOSH IP {addr_string}");
                sock = Some(s);
                break;
            }
            Err(e) => err = perl_errstr(&e),
        }
    }
    let Some(sock) = sock else {
        die(
            &format!("{argv0}: Could not connect to {host}, last tried {addr_string}: {err}\n"),
            DIE,
        );
    };

    /* Act like netcat */
    let socket_to_stdout = match sock.try_clone() {
        Ok(mut from) => std::thread::spawn(move || {
            cat(&mut from, &mut std::io::stdout().lock());
            let _ = from.shutdown(Shutdown::Read);
        }),
        Err(e) => die(&format!("{argv0}: fork: {}\n", perl_errstr(&e)), DIE),
    };
    process::ignore_sighup();
    let mut to = &sock;
    cat(&mut std::io::stdin().lock(), &mut to);
    let _ = sock.shutdown(Shutdown::Write);
    let _ = socket_to_stdout.join();
    exit(0)
}

/// `m{^MOSH CONNECT (\d+?) ([A-Za-z0-9/+]{22})\s*$}`
fn parse_mosh_connect(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("MOSH CONNECT ")?;
    let (port, rest) = rest.split_once(' ')?;
    if port.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let key = rest.trim_end();
    let key_ok = key.len() == 22
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'/' || b == b'+');
    key_ok.then(|| (port.to_string(), key.to_string()))
}

/// `m{^MOSH IP (\S+)\s*$}`
fn parse_mosh_ip(line: &str) -> Option<String> {
    let rest = line.strip_prefix("MOSH IP ")?;
    let ip = rest.trim_end();
    (!ip.is_empty() && !ip.contains(char::is_whitespace)).then(|| ip.to_string())
}

fn os(v: OsString) -> String {
    v.to_string_lossy().into_owned()
}

/// How a mosh.pl option takes its value (the GetOptions spec suffix).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Spec {
    /// No suffix: a flag.
    Flag,
    /// `!`: a flag that also accepts `--noNAME` and `--no-NAME`.
    Negatable,
    /// `=s`: a mandatory string value.
    Value,
}

/// mosh.pl lines 155-174 in order (names and aliases, lowercase), plus
/// `--scrollback` (mosh-rs only, PLAN section 6).
const OPTIONS: &[(&[&str], Spec)] = &[
    (&["client"], Spec::Value),
    (&["server"], Spec::Value),
    (&["predict"], Spec::Value),
    (&["predict-overwrite", "o"], Spec::Negatable),
    (&["port"], Spec::Value),
    (&["a"], Spec::Flag),
    (&["n"], Spec::Flag),
    (&["family"], Spec::Value),
    (&["4"], Spec::Flag),
    (&["6"], Spec::Flag),
    (&["p"], Spec::Value),
    (&["ssh"], Spec::Value),
    (&["ssh-pty"], Spec::Negatable),
    (&["init"], Spec::Negatable),
    (&["local"], Spec::Flag),
    (&["help"], Spec::Flag),
    (&["version"], Spec::Flag),
    (&["fake-proxy"], Spec::Negatable),
    (&["bind-server"], Spec::Value),
    (&["experimental-remote-ip"], Spec::Value),
    (&["scrollback"], Spec::Flag),
];

/// One option from the command line: the first name of its spec, whether
/// it was given as `--noNAME`/`--no-NAME`, and its value (empty for flags).
struct Opt {
    name: &'static str,
    negated: bool,
    value: String,
}

/// `GetOptions` with mosh.pl's configuration, which is Getopt::Long's
/// default: options and arguments may be mixed (permute) until `--`; `-x`
/// and `--x` are the same; long names match case-insensitively and by any
/// unique prefix; no bundling, so `-an` is the option `an`; a mandatory
/// value is `--name=value` or the next argument, whatever it looks like.
/// Prints Getopt::Long's warnings and returns None if any option failed.
fn get_options(args: &[String]) -> Option<(Vec<Opt>, Vec<String>)> {
    let mut table: Vec<(String, usize, bool)> = Vec::new();
    for (i, (names, spec)) in OPTIONS.iter().enumerate() {
        for name in names.iter() {
            table.push((name.to_string(), i, false));
        }
        if *spec == Spec::Negatable {
            for name in names.iter() {
                table.push((format!("no{name}"), i, true));
                table.push((format!("no-{name}"), i, true));
            }
        }
    }

    let mut ok = true;
    let mut opts = Vec::new();
    let mut positional = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        if arg == "--" {
            positional.extend(it.by_ref().cloned());
            break;
        }
        let body = match arg.strip_prefix("--").or_else(|| arg.strip_prefix('-')) {
            Some(body) if arg != "-" => body,
            _ => {
                positional.push(arg.clone());
                continue;
            }
        };
        let (name, inline) = match body.split_once('=') {
            Some((name, value)) => (name, Some(value)),
            None => (body, None),
        };
        let lname = name.to_lowercase();
        if lname.is_empty() {
            errln!("Unknown option: {}", body.to_lowercase());
            ok = false;
            continue;
        }
        let hit = match table.iter().find(|e| e.0 == lname) {
            Some(hit) => hit,
            None => {
                let hits: Vec<&(String, usize, bool)> =
                    table.iter().filter(|e| e.0.starts_with(&lname)).collect();
                let Some(&first) = hits.first() else {
                    errln!("Unknown option: {lname}");
                    ok = false;
                    continue;
                };
                if hits.iter().any(|h| (h.1, h.2) != (first.1, first.2)) {
                    let mut names: Vec<&str> = hits.iter().map(|h| h.0.as_str()).collect();
                    names.sort_unstable();
                    errln!("Option {lname} is ambiguous ({})", names.join(", "));
                    ok = false;
                    continue;
                }
                first
            }
        };
        let (names, spec) = OPTIONS[hit.1];
        let value = if spec == Spec::Value {
            match inline {
                Some(v) if !v.is_empty() => v.to_string(),
                Some(_) => {
                    errln!("Option {} requires an argument", hit.0);
                    ok = false;
                    continue;
                }
                None => match it.next() {
                    Some(v) => v.clone(),
                    None => {
                        errln!("Option {} requires an argument", hit.0);
                        ok = false;
                        continue;
                    }
                },
            }
        } else {
            if inline.is_some() {
                errln!("Option {} does not take an argument", hit.0);
                ok = false;
                continue;
            }
            String::new()
        };
        opts.push(Opt {
            name: names[0],
            negated: hit.2,
            value,
        });
    }
    ok.then_some((opts, positional))
}

/// Run the launcher with the process arguments; never returns.
pub fn main() -> ! {
    let mut args = std::env::args_os();
    let argv0 = args.next().map(os).unwrap_or_else(|| "mosh".to_string());
    let cmdline: Vec<String> = args.map(os).collect();

    let mut client = process::default_client_program();
    let mut server = "mosh-server".to_string();
    let mut predict: Option<String> = None;
    let mut overwrite = false;
    let mut bind_ip: Option<String> = None;
    let mut use_remote_ip = "proxy".to_string();
    let mut family = "prefer-inet".to_string();
    let mut port_request: Option<String> = None;
    let mut ssh = vec!["ssh".to_string()];
    let mut term_init = true;
    let mut localhost = false;
    let mut ssh_pty = true;
    let mut help = false;
    let mut version = false;
    let mut fake_proxy_opt: Option<bool> = None;
    let mut scrollback = false;

    let Some((opts, mut positional)) = get_options(&cmdline) else {
        die(&usage(&argv0), DIE);
    };
    for opt in opts {
        match opt.name {
            "client" => client = opt.value,
            "server" => server = opt.value,
            "predict" => predict = Some(opt.value),
            "predict-overwrite" => overwrite = !opt.negated,
            "port" | "p" => port_request = Some(opt.value),
            "a" => predict = Some("always".to_string()),
            "n" => predict = Some("never".to_string()),
            "family" => family = opt.value,
            "4" => family = "inet".to_string(),
            "6" => family = "inet6".to_string(),
            "ssh" => ssh = shellwords(&opt.value),
            "ssh-pty" => ssh_pty = !opt.negated,
            "init" => term_init = !opt.negated,
            "local" => localhost = true,
            "help" => help = true,
            "version" => version = true,
            /* `\my $fake_proxy`: --nofake-proxy also defines it */
            "fake-proxy" => fake_proxy_opt = Some(!opt.negated),
            "bind-server" => bind_ip = Some(opt.value),
            "experimental-remote-ip" => use_remote_ip = opt.value,
            "scrollback" => scrollback = true,
            other => unreachable!("option {other} is in OPTIONS but not handled"),
        }
    }

    if help {
        outp!("{}", usage(&argv0));
        exit(0);
    }
    if version {
        outp!("{}", version_message());
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

    if !["local", "remote", "proxy"].contains(&use_remote_ip.as_str()) {
        // mosh.pl line 196; Perl's " at SCRIPT line N." suffix is not copied.
        die(&format!("Unknown parameter {use_remote_ip}\n"), DIE);
    }

    family = family.to_lowercase();
    if overwrite {
        std::env::set_var("MOSH_PREDICTION_OVERWRITE", "yes");
    }

    if let Some(port_request) = &port_request {
        check_port_request(&argv0, port_request);
    }

    std::env::remove_var("MOSH_PREDICTION_DISPLAY");

    if fake_proxy_opt.is_some() {
        let [host, port, ..] = positional.as_slice() else {
            die(&usage(&argv0), DIE);
        };
        fake_proxy(&argv0, &family, host, port);
    }

    if positional.is_empty() || ssh.is_empty() {
        die(&usage(&argv0), DIE);
    }
    let mut userhost = positional.remove(0);
    let command = positional;
    let mut bind_arguments: Vec<String> = Vec::new();
    match bind_ip.as_deref() {
        None => {}
        Some(b) if b.eq_ignore_ascii_case("ssh") => {}
        Some(b) if b.eq_ignore_ascii_case("any") => {}
        // PLAN 9.3: only ssh and any.
        Some(b) => die(
            &format!("{argv0}: --bind-server={b} is not supported (use ssh or any).\n"),
            DIE,
        ),
    }
    if bind_ip
        .as_deref()
        .is_none_or(|b| b.eq_ignore_ascii_case("ssh"))
    {
        if !localhost {
            bind_arguments.push("-s".to_string());
        } else {
            bind_arguments.extend(["-i".to_string(), userhost.clone()]);
        }
    }

    /* Count colors */
    let colors = match Command::new(&client)
        .arg("-c")
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).into_owned(),
        /* close COLORCOUNT or die: `die` exits with the child's status */
        Ok(out) => die(
            "Died\n",
            out.status.code().filter(|&c| c != 0).unwrap_or(DIE),
        ),
        Err(e) => {
            errln!("Can't exec \"{client}\": {}", perl_errstr(&e));
            die(
                &format!("Can't count colors: {}\n", perl_errstr(&e)),
                e.raw_os_error().filter(|&c| c != 0).unwrap_or(DIE),
            );
        }
    };
    /* chomp removes one newline; `$` in the check also matches before a final newline */
    let colors = colors.strip_suffix('\n').unwrap_or(&colors);
    let digits = colors.strip_suffix('\n').unwrap_or(colors);
    let colors = if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
        colors.to_string()
    } else {
        "0".to_string()
    };

    std::env::set_var("MOSH_CLIENT_PID", std::process::id().to_string()); /* We don't support this, but it's useful for test and debug. */

    /* If we are using a locally-resolved address, we have to get it before we fork,
    so both parent and child get it. */
    let mut ip: Option<String> = None;
    if use_remote_ip == "local" {
        /* "parse" the host from what the user gave us */
        let (user, host) = match userhost.rfind('@') {
            Some(at) => userhost.split_at(at + 1),
            None => ("", userhost.as_str()),
        };
        /* get list of addresses */
        let res = resolvename(&argv0, host, "22", &family);
        /* Use only the first address as the Mosh IP */
        let Some(hostaddr) = res.first() else {
            // mosh.pl line 343; Perl's " at SCRIPT line N." suffix is not copied.
            die(&format!("could not find address for {host}\n"), DIE);
        };
        let addr_string = numeric_host(hostaddr);
        userhost = format!("{user}{addr_string}");
        ip = Some(addr_string);
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

    /* Lines the child prints before anything ssh prints. */
    let mut first_lines: Vec<Vec<u8>> = Vec::new();
    let mut cmd;
    if localhost {
        cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(&server_command)
            .env_remove("SSH_CONNECTION");
        if let Some(home) = std::env::var_os("HOME") {
            cmd.current_dir(home); /* chdir; # $HOME */
        }
        /* print "MOSH IP ${userhost}\n" */
        first_lines.push(format!("MOSH IP {userhost}").into_bytes());
    } else {
        cmd = Command::new(&ssh[0]);
        cmd.args(&ssh[1..]).arg("-n");
        if ssh_pty {
            cmd.arg("-tt");
        }
        let mut ssh_connection = String::new();
        if use_remote_ip == "remote" {
            /* Ask the server for its IP.  The user's shell may not be
            Posix-compatible so invoke sh explicitly. */
            ssh_connection = format!(
                "sh -c {} ; ",
                shell_quote(&[
                    r#"[ -n "$SSH_CONNECTION" ] && printf "\nMOSH SSH_CONNECTION %s\n" "$SSH_CONNECTION""#
                        .to_string()
                ])
            );
            /* Only with 'remote', we may need to tell SSH which protocol to use. */
            if family == "inet" {
                cmd.arg("-4");
            } else if family == "inet6" {
                cmd.arg("-6");
            }
        }
        if use_remote_ip == "proxy" {
            /* Non-standard shells and broken shrc files cause the ssh
            proxy to break mysteriously. */
            cmd.env("SHELL", "/bin/sh");
            let me =
                std::env::current_exe().map_or(argv0.clone(), |p| p.to_string_lossy().into_owned());
            // `shell_quote( $0, "--family=$family" )`, but quoted for how this
            // system's ssh runs the ProxyCommand (no shell on Windows).
            let quoted_proxy_command = [me, format!("--family={family}")]
                .iter()
                .map(|a| process::proxy_command_quote(a))
                .collect::<Vec<_>>()
                .join(" ");
            cmd.args(["-S", "none", "-o"]).arg(format!(
                "ProxyCommand={quoted_proxy_command} --fake-proxy -- %h %p"
            ));
        }
        cmd.arg(&userhost)
            .arg("--")
            .arg(format!("{ssh_connection}{server_command}"));
    }

    let mut sship: Option<String> = None;
    let mut port_key: Option<(String, String)> = None;
    let mut bad_udp_port_warning = false;
    let mut child = match process::spawn_merged(&mut cmd) {
        Ok(child) => Some(child),
        Err(e) => {
            // The forked Perl child warns (`use warnings`) and dies into the
            // pipe. With --local Perl execs through /bin/sh, which exists.
            let what = if localhost { server.as_str() } else { "ssh" };
            if !localhost {
                first_lines
                    .push(format!("Can't exec \"{}\": {}", ssh[0], perl_errstr(&e)).into_bytes());
            }
            first_lines.push(format!("Cannot exec {what}: {}", perl_errstr(&e)).into_bytes());
            None
        }
    };
    let mut first_lines = first_lines.into_iter();
    let stdout = std::io::stdout();
    loop {
        let raw = match first_lines.next() {
            Some(raw) => raw,
            None => match child.as_mut().map(|c| c.read_line()) {
                Some(Ok(Some(raw))) => raw,
                _ => break,
            },
        };
        let line = String::from_utf8_lossy(&raw);
        if line.starts_with("MOSH IP ") {
            if ip.is_some() {
                die(
                    &format!("{argv0} error: detected attempt to redefine MOSH IP.\n"),
                    DIE_IN_READ_LOOP,
                );
            }
            ip = Some(parse_mosh_ip(&line).unwrap_or_else(|| {
                die(&format!("Bad MOSH IP string: {line}\n"), DIE_IN_READ_LOOP)
            }));
        } else if line.starts_with("MOSH SSH_CONNECTION ") {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() == 6 {
                sship = Some(words[4].to_string());
            } else {
                die(
                    &format!("Bad MOSH SSH_CONNECTION string: {line}\n"),
                    DIE_IN_READ_LOOP,
                );
            }
        } else if line.starts_with("MOSH CONNECT ") {
            match parse_mosh_connect(&line) {
                Some(pk) => {
                    port_key = Some(pk);
                    break;
                }
                None => die(
                    &format!("Bad MOSH CONNECT string: {line}\n"),
                    DIE_IN_READ_LOOP,
                ),
            }
        } else {
            if port_request.as_deref().is_some_and(|p| p.contains(':'))
                && line.contains("Bad UDP port")
            {
                bad_udp_port_warning = true;
            }
            let mut out = stdout.lock();
            let _ = out.write_all(&raw);
            let _ = out.write_all(b"\n");
            let _ = out.flush();
        }
    }
    if let Some(child) = child {
        let _ = child.finish();
    }

    let ip = match (ip, sship) {
        (Some(ip), _) => ip,
        (None, Some(sship)) => {
            errp!("{argv0}: Using remote IP address {sship} from $SSH_CONNECTION for hostname {userhost}\n");
            sship
        }
        (None, None) => die(
            &format!("{argv0}: Did not find remote IP address (is SSH ProxyCommand disabled?).\n"),
            DIE_AFTER_WAIT,
        ),
    };
    let Some((port, key)) = port_key else {
        if bad_udp_port_warning {
            die(
                &format!("{argv0}: Server does not support UDP port range option.\n"),
                DIE_AFTER_WAIT,
            );
        }
        die(
            &format!("{argv0}: Did not find mosh server startup message. (Have you installed mosh on your server?)\n"),
            DIE_AFTER_WAIT,
        );
    };

    /* Now start real mosh client */
    let mut client_cmd = Command::new(&client);
    client_cmd
        .arg(format!("-# {} |", cmdline.join(" ")))
        .arg(&ip)
        .arg(&port);
    client_cmd
        .env("MOSH_KEY", key)
        .env("MOSH_PREDICTION_DISPLAY", &predict);
    if !term_init {
        client_cmd.env("MOSH_NO_TERM_INIT", "1");
    }
    /* mosh-rs scrollback (PLAN.md 6.4): the client asks the server for history */
    if scrollback {
        client_cmd.env("MOSH_SCROLLBACK", "1");
    }
    let err = process::exec(&mut client_cmd);
    die(&format!("{argv0}: Cannot exec {client}: {err}\n"), DIE);
}
