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
// Ported from: (none; C stdio's error handling, which every upstream printf and fprintf relies on)

//! Writes to stdout and stderr that ignore errors, as C's `printf` and
//! `fprintf` do. Rust's `print!` and `eprint!` family panics when the
//! write fails (stderr closed or full, or a pipe whose reader has gone,
//! for example after ssh exits), which upstream never does.
//!
//! The macros `outp!`, `outln!`, `errp!` and `errln!` (exported at the
//! crate root) replace `print!`, `println!`, `eprint!` and `eprintln!`.

use std::fmt;
use std::io::{self, Write};

/// Writes `args` to stdout, ignoring any error.
pub fn write_stdout(args: fmt::Arguments<'_>) {
    let _ = io::stdout().write_fmt(args);
}

/// Writes `args` to stderr, ignoring any error.
pub fn write_stderr(args: fmt::Arguments<'_>) {
    let _ = io::stderr().write_fmt(args);
}

/// `print!` that ignores write errors (C `printf`).
#[macro_export]
macro_rules! outp {
    ($($arg:tt)*) => {
        $crate::stdio::write_stdout(format_args!($($arg)*))
    };
}

/// `println!` that ignores write errors.
#[macro_export]
macro_rules! outln {
    () => {
        $crate::stdio::write_stdout(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::stdio::write_stdout(format_args!("{}\n", format_args!($($arg)*)))
    };
}

/// `eprint!` that ignores write errors (C `fprintf( stderr, ... )`).
#[macro_export]
macro_rules! errp {
    ($($arg:tt)*) => {
        $crate::stdio::write_stderr(format_args!($($arg)*))
    };
}

/// `eprintln!` that ignores write errors.
#[macro_export]
macro_rules! errln {
    () => {
        $crate::stdio::write_stderr(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::stdio::write_stderr(format_args!("{}\n", format_args!($($arg)*)))
    };
}

/// C `perror( s )`: `s: <strerror text>` on stderr, the system message
/// without Rust's " (os error N)" suffix (`socket::strerror`), as upstream
/// prints it. An error without an OS code prints its own text.
pub fn perror(s: &str, e: &io::Error) {
    match e.raw_os_error() {
        Some(errno) => write_stderr(format_args!("{s}: {}\n", crate::socket::strerror(errno))),
        None => write_stderr(format_args!("{s}: {e}\n")),
    }
}
