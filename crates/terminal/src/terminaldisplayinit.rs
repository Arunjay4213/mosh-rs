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
// "rmcup". Here the same control flow reads those four answers from the fixed
// table `TERMINFO` below, keyed on TERM, whose rows are copied from
// `infocmp -1 <name>` on devbox (ncurses 6.4). Unknown TERMs get xterm's.

//! Choosing the local terminal's capabilities. Port of upstream `src/terminal/terminaldisplayinit.cc`, without terminfo (PLAN.md 9.6).

use crate::terminaldisplay::Display;

/// The terminfo answers upstream would get for one terminal type.
/// `None` for smcup/rmcup means the capability is absent (tigetstr NULL).
struct TermCaps {
    ech: bool,
    bce: bool,
    smcup: Option<&'static str>,
    rmcup: Option<&'static str>,
}

const fn caps(
    ech: bool,
    bce: bool,
    smcup: Option<&'static str>,
    rmcup: Option<&'static str>,
) -> TermCaps {
    TermCaps {
        ech,
        bce,
        smcup,
        rmcup,
    }
}

const XTERM_ALT: (Option<&str>, Option<&str>) = (
    Some("\x1b[?1049h\x1b[22;0;0t"),
    Some("\x1b[?1049l\x1b[23;0;0t"),
);
const PLAIN_ALT: (Option<&str>, Option<&str>) = (Some("\x1b[?1049h"), Some("\x1b[?1049l"));
const RXVT_ALT: (Option<&str>, Option<&str>) =
    (Some("\x1b7\x1b[?47h"), Some("\x1b[2J\x1b[?47l\x1b8"));
const URXVT_ALT: (Option<&str>, Option<&str>) = (Some("\x1b[?1049h"), Some("\x1b[r\x1b[?1049l"));
const NO_ALT: (Option<&str>, Option<&str>) = (None, None);

/// ech, bce, smcup and rmcup per TERM, copied from `infocmp -1 <name>`
/// (ncurses 6.4.20240113 terminfo on devbox, Ubuntu 24.04).
const TERMINFO: &[(&str, TermCaps)] = &[
    ("xterm", caps(true, true, XTERM_ALT.0, XTERM_ALT.1)),
    ("xterm-256color", caps(true, true, XTERM_ALT.0, XTERM_ALT.1)),
    ("screen", caps(false, false, PLAIN_ALT.0, PLAIN_ALT.1)),
    (
        "screen-256color",
        caps(false, false, PLAIN_ALT.0, PLAIN_ALT.1),
    ),
    ("tmux", caps(false, false, PLAIN_ALT.0, PLAIN_ALT.1)),
    (
        "tmux-256color",
        caps(false, false, PLAIN_ALT.0, PLAIN_ALT.1),
    ),
    ("rxvt", caps(false, true, RXVT_ALT.0, RXVT_ALT.1)),
    ("rxvt-256color", caps(false, true, RXVT_ALT.0, RXVT_ALT.1)),
    ("rxvt-unicode", caps(true, true, URXVT_ALT.0, URXVT_ALT.1)),
    (
        "rxvt-unicode-256color",
        caps(true, true, URXVT_ALT.0, URXVT_ALT.1),
    ),
    ("linux", caps(true, true, NO_ALT.0, NO_ALT.1)),
    ("vt100", caps(false, false, NO_ALT.0, NO_ALT.1)),
    ("vt102", caps(false, false, NO_ALT.0, NO_ALT.1)),
    ("vt220", caps(true, false, NO_ALT.0, NO_ALT.1)),
    ("alacritty", caps(true, true, XTERM_ALT.0, XTERM_ALT.1)),
    ("kitty", caps(true, false, PLAIN_ALT.0, PLAIN_ALT.1)),
    ("foot", caps(true, true, XTERM_ALT.0, XTERM_ALT.1)),
    ("st-256color", caps(true, true, PLAIN_ALT.0, PLAIN_ALT.1)),
    ("putty", caps(true, true, PLAIN_ALT.0, PLAIN_ALT.1)),
    ("dumb", caps(false, false, NO_ALT.0, NO_ALT.1)),
];

/// The capabilities for `term_type`: an exact entry from `TERMINFO`, else the
/// entry of its family (a TERM that starts with "screen", "tmux",
/// "rxvt-unicode" or "rxvt" behaves like the base entry in terminfo), else
/// xterm's.
fn term_caps(term_type: Option<&str>) -> &'static TermCaps {
    let lookup = |name: &str| {
        TERMINFO
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, c)| c)
            .expect("name is in TERMINFO")
    };
    let Some(t) = term_type else {
        return lookup("xterm");
    };
    if let Some((_, c)) = TERMINFO.iter().find(|(n, _)| *n == t) {
        return c;
    }
    for family in ["screen", "tmux", "rxvt-unicode", "rxvt"] {
        if t.starts_with(family) {
            return lookup(family);
        }
    }
    lookup("xterm")
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
                d.smcup = caps.smcup;
                d.rmcup = caps.rmcup;
            }
        }
        d
    }
}
