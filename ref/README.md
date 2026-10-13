# ref

Small C++ programs linked against the upstream mosh build (`upstream/`, mosh master at decd9b7), used as oracles for differential testing: `ref-term` (emulator and `new_frame`) and `ref-crypto` (OCB3 packets).
Each directory's README describes its command line; the Rust twins are `vt-check` and `crypto-check` in `tests/src/bin/`.
Not shipped.
See PLAN.md section 3.
