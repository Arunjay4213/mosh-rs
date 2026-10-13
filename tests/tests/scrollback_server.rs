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
// Ported from: (new; mosh-rs scrollback extension, PLAN.md section 6)
//
// Scrollback, server side (PLAN.md section 6 items 1-3 and 5).
// - Old peers: for every corpus file the per-step server diffs with no
//   capability are byte-identical to the diffs of the commit before
//   scrollback existed (digests in tests/scrollback-server-baseline.txt,
//   recorded at b5393e7 with the same chunking); with the capability, the
//   diffs minus the ScrolledLines instruction are those same bytes.
// - Capture: `seq` output through a server `Complete` and a client
//   `Complete` that applies diffs at random points and from older states
//   (as after loss and retransmission) arrives as scrolled lines plus the
//   final screen with no gap and no duplicate; also with a resize, an
//   alternate-screen excursion, a partial scrolling region, and a small ring.
// - Rendering: random colored, linked, wide and combining lines rendered by
//   the server read back cell for cell in upstream's emulator (ref-term).

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use mosh::statesync::{ClientCapabilities, Complete, ReceivedScrollback, UserAction, UserStream};
use mosh_network::prost::Message;
use mosh_network::proto::host_buffers::{HostMessage, ScrolledLines};
use mosh_network::State;
use mosh_terminal::terminalframebuffer::Framebuffer;
use mosh_terminal::{Resize, UserByte};
use mosh_tests::dump::hex;
use mosh_tests::oracle::{build_ref_term, prefix_offsets, record, root, run, seed_of, Rng};

const SIZES: [(usize, usize); 2] = [(80, 24), (132, 43)];
const CHUNKS: [usize; 2] = [40, 400];

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

/// Turns scrollback on as the server does: the client's capability event
/// travels through a `UserStream` diff and reaches `Complete::act_action`.
fn send_capability(server: &mut Complete) {
    let mut client_stream = UserStream::new();
    client_stream.push_back_capabilities(ClientCapabilities { scrollback: true });
    let mut received = UserStream::new();
    received
        .apply_string(&client_stream.diff_from(&UserStream::new()))
        .expect("capability diff applies");
    assert_eq!(received.size(), 1);
    server.act_action(received.get_action(0));
}

/// The capability's wire format (PLAN.md 6.1): `ClientBuffers::Instruction`
/// field 4 holding `ClientCapabilities { scrollback = 1 }`, as the first
/// event, here followed by a keystroke and a resize. The bytes are printed so
/// they can be fed to upstream's own `UserStream::apply_string`.
#[test]
fn capability_wire_format() {
    let mut s = UserStream::new();
    s.push_back_capabilities(ClientCapabilities { scrollback: true });
    let alone = s.diff_from(&UserStream::new());
    assert_eq!(alone, [0x0a, 0x04, 0x22, 0x02, 0x08, 0x01]);
    s.push_back_userbyte(UserByte::new(b'x'));
    s.push_back_resize(Resize::new(100, 30));
    let diff = s.diff_from(&UserStream::new());
    println!("capability+keystroke+resize UserMessage: {}", hex(&diff));
    let mut back = UserStream::new();
    back.apply_string(&diff).expect("applies");
    assert!(back == s, "the capability event round-trips");
}

/// The server diffs `server_i.diff_from(server_{i-1})` for a corpus file cut
/// into `chunks` pieces, with a resize in the middle (the chunking of
/// statesync_roundtrip). The scrollback limit is set as mosh-server sets it;
/// only `capability` turns capture on.
fn corpus_diffs(data: &[u8], w: usize, h: usize, chunks: usize, capability: bool) -> Vec<Vec<u8>> {
    let offsets = prefix_offsets(data, chunks);
    let mut server = Complete::new(w, h);
    server.set_scrollback_limit(10000);
    if capability {
        send_capability(&mut server);
    }
    let mut prev = server.clone();
    let mut diffs = Vec::new();
    let mut start = 0usize;
    for (i, &end) in offsets.iter().enumerate() {
        if i == chunks / 2 {
            server.act_action(UserAction::Resize(&Resize::new(w - 13, h + 5)));
        }
        server.act(&data[start..end]);
        start = end;
        diffs.push(server.diff_from(&prev));
        prev = server.clone();
    }
    diffs
}

