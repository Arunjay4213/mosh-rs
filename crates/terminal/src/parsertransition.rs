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
// Ported from: upstream/src/terminal/parsertransition.h

//! A parser transition: an action plus an optional next state. Port of upstream `src/terminal/parsertransition.h`.

use crate::parseraction::{Action, ActionType};
use crate::parserstate::State;

/// Transition is only a courier for an Action.
#[derive(Clone, Copy, Debug)]
pub struct Transition {
    /// The action to run.
    pub action: Action,
    /// The state to move to, or None to stay.
    pub next_state: Option<State>,
}

impl Transition {
    /// `Transition( ActionPointer s_action = Ignore, State* s_next_state = NULL )`
    pub fn new(s_action: ActionType, s_next_state: Option<State>) -> Transition {
        Transition {
            action: Action::new(s_action),
            next_state: s_next_state,
        }
    }

    /// `Transition()`: an Ignore action and no state change.
    pub fn none() -> Transition {
        Transition::new(ActionType::Ignore, None)
    }

    /// The one-argument form: `Transition( State* s_next_state )` with an Ignore action.
    pub fn to(s_next_state: State) -> Transition {
        Transition::new(ActionType::Ignore, Some(s_next_state))
    }

    /// `Transition( std::make_shared<X>() )`: action, no state change.
    pub fn act(s_action: ActionType) -> Transition {
        Transition::new(s_action, None)
    }

    /// `Transition( std::make_shared<X>(), &family->s_Y )`
    pub fn act_to(s_action: ActionType, s_next_state: State) -> Transition {
        Transition::new(s_action, Some(s_next_state))
    }
}
