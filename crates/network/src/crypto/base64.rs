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
// Ported from: upstream/src/crypto/base64.cc

//! Port of upstream `src/crypto/base64.cc`.
//!
//! Base64 for 128-bit Mosh keys only: 16 raw bytes <-> 24 characters
//! ending in "==".

const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Reverse of `TABLE`; 0xff for characters outside the alphabet
/// (identical to upstream's literal `reverse[]` table).
const REVERSE: [u8; 256] = {
    let mut r = [0xffu8; 256];
    let mut i = 0;
    while i < 64 {
        r[TABLE[i] as usize] = i as u8;
        i += 1;
    }
    r
};

/// Reverse maps from an ASCII char to a base64 sixbit value. Returns > 0x3f on failure.
fn base64_char_to_sixbit(c: u8) -> u8 {
    REVERSE[c as usize]
}

/// Decodes exactly 24 characters (22 + "==") into 16 bytes.
pub fn base64_decode(b64: &[u8], raw: &mut [u8; 16]) -> bool {
    assert!(b64.len() == 24, "b64_len == 24"); /* only useful for Mosh keys */

    let mut bytes: u32 = 0;
    let mut out = 0;
    for (i, &c) in b64[..22].iter().enumerate() {
        let sixbit = base64_char_to_sixbit(c);
        if sixbit > 0x3f {
            return false;
        }
        bytes <<= 6;
        bytes |= sixbit as u32;
        /* write groups of 3 */
        if i % 4 == 3 {
            /* as u8 keeps the low byte, as the C uint8_t assignments do */
            raw[out] = (bytes >> 16) as u8;
            raw[out + 1] = (bytes >> 8) as u8;
            raw[out + 2] = bytes as u8;
            out += 3;
            bytes = 0;
        }
    }
    /* last byte of output */
    /* as u8 keeps the low byte, as the C uint8_t assignment does */
    raw[out] = (bytes >> 4) as u8;
    if b64[22] != b'=' || b64[23] != b'=' {
        return false;
    }
    true
}

/// Encodes 16 bytes into 24 characters (the last two are "==").
pub fn base64_encode(raw: &[u8; 16]) -> [u8; 24] {
    let mut b64 = [0u8; 24];
    /* first 15 bytes of input */
    for i in 0..5 {
        let r = &raw[3 * i..3 * i + 3];
        let bytes: u32 = ((r[0] as u32) << 16) | ((r[1] as u32) << 8) | r[2] as u32;
        let o = &mut b64[4 * i..4 * i + 4];
        o[0] = TABLE[((bytes >> 18) & 0x3f) as usize];
        o[1] = TABLE[((bytes >> 12) & 0x3f) as usize];
        o[2] = TABLE[((bytes >> 6) & 0x3f) as usize];
        o[3] = TABLE[(bytes & 0x3f) as usize];
    }
    /* last byte of input, last 4 of output */
    let lastchar = raw[15];
    b64[20] = TABLE[((lastchar >> 2) & 0x3f) as usize];
    b64[21] = TABLE[((lastchar << 4) & 0x3f) as usize];
    b64[22] = b'=';
    b64[23] = b'=';
    b64
}
