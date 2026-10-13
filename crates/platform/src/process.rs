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
// Ported from: upstream/scripts/mosh.pl (the `open(my $pipe, "-|")` child
// whose stdout and stderr both go into the pipe, the `while ( <$pipe> )` line
// loop, `close $pipe; waitpid`, and `exec` of the client).

//! Child processes for the launcher, ported from upstream `scripts/mosh.pl`:
//! ssh with stdout and stderr merged into one pipe read by lines, exec, and
//! ignoring SIGHUP in the `--fake-proxy` relay, and quoting for the
//! ProxyCommand string ssh runs.

use std::io::{self, BufRead, BufReader, PipeReader};
use std::process::{Child, Command, ExitStatus, Stdio};

/// A child process whose stdout and stderr are merged into one pipe that we
/// read line by line, as mosh.pl does with `open(STDERR, ">&STDOUT")`.
/// stdin is inherited, so ssh can still ask for passwords and host key
/// confirmation on the terminal.
pub struct LineProcess {
    child: Child,
    reader: BufReader<PipeReader>,
}

/// Spawn `cmd` with stdout and stderr into one pipe and stdin inherited.
pub fn spawn_merged(cmd: &mut Command) -> io::Result<LineProcess> {
    let (reader, writer) = io::pipe()?;
    cmd.stdin(Stdio::inherit())
        .stdout(writer.try_clone()?)
        .stderr(writer);
    let child = cmd.spawn();
    // `cmd` still holds our copies of the write end; replace them so the
    // pipe reaches end of file when the child exits.
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    Ok(LineProcess {
        child: child?,
        reader: BufReader::new(reader),
    })
}

impl LineProcess {
    /// Next line without its trailing "\n" (a "\r" from ssh -tt stays), or
    /// None at end of file.
    pub fn read_line(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut line = Vec::new();
        if self.reader.read_until(b'\n', &mut line)? == 0 {
            return Ok(None);
        }
        if line.last() == Some(&b'\n') {
            line.pop();
        }
        Ok(Some(line))
    }

    /// `close $pipe; waitpid $pid, 0`: stop reading, then wait for the child.
    pub fn finish(self) -> io::Result<ExitStatus> {
        let LineProcess { mut child, reader } = self;
        drop(reader);
        child.wait()
    }
}

/// The client the launcher runs by default. On unix this is `mosh-client`
/// from PATH, as mosh.pl line 64. On Windows it is `mosh-client.exe` next to
/// the launcher's own executable when that file exists (the release zip
/// ships both side by side and is usually not on PATH), else `mosh-client`
/// from PATH.
pub fn default_client_program() -> String {
    #[cfg(windows)]
    if let Ok(exe) = std::env::current_exe() {
        let sibling = exe.with_file_name("mosh-client.exe");
        if sibling.is_file() {
            return sibling.to_string_lossy().into_owned();
        }
    }
    "mosh-client".to_string()
}

/// Quote one word of the ProxyCommand string for the way this system's ssh
/// runs it. OpenSSH on unix runs ProxyCommand with `$SHELL -c` (mosh.pl
/// sets SHELL=/bin/sh), so the word is single-quoted as mosh.pl's
/// `shell_quote` does. Win32-OpenSSH runs no shell: `ssh_proxy_connect`
/// (sshconnect.c, `FORK_NOT_SUPPORTED`) hands the whole string to its
/// `posix_spawnp`, whose `build_commandline_string`
/// (contrib/win32/win32compat/misc.c) passes it to `CreateProcessW`
/// unchanged when it starts with `"`, and otherwise only adds double quotes
/// around the part up to the first space after `.exe`. Single quotes there
/// become part of the file name (CreateProcessW error 2), so on Windows the
/// word is quoted with `windows_command_line_quote`.
pub fn proxy_command_quote(arg: &str) -> String {
    #[cfg(windows)]
    {
        windows_command_line_quote(arg)
    }
    #[cfg(not(windows))]
    {
        format!("'{}'", arg.replace('\'', "'\\''"))
    }
}

/// Quote one argument by the Windows command-line rules that
/// `CommandLineToArgvW`, the MSVC runtime and Rust's `std::env::args` use to
/// split a command line: unchanged unless it is empty or has a space, tab or
/// double quote; otherwise wrapped in double quotes, with each `"` written
/// as `\"` and backslashes doubled only where they come right before a `"`
/// or the closing quote. Pure string code, compiled everywhere so it can be
/// checked on any system; only Windows uses it.
pub fn windows_command_line_quote(arg: &str) -> String {
    if !arg.is_empty() && !arg.contains([' ', '\t', '"']) {
        return arg.to_string();
    }
    let mut quoted = String::from('"');
    let mut backslashes = 0;
    for c in arg.chars() {
        if c == '\\' {
            backslashes += 1;
        } else {
            if c == '"' {
                /* Double the run of backslashes before a quote, then escape the quote. */
                quoted.extend(std::iter::repeat_n('\\', backslashes + 1));
            }
            backslashes = 0;
        }
        quoted.push(c);
    }
    /* Double the trailing backslashes so they do not escape the closing quote. */
    quoted.extend(std::iter::repeat_n('\\', backslashes));
    quoted.push('"');
    quoted
}

/// `$SIG{ 'HUP' } = 'IGNORE'` (mosh.pl line 305): ssh sends SIGHUP to its
/// ProxyCommand when it exits, and the `--fake-proxy` relay ignores it so
/// the copy from stdin to the socket can finish.
/// Windows has no SIGHUP (ssh.exe ends the proxy differently), so this does
/// nothing there.
pub fn ignore_sighup() {
    #[cfg(unix)]
    // SAFETY: SIG_IGN is a valid disposition for SIGHUP; no handler code runs.
    unsafe {
        libc::signal(libc::SIGHUP, libc::SIG_IGN);
    }
}

/// Replace this process with `cmd` (Perl `exec`). Only returns on failure.
/// Windows has no exec: run the command, wait, and exit with its code.
pub fn exec(cmd: &mut Command) -> io::Error {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.exec()
    }
    #[cfg(windows)]
    {
        /* On unix the launcher becomes the client. Here it waits for it, and
        Ctrl+Break reaches both processes: the client turns it into its clean
        shutdown (select.rs), so the waiting launcher ignores it rather than
        exiting first and giving the console back while the client still
        runs. */
        unsafe extern "system" fn ignore_interrupt(ctrl_type: u32) -> windows_sys::core::BOOL {
            use windows_sys::Win32::System::Console::{CTRL_BREAK_EVENT, CTRL_C_EVENT};
            i32::from(ctrl_type == CTRL_BREAK_EVENT || ctrl_type == CTRL_C_EVENT)
        }
        // SAFETY: registers a handler with the documented signature; it only
        // compares its argument.
        unsafe {
            windows_sys::Win32::System::Console::SetConsoleCtrlHandler(Some(ignore_interrupt), 1);
        }
        match cmd.status() {
            Ok(status) => std::process::exit(status.code().unwrap_or(1)),
            Err(e) => e,
        }
    }
}
