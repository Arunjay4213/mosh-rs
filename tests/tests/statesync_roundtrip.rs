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
// Ported from: (none; round-trip test for the port of upstream/src/statesync)
//
// State synchronization round trip. A server `Complete` acts on a corpus file
// in 41 chunks (with one resize in the middle); a client `Complete` applies
// each `server_i.diff_from(server_{i-1})`. Checked:
// - after every step, the client shows what the server shows: every field
//   `Display::new_frame` transmits (DrawState::operator== fields, every cell,
//   title, clipboard). Full equality is not expected even upstream:
//   `Complete::operator==` compares framebuffer row pointers, and new_frame
//   does not carry tabs, scroll region, saved cursor or the bell count;
// - at the end, the server's full dump equals ref-term (upstream C++) replaying
//   the server's input, and the client's full dump equals ref-term replaying
//   exactly the host bytes and resizes the client applied;
// - init_diff into a fresh client shows the same screen;
// - UserStream diff/apply/subtract, and that an unknown instruction type is
//   ignored.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use mosh::statesync::{Complete, UserAction, UserEventType, UserStream};
use mosh_network::prost::Message;
use mosh_network::proto::client_buffers::{self, UserMessage};
use mosh_network::proto::host_buffers::{self, HostMessage};
use mosh_network::proto::transport_buffers;
use mosh_network::transport::transportfragment::Fragmenter;
use mosh_network::{Connection, Error, Session, State, Transport};
use mosh_terminal::terminalframebuffer::Framebuffer;
use mosh_terminal::{Resize, UserByte};
use mosh_tests::dump::{dump, hex, strip_gen};
use mosh_tests::oracle::{
    build_ref_term, first_difference, prefix_offsets, record, root, run, Rng,
};

const SIZES: [(usize, usize); 2] = [(80, 24), (132, 43)];
const CHUNKS: usize = 40;

/// Every `*.bin` under tests/corpus, recursively.
fn all_corpus(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries {
        let p = e.expect("dir entry").path();
        if p.is_dir() {
            all_corpus(&p, out);
        } else if p.extension().is_some_and(|x| x == "bin") {
            out.push(p);
        }
    }
}

/// What `Display::new_frame` transmits: the fields DrawState::operator==
/// compares, every cell, and the title and clipboard.
fn screen(fb: &Framebuffer) -> String {
    let ds = &fb.ds;
    let mut o = String::new();
    let _ = writeln!(
        o,
        "size {} {} cursor {} {} visible {} rv {} paste {} mouse {} {} {} {}",
        ds.get_width(),
        ds.get_height(),
        ds.get_cursor_row(),
        ds.get_cursor_col(),
        ds.cursor_visible,
        ds.reverse_video,
        ds.bracketed_paste,
        ds.mouse_reporting_mode as i32,
        ds.mouse_focus_event,
        ds.mouse_alternate_scroll,
        ds.mouse_encoding_mode as i32
    );
    let _ = writeln!(
        o,
        "renditions {:?} hyperlink {:?}",
        ds.get_renditions(),
        ds.get_hyperlink()
    );
    // new_frame sends titles only when title_initialized and they differ, so
    // an empty title never marks the client's title as initialized; compare
    // the values, not the flag.
    let _ = writeln!(
        o,
        "title {:x?} icon {:x?}",
        fb.get_window_title(),
        fb.get_icon_name()
    );
    let _ = writeln!(o, "clipboard {:x?}", fb.get_clipboard());
    // Cells as upstream's Cell::compare judges them: the printed grapheme
    // (an empty cell prints as a space, which is how new_frame may draw it),
    // width, renditions and wrap; plus the hyperlink.
    for (r, row) in fb.get_rows().iter().enumerate() {
        for (c, cell) in row.cells.iter().enumerate() {
            let mut g = String::new();
            cell.print_grapheme(&mut g);
            let _ = writeln!(
                o,
                "{} {} {} wide {} wrap {} {:?} {:?}",
                r,
                c,
                hex(g.as_bytes()),
                cell.get_wide(),
                cell.get_wrap(),
                cell.get_renditions(),
                cell.get_hyperlink()
            );
        }
    }
    o
}

