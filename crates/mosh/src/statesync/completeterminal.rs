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
// Ported from: upstream/src/statesync/completeterminal.h, upstream/src/statesync/completeterminal.cc

//! The server-to-client state: a UTF-8 parser feeding an emulator, plus the
//! echo-ack bookkeeping, and its protobuf diff format
//! (`HostBuffers::HostMessage`, whose host bytes come from
//! `Display::new_frame`). Port of upstream `src/statesync/completeterminal.{h,cc}`.
//!
//! mosh-rs scrollback (PLAN.md section 6, 9.4), not in upstream. On the
//! server, once the client's capabilities ask for it (`act_action`), the
//! framebuffer collects the rows that scroll off the top (see
//! `Framebuffer::scroll`) and `act` renders them into a `ScrolledRing`, a
//! backlog of lines not yet delivered, shared by every copy of this state.
//! The state itself only carries `scrolled_total`, the number of lines
//! published so far. The server loop publishes (`publish_scrollback`) at
//! most `ScrolledRing::WINDOW` bytes of lines beyond what the client has
//! acknowledged, so every diff stays a few fragments long and history
//! trickles behind the screen; the screen itself is never held back.
//! `diff_from` sends the published lines numbered
//! `(existing.scrolled_total, self.scrolled_total]` as `ScrolledLines`
//! instructions placed after the echo ack and before the resize and host
//! bytes, so a client can print the lines first and then the new frame. On
//! the client, `apply_string` hands received lines to a `ReceivedScrollback`
//! shared by every copy, which drops by number the lines it already has
//! (a retransmitted diff repeats them). Both sides also carry
//! `scrollback_on`, so the server's first diff after the capability holds a
//! `ScrolledLines` (with no line if none is due): the client's announcement
//! that this server keeps history. Without the capability nothing here
//! is active and every diff is byte-identical to upstream's.

use mosh_platform::errln;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use mosh_network::prost::Message;
use mosh_network::proto::host_buffers::{
    EchoAck, HostBytes, HostMessage, Instruction, ResizeMessage, ScrolledLines,
};
use mosh_network::transport::dos_detected;
use mosh_network::{CryptoError, State};
use mosh_terminal::terminalframebuffer::Framebuffer;
use mosh_terminal::{Actions, Display, Emulator, Resize, UTF8Parser};

use super::user::UserAction;

/// Upstream `std::list<std::pair<uint64_t, uint64_t>>`: (frame number, time).
type InputHistoryType = VecDeque<(u64, u64)>;

/// One backlog entry: a rendered line, or a run of lines dropped unsent.
#[derive(Debug)]
enum Backlog {
    Line(Vec<u8>),
    Dropped(u64),
}

impl Backlog {
    /// How many line numbers the entry covers.
    fn count(&self) -> u64 {
        match self {
            Backlog::Line(_) => 1,
            Backlog::Dropped(n) => *n,
        }
    }

    fn bytes(&self) -> usize {
        match self {
            Backlog::Line(l) => l.len(),
            Backlog::Dropped(_) => 0,
        }
    }
}

/// mosh-rs scrollback, server side: the rendered lines that scrolled off the
/// screen and have not been delivered yet, numbered from 1 in the order they
/// left. Lines up to the client's acknowledged count are freed, so this is a
/// backlog, not a history. It is capped in bytes; on overflow the oldest
/// lines not yet published are replaced by a count of dropped lines.
#[derive(Debug)]
pub struct ScrolledRing {
    entries: VecDeque<Backlog>,
    /// Number of the first line `entries` covers.
    first: u64,
    /// Number of the newest captured line.
    captured: u64,
    /// Number of the newest published line (the live `scrolled_total`).
    published: u64,
    /// Bytes of the held lines.
    bytes: usize,
    /// Cap on `bytes`.
    limit: usize,
}

