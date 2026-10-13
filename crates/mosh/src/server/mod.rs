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
// Ported from: upstream/src/frontend/mosh-server.cc

//! The mosh server: binds the UDP port, prints `MOSH CONNECT`, runs the user's shell
//! under a pseudoterminal and synchronizes its screen to the client.
//! Ported from upstream `src/frontend/mosh-server.cc`, top to bottom, with
//! the same function names. Unix only, as upstream.
//!
//! Not ported, as the devbox upstream build (`config.h`) has them off or
//! PLAN section 9 item 2 excludes them: syslog (`HAVE_SYSLOG` undefined),
//! utmp records (`HAVE_UTEMPTER` undefined, PLAN 9.2), `pledge`
//! (OpenBSD only). `warn_unattached` (`HAVE_UTMPX_H`, which only reads
//! utmp) is ported.
//!
//! Upstream's `try`/`catch` inside the `serve` loop is `serve_iteration`
//! returning `Result`: an error skips the rest of the iteration, as a
//! throw does.

#![cfg(unix)]

use mosh_platform::{errln, errp, outln, outp};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use mosh_network::crypto::myatoi;
use mosh_network::{freeze_timestamp, timestamp, Connection, Error, State, Transport};
use mosh_platform::coredump::{disable_dumping_core, reenable_dumping_core};
use mosh_platform::locale::{
    clear_locale_variables, get_ctype, is_utf8_locale, locale_charset, set_native_locale,
};
use mosh_platform::pty::{self, Fork};
use mosh_platform::select::{Select, SIGHUP, SIGINT, SIGPIPE, SIGTERM, SIGUSR1};
use mosh_platform::server as sys;
use mosh_platform::socket::strerror;

use crate::statesync::{Complete, UserAction, UserStream};

type ServerConnection = Transport<Complete, UserStream>;

/// Upstream `PACKAGE_STRING`, following the launcher's naming.
const PACKAGE_STRING: &str = concat!("mosh-rs ", env!("CARGO_PKG_VERSION"));
/// Upstream `BUILD_VERSION`.
const BUILD_VERSION: &str = concat!("mosh-rs ", env!("CARGO_PKG_VERSION"));

/// `_PATH_BSHELL`.
const PATH_BSHELL: &str = "/bin/sh";

const STDIN_FILENO: RawFd = 0;
const STDOUT_FILENO: RawFd = 1;
const STDERR_FILENO: RawFd = 2;

/// C `perror( s )`.
fn perror(s: &str, e: &io::Error) {
    errln!("{s}: {}", strerror(e.raw_os_error().unwrap_or(0)));
}

/// glibc's `program_invocation_short_name`, which `err` and `warn` print.
fn progname() -> String {
    let argv0 = std::env::args_os().next().unwrap_or_default();
    let bytes = argv0.as_bytes();
    let base = match bytes.iter().rposition(|&b| b == b'/') {
        Some(i) => &bytes[i + 1..],
        None => bytes,
    };
    String::from_utf8_lossy(base).into_owned()
}

/// C `warn( msg )`: `progname: msg: strerror(errno)`.
fn warn(msg: &str, e: &io::Error) {
    errln!(
        "{}: {msg}: {}",
        progname(),
        strerror(e.raw_os_error().unwrap_or(0))
    );
}

/// C `err( eval, msg )`.
fn err(eval: i32, msg: &str, e: &io::Error) -> ! {
    warn(msg, e);
    std::process::exit(eval);
}

/// C `exit()`: flushes stdout first, as stdio does.
fn exit(code: i32) -> ! {
    let _ = io::stdout().flush();
    std::process::exit(code);
}

fn print_version(file: &mut dyn Write) {
    let _ = write!(
        file,
        "mosh-server ({PACKAGE_STRING}) [build {BUILD_VERSION}]\n\
         Copyright 2012 Keith Winstein <mosh-devel@mit.edu>\n\
         License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
         This is free software: you are free to change and redistribute it.\n\
         There is NO WARRANTY, to the extent permitted by law.\n"
    );
}

fn print_usage(stream: &mut dyn Write, argv0: &str) {
    let _ = writeln!(
        stream,
        "Usage: {argv0} new [-s] [-v] [-i LOCALADDR] [-p PORT[:PORT2]] [-c COLORS] [-l NAME=VALUE] [-- COMMAND...]"
    );
}

/* Simple spinloop */
fn spin() {
    static SPINCOUNT: AtomicU32 = AtomicU32::new(0);
    let spincount = SPINCOUNT.fetch_add(1, Ordering::Relaxed).saturating_add(1);

    if spincount > 10 {
        std::thread::sleep(Duration::from_nanos(100_000_000)); /* 0.1 sec */
        freeze_timestamp();
    }
}