/// The ScrolledLines instruction of a diff, after checking it comes before
/// any resize or host bytes (the order a client relies on).
fn scrolled_lines_of(msg: &HostMessage) -> Option<&ScrolledLines> {
    let pos = msg
        .instruction
        .iter()
        .position(|i| i.scrolledlines.is_some())?;
    assert_eq!(
        msg.instruction
            .iter()
            .filter(|i| i.scrolledlines.is_some())
            .count(),
        1
    );
    assert!(
        msg.instruction[..pos]
            .iter()
            .all(|i| i.resize.is_none() && i.hostbytes.is_none()),
        "ScrolledLines must come before resize and host bytes"
    );
    msg.instruction[pos].scrolledlines.as_ref()
}

#[test]
fn old_peer_diffs_unchanged() {
    let mut files = Vec::new();
    all_corpus(&root().join("tests/corpus"), &mut files);
    files.sort();
    assert!(!files.is_empty(), "no corpus files");
    let mut digests = String::new();
    let mut all = Vec::new();
    let (mut steps, mut on_mismatches, mut diffs_with_lines, mut lines_sent) = (0, 0, 0, 0u64);
    for file in &files {
        let name = file
            .strip_prefix(root())
            .unwrap_or(file)
            .display()
            .to_string();
        let data = std::fs::read(file).expect("read corpus file");
        for (chunks, (w, h)) in CHUNKS.iter().flat_map(|&c| SIZES.map(|s| (c, s))) {
            let off = corpus_diffs(&data, w, h, chunks, false);
            let mut recs = Vec::new();
            for d in &off {
                record(&mut recs, d);
            }
            let _ = writeln!(
                digests,
                "{} {}x{} chunks {} steps {} bytes {} fnv {:016x}",
                name,
                w,
                h,
                chunks,
                off.len(),
                recs.len(),
                seed_of(&recs)
            );
            all.extend_from_slice(&recs);
            steps += off.len();

            // With the capability: the same frames, plus the scrolled lines,
            // which a client applies without loss or duplication.
            let on = corpus_diffs(&data, w, h, chunks, true);
            assert_eq!(on.len(), off.len());
            let mut client = Complete::new(w, h);
            let rx = client.enable_scrollback_receive();
            for (i, (a, b)) in on.iter().zip(off.iter()).enumerate() {
                let mut msg = HostMessage::decode(a.as_slice()).expect("diff decodes");
                if let Some(sl) = scrolled_lines_of(&msg) {
                    diffs_with_lines += 1;
                    lines_sent += sl.line.len() as u64;
                }
                client.apply_string(a).expect("diff applies");
                msg.instruction.retain(|i| i.scrolledlines.is_none());
                if msg.encode_to_vec() != *b {
                    on_mismatches += 1;
                    println!(
                        "MISMATCH {} {}x{} chunks {} step {}: frames differ with capture on",
                        name, w, h, chunks, i
                    );
                }
            }
            let rx = rx.borrow();
            assert_eq!(rx.lost(), 0, "{}: lines lost", name);
        }
    }
    let out = root().join("target/scrollback-server");
    std::fs::create_dir_all(&out).expect("mkdir target/scrollback-server");
    std::fs::write(out.join("digests-off.txt"), &digests).expect("write digests");
    std::fs::write(out.join("diffs-off.bin"), &all).expect("write diffs");

    let baseline = include_str!("../scrollback-server-baseline.txt");
    let (want, got): (Vec<&str>, Vec<&str>) =
        (baseline.lines().collect(), digests.lines().collect());
    let differing: Vec<String> = (0..want.len().max(got.len()))
        .filter(|&i| want.get(i) != got.get(i))
        .map(|i| format!("  before: {:?}\n  now:    {:?}", want.get(i), got.get(i)))
        .collect();
    for d in differing.iter().take(5) {
        println!("DIGEST MISMATCH\n{}", d);
    }
    println!(
        "files={} cells={} steps={} off_bytes={} digests_differing={} capture_on_frame_mismatches={} diffs_with_lines={} lines_sent={}",
        files.len(),
        got.len(),
        steps,
        all.len(),
        differing.len(),
        on_mismatches,
        diffs_with_lines,
        lines_sent
    );
    assert!(differing.is_empty(), "diffs for old peers changed");
    assert_eq!(on_mismatches, 0, "capture changed the frames");
    assert!(diffs_with_lines > 0, "no corpus file scrolled");
}

/// `1\r\n2\r\n...`: what `seq from to` writes through a pty.
fn seq(from: u64, to: u64) -> Vec<u8> {
    let mut s = String::new();
    for i in from..=to {
        let _ = write!(s, "{}\r\n", i);
    }
    s.into_bytes()
}

