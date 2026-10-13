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
// Ported from: upstream/src/util/select.h, upstream/src/util/select.cc
// The Windows half is new (no upstream equivalent), following PLAN section 7.

//! Upstream's comment, which still applies on unix: any signals blocked by
//! calling sigprocmask() outside this code will still be received during
//! `Select::select()`. So don't do that.
//!
//! Differences from upstream, all forced by the language: there is no
//! `get_instance()` singleton because only the `got_signal` array has to be
//! global (the signal handler writes it), and `select()` does not call
//! `freeze_timestamp()` because the timestamp lives with the network code;
//! the caller does that right after `select()` returns.

use crate::errln;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::tty::Fd;

const MAX_SIGNAL_NUMBER: usize = 64;
/* Number of 0-timeout selects after which we begin to think
 * something's wrong. */
const MAX_POLLS: u32 = 10;

#[allow(clippy::declare_interior_mutable_const)]
const NO_SIGNAL: AtomicBool = AtomicBool::new(false);
/* We assume writes to got_signal are atomic, though we also try to mask out
concurrent signal handlers. */
static GOT_SIGNAL: [AtomicBool; MAX_SIGNAL_NUMBER + 1] = [NO_SIGNAL; MAX_SIGNAL_NUMBER + 1];
static VERBOSE: AtomicU32 = AtomicU32::new(0);

#[cfg(unix)]
pub use libc::{SIGCONT, SIGHUP, SIGINT, SIGPIPE, SIGTERM, SIGUSR1, SIGWINCH};

/// Windows has no such signals. The numbers exist so the event loop reads
/// the same on both systems; only SIGWINCH is ever raised (by polling the
/// console size), and Ctrl-C arrives as byte 0x03 instead of SIGINT.
#[cfg(windows)]
mod winsig {
    /// Hangup.
    pub const SIGHUP: i32 = 1;
    /// Interrupt.
    pub const SIGINT: i32 = 2;
    /// User signal 1.
    pub const SIGUSR1: i32 = 10;
    /// Broken pipe.
    pub const SIGPIPE: i32 = 13;
    /// Terminate.
    pub const SIGTERM: i32 = 15;
    /// Continue after stop.
    pub const SIGCONT: i32 = 18;
    /// Window size changed.
    pub const SIGWINCH: i32 = 28;
}
#[cfg(windows)]
pub use winsig::*;

fn clear_got_signal() {
    for p in GOT_SIGNAL.iter() {
        p.store(false, Ordering::SeqCst);
    }
}

fn signal_index(signum: i32) -> usize {
    assert!(signum >= 0);
    assert!(signum as usize <= MAX_SIGNAL_NUMBER);
    signum as usize
}

#[cfg(unix)]
extern "C" fn handle_signal(signum: libc::c_int) {
    // No assert here: panicking in a signal handler is not allowed. The
    // handler is only installed for numbers add_signal() checked.
    if let Some(flag) = GOT_SIGNAL.get(signum as usize) {
        flag.store(true, Ordering::SeqCst);
    }
}

/// Waits for readable fds and signals with a timeout, as upstream `Select`.
pub struct Select {
    #[cfg(unix)]
    max_fd: Fd,
    #[cfg(unix)]
    all_fds: libc::fd_set,
    #[cfg(unix)]
    read_fds: libc::fd_set,
    #[cfg(unix)]
    empty_sigset: libc::sigset_t,

    #[cfg(windows)]
    all_fds: Vec<Fd>,
    #[cfg(windows)]
    read_fds: Vec<Fd>,
    #[cfg(windows)]
    watched_size: Option<std::io::Result<(u16, u16)>>,