fn get_ssh_ip() -> String {
    let Some(ssh_connection) = std::env::var_os("SSH_CONNECTION") else {
        /* Older sshds don't set this */
        errln!("Warning: SSH_CONNECTION not found; binding to any interface.");
        return String::new();
    };
    let ssh_connection = ssh_connection.to_string_lossy().into_owned();
    let Some(local_interface_ip) = ssh_connection.split_ascii_whitespace().nth(2) else {
        errln!("Warning: Could not parse SSH_CONNECTION; binding to any interface.");
        return String::new();
    };

    /* Strip IPv6 prefix. */
    const IPV6_PREFIX: &str = "::ffff:";

    if local_interface_ip.len() > IPV6_PREFIX.len()
        && local_interface_ip.as_bytes()[..IPV6_PREFIX.len()]
            .eq_ignore_ascii_case(IPV6_PREFIX.as_bytes())
    {
        return local_interface_ip[IPV6_PREFIX.len()..].to_string();
    }

    local_interface_ip.to_string()
}

/// glibc `getopt` (with its default argument permutation) called until it
/// returns -1: `handle` gets each option letter and its argument, or `'?'`
/// right after the error was printed, so messages interleave as upstream's.
fn getopt(args: &[OsString], optstring: &str, mut handle: impl FnMut(char, Option<OsString>)) {
    // getopt is handed argv + 1, so its argv[0], used in the messages, is "new".
    let name = args[0].to_string_lossy().into_owned();
    let posixly_correct = std::env::var_os("POSIXLY_CORRECT").is_some();
    let takes_arg = |c: u8| -> Option<bool> {
        let pos = optstring
            .as_bytes()
            .iter()
            .position(|&o| o == c && c != b':')?;
        Some(optstring.as_bytes().get(pos + 1) == Some(&b':'))
    };
    let mut i = 1;
    while i < args.len() {
        let arg = args[i].as_bytes();
        if arg == b"--" {
            break;
        }
        if arg.len() < 2 || arg[0] != b'-' {
            /* a non-option: permuted past, unless POSIXLY_CORRECT */
            if posixly_correct {
                break;
            }
            i += 1;
            continue;
        }
        i += 1;
        let mut j = 1;
        while j < arg.len() {
            let c = arg[j];
            j += 1;
            match takes_arg(c) {
                None => {
                    errln!("{name}: invalid option -- '{}'", c as char);
                    handle('?', None);
                }
                Some(false) => handle(c as char, None),
                Some(true) => {
                    if j < arg.len() {
                        handle(c as char, Some(OsStr::from_bytes(&arg[j..]).into()));
                    } else if i < args.len() {
                        handle(c as char, Some(args[i].clone()));
                        i += 1;
                    } else {
                        errln!("{name}: option requires an argument -- '{}'", c as char);
                        handle('?', None);
                    }
                    break;
                }
            }
        }
    }
}

/// C `strtol( s, &endptr, 10 )` on LP64: (value, whether `*endptr == '\0'`).
/// Overflow saturates without being reported, as upstream never checks ERANGE.
fn strtol_whole(s: &[u8]) -> (i64, bool) {
    let mut i = 0;
    while i < s.len() && matches!(s[i], b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
        i += 1;
    }
    let mut negative = false;
    if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
        negative = s[i] == b'-';
        i += 1;
    }
    let digits_start = i;
    let mut value: i64 = 0;
    while i < s.len() && s[i].is_ascii_digit() {
        let d = (s[i] - b'0') as i64;
        value = value.saturating_mul(10);
        value = if negative {
            value.saturating_sub(d)
        } else {
            value.saturating_add(d)
        };
        i += 1;
    }
    if i == digits_start {
        /* no conversion: endptr is the start of the string */
        return (0, s.is_empty());
    }
    (value, i == s.len())
}

