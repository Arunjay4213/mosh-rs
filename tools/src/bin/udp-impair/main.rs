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

//! udp-impair: a UDP proxy for one mosh client/server pair that drops,
//! delays, jitters and reorders packets, and can change its source port.
//!
//! udp-impair --listen ADDR --target ADDR --control ADDR
//!            [--loss P] [--delay-ms MS] [--jitter-ms MS] [--reorder P] [--seed N]
//!
//! Client packets arrive on --listen. They are forwarded to --target from an
//! "upstream" socket bound to an ephemeral port. Replies from the target go
//! back to the most recent client address. Both directions get the same
//! impairment: each packet is dropped with probability `loss`, otherwise it is
//! delivered at now + delay + uniform(0, jitter). With probability `reorder`
//! a packet is held back and delivered right after the next packet of the
//! same direction (or after HOLD_TIMEOUT if no next packet comes).
//!
//! Control datagrams on --control (each gets a one-line reply):
//!   rebind        new upstream socket, so the target sees a new source port
//!   blackout MS   drop everything, both directions, for MS milliseconds
//!   loss P        set loss probability
//!   delay MS      set base delay
//!   stats         counters per direction
//!
//! Any of the ports may be 0. The bound addresses are printed as one line on
//! stdout at startup. On SIGINT or SIGTERM the stats go to stderr and the
//! process exits 0.
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
    eprintln!("udp-impair is unix-only");
    std::process::exit(1);
}