    consecutive_polls: u32,
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl Select {
    /// Set the verbosity for poll diagnostics.
    pub fn set_verbose(s_verbose: u32) {
        VERBOSE.store(s_verbose, Ordering::Relaxed);
    }

    /// Consumes a signal notification.
    pub fn signal(&mut self, signum: i32) -> bool {
        /* XXX This requires a guard against concurrent signals. */
        GOT_SIGNAL[signal_index(signum)].swap(false, Ordering::SeqCst)
    }

    /// Does not consume signal notifications.
    pub fn any_signal(&self) -> bool {
        let mut rv = false;
        for flag in GOT_SIGNAL.iter().take(MAX_SIGNAL_NUMBER) {
            rv |= flag.load(Ordering::SeqCst);
        }
        rv
    }

    /// The poll rate limiter shared by both systems. Returns the timeout to use.
    fn rate_limit(&mut self, mut timeout: i32) -> i32 {
        let verbose = VERBOSE.load(Ordering::Relaxed);
        /* Rate-limit and warn about polls. */
        if verbose > 1 && timeout == 0 {
            errln!("select: got poll (timeout 0)");
        }
        if timeout == 0 {
            self.consecutive_polls += 1;
        }
        if timeout == 0 && self.consecutive_polls >= MAX_POLLS {
            if verbose > 1 && self.consecutive_polls == MAX_POLLS {
                errln!("select: got {MAX_POLLS} polls, rate limiting.");
            }
            timeout = 1;
        } else if timeout != 0 && self.consecutive_polls != 0 {
            if verbose > 1 && self.consecutive_polls >= MAX_POLLS {
                errln!("select: got {} consecutive polls", self.consecutive_polls);
            }
            self.consecutive_polls = 0;
        }
        timeout
    }
}

#[cfg(unix)]
impl Select {
    /// No fds, no signals registered.
    pub fn new() -> Select {
        // SAFETY: fd_set and sigset_t are plain data; FD_ZERO and
        // sigemptyset initialize them.
        unsafe {
            let mut all_fds: libc::fd_set = std::mem::zeroed();
            let mut read_fds: libc::fd_set = std::mem::zeroed();
            libc::FD_ZERO(&mut all_fds);
            libc::FD_ZERO(&mut read_fds);
            let mut empty_sigset: libc::sigset_t = std::mem::zeroed();
            assert_eq!(0, libc::sigemptyset(&mut empty_sigset));
            Select {
                max_fd: -1,
                all_fds,
                read_fds,
                empty_sigset,
                consecutive_polls: 0,
            }
        }
    }

    /// Watch `fd` for reading in the next `select`.
    pub fn add_fd(&mut self, fd: Fd) {
        assert!((0..libc::FD_SETSIZE as Fd).contains(&fd));
        if fd > self.max_fd {
            self.max_fd = fd;
        }
        // SAFETY: fd is within FD_SETSIZE.
        unsafe { libc::FD_SET(fd, &mut self.all_fds) };
    }

    /// Stop watching every fd.
    pub fn clear_fds(&mut self) {
        // SAFETY: all_fds is a valid fd_set.
        unsafe { libc::FD_ZERO(&mut self.all_fds) };
    }

    /// Block the signal so we don't get it outside of pselect(), and register
    /// a handler, which will only be called when pselect() is interrupted by
    /// a (possibly queued) signal.
    pub fn add_signal(&mut self, signum: i32) {
        signal_index(signum);
        // SAFETY: standard sigprocmask/sigaction setup with initialized sets.
        unsafe {
            let mut to_block: libc::sigset_t = std::mem::zeroed();
            assert_eq!(0, libc::sigemptyset(&mut to_block));
            assert_eq!(0, libc::sigaddset(&mut to_block, signum));
            assert_eq!(
                0,
                libc::sigprocmask(libc::SIG_BLOCK, &to_block, std::ptr::null_mut())
            );

            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_flags = libc::SA_RESTART;
            sa.sa_sigaction = handle_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
            assert_eq!(0, libc::sigfillset(&mut sa.sa_mask));
            assert_eq!(0, libc::sigaction(signum, &sa, std::ptr::null_mut()));
        }
    }

    /// Timeout unit: milliseconds; negative timeout means wait forever.
    /// Returns the number of ready fds, 0 on timeout or signal.
    pub fn select(&mut self, timeout: i32) -> std::io::Result<i32> {
        self.read_fds = self.all_fds;
        clear_got_signal();

        let timeout = self.rate_limit(timeout);

        let ts;
        let tsp = if timeout >= 0 {
            ts = libc::timespec {
                tv_sec: (timeout / 1000) as libc::time_t,
                tv_nsec: 1_000_000 * (timeout as libc::c_long % 1000),
            };
            &ts as *const libc::timespec
        } else {
            std::ptr::null()
        };

        // SAFETY: read_fds and empty_sigset are initialized; tsp is null or valid.
        let mut ret = unsafe {
            libc::pselect(
                self.max_fd + 1,
                &mut self.read_fds,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                tsp,
                &self.empty_sigset,
            )
        };
        let err = std::io::Error::last_os_error();

        if ret == 0 || (ret == -1 && err.kind() == std::io::ErrorKind::Interrupted) {
            /* Look for and report Cygwin select() bug. */
            if ret == 0 {
                for fd in 0..=self.max_fd {
                    // SAFETY: fd is within the range add_fd checked.
                    if unsafe { libc::FD_ISSET(fd, &self.read_fds) } {
                        errln!("select(): nfds = 0 but read fd {fd} is set");
                    }
                }
            }
            /* The user should process events as usual. */
            // SAFETY: read_fds is a valid fd_set.
            unsafe { libc::FD_ZERO(&mut self.read_fds) };
            ret = 0;
        }
        if ret < 0 {
            return Err(err);
        }
        Ok(ret)
    }

