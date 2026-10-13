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

//! End-to-end test of tools/udp-impair: runs the real binary between a
//! client socket and an echo server and checks that loss, delay, reordering,
//! rebind, blackout and stats behave as advertised. Unix only, as udp-impair.

#![cfg(unix)]

use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, UdpSocket};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

const PACKETS: u32 = 2000;

fn binary() -> PathBuf {
    static BIN: OnceLock<PathBuf> = OnceLock::new();
    BIN.get_or_init(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let status = Command::new(env!("CARGO"))
            .args([
                "build",
                "--release",
                "-p",
                "mosh-tools",
                "--bin",
                "udp-impair",
            ])
            .current_dir(&root)
            .status()
            .expect("run cargo build");
        assert!(status.success(), "cargo build of udp-impair failed");
        let target = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("target"));
        target.join("release").join("udp-impair")
    })
    .clone()
}

/// Echo server: replies with the same bytes and records every source address.
struct Echo {
    addr: SocketAddr,
    sources: Arc<Mutex<Vec<SocketAddr>>>,
}

fn start_echo() -> Echo {
    let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
    let addr = sock.local_addr().unwrap();
    let sources = Arc::new(Mutex::new(Vec::new()));
    let rec = sources.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 2048];
        loop {
            let (n, from) = match sock.recv_from(&mut buf) {
                Ok(x) => x,
                Err(_) => return,
            };
            rec.lock().unwrap().push(from);
            let _ = sock.send_to(&buf[..n], from);
        }
    });
    Echo { addr, sources }
}

struct Impair {
    child: Child,
    listen: SocketAddr,
    control: SocketAddr,
}