/// Upstream `main`; returns the exit status.
pub fn main() -> i32 {
    /* For security, make sure we don't dump core */
    disable_dumping_core();

    let argv: Vec<OsString> = std::env::args_os().collect();
    /* Detect edge case */
    assert!(!argv.is_empty());
    let argv0 = argv[0].to_string_lossy().into_owned();
    let mut argc = argv.len();

    let mut desired_ip: Option<String> = None;
    let mut desired_port: Option<String> = None;
    let mut command_argv: Option<Vec<OsString>> = None;
    let mut colors: i32 = 0;
    /* mosh-rs scrollback (PLAN.md 6.2): lines kept for a client that asks */
    let mut scrollback: usize = 10000;
    let mut verbose: u32 = 0; /* don't close stdin/stdout/stderr */
    /* Will cause mosh-server not to correctly detach on old versions of sshd. */
    let mut locale_vars: Vec<OsString> = Vec::new();

    /* strip off command */
    for i in 1..argc {
        if argv[i] == "--help" || argv[i] == "-h" {
            print_usage(&mut io::stdout(), &argv0);
            exit(0);
        }
        if argv[i] == "--version" {
            print_version(&mut io::stdout());
            exit(0);
        }
        if argv[i] == "--" {
            /* -- is mandatory */
            if i != argc - 1 {
                command_argv = Some(argv[i + 1..].to_vec());
            }
            argc = i; /* rest of options before -- */
            break;
        }
    }

    /* Parse new command-line syntax */
    if argc >= 2 && argv[1] == "new" {
        /* mosh-rs scrollback (PLAN.md 6.2): `--scrollback N` or
        `--scrollback=N`, taken out before upstream's getopt sees the rest. */
        let mut new_args: Vec<OsString> = Vec::new();
        let mut i = 1;
        while i < argc {
            let value = if argv[i] == "--scrollback" {
                i += 1;
                Some(if i < argc {
                    argv[i].clone()
                } else {
                    OsString::new()
                })
            } else {
                argv[i]
                    .as_bytes()
                    .strip_prefix(b"--scrollback=")
                    .map(|v| OsStr::from_bytes(v).to_os_string())
            };
            match value {
                None => new_args.push(argv[i].clone()),
                Some(v) => {
                    let v = v.to_string_lossy().into_owned();
                    match myatoi(&v) {
                        Ok(n) if n >= 0 && !v.is_empty() => scrollback = n as usize,
                        _ => {
                            errln!("{argv0}: Bad scrollback size ({v})");
                            print_usage(&mut io::stderr(), &argv0);
                            exit(1);
                        }
                    }
                }
            }
            i += 1;
        }
        /* new option syntax */
        getopt(&new_args, "@:i:p:c:svl:", |opt, optarg| {
            let optarg = optarg.map(|a| a.to_string_lossy().into_owned());
            match opt {
                /*
                 * This undocumented option does nothing but eat its argument.
                 * Useful in scripting where you prepend something to a
                 * mosh-server argv, and might end up with something like
                 * "mosh-server new -v new -c 256", now you can say
                 * "mosh-server new -v -@ new -c 256" to discard the second
                 * "new".
                 */
                '@' => {}
                'i' => desired_ip = optarg,
                'p' => desired_port = optarg,
                's' => {
                    desired_ip = None;
                    let desired_ip_str = get_ssh_ip();
                    if !desired_ip_str.is_empty() {
                        desired_ip = Some(desired_ip_str);
                    }
                }
                'c' => {
                    let optarg = optarg.unwrap_or_default();
                    match myatoi(&optarg) {
                        // long to int, as upstream's `colors = myatoi( optarg )`.
                        Ok(n) => colors = n as i32,
                        Err(_) => {
                            errln!("{argv0}: Bad number of colors ({optarg})");
                            print_usage(&mut io::stderr(), &argv0);
                            exit(1);
                        }
                    }
                }
                'v' => verbose += 1,
                'l' => locale_vars.push(optarg.unwrap_or_default().into()),
                _ => {
                    /* don't die on unknown options */
                    print_usage(&mut io::stderr(), &argv0);
                }
            }
        });
    } else if argc == 1 {
        /* legacy argument parsing for older client wrapper script */
        /* do nothing */
    } else if argc == 2 {
        desired_ip = Some(argv[1].to_string_lossy().into_owned());
    } else if argc == 3 {
        desired_ip = Some(argv[1].to_string_lossy().into_owned());
        desired_port = Some(argv[2].to_string_lossy().into_owned());
    } else {
        print_usage(&mut io::stderr(), &argv0);
        exit(1);
    }

    /* Sanity-check arguments */
    let (mut dpl, mut dph) = (0, 0);
    if let Some(port) = &desired_port {
        if !Connection::parse_portrange(port, &mut dpl, &mut dph) {
            errln!("{argv0}: Bad UDP port range ({port})");
            print_usage(&mut io::stderr(), &argv0);
            exit(1);
        }
    }

    let mut with_motd = false;

    /* Get shell */
    let mut command_path = OsString::new();
    let command_argv = match command_argv {
        Some(c) => c,
        None => {
            /* get shell name */
            let shell = match std::env::var_os("SHELL") {
                Some(shell) => shell,
                None => match sys::getpwuid() {
                    Ok(pw) => pw.shell.into(),
                    Err(e) => {
                        perror("getpwuid", &e);
                        exit(1);
                    }
                },
            };

            let mut shell_path = shell;
            if shell_path.is_empty() {
                /* empty shell means Bourne shell */
                shell_path = PATH_BSHELL.into();
            }

            command_path = shell_path.clone();

            let bytes = shell_path.as_bytes();
            let shell_name = match bytes.iter().rposition(|&b| b == b'/') {
                None => bytes,
                Some(shell_slash) => &bytes[shell_slash + 1..],
            };

            /* prepend '-' to make login shell */
            let mut login_name = OsString::from("-");
            login_name.push(OsStr::from_bytes(shell_name));

            with_motd = true;
            vec![login_name]
        }
    };

    if command_path.is_empty() {
        command_path = command_argv[0].clone();
    }

    /* Adopt implementation locale */
    set_native_locale();
    if !is_utf8_locale() {
        /* save details for diagnostic */
        let native_ctype = get_ctype();
        let native_charset = locale_charset();

        /* apply locale-related environment variables from client */
        clear_locale_variables();
        for i in &locale_vars {
            /* putenv( "NAME=VALUE" ); glibc removes NAME when there is no '=' */
            let bytes = i.as_bytes();
            match bytes.iter().position(|&b| b == b'=') {
                Some(eq) => std::env::set_var(
                    OsStr::from_bytes(&bytes[..eq]),
                    OsStr::from_bytes(&bytes[eq + 1..]),
                ),
                None => std::env::remove_var(i),
            }
        }

        /* check again */
        set_native_locale();
        if !is_utf8_locale() {
            let client_ctype = get_ctype();
            let client_charset = locale_charset();

            errp!(
                "mosh-server needs a UTF-8 native locale to run.\n\n\
                 Unfortunately, the local environment ({}) specifies\n\
                 the character set \"{}\",\n\n\
                 The client-supplied environment ({}) specifies\n\
                 the character set \"{}\".\n\n",
                native_ctype.str(),
                native_charset,
                client_ctype.str(),
                client_charset
            );
            let _ = std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg("locale")
                .status();
            exit(1);
        }
    }

    match run_server(
        desired_ip.as_deref(),
        desired_port.as_deref(),
        &command_path,
        &command_argv,
        colors,
        verbose,
        with_motd,
        scrollback,
    ) {
        Ok(code) => code,
        Err(Error::Network(e)) => {
            errln!("Network exception: {e}");
            1
        }
        Err(Error::Crypto(e)) => {
            errln!("Crypto exception: {e}");
            1
        }
    }
}

