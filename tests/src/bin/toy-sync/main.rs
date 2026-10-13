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
// Ported from: (none; test endpoint whose main loops follow upstream/src/frontend/stmclient.cc and mosh-server.cc)

//! toy-sync: two SSP endpoints synchronizing a toy state, one per
//! process, each running upstream's loop shape: compute `wait_time()`,
//! poll the network fds, freeze the clock, `recv()`, `tick()`.
//!
//!   toy-sync server [PORT]
//!       Binds 127.0.0.1 (PORT or the 60001-60999 range), prints
//!       `MOSH CONNECT <port> <key>`, and runs until stdin is closed.
//!       Then prints one line:
//!       `server packets=<n> acked_ts_advance=<ms> remote_addr=<addr|none> remote=<hex>`
//!       where `remote` is every byte received, in order.
//!   toy-sync client KEY IP PORT CHANGES DURATION_MS SEED
//!       Appends CHANGES random chunks to its state, evenly over
//!       DURATION_MS, then runs until the server has acknowledged all of
//!       it (60 s limit). Prints one line:
//!       `client status=<done|timeout> elapsed_ms=<n> packets=<n> acked_ts_advance=<ms> log=<hex>`
//!
//! Unix only: on other targets the binary prints that and exits 1.

#[cfg(unix)]
mod unix;

#[cfg(unix)]
fn main() -> std::process::ExitCode {
    unix::main()
}

#[cfg(not(unix))]
fn main() {
    eprintln!("toy-sync is unix-only");
    std::process::exit(1);
}
