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
// Ported from: upstream/src/terminal/terminaldispatcher.h, upstream/src/terminal/terminaldispatcher.cc

//! Parameter collection and function dispatch tables. Port of upstream `src/terminal/terminaldispatcher.{h,cc}`.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::parseraction::{wchar_t, Action};
use crate::terminalframebuffer::Framebuffer;

/// Which dispatch table a sequence is looked up in.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Function_Type {
    /// ESC sequences, keyed by intermediates plus final byte.
    ESCAPE,
    /// CSI sequences, keyed by private marker, intermediates and final byte.
    CSI,
    /// C0/C1 controls, keyed by the control byte.
    CONTROL,
}

/// A registered terminal function.
#[derive(Clone, Copy)]
pub struct Function {
    /// The function to run.
    pub function: fn(&mut Framebuffer, &mut Dispatcher),
    /// Whether dispatching it clears `next_print_will_wrap` first.
    pub clears_wrap_state: bool,
}

/// Dispatch key to function.
// Upstream name, kept so the port can be compared line by line.
#[allow(non_camel_case_types)]
pub type dispatch_map_t = BTreeMap<Vec<u8>, Function>;

/// The three dispatch tables.
pub struct DispatchRegistry {
    /// ESC sequences.
    pub escape: dispatch_map_t,
    /// CSI sequences.
    // Upstream name, kept so the port can be compared line by line.
    #[allow(non_snake_case)]
    pub CSI: dispatch_map_t,
    /// Control characters.
    pub control: dispatch_map_t,
}

impl DispatchRegistry {
    fn new() -> DispatchRegistry {
        DispatchRegistry {
            escape: BTreeMap::new(),
            CSI: BTreeMap::new(),
            control: BTreeMap::new(),
        }
    }

    /// Upstream's `register_function`, called from each `Function`
    /// constructor. `std::map::insert` keeps the first entry for a key.
    pub fn register_function(
        &mut self,
        ty: Function_Type,
        dispatch_chars: &[u8],
        s_function: fn(&mut Framebuffer, &mut Dispatcher),
        s_clears_wrap_state: bool,
    ) {
        let f = Function {
            function: s_function,
            clears_wrap_state: s_clears_wrap_state,
        };
        let map = match ty {
            Function_Type::ESCAPE => &mut self.escape,
            Function_Type::CSI => &mut self.CSI,
            Function_Type::CONTROL => &mut self.control,
        };
        map.entry(dispatch_chars.to_vec()).or_insert(f);
    }
}

/// Upstream builds the registry from static `Function` objects in
/// terminalfunctions.cc; here terminalfunctions.rs registers the same list
/// on first use.
pub fn get_global_dispatch_registry() -> &'static DispatchRegistry {
    static GLOBAL_DISPATCH_REGISTRY: OnceLock<DispatchRegistry> = OnceLock::new();
    GLOBAL_DISPATCH_REGISTRY.get_or_init(|| {
        let mut r = DispatchRegistry::new();
        crate::terminalfunctions::register_all(&mut r);
        r
    })
}

const MAXIMUM_CLIPBOARD_SIZE: usize = 16 * 1024;

/// Upstream `Terminal::Dispatcher`: collects parameters, intermediates and OSC strings, and dispatches functions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dispatcher {
    params: Vec<u8>,
    parsed_params: Vec<i32>,
    parsed: bool,

    dispatch_chars: Vec<u8>,
    // Upstream name, kept so the port can be compared line by line.
    #[allow(non_snake_case)]
    pub(crate) OSC_string: Vec<wchar_t>,

    /// this is the reply string
    pub terminal_to_host: Vec<u8>,
}

/// `strtol( segment_begin, &endptr, 10 )` followed by upstream's checks, for
/// a segment that holds only ASCII digits (newparamchar guarantees it).
fn parse_one_param(segment: &[u8]) -> i32 {
    if segment.is_empty() {
        return -1; /* endptr == segment_begin */
    }
    let mut val: i64 = 0;
    for &d in segment {
        match val
            .checked_mul(10)
            .and_then(|v| v.checked_add((d - b'0') as i64))
        {
            Some(v) => val = v,
            None => return -1, /* errno == ERANGE */
        }
    }
    if val > Dispatcher::PARAM_MAX as i64 {
        return -1;
    }
    val as i32
}

impl Dispatcher {
    /// prevent evil escape sequences from causing long loops
    pub const PARAM_MAX: i32 = 65535;

    /// An empty dispatcher.
    pub fn new() -> Dispatcher {
        Dispatcher::default()
    }