/// `getenv` plus `strtol` for the two timeout variables, with upstream's
/// messages. As upstream, an invalid value is reported but whatever strtol
/// parsed is kept; only a negative value is reset to 0.
fn get_timeout_env(name: &str) -> i64 {
    let mut timeout = 0;
    if let Some(envar) = std::env::var_os(name) {
        if !envar.is_empty() {
            let (value, whole) = strtol_whole(envar.as_bytes());
            timeout = value;
            if !whole {
                errln!("{name} not a valid integer, ignoring");
            } else if timeout < 0 {
                errln!("{name} is negative, ignoring");
                timeout = 0;
            }
        }
    }
    timeout
}

// Upstream's seven parameters plus the mosh-rs scrollback limit.
#[allow(clippy::too_many_arguments)]
fn run_server(
    desired_ip: Option<&str>,
    desired_port: Option<&str>,
    command_path: &OsStr,
    command_argv: &[OsString],
    colors: i32,
    verbose: u32,
    with_motd: bool,
    scrollback: usize,
) -> Result<i32, Error> {
    /* get network idle timeout */
    let network_timeout = get_timeout_env("MOSH_SERVER_NETWORK_TMOUT");
    /* get network signaled idle timeout */
    let network_signaled_timeout = get_timeout_env("MOSH_SERVER_SIGNAL_TMOUT");
    /* get initial window size */
    let window_size = match pty::get_winsize(STDIN_FILENO) {
        Ok(ws) if ws.ws_col != 0 && ws.ws_row != 0 => ws,
        _ => {
            /* Fill in sensible defaults. */
            /* They will be overwritten by client on first connection. */
            libc::winsize {
                ws_row: 24,
                ws_col: 80,
                ws_xpixel: 0,
                ws_ypixel: 0,
            }
        }
    };

    /* open parser and terminal */
    let mut terminal = Complete::new(window_size.ws_col as usize, window_size.ws_row as usize);
    /* mosh-rs scrollback: used only if the client's capabilities ask for it */
    terminal.set_scrollback_limit(scrollback);

    /* open network */
    let blank = UserStream::new();
    let mut network = ServerConnection::new_server(&terminal, &blank, desired_ip, desired_port)?;

    network.set_verbose(verbose);
    Select::set_verbose(verbose);

    /*
     * If mosh-server is run on a pty, then typeahead may echo and break mosh.pl's
     * detection of the MOSH CONNECT message.  Print it on a new line to bodge
     * around that.
     */
    if sys::isatty(STDIN_FILENO) {
        outp!("\r\n\n"); /* puts( "\r\n" ) */
    }
    outln!("MOSH CONNECT {} {}", network.port()?, network.get_key());

    /* don't let signals kill us */
    sys::set_signal_disposition(SIGHUP, true).expect("fatal_assert: sigaction( SIGHUP )");
    sys::set_signal_disposition(SIGPIPE, true).expect("fatal_assert: sigaction( SIGPIPE )");

    /* detach from terminal */
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    match sys::fork() {
        Err(e) => perror("fork", &e),
        Ok(0) => {}
        Ok(the_pid) => {
            errp!(
                "\nmosh-server ({PACKAGE_STRING}) [build {BUILD_VERSION}]\n\
                 Copyright 2012 Keith Winstein <mosh-devel@mit.edu>\n\
                 License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
                 This is free software: you are free to change and redistribute it.\n\
                 There is NO WARRANTY, to the extent permitted by law.\n\n"
            );

            errln!("[mosh-server detached, pid = {the_pid}]");

            let _ = io::stdout().flush();
            if sys::isatty(STDOUT_FILENO) {
                sys::tcdrain(STDOUT_FILENO);
            }
            if sys::isatty(STDERR_FILENO) {
                sys::tcdrain(STDERR_FILENO);
            }
            exit(0);
        }
    }

    /* close file descriptors */
    if verbose == 0 {
        /* Necessary to properly detach on old versions of sshd (e.g. RHEL/CentOS 5.0). */
        if let Err((call, e)) = sys::redirect_stdio_to_dev_null() {
            perror(call, &e);
            exit(1);
        }
    }

    let utmp_entry = format!("mosh [{}]", std::process::id());

    /* Fork child process */
    let (pipe_read, pipe_write) = match sys::pipe() {
        Ok(p) => p,
        Err(e) => {
            perror("pipe", &e);
            exit(1);
        }
    };
    let fork = match pty::forkpty_winsize(&window_size) {
        Ok(f) => f,
        Err(e) => {
            perror("forkpty", &e);
            exit(1);
        }
    };

    let master = match fork {
        Fork::Child => {
            /* child */
            if let Err(e) = sys::close(pipe_write) {
                perror("child write pipe close", &e);
                exit(1);
            }

            /* reenable signals */
            sys::set_signal_disposition(SIGHUP, false).expect("fatal_assert: sigaction( SIGHUP )");
            sys::set_signal_disposition(SIGPIPE, false)
                .expect("fatal_assert: sigaction( SIGPIPE )");

            /* close server-related file descriptors */
            drop(network);

            /* set IUTF8 if available */
            if let Err((call, e)) = sys::set_iutf8_stdin() {
                perror(call, &e);
                exit(1);
            }

            /* set TERM */
            let default_term = "xterm";
            let color_term = "xterm-256color";

            std::env::set_var(
                "TERM",
                if colors == 256 {
                    color_term
                } else {
                    default_term
                },
            );

            /* ask ncurses to send UTF-8 instead of ISO 2022 for line-drawing chars */
            std::env::set_var("NCURSES_NO_UTF8_ACS", "1");

            /* clear STY environment variable so GNU screen regards us as top level */
            std::env::remove_var("STY");

            chdir_homedir();

            if with_motd && !motd_hushed() {
                // For Ubuntu, try and print one of {,/var}/run/motd.dynamic.
                // This file is only updated when pam_motd is run, but when
                // mosh-server is run in the usual way with ssh via the script,
                // this always happens.
                // XXX Hackish knowledge of Ubuntu PAM configuration.
                // But this seems less awful than build-time detection with autoconf.
                if !print_motd("/run/motd.dynamic") {
                    print_motd("/var/run/motd.dynamic");
                }
                // Always print traditional /etc/motd.
                print_motd("/etc/motd");
                warn_unattached(&utmp_entry);
            }

            /* Wait for parent to release us. */
            let mut linebuf = [0u8; 81];
            // -1, errno == EINTR -- retry
            // otherwise, give up
            loop {
                match sys::read(&pipe_read, &mut linebuf) {
                    Ok(_) => break,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => err(1, "parent signal", &e),
                }
            }

            reenable_dumping_core();

            // stdio is flushed by exec in C only for line-buffered output
            // already written out; flush what Rust buffered (motd) here.
            let _ = io::stdout().flush();
            let e = pty::execvp(command_path, command_argv);
            warn(&format!("execvp: {}", command_path.to_string_lossy()), &e);
            std::thread::sleep(Duration::from_secs(3));
            exit(1);
        }
        Fork::Parent { master, .. } => master,
    };

    /* parent */
    if let Err(e) = sys::close(pipe_read) {
        perror("parent read pipe close", &e);
        exit(1);
    }

    match serve(
        &master,
        pipe_write,
        &mut terminal,
        &mut network,
        network_timeout,
        network_signaled_timeout,
    ) {
        Ok(()) => {}
        Err(Error::Network(e)) => errln!("Network exception: {e}"),
        Err(Error::Crypto(e)) => errln!("Crypto exception: {e}"),
    }

    if let Err(e) = sys::close(master) {
        perror("close", &e);
        exit(1);
    }

    outp!("\n[mosh-server is exiting.]\n");
    let _ = io::stdout().flush();

    Ok(0)
}

