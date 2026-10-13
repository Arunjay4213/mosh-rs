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
//
// Ported from: upstream/src/util/locale_utils.h, upstream/src/util/locale_utils.cc
// (and the variable list of `locale_vars` in upstream/scripts/mosh.pl).
// On Windows the console is switched to the UTF-8 code page by
// `tty::raw_mode_on`, so the charset is always UTF-8 there.

//! Locale checks, ported from upstream `src/util/locale_utils.cc`.

/// The locale variables, in upstream order. mosh.pl forwards the ones that
/// are set to the server (`-l NAME=VALUE`); `clear_locale_variables` unsets
/// all of them.
pub const LOCALE_VARS: [&str; 15] = [
    "LANG",
    "LANGUAGE",
    "LC_CTYPE",
    "LC_NUMERIC",
    "LC_TIME",
    "LC_COLLATE",
    "LC_MONETARY",
    "LC_MESSAGES",
    "LC_PAPER",
    "LC_NAME",
    "LC_ADDRESS",
    "LC_TELEPHONE",
    "LC_MEASUREMENT",
    "LC_IDENTIFICATION",
    "LC_ALL",
];

/// A locale variable and its value, as upstream `LocaleVar`.
pub struct LocaleVar {
    /// Variable name, empty when none is set.
    pub name: String,
    /// Its value.
    pub value: String,
}

impl LocaleVar {
    /// `NAME=value`, or `[no charset variables]`.
    pub fn str(&self) -> String {
        if self.name.is_empty() {
            return "[no charset variables]".to_string();
        }
        format!("{}={}", self.name, self.value)
    }
}

/// Reimplement the search logic, just for diagnostics.
pub fn get_ctype() -> LocaleVar {
    for name in ["LC_ALL", "LC_CTYPE", "LANG"] {
        if let Some(value) = std::env::var_os(name) {
            return LocaleVar {
                name: name.to_string(),
                value: value.to_string_lossy().into_owned(),
            };
        }
    }
    LocaleVar {
        name: String::new(),
        value: String::new(),
    }
}

/// The charset of the current locale (`nl_langinfo(CODESET)`), with
/// `ANSI_X3.4-1968` shown as `US-ASCII`.
#[cfg(unix)]
pub fn locale_charset() -> String {
    // SAFETY: nl_langinfo returns a pointer to a static NUL-terminated string.
    let ret = unsafe { std::ffi::CStr::from_ptr(libc::nl_langinfo(libc::CODESET)) };
    let ret = ret.to_string_lossy().into_owned();
    /* Produce more pleasant name of US-ASCII */
    if ret == "ANSI_X3.4-1968" {
        return "US-ASCII".to_string();
    }
    ret
}

/// Always `UTF-8`: the console code page is set to UTF-8 in raw mode.
#[cfg(windows)]
pub fn locale_charset() -> String {
    "UTF-8".to_string()
}

/// Whether the locale's charset is UTF-8.
pub fn is_utf8_locale() -> bool {
    /* Verify locale calls for UTF-8 */
    let charset = locale_charset();
    charset == "UTF-8" || charset == "utf-8"
}

/// Adopt native locale.
#[cfg(unix)]
pub fn set_native_locale() {
    // SAFETY: setlocale with an empty string reads the environment.
    if unsafe { libc::setlocale(libc::LC_ALL, c"".as_ptr()) }.is_null() {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() == Some(libc::ENOENT) {
            let ctype = get_ctype();
            eprintln!(
                "The locale requested by {} isn't available here.",
                ctype.str()
            );
            if !ctype.name.is_empty() {
                eprintln!("Running `locale-gen {}' may be necessary.\n", ctype.value);
            }
        } else {
            eprintln!("setlocale: {err}");
        }
    }
}

/// Nothing to adopt on Windows.
#[cfg(windows)]
pub fn set_native_locale() {}

/// Unset every variable in `LOCALE_VARS`.
pub fn clear_locale_variables() {
    for name in LOCALE_VARS {
        std::env::remove_var(name);
    }
}
