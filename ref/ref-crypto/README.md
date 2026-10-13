# ref-crypto

Oracle for the mosh crypto layer.
`ref-crypto` is built from `ref-crypto.cc` and linked against upstream's `libmoshnetwork.a`, `libmoshcrypto.a` and `libmoshutil.a` (OpenSSL AES-OCB), so it runs upstream's `Crypto::Base64Key`, `Crypto::Session`, `Crypto::Nonce` and `Network::Packet`.
Its Rust counterpart is `tests/src/bin/crypto-check.rs`, which uses only `mosh_network`.
Both programs have the identical CLI and output below; conformance means the outputs are byte-identical.

Build: `make -C ref/ref-crypto` (needs the built upstream tree at `upstream/`, plus `libssl-dev`).

## CLI

- `enc KEY DIR SEQ TS TSR` : KEY is the 22-char base64 key, DIR is 0 (to server) or 1 (to client), SEQ a u64 decimal, TS and TSR u16 decimal.
  Reads the payload bytes from stdin (binary, may be empty).
  Writes the wire packet to stdout as lowercase hex followed by newline.
  The plaintext is built exactly as `Network::Packet::toMessage` does (2-byte BE timestamp, 2-byte BE timestamp_reply, payload) with nonce value `(DIR << 63) | SEQ`, encrypted with `Crypto::Session`, with the 8 nonce bytes (`Nonce::cc_str`) prepended.
  SEQ is masked to its low 63 bits, as `Packet::toMessage` does.
- `dec KEY` : reads the wire packet as hex on stdin.
  On success prints one line `dir=<0|1> seq=<u64> ts=<u16> tsr=<u16> payload=<hex>` and exits 0.
  On authentication failure prints `error` and exits 1.
- `vectors` : runs the RFC 7253 vectors from `upstream/src/tests/ocb-aes.cc` and prints `ok` or `FAIL`.

Details shared by both programs:

- A KEY that `Base64Key` rejects makes `enc` and `dec` print `error` and exit 1 (the reason goes to stderr).
- `dec` also prints `error` and exits 1 for anything else upstream rejects with a `CryptoException`: fewer than 24 bytes, or a plaintext shorter than the 4 timestamp bytes.
- Hex input may contain ASCII whitespace; any other non-hex character, or an odd digit count, exits 2.
- Bad usage (wrong argument count, DIR not 0/1, SEQ/TS/TSR out of range) exits 2.
- `ref-crypto vectors` runs upstream's own `ocb-aes.cc` (compiled with `main` renamed) in a forked child, because it reports failure with `abort()`; it prints `ok` only if the child exits 0.
  `crypto-check vectors` reads the `TEST_VECTOR(...)` calls out of `upstream/src/tests/ocb-aes.cc` at run time (override the path with `MOSH_OCB_AES_CC`) and repeats upstream's checks: encrypt, decrypt, 64 random single-bit corruptions that must fail, and the iterative test.