/// swrite( fd, str, len ): write everything or fail; `perror( "write" )` on
/// failure. A short or interrupted write is not retried.
fn swrite(fd: &OwnedFd, s: &[u8]) -> Result<(), ()> {
    let mut total_bytes_written = 0;
    while total_bytes_written < s.len() {
        match sys::write(fd, &s[total_bytes_written..]) {
            Ok(n) if n > 0 => total_bytes_written += n,
            Ok(_) => {
                perror("write", &io::Error::from_raw_os_error(0));
                return Err(());
            }
            Err(e) => {
                perror("write", &e);
                return Err(());
            }
        }
    }
    Ok(())
}

/// The variables of `serve` that live across loop iterations.
struct Serve<'a> {
    host_fd: &'a OwnedFd,
    pipe_fd: Option<OwnedFd>,
    terminal: &'a mut Complete,
    network: &'a mut ServerConnection,
    network_timeout_ms: u64,
    network_signaled_timeout_ms: u64,
    sel: Select,
    last_remote_num: u64,
    child_released: bool,
}

/// What one loop iteration asks for.
enum Next {
    Continue,
    Break,
}

fn serve(
    host_fd: &OwnedFd,
    pipe_fd: OwnedFd,
    terminal: &mut Complete,
    network: &mut ServerConnection,
    network_timeout: i64,
    network_signaled_timeout: i64,
) -> Result<(), Error> {
    /* scale timeouts */
    let network_timeout_ms = (network_timeout as u64).wrapping_mul(1000);
    let network_signaled_timeout_ms = (network_signaled_timeout as u64).wrapping_mul(1000);
    /* prepare to poll for events */
    let mut sel = Select::new();
    sel.add_signal(SIGTERM);
    sel.add_signal(SIGINT);
    sel.add_signal(SIGUSR1);

    let last_remote_num = network.get_remote_state_num();

    let mut s = Serve {
        host_fd,
        pipe_fd: Some(pipe_fd),
        terminal,
        network,
        network_timeout_ms,
        network_signaled_timeout_ms,
        sel,
        last_remote_num,
        child_released: false,
    };

    loop {
        match s.serve_iteration() {
            Ok(Next::Continue) => {}
            Ok(Next::Break) => break,
            Err(Error::Network(e)) => {
                errln!("{e}");
                spin();
            }
            Err(Error::Crypto(e)) => {
                if e.fatal {
                    return Err(Error::Crypto(e));
                } else {
                    errln!("Crypto exception: {e}");
                }
            }
        }
    }
    Ok(())
}

