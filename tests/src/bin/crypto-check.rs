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
// Ported from: (none; Rust counterpart of ref/ref-crypto/ref-crypto.cc; vectors mode follows upstream/src/tests/ocb-aes.cc)

//! crypto-check: the mosh-rs side of the crypto oracle. Same CLI and
//! output as ref/ref-crypto (see ref/ref-crypto/README.md). Uses only
//! mosh_network.

use std::io::{Read, Write};
use std::process::ExitCode;

use mosh_network::crypto::ae::{ae_decrypt, ae_encrypt, ae_init, KEY_LEN, NONCE_LEN, TAG_LEN};
use mosh_network::crypto::prng::Prng;
use mosh_network::{Base64Key, Direction, Packet, Session};

fn usage() -> ExitCode {
    eprintln!(
        "usage: crypto-check enc KEY DIR SEQ TS TSR < payload\n       crypto-check dec KEY < hex\n       crypto-check vectors"
    );
    ExitCode::from(2)
}

fn read_stdin() -> Vec<u8> {
    let mut v = Vec::new();
    std::io::stdin().read_to_end(&mut v).expect("read stdin");
    v
}

fn to_hex(s: &[u8]) -> String {
    s.iter().map(|b| format!("{b:02x}")).collect()
}

/// Returns None on malformed hex. ASCII whitespace is ignored.
fn from_hex(input: &[u8]) -> Option<Vec<u8>> {
    let digits: Vec<u8> = input
        .iter()
        .copied()
        .filter(|c| !matches!(c, b' ' | b'\n' | b'\r' | b'\t'))
        .collect();
    if !digits.len().is_multiple_of(2) {
        return None;
    }
    let val = |c: u8| (c as char).to_digit(16);
    digits
        .chunks(2)
        .map(|p| Some((val(p[0])? * 16 + val(p[1])?) as u8))
        .collect()
}

fn parse_u64(s: &str, max: u64) -> Option<u64> {
    if !s.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    s.parse::<u64>().ok().filter(|v| *v <= max)
}

fn do_enc(args: &[String]) -> ExitCode {
    if args.len() != 7 {
        return usage();
    }
    let (Some(dir), Some(seq), Some(ts), Some(tsr)) = (
        parse_u64(&args[3], 1),
        parse_u64(&args[4], u64::MAX),
        parse_u64(&args[5], 65535),
        parse_u64(&args[6], 65535),
    ) else {
        return usage();
    };
    let payload = read_stdin();
    let key = match Base64Key::from_printable(&args[2]) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("crypto-check: {e}");
            println!("error");
            return ExitCode::from(1);
        }
    };
    let mut session = Session::new(key);
    let p = Packet {
        seq,
        direction: if dir == 1 { Direction::ToClient } else { Direction::ToServer },
        timestamp: ts as u16,
        timestamp_reply: tsr as u16,
        payload,
    };
    match session.encrypt(&p.to_message()) {
        Ok(wire) => {
            println!("{}", to_hex(&wire));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("crypto-check: {e}");
            println!("error");
            ExitCode::from(1)
        }
    }
}