impl ScrolledRing {
    /// Bytes of rendered lines published beyond the client's acknowledged
    /// count: the payload of four fragments at upstream's IPv4 MTU
    /// (1280 - 28 IP/UDP - 12 nonce and timestamps - 16 OCB tag - 10
    /// fragment header = 1214 bytes each). Lines compress well (`seq` about
    /// 4:1), so a typical diff carries them in one or two fragments; even
    /// incompressible lines need at most five with the protobuf framing.
    pub const WINDOW: usize = 4 * 1214;

    fn new(limit: usize) -> ScrolledRing {
        ScrolledRing {
            entries: VecDeque::new(),
            first: 1,
            captured: 0,
            published: 0,
            bytes: 0,
            limit,
        }
    }

    fn push(&mut self, line: Vec<u8>) {
        self.captured += 1;
        self.bytes += line.len();
        self.entries.push_back(Backlog::Line(line));
        if self.bytes > self.limit {
            // Drop a sixteenth more than needed, so a steady flood does not
            // search for the oldest unpublished line on every push.
            self.drop_oldest_unpublished(self.limit - self.limit / 16);
        }
    }

    /// Replaces the oldest unpublished lines by one dropped run until the
    /// held bytes are at most `target` (published lines are kept: the
    /// client may still need them resent).
    fn drop_oldest_unpublished(&mut self, target: usize) {
        let mut index = self.first;
        let mut p = 0;
        while p < self.entries.len() && index <= self.published {
            index += self.entries[p].count();
            p += 1;
        }
        if p == self.entries.len() {
            return;
        }
        if let Backlog::Line(l) = &self.entries[p] {
            self.bytes -= l.len();
            self.entries[p] = Backlog::Dropped(1);
        }
        while self.bytes > target && p + 1 < self.entries.len() {
            let next = self.entries.remove(p + 1).expect("entry");
            self.bytes -= next.bytes();
            if let Backlog::Dropped(n) = &mut self.entries[p] {
                *n += next.count();
            }
        }
    }

    /// Frees the lines up to `acked` (the client has them) and publishes
    /// lines until the bytes published beyond `acked` would exceed
    /// `WINDOW`, always at least one line when none is outstanding, and
    /// dropped runs whole. Returns the new published count.
    fn publish(&mut self, acked: u64) -> u64 {
        while let Some(e) = self.entries.front_mut() {
            let c = e.count();
            if self.first + c - 1 <= acked {
                self.bytes -= e.bytes();
                self.entries.pop_front();
                self.first += c;
            } else {
                if let (Backlog::Dropped(n), true) = (&mut *e, self.first <= acked) {
                    *n -= acked + 1 - self.first;
                    self.first = acked + 1;
                }
                break;
            }
        }
        self.first = self.first.max(acked + 1);

        let (mut index, mut p, mut used) = (self.first, 0, 0);
        while p < self.entries.len() && index <= self.published {
            used += self.entries[p].bytes();
            index += self.entries[p].count();
            p += 1;
        }
        while p < self.entries.len() {
            let e = &self.entries[p];
            if let Backlog::Line(l) = e {
                if used > 0 && used + l.len() > Self::WINDOW {
                    break;
                }
                used += l.len();
            }
            index += e.count();
            self.published = index - 1;
            p += 1;
        }
        self.published
    }