impl Serve<'_> {
    /// The body of upstream's `while ( true ) { try { ... } }`.
    fn serve_iteration(&mut self) -> Result<Next, Error> {
        const TIMEOUT_IF_NO_CLIENT: u64 = 60000;
        let host_fd = self.host_fd;
        let network = &mut *self.network;
        let terminal = &mut *self.terminal;
        let sel = &mut self.sel;

        let mut timeout = i32::MAX;
        let mut now = timestamp();

        timeout = timeout.min(network.wait_time());
        timeout = timeout.min(terminal.wait_time(now));
        if network.get_remote_state_num() == 0 || network.shutdown_in_progress() {
            timeout = timeout.min(5000);
        }
        /*
         * The server goes completely asleep if it has no remote peer.
         * We may want to wake up sooner.
         */
        if self.network_timeout_ms != 0 {
            let mut network_sleep = self
                .network_timeout_ms
                .wrapping_sub(now.wrapping_sub(network.get_latest_remote_state().timestamp))
                as i64;
            if network_sleep < 0 {
                network_sleep = 0;
            } else if network_sleep > i32::MAX as i64 {
                /* 24 days might be too soon.  That's OK. */
                network_sleep = i32::MAX as i64;
            }
            timeout = timeout.min(network_sleep as i32);
        }

        /* poll for events */
        sel.clear_fds();
        let fd_list = network.fds();
        assert!(fd_list.len() == 1); /* servers don't hop */
        let network_fd = *fd_list.last().expect("one fd");
        sel.add_fd(network_fd);
        if !network.shutdown_in_progress() {
            sel.add_fd(host_fd.as_raw_fd());
        }

        let active_fds = sel.select(timeout);
        freeze_timestamp(); /* inside upstream's Select::select */
        if let Err(e) = active_fds {
            perror("select", &e);
            return Ok(Next::Break);
        }

        now = timestamp();
        let time_since_remote_state = now.wrapping_sub(network.get_latest_remote_state().timestamp);
        let mut terminal_to_host: Vec<u8> = Vec::new();

        if sel.read(network_fd) {
            /* packet received from the network */
            network.recv()?;

            /* is new user input available for the terminal? */
            if network.get_remote_state_num() != self.last_remote_num {
                self.last_remote_num = network.get_remote_state_num();

                let mut us = UserStream::new();
                us.apply_string(&network.get_remote_diff())?;
                /* apply userstream to terminal */
                for i in 0..us.size() {
                    let action = us.get_action(i);
                    if let UserAction::Resize(res) = action {
                        /* apply only the last consecutive Resize action */
                        if i < us.size() - 1 {
                            if let UserAction::Resize(_) = us.get_action(i + 1) {
                                continue;
                            }
                        }
                        /* tell child process of resize */
                        let mut window_size = match pty::get_winsize(host_fd.as_raw_fd()) {
                            Ok(ws) => ws,
                            Err(e) => {
                                perror("ioctl TIOCGWINSZ", &e);
                                network.start_shutdown();
                                libc::winsize {
                                    ws_row: 0,
                                    ws_col: 0,
                                    ws_xpixel: 0,
                                    ws_ypixel: 0,
                                }
                            }
                        };
                        // size_t to unsigned short, as upstream's assignment.
                        window_size.ws_col = res.width as u16;
                        window_size.ws_row = res.height as u16;
                        if let Err(e) = pty::set_winsize(host_fd.as_raw_fd(), &window_size) {
                            perror("ioctl TIOCSWINSZ", &e);
                            network.start_shutdown();
                        }
                    }
                    terminal_to_host.extend(terminal.act_action(action));
                }

                if !us.empty() {
                    /* register input frame number for future echo ack */
                    terminal.register_input_frame(self.last_remote_num, now);
                }

                /* update client with new state of terminal */
                if !network.shutdown_in_progress() {
                    network.set_current_state(terminal);
                }

                /* Tell child to start login session. */
                if !self.child_released {
                    if let Some(pipe_fd) = self.pipe_fd.take() {
                        if let Err(e) = sys::close(pipe_fd) {
                            err(1, "child release", &e);
                        }
                    }
                    self.child_released = true;
                }
            }
        }

        if !network.shutdown_in_progress() && sel.read(host_fd.as_raw_fd()) {
            /* input from the host needs to be fed to the terminal */
            const BUF_SIZE: usize = 16384;
            let mut buf = [0u8; BUF_SIZE];

            /* fill buffer if possible */
            let bytes_read = sys::read(host_fd, &mut buf);

            /* If the pty slave is closed, reading from the master can fail with
            EIO (see #264).  So we treat errors on read() like EOF. */
            match bytes_read {
                Ok(0) | Err(_) => network.start_shutdown(),
                Ok(n) => {
                    terminal_to_host.extend(terminal.act(&buf[..n]));

                    /* update client with new state of terminal */
                    network.set_current_state(terminal);
                }
            }
        }

        /* write user input and terminal writeback to the host */
        if swrite(host_fd, &terminal_to_host).is_err() {
            network.start_shutdown();
        }

        let mut idle_shutdown = false;
        if self.network_timeout_ms != 0 && self.network_timeout_ms <= time_since_remote_state {
            idle_shutdown = true;
            errln!(
                "Network idle for {} seconds.",
                time_since_remote_state / 1000
            );
        }
        if sel.signal(SIGUSR1)
            && (self.network_signaled_timeout_ms == 0
                || self.network_signaled_timeout_ms <= time_since_remote_state)
        {
            idle_shutdown = true;
            errln!(
                "Network idle for {} seconds when SIGUSR1 received",
                time_since_remote_state / 1000
            );
        }

        if sel.any_signal() || idle_shutdown {
            /* shutdown signal */
            if network.has_remote_addr() && !network.shutdown_in_progress() {
                network.start_shutdown();
            } else {
                return Ok(Next::Break);
            }
        }

        /* quit if our shutdown has been acknowledged */
        if network.shutdown_in_progress() && network.shutdown_acknowledged() {
            return Ok(Next::Break);
        }

        /* quit after shutdown acknowledgement timeout */
        if network.shutdown_in_progress() && network.shutdown_ack_timed_out() {
            return Ok(Next::Break);
        }

        /* quit if we received and acknowledged a shutdown request */
        if network.counterparty_shutdown_ack_sent() {
            return Ok(Next::Break);
        }

        if terminal.set_echo_ack(now) && !network.shutdown_in_progress() {
            /* update client with new echo ack */
            network.set_current_state(terminal);
        }

        if network.get_remote_state_num() == 0 && time_since_remote_state >= TIMEOUT_IF_NO_CLIENT {
            errln!(
                "No connection within {} seconds.",
                TIMEOUT_IF_NO_CLIENT / 1000
            );
            return Ok(Next::Break);
        }

        network.tick()?;
        Ok(Next::Continue)
    }
}

