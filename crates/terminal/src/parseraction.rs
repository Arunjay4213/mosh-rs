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

use crate::terminal::Emulator;

/// Upstream `wchar_t`. On the arm64 Linux build that serves as the oracle it
/// is a 32-bit unsigned integer; every value the parser produces is at most
/// 0x10FFFF (or the -1 default, which no code path reads).
#[allow(non_camel_case_types)]
pub type wchar_t = u32;

/// The concrete action classes of upstream's `Parser::Action` hierarchy.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionType {
    Ignore,
    Print,
    Execute,
    Clear,
    Collect,
    Param,
    Esc_Dispatch,
    CSI_Dispatch,
    Hook,
    Put,
    Unhook,
    OSC_Start,
    OSC_Put,
    OSC_End,
}

/// Upstream `Parser::Action` (host-source actions only; `UserByte` and
/// `Resize` are separate structs below, as they are not produced by the
/// state machine).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Action {
    pub kind: ActionType,
    pub ch: wchar_t,
    pub char_present: bool,
}

pub type Actions = Vec<Action>;

impl Action {
    /// `Action() : ch( -1 ), char_present( false )`
    pub fn new(kind: ActionType) -> Action {
        Action { kind, ch: u32::MAX, char_present: false }
    }

    pub fn name(&self) -> &'static str {
        match self.kind {
            ActionType::Ignore => "Ignore",
            ActionType::Print => "Print",
            ActionType::Execute => "Execute",
            ActionType::Clear => "Clear",
            ActionType::Collect => "Collect",
            ActionType::Param => "Param",
            ActionType::Esc_Dispatch => "Esc_Dispatch",
            ActionType::CSI_Dispatch => "CSI_Dispatch",
            ActionType::Hook => "Hook",
            ActionType::Put => "Put",
            ActionType::Unhook => "Unhook",
            ActionType::OSC_Start => "OSC_Start",
            ActionType::OSC_Put => "OSC_Put",
            ActionType::OSC_End => "OSC_End",
        }
    }

    pub fn ignore(&self) -> bool {
        self.kind == ActionType::Ignore
    }

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
    pub fn new(s_c: u8) -> UserByte {
        UserByte { c: s_c }
    }

    pub fn name(&self) -> &'static str {
        "UserByte"
    }

    pub fn act_on_terminal(&self, emu: &mut Emulator) {
        let s = emu.user.input(self, emu.fb.ds.application_mode_cursor_keys);
        emu.dispatch.terminal_to_host.extend_from_slice(&s);
    }
}

/// resize event -- not part of the host-source state machine
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resize {
    pub width: usize,
    pub height: usize,
}

impl Resize {
    pub fn new(s_width: usize, s_height: usize) -> Resize {
        Resize { width: s_width, height: s_height }
    }

    pub fn name(&self) -> &'static str {
        "Resize"
    }

    pub fn act_on_terminal(&self, emu: &mut Emulator) {
        emu.resize(self.width, self.height);
    }
}
