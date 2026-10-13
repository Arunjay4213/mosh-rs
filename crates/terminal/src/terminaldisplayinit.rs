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
// Ported from: upstream/src/terminal/terminaldisplayinit.cc
//
// PLAN.md section 9.6: there is no runtime terminfo lookup. Upstream calls
// setupterm() for $TERM and reads "ech", "bce", "smcup" and "rmcup"; here the
// same control flow reads those answers from `crate::terminfo`, a table
// generated from every entry of the devbox terminfo database by
// ref/ref-term/gen-terminfo.c. An unknown TERM (where upstream would fail to
// start) gets xterm's answers.

//! Choosing the local terminal's capabilities. Port of upstream
//! `src/terminal/terminaldisplayinit.cc`, with the generated `terminfo` table in
//! place of runtime terminfo (PLAN.md 9.6).

use crate::terminaldisplay::Display;
use crate::terminfo::{lookup, TermCaps};

/// The terminfo answers for `term_type`: its own entry, or xterm's when it is
/// unset or not in the database.
fn term_caps(term_type: Option<&str>) -> &'static TermCaps {
    term_type
        .and_then(lookup)
        .or_else(|| lookup("xterm"))
        .expect("the generated terminfo table has an xterm entry")
}

/// The capability decisions a `Display` made (see `Display::debug_caps`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayCaps {
    /// has_ech
    pub ech: bool,
    /// has_bce
    pub bce: bool,
    /// has_title
    pub title: bool,
    /// smcup bytes, None when absent or MOSH_NO_TERM_INIT is set
    pub smcup: Option<&'static [u8]>,
    /// rmcup bytes, None when absent or MOSH_NO_TERM_INIT is set
    pub rmcup: Option<&'static [u8]>,
}

impl Display {
    /// Terminal capabilities. With `use_environment`, from `TERM` (via the
    /// generated terminfo table) and `MOSH_NO_TERM_INIT`, as upstream.
    pub fn new(use_environment: bool) -> Display {
        let mut d = Display {
            has_ech: true,
            has_bce: true,
            has_title: true,
            smcup: None,
            rmcup: None,
            host_history: false,
        };

        if use_environment {
            // getenv( "TERM" ): raw bytes; a non-UTF-8 value cannot name a
            // table entry, so it gets the xterm fallback.
            let term_os = std::env::var_os("TERM");
            let term_bytes = term_os.as_ref().map(|t| t.as_encoded_bytes());
            let caps = term_caps(term_bytes.and_then(|b| std::str::from_utf8(b).ok()));

            /* check for ECH */
            d.has_ech = caps.ech;

            /* check for BCE */
            d.has_bce = caps.bce;

            /* Check if we can set the window title and icon name.  terminfo does not
            have reliable information on this, so we hardcode a whitelist of
            terminal type prefixes. */
            const TITLE_TERM_TYPES: [&str; 7] = [
                "xterm",
                "rxvt",
                "kterm",
                "Eterm",
                "alacritty",
                "screen",
                "tmux",
            ];

            d.has_title = false;
            if let Some(term_type) = term_bytes {
                for t in TITLE_TERM_TYPES {
                    if term_type.starts_with(t.as_bytes()) {
                        d.has_title = true;
                        break;
                    }
                }
            }

            if std::env::var_os("MOSH_NO_TERM_INIT").is_none() {
                d.smcup = caps.smcup;
                d.rmcup = caps.rmcup;
            }
        }
        d
    }

    /// Debug accessor (not upstream): the decisions upstream's
    /// `Display(true)` makes, for comparing with ref-term's `caps` mode.
    pub fn debug_caps(&self) -> DisplayCaps {
        DisplayCaps {
            ech: self.has_ech,
            bce: self.has_bce,
            title: self.has_title,
            smcup: self.smcup,
            rmcup: self.rmcup,
        }
    }
}
