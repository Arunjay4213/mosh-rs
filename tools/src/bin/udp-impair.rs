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

use std::net::{SocketAddr, UdpSocket};
use std::os::fd::AsFd;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use nix::sys::signal::{signal, SigHandler, Signal};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

const HOLD_TIMEOUT: Duration = Duration::from_millis(100);
const C2S: usize = 0;
const S2C: usize = 1;
const DIR_NAMES: [&str; 2] = ["c2s", "s2c"];

static STOP: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_: libc::c_int) {
    STOP.store(true, Ordering::SeqCst);
}

struct Packet {
    due: Instant,
    order: u64,
    dir: usize,
    data: Vec<u8>,
}

#[derive(Default, Clone, Copy)]
struct Counters {
    forwarded: u64,
    dropped: u64,
    reordered: u64,
}

struct Proxy {
    listen: UdpSocket,
    upstream: UdpSocket,
    control: UdpSocket,
    target: SocketAddr,
    client: Option<SocketAddr>,
    loss: f64,
    delay: Duration,
    jitter: Duration,
    reorder: f64,
    rng: StdRng,
    blackout_until: Option<Instant>,
    queue: Vec<Packet>,
    held: [Option<Packet>; 2],
    next_order: u64,
    counters: [Counters; 2],
}

fn usage() -> ! {
    eprintln!(
        "usage: udp-impair --listen ADDR --target ADDR --control ADDR \
         [--loss P] [--delay-ms MS] [--jitter-ms MS] [--reorder P] [--seed N]"
    );
    exit(2);
}

fn bind_nonblocking(addr: SocketAddr) -> UdpSocket {
    let sock = UdpSocket::bind(addr).unwrap_or_else(|e| {
        eprintln!("udp-impair: bind {addr}: {e}");
        exit(1);
    });
    sock.set_nonblocking(true).expect("set_nonblocking");
    sock
}

fn upstream_bind_addr(target: SocketAddr) -> SocketAddr {
    if target.is_ipv4() {
        "0.0.0.0:0".parse().unwrap()
    } else {
        "[::]:0".parse().unwrap()
    }
}

impl Proxy {
    fn stats_line(&self) -> String {
        let mut parts = Vec::new();
        for dir in [C2S, S2C] {
            let c = self.counters[dir];
            parts.push(format!(
                "{} forwarded={} dropped={} reordered={}",
                DIR_NAMES[dir], c.forwarded, c.dropped, c.reordered
            ));
        }
        parts.join(" ")
    }

    fn in_blackout(&self, now: Instant) -> bool {
        self.blackout_until.is_some_and(|t| now < t)
    }

    fn schedule(&mut self, mut p: Packet) {
        p.order = self.next_order;
        self.next_order += 1;
        self.queue.push(p);
    }

    /// A packet has arrived from one side; decide its fate.
    fn admit(&mut self, dir: usize, data: Vec<u8>, now: Instant) {
        if self.in_blackout(now) || self.rng.random::<f64>() < self.loss {
            self.counters[dir].dropped += 1;
            return;
        }
        let mut due = now + self.delay;
        if !self.jitter.is_zero() {
            due += self.jitter.mul_f64(self.rng.random::<f64>());
        }
        let p = Packet {
            due,
            order: 0,
            dir,
            data,
        };
        if self.rng.random::<f64>() < self.reorder {
            // Hold this one back. A previously held packet is released now.
            if let Some(mut h) = self.held[dir].take() {
                h.due = h.due.max(due);
                self.schedule(h);
            }
            self.counters[dir].reordered += 1;
            self.held[dir] = Some(p);
        } else {
            let p_due = p.due;
            self.schedule(p);
            // Release the held packet one slot behind this one.
            if let Some(mut h) = self.held[dir].take() {
                h.due = h.due.max(p_due);
                self.schedule(h);
            }
        }
    }

