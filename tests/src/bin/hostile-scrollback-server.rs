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
// Ported from: (none; test server whose loop follows upstream/src/frontend/mosh-server.cc)

//! hostile-scrollback-server: a mosh server that holds the session key and
//! sends one crafted scrollback diff, for testing that a client in
//! scrollback mode stays bounded against a hostile or broken peer
//! (reviews/2026-10-13-phase2-4-client-server-scrollback.md, G1 and G2).
//!
//! hostile-scrollback-server PORT KIND SIZE [SECONDS]
//!
//! Binds 127.0.0.1:PORT, prints `MOSH CONNECT <port> <key>` and serves for
//! SECONDS (default 60). Once the client's first state arrives, the server's
//! state becomes one whose diff from the initial state holds a
//! `ScrolledLines` instruction and then host bytes that draw `FAKE-READY` at
//! the top of the screen. KIND picks the history line:
//! - `newlines`: SIZE line feeds in one line (a real server's renderer never
//!   writes a line feed, so each would become a host row),
//! - `letters`: SIZE letters in one line (wraps into SIZE / width rows),
//! - `maxindex`: the line `HOSTILE` numbered `u64::MAX - SIZE`.

use std::io::Write;
use std::time::{Duration, Instant};

use mosh::statesync::UserStream;
use mosh_network::prost::Message;
use mosh_network::proto::host_buffers::{HostBytes, HostMessage, Instruction, ScrolledLines};
use mosh_network::{freeze_timestamp, CryptoError, Error, State, Transport};

/// The server's state: 0 before the client spoke, 1 after.
#[derive(Clone, PartialEq, Debug)]
struct Hostile {
    step: u64,
    first_index: u64,
    line: std::rc::Rc<Vec<u8>>,
}

impl State for Hostile {
    fn diff_from(&self, existing: &Hostile) -> Vec<u8> {
        let mut msg = HostMessage::default();
        if existing.step < self.step {
            msg.instruction.push(Instruction {
                scrolledlines: Some(ScrolledLines {
                    first_index: Some(self.first_index),
                    line: vec![self.line.to_vec()],
                    dropped: None,
                }),
                ..Default::default()
            });
            msg.instruction.push(Instruction {
                hostbytes: Some(HostBytes {
                    hoststring: Some(b"\x1b[H\x1b[2JFAKE-READY".to_vec()),
                }),
                ..Default::default()
            });
        }
        msg.encode_to_vec()
    }

    fn init_diff(&self) -> Vec<u8> {
        self.diff_from(&Hostile {
            step: 0,
            ..self.clone()
        })
    }

    fn apply_string(&mut self, _diff: &[u8]) -> Result<(), CryptoError> {
        Ok(())
    }

    fn subtract(&mut self, _prefix: &Hostile) {}

    fn reset_input(&mut self) {}

    fn compare(&self, _other: &Hostile) -> bool {
        false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: hostile-scrollback-server PORT newlines|letters|maxindex SIZE [SECONDS]");
        std::process::exit(2);
    }
    let size: u64 = args[2].parse().expect("SIZE is a number");
    let seconds: u64 = args.get(3).map_or(60, |s| s.parse().expect("SECONDS"));
    let (first_index, line) = match args[1].as_str() {
        "newlines" => (1, vec![b'\n'; size as usize]),
        "letters" => (1, vec![b'x'; size as usize]),
        "maxindex" => (u64::MAX - size, b"HOSTILE".to_vec()),
        k => panic!("unknown KIND {k}"),
    };
    let start_state = Hostile {
        step: 0,
        first_index,
        line: std::rc::Rc::new(line),
    };
    let mut network: Transport<Hostile, UserStream> = Transport::new_server(
        &start_state,
        &UserStream::new(),
        Some("127.0.0.1"),
        Some(&args[0]),
    )
    .unwrap_or_else(|e| panic!("bind: {e}"));
    println!(
        "MOSH CONNECT {} {}",
        network.port().expect("port"),
        network.get_key()
    );
    let _ = std::io::stdout().flush();

    let start = Instant::now();
    let mut sent = false;
    while start.elapsed() < Duration::from_secs(seconds) {
        freeze_timestamp();
        loop {
            match network.recv() {
                Ok(()) => {}
                Err(Error::Network(e)) if e.function == "No packet received" => break,
                Err(e) => eprintln!("{e}"),
            }
        }
        if !sent && network.get_remote_state_num() != 0 {
            sent = true;
            network.set_current_state(&Hostile {
                step: 1,
                ..start_state.clone()
            });
        }
        if let Err(e) = network.tick() {
            eprintln!("{e}");
        }
        let wait = network.wait_time().clamp(0, 5);
        std::thread::sleep(Duration::from_millis(wait as u64));
    }
}