    /// The `ScrolledLines` instructions for lines `(from, to]`: one per run
    /// of held lines, each with the count of lines missing before it (freed
    /// or dropped), and a last one with no lines if the range ends in a gap.
    fn scrolled_lines(&self, from: u64, to: u64) -> Vec<ScrolledLines> {
        let mut out = Vec::new();
        let mut run: Option<ScrolledLines> = None;
        let mut dropped = self.first.min(to + 1).saturating_sub(from + 1);
        let mut index = self.first;
        for e in &self.entries {
            if index > to {
                break;
            }
            match e {
                Backlog::Line(l) if index > from => {
                    let r = run.get_or_insert_with(|| ScrolledLines {
                        first_index: Some(index),
                        line: Vec::new(),
                        dropped: (dropped > 0).then_some(dropped),
                    });
                    r.line.push(l.clone());
                    dropped = 0;
                }
                Backlog::Line(_) => {}
                Backlog::Dropped(n) => {
                    let (lo, hi) = (index.max(from + 1), (index + n - 1).min(to));
                    if hi >= lo {
                        out.extend(run.take());
                        dropped += hi - lo + 1;
                    }
                }
            }
            index += e.count();
        }
        out.extend(run);
        if dropped > 0 {
            out.push(ScrolledLines {
                first_index: Some(to + 1),
                line: Vec::new(),
                dropped: Some(dropped),
            });
        }
        out
    }
}

/// mosh-rs scrollback, client side: lines received from the server, each
/// with its number, in order and without duplicates, until the client takes
/// them. Shared by every copy of the client's `Complete`.
#[derive(Debug)]
pub struct ReceivedScrollback {
    pending: VecDeque<(u64, Vec<u8>)>,
    /// Number of the next line not yet received (lines start at 1).
    next_index: u64,
    /// Lines the server could no longer send (evicted from its ring).
    lost: u64,
}

impl ReceivedScrollback {
    fn new() -> ReceivedScrollback {
        ReceivedScrollback {
            pending: VecDeque::new(),
            next_index: 1,
            lost: 0,
        }
    }

    /// Takes the lines received since the last call, oldest first, each with
    /// its number. Numbers are consecutive except where lines were lost.
    pub fn take_pending(&mut self) -> VecDeque<(u64, Vec<u8>)> {
        std::mem::take(&mut self.pending)
    }

    /// How many lines the server reported as evicted before it could send them.
    pub fn lost(&self) -> u64 {
        self.lost
    }

    /// Lines numbered `first_index, first_index + 1, ...`. Numbers already
    /// received are skipped (a retransmitted diff repeats them); a jump
    /// forward, including one past the last line, counts as lost.
    fn receive(&mut self, first_index: u64, lines: &[Vec<u8>]) {
        let mut index = first_index;
        for line in lines {
            if index >= self.next_index {
                self.lost += index - self.next_index;
                self.pending.push_back((index, line.clone()));
                self.next_index = index + 1;
            }
            index += 1;
        }
        if index > self.next_index {
            self.lost += index - self.next_index;
            self.next_index = index;
        }
    }
}

/// Upstream `Terminal::Complete`: a UTF8Parser feeding Actions to an Emulator.
#[derive(Clone, Debug)]
pub struct Complete {
    parser: UTF8Parser,
    terminal: Emulator,
    display: Display,

    // Only used locally by act(), but kept here as upstream does. It must
    // always be empty outside calls to act().
    actions: Actions,

    input_history: InputHistoryType,
    echo_ack: u64,

    /// mosh-rs scrollback: cap in bytes on the server's backlog (0 = never
    /// capture).
    scrollback_limit: usize,
    /// mosh-rs scrollback, server: the shared ring; `Some` once the client's
    /// capabilities asked for scrollback.
    ring: Option<Rc<RefCell<ScrolledRing>>>,
    /// mosh-rs scrollback: number of lines that scrolled off and were
    /// published up to this state (server: `publish_scrollback`; client:
    /// covered by applied diffs).
    scrolled_total: u64,
    /// mosh-rs scrollback, client: where `apply_string` puts received lines.
    received: Option<Rc<RefCell<ReceivedScrollback>>>,
    /// mosh-rs scrollback: the session is in scrollback mode (server: the
    /// client's capabilities turned capture on; client: a diff carried a
    /// `ScrolledLines` instruction). A diff from a state without it to one
    /// with it always carries at least one `ScrolledLines`, so a client can
    /// tell a scrollback server from an old one by its first frame.
    scrollback_on: bool,
}

impl Complete {
    /// for late ack
    const ECHO_TIMEOUT: u64 = 50;

