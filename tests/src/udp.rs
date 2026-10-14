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
// Ported from: (none; test harness)

//! Which process has a UDP port bound, for the tests that start a
//! `mosh-server` on a fixed port and must kill a leftover from a failed run.
//! `ss` on Linux; macOS has no `ss`, so `lsof` there.

use std::process::Command;

/// The pid of the process with UDP `port` bound, if any. Runs `ss -Hulnp
/// "sport = :PORT"` and reads its `pid=N`; where `ss` is missing (macOS)
/// runs `lsof -t -nP -iUDP:PORT`, whose `-t` output is pids only.
pub fn listener_pid(port: u16) -> Option<u32> {
    let Ok(out) = Command::new("ss")
        .args(["-H", "-u", "-l", "-n", "-p"])
        .arg(format!("sport = :{port}"))
        .output()
    else {
        let out = Command::new("lsof")
            .args(["-t", "-nP", &format!("-iUDP:{port}")])
            .output()
            .ok()?;
        return String::from_utf8_lossy(&out.stdout)
            .lines()
            .next()?
            .trim()
            .parse()
            .ok();
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let at = text.find("pid=")? + 4;
    text[at..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

/// `kill -9` whatever has UDP `port` bound.
pub fn kill_listener(port: u16) {
    if let Some(pid) = listener_pid(port) {
        let _ = Command::new("kill").arg("-9").arg(pid.to_string()).status();
    }
}
