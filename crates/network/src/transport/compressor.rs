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
// Ported from: upstream/src/network/compressor.h, upstream/src/network/compressor.cc

//! zlib `compress()` / `uncompress()` with upstream's fixed 4 MiB output
//! buffer as the size limit. Upstream keeps one static `Compressor` to
//! reuse that buffer; here the functions allocate per call instead.

use flate2::write::ZlibEncoder;
use flate2::{Compression, Decompress, FlushDecompress, Status};
use std::io::Write;

use super::dos_detected;
use crate::crypto::CryptoError;

/// effective limit on terminal size
pub const BUFFER_SIZE: usize = 2048 * 2048;

/// `compress()` uses `Z_DEFAULT_COMPRESSION`.
pub fn compress_str(input: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let ok = e.write_all(input).is_ok();
    let out = e.finish();
    match out {
        Ok(out) if ok && out.len() <= BUFFER_SIZE => Ok(out),
        _ => Err(dos_detected("Z_OK == compress( buffer, &len, input.data(), input.size() )", "compress_str")),
    }
}

/// `uncompress()` into a buffer of `BUFFER_SIZE` bytes: fails on corrupt
/// or truncated input, and when the output would exceed the buffer.
/// Bytes after the end of the zlib stream are ignored, as `uncompress()` does.
pub fn uncompress_str(input: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let err = || dos_detected("Z_OK == uncompress( buffer, &len, input.data(), input.size() )", "uncompress_str");
    let mut d = Decompress::new(true);
    let mut out: Vec<u8> = Vec::with_capacity((input.len() * 4 + 64).min(BUFFER_SIZE));
    loop {
        let consumed = d.total_in() as usize;
        let before_out = out.len();
        let status = d
            .decompress_vec(&input[consumed..], &mut out, FlushDecompress::Finish)
            .map_err(|_| err())?;
        if out.len() > BUFFER_SIZE {
            return Err(err());
        }
        if status == Status::StreamEnd {
            return Ok(out);
        }
        if out.len() == out.capacity() {
            if out.len() >= BUFFER_SIZE {
                return Err(err());
            }
            let grow = out.capacity().min(BUFFER_SIZE - out.len());
            out.reserve_exact(grow);
            continue;
        }
        if d.total_in() as usize == consumed && out.len() == before_out {
            /* no progress with output space available: input is truncated */
            return Err(err());
        }
    }
}