    /// A blank terminal of `width` x `height`.
    pub fn new(width: usize, height: usize) -> Complete {
        Complete {
            parser: UTF8Parser::new(),
            terminal: Emulator::new(width, height),
            display: Display::new(false),
            actions: Actions::new(),
            input_history: VecDeque::new(),
            echo_ack: 0,
            scrollback_limit: 0,
            ring: None,
            scrolled_total: 0,
            received: None,
            scrollback_on: false,
        }
    }

    /// mosh-rs scrollback, server: the cap in bytes on undelivered lines kept
    /// for a client that asks for scrollback (`mosh-server --scrollback N`,
    /// in MiB); 0 never captures.
    pub fn set_scrollback_limit(&mut self, limit: usize) {
        self.scrollback_limit = limit;
    }

    /// mosh-rs scrollback, server: frees the lines up to `acked` (the
    /// `scrolled_total` of the newest state the client acknowledged) and
    /// publishes more within the window; returns whether `scrolled_total`
    /// changed (then the state must be handed to the transport again).
    pub fn publish_scrollback(&mut self, acked: u64) -> bool {
        let Some(ring) = &self.ring else {
            return false;
        };
        let published = ring.borrow_mut().publish(acked);
        let changed = published != self.scrolled_total;
        self.scrolled_total = published;
        changed
    }

    /// mosh-rs scrollback, server: lines captured so far (published or not).
    pub fn get_scrollback_captured(&self) -> u64 {
        match &self.ring {
            Some(ring) => ring.borrow().captured,
            None => self.scrolled_total,
        }
    }

    /// mosh-rs scrollback, client: from now on `apply_string` delivers
    /// received lines to the returned queue (shared by every copy of this
    /// state, so it can be read whichever copy the transport applied to).
    pub fn enable_scrollback_receive(&mut self) -> Rc<RefCell<ReceivedScrollback>> {
        self.received
            .get_or_insert_with(|| Rc::new(RefCell::new(ReceivedScrollback::new())))
            .clone()
    }

    /// mosh-rs scrollback: lines that scrolled off up to this state.
    pub fn get_scrolled_total(&self) -> u64 {
        self.scrolled_total
    }

    /// mosh-rs scrollback: whether the session is in scrollback mode (see
    /// `scrollback_on`). On the client: the server announced scrollback.
    pub fn get_scrollback_on(&self) -> bool {
        self.scrollback_on
    }

    /// Feeds host output bytes; returns the replies for the host.
    pub fn act(&mut self, s: &[u8]) -> Vec<u8> {
        let terminal_to_host = self.act_octets(s);
        // mosh-rs scrollback: render the rows that scrolled off into the
        // backlog; publish_scrollback decides when they are sent.
        if let Some(ring) = &self.ring {
            let rows = self.terminal.get_fb_mut().take_scrolled_rows();
            let mut ring = ring.borrow_mut();
            for row in &rows {
                ring.push(Display::render_scrolled_row(row).into_bytes());
            }
        }
        terminal_to_host
    }

    /// Upstream's `act( const std::string& )`.
    fn act_octets(&mut self, s: &[u8]) -> Vec<u8> {
        for &c in s {
            /* parse octet into up to three actions */
            self.parser.input(c, &mut self.actions);

            /* apply actions to terminal and delete them */
            for act in self.actions.iter() {
                act.act_on_terminal(&mut self.terminal);
            }
            self.actions.clear();
        }

        self.terminal.read_octets_to_host()
    }

