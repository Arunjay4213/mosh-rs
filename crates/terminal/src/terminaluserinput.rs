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
// Ported from: upstream/src/terminal/terminaluserinput.h, upstream/src/terminal/terminaluserinput.cc

use crate::parseraction::UserByte;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserInputState {
    Ground,
    ESC,
    SS3,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserInput {
    state: UserInputState,
}

impl Default for UserInput {
    fn default() -> Self {
        Self::new()
    }
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput { state: UserInputState::Ground }
    }

    pub fn input(&mut self, act: &UserByte, application_mode_cursor_keys: bool) -> Vec<u8> {
        /* The user will always be in application mode. If stm is not in
        application mode, convert user's cursor control function to an
        ANSI cursor control sequence */

        /* We need to look ahead one byte in the SS3 state to see if
        the next byte will be A, B, C, or D (cursor control keys). */

        /* This doesn't handle the 8-bit SS3 C1 control, which would be
        two octets in UTF-8. Fortunately nobody seems to send this. */

        match self.state {
            UserInputState::Ground => {
                if act.c == 0x1b {
                    /* ESC */
                    self.state = UserInputState::ESC;
                }
                vec![act.c]
            }

            UserInputState::ESC => {
                if act.c == b'O' {
                    /* ESC O = 7-bit SS3 */
                    self.state = UserInputState::SS3;
                    return Vec::new();
                }
                self.state = UserInputState::Ground;
                vec![act.c]
            }

            UserInputState::SS3 => {
                self.state = UserInputState::Ground;
                if (!application_mode_cursor_keys) && (act.c >= b'A') && (act.c <= b'D') {
                    vec![b'[', act.c]
                } else {
                    vec![b'O', act.c]
                }
            }
        }
    }
}