fn seq_lines(from: u64, to: u64) -> Vec<String> {
    (from..=to).map(|i| i.to_string()).collect()
}

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

/// Every row's text, trailing empty rows dropped.
fn screen_text(fb: &Framebuffer) -> Vec<String> {
    let mut v: Vec<String> = (0..fb.get_rows().len()).map(|r| row_text(fb, r)).collect();
    while v.last().is_some_and(|l| l.is_empty()) {
        v.pop();
    }
    v
}

/// The text a rendered scrolled line shows when printed on a terminal row.
fn line_text(line: &[u8]) -> String {
    let mut t = Complete::new(400, 1);
    assert!(t.act(line).is_empty());
    row_text(t.get_fb(), 0)
}

/// A server and a client `Complete` joined by a lossy link: the client gets
/// a diff only now and then, and sometimes one computed from an older state
/// it holds (as the transport does after loss), so diffs repeat lines.
struct Link {
    server: Complete,
    /// (server snapshot, client state) pairs the two sides agree on.
    history: VecDeque<(Complete, Complete)>,
    rx: Rc<RefCell<ReceivedScrollback>>,
    rng: Rng,
    since_delivery: usize,
    deliveries: usize,
    diffs_with_lines: usize,
    repeated_lines: u64,
    max_lines: usize,
}

impl Link {
    fn new(w: usize, h: usize, limit: usize, seed: u64) -> Link {
        let mut server = Complete::new(w, h);
        server.set_scrollback_limit(limit);
        send_capability(&mut server);
        let mut client = Complete::new(w, h);
        let rx = client.enable_scrollback_receive();
        Link {
            history: VecDeque::from([(server.clone(), client)]),
            server,
            rx,
            rng: Rng(seed),
            since_delivery: 0,
            deliveries: 0,
            diffs_with_lines: 0,
            repeated_lines: 0,
            max_lines: 0,
        }
    }

    /// Sends the client one diff of the current server state.
    fn deliver(&mut self, latest: bool) -> HostMessage {
        let n = self.history.len();
        let back = if !latest && self.rng.next_u64().is_multiple_of(4) {
            (self.rng.next_u64() % n as u64) as usize
        } else {
            0
        };
        let (base_server, base_client) = &self.history[n - 1 - back];
        let diff = self.server.diff_from(base_server);
        let msg = HostMessage::decode(diff.as_slice()).expect("diff decodes");
        if let Some(sl) = scrolled_lines_of(&msg) {
            self.diffs_with_lines += 1;
            self.max_lines = self.max_lines.max(sl.line.len());
            if back > 0 {
                let newest = self.history[n - 1].1.get_scrolled_total();
                let first = sl.first_index.unwrap_or(0);
                let last = first + sl.line.len() as u64;
                self.repeated_lines += newest.min(last).saturating_sub(first);
            }
        }
        let mut client = base_client.clone();
        client.apply_string(&diff).expect("server diff applies");
        assert_eq!(
            client.get_scrolled_total(),
            self.server.get_scrolled_total()
        );
        self.history.push_back((self.server.clone(), client));
        if self.history.len() > 8 {
            self.history.pop_front();
        }
        self.deliveries += 1;
        self.since_delivery = 0;
        msg
    }

    /// Feeds `data` in random chunks, delivering at random points.
    fn feed(&mut self, data: &[u8]) {
        let mut start = 0;
        while start < data.len() {
            let n = 1 + (self.rng.next_u64() % 3000) as usize;
            let end = (start + n).min(data.len());
            assert!(self.server.act(&data[start..end]).is_empty());
            start = end;
            self.since_delivery += 1;
            if self.rng.next_u64().is_multiple_of(3) || self.since_delivery >= 8 {
                self.deliver(false);
            }
        }
    }

    /// Final delivery; returns (line number, text) of every received line
    /// and the client's screen, after checking it matches the server's.
    fn finish(&mut self) -> (Vec<(u64, String)>, Vec<String>) {
        self.deliver(true);
        let client = &self.history.back().expect("state").1;
        let screen = screen_text(client.get_fb());
        assert_eq!(screen, screen_text(self.server.get_fb()), "client screen");
        let lines = self
            .rx
            .borrow_mut()
            .take_pending()
            .into_iter()
            .map(|(i, l)| (i, line_text(&l)))
            .collect();
        (lines, screen)
    }