    /// Applies one user-side action (keystroke or resize); returns the bytes
    /// for the host. Upstream `Complete::act( const Parser::Action& )`.
    pub fn act_action(&mut self, act: UserAction<'_>) -> Vec<u8> {
        /* apply action to terminal */
        match act {
            UserAction::UserByte(u) => u.act_on_terminal(&mut self.terminal),
            UserAction::Resize(r) => r.act_on_terminal(&mut self.terminal),
            UserAction::Capabilities(c) => {
                // mosh-rs (PLAN.md 6.1): the session turns scrollback on once.
                if c.scrollback && self.scrollback_limit > 0 && self.ring.is_none() {
                    self.ring = Some(Rc::new(RefCell::new(ScrolledRing::new(
                        self.scrollback_limit,
                    ))));
                    self.terminal.get_fb_mut().set_scroll_capture(true);
                    self.scrollback_on = true;
                }
            }
        }
        self.terminal.read_octets_to_host()
    }

    /// The emulator's framebuffer.
    pub fn get_fb(&self) -> &Framebuffer {
        self.terminal.get_fb()
    }

    /// The newest input frame number the client may consider echoed.
    pub fn get_echo_ack(&self) -> u64 {
        self.echo_ack
    }

    /// Advances `echo_ack` to the newest input frame older than
    /// `ECHO_TIMEOUT` ms; returns whether it changed.
    pub fn set_echo_ack(&mut self, now: u64) -> bool {
        let mut ret = false;
        let mut newest_echo_ack: u64 = 0;

        // uint64_t arithmetic: now - ECHO_TIMEOUT wraps when now < 50, as upstream.
        for i in self.input_history.iter() {
            if i.1 <= now.wrapping_sub(Self::ECHO_TIMEOUT) {
                newest_echo_ack = i.0;
            }
        }

        self.input_history.retain(|i| i.0 >= newest_echo_ack);

        if self.echo_ack != newest_echo_ack {
            ret = true;
        }

        self.echo_ack = newest_echo_ack;

        ret
    }

    /// Records that input frame `n` arrived at time `now`.
    pub fn register_input_frame(&mut self, n: u64, now: u64) {
        self.input_history.push_back((n, now));
    }

    /// Milliseconds until the next echo ack is due (`i32::MAX` if none).
    pub fn wait_time(&self, now: u64) -> i32 {
        if self.input_history.len() < 2 {
            return i32::MAX;
        }

        let it = &self.input_history[1];

        let next_echo_ack_time = it.1.wrapping_add(Self::ECHO_TIMEOUT);
        if next_echo_ack_time <= now {
            return 0;
        }
        // uint64_t to int, as upstream's return: truncates if over i32 range.
        (next_echo_ack_time - now) as i32
    }

    /// Upstream's `diff_from`. mosh-rs scrollback: with `with_lines` false
    /// no line is sent, only the count (all reported dropped). `init_diff`
    /// uses that: the backlog no longer holds delivered lines, and the
    /// sender's verbose check compares the init diffs of the current state
    /// and of a copy of an older one, which may predate the backlog.
    fn diff_with_lines(&self, existing: &Complete, with_lines: bool) -> Vec<u8> {
        let mut output = HostMessage::default();

        if existing.get_echo_ack() != self.get_echo_ack() {
            assert!(self.get_echo_ack() >= existing.get_echo_ack());
            output.instruction.push(Instruction {
                echoack: Some(EchoAck {
                    echo_ack_num: Some(self.get_echo_ack()),
                }),
                ..Default::default()
            });
        }

        // mosh-rs scrollback (PLAN.md 6.3): the lines that scrolled off since
        // `existing`, before the resize and host bytes, so a client prints
        // them first. Never present unless the client asked for scrollback.
        if existing.scrolled_total != self.scrolled_total {
            assert!(self.scrolled_total > existing.scrolled_total);
            let runs = match (&self.ring, with_lines) {
                (Some(ring), true) => ring
                    .borrow()
                    .scrolled_lines(existing.scrolled_total, self.scrolled_total),
                _ => {
                    // No lines (a client-side state keeps none; it is never
                    // asked for a diff in practice): all reported dropped.
                    vec![ScrolledLines {
                        first_index: Some(self.scrolled_total + 1),
                        line: Vec::new(),
                        dropped: Some(self.scrolled_total - existing.scrolled_total),
                    }]
                }
            };
            for lines in runs {
                output.instruction.push(Instruction {
                    scrolledlines: Some(lines),
                    ..Default::default()
                });
            }
        } else if self.scrollback_on && !existing.scrollback_on {
            // mosh-rs scrollback (PLAN.md 6.3): announce scrollback mode with
            // an instruction that covers no line, so the client's first frame
            // tells it this server keeps history even before any line exists.
            output.instruction.push(Instruction {
                scrolledlines: Some(ScrolledLines {
                    first_index: Some(self.scrolled_total + 1),
                    line: Vec::new(),
                    dropped: None,
                }),
                ..Default::default()
            });
        }

        if !(existing.get_fb() == self.get_fb()) {
            let (ew, eh) = (
                existing.get_fb().ds.get_width(),
                existing.get_fb().ds.get_height(),
            );
            let (w, h) = (self.get_fb().ds.get_width(), self.get_fb().ds.get_height());
            if (ew != w) || (eh != h) {
                output.instruction.push(Instruction {
                    resize: Some(ResizeMessage {
                        width: Some(w),
                        height: Some(h),
                    }),
                    ..Default::default()
                });
            }
            let update = self
                .display
                .new_frame(true, existing.get_fb(), self.get_fb());
            if !update.is_empty() {
                output.instruction.push(Instruction {
                    hostbytes: Some(HostBytes {
                        hoststring: Some(update.into_bytes()),
                    }),
                    ..Default::default()
                });
            }
        }

        output.encode_to_vec()
    }
}

