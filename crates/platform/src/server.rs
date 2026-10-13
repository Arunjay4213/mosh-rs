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
// Ported from: upstream/src/frontend/mosh-server.cc (the system calls it
// makes directly: sigaction, fork, isatty, tcdrain, the /dev/null redirect,
// pipe, read, write, close, IUTF8, getpwuid, getutxent). Unix only, as
// mosh-server is.

//! System calls that only mosh-server makes, ported from upstream
//! `src/frontend/mosh-server.cc`. Each is a plain wrapper; the order of the
//! calls and the error handling stay in the caller, as upstream.

use std::ffi::CStr;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};

/// `sigaction( signum, &sa, NULL )` with `sa_handler` SIG_IGN (`ignore`) or
/// SIG_DFL, `sa_flags = 0` and a full `sa_mask`, as mosh-server.cc lines
/// 453-458 and 541-546.
pub fn set_signal_disposition(signum: i32, ignore: bool) -> io::Result<()> {
    // SAFETY: sa is zero-initialized then filled; SIG_IGN and SIG_DFL run no
    // handler code.
    unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = if ignore { libc::SIG_IGN } else { libc::SIG_DFL };
        sa.sa_flags = 0;
        if libc::sigfillset(&mut sa.sa_mask) != 0
            || libc::sigaction(signum, &sa, std::ptr::null_mut()) != 0
        {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

/// `fork()`: Ok(0) in the child, Ok(pid) in the parent.
pub fn fork() -> io::Result<libc::pid_t> {
    // SAFETY: mosh-server has a single thread when it forks, so the child
    // can go on running ordinary code, as upstream's does.
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(pid)
}

/// `isatty( fd )`.
pub fn isatty(fd: RawFd) -> bool {
    // SAFETY: isatty only inspects the descriptor.
    unsafe { libc::isatty(fd) == 1 }
}

/// `tcdrain( fd )`; the result is ignored, as upstream.
pub fn tcdrain(fd: RawFd) {
    // SAFETY: tcdrain only waits on the descriptor.
    unsafe { libc::tcdrain(fd) };
}

/// mosh-server.cc lines 496-513: open /dev/null and dup2 it over stdin,
/// stdout and stderr. The error names the failing call ("open", "dup2" or
/// "close") for upstream's `perror`.
pub fn redirect_stdio_to_dev_null() -> Result<(), (&'static str, io::Error)> {
    // SAFETY: plain open/dup2/close on a descriptor this function owns and
    // on the three standard descriptors.
    unsafe {
        let nullfd = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR);
        if nullfd == -1 {
            return Err(("open", io::Error::last_os_error()));
        }
        if libc::dup2(nullfd, libc::STDIN_FILENO) < 0
            || libc::dup2(nullfd, libc::STDOUT_FILENO) < 0
            || libc::dup2(nullfd, libc::STDERR_FILENO) < 0
        {
            return Err(("dup2", io::Error::last_os_error()));
        }
        if libc::close(nullfd) < 0 {
            return Err(("close", io::Error::last_os_error()));
        }
    }
    Ok(())
}

/// `pipe( pipes )`: (read end, write end). Not close-on-exec, as upstream.
pub fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    let mut pipes = [0 as libc::c_int; 2];
    // SAFETY: pipes has room for the two descriptors.
    if unsafe { libc::pipe(pipes.as_mut_ptr()) } == -1 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: both descriptors were just created and nothing else owns them.
    Ok(unsafe {
        (
            OwnedFd::from_raw_fd(pipes[0]),
            OwnedFd::from_raw_fd(pipes[1]),
        )
    })
}

