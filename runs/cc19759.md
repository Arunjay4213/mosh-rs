result: failure
commit: cc19759f86b02b1855aed79781b4ebb14d230592
subject: network: rationalize sent and received states without cloning the front state (review F19)
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38358635919
fmt: success
clippy: success
test: failure

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.21s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-40aa4509f41f2638)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-a10eab8987f39cd9)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.04s

     Running tests/launcher.rs (target/release/deps/launcher-48d49136e47fba5c)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test launches_session_with_user_at_ip_literal ... ok
test missing_server_is_reported ... FAILED
test launches_session_on_localhost_with_command ... FAILED

failures:

---- missing_server_is_reported stdout ----
status: exit status: 255
stdout: stderr: TERM environment variable not set.
Can't count colors: /usr/bin/mosh-client -c exited with exit status: 1


thread 'missing_server_is_reported' (18452) panicked at tests/tests/launcher.rs:205:5:
missing mosh.pl error message
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- launches_session_on_localhost_with_command stdout ----

thread 'launches_session_on_localhost_with_command' (18189) panicked at tests/tests/launcher.rs:116:13:
timed out waiting for "LAUNCH_OK"; screen:



mosh did not make a successful connection to 127.0.0.1:60500.
Please verify that UDP port 60500 is not firewalled and can reach the server.

(By default, mosh uses a UDP port between 60000 and 61000. The -p option
selects a specific UDP port number.)
[mosh is exiting.]
MOSH_EXIT=1

















failures:
    launches_session_on_localhost_with_command
    missing_server_is_reported

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 30.35s

error: test failed, to rerun pass `-p mosh-tests --test launcher`
```

## log-test-ubuntu-latest (last 80 lines)
```

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-cee3d1d7a6e21b21)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-169cb0af354bbf50)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.86s

     Running tests/launcher.rs (target/release/deps/launcher-fc5dae2c425b84b9)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test launches_session_with_user_at_ip_literal ... ok
test missing_server_is_reported ... FAILED
test launches_session_on_localhost_with_command ... FAILED

failures:

---- missing_server_is_reported stdout ----
status: exit status: 255
stdout: stderr: TERM environment variable not set.
Can't count colors: /usr/bin/mosh-client -c exited with exit status: 1


thread 'missing_server_is_reported' (18544) panicked at tests/tests/launcher.rs:205:5:
missing mosh.pl error message
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- launches_session_on_localhost_with_command stdout ----

thread 'launches_session_on_localhost_with_command' (18277) panicked at tests/tests/launcher.rs:116:13:
timed out waiting for "LAUNCH_OK"; screen:



mosh did not make a successful connection to 127.0.0.1:60500.
Please verify that UDP port 60500 is not firewalled and can reach the server.

(By default, mosh uses a UDP port between 60000 and 61000. The -p option
selects a specific UDP port number.)
[mosh is exiting.]
MOSH_EXIT=1

















failures:
    launches_session_on_localhost_with_command
    missing_server_is_reported

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 30.56s

error: test failed, to rerun pass `-p mosh-tests --test launcher`
```
