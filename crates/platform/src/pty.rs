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
// Ported from: upstream/src/frontend/mosh-server.cc (forkpty call, TIOCSWINSZ,
// child setenv + execvp) and upstream/src/util/pty_compat.cc (forkpty).
// Unix only. The utmp code in mosh-server.cc is not ported (PLAN 9.2).

//! Pseudoterminal for mosh-server, ported from upstream
//! `src/frontend/mosh-server.cc` and `src/util/pty_compat.cc`: forkpty,
//! window size, exec in the child, and waiting for the child. Unix only.

use std::ffi::{CString, OsStr, OsString};
use std::io;
use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

/// Result of `forkpty`, as in upstream: the parent gets the master side and
/// the child pid; the child gets the slave side as stdin, stdout and stderr,
/// is a session leader, and has the pty as its controlling terminal.
pub enum Fork {
    /// In the parent.
    Parent {
        /// Master side of the pty.
        master: OwnedFd,
        /// Process id of the child.
        child: libc::pid_t,
    },
    /// In the child.
    Child,
}

/// `forkpty( &master, NULL, NULL, &window_size )` from mosh-server.cc.
///
/// # Safety concerns
/// In the child only async-signal-safe work is guaranteed to be sound if the
/// parent had other threads. Call `exec` promptly in the child.
pub fn forkpty(cols: u16, rows: u16) -> io::Result<Fork> {
    forkpty_winsize(&libc::winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    })
}

/// `forkpty( &master, NULL, NULL, &window_size )` with the whole winsize
/// (pixel fields included), as mosh-server.cc passes it.
pub fn forkpty_winsize(window_size: &libc::winsize) -> io::Result<Fork> {
    // SAFETY: the child either execs or exits; see `forkpty`.
    match unsafe { nix::pty::forkpty(Some(window_size), None) } {
        Ok(nix::pty::ForkptyResult::Parent { child, master }) => Ok(Fork::Parent {
            master,
            child: child.as_raw(),
        }),
        Ok(nix::pty::ForkptyResult::Child) => Ok(Fork::Child),
        Err(e) => Err(io::Error::from(e)),
    }
}

/// `ioctl( fd, TIOCGWINSZ, &window_size )`, the whole struct.
pub fn get_winsize(fd: RawFd) -> io::Result<libc::winsize> {
    let mut window_size = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: TIOCGWINSZ writes one winsize struct through the pointer.
    if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut window_size) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(window_size)
}

/// `ioctl( fd, TIOCSWINSZ, &window_size )`, the whole struct.
pub fn set_winsize(fd: RawFd, window_size: &libc::winsize) -> io::Result<()> {
    // SAFETY: TIOCSWINSZ reads one winsize struct from the pointer.
    if unsafe { libc::ioctl(fd, libc::TIOCSWINSZ, window_size) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// `ioctl( host_fd, TIOCSWINSZ, &window_size )`. The kernel sends SIGWINCH to
/// the foreground process group of the pty.
pub fn set_window_size(fd: &impl AsRawFd, cols: u16, rows: u16) -> io::Result<()> {
    let window_size = libc::winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: TIOCSWINSZ reads one winsize struct from the pointer.
    if unsafe { libc::ioctl(fd.as_raw_fd(), libc::TIOCSWINSZ, &window_size) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Child side, after `forkpty`: set the given environment variables (as the
/// upstream child does with `setenv( "TERM", ... )` etc.) and `execvp` the
/// command. Only returns on failure.
pub fn exec(argv: &[String], env: &[(String, String)]) -> io::Error {
    for (name, value) in env {
        let (Ok(name), Ok(value)) = (CString::new(name.as_str()), CString::new(value.as_str()))
        else {
            return io::Error::from(io::ErrorKind::InvalidInput);
        };
        // SAFETY: both are valid C strings; overwrite = true as upstream.
        if unsafe { libc::setenv(name.as_ptr(), value.as_ptr(), 1) } < 0 {
            return io::Error::last_os_error();
        }
    }
    let argv: Vec<OsString> = argv.iter().map(OsString::from).collect();
    let Some(file) = argv.first() else {
        return io::Error::from(io::ErrorKind::InvalidInput);
    };
    execvp(file, &argv)
}

/// `execvp( file, argv )`: `file` is looked up in PATH, `argv[0]` may differ
/// from it (mosh-server runs `/bin/bash` with argv[0] `-bash`). Only returns
/// on failure.
pub fn execvp(file: &OsStr, argv: &[OsString]) -> io::Error {
    let (Ok(file), Ok(args)) = (
        CString::new(file.as_bytes()),
        argv.iter()
            .map(|a| CString::new(a.as_bytes()))
            .collect::<Result<Vec<_>, _>>(),
    ) else {
        return io::Error::from(io::ErrorKind::InvalidInput);
    };
    let mut ptrs: Vec<*const libc::c_char> = args.iter().map(|a| a.as_ptr()).collect();
    ptrs.push(std::ptr::null());
    // SAFETY: file is a valid C string; ptrs is a NULL-terminated array of
    // valid C strings that outlive the call.
    unsafe { libc::execvp(file.as_ptr(), ptrs.as_ptr()) };
    io::Error::last_os_error()
}

/// Blocking `waitpid` for the child, retried on EINTR.
pub fn wait_child(pid: libc::pid_t) -> io::Result<ExitStatus> {
    let mut status: libc::c_int = 0;
    loop {
        // SAFETY: plain waitpid on our own child.
        if unsafe { libc::waitpid(pid, &mut status, 0) } >= 0 {
            return Ok(ExitStatus::from_raw(status));
        }
        let err = io::Error::last_os_error();
        if err.kind() != io::ErrorKind::Interrupted {
            return Err(err);
        }
    }
}