fn full_dump(fb: &Framebuffer, to_host: &[u8]) -> String {
    let mut o = String::new();
    dump(fb, to_host, false, &mut o);
    o
}

fn bytes_rec(log: &mut Vec<u8>, b: &[u8]) {
    let mut r = vec![b'B'];
    r.extend_from_slice(b);
    record(log, &r);
}

fn resize_rec(log: &mut Vec<u8>, w: usize, h: usize) {
    let mut r = vec![b'R'];
    r.extend_from_slice(&(w as u32).to_be_bytes());
    r.extend_from_slice(&(h as u32).to_be_bytes());
    record(log, &r);
}

fn check(mismatches: &mut usize, what: &str, a: &str, b: &str) {
    if a != b {
        *mismatches += 1;
        // first_difference labels its sides ref-term / vt-check; here they
        // are the expected side (server or ref-term) and the Rust side.
        println!(
            "MISMATCH {}: {}",
            what,
            first_difference(a.as_bytes(), b.as_bytes())
        );
    }
}

#[test]
fn complete_roundtrip() {
    let ref_term = build_ref_term();
    let mut files = Vec::new();
    all_corpus(&root().join("tests/corpus"), &mut files);
    files.sort();
    assert!(!files.is_empty(), "no corpus files");
    let (mut steps, mut mismatches, mut resizes_seen) = (0usize, 0usize, 0usize);
    let mut upstream_divergences = 0usize;

    for file in &files {
        let name = file
            .strip_prefix(root())
            .unwrap_or(file)
            .display()
            .to_string();
        let data = std::fs::read(file).expect("read corpus file");
        let offsets = prefix_offsets(&data, CHUNKS);

        for (w, h) in SIZES {
            let (rw, rh) = (w - 13, h + 5);
            let mut server = Complete::new(w, h);
            let mut client = Complete::new(w, h);
            let mut server_log = Vec::new();
            let mut client_log = Vec::new();
            let mut server_to_host = Vec::new();
            let mut prev = server.clone();
            let mut start = 0usize;

            for (i, &end) in offsets.iter().enumerate() {
                let at = format!("{} {}x{} step {}", name, w, h, i);
                if i == CHUNKS / 2 {
                    let r = Resize::new(rw, rh);
                    server_to_host.extend(server.act_action(UserAction::Resize(&r)));
                    resize_rec(&mut server_log, rw, rh);
                }
                server_to_host.extend(server.act(&data[start..end]));
                bytes_rec(&mut server_log, &data[start..end]);
                start = end;

                assert!(
                    server.clone() == server,
                    "{}: a copy must equal its source",
                    at
                );
                let diff = server.diff_from(&prev);
                let msg = HostMessage::decode(diff.as_slice()).expect("diff decodes");
                for inst in &msg.instruction {
                    if let Some(hb) = &inst.hostbytes {
                        bytes_rec(&mut client_log, hb.hoststring.as_deref().unwrap_or(&[]));
                    } else if let Some(r) = &inst.resize {
                        resize_rec(
                            &mut client_log,
                            r.width.unwrap_or(0) as usize,
                            r.height.unwrap_or(0) as usize,
                        );
                    }
                }
                if i == CHUNKS / 2 {
                    let r = msg.instruction.iter().find_map(|x| x.resize.as_ref());
                    assert_eq!(
                        r.map(|r| (r.width, r.height)),
                        Some((Some(rw as i32), Some(rh as i32))),
                        "{}: diff after the resize must carry it",
                        at
                    );
                    resizes_seen += 1;
                }
                client.apply_string(&diff).expect("server diff applies");
                steps += 1;
                let (ss, cs) = (screen(server.get_fb()), screen(client.get_fb()));
                if ss != cs {
                    // Upstream's own server/client pair can show a transient
                    // difference (e.g. new_frame backspacing from a believed
                    // column 80 when the real cursor waits at 79 to wrap).
                    // Accept it only if both sides are exactly what upstream
                    // holds after the same input: ref-term replays of the
                    // server's input and of the bytes the client applied.
                    let args = ["steps".to_string(), w.to_string(), h.to_string()];
                    let up_server = strip_gen(&String::from_utf8_lossy(&run(
                        &ref_term,
                        &args,
                        &server_log,
                    )));
                    let up_client = strip_gen(&String::from_utf8_lossy(&run(
                        &ref_term,
                        &args,
                        &client_log,
                    )));
                    if up_server == full_dump(server.get_fb(), &server_to_host)
                        && up_client == full_dump(client.get_fb(), &[])
                    {
                        upstream_divergences += 1;
                        println!(
                            "upstream-identical divergence {}: {}",
                            at,
                            first_difference(ss.as_bytes(), cs.as_bytes()).replace('\n', " ")
                        );
                    } else {
                        check(&mut mismatches, &at, &ss, &cs);
                    }
                }
                assert_eq!(client.get_echo_ack(), server.get_echo_ack());
                prev = server.clone();
            }

            let at = format!("{} {}x{}", name, w, h);
            let want_server = strip_gen(&String::from_utf8_lossy(&run(
                &ref_term,
                &["steps".into(), w.to_string(), h.to_string()],
                &server_log,
            )));
            check(
                &mut mismatches,
                &format!("{} server vs ref-term", at),
                &want_server,
                &full_dump(server.get_fb(), &server_to_host),
            );
            let want_client = strip_gen(&String::from_utf8_lossy(&run(
                &ref_term,
                &["steps".into(), w.to_string(), h.to_string()],
                &client_log,
            )));
            check(
                &mut mismatches,
                &format!("{} client vs ref-term replay of the diffs", at),
                &want_client,
                &full_dump(client.get_fb(), &[]),
            );

            // init_diff into a fresh client: init_diff is relative to a blank
            // terminal of the server's current size, so the client starts there.
            let mut fresh = Complete::new(
                server.get_fb().ds.get_width() as usize,
                server.get_fb().ds.get_height() as usize,
            );
            fresh
                .apply_string(&server.init_diff())
                .expect("init_diff applies");
            check(
                &mut mismatches,
                &format!("{} init_diff", at),
                &screen(server.get_fb()),
                &screen(fresh.get_fb()),
            );
        }
    }

    println!(
        "files={} steps={} upstream_identical_divergences={} mismatches={}",
        files.len(),
        steps,
        upstream_divergences,
        mismatches
    );
    assert_eq!(resizes_seen, files.len() * SIZES.len());
    assert_eq!(mismatches, 0, "statesync round trip FAILED");
}