/* Print the motd from a given file, if available */
fn print_motd(filename: &str) -> bool {
    let Ok(mut motd) = File::open(filename) else {
        return false;
    };

    const BUFSIZE: usize = 256;

    let mut buffer = [0u8; BUFSIZE];
    let mut stdout = io::stdout();
    loop {
        let bytes_read: usize = motd.read(&mut buffer).unwrap_or_default();
        if bytes_read == 0 {
            break; /* don't report error */
        }
        if stdout.write_all(&buffer[..bytes_read]).is_err() {
            break;
        }
    }

    true
}

fn chdir_homedir() {
    let home: OsString = match std::env::var_os("HOME") {
        Some(home) => home,
        None => match sys::getpwuid() {
            Ok(pw) => pw.dir.into(),
            Err(e) => {
                perror("getpwuid", &e);
                return; /* non-fatal */
            }
        },
    };

    if let Err(e) = std::env::set_current_dir(&home) {
        perror("chdir", &e);
    }

    std::env::set_var("PWD", &home);
}

fn motd_hushed() -> bool {
    /* must be in home directory already */
    std::fs::symlink_metadata(".hushlogin").is_ok()
}

fn device_exists(ut_line: &str) -> bool {
    let device_name = format!("/dev/{ut_line}");
    std::fs::symlink_metadata(device_name).is_ok()
}