impl Drop for Impair {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn start_impair(target: SocketAddr, extra: &[&str]) -> Impair {
    let mut child = Command::new(binary())
        .args(["--listen", "127.0.0.1:0", "--control", "127.0.0.1:0"])
        .args(["--target", &target.to_string()])
        .args(extra)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn udp-impair");
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let field = |name: &str| -> SocketAddr {
        line.split_whitespace()
            .find_map(|w| w.strip_prefix(name))
            .unwrap_or_else(|| panic!("no {name} in startup line {line:?}"))
            .parse()
            .unwrap()
    };
    let listen = field("listen=");
    let control = field("control=");
    Impair {
        child,
        listen,
        control,
    }
}

fn control(imp: &Impair, cmd: &str) -> String {
    let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
    sock.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    sock.send_to(cmd.as_bytes(), imp.control).unwrap();
    let mut buf = [0u8; 1024];
    let (n, _) = sock.recv_from(&mut buf).expect("control reply");
    String::from_utf8_lossy(&buf[..n]).into_owned()
}

/// One received echo: sequence number and round-trip time.
struct Reply {
    seq: u32,
    rtt: Duration,
}

/// Send `count` numbered packets `gap` apart through the proxy, starting at
/// `first`, and collect replies until `quiet` passes with nothing arriving
/// after the last send.
fn run_traffic(imp: &Impair, first: u32, count: u32, gap: Duration, quiet: Duration) -> Vec<Reply> {
    let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
    sock.connect(imp.listen).unwrap();
    let rx = sock.try_clone().unwrap();
    let sends: Arc<Mutex<Vec<Instant>>> = Arc::new(Mutex::new(Vec::new()));
    let sends_rx = sends.clone();
    let done = Arc::new(Mutex::new(None::<Instant>));
    let done_rx = done.clone();
    let receiver = thread::spawn(move || {
        rx.set_read_timeout(Some(Duration::from_millis(20)))
            .unwrap();
        let mut out = Vec::new();
        let mut buf = [0u8; 64];
        loop {
            match rx.recv(&mut buf) {
                Ok(n) if n >= 4 => {
                    let at = Instant::now();
                    let seq = u32::from_be_bytes(buf[..4].try_into().unwrap());
                    let sent = sends_rx.lock().unwrap()[(seq - first) as usize];
                    out.push(Reply {
                        seq,
                        rtt: at - sent,
                    });
                }
                _ => {}
            }
            if let Some(end) = *done_rx.lock().unwrap() {
                if Instant::now() > end + quiet {
                    return out;
                }
            }
        }
    });
    for i in 0..count {
        let seq = first + i;
        sends.lock().unwrap().push(Instant::now());
        // A full send buffer just means this packet is lost locally; keep going.
        let _ = sock.send(&seq.to_be_bytes());
        thread::sleep(gap);
    }
    *done.lock().unwrap() = Some(Instant::now());
    receiver.join().unwrap()
}

fn out_of_sequence(replies: &[Reply]) -> usize {
    let mut max = None;
    let mut n = 0;
    for r in replies {
        match max {
            Some(m) if r.seq < m => n += 1,
            _ => max = Some(r.seq),
        }
    }
    n
}

#[test]
fn loss_half_each_direction() {
    let echo = start_echo();
    let imp = start_impair(echo.addr, &["--loss", "0.5", "--seed", "7"]);
    let replies = run_traffic(
        &imp,
        0,
        PACKETS,
        Duration::from_micros(300),
        Duration::from_millis(500),
    );
    let at_server = echo.sources.lock().unwrap().len();
    println!(
        "loss 0.5: {PACKETS} sent, {at_server} reached the echo server, {} echo replies came back; stats: {}",
        replies.len(),
        control(&imp, "stats")
    );
    // One direction: about half survive. Round trip: about a quarter.
    assert!(
        (800..=1200).contains(&at_server),
        "one-way survivors {at_server}"
    );
    assert!(
        (400..=600).contains(&replies.len()),
        "round-trip survivors {}",
        replies.len()
    );
}

#[test]
fn delay_applies_both_directions() {
    let echo = start_echo();
    let imp = start_impair(
        echo.addr,
        &["--delay-ms", "200", "--jitter-ms", "50", "--seed", "1"],
    );
    let replies = run_traffic(
        &imp,
        0,
        PACKETS,
        Duration::from_micros(500),
        Duration::from_millis(800),
    );
    let mut rtts: Vec<Duration> = replies.iter().map(|r| r.rtt).collect();
    rtts.sort();
    let median = rtts[rtts.len() / 2];
    println!(
        "delay 200 jitter 50: {} replies, min rtt {:?}, median rtt {:?}, max rtt {:?}",
        rtts.len(),
        rtts[0],
        median,
        rtts[rtts.len() - 1]
    );
    assert_eq!(rtts.len(), PACKETS as usize);
    assert!(median >= Duration::from_millis(400), "median {median:?}");
    assert!(median < Duration::from_millis(600), "median {median:?}");
}

#[test]
fn reorder_delivers_out_of_sequence() {
    let echo = start_echo();
    let imp = start_impair(echo.addr, &["--reorder", "0.3", "--seed", "3"]);
    let replies = run_traffic(
        &imp,
        0,
        PACKETS,
        Duration::from_micros(500),
        Duration::from_millis(500),
    );
    let ooo = out_of_sequence(&replies);
    println!(
        "reorder 0.3: {} replies, {ooo} out of sequence; stats: {}",
        replies.len(),
        control(&imp, "stats")
    );
    assert_eq!(
        replies.len(),
        PACKETS as usize,
        "reorder must not lose packets"
    );
    assert!(ooo >= 100, "only {ooo} out of sequence");
}

#[test]
fn rebind_changes_source_port() {
    let echo = start_echo();
    let imp = start_impair(echo.addr, &[]);
    let before = run_traffic(
        &imp,
        0,
        100,
        Duration::from_millis(1),
        Duration::from_millis(300),
    );
    let old_ports: Vec<u16> = echo
        .sources
        .lock()
        .unwrap()
        .iter()
        .map(|a| a.port())
        .collect();
    let reply = control(&imp, "rebind");
    let n_before = old_ports.len();
    let after = run_traffic(
        &imp,
        0,
        100,
        Duration::from_millis(1),
        Duration::from_millis(300),
    );
    let all: Vec<u16> = echo
        .sources
        .lock()
        .unwrap()
        .iter()
        .map(|a| a.port())
        .collect();
    let new_ports = &all[n_before..];
    println!(
        "rebind: reply {reply:?}; before {} replies from port(s) {:?}; after {} replies from port(s) {:?}",
        before.len(),
        dedup(&old_ports),
        after.len(),
        dedup(new_ports)
    );
    assert_eq!(dedup(&old_ports).len(), 1);
    assert_eq!(dedup(new_ports).len(), 1);
    assert_ne!(old_ports[0], new_ports[0], "source port did not change");
    assert_eq!(before.len(), 100);
    assert_eq!(after.len(), 100, "traffic must still flow after rebind");
}

fn dedup(ports: &[u16]) -> Vec<u16> {
    let mut v = ports.to_vec();
    v.sort();
    v.dedup();
    v
}

/// Send numbered packets 10 ms apart for `dur`, recording each send time in
/// `sent` (index = sequence number) and each reply's sequence in `replies`.
fn send_for(sock: &UdpSocket, sent: &mut Vec<Instant>, replies: &mut Vec<u32>, dur: Duration) {
    let end = Instant::now() + dur;
    while Instant::now() < end {
        let seq = sent.len() as u32;
        sent.push(Instant::now());
        let _ = sock.send(&seq.to_be_bytes());
        receive_for(sock, replies, Duration::from_millis(10));
    }
}

/// Collect replies for `dur`.
fn receive_for(sock: &UdpSocket, replies: &mut Vec<u32>, dur: Duration) {
    let end = Instant::now() + dur;
    let mut buf = [0u8; 64];
    loop {
        let left = end.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return;
        }
        sock.set_read_timeout(Some(left)).unwrap();
        if let Ok(n) = sock.recv(&mut buf) {
            if n >= 4 {
                replies.push(u32::from_be_bytes(buf[..4].try_into().unwrap()));
            }
        }
    }
}

