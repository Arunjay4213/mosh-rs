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
// Ported from: upstream/src/crypto/prng.h

//! OS randomness (upstream uses getrandom(2) where available).

use super::CryptoError;
use rand::rngs::SysRng;
use rand::TryRng;

pub struct Prng;

impl Prng {
    pub fn new() -> Prng {
        Prng
    }

    pub fn fill(&mut self, dest: &mut [u8]) -> Result<(), CryptoError> {
        if dest.is_empty() {
            return Ok(());
        }
        SysRng
            .try_fill_bytes(dest)
            .map_err(|_| CryptoError::new("getrandom fell short"))
    }

    pub fn uint8(&mut self) -> Result<u8, CryptoError> {
        let mut x = [0u8; 1];
        self.fill(&mut x)?;
        Ok(x[0])
    }

    pub fn uint32(&mut self) -> Result<u32, CryptoError> {
        let mut x = [0u8; 4];
        self.fill(&mut x)?;
        Ok(u32::from_ne_bytes(x))
    }

    pub fn uint64(&mut self) -> Result<u64, CryptoError> {
        let mut x = [0u8; 8];
        self.fill(&mut x)?;
        Ok(u64::from_ne_bytes(x))
    }
}

impl Default for Prng {
    fn default() -> Self {
        Prng::new()
    }
}