#[test]
fn userstream_roundtrip() {
    let mut rng = Rng(42);
    let mut full = UserStream::new();
    let resize_at: Vec<usize> = (0..5).map(|k| 20 + k * 40).collect();
    for i in 0..200 {
        if resize_at.contains(&i) {
            full.push_back_resize(Resize::new(
                (rng.next_u64() % 300 + 1) as usize,
                (rng.next_u64() % 100 + 1) as usize,
            ));
        }
        full.push_back_userbyte(UserByte::new((rng.next_u64() % 256) as u8));
    }
    assert_eq!(full.size(), 205);

    let mut older = UserStream::new();
    for e in full.events().iter().take(97) {
        match e.r#type {
            UserEventType::UserByteType => older.push_back_userbyte(e.userbyte),
            UserEventType::ResizeType => older.push_back_resize(e.resize),
        }
    }

    // diff_from decodes into exactly the newer events, keystrokes coalesced.
    let diff = full.diff_from(&older);
    let msg = UserMessage::decode(diff.as_slice()).expect("UserMessage decodes");
    let mut got = UserStream::new();
    let mut runs = 0;
    for inst in &msg.instruction {
        if let Some(k) = &inst.keystroke {
            runs += 1;
            for &b in k.keys.as_deref().unwrap_or(&[]) {
                got.push_back_userbyte(UserByte::new(b));
            }
        } else if let Some(r) = &inst.resize {
            got.push_back_resize(Resize::new(
                r.width.unwrap_or(0) as usize,
                r.height.unwrap_or(0) as usize,
            ));
        }
    }
    let newer: Vec<_> = full.events().iter().skip(97).copied().collect();
    assert_eq!(got.events().iter().copied().collect::<Vec<_>>(), newer);
    let resizes_after = resize_at.iter().filter(|&&i| i + 1 > 97 - 3).count();
    assert!(
        runs <= resizes_after + 1,
        "keystrokes between resizes must coalesce"
    );

    // apply_string reproduces equality.
    let mut applied = older.clone();
    applied.apply_string(&diff).expect("diff applies");
    assert!(applied == full);
    let mut from_init = UserStream::new();
    from_init
        .apply_string(&full.init_diff())
        .expect("init_diff applies");
    assert!(from_init == full);

    // subtract leaves the tail.
    let mut tail = full.clone();
    tail.subtract(&older);
    assert_eq!(tail.events().iter().copied().collect::<Vec<_>>(), newer);

    // An instruction with only field 4 (a length-delimited blob, like a future
    // extension) is ignored; a keystroke after it still applies.
    let unknown_inst = [0x22u8, 0x03, b'a', b'b', b'c'];
    let keystroke_inst = [0x12u8, 0x03, 0x22, 0x01, b'x'];
    let mut bytes = vec![0x0a, unknown_inst.len() as u8];
    bytes.extend_from_slice(&unknown_inst);
    let mut with_key = bytes.clone();
    with_key.extend_from_slice(&[0x0a, keystroke_inst.len() as u8]);
    with_key.extend_from_slice(&keystroke_inst);
    let mut s = full.clone();
    s.apply_string(&bytes)
        .expect("unknown instruction is not an error");
    assert!(s == full, "unknown instruction must add no event");
    s.apply_string(&with_key).expect("keystroke applies");
    assert_eq!(s.size(), full.size() + 1);
    assert_eq!(s.events().back().map(|e| e.userbyte.c), Some(b'x'));

    // get_action through Complete::act(Action): SS3 cursor keys become CSI
    // when the host is not in application cursor-key mode.
    let mut keys = UserStream::new();
    for &b in b"\x1bOAq" {
        keys.push_back_userbyte(UserByte::new(b));
    }
    let mut term = Complete::new(80, 24);
    let mut out = Vec::new();
    for i in 0..keys.size() {
        out.extend(term.act_action(keys.get_action(i)));
    }
    assert_eq!(out, b"\x1b[Aq");
    println!(
        "userstream events={} diff_instructions={}",
        full.size(),
        msg.instruction.len()
    );
}

