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
//! `Framebuffer::scroll`); `act` renders them into a `ScrolledRing` shared by
//! every copy of this state, and the state itself only counts them
//! (`scrolled_total`). `diff_from` sends the lines numbered
//! `(existing.scrolled_total, self.scrolled_total]` as a `ScrolledLines`
//! instruction placed after the echo ack and before the resize and host
//! bytes, so a client can print the lines first and then the new frame. On
//! the client, `apply_string` hands received lines to a `ReceivedScrollback`
//! shared by every copy, which drops by number the lines it already has
//! (a retransmitted diff repeats them). Without the capability nothing here
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

/// mosh-rs scrollback, server side: the newest rendered lines that scrolled
/// off the screen, numbered from 1. Holds lines `(total - len, total]`.
#[derive(Debug)]
pub struct ScrolledRing {
    lines: VecDeque<Vec<u8>>,
    limit: usize,
    total: u64,
}

impl ScrolledRing {
    fn new(limit: usize) -> ScrolledRing {
        ScrolledRing {
            lines: VecDeque::new(),
            limit,
            total: 0,
        }
    }

    fn push(&mut self, line: Vec<u8>) {
        if self.lines.len() == self.limit {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
        self.total += 1;
    }

    /// The `ScrolledLines` for lines `(from, to]`: those still held, and how
    /// many before them were evicted.
    fn scrolled_lines(&self, from: u64, to: u64) -> ScrolledLines {
        let oldest = self.total - self.lines.len() as u64 + 1;
        let start = (from + 1).max(oldest).min(to + 1);
        let line = (start..=to)
            .map(|i| self.lines[(i - oldest) as usize].clone())
            .collect();
        let dropped = start - (from + 1);
        ScrolledLines {
            first_index: Some(start),
            line,
            dropped: (dropped > 0).then_some(dropped),
        }
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

    /// mosh-rs scrollback: ring size the server allows (0 = never capture).
    scrollback_limit: usize,
    /// mosh-rs scrollback, server: the shared ring; `Some` once the client's
    /// capabilities asked for scrollback.
    ring: Option<Rc<RefCell<ScrolledRing>>>,
    /// mosh-rs scrollback: number of lines that scrolled off up to this
    /// state (server: captured; client: covered by applied diffs).
    scrolled_total: u64,
    /// mosh-rs scrollback, client: where `apply_string` puts received lines.
    received: Option<Rc<RefCell<ReceivedScrollback>>>,
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
        }
    }

    /// mosh-rs scrollback, server: the most lines to keep for a client that
    /// asks for scrollback (`mosh-server --scrollback N`); 0 never captures.
    pub fn set_scrollback_limit(&mut self, limit: usize) {
        self.scrollback_limit = limit;
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

    /// Feeds host output bytes; returns the replies for the host.
    pub fn act(&mut self, s: &[u8]) -> Vec<u8> {
        let terminal_to_host = self.act_octets(s);
        // mosh-rs scrollback: render the rows that scrolled off into the ring.
        if let Some(ring) = &self.ring {
            let rows = self.terminal.get_fb_mut().take_scrolled_rows();
            let mut ring = ring.borrow_mut();
            for row in &rows {
                ring.push(Display::render_scrolled_row(row).into_bytes());
            }
            self.scrolled_total = ring.total;
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
    }
}

impl State for Complete {
    fn subtract(&mut self, _prefix: &Complete) {}

    /// Upstream `subtract( const Complete* ) const {}` does nothing, also
    /// when handed itself, so there is nothing to copy or change.
    fn subtract_self(&mut self) {}

    fn diff_from(&self, existing: &Complete) -> Vec<u8> {
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
            let lines = match &self.ring {
                Some(ring) => ring
                    .borrow()
                    .scrolled_lines(existing.scrolled_total, self.scrolled_total),
                None => {
                    // A client-side state, which keeps no lines (it is never
                    // asked for a diff in practice): all are reported dropped.
                    ScrolledLines {
                        first_index: Some(self.scrolled_total + 1),
                        line: Vec::new(),
                        dropped: Some(self.scrolled_total - existing.scrolled_total),
                    }
                }
            };
            output.instruction.push(Instruction {
                scrolledlines: Some(lines),
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

    fn init_diff(&self) -> Vec<u8> {
        // size_t from the int width/height, which are always >= 1.
        self.diff_from(&Complete::new(
            self.get_fb().ds.get_width() as usize,
            self.get_fb().ds.get_height() as usize,
        ))
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

        // mosh-rs scrollback: 0 on both sides unless the client asked for it.
        if self.scrolled_total != other.scrolled_total {
            errln!(
                "Scrolled line count mismatch: {} vs. {}.",
                self.scrolled_total,
                other.scrolled_total
            );
            ret = true;
        }

        ret
    }
}