    /// Whether `fd` was readable after the last `select`.
    pub fn read(&self, fd: Fd) -> bool {
        // SAFETY: fd sets are valid; fd was range-checked in add_fd.
        unsafe {
            debug_assert!(libc::FD_ISSET(fd, &self.all_fds));
            libc::FD_ISSET(fd, &self.read_fds)
        }
    }
}

#[cfg(windows)]
impl Select {
    /// No fds, no signals registered.
    pub fn new() -> Select {
        Select {
            all_fds: Vec::new(),
            read_fds: Vec::new(),
            watched_size: None,
            consecutive_polls: 0,
        }
    }

    /// Watch socket `fd` for reading in the next `select`.
    pub fn add_fd(&mut self, fd: Fd) {
        if !self.all_fds.contains(&fd) {
            self.all_fds.push(fd);
        }
    }

    /// Stop watching every socket.
    pub fn clear_fds(&mut self) {
        self.all_fds.clear();
    }

    /// Only SIGWINCH has an effect: from now on `select()` compares the
    /// console size on every call and raises SIGWINCH when it changed.
    pub fn add_signal(&mut self, signum: i32) {
        signal_index(signum);
        if signum == SIGWINCH && self.watched_size.is_none() {
            self.watched_size = Some(crate::tty::window_size());
        }
    }

    /// Timeout unit: milliseconds; negative timeout means wait forever.
    /// While SIGWINCH is watched the wait is capped at 50 ms, because the
    /// console size is only noticed when this function runs (PLAN 7).
    pub fn select(&mut self, timeout: i32) -> std::io::Result<i32> {
        use windows_sys::Win32::Networking::WinSock::{
            WSAGetLastError, WSAPoll, POLLRDNORM, WSAPOLLFD,
        };

        self.read_fds.clear();
        clear_got_signal();

        let mut timeout = self.rate_limit(timeout);
        if self.watched_size.is_some() && !(0..=50).contains(&timeout) {
            timeout = 50;
        }

        let mut ret = 0;
        if self.all_fds.is_empty() {
            if timeout > 0 {
                std::thread::sleep(std::time::Duration::from_millis(timeout as u64));
            }
        } else {
            let mut pollfds: Vec<WSAPOLLFD> = self
                .all_fds
                .iter()
                .map(|&fd| WSAPOLLFD {
                    fd: fd as usize,
                    events: POLLRDNORM,
                    revents: 0,
                })
                .collect();
            // SAFETY: pollfds is a valid array of the given length.
            ret = unsafe { WSAPoll(pollfds.as_mut_ptr(), pollfds.len() as u32, timeout) };
            if ret < 0 {
                // SAFETY: plain call.
                return Err(std::io::Error::from_raw_os_error(unsafe {
                    WSAGetLastError()
                }));
            }
            // Readable, hung up or errored all mean "a read will not block".
            self.read_fds = pollfds
                .iter()
                .filter(|p| p.revents != 0)
                .map(|p| p.fd as Fd)
                .collect();
        }

        if let Some(old) = &self.watched_size {
            let now = crate::tty::window_size();
            if now.as_ref().ok() != old.as_ref().ok() {
                GOT_SIGNAL[SIGWINCH as usize].store(true, Ordering::SeqCst);
                self.watched_size = Some(now);
            }
        }
        Ok(ret)
    }

    /// Whether `fd` was readable after the last `select`.
    pub fn read(&self, fd: Fd) -> bool {
        debug_assert!(self.all_fds.contains(&fd));
        self.read_fds.contains(&fd)
    }
}
