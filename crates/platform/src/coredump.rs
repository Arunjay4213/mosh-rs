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
// Ported from: upstream/src/crypto/crypto.cc (disable_dumping_core, reenable_dumping_core)

//! Core dump control, from upstream `src/crypto/crypto.cc`: the client and
//! server turn core dumps off early in `main()` so the session key never
//! lands on disk. Unix only, as upstream.

#[cfg(unix)]
use std::sync::Mutex;

#[cfg(unix)]
static SAVED_CORE_RLIMIT: Mutex<libc::rlim_t> = Mutex::new(0);

/// Disable dumping core, as a precaution to avoid saving sensitive data
/// to disk. Exits the process if the limit cannot be read or set.
#[cfg(unix)]
pub fn disable_dumping_core() {
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: getrlimit writes one rlimit through a pointer to a live local.
    if unsafe { libc::getrlimit(libc::RLIMIT_CORE, &mut limit) } != 0 {
        /* We don't throw CryptoException because this is called very early
        in main(), outside of 'try'. */
        crate::stdio::perror("getrlimit(RLIMIT_CORE)", &std::io::Error::last_os_error());
        std::process::exit(1);
    }
    *SAVED_CORE_RLIMIT.lock().unwrap_or_else(|e| e.into_inner()) = limit.rlim_cur;
    limit.rlim_cur = 0;
    // SAFETY: setrlimit reads one rlimit through a pointer to a live local.
    if unsafe { libc::setrlimit(libc::RLIMIT_CORE, &limit) } != 0 {
        crate::stdio::perror("setrlimit(RLIMIT_CORE)", &std::io::Error::last_os_error());
        std::process::exit(1);
    }
}

/// Restore the core dump limit saved by `disable_dumping_core`.
/// Silent failure is safe.
#[cfg(unix)]
pub fn reenable_dumping_core() {
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: getrlimit writes one rlimit through a pointer to a live local.
    if unsafe { libc::getrlimit(libc::RLIMIT_CORE, &mut limit) } == 0 {
        limit.rlim_cur = *SAVED_CORE_RLIMIT.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: setrlimit reads one rlimit through a pointer to a live local.
        unsafe { libc::setrlimit(libc::RLIMIT_CORE, &limit) };
    }
}

/// Windows writes no core files, so there is nothing to disable; this lets
/// `mosh-client` call the same function on both systems.
#[cfg(windows)]
pub fn disable_dumping_core() {}