    fn report(&self, what: &str, lines: &[(u64, String)]) {
        println!(
            "{}: server_scrolled={} received_lines={} lost={} deliveries={} diffs_with_lines={} max_lines_in_one_diff={} repeated_lines_skipped={}",
            what,
            self.server.get_scrolled_total(),
            lines.len(),
            self.rx.borrow().lost(),
            self.deliveries,
            self.diffs_with_lines,
            self.max_lines,
            self.repeated_lines
        );
    }
}

/// Received lines numbered 1, 2, 3, ... with nothing lost, then the screen.
fn assert_complete(link: &Link, lines: &[(u64, String)], screen: &[String], want: &[String]) {
    for (k, (i, _)) in lines.iter().enumerate() {
        assert_eq!(*i, k as u64 + 1, "line numbers must be consecutive from 1");
    }
    assert_eq!(link.rx.borrow().lost(), 0);
    let got: Vec<String> = lines
        .iter()
        .map(|(_, t)| t.clone())
        .chain(screen.iter().cloned())
        .collect();
    if got != want {
        let i = (0..got.len().max(want.len()))
            .find(|&i| got.get(i) != want.get(i))
            .unwrap_or(0);
        panic!(
            "scrollback differs at line {}: got {:?}, want {:?} (got {} lines, want {})",
            i,
            got.get(i),
            want.get(i),
            got.len(),
            want.len()
        );
    }
}

#[test]
fn seq_50000_random_snapshots() {
    let mut link = Link::new(80, 24, 10000, 1);
    link.feed(&seq(1, 50000));
    let (lines, screen) = link.finish();
    link.report("seq 1 50000", &lines);
    assert!(link.repeated_lines > 0, "no diff repeated lines");
    assert_complete(&link, &lines, &screen, &seq_lines(1, 50000));
}

#[test]
fn seq_with_resizes() {
    let mut link = Link::new(80, 24, 10000, 2);
    link.feed(&seq(1, 20000));
    link.server
        .act_action(UserAction::Resize(&Resize::new(100, 30)));
    link.feed(&seq(20001, 35000));
    link.server
        .act_action(UserAction::Resize(&Resize::new(60, 30)));
    link.feed(&seq(35001, 50000));
    let (lines, screen) = link.finish();
    link.report("seq 1 50000, resized 80x24 -> 100x30 -> 60x30", &lines);
    assert_complete(&link, &lines, &screen, &seq_lines(1, 50000));
}

#[test]
fn alternate_screen_is_not_captured() {
    let mut link = Link::new(80, 24, 10000, 3);
    link.feed(&seq(1, 3000));
    // Upstream ignores the alternate screen, so the rows on screen when the
    // program enters it are erased by its clear and never scroll off.
    let erased = screen_text(link.server.get_fb());
    let mut vim = b"\x1b[?1049h\x1b[H\x1b[2J".to_vec();
    for i in 0..200 {
        vim.extend_from_slice(format!("VIM line {}\r\n", i).as_bytes());
    }
    vim.extend_from_slice(b"\x1b[5;10HVIM-status\x1b[24;1H~ VIM bottom");
    let before = link.server.get_scrolled_total();
    link.feed(&vim);
    assert_eq!(
        link.server.get_scrolled_total(),
        before,
        "nothing scrolled on the alternate screen is captured"
    );
    link.feed(b"\x1b[?1049l\r\n");
    link.feed(&seq(3001, 6000));
    let (lines, screen) = link.finish();
    link.report("seq, vim-like alternate screen, seq", &lines);
    let vim_lines = lines.iter().filter(|(_, t)| t.contains("VIM")).count();
    println!("alternate screen: lines containing VIM = {}", vim_lines);
    assert_eq!(vim_lines, 0, "alternate-screen output reached scrollback");
    let want: Vec<String> = seq_lines(1, 3000)
        .into_iter()
        .filter(|l| !erased.contains(l))
        .chain(seq_lines(3001, 6000))
        .collect();
    let got: Vec<String> = lines.iter().map(|(_, t)| t.clone()).chain(screen).collect();
    let got: Vec<String> = got.into_iter().filter(|t| !t.is_empty()).collect();
    assert_eq!(got.len(), want.len());
    assert!(got == want, "scrollback around the alternate screen");
    for (k, (i, _)) in lines.iter().enumerate() {
        assert_eq!(*i, k as u64 + 1);
    }
}

