result: failure
commit: afda5aeea1f61980cd71156840ac56623ec76c34
branch: laptop/windows-verified
pr: 44
subject: README: Windows sleep/wake, idle CPU and Ctrl-^ Ctrl-Z results; INTERVENTIONS entry
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38490666063
fmt: success
clippy: cancelled
test: cancelled

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.87s
```

## log-fmt: failures
```
```

## log-fmt (last 80 lines)
```
```

## log-test-client-ubuntu-24.04-arm: failures
```
```

## log-test-client-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 18.36s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.07s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-07158bb8782756dc)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
test killed_server_shows_last_contact ... ok
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 23.19s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.14s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-9a2f539aa70c64aa)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
test killed_server_shows_last_contact ... ok
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tools

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --lib --bins --no-fail-fast
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 10.72s
     Running unittests src/lib.rs (target/release/deps/mosh_tests-8c2b5698bb3762a2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-c0cb09b509da95af)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/hostile-scrollback-server.rs (target/release/deps/hostile_scrollback_server-90f2bb6bf2a6ccf4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/scrollback-probe.rs (target/release/deps/scrollback_probe-24dd089922a9bc23)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync/main.rs (target/release/deps/toy_sync-8fe6d0942b759b3a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-b8e7901fa2d95209)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-e346e94ca7e99b6d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-ef2cc0025beac639)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --test crypto_conformance
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.91s
     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-5b20019bc3a1b2e1)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.38s

== cargo test --release -p mosh-tests --test terminal_conformance
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_terminal

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tools

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --lib --bins --no-fail-fast
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 14.65s
     Running unittests src/lib.rs (target/release/deps/mosh_tests-5f25247ace063d46)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-ed6660a489795bc4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/hostile-scrollback-server.rs (target/release/deps/hostile_scrollback_server-5a5925ac7e46ca81)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/scrollback-probe.rs (target/release/deps/scrollback_probe-3dfde5c726023cd0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync/main.rs (target/release/deps/toy_sync-aa205a89f91ba58f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-aca9fe3de9cc13bc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-ad1da3fb4a44c324)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-01deb80593e9e50d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --test crypto_conformance
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.56s
     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-4ecf359d0c0b7cd2)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 18.52s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.9
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.0
PASS client=cxx server=cxx profile=clean scenario=typing seconds=11.8
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=11.9
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=8.2
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=19.2
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.5
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 25.35s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.8
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.9
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=8.2
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.3
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
```

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 16.35s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.91s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-7626b7b0039291b8)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.82s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.96s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-626ce7029d02d391)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 15.41s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.21s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-5c5c4e833fd291ae)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.79s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.25s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-568ca713f7af071d)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
```