    fn release_stale_holds(&mut self, now: Instant) {
        for dir in [C2S, S2C] {
            if self.held[dir]
                .as_ref()
                .is_some_and(|h| now >= h.due + HOLD_TIMEOUT)
            {
                let h = self.held[dir].take().unwrap();
                self.schedule(h);
            }
        }
    }

    fn deliver_due(&mut self, now: Instant) {
        if !self.queue.iter().any(|p| p.due <= now) {
            return;
        }
        self.queue.sort_by_key(|p| (p.due, p.order));
        let split = self
            .queue
            .iter()
            .position(|p| p.due > now)
            .unwrap_or(self.queue.len());
        let ready: Vec<Packet> = self.queue.drain(..split).collect();
        let blackout = self.in_blackout(now);
        for p in ready {
            if blackout {
                self.counters[p.dir].dropped += 1;
                continue;
            }
            let res = if p.dir == C2S {
                self.upstream.send_to(&p.data, self.target)
            } else {
                match self.client {
                    Some(c) => self.listen.send_to(&p.data, c),
                    None => continue,
                }
            };
            match res {
                Ok(_) => self.counters[p.dir].forwarded += 1,
                Err(_) => self.counters[p.dir].dropped += 1,
            }
        }
    }

    fn next_deadline(&self) -> Option<Instant> {
        let q = self.queue.iter().map(|p| p.due).min();
        let h = self
            .held
            .iter()
            .flatten()
            .map(|h| h.due + HOLD_TIMEOUT)
            .min();
        match (q, h) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    fn handle_control(&mut self, msg: &str, now: Instant) -> String {
        let mut words = msg.split_whitespace();
        let cmd = words.next().unwrap_or("");
        let arg = words.next();
        match (cmd, arg) {
            ("rebind", None) => {
                let addr = upstream_bind_addr(self.target);
                self.upstream = bind_nonblocking(addr);
                let port = self.upstream.local_addr().map(|a| a.port()).unwrap_or(0);
                format!("ok rebind port={port}")
            }
            ("blackout", Some(ms)) => match ms.parse::<u64>() {
                Ok(ms) => {
                    self.blackout_until = Some(now + Duration::from_millis(ms));
                    "ok".to_string()
                }
                Err(_) => "error bad milliseconds".to_string(),
            },
            ("loss", Some(p)) => match p.parse::<f64>() {
                Ok(p) if (0.0..=1.0).contains(&p) => {
                    self.loss = p;
                    "ok".to_string()
                }
                _ => "error bad probability".to_string(),
            },
            ("delay", Some(ms)) => match ms.parse::<u64>() {
                Ok(ms) => {
                    self.delay = Duration::from_millis(ms);
                    "ok".to_string()
                }
                Err(_) => "error bad milliseconds".to_string(),
            },
            ("stats", None) => self.stats_line(),
            _ => format!("error unknown command: {}", msg.trim()),
        }
    }
}

fn parse_args() -> Proxy {
    let mut listen = None;
    let mut target = None;
    let mut control = None;
    let mut loss = 0.0;
    let mut delay_ms = 0u64;
    let mut jitter_ms = 0u64;
    let mut reorder = 0.0;
    let mut seed = None;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().unwrap_or_else(|| usage());
        let bad = |what: &str| -> ! {
            eprintln!("udp-impair: bad value for {flag}: {value} ({what})");
            exit(2);
        };
        match flag.as_str() {
            "--listen" => {
                listen = Some(
                    value
                        .parse::<SocketAddr>()
                        .unwrap_or_else(|_| bad("address")),
                )
            }
            "--target" => {
                target = Some(
                    value
                        .parse::<SocketAddr>()
                        .unwrap_or_else(|_| bad("address")),
                )
            }
            "--control" => {
                control = Some(
                    value
                        .parse::<SocketAddr>()
                        .unwrap_or_else(|_| bad("address")),
                )
            }
            "--loss" => loss = value.parse().unwrap_or_else(|_| bad("probability")),
            "--delay-ms" => delay_ms = value.parse().unwrap_or_else(|_| bad("milliseconds")),
            "--jitter-ms" => jitter_ms = value.parse().unwrap_or_else(|_| bad("milliseconds")),
            "--reorder" => reorder = value.parse().unwrap_or_else(|_| bad("probability")),
            "--seed" => seed = Some(value.parse::<u64>().unwrap_or_else(|_| bad("integer"))),
            _ => usage(),
        }
    }
    let (Some(listen), Some(target), Some(control)) = (listen, target, control) else {
        usage();
    };
    let rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => rand::make_rng(),
    };
    Proxy {
        listen: bind_nonblocking(listen),
        upstream: bind_nonblocking(upstream_bind_addr(target)),
        control: bind_nonblocking(control),
        target,
        client: None,
        loss,
        delay: Duration::from_millis(delay_ms),
        jitter: Duration::from_millis(jitter_ms),
        reorder,
        rng,
        blackout_until: None,
        queue: Vec::new(),
        held: [None, None],
        next_order: 0,
        counters: [Counters::default(); 2],
    }
}

