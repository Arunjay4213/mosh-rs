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
// Ported from: (new; no upstream equivalent)

//! The real mosh 1.4.0 programs (`mosh`, `mosh-client`, `mosh-server`)
//! that tests run as oracles next to ours. They are found on PATH: apt
//! installs them in /usr/bin on Linux, Homebrew in /opt/homebrew/bin on
//! macOS. A program whose `--version` names mosh-rs is never taken, so a
//! PATH that happens to contain our own build cannot turn a comparison into
//! ours against ours.

use std::process::Command;
use std::sync::OnceLock;

/// The real mosh programs found on PATH.
pub struct Upstream {
    /// The Perl launcher `mosh`.
    pub mosh: String,
    /// The C++ `mosh-client`.
    pub client: String,
    /// The C++ `mosh-server`.
    pub server: String,
}

/// The first `name` on PATH that is not a mosh-rs build.
pub fn find(name: &str) -> Option<String> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .filter(|p| p.is_file())
        .find(|p| {
            Command::new(p)
                .arg("--version")
                .output()
                .is_ok_and(|out| !String::from_utf8_lossy(&out.stdout).contains("mosh-rs"))
        })
        .map(|p| p.display().to_string())
}

/// All three real mosh programs, found once, or None after printing why
/// the calling test is skipped. Every test that runs them starts with
/// `let Some(_) = upstream::require() else { return };`.
pub fn require() -> Option<&'static Upstream> {
    static FOUND: OnceLock<Option<Upstream>> = OnceLock::new();
    let found = FOUND.get_or_init(|| {
        Some(Upstream {
            mosh: find("mosh")?,
            client: find("mosh-client")?,
            server: find("mosh-server")?,
        })
    });
    if found.is_none() {
        println!(
            "SKIPPED: the real mosh 1.4.0 (mosh, mosh-client, mosh-server) is not on PATH \
             (apt install mosh, or brew install mosh)"
        );
    }
    found.as_ref()
}

/// The programs `require` found, for helpers that run after the test's
/// `require` guard.
pub fn get() -> &'static Upstream {
    require().expect("the test checks upstream::require() before running the real mosh")
}