/// Collect replies until none arrives for `quiet`.
fn drain(sock: &UdpSocket, replies: &mut Vec<u32>, quiet: Duration) {
    let mut buf = [0u8; 64];
    sock.set_read_timeout(Some(quiet)).unwrap();
    while let Ok(n) = sock.recv(&mut buf) {
        if n >= 4 {
            replies.push(u32::from_be_bytes(buf[..4].try_into().unwrap()));
        }
    }
}

#[test]
fn blackout_silences_then_resumes() {
    const BLACKOUT: Duration = Duration::from_millis(2000);
    // Longest a packet may take from our send to the proxy (loopback, no
    // configured delay, but a loaded machine). Packets sent this close to
    // the blackout's earliest possible end are not judged.
    const TRANSIT: Duration = Duration::from_millis(300);
    // The proxy has no configured delay, so in-flight replies are gone well
    // within this; it stands in for "twice the configured delay".
    const QUIET: Duration = Duration::from_millis(200);

    let echo = start_echo();
    let imp = start_impair(echo.addr, &[]);
    let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
    sock.connect(imp.listen).unwrap();
    let mut sent = Vec::new();
    let mut replies = Vec::new();

    // 1 s of traffic, then stop and drain, so no reply to a packet sent
    // before the blackout is still in flight when it starts.
    send_for(&sock, &mut sent, &mut replies, Duration::from_secs(1));
    drain(&sock, &mut replies, QUIET);
    let sent_before = sent.len();
    let replies_before = replies.len();

    // The proxy starts the blackout when it handles the command: after
    // ctl_sent and before ok_at.
    let ctl_sent = Instant::now();
    assert_eq!(control(&imp, "blackout 2000"), "ok");
    let ok_at = Instant::now();
    send_for(
        &sock,
        &mut sent,
        &mut replies,
        BLACKOUT + Duration::from_secs(1),
    );
    drain(&sock, &mut replies, QUIET);

    // Sent after the blackout surely began and early enough to reach the
    // proxy before it can have ended: the proxy must drop these.
    let inside: Vec<u32> = (sent_before..sent.len())
        .filter(|&i| sent[i] >= ok_at && sent[i] + TRANSIT <= ctl_sent + BLACKOUT)
        .map(|i| i as u32)
        .collect();
    // Sent after the blackout surely ended: these must come back.
    let after: Vec<u32> = (sent_before..sent.len())
        .filter(|&i| sent[i] >= ok_at + BLACKOUT)
        .map(|i| i as u32)
        .collect();
    let answered = |seqs: &[u32]| seqs.iter().filter(|s| replies.contains(s)).count();
    let inside_answered = answered(&inside);
    let after_answered = answered(&after);
    println!(
        "blackout 2000: before: {replies_before}/{sent_before} answered; sent inside the blackout: {} ({inside_answered} answered); sent after it: {} ({after_answered} answered); stats: {}",
        inside.len(),
        after.len(),
        control(&imp, "stats")
    );
    assert!(
        replies_before * 10 >= sent_before * 9,
        "traffic before the blackout"
    );
    assert!(
        inside.len() >= 100,
        "too few packets sent inside the blackout"
    );
    assert_eq!(
        inside_answered, 0,
        "replies to packets sent during the blackout"
    );
    assert!(after.len() >= 50, "too few packets sent after the blackout");
    assert!(
        after_answered * 10 >= after.len() * 9,
        "traffic did not resume"
    );
}

#[test]
fn stats_and_sigterm() {
    let echo = start_echo();
    let mut imp = start_impair(
        echo.addr,
        &["--loss", "0.2", "--reorder", "0.1", "--seed", "9"],
    );
    let replies = run_traffic(
        &imp,
        0,
        500,
        Duration::from_micros(500),
        Duration::from_millis(300),
    );
    let line = control(&imp, "stats");
    println!("stats reply: {line}");
    for key in ["c2s forwarded=", "s2c forwarded=", "dropped=", "reordered="] {
        assert!(line.contains(key), "stats line lacks {key}");
    }
    let num = |dir: &str, key: &str| -> u64 {
        let rest = &line[line.find(dir).unwrap()..];
        let v = &rest[rest.find(key).unwrap() + key.len()..];
        v.split_whitespace().next().unwrap().parse().unwrap()
    };
    assert_eq!(num("c2s", "forwarded=") + num("c2s", "dropped="), 500);
    assert_eq!(num("s2c", "forwarded="), replies.len() as u64);
    assert_eq!(control(&imp, "loss 0"), "ok");
    assert_eq!(control(&imp, "delay 5"), "ok");
    assert!(control(&imp, "bogus").starts_with("error"));

    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(imp.child.id() as i32),
        nix::sys::signal::Signal::SIGTERM,
    )
    .unwrap();
    let status = imp.child.wait().unwrap();
    let mut err = String::new();
    imp.child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut err)
        .unwrap();
    println!("after SIGTERM: {status}, stderr: {}", err.trim());
    assert!(status.success());
    assert!(err.contains("c2s forwarded="));
}
