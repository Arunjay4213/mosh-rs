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
// Ported from: upstream/src/statesync/user.h, upstream/src/statesync/user.cc

//! The client-to-server state: an ordered stream of keystrokes and resizes,
//! and its protobuf diff format (`ClientBuffers::UserMessage`).
//! Port of upstream `src/statesync/user.{h,cc}`.

use std::collections::VecDeque;

use mosh_network::prost::Message;
use mosh_network::proto::client_buffers::{Instruction, Keystroke, ResizeMessage, UserMessage};
use mosh_network::transport::dos_detected;
use mosh_network::{CryptoError, State};
use mosh_terminal::{Resize, UserByte};

/// Which of the two actions a `UserEvent` holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserEventType {
    /// A keystroke byte.
    UserByteType = 0,
    /// A window resize.
    ResizeType = 1,
}

/// One user event. As upstream, both actions are stored and compared; the
/// unused one holds upstream's placeholder (`UserByte(0)` or `Resize(-1, -1)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserEvent {
    /// Which action is meaningful.
    pub r#type: UserEventType,
    /// The keystroke, for `UserByteType`.
    pub userbyte: UserByte,
    /// The resize, for `ResizeType`.
    pub resize: Resize,
}

impl UserEvent {
    /// A keystroke event.
    pub fn from_userbyte(s_userbyte: UserByte) -> UserEvent {
        // Upstream's Resize( -1, -1 ) on size_t members is SIZE_MAX.
        UserEvent {
            r#type: UserEventType::UserByteType,
            userbyte: s_userbyte,
            resize: Resize::new(usize::MAX, usize::MAX),
        }
    }

    /// A resize event.
    pub fn from_resize(s_resize: Resize) -> UserEvent {
        UserEvent {
            r#type: UserEventType::ResizeType,
            userbyte: UserByte::new(0),
            resize: s_resize,
        }
    }
}

/// Upstream's `const Parser::Action&` as returned by `get_action`: one of the
/// two user-side actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAction<'a> {
    /// A keystroke.
    UserByte(&'a UserByte),
    /// A resize.
    Resize(&'a Resize),
}

/// Upstream `Network::UserStream`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserStream {
    actions: VecDeque<UserEvent>,
}

impl UserStream {
    /// An empty stream.
    pub fn new() -> UserStream {
        UserStream {
            actions: VecDeque::new(),
        }
    }

    /// Appends a keystroke.
    pub fn push_back_userbyte(&mut self, s_userbyte: UserByte) {
        self.actions.push_back(UserEvent::from_userbyte(s_userbyte));
    }

    /// Appends a resize.
    pub fn push_back_resize(&mut self, s_resize: Resize) {
        self.actions.push_back(UserEvent::from_resize(s_resize));
    }

    /// Whether the stream has no events.
    pub fn empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Number of events.
    pub fn size(&self) -> usize {
        self.actions.len()
    }

    /// The events, oldest first.
    pub fn events(&self) -> &VecDeque<UserEvent> {
        &self.actions
    }

    /// The action of event `i`.
    pub fn get_action(&self, i: usize) -> UserAction<'_> {
        let e = &self.actions[i];
        match e.r#type {
            UserEventType::UserByteType => UserAction::UserByte(&e.userbyte),
            UserEventType::ResizeType => UserAction::Resize(&e.resize),
        }
    }
}

impl State for UserStream {
    fn subtract(&mut self, prefix: &UserStream) {
        // Upstream also short-cuts `this == prefix` (same object) by clearing;
        // with Rust's borrow rules `prefix` can never alias `self`.
        for i in prefix.actions.iter() {
            assert!(!self.actions.is_empty());
            assert!(Some(i) == self.actions.front());
            self.actions.pop_front();
        }
    }

    fn subtract_self(&mut self) {
        // if we are subtracting ourself from ourself, just clear the std::deque
        self.actions.clear();
    }

    fn diff_from(&self, existing: &UserStream) -> Vec<u8> {
        let mut my_it = self.actions.iter();

        for i in existing.actions.iter() {
            let mine = my_it.next();
            assert!(mine.is_some());
            assert!(Some(i) == mine);
        }

        let mut output = UserMessage::default();

        for e in my_it {
            match e.r#type {
                UserEventType::UserByteType => {
                    let the_byte = e.userbyte.c;
                    /* can we combine this with a previous Keystroke? */
                    match output.instruction.last_mut() {
                        Some(Instruction {
                            keystroke: Some(k), ..
                        }) => {
                            k.keys.get_or_insert_with(Vec::new).push(the_byte);
                        }
                        _ => output.instruction.push(Instruction {
                            keystroke: Some(Keystroke {
                                keys: Some(vec![the_byte]),
                            }),
                            ..Default::default()
                        }),
                    }
                }
                UserEventType::ResizeType => {
                    // size_t to int32, as upstream's set_width( size_t ).
                    output.instruction.push(Instruction {
                        resize: Some(ResizeMessage {
                            width: Some(e.resize.width as i32),
                            height: Some(e.resize.height as i32),
                        }),
                        ..Default::default()
                    });
                }
            }
        }

        output.encode_to_vec()
    }

    fn init_diff(&self) -> Vec<u8> {
        self.diff_from(&UserStream::new())
    }

    fn apply_string(&mut self, diff: &[u8]) -> Result<(), CryptoError> {
        // Upstream: fatal_assert( input.ParseFromString( diff ) ). PLAN.md 9.8:
        // malformed input from a keyed peer is an error, not an abort.
        let input = UserMessage::decode(diff).map_err(|_| {
            dos_detected("input.ParseFromString( diff )", "UserStream::apply_string")
        })?;

        for inst in &input.instruction {
            if let Some(k) = &inst.keystroke {
                for &b in k.keys.as_deref().unwrap_or(&[]) {
                    self.actions
                        .push_back(UserEvent::from_userbyte(UserByte::new(b)));
                }
            } else if let Some(r) = &inst.resize {
                // Upstream converts int32 to size_t here and aborts later, when
                // the server's Framebuffer::resize asserts a size above 0 (or
                // fails to allocate a wrapped negative one). PLAN.md 9.8: an
                // error now. Both values are >= 1 after the check, so the
                // casts are lossless.
                let (w, h) = (r.width.unwrap_or(0), r.height.unwrap_or(0));
                if w < 1 || h < 1 {
                    return Err(dos_detected(
                        "width > 0 && height > 0",
                        "UserStream::apply_string",
                    ));
                }
                self.actions
                    .push_back(UserEvent::from_resize(Resize::new(w as usize, h as usize)));
            }
            /* any other instruction type (unknown extension) is ignored */
        }
        Ok(())
    }

    fn reset_input(&mut self) {
        // Upstream UserStream has no reset_input: the template member
        // TransportSender::set_current_state that calls it is only instantiated
        // for Complete (mosh-server.cc). Nothing to reset here.
    }
    /// Upstream's `compare` (diagnostics only): always false.
    fn compare(&self, _other: &UserStream) -> bool {
        false
    }
}
