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
// Ported from: upstream/src/terminal/parseraction.h, upstream/src/terminal/parseraction.cc

//! Parser actions and how each acts on the emulator. Port of upstream `src/terminal/parseraction.{h,cc}`.

use crate::terminal::Emulator;

/// Upstream `wchar_t`. On the arm64 Linux build that serves as the oracle it
/// is a 32-bit unsigned integer; every value the parser produces is at most
/// 0x10FFFF (or the -1 default, which no code path reads).
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
pub type wchar_t = u32;

/// The concrete action classes of upstream's `Parser::Action` hierarchy.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionType {
    /// No-op; never appended to an action list.
    Ignore,
    /// Print the character at the cursor.
    Print,
    /// Run a C0/C1 control function.
    Execute,
    /// Clear the dispatcher's parameters and intermediates.
    Clear,
    /// Collect an intermediate or private-marker byte.
    Collect,
    /// Collect a parameter digit or ';'.
    Param,
    /// Dispatch an ESC sequence (or a 7-bit C1 control).
    Esc_Dispatch,
    /// Dispatch a CSI sequence.
    CSI_Dispatch,
    /// Start of a DCS string (ignored by the emulator).
    Hook,
    /// DCS string byte (ignored by the emulator).
    Put,
    /// End of a DCS string (ignored by the emulator).
    Unhook,
    /// Start of an OSC string.
    OSC_Start,
    /// OSC string character.
    OSC_Put,
    /// End of an OSC string: dispatch it.
    OSC_End,
}

/// Upstream `Parser::Action` (host-source actions only; `UserByte` and
/// `Resize` are separate structs below, as they are not produced by the
/// state machine).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Action {
    /// Which action this is.
    pub kind: ActionType,
    /// The code point that triggered the action (`u32::MAX`, upstream's -1, when none).
    pub ch: wchar_t,
    /// Whether `ch` is meaningful.
    pub char_present: bool,
}

/// Upstream `Parser::Actions`.
pub type Actions = Vec<Action>;

impl Action {
    /// `Action() : ch( -1 ), char_present( false )`
    pub fn new(kind: ActionType) -> Action {
        Action {
            kind,
            ch: u32::MAX,
            char_present: false,
        }
    }

    /// True for the Ignore action.
    pub fn ignore(&self) -> bool {
        self.kind == ActionType::Ignore
    }

    /// Applies the action to the emulator, as upstream's virtual `act_on_terminal` does.
    pub fn act_on_terminal(&self, emu: &mut Emulator) {
        match self.kind {
            ActionType::Print => emu.print(self),
            ActionType::Execute => emu.execute(self),
            ActionType::Clear => emu.dispatch.clear(self),
            ActionType::Param => emu.dispatch.newparamchar(self),
            ActionType::Collect => emu.dispatch.collect(self),
            ActionType::CSI_Dispatch => emu.CSI_dispatch(self),
            ActionType::Esc_Dispatch => emu.Esc_dispatch(self),
            ActionType::OSC_Put => emu.dispatch.OSC_put(self),
            ActionType::OSC_Start => emu.dispatch.OSC_start(self),
            ActionType::OSC_End => emu.OSC_end(self),
            /* Ignore, Hook, Put, Unhook: base-class no-op */
            ActionType::Ignore | ActionType::Hook | ActionType::Put | ActionType::Unhook => {}
        }
    }
}

/// user keystroke -- not part of the host-source state machine
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserByte {
    /// The user-source byte. We don't try to interpret the charset
    pub c: u8,
}

impl UserByte {
    /// A user keystroke byte.
    pub fn new(s_c: u8) -> UserByte {
        UserByte { c: s_c }
    }

    /// Translates the byte for the host (cursor-key mode) and queues it in `terminal_to_host`.
    pub fn act_on_terminal(&self, emu: &mut Emulator) {
        let s = emu.user.input(self, emu.fb.ds.application_mode_cursor_keys);
        emu.dispatch.terminal_to_host.extend_from_slice(&s);
    }
}

/// resize event -- not part of the host-source state machine
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resize {
    /// New width in columns.
    pub width: usize,
    /// New height in rows.
    pub height: usize,
}

impl Resize {
    /// A resize to `s_width` x `s_height`.
    pub fn new(s_width: usize, s_height: usize) -> Resize {
        Resize {
            width: s_width,
            height: s_height,
        }
    }

    /// Resizes the emulator's framebuffer.
    pub fn act_on_terminal(&self, emu: &mut Emulator) {
        emu.resize(self.width, self.height);
    }
}