/// `Transport::recv` on the next datagram, waiting up to 2 s for it to arrive.
fn recv_next<M: State, R: State>(t: &mut Transport<M, R>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    loop {
        match t.recv() {
            Err(Error::Network(e)) if e.function == "No packet received" => {
                assert!(start.elapsed().as_secs() < 2, "datagram never arrived");
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            r => return r,
        }
    }
}

/// A peer that holds the session key and sends hand-made instructions.
struct Peer {
    conn: Connection,
    fragmenter: Fragmenter,
    mtu: usize,
}

impl Peer {
    fn new(key: &str, port: &str) -> Peer {
        let conn = Connection::new_client(key, "127.0.0.1", port).expect("peer connection");
        // size_t arithmetic in upstream; the MTU is well above the overheads.
        let mtu = (conn.get_mtu() - Connection::ADDED_BYTES - Session::ADDED_BYTES as i32) as usize;
        Peer {
            conn,
            fragmenter: Fragmenter::new(),
            mtu,
        }
    }

    fn send(&mut self, old: u64, new: u64, diff: Vec<u8>) {
        let inst = transport_buffers::Instruction {
            protocol_version: Some(2),
            old_num: Some(old),
            new_num: Some(new),
            ack_num: Some(0),
            throwaway_num: Some(0),
            diff: Some(diff),
            chaff: Some(Vec::new()),
        };
        for f in self
            .fragmenter
            .make_fragments(&inst, self.mtu)
            .expect("fragments")
        {
            self.conn.send(&f.tostring()).expect("send");
        }
    }
}

fn user_diff(inst: client_buffers::Instruction) -> Vec<u8> {
    UserMessage {
        instruction: vec![inst],
    }
    .encode_to_vec()
}

fn host_diff(inst: host_buffers::Instruction) -> Vec<u8> {
    HostMessage {
        instruction: vec![inst],
    }
    .encode_to_vec()
}

/// PLAN.md 9.8: a keyed peer sending a well-formed, authenticated fragment
/// whose diff the state cannot apply gets an `Err` from `recv` (upstream
/// aborts); the packet is dropped and the receiver keeps working.
#[test]
fn malformed_diffs_through_transport() {
    // Server direction: diffs of a UserStream.
    let mut server: Transport<Complete, UserStream> = Transport::new_server(
        &Complete::new(80, 24),
        &UserStream::new(),
        Some("127.0.0.1"),
        None,
    )
    .expect("server transport");
    let port = server.port().expect("port");
    let mut peer = Peer::new(&server.get_key(), &port);
    let bad_user_diffs = [
        b"\xff\xff\xff\xff".to_vec(),
        user_diff(client_buffers::Instruction {
            resize: Some(client_buffers::ResizeMessage {
                width: Some(0),
                height: Some(24),
            }),
            ..Default::default()
        }),
        user_diff(client_buffers::Instruction {
            resize: Some(client_buffers::ResizeMessage {
                width: Some(80),
                height: Some(-1),
            }),
            ..Default::default()
        }),
    ];
    // Each malformed send uses its own new_num: the Fragmenter (as upstream)
    // asserts that one (old, new) pair always carries the same diff.
    for (i, diff) in bad_user_diffs.into_iter().enumerate() {
        peer.send(0, 100 + i as u64, diff);
        assert!(matches!(recv_next(&mut server), Err(Error::Crypto(_))));
        assert_eq!(
            server.get_remote_state_num(),
            0,
            "malformed diff added a state"
        );
    }
    peer.send(
        0,
        1,
        user_diff(client_buffers::Instruction {
            keystroke: Some(client_buffers::Keystroke {
                keys: Some(b"x".to_vec()),
            }),
            ..Default::default()
        }),
    );
    assert!(recv_next(&mut server).is_ok());
    assert_eq!(server.get_remote_state_num(), 1);
    let got = &server.get_latest_remote_state().state;
    assert_eq!(got.size(), 1);
    assert_eq!(got.events()[0].userbyte.c, b'x');

    // Client direction: diffs of a Complete.
    let mut client: Transport<UserStream, Complete> = Transport::new_server(
        &UserStream::new(),
        &Complete::new(80, 24),
        Some("127.0.0.1"),
        None,
    )
    .expect("client-side transport");
    let port = client.port().expect("port");
    let mut peer = Peer::new(&client.get_key(), &port);
    let hostbytes = |b: &[u8]| {
        host_diff(host_buffers::Instruction {
            hostbytes: Some(host_buffers::HostBytes {
                hoststring: Some(b.to_vec()),
            }),
            ..Default::default()
        })
    };
    let echoack = |n: u64| {
        host_diff(host_buffers::Instruction {
            echoack: Some(host_buffers::EchoAck {
                echo_ack_num: Some(n),
            }),
            ..Default::default()
        })
    };
    // valid: state 1 with echo ack 5
    peer.send(0, 1, echoack(5));
    assert!(recv_next(&mut client).is_ok());
    assert_eq!(client.get_remote_state_num(), 1);
    let bad_host_diffs = [
        b"\x0a\x05\x12\x03\xff".to_vec(), // truncated nested message
        hostbytes(b"\x1b[6n"),            // a query: the client would have to reply
        echoack(3),                       // echo ack going backwards from 5
        host_diff(host_buffers::Instruction {
            resize: Some(host_buffers::ResizeMessage {
                width: Some(-5),
                height: Some(10),
            }),
            ..Default::default()
        }),
    ];
    for (i, diff) in bad_host_diffs.into_iter().enumerate() {
        peer.send(1, 100 + i as u64, diff);
        assert!(matches!(recv_next(&mut client), Err(Error::Crypto(_))));
        assert_eq!(
            client.get_remote_state_num(),
            1,
            "malformed diff added a state"
        );
    }
    peer.send(1, 2, hostbytes(b"hi"));
    assert!(recv_next(&mut client).is_ok());
    assert_eq!(client.get_remote_state_num(), 2);
    let fb = client.get_latest_remote_state().state.get_fb();
    assert_eq!(fb.get_cell(0, 0).debug_contents_bytes(), b"h");
    assert_eq!(fb.get_cell(0, 1).debug_contents_bytes(), b"i");
    assert_eq!(client.get_latest_remote_state().state.get_echo_ack(), 5);
    println!(
        "malformed diffs rejected: user=3 host=4; both receivers accepted the next valid diff"
    );
}
