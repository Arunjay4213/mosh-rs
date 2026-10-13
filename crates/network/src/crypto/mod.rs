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
// Ported from: upstream/src/crypto/crypto.h, upstream/src/crypto/crypto.cc

pub mod ae;
pub mod base64;
pub mod prng;

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use ae::{ae_decrypt, ae_encrypt, ae_init, AeCtx};
use base64::{base64_decode, base64_encode};
use prng::Prng;

/// `Crypto::CryptoException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoError {
    pub text: String,
    pub fatal: bool,
}

impl CryptoError {
    pub fn new(text: impl Into<String>) -> CryptoError {
        CryptoError {
            text: text.into(),
            fatal: false,
        }
    }

    pub fn fatal(text: impl Into<String>) -> CryptoError {
        CryptoError {
            text: text.into(),
            fatal: true,
        }
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl std::error::Error for CryptoError {}

/// C `strtol( str, &end, 10 )` on an LP64 system: returns
/// (value, end offset, overflowed). `end` is 0 when no digits were found.
pub(crate) fn strtol(s: &[u8]) -> (i64, usize, bool) {
    let mut i = 0;
    while i < s.len() && matches!(s[i], b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
        i += 1;
    }
    let mut negative = false;
    if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
        negative = s[i] == b'-';
        i += 1;
    }
    let digits_start = i;
    let mut value: i64 = 0;
    let mut overflow = false;
    while i < s.len() && s[i].is_ascii_digit() {
        let d = (s[i] - b'0') as i64;
        if !overflow {
            match value.checked_mul(10).and_then(|v| {
                if negative {
                    v.checked_sub(d)
                } else {
                    v.checked_add(d)
                }
            }) {
                Some(v) => value = v,
                None => overflow = true,
            }
        }
        i += 1;
    }
    if i == digits_start {
        return (0, 0, false);
    }
    if overflow {
        value = if negative { i64::MIN } else { i64::MAX };
    }
    (value, i, overflow)
}

/// `myatoi`: strict decimal integer parse.
pub fn myatoi(s: &str) -> Result<i64, CryptoError> {
    let (ret, end, overflow) = strtol(s.as_bytes());
    if overflow || end != s.len() {
        return Err(CryptoError::new("Bad integer."));
    }
    Ok(ret)
}

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

/// OCB (and other algorithms) require a source of nonce/sequence
/// numbers that never repeats its output. Enforce that with this
/// function.
pub fn unique() -> Result<u64, CryptoError> {
    let rv = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    if rv.wrapping_add(1) == 0 {
        return Err(CryptoError::fatal("Counter wrapped"));
    }
    Ok(rv)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Base64Key {
    key: [u8; 16],
}

impl Base64Key {
    /// Random key.
    pub fn new() -> Result<Base64Key, CryptoError> {
        Base64Key::from_prng(&mut Prng::new())
    }

    pub fn from_prng(prng: &mut Prng) -> Result<Base64Key, CryptoError> {
        let mut key = [0u8; 16];
        prng.fill(&mut key)?;
        Ok(Base64Key { key })
    }

    pub fn from_printable(printable_key: &str) -> Result<Base64Key, CryptoError> {
        if printable_key.len() != 22 {
            return Err(CryptoError::new("Key must be 22 letters long."));
        }
        let mut base64 = printable_key.as_bytes().to_vec();
        base64.extend_from_slice(b"==");
        let mut key = [0u8; 16];
        if !base64_decode(&base64, &mut key) {
            return Err(CryptoError::new("Key must be well-formed base64."));
        }
        let ret = Base64Key { key };
        /* to catch changes after the first 128 bits */
        if printable_key != ret.printable_key() {
            return Err(CryptoError::new("Base64 key was not encoded 128-bit key."));
        }
        Ok(ret)
    }

    pub fn printable_key(&self) -> String {
        let base64 = base64_encode(&self.key);
        assert!(base64[23] == b'=' && base64[22] == b'=', "Unexpected output from base64_encode");
        String::from_utf8(base64[..22].to_vec()).expect("base64 is ASCII")
    }

    pub fn data(&self) -> &[u8; 16] {
        &self.key
    }
}

impl fmt::Debug for Base64Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Base64Key(..)")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Nonce {
    bytes: [u8; Nonce::NONCE_LEN],
}

impl Nonce {
    pub const NONCE_LEN: usize = 12;

    pub fn new(val: u64) -> Nonce {
        let mut bytes = [0u8; Nonce::NONCE_LEN];
        bytes[4..].copy_from_slice(&val.to_be_bytes());
        Nonce { bytes }
    }

    pub fn from_bytes(s_bytes: &[u8]) -> Result<Nonce, CryptoError> {
        if s_bytes.len() != 8 {
            return Err(CryptoError::new("Nonce representation must be 8 octets long."));
        }
        let mut bytes = [0u8; Nonce::NONCE_LEN];
        bytes[4..].copy_from_slice(s_bytes);
        Ok(Nonce { bytes })
    }

    /// The 8 bytes that go on the wire.
    pub fn cc_str(&self) -> &[u8] {
        &self.bytes[4..]
    }