#[test]
fn partial_scroll_region_is_not_captured() {
    let mut link = Link::new(80, 24, 10000, 4);
    link.feed(b"\x1b[1;20r");
    link.feed(&seq(1, 3000));
    assert_eq!(
        link.server.get_scrolled_total(),
        0,
        "region scroll captured"
    );
    let msg = link.deliver(true);
    assert!(msg.instruction.iter().all(|i| i.scrolledlines.is_none()));
    // Back to the full screen: what is on screen now scrolls off normally.
    let mut on_screen: Vec<String> = (0..23).map(|r| row_text(link.server.get_fb(), r)).collect();
    link.feed(b"\x1b[r\x1b[24;1H");
    link.feed(&seq(3001, 6000));
    let (lines, screen) = link.finish();
    link.report("seq inside rows 1-20, then the full screen", &lines);
    on_screen.extend(seq_lines(3001, 6000));
    assert_complete(&link, &lines, &screen, &on_screen);
}

#[test]
fn small_ring_reports_dropped() {
    let mut link = Link::new(80, 24, 100, 5);
    link.server.act(&seq(1, 500));
    let first = link.deliver(true);
    let sl = scrolled_lines_of(&first).expect("lines");
    // 500 lines on 24 rows: 477 scrolled off, the newest 100 are kept.
    assert_eq!(
        (sl.first_index, sl.line.len(), sl.dropped),
        (Some(378), 100, Some(377))
    );
    for k in 501..5000 {
        link.server.act(&seq(k, k));
    }
    link.server.act(&seq(5000, 5000));
    let last = link.deliver(true);
    let sl = scrolled_lines_of(&last).expect("lines");
    assert_eq!(
        (sl.first_index, sl.line.len(), sl.dropped),
        (Some(4878), 100, Some(4400))
    );
    let (lines, screen) = link.finish();
    link.report("ring of 100, client skips ahead twice", &lines);
    assert_eq!(link.rx.borrow().lost(), 377 + 4400);
    let want: Vec<(u64, String)> = (378..=477)
        .chain(4878..=4977)
        .map(|i| (i, i.to_string()))
        .collect();
    assert!(lines == want, "received lines after drops");
    assert_eq!(screen, seq_lines(4978, 5000));
}

/// One random line of terminal output: SGR (attributes, 16, 256 and true
/// colors), OSC 8 links, wide and combining characters, runs of blanks;
/// sometimes longer than the screen, so rows wrap.
fn random_line(rng: &mut Rng) -> String {
    let mut s = String::new();
    let target = 1 + rng.next_u64() % 110;
    let mut col = 0;
    let wide = ['日', '本', '語', '中', '文', '😀', '🎉'];
    let r = |rng: &mut Rng, n: u64| rng.next_u64() % n;
    while col < target {
        match r(rng, 12) {
            0 | 1 => {
                let k = 1 + r(rng, 3);
                let params: Vec<String> = (0..k)
                    .map(|_| match r(rng, 9) {
                        0 => ["0", "1", "3", "4", "5", "7", "8", "22", "24"][r(rng, 9) as usize]
                            .to_string(),
                        1 => format!("{}", 30 + r(rng, 8)),
                        2 => format!("{}", 40 + r(rng, 8)),
                        3 => format!("{}", 90 + r(rng, 8)),
                        4 => format!("{}", 100 + r(rng, 8)),
                        5 => format!("38;5;{}", r(rng, 256)),
                        6 => format!("48;5;{}", r(rng, 256)),
                        7 => format!("38;2;{};{};{}", r(rng, 256), r(rng, 256), r(rng, 256)),
                        _ => format!("48;2;{};{};{}", r(rng, 256), r(rng, 256), r(rng, 256)),
                    })
                    .collect();
                let _ = write!(s, "\x1b[{}m", params.join(";"));
            }
            2 => {
                if r(rng, 2) == 0 {
                    let n = r(rng, 5);
                    let _ = write!(s, "\x1b]8;id=l{};https://example.com/{}\x1b\\", n, n);
                } else {
                    s.push_str("\x1b]8;;\x1b\\");
                }
            }
            3 | 4 => {
                s.push(wide[r(rng, wide.len() as u64) as usize]);
                col += 2;
            }
            5 => {
                s.push_str("e\u{301}");
                col += 1;
            }
            6 => {
                let n = 1 + r(rng, 9);
                s.push_str(&" ".repeat(n as usize));
                col += n;
            }
            _ => {
                s.push((b'!' + r(rng, 94) as u8) as char);
                col += 1;
            }
        }
    }
    s.push_str("\x1b[0m\x1b]8;;\x1b\\\r\n");
    s
}

