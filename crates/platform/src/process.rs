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
        match cmd.status() {
            Ok(status) => std::process::exit(status.code().unwrap_or(1)),
            Err(e) => e,
        }
    }
}