impl PartialEq for Complete {
    fn eq(&self, x: &Complete) -> bool {
        // assert( parser == x.parser ); /* parser state is irrelevant for us */
        // Emulator equality is Framebuffer equality, which compares row
        // pointers: two Completes are equal only when one is a copy of the
        // other with no row written since.
        // mosh-rs scrollback: scrolled_total too, or lines that scroll off
        // without changing the final screen would never be sent. It is 0 on
        // both sides unless the client asked for scrollback.
        (self.terminal == x.terminal)
            && (self.echo_ack == x.echo_ack)
            && (self.scrolled_total == x.scrolled_total)
            && (self.scrollback_on == x.scrollback_on)
    }
}

impl State for Complete {
    fn subtract(&mut self, _prefix: &Complete) {}

    /// Upstream `subtract( const Complete* ) const {}` does nothing, also
    /// when handed itself, so there is nothing to copy or change.
    fn subtract_self(&mut self) {}

    fn diff_from(&self, existing: &Complete) -> Vec<u8> {
        self.diff_with_lines(existing, true)
    }

    fn init_diff(&self) -> Vec<u8> {
        // size_t from the int width/height, which are always >= 1.
        self.diff_with_lines(
            &Complete::new(
                self.get_fb().ds.get_width() as usize,
                self.get_fb().ds.get_height() as usize,
            ),
            false,
        )
    }