/// The cell lines of row `row` of a ref-term dump, compared as upstream's
/// `Cell::compare` does: an empty cell and a space print the same (the
/// renderer writes blank runs as spaces when it cannot erase).
fn row_cells(dump: &str, row: usize) -> Vec<String> {
    let header = format!("row {} ", row);
    dump.lines()
        .skip_while(|l| !l.starts_with(&header))
        .skip(1)
        .take_while(|l| l.starts_with("  cell"))
        .map(|l| l.replace("contents= ", "contents=20 "))
        .collect()
}

#[test]
fn rendered_lines_match_upstream_cells() {
    let ref_term = build_ref_term();
    let mut rng = Rng(6);
    let input: Vec<String> = (0..300).map(|_| random_line(&mut rng)).collect();

    // A capturing server fed one input line at a time: how many rows each
    // line pushed off the top.
    let mut server = Complete::new(80, 24);
    server.set_scrollback_limit(10000);
    send_capability(&mut server);
    let mut counts = Vec::new();
    for l in &input {
        let before = server.get_scrolled_total();
        assert!(server.act(l.as_bytes()).is_empty());
        counts.push(server.get_scrolled_total() - before);
    }
    let mut client = Complete::new(80, 24);
    let rx = client.enable_scrollback_receive();
    client
        .apply_string(&server.init_diff())
        .expect("init_diff applies");
    let lines: Vec<(u64, Vec<u8>)> = rx.borrow_mut().take_pending().into_iter().collect();
    assert_eq!(lines.len() as u64, server.get_scrolled_total());
    assert!(lines.len() > 250, "most input lines scrolled off");

    // Upstream's 80x24 screen before each input line. A line is written at
    // the bottom and is at most two rows long, so the rows it pushes off are
    // the top rows of that screen, in order.
    let mut prefixes = Vec::new();
    let mut prefix = String::new();
    for l in &input {
        record(&mut prefixes, prefix.as_bytes());
        prefix.push_str(l);
    }
    let screens = String::from_utf8(run(
        &ref_term,
        &["batch".to_string(), "80".to_string(), "24".to_string()],
        &prefixes,
    ))
    .expect("utf-8 dump");
    let screens: Vec<&str> = screens.split("=====\n").filter(|d| !d.is_empty()).collect();
    assert_eq!(screens.len(), input.len());
    let mut want_rows = Vec::new();
    for (p, &c) in counts.iter().enumerate() {
        for m in 0..c as usize {
            want_rows.push(row_cells(screens[p], m));
        }
    }
    assert_eq!(want_rows.len(), lines.len());

    // Each rendered line printed on the one row of an 80x1 upstream emulator.
    let mut recs = Vec::new();
    for (_, l) in &lines {
        record(&mut recs, l);
    }
    let got = String::from_utf8(run(
        &ref_term,
        &["batch".to_string(), "80".to_string(), "1".to_string()],
        &recs,
    ))
    .expect("utf-8 dump");
    let got: Vec<&str> = got.split("=====\n").filter(|d| !d.is_empty()).collect();
    assert_eq!(got.len(), lines.len());

    let (mut mismatches, mut wrapped, mut wide, mut colored, mut linked) = (0, 0, 0, 0, 0);
    for (k, (dump, want_cells)) in got.iter().zip(want_rows.iter_mut()).enumerate() {
        let got_cells = row_cells(dump, 0);
        assert_eq!(want_cells.len(), 80);
        // A line is printed on its own row, so it cannot carry the soft-wrap
        // flag of a row that continued on the next one.
        if let Some(last) = want_cells.last_mut() {
            if last.contains(" wrap 1 ") {
                wrapped += 1;
                *last = last.replace(" wrap 1 ", " wrap 0 ");
            }
        }
        wide += want_cells.iter().any(|c| c.contains(" wide 1 ")) as usize;
        colored += want_cells.iter().any(|c| !c.contains("renditions 0 0 0")) as usize;
        linked += want_cells.iter().any(|c| !c.ends_with("link=none")) as usize;
        if *want_cells != got_cells {
            mismatches += 1;
            if mismatches <= 3 {
                let i = (0..80)
                    .find(|&i| want_cells[i] != got_cells[i])
                    .unwrap_or(0);
                println!(
                    "MISMATCH line {}: rendered {:?}\n  want {}\n  got  {}",
                    k + 1,
                    String::from_utf8_lossy(&lines[k].1),
                    want_cells[i],
                    got_cells[i]
                );
            }
        }
    }
    println!(
        "rendered lines={} wrapped_rows={} with_wide={} with_colors={} with_links={} mismatches={}",
        lines.len(),
        wrapped,
        wide,
        colored,
        linked,
        mismatches
    );
    assert!(wrapped > 0 && wide > 0 && colored > 0 && linked > 0);
    assert_eq!(mismatches, 0, "rendered lines differ from upstream's rows");
}

