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
// Ported from: upstream/src/network/transportstate.h

//! Port of upstream `src/network/transportstate.h` (`TimestampedState`),
//! plus the `State` trait that stands for the template parameter
//! upstream's `TransportSender` and `Transport` take.

/// What `TransportSender<MyState>` and `Transport<MyState, RemoteState>`
/// require of a state type (upstream's implicit template interface).
pub trait State: PartialEq + Clone {
    /// The bytes that turn `existing` into `self`.
    fn diff_from(&self, existing: &Self) -> Vec<u8>;
    /// The diff from the initial (empty) state.
    fn init_diff(&self) -> Vec<u8>;
    /// Applies a diff produced by `diff_from`.
    fn apply_string(&mut self, diff: &[u8]);
    /// Drop the part of `self` that `prefix` already contains.
    fn subtract(&mut self, prefix: &Self);
    /// `subtract` with `self` as the prefix. Upstream passes the same object
    /// as both (`UserStream::subtract` clears on `this == prefix`); Rust
    /// borrows cannot alias, so this is its own method. The default is
    /// exactly that call on a copy; an implementation overrides it to skip
    /// the copy (`UserStream`: clear, `Complete`: nothing).
    fn subtract_self(&mut self) {
        let prefix = self.clone();
        self.subtract(&prefix);
    }
    /// Called on the new current state by `set_current_state`.
    fn reset_input(&mut self);
}

/// A state with its number and the time it was sent or received.
#[derive(Clone, Debug)]
pub struct TimestampedState<S> {
    /// Milliseconds (`timestamp()`).
    pub timestamp: u64,
    /// State number.
    pub num: u64,
    /// The state.
    pub state: S,
}

impl<S> TimestampedState<S> {
    /// A timestamped state from its parts.
    pub fn new(s_timestamp: u64, s_num: u64, s_state: S) -> TimestampedState<S> {
        TimestampedState {
            timestamp: s_timestamp,
            num: s_num,
            state: s_state,
        }
    }
}
