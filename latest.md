result: failure
commit: 31af3f31d7bb158bb23077697b1f828ccfdc3e1b
subject: Rules: read the markdown before work, update it in the same commit as the change
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38356911292
fmt: success
clippy: success
test: failure

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.55s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-ad4d990e572312f3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-f093f1482b687e89)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tests-61d233b33c336685)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-4d0a991918ea36bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync.rs (target/release/deps/toy_sync-3ca43513607827d0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-154069677c0bb766)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-21d2b4854e3dfdcb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-75be59f05b5a6b32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-a3a42611cdc0a2ed)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
ref-crypto.cc:48:10: fatal error: src/crypto/crypto.h: No such file or directory
   48 | #include "src/crypto/crypto.h"
      |          ^~~~~~~~~~~~~~~~~~~~~
compilation terminated.
make: *** [Makefile:20: ref-crypto.o] Error 1
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... FAILED

failures:

---- crypto_conformance stdout ----

thread 'crypto_conformance' (3019) panicked at tests/tests/crypto_conformance.rs:187:5:
make -C ref/ref-crypto failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    crypto_conformance

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s

error: test failed, to rerun pass `-p mosh-tests --test crypto_conformance`
```

## log-test-ubuntu-latest (last 80 lines)
```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-6dfede581caad4b7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-936453b01bc39a3a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tests-de153860463d1424)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-5f3c6c303bd92025)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync.rs (target/release/deps/toy_sync-a7e8633597707ea4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-c683db55508d9b09)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-c53c7a329874620b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-dd51831fcbd707af)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-2c4bb4d6dc0b5847)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
ref-crypto.cc:48:10: fatal error: src/crypto/crypto.h: No such file or directory
   48 | #include "src/crypto/crypto.h"
      |          ^~~~~~~~~~~~~~~~~~~~~
compilation terminated.
make: *** [Makefile:20: ref-crypto.o] Error 1
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... FAILED

failures:

---- crypto_conformance stdout ----

thread 'crypto_conformance' (2959) panicked at tests/tests/crypto_conformance.rs:187:5:
make -C ref/ref-crypto failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    crypto_conformance

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.24s

error: test failed, to rerun pass `-p mosh-tests --test crypto_conformance`
```
