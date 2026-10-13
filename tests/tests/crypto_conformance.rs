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
// Ported from: (none; differential test of crypto-check against ref/ref-crypto)

//! Differential test: upstream's crypto (ref/ref-crypto) against
//! mosh_network (crypto-check). Every case must give byte-identical
//! output from both programs.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use mosh_network::crypto::base64::base64_encode;
use rand::rngs::StdRng;
use rand::{Rng, RngExt, SeedableRng};

const CASES: usize = 2000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn ref_crypto() -> PathBuf {
    repo_root().join("ref/ref-crypto/ref-crypto")
}

fn crypto_check() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_crypto-check"))
}

/// Runs a program with stdin; returns (exit code, stdout).
fn run(prog: &PathBuf, args: &[&str], stdin: &[u8]) -> (i32, String) {
    let mut child = Command::new(prog)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", prog.display()));
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(stdin)
        .expect("write stdin");
    let out = child.wait_with_output().expect("wait");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8(out.stdout).expect("utf8 stdout"),
    )
}

/// Runs the same command on both programs and requires identical results.
fn both(args: &[&str], stdin: &[u8], what: &str) -> (i32, String) {
    let r = run(&ref_crypto(), args, stdin);
    let c = run(&crypto_check(), args, stdin);
    assert_eq!(
        r, c,
        "{what}: ref-crypto and crypto-check differ for args {args:?}"
    );
    r
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

fn key_string(rng: &mut StdRng) -> String {
    let mut raw = [0u8; 16];
    rng.fill_bytes(&mut raw);
    String::from_utf8(base64_encode(&raw)[..22].to_vec()).expect("ascii")
}

struct Case {
    key: String,
    dir: u64,
    seq: u64,
    ts: u16,
    tsr: u16,
    payload: Vec<u8>,
    flip_at: usize,
    flip_mask: u8,
}

fn check_case(c: &Case) {
    let (dir, seq, ts, tsr) = (
        c.dir.to_string(),
        c.seq.to_string(),
        c.ts.to_string(),
        c.tsr.to_string(),
    );
    let enc_args = ["enc", &c.key, &dir, &seq, &ts, &tsr];

    /* encrypt: byte-identical wire packets */
    let r = run(&ref_crypto(), &enc_args, &c.payload);
    let k = run(&crypto_check(), &enc_args, &c.payload);
    assert_eq!(r.0, 0, "ref-crypto enc failed for {enc_args:?}");
    assert_eq!(
        r,
        k,
        "enc differs for {enc_args:?} payload {}",
        hex(&c.payload)
    );
    let wire = r.1.trim_end().to_string();
    assert_eq!(
        wire.len(),
        2 * (8 + 4 + c.payload.len() + 16),
        "wire length"
    );

    /* decrypt each other's packet */
    let expected = format!(
        "dir={} seq={} ts={} tsr={} payload={}\n",
        c.dir,
        c.seq & !(1u64 << 63),
        c.ts,
        c.tsr,
        hex(&c.payload)
    );
    let ref_of_rust = run(&ref_crypto(), &["dec", &c.key], k.1.as_bytes());
    let rust_of_ref = run(&crypto_check(), &["dec", &c.key], r.1.as_bytes());
    assert_eq!(
        ref_of_rust,
        (0, expected.clone()),
        "ref-crypto dec of crypto-check packet"
    );
    assert_eq!(
        rust_of_ref,
        (0, expected),
        "crypto-check dec of ref-crypto packet"
    );

    /* one flipped byte must be rejected by both */
    let mut bad = unhex(&wire);
    let at = c.flip_at % bad.len();
    bad[at] ^= c.flip_mask;
    let bad_hex = hex(&bad) + "\n";
    let res = both(&["dec", &c.key], bad_hex.as_bytes(), "flipped byte");
    assert_eq!(
        res,
        (1, "error\n".to_string()),
        "flipped byte at {at} was accepted"
    );
}

#[test]
fn crypto_conformance() {
    let make = Command::new("make")
        .arg("-C")
        .arg(repo_root().join("ref/ref-crypto"))
        .status()
        .expect("run make -C ref/ref-crypto");
    assert!(make.success(), "make -C ref/ref-crypto failed");

    /* RFC 7253 vectors */
    assert_eq!(
        run(&ref_crypto(), &["vectors"], b""),
        (0, "ok\n".to_string()),
        "ref-crypto vectors"
    );
    assert_eq!(
        run(&crypto_check(), &["vectors"], b""),
        (0, "ok\n".to_string()),
        "crypto-check vectors"
    );

    let seed: u64 = match std::env::var("CRYPTO_SEED") {
        Ok(s) => s.parse().expect("CRYPTO_SEED is a u64"),
        Err(_) => rand::random(),
    };
    println!("crypto_conformance seed={seed} (rerun with CRYPTO_SEED={seed})");
    let mut rng = StdRng::seed_from_u64(seed);

    let special_seqs = [0u64, 1, 1 << 32, (1 << 63) - 1];
    let cases: Vec<Case> = (0..CASES)
        .map(|i| {
            let seq = match i % 5 {
                n @ 0..=3 => special_seqs[n],
                _ => rng.random(),
            };
            let len = if i % 50 == 0 {
                0
            } else {
                rng.random_range(0..=1400usize)
            };
            let mut payload = vec![0u8; len];
            rng.fill_bytes(&mut payload);
            Case {
                key: key_string(&mut rng),
                dir: (i as u64 / 5) % 2,
                seq,
                ts: if i % 7 == 0 { 65535 } else { rng.random() },
                tsr: if i % 11 == 0 { 65535 } else { rng.random() },
                payload,
                flip_at: rng.random::<u32>() as usize,
                flip_mask: rng.random_range(1..=255u8),
            }
        })
        .collect();

    let start = std::time::Instant::now();
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|s| {
        for t in 0..threads {
            let cases = &cases;
            s.spawn(move || {
                for c in cases.iter().skip(t).step_by(threads) {
                    check_case(c);
                }
            });
        }
    });
    println!(
        "{CASES} random cases identical in {:.1}s",
        start.elapsed().as_secs_f64()
    );

    /* key parsing: upstream's bad keys of length 22, and keys whose last
    character carries bits beyond 128 */
    let mut bad_keys: Vec<String> = vec![
        "AAAAAAAAAAAAAAAAAAAAA~".into(),
        "~AAAAAAAAAAAAAAAAAAAAA".into(),
        "AAAAAAAAAA~AAAAAAAAAAA".into(),
        "AAAAAAAAAAAAAAAAAAAAA=".into(),
        "AAAAAAAAAAAAAAAAAAAAAAA".into(),
        "AAAAAAAAAAAAAAAAAAAAA".into(),
        "".into(),
    ];
    for c in ["B", "C", "D", "P", "/", "+", "z"] {
        bad_keys.push(format!("AAAAAAAAAAAAAAAAAAAAA{c}"));
    }
    let mut accepted = 0;
    for key in &bad_keys {
        for args in [
            vec!["enc", key.as_str(), "0", "1", "2", "3"],
            vec!["dec", key.as_str()],
        ] {
            let stdin: &[u8] = if args[0] == "enc" { b"x" } else { b"00" };
            let res = both(&args, stdin, "key parsing");
            if res.0 == 0 {
                accepted += 1;
            }
        }
    }
    /* of these, only last characters A, Q, g, w are canonical; none listed */
    assert_eq!(accepted, 0, "a non-canonical key was accepted");
    println!("{} bad keys rejected by both", bad_keys.len());
}
