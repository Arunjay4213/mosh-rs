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
// Ported from: upstream/src/frontend/mosh-client.cc

//! The mosh client: raw terminal, event loop, rendering and the prediction
//! overlay. This file is upstream `src/frontend/mosh-client.cc` (argument
//! parsing, `MOSH_KEY`, exit codes); `stmclient` and `terminaloverlay` are
//! the files of the same names.

pub mod stmclient;
pub mod terminaloverlay;

use mosh_platform::{errln, errp, outln};
use std::ffi::OsString;
use std::io::Write;

use mosh_network::Error;
use mosh_platform::coredump::disable_dumping_core;
use mosh_platform::locale::set_native_locale;

use stmclient::STMClient;

/// `PACKAGE_STRING` and `BUILD_VERSION` of this port.
const PACKAGE_STRING: &str = concat!("mosh-rs ", env!("CARGO_PKG_VERSION"));

fn print_version(file: &mut dyn Write) {
    let _ = write!(
        file,
        "mosh-client ({PACKAGE_STRING}) [build {PACKAGE_STRING}]\n\
         Copyright 2012 Keith Winstein <mosh-devel@mit.edu>\n\
         License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
         This is free software: you are free to change and redistribute it.\n\
         There is NO WARRANTY, to the extent permitted by law.\n"
    );
}

fn print_usage(file: &mut dyn Write, argv0: &str) {
    print_version(file);
    let _ = write!(
        file,
        "\nUsage: {argv0} [-# 'ARGS'] IP PORT\n       {argv0} -c\n"
    );
}

/// `mosh-client -c`: the number of colors of the terminal named by TERM.
///
/// Upstream calls `setupterm( (char*)0, 1, (int*)0 )` and prints
/// `tigetnum( "colors" )`; with a null errret, ncurses itself prints the
/// error and exits 1 for an unset TERM, a hardcopy or a generic type. Here
/// the answers come from the generated terminfo table (PLAN 9.6), which
/// reproduces those messages; an unknown TERM gets xterm's count where
/// upstream reports "unknown terminal type".
fn print_colorcount() -> ! {
    /* check colors */
    // getenv( "TERM" ): a value that is not UTF-8 cannot name a table entry.
    let term = std::env::var_os("TERM");
    let term = term.as_ref().map(|t| t.to_str().unwrap_or("\u{fffd}"));
    match mosh_terminal::terminfo::colors(term) {
        Ok(color_val) => {
            outln!("{color_val}");
            std::process::exit(0);
        }
        Err(msg) => {
            errln!("{msg}");
            std::process::exit(1);
        }
    }
}

/// Upstream `getopt( argc, argv, "#:cv" )` with GNU argument permutation:
/// options may follow the operands, `--` ends options. Returns the verbose
/// count and the operands; exits on `-c` and on errors, as upstream.
fn parse_options(argv0: &str, args: &[String]) -> (u32, Vec<String>) {
    let mut verbose = 0;
    let mut operands = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        i += 1;
        if arg == "--" {
            operands.extend(args[i..].iter().cloned());
            break;
        }
        if !arg.starts_with('-') || arg == "-" {
            operands.push(arg.clone());
            continue;
        }
        let chars: Vec<char> = arg.chars().skip(1).collect();
        let mut k = 0;
        while k < chars.len() {
            match chars[k] {
                '#' => {
                    // Ignore the original arguments to mosh wrapper
                    if k + 1 == chars.len() {
                        if i == args.len() {
                            errln!("{argv0}: option requires an argument -- '#'");
                            print_usage(&mut std::io::stderr(), argv0);
                            std::process::exit(1);
                        }
                        i += 1;
                    }
                    break;
                }
                'c' => print_colorcount(),
                'v' => verbose += 1,
                c => {
                    errln!("{argv0}: invalid option -- '{c}'");
                    print_usage(&mut std::io::stderr(), argv0);
                    std::process::exit(1);
                }
            }
            k += 1;
        }
    }
    (verbose, operands)
}

/// Thread name of the panic hook's message: the default hook prints it.
fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        /* leave the alternate screen and raw mode before the message is printed */
        stmclient::restore_terminal_after_panic();
        default_hook(info);
    }));
}

/// `main()` of mosh-client. Returns the exit status.
pub fn main() -> i32 {
    /* For security, make sure we don't dump core */
    disable_dumping_core();

    let argv: Vec<String> = std::env::args_os()
        .map(|a: OsString| a.to_string_lossy().into_owned())
        .collect();
    /* Detect edge case */
    assert!(!argv.is_empty());
    let argv0 = argv[0].clone();

    /* Get arguments */
    for arg in &argv[1..] {
        if arg == "--help" {
            print_usage(&mut std::io::stdout(), &argv0);
            std::process::exit(0);
        }
        if arg == "--version" {
            print_version(&mut std::io::stdout());
            std::process::exit(0);
        }
    }

    let (verbose, operands) = parse_options(&argv0, &argv[1..]);

    if operands.len() != 2 {
        print_usage(&mut std::io::stderr(), &argv0);
        std::process::exit(1);
    }

    let ip = &operands[0];
    let desired_port = &operands[1];

    /* Sanity-check arguments */
    if !desired_port.bytes().all(|b| b.is_ascii_digit()) {
        errp!("{argv0}: Bad UDP port ({desired_port})\n\n");
        print_usage(&mut std::io::stderr(), &argv0);
        std::process::exit(1);
    }

    /* Read key from environment */
    let Some(env_key) = std::env::var_os("MOSH_KEY") else {
        errln!("MOSH_KEY environment variable not found.");
        std::process::exit(1);
    };

    /* Read prediction preference */
    let predict_mode = std::env::var("MOSH_PREDICTION_DISPLAY").ok();
    /* can be NULL */

    /* Read prediction insertion preference */
    let predict_overwrite = std::env::var("MOSH_PREDICTION_OVERWRITE").ok();
    /* can be NULL */

    let key = env_key.to_string_lossy().into_owned();

    std::env::remove_var("MOSH_KEY");

    /* Adopt native locale */
    set_native_locale();

    install_panic_hook();

    let mut client = STMClient::new(
        ip,
        desired_port,
        &key,
        predict_mode.as_deref(),
        verbose,
        predict_overwrite.as_deref(),
    );
    client.init();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| client.main()));
    let success = match result {
        Ok(result) => {
            client.shutdown();
            match result {
                Ok(success) => success,
                Err(Error::Network(e)) => {
                    errp!("Network exception: {e}\r\n");
                    false
                }
                Err(Error::Crypto(e)) => {
                    errp!("Crypto exception: {e}\r\n");
                    false
                }
            }
        }
        // The panic hook restored the terminal and printed the message;
        // drawing a last frame now would land on the user's normal screen.
        Err(_) => false,
    };

    outln!("[mosh is exiting.]");
    let _ = std::io::stdout().flush();

    i32::from(!success)
}
