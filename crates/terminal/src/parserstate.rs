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
// Ported from: upstream/src/terminal/parserstate.h, upstream/src/terminal/parserstate.cc,
//              upstream/src/terminal/parserstatefamily.h

use crate::parseraction::{wchar_t, Action, ActionType};
use crate::parsertransition::Transition;

/// One member of upstream's `StateFamily`. Upstream has one class per state
/// and a family object holding one instance of each; the states are
/// stateless, so here each is an enum value.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Ground,

    Escape,
    Escape_Intermediate,

    CSI_Entry,
    CSI_Param,
    CSI_Intermediate,
    CSI_Ignore,

    DCS_Entry,
    DCS_Param,
    DCS_Intermediate,
    DCS_Passthrough,
    DCS_Ignore,

    OSC_String,
    SOS_PM_APC_String,
}

use ActionType::*;
use State::*;

impl State {
    fn anywhere_rule(self, ch: wchar_t) -> Transition {
        if (ch == 0x18)
            || (ch == 0x1A)
            || ((0x80..=0x8F).contains(&ch))
            || ((0x91..=0x97).contains(&ch))
            || (ch == 0x99)
            || (ch == 0x9A)
        {
            return Transition::act_to(Execute, Ground);
        } else if ch == 0x9C {
            return Transition::to(Ground);
        } else if ch == 0x1B {
            return Transition::to(Escape);
        } else if (ch == 0x98) || (ch == 0x9E) || (ch == 0x9F) {
            return Transition::to(SOS_PM_APC_String);
        } else if ch == 0x90 {
            return Transition::to(DCS_Entry);
        } else if ch == 0x9D {
            return Transition::to(OSC_String);
        } else if ch == 0x9B {
            return Transition::to(CSI_Entry);
        }

        Transition::none() /* don't allocate an Ignore action */
    }

    pub fn input(self, ch: wchar_t) -> Transition {
        /* Check for immediate transitions. */
        let mut anywhere = self.anywhere_rule(ch);
        if anywhere.next_state.is_some() {
            anywhere.action.char_present = true;
            anywhere.action.ch = ch;
            return anywhere;
        }
        /* Normal X.364 state machine. */
        /* Parse high Unicode codepoints like 'A'. */
        let mut ret = self.input_state_rule(if ch >= 0xA0 { 0x41 } else { ch });
        ret.action.char_present = true;
        ret.action.ch = ch;
        ret
    }

    pub fn enter(self) -> Action {
        match self {
            Escape => Action::new(Clear),
            CSI_Entry => Action::new(Clear),
            DCS_Entry => Action::new(Clear),
            DCS_Passthrough => Action::new(Hook),
            OSC_String => Action::new(OSC_Start),
            _ => Action::new(Ignore),
        }
    }

    pub fn exit(self) -> Action {
        match self {
            DCS_Passthrough => Action::new(Unhook),
            OSC_String => Action::new(OSC_End),
            _ => Action::new(Ignore),
        }
    }

    fn input_state_rule(self, ch: wchar_t) -> Transition {
        match self {
            Ground => ground_rule(ch),
            Escape => escape_rule(ch),
            Escape_Intermediate => escape_intermediate_rule(ch),
            CSI_Entry => csi_entry_rule(ch),
            CSI_Param => csi_param_rule(ch),
            CSI_Intermediate => csi_intermediate_rule(ch),
            CSI_Ignore => csi_ignore_rule(ch),
            DCS_Entry => dcs_entry_rule(ch),
            DCS_Param => dcs_param_rule(ch),
            DCS_Intermediate => dcs_intermediate_rule(ch),
            DCS_Passthrough => dcs_passthrough_rule(ch),
            DCS_Ignore => dcs_ignore_rule(ch),
            OSC_String => osc_string_rule(ch),
            SOS_PM_APC_String => sos_pm_apc_string_rule(ch),
        }
    }
}

#[allow(non_snake_case)]
fn C0_prime(ch: wchar_t) -> bool {
    (ch <= 0x17) || (ch == 0x19) || ((0x1C..=0x1F).contains(&ch))
}

#[allow(non_snake_case)]
fn GLGR(ch: wchar_t) -> bool {
    ((0x20..=0x7F).contains(&ch)) /* GL area */
        || ((0xA0..=0xFF).contains(&ch)) /* GR area */
}

fn ground_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if GLGR(ch) {
        return Transition::act(Print);
    }

    Transition::none()
}