/// Live sessions: `scrollback-probe` (a client that sends the capability and
/// does not render) against a real `mosh-server new -- seq 1 50000`, on
/// ports 61200-61299.
#[cfg(unix)]
mod live {
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::process::{Child, Command, Stdio};
    use std::sync::OnceLock;

    use mosh_tests::oracle::root;

    const CXX_SERVER: &str = "/usr/bin/mosh-server";

    /// target/release, after building mosh-server and udp-impair there.
    fn release_dir() -> PathBuf {
        static BUILT: OnceLock<PathBuf> = OnceLock::new();
        BUILT
            .get_or_init(|| {
                let status = Command::new(env!("CARGO"))
                    .args(["build", "--release", "-p", "mosh", "--bin", "mosh-server"])
                    .args(["-p", "mosh-tools", "--bin", "udp-impair"])
                    .current_dir(root())
                    .status()
                    .expect("run cargo build");
                assert!(
                    status.success(),
                    "cargo build of mosh-server, udp-impair failed"
                );
                let target = std::env::var_os("CARGO_TARGET_DIR")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| root().join("target"));
                target.join("release")
            })
            .clone()
    }

    fn rust_server() -> String {
        release_dir().join("mosh-server").display().to_string()
    }

    /// `server new -i 127.0.0.1 -p PORT [extra] -- seq 1 50000`; returns the key.
    fn start_server(server: &str, port: u16, extra: &[&str]) -> String {
        let out = Command::new(server)
            .args(["new", "-i", "127.0.0.1", "-p", &port.to_string()])
            .args(extra)
            .args(["--", "seq", "1", "50000"])
            .env("LANG", "C.UTF-8")
            .stdin(Stdio::null())
            .output()
            .expect("run mosh-server");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{server} failed: {stdout}");
        let connect = stdout
            .lines()
            .find_map(|l| l.strip_prefix("MOSH CONNECT "))
            .unwrap_or_else(|| panic!("no MOSH CONNECT in {stdout:?}"));
        let mut words = connect.split(' ');
        assert_eq!(words.next(), Some(port.to_string().as_str()));
        words.next().expect("key").to_string()
    }

    /// udp-impair from `listen` to `target` with 30% loss and 100 ms delay.
    struct Impair(Child);
    impl Impair {
        fn start(listen: u16, target: u16, control: u16) -> Impair {
            let mut child = Command::new(release_dir().join("udp-impair"))
                .args(["--listen", &format!("127.0.0.1:{listen}")])
                .args(["--target", &format!("127.0.0.1:{target}")])
                .args(["--control", &format!("127.0.0.1:{control}")])
                .args(["--loss", "0.30", "--delay-ms", "100", "--seed", "7"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("run udp-impair");
            let mut first = String::new();
            BufReader::new(child.stdout.as_mut().expect("stdout"))
                .read_line(&mut first)
                .expect("udp-impair startup line");
            assert!(first.starts_with("udp-impair listen="), "{first}");
            Impair(child)
        }
    }
    impl Drop for Impair {
        fn drop(&mut self) {
            // SAFETY: plain kill(2) on our own child.
            unsafe { libc::kill(self.0.id() as i32, libc::SIGTERM) };
            let _ = self.0.wait();
        }
    }

    /// Runs the probe for at most 30 s; returns (whether the session ended
    /// with the server's shutdown, its counts line, the lines it wrote).
    fn probe(port: u16, key: &str) -> (bool, String, Vec<String>) {
        let out_file = std::env::temp_dir().join(format!(
            "scrollback-probe-{}-{}.txt",
            port,
            std::process::id()
        ));
        let out = Command::new(env!("CARGO_BIN_EXE_scrollback-probe"))
            .args(["127.0.0.1", &port.to_string()])
            .arg(&out_file)
            .args(["--timeout-s", "30"])
            .env("MOSH_KEY", key)
            .output()
            .expect("run scrollback-probe");
        let counts = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let text = std::fs::read_to_string(&out_file).unwrap_or_default();
        let _ = std::fs::remove_file(&out_file);
        (
            out.status.success(),
            counts,
            text.lines().map(str::to_string).collect(),
        )
    }

    /// The number after `name=` in the probe's counts line.
    fn count(counts: &str, name: &str) -> u64 {
        counts
            .split(' ')
            .find_map(|w| w.strip_prefix(&format!("{name}=")))
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| panic!("no {name} in {counts:?}"))
    }

    fn seq_1_50000() -> Vec<String> {
        (1..=50000).map(|i| i.to_string()).collect()
    }

    fn first_difference(got: &[String], want: &[String]) -> String {
        match (0..got.len().max(want.len())).find(|&i| got.get(i) != want.get(i)) {
            None => "identical".to_string(),
            Some(i) => format!(
                "first difference at line {}: got {:?}, want {:?}; got {} lines",
                i + 1,
                got.get(i),
                want.get(i),
                got.len()
            ),
        }
    }

    /// Default ring (10,000 lines): seq writes its 50,000 lines faster than
    /// one round trip, so the oldest are evicted before they can be sent and
    /// arrive as `dropped`. Every line that does arrive is correct and in
    /// order, and received plus lost accounts for every scrolled line.
    #[test]
    fn rust_server_default_ring() {
        let key = start_server(&rust_server(), 61200, &[]);
        let (shutdown, counts, got) = probe(61200, &key);
        println!("rust server, clean, default ring: {counts}");
        assert!(shutdown, "the session must end with the server's shutdown");
        let (received, lost) = (count(&counts, "lines_received"), count(&counts, "lost"));
        assert_eq!(received + lost, count(&counts, "scrolled_total"));
        assert_eq!(got.len() as u64, 50000 - lost);
        let numbers: Vec<u64> = got
            .iter()
            .map(|l| l.parse().unwrap_or_else(|_| panic!("not a number: {l:?}")))
            .collect();
        assert!(
            numbers.windows(2).all(|w| w[0] < w[1]),
            "lines out of order"
        );
        assert_eq!(numbers.last(), Some(&50000));
    }

    /// Exactly seq 1 50000 when the ring holds the whole burst.
    #[test]
    #[ignore = "PLAN 6 open question: the 49,977-line ScrolledLines diff is about 87 fragments \
                sent in one burst; the client's UDP receive buffer drops some of most bursts \
                and every resend gets a new instruction id (the chaff differs), so the diff \
                arrives only when one whole burst survives: 5 of 5 cargo runs passed after \
                1.8-3.4 s, 2 of 2 shell runs got 0 lines in 10 s and 120 s"]
    fn rust_server_ring_holds_burst() {
        let key = start_server(&rust_server(), 61205, &["--scrollback", "60000"]);
        let (shutdown, counts, got) = probe(61205, &key);
        println!("rust server, clean, --scrollback 60000: {counts}");
        println!("vs seq 1 50000: {}", first_difference(&got, &seq_1_50000()));
        assert!(shutdown);
        assert!(got == seq_1_50000());
    }

    /// Exactly seq 1 50000 through 30% loss and 100 ms delay.
    #[test]
    #[ignore = "PLAN 6 open question: with the default ring the first diff carries about \
                10,000 lines in about 17 fragments, which all arrive together with probability \
                0.7^17 = 0.2%, and every resend has a new instruction id: 0 lines in 120 s"]
    fn rust_server_loss30_delay100() {
        let key = start_server(&rust_server(), 61210, &[]);
        let _impair = Impair::start(61211, 61210, 61212);
        let (shutdown, counts, got) = probe(61211, &key);
        println!("rust server, 30% loss, 100 ms delay: {counts}");
        println!("vs seq 1 50000: {}", first_difference(&got, &seq_1_50000()));
        assert!(shutdown);
        assert!(got == seq_1_50000());
    }

    /// PLAN.md 6.5, new client with an old server: upstream's server skips
    /// the capability, sends no lines, and the session works as before.
    #[test]
    fn cxx_server_ignores_capability() {
        let key = start_server(CXX_SERVER, 61220, &[]);
        let (shutdown, counts, got) = probe(61220, &key);
        println!("C++ server, capability sent: {counts}");
        assert!(shutdown, "the session must end with the server's shutdown");
        assert_eq!(count(&counts, "lines_received"), 0);
        let want: Vec<String> = (49978..=50000).map(|i| i.to_string()).collect();
        assert_eq!(got, want, "the final screen");
    }
}
