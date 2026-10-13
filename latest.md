result: failure
commit: ed970c18508706df01fcef68ed5bdde8b6655716
subject: INTERVENTIONS: launcher CI failures diagnosed, run-all not in CI
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38359793421
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.56s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-4feb915907cef5c2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-df0a40d873ed74c4)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.18s

     Running tests/launcher.rs (target/release/deps/launcher-28457c42d342bfd4)

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


thread 'missing_server_is_reported' (18482) panicked at tests/tests/launcher.rs:205:5:
missing mosh.pl error message
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- launches_session_on_localhost_with_command stdout ----

thread 'launches_session_on_localhost_with_command' (18219) panicked at tests/tests/launcher.rs:116:13:
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

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 30.39s

error: test failed, to rerun pass `-p mosh-tests --test launcher`
```

## log-test-ubuntu-latest (last 80 lines)
```

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-d26d1c04ae487750)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-711290cf08aa3ed2)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.57s

     Running tests/launcher.rs (target/release/deps/launcher-92d422cdd2a3dec9)

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


thread 'missing_server_is_reported' (18566) panicked at tests/tests/launcher.rs:205:5:
missing mosh.pl error message
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- launches_session_on_localhost_with_command stdout ----

thread 'launches_session_on_localhost_with_command' (18301) panicked at tests/tests/launcher.rs:116:13:
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

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 30.55s

error: test failed, to rerun pass `-p mosh-tests --test launcher`
```
