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

use std::io::Write;
use std::os::fd::RawFd;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use mosh_network::{freeze_timestamp, Error, State, Transport};
use rand::rngs::StdRng;
use rand::{Rng, RngExt, SeedableRng};

/// Toy state: an append-only byte log.
#[derive(Clone, PartialEq, Default, Debug)]
struct Log {
    bytes: Vec<u8>,
}

impl State for Log {
    /// bytes appended since `existing`
    fn diff_from(&self, existing: &Log) -> Vec<u8> {
        assert!(
            self.bytes.starts_with(&existing.bytes),
            "diff_from: existing is not a prefix"
        );
        self.bytes[existing.bytes.len()..].to_vec()
    }

    fn init_diff(&self) -> Vec<u8> {
        self.diff_from(&Log::default())
    }

    fn apply_string(&mut self, diff: &[u8]) {
        self.bytes.extend_from_slice(diff);
    }

    /// drop the prefix (asserted to be one, as UserStream::subtract does)
    fn subtract(&mut self, prefix: &Log) {
        assert!(
            self.bytes.starts_with(&prefix.bytes),
            "subtract: not a prefix"
        );
        self.bytes.drain(..prefix.bytes.len());
    }

    fn subtract_self(&mut self) {
        self.bytes.clear();
    }

    fn reset_input(&mut self) {}
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// poll(2) on the network fds plus `extra` (if any). Returns (network readable, extra readable).
fn poll_fds(net: &[RawFd], extra: Option<RawFd>, timeout_ms: i32) -> (bool, bool) {
    let mut pfds: Vec<libc::pollfd> = net
        .iter()
        .chain(extra.iter())
        .map(|&fd| libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        })
        .collect();
    // SAFETY: pfds is a valid array of pollfd of the given length.
    let n = unsafe { libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, timeout_ms) };
    if n < 0 {
        let e = std::io::Error::last_os_error();
        assert_eq!(e.kind(), std::io::ErrorKind::Interrupted, "poll: {e}");
        return (false, false);
    }
    let ready = |p: &libc::pollfd| p.revents & (libc::POLLIN | libc::POLLERR | libc::POLLHUP) != 0;
    let net_ready = pfds[..net.len()].iter().any(ready);
    let extra_ready = extra.is_some() && ready(&pfds[net.len()]);
    (net_ready, extra_ready)
}

/// The frontends print network errors and keep going (fatal crypto errors end the session).
fn report(e: Error) {
    match e {
        Error::Crypto(c) if c.fatal => panic!("fatal crypto error: {c}"),
        e => eprintln!("toy-sync: {e}"),
    }
}

fn server(port: Option<&str>) -> ExitCode {
    let mut network: Transport<Log, Log> =
        Transport::new_server(&Log::default(), &Log::default(), Some("127.0.0.1"), port)
            .expect("bind server");
    println!(
        "MOSH CONNECT {} {}",
        network.port().expect("port"),
        network.get_key()
    );
    std::io::stdout().flush().expect("flush");

    let acked_ts_start = network.get_sent_state_acked_timestamp();
    let mut packets = 0u64;
    let mut received = Vec::new();
    loop {
        let wait_time = network.wait_time().min(1000);
        let (net_ready, stdin_ready) = poll_fds(&network.fds(), Some(0), wait_time);
        freeze_timestamp();

        if net_ready {
            match network.recv() {
                Ok(()) => {
                    packets += 1;
                    received.extend_from_slice(&network.get_remote_diff());
                }
                Err(e) => report(e),
            }
        }
        if stdin_ready {
            let mut buf = [0u8; 64];
            // SAFETY: reading into a local buffer.
            let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
            if n <= 0 {
                break;
            }
        }
        if let Err(e) = network.tick() {
            report(e);
        }
    }

    println!(
        "server packets={} acked_ts_advance={} remote_addr={} remote={}",
        packets,
        network.get_sent_state_acked_timestamp() - acked_ts_start,
        network
            .get_remote_addr()
            .map_or("none".to_string(), |a| a.to_string()),
        hex(&received)
    );
    ExitCode::SUCCESS
}

fn client(
    key: &str,
    ip: &str,
    port: &str,
    changes: usize,
    duration_ms: u64,
    seed: u64,
) -> ExitCode {
    let mut network: Transport<Log, Log> =
        Transport::new_client(&Log::default(), &Log::default(), key, ip, port).expect("client");
    network.set_send_delay(1); /* minimal delay on outgoing keystrokes, as stmclient does */

    let mut rng = StdRng::seed_from_u64(seed);
    let mut log = Vec::new();
    let start = Instant::now();
    let limit = Duration::from_secs(60);
    let acked_ts_start = network.get_sent_state_acked_timestamp();
    let mut packets = 0u64;
    let mut pushed = 0usize;

    let status = loop {
        if start.elapsed() > limit {
            break "timeout";
        }
        let due = if changes == 0 {
            Duration::ZERO
        } else {
            Duration::from_millis(duration_ms * pushed as u64 / changes as u64)
        };
        let mut wait_time = network.wait_time().min(250);
        if pushed < changes {
            wait_time = wait_time.min(due.saturating_sub(start.elapsed()).as_millis() as i32);
        }

        let (net_ready, _) = poll_fds(&network.fds(), None, wait_time);
        freeze_timestamp();

        if net_ready {
            match network.recv() {
                Ok(()) => packets += 1,
                Err(e) => report(e),
            }
        }

        if pushed < changes && start.elapsed() >= due {
            let mut chunk = vec![0u8; rng.random_range(1..=40usize)];
            rng.fill_bytes(&mut chunk);
            network.get_current_state().apply_string(&chunk);
            log.extend_from_slice(&chunk);
            pushed += 1;
        }

        if let Err(e) = network.tick() {
            report(e);
        }

        /* rationalize_states() subtracts the acknowledged state from the current
        state, so an empty current state means the server has acknowledged everything */
        if pushed == changes && network.get_current_state().bytes.is_empty() {
            break "done";
        }
    };

    println!(
        "client status={} elapsed_ms={} packets={} acked_ts_advance={} log={}",
        status,
        start.elapsed().as_millis(),
        packets,
        network.get_sent_state_acked_timestamp() - acked_ts_start,
        hex(&log)
    );
    if status == "done" {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        [_, "server"] => server(None),
        [_, "server", port] => server(Some(port)),
        [_, "client", key, ip, port, changes, duration_ms, seed] => client(
            key,
            ip,
            port,
            changes.parse().expect("CHANGES"),
            duration_ms.parse().expect("DURATION_MS"),
            seed.parse().expect("SEED"),
        ),
        _ => {
            eprintln!("usage: toy-sync server [PORT] | toy-sync client KEY IP PORT CHANGES DURATION_MS SEED");
            ExitCode::from(2)
        }
    }
}