fn do_dec(args: &[String]) -> ExitCode {
    if args.len() != 3 {
        return usage();
    }
    let Some(wire) = from_hex(&read_stdin()) else {
        eprintln!("crypto-check: malformed hex on stdin");
        return ExitCode::from(2);
    };
    let result = Base64Key::from_printable(&args[2])
        .and_then(|key| Session::new(key).decrypt(&wire))
        .and_then(|m| Packet::from_message(&m));
    match result {
        Ok(p) => {
            println!(
                "dir={} seq={} ts={} tsr={} payload={}",
                if p.direction == Direction::ToClient { 1 } else { 0 },
                p.seq,
                p.timestamp,
                p.timestamp_reply,
                to_hex(&p.payload)
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("crypto-check: {e}");
            println!("error");
            ExitCode::from(1)
        }
    }
}

/* ---- vectors: the test vectors of upstream/src/tests/ocb-aes.cc ---- */

/// Path of upstream's OCB test program, whose TEST_VECTOR() calls are
/// read at run time so the vectors are exactly upstream's.
fn ocb_aes_source() -> String {
    std::env::var("MOSH_OCB_AES_CC")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../upstream/src/tests/ocb-aes.cc").to_string())
}

/// Parses one C expression made of adjacent string literals (with
/// comments between them) into bytes. Only the escapes used in
/// ocb-aes.cc (`\xHH...`) are accepted.
fn c_string_concat(expr: &str) -> Option<Vec<u8>> {
    let b = expr.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut saw_literal = false;
    while i < b.len() {
        match b[i] {
            b' ' | b'\n' | b'\t' | b'\r' => i += 1,
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let end = expr[i + 2..].find("*/")? + i + 2;
                i = end + 2;
            }
            b'"' => {
                saw_literal = true;
                i += 1;
                while b[i] != b'"' {
                    if b[i] == b'\\' {
                        if b[i + 1] != b'x' {
                            return None;
                        }
                        i += 2;
                        let mut v: u32 = 0;
                        let start = i;
                        while i < b.len() && b[i].is_ascii_hexdigit() {
                            v = v * 16 + (b[i] as char).to_digit(16)?;
                            i += 1;
                        }
                        if i == start || v > 0xff {
                            return None;
                        }
                        out.push(v as u8);
                    } else {
                        out.push(b[i]);
                        i += 1;
                    }
                }
                i += 1;
            }
            _ => return None,
        }
    }
    saw_literal.then_some(out)
}

/// Splits macro arguments at top-level commas (outside string literals and comments).
fn split_args(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'"' => {
                let mut j = i + 1;
                while b[j] != b'"' {
                    j += if b[j] == b'\\' { 2 } else { 1 };
                }
                cur.push_str(&s[i..=j]);
                i = j + 1;
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let end = s[i + 2..].find("*/").expect("closed comment") + i + 2;
                i = end + 2;
            }
            b',' => {
                args.push(std::mem::take(&mut cur));
                i += 1;
            }
            c => {
                cur.push(c as char);
                i += 1;
            }
        }
    }
    args.push(cur);
    args
}

struct Vector {
    key: Vec<u8>,
    nonce: Vec<u8>,
    assoc: Vec<u8>,
    pt: Vec<u8>,
    ct: Vec<u8>,
}

fn load_vectors() -> Result<Vec<Vector>, String> {
    let path = ocb_aes_source();
    let src = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let body_start = src
        .find("static void test_all_vectors( void )")
        .ok_or("test_all_vectors not found")?;
    let body = &src[body_start..];
    let body = &body[..body.find("\n}\n").ok_or("end of test_all_vectors not found")?];

    let named = |name: &str| -> Result<Vec<u8>, String> {
        let decl = format!("const char {name}[] = ");
        let at = body.find(&decl).ok_or(format!("{name} not found"))? + decl.len();
        let end = body[at..].find(';').ok_or("unterminated declaration")? + at;
        c_string_concat(&body[at..end]).ok_or(format!("cannot parse {name}"))
    };
    let ietf_key = named("ietf_key")?;
    let ietf_nonce = named("ietf_nonce")?;

    let mut vectors = Vec::new();
    let mut rest = body;
    while let Some(at) = rest.find("TEST_VECTOR(") {
        let args_start = at + "TEST_VECTOR(".len();
        let end = rest[args_start..].find(");").ok_or("unterminated TEST_VECTOR")? + args_start;
        let args = split_args(&rest[args_start..end]);
        if args.len() != 5 {
            return Err(format!("TEST_VECTOR with {} arguments", args.len()));
        }
        let arg = |a: &str| -> Result<Vec<u8>, String> {
            match a.trim() {
                "ietf_key" => Ok(ietf_key.clone()),
                "ietf_nonce" => Ok(ietf_nonce.clone()),
                other => c_string_concat(other).ok_or(format!("cannot parse argument {other:.40}")),
            }
        };
        vectors.push(Vector {
            key: arg(&args[0])?,
            nonce: arg(&args[1])?,
            assoc: arg(&args[2])?,
            pt: arg(&args[3])?,
            ct: arg(&args[4])?,
        });
        rest = &rest[end..];
    }
    Ok(vectors)
}