    fn apply_string(&mut self, diff: &[u8]) -> Result<(), CryptoError> {
        // Upstream fatal_asserts the parse and asserts the three checks below.
        // PLAN.md 9.8: malformed input from a keyed peer is an error, not an
        // abort. On error the state may be partly applied; Transport::recv
        // applies to a copy and drops it.
        let input = HostMessage::decode(diff)
            .map_err(|_| dos_detected("input.ParseFromString( diff )", "Complete::apply_string"))?;

        for inst in &input.instruction {
            if let Some(hb) = &inst.hostbytes {
                let terminal_to_host = self.act_octets(hb.hoststring.as_deref().unwrap_or(&[]));
                // mosh-rs scrollback: a copy of a server state (the sender's
                // round-trip check) may have capture on; rows scrolled by a
                // diff are not new history, so they are discarded.
                self.terminal.get_fb_mut().take_scrolled_rows();
                if !terminal_to_host.is_empty() {
                    /* server never interrogates client terminal */
                    return Err(dos_detected(
                        "terminal_to_host.empty()",
                        "Complete::apply_string",
                    ));
                }
            } else if let Some(r) = &inst.resize {
                // int32 to size_t, as upstream's Resize( width(), height() );
                // upstream then asserts a size above 0 in Framebuffer::resize.
                // Both values are >= 1 after the check, so the casts are lossless.
                let (w, h) = (r.width.unwrap_or(0), r.height.unwrap_or(0));
                if w < 1 || h < 1 {
                    return Err(dos_detected(
                        "width > 0 && height > 0",
                        "Complete::apply_string",
                    ));
                }
                let resize = Resize::new(w as usize, h as usize);
                self.act_action(UserAction::Resize(&resize));
            } else if let Some(e) = &inst.echoack {
                let inst_echo_ack_num = e.echo_ack_num.unwrap_or(0);
                if inst_echo_ack_num < self.echo_ack {
                    return Err(dos_detected(
                        "inst_echo_ack_num >= echo_ack",
                        "Complete::apply_string",
                    ));
                }
                self.echo_ack = inst_echo_ack_num;
            } else if let Some(sl) = &inst.scrolledlines {
                // mosh-rs scrollback (PLAN.md 6.3), after upstream's arms.
                // `end` is one past the last line number the diff covers.
                let first_index = sl.first_index.unwrap_or(0);
                let end = first_index.saturating_add(sl.line.len() as u64);
                if first_index == 0 || end - 1 < self.scrolled_total {
                    return Err(dos_detected(
                        "scrolled lines do not go backwards",
                        "Complete::apply_string",
                    ));
                }
                if let Some(received) = &self.received {
                    received.borrow_mut().receive(first_index, &sl.line);
                }
                self.scrolled_total = end - 1;
                self.scrollback_on = true;
            }
        }
        Ok(())
    }

    fn reset_input(&mut self) {
        self.parser.reset_input();
    }

    /// Diagnostics only, as upstream: reports differences between the two
    /// screens on stderr and returns whether any were found.
    fn compare(&self, other: &Complete) -> bool {
        let mut ret = false;
        let fb = self.terminal.get_fb();
        let other_fb = other.terminal.get_fb();
        let (height, other_height) = (fb.ds.get_height(), other_fb.ds.get_height());
        let (width, other_width) = (fb.ds.get_width(), other_fb.ds.get_width());

        if height != other_height || width != other_width {
            errln!(
                "Framebuffer size ({}x{}, {}x{}) differs.",
                width,
                height,
                other_width,
                other_height
            );
            return true;
        }

        for y in 0..height {
            for x in 0..width {
                if fb.get_cell(y, x).compare(other_fb.get_cell(y, x)) {
                    errln!("Cell ({}, {}) differs.", y, x);
                    ret = true;
                }
            }
        }

        if (fb.ds.get_cursor_row() != other_fb.ds.get_cursor_row())
            || (fb.ds.get_cursor_col() != other_fb.ds.get_cursor_col())
        {
            errln!(
                "Cursor mismatch: ({}, {}) vs. ({}, {}).",
                fb.ds.get_cursor_row(),
                fb.ds.get_cursor_col(),
                other_fb.ds.get_cursor_row(),
                other_fb.ds.get_cursor_col()
            );
            ret = true;
        }
        /* XXX should compare other terminal state too (mouse mode, bell. etc.) */

        // mosh-rs scrollback: 0 and off on both sides unless the client asked for it.
        if self.scrolled_total != other.scrolled_total {
            errln!(
                "Scrolled line count mismatch: {} vs. {}.",
                self.scrolled_total,
                other.scrolled_total
            );
            ret = true;
        }
        if self.scrollback_on != other.scrollback_on {
            errln!(
                "Scrollback mode mismatch: {} vs. {}.",
                self.scrollback_on,
                other.scrollback_on
            );
            ret = true;
        }

        ret
    }
}
