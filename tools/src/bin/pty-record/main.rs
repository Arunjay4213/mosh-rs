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
// Ported from: (new; test tooling)

//! pty-record: run a command under a pseudoterminal, drive it from a script,
//! and record every byte it writes.
//!
//! pty-record --cols 80 --rows 24 --out FILE --script SCRIPT -- COMMAND ARGS...
//!
//! The child gets TERM=xterm-256color and LANG=C.UTF-8, with COLUMNS and
//! LINES removed, so programs size themselves from the pty. FILE receives the
//! raw output bytes. FILE.meta receives command, size, byte count, duration
//! and exit status.
//!
//! Script lines (blank lines and lines starting with '#' are ignored):
//!   wait MS             keep recording for MS milliseconds
//!   send TEXT           write TEXT to the child; escapes \n \r \t \\ \xHH
//!   resize COLSxROWS    TIOCSWINSZ on the pty, then SIGWINCH to the
//!                       foreground process group
//!   expect TEXT MS      wait until TEXT (escapes allowed) appears in the
//!                       output after the previous expect's match; fail if it
//!                       does not appear within MS milliseconds
//!   end                 send nothing more; wait up to 10 s for the child to
//!                       exit, then SIGHUP it
//! A script that runs out of lines behaves as if it ended with `end`.
//!
//! Unix only: on other targets the binary prints that and exits 1.

#[cfg(unix)]
mod unix;

#[cfg(unix)]
fn main() {
    unix::main();
}

#[cfg(not(unix))]
fn main() {
    eprintln!("pty-record is unix-only");
    std::process::exit(1);
}