/// `close( fd )`, reporting the error that dropping an `OwnedFd` hides.
pub fn close(fd: impl IntoRawFd) -> io::Result<()> {
    // SAFETY: into_raw_fd gave up ownership, so the descriptor is closed once.
    if unsafe { libc::close(fd.into_raw_fd()) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// One `read( fd, buf, len )`: Ok(0) is end of file; EINTR is an error.
pub fn read(fd: &impl AsRawFd, buf: &mut [u8]) -> io::Result<usize> {
    // SAFETY: buf is valid for buf.len() bytes.
    let n = unsafe { libc::read(fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
    if n < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(n as usize)
}

/// One `write( fd, buf, len )`: may write less than `buf`; EINTR is an error.
pub fn write(fd: &impl AsRawFd, buf: &[u8]) -> io::Result<usize> {
    // SAFETY: buf is valid for buf.len() bytes.
    let n = unsafe { libc::write(fd.as_raw_fd(), buf.as_ptr().cast(), buf.len()) };
    if n < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(n as usize)
}

/// mosh-server.cc lines 557-568: set IUTF8 in stdin's termios. The error
/// names the failing call ("tcgetattr" or "tcsetattr"). Returns Ok without
/// doing anything where IUTF8 does not exist (upstream's `HAVE_IUTF8`).
pub fn set_iutf8_stdin() -> Result<(), (&'static str, io::Error)> {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    // SAFETY: tcgetattr fills the zeroed struct; tcsetattr reads it.
    unsafe {
        let mut child_termios: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(libc::STDIN_FILENO, &mut child_termios) < 0 {
            return Err(("tcgetattr", io::Error::last_os_error()));
        }
        child_termios.c_iflag |= libc::IUTF8;
        if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &child_termios) < 0 {
            return Err(("tcsetattr", io::Error::last_os_error()));
        }
    }
    Ok(())
}

/// The fields of `getpwuid( getuid() )` that mosh-server reads.
pub struct Passwd {
    /// `pw_name`.
    pub name: String,
    /// `pw_dir`.
    pub dir: String,
    /// `pw_shell`.
    pub shell: String,
}

/// `getpwuid( getuid() )`; the error carries errno for `perror( "getpwuid" )`.
pub fn getpwuid() -> io::Result<Passwd> {
    nix::errno::Errno::clear();
    // SAFETY: getpwuid returns NULL or a pointer to a static struct whose
    // strings are copied out before any other passwd call.
    unsafe {
        let pw = libc::getpwuid(libc::getuid());
        if pw.is_null() {
            return Err(io::Error::last_os_error());
        }
        let s = |p: *const libc::c_char| CStr::from_ptr(p).to_string_lossy().into_owned();
        Ok(Passwd {
            name: s((*pw).pw_name),
            dir: s((*pw).pw_dir),
            shell: s((*pw).pw_shell),
        })
    }
}

/// One utmpx record: the fields `warn_unattached` reads.
pub struct UtmpxEntry {
    /// `ut_type == USER_PROCESS`.
    pub user_process: bool,
    /// `ut_user`.
    pub user: String,
    /// `ut_host`.
    pub host: String,
    /// `ut_line`.
    pub line: String,
}

/// `while ( struct utmpx* entry = getutxent() )`, collected. As upstream,
/// neither `setutxent` nor `endutxent` is called.
///
/// On Linux the utmp file is read directly instead of through `getutxent`:
/// musl's `getutxent` is a stub that always returns NULL, so a static musl
/// `mosh-server` would never see a detached session. Reading the file the
/// same way in both builds keeps glibc and musl identical. Upstream's
/// glibc reads `_PATH_UTMP`, `/var/run/utmp`, which is a link to
/// `/run/utmp` on current systems.
///
/// Record layout, glibc `struct utmpx` from `<utmpx.h>`, measured on devbox
/// with `offsetof`/`sizeof` (gcc for aarch64, x86_64-linux-gnu-gcc run under
/// qemu-x86_64): `ut_type` short at 0, `ut_line` char[32] at 8, `ut_user`
/// char[32] at 44, `ut_host` char[256] at 76, identical on both. The record
/// size differs: x86_64 keeps 32-bit `ut_session` and `ut_tv` for
/// compatibility with 32-bit programs (`__WORDSIZE_TIME64_COMPAT32`), so its
/// record is 384 bytes; aarch64 has a 64-bit `long` session and `timeval`,
/// so its record is 400 bytes (devbox's `/run/utmp` parses as 48 records of
/// 400 bytes, all with a valid `ut_type`). The file is written in the
/// machine's byte order. A file whose size is not a whole number of records
/// was not written with this layout, so it yields no records, as
/// `getutxent` yields none for a file it cannot use (PLAN 9 item 15).
#[cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
pub fn getutxent_all() -> Vec<UtmpxEntry> {
    #[cfg(target_arch = "x86_64")]
    const RECORD: usize = 384;
    #[cfg(target_arch = "aarch64")]
    const RECORD: usize = 400;
    const UT_TYPE: usize = 0;
    const UT_LINE: (usize, usize) = (8, 32);
    const UT_USER: (usize, usize) = (44, 32);
    const UT_HOST: (usize, usize) = (76, 256);

    fn field(record: &[u8], (at, len): (usize, usize)) -> String {
        let bytes = &record[at..at + len];
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(len);
        String::from_utf8_lossy(&bytes[..end]).into_owned()
    }
    // As getutxent, an unreadable file means no records.
    let data = std::fs::read("/run/utmp")
        .or_else(|_| std::fs::read("/var/run/utmp"))
        .unwrap_or_default();
    if !data.len().is_multiple_of(RECORD) {
        return Vec::new();
    }
    data.as_chunks::<RECORD>()
        .0
        .iter()
        .map(|r| {
            let ut_type = i16::from_ne_bytes([r[UT_TYPE], r[UT_TYPE + 1]]);
            UtmpxEntry {
                user_process: ut_type == libc::USER_PROCESS,
                user: field(r, UT_USER),
                host: field(r, UT_HOST),
                line: field(r, UT_LINE),
            }
        })
        .collect()
}

/// `while ( struct utmpx* entry = getutxent() )`, collected, on systems
/// whose utmp file layout is not known here (see the Linux version above).
#[cfg(not(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
pub fn getutxent_all() -> Vec<UtmpxEntry> {
    // c_char is i8 on x86_64 and u8 on aarch64, so the cast is needed on one.
    #[allow(clippy::unnecessary_cast)]
    fn field(chars: &[libc::c_char]) -> String {
        let bytes: Vec<u8> = chars
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as u8)
            .collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
    let mut entries = Vec::new();
    loop {
        // SAFETY: getutxent returns NULL or a pointer to a static record
        // that is copied out before the next call.
        let entry = unsafe { libc::getutxent() };
        if entry.is_null() {
            break;
        }
        // SAFETY: non-null and valid until the next getutxent call.
        let e = unsafe { &*entry };
        entries.push(UtmpxEntry {
            user_process: e.ut_type == libc::USER_PROCESS,
            user: field(&e.ut_user),
            host: field(&e.ut_host),
            line: field(&e.ut_line),
        });
    }
    entries
}
