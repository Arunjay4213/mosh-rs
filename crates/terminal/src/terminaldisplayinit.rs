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
// PLAN.md section 9.6: there is no terminfo lookup. Upstream asks terminfo
// (via setupterm/tigetstr/tigetflag) for "ech", "bce", "smcup" and
// "rmcup". Here the same control flow reads those four answers from a fixed
// table keyed on TERM: xterm-class defaults, with overrides for the terminals
// known to lack ECH and BCE. The values are what ncurses 6.4 terminfo on the
// devbox returns (`infocmp -1 xterm`, `infocmp -1 screen`, `infocmp -1 tmux`).

//! Choosing the local terminal's capabilities. Port of upstream `src/terminal/terminaldisplayinit.cc`, without terminfo (PLAN.md 9.6).

use crate::terminaldisplay::Display;

/// The terminfo answers upstream would get for one terminal type.
struct TermCaps {
    ech: bool,
    bce: bool,
    smcup: &'static str,
    rmcup: &'static str,
}

/// xterm, xterm-256color, alacritty and most modern emulators.
const XTERM_CLASS: TermCaps = TermCaps {
    ech: true,
    bce: true,
    smcup: "\x1b[?1049h\x1b[22;0;0t",
    rmcup: "\x1b[?1049l\x1b[23;0;0t",
};

/// "screen", "screen-256color", "tmux", "tmux-256color": no ech, no bce.
const SCREEN_CLASS: TermCaps = TermCaps {
    ech: false,
    bce: false,
    smcup: "\x1b[?1049h",
    rmcup: "\x1b[?1049l",
};

fn term_caps(term_type: Option<&str>) -> &'static TermCaps {
    match term_type {
        Some(t) if t.starts_with("screen") || t.starts_with("tmux") => &SCREEN_CLASS,
        _ => &XTERM_CLASS,
    }
}

impl Display {
    /// Terminal capabilities; with `use_environment`, chosen from `TERM` and `MOSH_NO_TERM_INIT` (no terminfo, PLAN.md 9.6).
    pub fn new(use_environment: bool) -> Display {
        let mut d = Display {
            has_ech: true,
            has_bce: true,
            has_title: true,
            smcup: None,
            rmcup: None,
        };

        if use_environment {
            let term_type = std::env::var("TERM").ok();
            let caps = term_caps(term_type.as_deref());

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
            if let Some(term_type) = term_type.as_deref() {
                for t in TITLE_TERM_TYPES {
                    if term_type.starts_with(t) {
                        d.has_title = true;
                        break;
                    }
                }
            }

            if std::env::var_os("MOSH_NO_TERM_INIT").is_none() {
                d.smcup = Some(caps.smcup);
                d.rmcup = Some(caps.rmcup);
            }
        }
        d
    }
}