fn warn_unattached(ignore_entry: &str) {
    /* get username */
    let pw = match sys::getpwuid() {
        Ok(pw) => pw,
        Err(e) => {
            perror("getpwuid", &e);
            /* non-fatal */
            return;
        }
    };

    let username = pw.name;

    /* look for unattached sessions */
    let mut unattached_mosh_servers: Vec<String> = Vec::new();

    for entry in sys::getutxent_all() {
        if entry.user_process && username == entry.user {
            /* does line show unattached mosh session */
            let text = entry.host;
            if text.len() >= 5
                && text.starts_with("mosh ")
                && text.ends_with(']')
                && text != ignore_entry
                && device_exists(&entry.line)
            {
                unattached_mosh_servers.push(text);
            }
        }
    }

    /* print out warning if necessary */
    if unattached_mosh_servers.is_empty() {
    } else if unattached_mosh_servers.len() == 1 {
        outp!(
            "\x1b[37;44mMosh: You have a detached Mosh session on this server ({}).\x1b[m\n\n",
            unattached_mosh_servers[0]
        );
    } else {
        let mut pid_string = String::new();

        for it in &unattached_mosh_servers {
            pid_string += &format!("        - {it}\n");
        }

        outp!(
            "\x1b[37;44mMosh: You have {} detached Mosh sessions on this server, with PIDs:\n{}\x1b[m\n",
            unattached_mosh_servers.len(),
            pid_string
        );
    }
}
