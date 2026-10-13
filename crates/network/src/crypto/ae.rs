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
// Ported from: upstream/src/crypto/ae.h (interface only; OCB comes from the ocb3 + aes crates)

//! The `ae_*` authenticated-encryption interface that `crypto.cc` and
//! `src/tests/ocb-aes.cc` use: AES-128 in OCB3 mode (RFC 7253), 12-byte
//! nonce, 16-byte tag. As in upstream, the tag is appended to the
//! ciphertext (`ae_encrypt` with `tag == NULL`).

use aes::Aes128;
use ocb3::aead::{AeadInOut, KeyInit};
use ocb3::consts::{U12, U16};
use ocb3::Ocb3;

pub const KEY_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;
pub const TAG_LEN: usize = 16;

/// Initialized `ae_ctx` (`ae_init( ctx, key, 16, 12, 16 )`).
pub struct AeCtx {
    cipher: Ocb3<Aes128, U12, U16>,
}

pub fn ae_init(key: &[u8; KEY_LEN]) -> AeCtx {
    AeCtx {
        cipher: Ocb3::new(&(*key).into()),
    }
}

/// `ae_encrypt( ctx, nonce, pt, pt_len, ad, ad_len, ct, NULL, AE_FINALIZE )`.
/// Returns ciphertext followed by the tag (`pt.len() + TAG_LEN` bytes).
pub fn ae_encrypt(ctx: &AeCtx, nonce: &[u8; NONCE_LEN], pt: &[u8], ad: &[u8]) -> Vec<u8> {
    let mut ct = Vec::with_capacity(pt.len() + TAG_LEN);
    ct.extend_from_slice(pt);
    let tag = ctx
        .cipher
        .encrypt_inout_detached(&(*nonce).into(), ad, ct.as_mut_slice().into())
        .expect("OCB input length within limits");
    ct.extend_from_slice(&tag);
    ct
}

/// `ae_decrypt( ctx, nonce, ct, ct_len, ad, ad_len, pt, NULL, AE_FINALIZE )`.
/// `ct` includes the trailing tag. Returns `None` where upstream returns
/// `AE_INVALID`.
pub fn ae_decrypt(ctx: &AeCtx, nonce: &[u8; NONCE_LEN], ct: &[u8], ad: &[u8]) -> Option<Vec<u8>> {
    if ct.len() < TAG_LEN {
        return None;
    }
    let (body, tag) = ct.split_at(ct.len() - TAG_LEN);
    let mut pt = body.to_vec();
    let tag: [u8; TAG_LEN] = tag.try_into().expect("tag length");
    match ctx
        .cipher
        .decrypt_inout_detached(&(*nonce).into(), ad, pt.as_mut_slice().into(), &tag.into())
    {
        Ok(()) => Some(pt),
        Err(_) => None,
    }
}