/// One upstream `test_vector()`: encrypt, decrypt, and 64 single-bit
/// corruptions that must not validate.
fn test_vector(v: &Vector, prng: &mut Prng) -> Result<(), String> {
    let key: [u8; KEY_LEN] = v.key.as_slice().try_into().map_err(|_| "key length")?;
    let nonce: [u8; NONCE_LEN] = v.nonce.as_slice().try_into().map_err(|_| "nonce length")?;
    let ctx = ae_init(&key);

    let observed_ct = ae_encrypt(&ctx, &nonce, &v.pt, &v.assoc);
    if observed_ct != v.ct {
        return Err(format!("encrypt mismatch: {} != {}", to_hex(&observed_ct), to_hex(&v.ct)));
    }
    match ae_decrypt(&ctx, &nonce, &v.ct, &v.assoc) {
        Some(pt) if pt == v.pt => {}
        _ => return Err("decrypt mismatch".to_string()),
    }

    /* Try some bad ciphertexts and make sure they don't validate. */
    for _ in 0..64 {
        let mut bad_ct = v.ct.clone();
        let idx = prng.uint32().map_err(|e| e.to_string())? as usize % bad_ct.len();
        let bit = prng.uint8().map_err(|e| e.to_string())? % 8;
        bad_ct[idx] ^= 1 << bit;
        if ae_decrypt(&ctx, &nonce, &bad_ct, &v.assoc).is_some() {
            return Err("corrupted ciphertext validated".to_string());
        }
    }
    Ok(())
}

/// upstream `test_iterative()` (RFC 7253 appendix A, iterated vector).
fn test_iterative() -> Result<(), String> {
    /* Key is always all zeros */
    let ctx = ae_init(&[0u8; KEY_LEN]);
    let mut nonce = [0u8; NONCE_LEN];

    let mut accumulator = Vec::with_capacity(22400);
    for i in 0..128usize {
        /* i bytes of zeros */
        let s = vec![0u8; i];

        /* Nonce is 11 bytes of zeros followed by 1 byte i */
        nonce[11] = i as u8;

        /* OCB-ENCRYPT(K,N,S,S) */
        accumulator.extend_from_slice(&ae_encrypt(&ctx, &nonce, &s, &s));
        /* OCB-ENCRYPT(K,N,<empty string>,S) */
        accumulator.extend_from_slice(&ae_encrypt(&ctx, &nonce, &s, &[]));
        /* OCB-ENCRYPT(K,N,S,<empty string>) */
        accumulator.extend_from_slice(&ae_encrypt(&ctx, &nonce, &[], &s)[..TAG_LEN]);
    }
    if accumulator.len() != 22400 {
        return Err(format!("accumulator length {}", accumulator.len()));
    }

    /* OCB-ENCRYPT(K,N,C,<empty string>) */
    let nonce = [0u8; NONCE_LEN];
    let out = ae_encrypt(&ctx, &nonce, &[], &accumulator);

    /* Check this final tag against the known value */
    let correct = [
        0xB2, 0xB4, 0x1C, 0xBF, 0x9B, 0x05, 0x03, 0x7D, 0xA7, 0xF1, 0x6C, 0x24, 0xA3, 0x5C, 0x1C, 0x94,
    ];
    if out != correct {
        return Err("iterative tag mismatch".to_string());
    }
    Ok(())
}

fn do_vectors() -> ExitCode {
    let result = (|| -> Result<usize, String> {
        let vectors = load_vectors()?;
        if vectors.is_empty() {
            return Err("no vectors found".to_string());
        }
        let mut prng = Prng::new();
        for (n, v) in vectors.iter().enumerate() {
            test_vector(v, &mut prng).map_err(|e| format!("vector {n}: {e}"))?;
        }
        test_iterative()?;
        Ok(vectors.len())
    })();
    match result {
        Ok(n) => {
            eprintln!("crypto-check: {n} vectors and the iterative test passed");
            println!("ok");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("crypto-check: {e}");
            println!("FAIL");
            ExitCode::from(1)
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("enc") => do_enc(&args),
        Some("dec") => do_dec(&args),
        Some("vectors") if args.len() == 2 => do_vectors(),
        _ => usage(),
    };
    std::io::stdout().flush().expect("flush stdout");
    code
}
