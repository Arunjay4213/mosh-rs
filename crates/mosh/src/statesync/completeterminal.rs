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

use mosh_platform::errln;
use std::collections::VecDeque;

use mosh_network::prost::Message;
use mosh_network::proto::host_buffers::{
    EchoAck, HostBytes, HostMessage, Instruction, ResizeMessage,
};
use mosh_network::transport::dos_detected;
use mosh_network::{CryptoError, State};
use mosh_terminal::terminalframebuffer::Framebuffer;
use mosh_terminal::{Actions, Display, Emulator, Resize, UTF8Parser};

use super::user::UserAction;

/// Upstream `std::list<std::pair<uint64_t, uint64_t>>`: (frame number, time).
type InputHistoryType = VecDeque<(u64, u64)>;

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
        }
    }

    /// Feeds host output bytes; returns the replies for the host.
    pub fn act(&mut self, s: &[u8]) -> Vec<u8> {
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
        (self.terminal == x.terminal) && (self.echo_ack == x.echo_ack)
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
                let terminal_to_host = self.act(hb.hoststring.as_deref().unwrap_or(&[]));
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

        ret
    }
}
