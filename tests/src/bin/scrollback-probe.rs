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
// Ported from: (none; test client whose loop follows upstream/src/frontend/stmclient.cc)

//! scrollback-probe: the client side of a mosh session with no rendering,
//! for testing the server half of scrollback (PLAN.md section 6).
//!
//! scrollback-probe IP PORT OUTFILE [--no-capability] [--timeout-s N]
//!
//! The key comes from `MOSH_KEY`, as for mosh-client. The probe sends the
//! scrollback capability (unless `--no-capability`) and an 80x24 resize as
//! its first events, then receives until the server shuts down (the session's
//! command exited). OUTFILE gets the text of every scrolled line received,
//! one per line, then the rows of the final screen (trailing empty rows
//! left out). Counts go to stdout. Exit status 0 when the server shut down,
//! 1 on timeout.

use std::time::{Duration, Instant};

use mosh::statesync::{ClientCapabilities, Complete, UserStream};
use mosh_network::{freeze_timestamp, Error, Transport};
use mosh_terminal::terminalframebuffer::Framebuffer;
use mosh_terminal::Resize;

/// The text of row `r`: graphemes, the empty half of a wide character
/// skipped, trailing blanks trimmed.
fn row_text(fb: &Framebuffer, r: usize) -> String {
    let mut s = String::new();
    let mut skip = false;
    for cell in &fb.get_rows()[r].cells {
        if skip {
            skip = false;
            continue;
        }
        cell.print_grapheme(&mut s);
        skip = cell.get_wide();
    }
    s.trim_end_matches(' ').to_string()
}

fn main() {
    let mut positional: Vec<String> = Vec::new();
    let mut capability = true;
    let mut timeout_s: u64 = 120;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--no-capability" => capability = false,
            "--timeout-s" => {
                timeout_s = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--timeout-s N")
            }
            _ => positional.push(a),
        }
    }
    if positional.len() != 3 {
        eprintln!("usage: scrollback-probe IP PORT OUTFILE [--no-capability] [--timeout-s N]");
        std::process::exit(2);
    }
    let (ip, port, outfile) = (&positional[0], &positional[1], &positional[2]);
    let key = std::env::var("MOSH_KEY").expect("MOSH_KEY");

    let (width, height) = (80, 24);
    let mut remote = Complete::new(width, height);
    let received = remote.enable_scrollback_receive();
    let mut network: Transport<UserStream, Complete> =
        Transport::new_client(&UserStream::new(), &remote, &key, ip, port)
            .unwrap_or_else(|e| panic!("connect: {e}"));
    if capability {
        network
            .get_current_state()
            .push_back_capabilities(ClientCapabilities { scrollback: true });
    }
    network
        .get_current_state()
        .push_back_resize(Resize::new(width, height));

    let start = Instant::now();
    let mut lines: Vec<String> = Vec::new();
    let mut gaps = 0u64;
    let mut next_index = 1u64;
    let mut done = false;
    while !done {
        if start.elapsed() > Duration::from_secs(timeout_s) {
            break;
        }
        freeze_timestamp();
        // Every datagram that is waiting, then the sender.
        loop {
            match network.recv() {
                Ok(()) => {}
                Err(Error::Network(e)) if e.function == "No packet received" => break,
                Err(Error::Crypto(e)) if e.fatal => panic!("fatal crypto error: {e}"),
                Err(e) => eprintln!("{e}"),
            }
        }
        for (index, line) in received.borrow_mut().take_pending() {
            if index != next_index {
                gaps += 1;
            }
            next_index = index + 1;
            let mut t = Complete::new(400, 1);
            assert!(
                t.act(&line).is_empty(),
                "a scrolled line made the emulator reply"
            );
            lines.push(row_text(t.get_fb(), 0));
        }
        if network.counterparty_shutdown_ack_sent() {
            done = true;
        }
        if let Err(e) = network.tick() {
            eprintln!("{e}");
        }
        let wait = network.wait_time().clamp(0, 5);
        std::thread::sleep(Duration::from_millis(wait as u64));
    }

    let state = &network.get_latest_remote_state().state;
    let fb = state.get_fb();
    let mut screen: Vec<String> = (0..fb.get_rows().len()).map(|r| row_text(fb, r)).collect();
    while screen.last().is_some_and(|l| l.is_empty()) {
        screen.pop();
    }
    let mut out = String::new();
    for l in lines.iter().chain(screen.iter()) {
        out.push_str(l);
        out.push('\n');
    }
    std::fs::write(outfile, out).expect("write OUTFILE");
    println!(
        "capability={} shutdown={} lines_received={} lost={} gaps={} scrolled_total={} screen_rows={} seconds={:.2}",
        capability,
        done,
        lines.len(),
        received.borrow().lost(),
        gaps,
        state.get_scrolled_total(),
        screen.len(),
        start.elapsed().as_secs_f64()
    );
    std::process::exit(if done { 0 } else { 1 });
}