fn main() {
    let mut proxy = parse_args();
    unsafe {
        signal(Signal::SIGINT, SigHandler::Handler(on_signal)).expect("signal");
        signal(Signal::SIGTERM, SigHandler::Handler(on_signal)).expect("signal");
    }
    println!(
        "udp-impair listen={} control={} upstream={} target={}",
        proxy.listen.local_addr().unwrap(),
        proxy.control.local_addr().unwrap(),
        proxy.upstream.local_addr().unwrap(),
        proxy.target
    );
    use std::io::Write;
    std::io::stdout().flush().ok();

    let mut buf = vec![0u8; 65536];
    while !STOP.load(Ordering::SeqCst) {
        let now = Instant::now();
        let timeout = match proxy.next_deadline() {
            None => PollTimeout::NONE,
            Some(d) => {
                // Round up so we never wake before the deadline and spin.
                let us = d.saturating_duration_since(now).as_micros();
                PollTimeout::try_from(us.div_ceil(1000)).unwrap_or(PollTimeout::MAX)
            }
        };
        let (listen_ready, upstream_ready, control_ready) = {
            let mut fds = [
                PollFd::new(proxy.listen.as_fd(), PollFlags::POLLIN),
                PollFd::new(proxy.upstream.as_fd(), PollFlags::POLLIN),
                PollFd::new(proxy.control.as_fd(), PollFlags::POLLIN),
            ];
            match poll(&mut fds, timeout) {
                Ok(_) => {}
                Err(nix::errno::Errno::EINTR) => continue,
                Err(e) => {
                    eprintln!("udp-impair: poll: {e}");
                    exit(1);
                }
            }
            let ready = |f: &PollFd| f.revents().is_some_and(|r| !r.is_empty());
            (ready(&fds[0]), ready(&fds[1]), ready(&fds[2]))
        };
        let now = Instant::now();
        if listen_ready {
            while let Ok((n, from)) = proxy.listen.recv_from(&mut buf) {
                proxy.client = Some(from);
                proxy.admit(C2S, buf[..n].to_vec(), now);
            }
        }
        if upstream_ready {
            while let Ok((n, from)) = proxy.upstream.recv_from(&mut buf) {
                if from == proxy.target {
                    proxy.admit(S2C, buf[..n].to_vec(), now);
                }
            }
        }
        if control_ready {
            while let Ok((n, from)) = proxy.control.recv_from(&mut buf) {
                let msg = String::from_utf8_lossy(&buf[..n]).into_owned();
                let reply = proxy.handle_control(&msg, now);
                let _ = proxy.control.send_to(reply.as_bytes(), from);
            }
        }
        let now = Instant::now();
        proxy.release_stale_holds(now);
        proxy.deliver_due(now);
    }
    eprintln!("udp-impair stats: {}", proxy.stats_line());
}