    pub fn data(&self) -> &[u8; Nonce::NONCE_LEN] {
        &self.bytes
    }

    pub fn val(&self) -> u64 {
        u64::from_be_bytes(self.bytes[4..].try_into().expect("8 bytes"))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Message {
    pub nonce: Nonce,
    pub text: Vec<u8>,
}

impl Message {
    pub fn new(nonce: Nonce, text: Vec<u8>) -> Message {
        Message { nonce, text }
    }
}

pub struct Session {
    key: Base64Key,
    ctx: AeCtx,
    blocks_encrypted: u64,
}

impl Session {
    pub const RECEIVE_MTU: usize = 2048;
    /// Overhead (not counting the nonce, which is handled by network transport)
    pub const ADDED_BYTES: usize = 16 /* final OCB block */;

    pub fn new(s_key: Base64Key) -> Session {
        Session {
            key: s_key,
            ctx: ae_init(s_key.data()),
            blocks_encrypted: 0,
        }
    }

    pub fn key(&self) -> &Base64Key {
        &self.key
    }

    pub fn encrypt(&mut self, plaintext: &Message) -> Result<Vec<u8>, CryptoError> {
        let pt_len = plaintext.text.len();
        let ciphertext_len = pt_len + 16;

        assert!(ciphertext_len <= Session::RECEIVE_MTU);
        assert!(pt_len <= Session::RECEIVE_MTU);

        let ciphertext = ae_encrypt(&self.ctx, plaintext.nonce.data(), &plaintext.text, &[]);
        assert_eq!(ciphertext.len(), ciphertext_len, "ae_encrypt() returned error.");

        self.blocks_encrypted += (pt_len >> 4) as u64;
        if pt_len & 0xF != 0 {
            /* partial block */
            self.blocks_encrypted += 1;
        }

        /* "Both the privacy and the authenticity properties of OCB degrade as
            per s^2 / 2^128, where s is the total number of blocks that the
            adversary acquires.... In order to ensure that s^2 / 2^128 remains
            small, a given key should be used to encrypt at most 2^48 blocks (2^55
            bits or 4 petabytes)"

           -- http://tools.ietf.org/html/draft-krovetz-ocb-03

           We deem it unlikely that a legitimate user will send 4 PB through a Mosh
           session.  If it happens, we simply kill the session.  The server and
           client use the same key, so we actually need to die after 2^47 blocks.
        */
        if self.blocks_encrypted >> 47 != 0 {
            return Err(CryptoError::fatal("Encrypted 2^47 blocks."));
        }

        let mut ret = Vec::with_capacity(8 + ciphertext_len);
        ret.extend_from_slice(plaintext.nonce.cc_str());
        ret.extend_from_slice(&ciphertext);
        Ok(ret)
    }

    pub fn decrypt(&self, s: &[u8]) -> Result<Message, CryptoError> {
        if s.len() < 24 {
            return Err(CryptoError::new("Ciphertext must contain nonce and tag."));
        }
        let body_len = s.len() - 8;
        let pt_len = body_len - 16;

        assert!(body_len <= Session::RECEIVE_MTU);
        assert!(pt_len <= Session::RECEIVE_MTU);

        let nonce = Nonce::from_bytes(&s[..8])?;
        match ae_decrypt(&self.ctx, nonce.data(), &s[8..], &[]) {
            Some(pt) if pt.len() == pt_len => Ok(Message::new(nonce, pt)),
            _ => Err(CryptoError::new("Packet failed integrity check.")),
        }
    }
}

#[cfg(unix)]
static SAVED_CORE_RLIMIT: AtomicU64 = AtomicU64::new(0);

/// Disable dumping core, as a precaution to avoid saving sensitive data
/// to disk.
#[cfg(unix)]
pub fn disable_dumping_core() {
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: getrlimit/setrlimit with a valid pointer to a local rlimit.
    unsafe {
        if libc::getrlimit(libc::RLIMIT_CORE, &mut limit) != 0 {
            /* We don't throw CryptoException because this is called very early
            in main(), outside of 'try'. */
            eprintln!("getrlimit(RLIMIT_CORE): {}", std::io::Error::last_os_error());
            std::process::exit(1);
        }
        #[allow(clippy::unnecessary_cast)] /* rlim_t is not u64 everywhere */
        SAVED_CORE_RLIMIT.store(limit.rlim_cur as u64, Ordering::Relaxed);
        limit.rlim_cur = 0;
        if libc::setrlimit(libc::RLIMIT_CORE, &limit) != 0 {
            eprintln!("setrlimit(RLIMIT_CORE): {}", std::io::Error::last_os_error());
            std::process::exit(1);
        }
    }
}

#[cfg(unix)]
pub fn reenable_dumping_core() {
    /* Silent failure is safe. */
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: getrlimit/setrlimit with a valid pointer to a local rlimit.
    unsafe {
        if libc::getrlimit(libc::RLIMIT_CORE, &mut limit) == 0 {
            limit.rlim_cur = SAVED_CORE_RLIMIT.load(Ordering::Relaxed) as libc::rlim_t;
            libc::setrlimit(libc::RLIMIT_CORE, &limit);
        }
    }
}