fn escape_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if (0x20..=0x2F).contains(&ch) {
        return Transition::act_to(Collect, Escape_Intermediate);
    }

    if ((0x30..=0x4F).contains(&ch))
        || ((0x51..=0x57).contains(&ch))
        || (ch == 0x59)
        || (ch == 0x5A)
        || (ch == 0x5C)
        || ((0x60..=0x7E).contains(&ch))
    {
        return Transition::act_to(Esc_Dispatch, Ground);
    }

    if ch == 0x5B {
        return Transition::to(CSI_Entry);
    }

    if ch == 0x5D {
        return Transition::to(OSC_String);
    }

    if ch == 0x50 {
        return Transition::to(DCS_Entry);
    }

    if (ch == 0x58) || (ch == 0x5E) || (ch == 0x5F) {
        return Transition::to(SOS_PM_APC_String);
    }

    Transition::none()
}

fn escape_intermediate_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if (0x20..=0x2F).contains(&ch) {
        return Transition::act(Collect);
    }

    if (0x30..=0x7E).contains(&ch) {
        return Transition::act_to(Esc_Dispatch, Ground);
    }

    Transition::none()
}

fn csi_entry_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::act_to(CSI_Dispatch, Ground);
    }

    if ((0x30..=0x39).contains(&ch)) || (ch == 0x3B) {
        return Transition::act_to(Param, CSI_Param);
    }

    if (0x3C..=0x3F).contains(&ch) {
        return Transition::act_to(Collect, CSI_Param);
    }

    if ch == 0x3A {
        return Transition::to(CSI_Ignore);
    }

    if (0x20..=0x2F).contains(&ch) {
        return Transition::act_to(Collect, CSI_Intermediate);
    }

    Transition::none()
}

fn csi_param_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if ((0x30..=0x39).contains(&ch)) || (ch == 0x3B) {
        return Transition::act(Param);
    }

    if (ch == 0x3A) || ((0x3C..=0x3F).contains(&ch)) {
        return Transition::to(CSI_Ignore);
    }

    if (0x20..=0x2F).contains(&ch) {
        return Transition::act_to(Collect, CSI_Intermediate);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::act_to(CSI_Dispatch, Ground);
    }

    Transition::none()
}

fn csi_intermediate_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if (0x20..=0x2F).contains(&ch) {
        return Transition::act(Collect);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::act_to(CSI_Dispatch, Ground);
    }

    if (0x30..=0x3F).contains(&ch) {
        return Transition::to(CSI_Ignore);
    }

    Transition::none()
}

fn csi_ignore_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) {
        return Transition::act(Execute);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::to(Ground);
    }

    Transition::none()
}

fn dcs_entry_rule(ch: wchar_t) -> Transition {
    if (0x20..=0x2F).contains(&ch) {
        return Transition::act_to(Collect, DCS_Intermediate);
    }

    if ch == 0x3A {
        return Transition::to(DCS_Ignore);
    }

    if ((0x30..=0x39).contains(&ch)) || (ch == 0x3B) {
        return Transition::act_to(Param, DCS_Param);
    }

    if (0x3C..=0x3F).contains(&ch) {
        return Transition::act_to(Collect, DCS_Param);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::to(DCS_Passthrough);
    }

    Transition::none()
}

fn dcs_param_rule(ch: wchar_t) -> Transition {
    if ((0x30..=0x39).contains(&ch)) || (ch == 0x3B) {
        return Transition::act(Param);
    }

    if (ch == 0x3A) || ((0x3C..=0x3F).contains(&ch)) {
        return Transition::to(DCS_Ignore);
    }

    if (0x20..=0x2F).contains(&ch) {
        return Transition::act_to(Collect, DCS_Intermediate);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::to(DCS_Passthrough);
    }

    Transition::none()
}

fn dcs_intermediate_rule(ch: wchar_t) -> Transition {
    if (0x20..=0x2F).contains(&ch) {
        return Transition::act(Collect);
    }

    if (0x40..=0x7E).contains(&ch) {
        return Transition::to(DCS_Passthrough);
    }

    if (0x30..=0x3F).contains(&ch) {
        return Transition::to(DCS_Ignore);
    }

    Transition::none()
}

fn dcs_passthrough_rule(ch: wchar_t) -> Transition {
    if C0_prime(ch) || ((0x20..=0x7E).contains(&ch)) {
        return Transition::act(Put);
    }

    if ch == 0x9C {
        return Transition::to(Ground);
    }

    Transition::none()
}

fn dcs_ignore_rule(ch: wchar_t) -> Transition {
    if ch == 0x9C {
        return Transition::to(Ground);
    }

    Transition::none()
}

fn osc_string_rule(ch: wchar_t) -> Transition {
    if (0x20..=0x7F).contains(&ch) {
        return Transition::act(OSC_Put);
    }

    if (ch == 0x9C) || (ch == 0x07) {
        /* 0x07 is xterm non-ANSI variant */
        return Transition::to(Ground);
    }

    Transition::none()
}

fn sos_pm_apc_string_rule(ch: wchar_t) -> Transition {
    if ch == 0x9C {
        return Transition::to(Ground);
    }

    Transition::none()
}
