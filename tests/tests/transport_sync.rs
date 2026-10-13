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
// Ported from: (none; end-to-end test of upstream/src/network/networktransport-impl.h behavior)

//! Two SSP endpoints (separate `toy-sync` processes) synchronize a toy
//! state over UDP on loopback.
//!
//! Environment:
//! - `MOSH_IMPAIR_PROXY=IP:PORT`: the client sends to this address (a
//!   `udp-impair` instance) instead of directly to the server. The proxy
//!   needs a fixed target, so the server then binds the port given in
//!   `MOSH_SYNC_SERVER_PORT` (required with the proxy).
//! - `MOSH_SYNC_SERVER_PORT=PORT`: server port (default: first free port
//!   in 60001-60999, as mosh-server does).
//! - `SYNC_SEED=N`: seed for the client's random changes.
//!
//! Unix only, as `toy-sync`.

#![cfg(unix)]

use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const CHANGES: usize = 300;
const DURATION_MS: u64 = 10_000;

fn field<'a>(line: &'a str, name: &str) -> &'a str {
    let key = format!("{name}=");
    line.split(' ')
        .find_map(|w| w.strip_prefix(key.as_str()))
        .unwrap_or_else(|| panic!("no {name}= in {line:?}"))
}

#[test]
fn transport_sync() {
    let toy = env!("CARGO_BIN_EXE_toy-sync");
    let proxy = std::env::var("MOSH_IMPAIR_PROXY").ok();
    let server_port = std::env::var("MOSH_SYNC_SERVER_PORT").ok();
    if proxy.is_some() {
        assert!(
            server_port.is_some(),
            "MOSH_IMPAIR_PROXY needs MOSH_SYNC_SERVER_PORT (the proxy's target port)"
        );
    }
    let seed: u64 = match std::env::var("SYNC_SEED") {
        Ok(s) => s.parse().expect("SYNC_SEED is a u64"),
        Err(_) => rand::random(),
    };
    println!("transport_sync seed={seed} proxy={proxy:?} server_port={server_port:?}");

    let start = Instant::now();
    let mut server = Command::new(toy)
        .arg("server")
        .args(server_port.iter())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("spawn toy-sync server");
    let mut server_out = BufReader::new(server.stdout.take().expect("server stdout"));
    let mut line = String::new();
    server_out.read_line(&mut line).expect("read MOSH CONNECT");
    let words: Vec<&str> = line.split_whitespace().collect();
    assert!(
        words.len() == 4 && words[0] == "MOSH" && words[1] == "CONNECT",
        "bad server line {line:?}"
    );
    let (port, key) = (words[2].to_string(), words[3].to_string());
    let port_num: u16 = port.parse().expect("port");
    assert!(
        (60001..=60999).contains(&port_num) || server_port.is_some(),
        "port {port} outside mosh range"
    );

    let (ip, cport) = match &proxy {
        Some(p) => {
            let a: SocketAddr = p.parse().expect("MOSH_IMPAIR_PROXY is IP:PORT");
            (a.ip().to_string(), a.port().to_string())
        }
        None => ("127.0.0.1".to_string(), port.clone()),
    };

    let client = Command::new(toy)
        .args(["client", &key, &ip, &cport])
        .arg(CHANGES.to_string())
        .arg(DURATION_MS.to_string())
        .arg(seed.to_string())
        .stderr(Stdio::inherit())
        .output()
        .expect("run toy-sync client");
    let client_line = String::from_utf8(client.stdout).expect("utf8");
    let client_line = client_line.trim_end();

    /* closing the server's stdin tells it to report and exit */
    drop(server.stdin.take());
    let mut server_line = String::new();
    server_out
        .read_line(&mut server_line)
        .expect("read server report");
    let server_line = server_line.trim_end();
    assert!(
        server.wait().expect("wait server").success(),
        "server failed"
    );
    let elapsed = start.elapsed();

    let client_log = field(client_line, "log");
    let server_remote = field(server_line, "remote");
    println!(
        "elapsed {:.2}s; client: status={} elapsed_ms={} packets_received={} acked_ts_advance={}; server: packets_received={} acked_ts_advance={} remote_addr={}; state {} bytes",
        elapsed.as_secs_f64(),
        field(client_line, "status"),
        field(client_line, "elapsed_ms"),
        field(client_line, "packets"),
        field(client_line, "acked_ts_advance"),
        field(server_line, "packets"),
        field(server_line, "acked_ts_advance"),
        field(server_line, "remote_addr"),
        client_log.len() / 2,
    );

    assert_eq!(
        field(client_line, "status"),
        "done",
        "client did not get everything acknowledged"
    );
    assert!(client.status.success(), "client failed");
    assert!(elapsed < Duration::from_secs(60), "took {elapsed:?}");
    assert!(!client_log.is_empty());
    assert_eq!(
        server_remote, client_log,
        "server's received state differs from the client's state"
    );
    let client_adv: u64 = field(client_line, "acked_ts_advance").parse().unwrap();
    let server_adv: u64 = field(server_line, "acked_ts_advance").parse().unwrap();
    assert!(client_adv > 0, "client's acked timestamp did not advance");
    assert!(server_adv > 0, "server's acked timestamp did not advance");
    let addr: SocketAddr = field(server_line, "remote_addr")
        .parse()
        .expect("server remote_addr is an address");
    assert!(
        addr.port() != 0 && !addr.ip().is_unspecified(),
        "bad remote addr {addr}"
    );
}