    /// Appends a parameter digit or ';' (at most 100 bytes are kept).
    pub fn newparamchar(&mut self, act: &Action) {
        assert!(act.char_present);
        assert!((act.ch == b';' as u32) || ((act.ch >= b'0' as u32) && (act.ch <= b'9' as u32)));
        if self.params.len() < 100 {
            /* enough for 16 five-char params plus 15 semicolons */
            self.params.push(act.ch as u8);
        }
        self.parsed = false;
    }

    /// Appends an intermediate byte (at most 8, and only 8-bit values).
    pub fn collect(&mut self, act: &Action) {
        assert!(act.char_present);
        if (self.dispatch_chars.len() < 8) /* never should need more than 2 */
            && (act.ch <= 255)
        {
            /* ignore non-8-bit */
            self.dispatch_chars.push(act.ch as u8);
        }
    }

    /// Clears parameters and intermediates.
    pub fn clear(&mut self, _act: &Action) {
        self.params.clear();
        self.dispatch_chars.clear();
        self.parsed = false;
    }

    fn parse_params(&mut self) {
        if self.parsed {
            return;
        }

        self.parsed_params.clear();
        /* every segment, including the last one after the final ';' */
        for segment in self.params.split(|&c| c == b';') {
            self.parsed_params.push(parse_one_param(segment));
        }

        self.parsed = true;
    }

    /// Parameter `n`, or `defaultval` when it is missing, empty, zero, or above `PARAM_MAX`.
    pub fn getparam(&mut self, n: usize, defaultval: i32) -> i32 {
        let mut ret = defaultval;
        if !self.parsed {
            self.parse_params();
        }

        if self.parsed_params.len() > n {
            ret = self.parsed_params[n];
        }

        if ret < 1 {
            ret = defaultval;
        }

        ret
    }

    /// Number of parameters (an empty parameter string counts as one).
    pub fn param_count(&mut self) -> i32 {
        if !self.parsed {
            self.parse_params();
        }

        self.parsed_params.len() as i32
    }

    /// Debug description, truncated to 63 bytes as upstream's 64-byte buffer does.
    pub fn str(&self) -> String {
        let mut s = format!(
            "[dispatch=\"{}\" params=\"{}\"]",
            String::from_utf8_lossy(&self.dispatch_chars),
            String::from_utf8_lossy(&self.params)
        );
        /* snprintf( assum, 64, ... ) */
        if s.len() > 63 {
            let mut end = 63;
            while !s.is_char_boundary(end) {
                end -= 1;
            }
            s.truncate(end);
        }
        s
    }

    /// Looks up and runs the function for a completed sequence; unknown sequences only clear the wrap state.
    pub fn dispatch(&mut self, ty: Function_Type, act: &Action, fb: &mut Framebuffer) {
        /* add final char to dispatch key */
        if (ty == Function_Type::ESCAPE) || (ty == Function_Type::CSI) {
            assert!(act.char_present);
            let act2 = Action {
                kind: crate::parseraction::ActionType::Collect,
                ch: act.ch,
                char_present: true,
            };
            self.collect(&act2);
        }

        let registry = get_global_dispatch_registry();
        let map = match ty {
            Function_Type::ESCAPE => &registry.escape,
            Function_Type::CSI => &registry.CSI,
            Function_Type::CONTROL => &registry.control,
        };

        let key: Vec<u8> = if ty == Function_Type::CONTROL {
            assert!(act.ch <= 255);
            vec![act.ch as u8]
        } else {
            self.dispatch_chars.clone()
        };

        let i = match map.get(&key) {
            None => {
                /* unknown function */
                fb.ds.next_print_will_wrap = false;
                return;
            }
            Some(i) => *i,
        };
        if i.clears_wrap_state {
            fb.ds.next_print_will_wrap = false;
        }
        (i.function)(fb, self);
    }

    /// The collected intermediates plus final byte.
    pub fn get_dispatch_chars(&self) -> &[u8] {
        &self.dispatch_chars
    }
    /// The collected OSC string.
    // Upstream name, kept so the port can be compared line by line.
    #[allow(non_snake_case)]
    pub fn get_OSC_string(&self) -> &[wchar_t] {
        &self.OSC_string
    }

    /// Appends a character to the OSC string (at most 16 KiB).
    // Upstream name, kept so the port can be compared line by line.
    #[allow(non_snake_case)]
    pub fn OSC_put(&mut self, act: &Action) {
        assert!(act.char_present);
        if self.OSC_string.len() < MAXIMUM_CLIPBOARD_SIZE {
            self.OSC_string.push(act.ch);
        }
    }

    /// Clears the OSC string.
    // Upstream name, kept so the port can be compared line by line.
    #[allow(non_snake_case)]
    pub fn OSC_start(&mut self, _act: &Action) {
        self.OSC_string.clear();
    }
}
