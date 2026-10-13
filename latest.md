result: failure
commit: b8706dae0a48da13e95ce91a89465199f327fb7e
branch: agent/phase5-gate
pr: 29
subject: Phase 5 gate PASSED: benchmarks published, v0.1.0 released; README status
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38454147633
fmt: cancelled
clippy: cancelled
test: cancelled

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.91s
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
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 15.99s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.36s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-05d18a1cd9e42bce)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
test killed_server_shows_last_contact ... ok
test no_server_times_out ... ok
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 24.44s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.18s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-8809f317143d72dc)

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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 10.45s
     Running unittests src/lib.rs (target/release/deps/mosh_tests-207ce313a7611bbc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-e683c8be01c5dc73)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/hostile-scrollback-server.rs (target/release/deps/hostile_scrollback_server-670ce2598de8dbd8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/scrollback-probe.rs (target/release/deps/scrollback_probe-699b5b3262352204)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync/main.rs (target/release/deps/toy_sync-f7bab2cd6098770b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-418ca7dd7ee9a24f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-a514b26e1f662616)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-0963149abd7f76d0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --test crypto_conformance
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.82s
     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-0aecc35e12285198)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.53s

== cargo test --release -p mosh-tests --test terminal_conformance
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
     Running unittests src/bin/mosh-server.rs (target/release/deps/mosh_server-97686eb1b6146fbb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_network-5da789ad08e44d8d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-6dfede581caad4b7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-936453b01bc39a3a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tools-e7d45bf71444b79f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/bench/main.rs (target/release/deps/bench-43f0c9476fafbe2d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/pty-record/main.rs (target/release/deps/pty_record-b9b1ffcf70908418)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/udp-impair/main.rs (target/release/deps/udp_impair-2c17e91983d2db3a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests mosh_network

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_platform

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_terminal

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tools

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --lib --bins --no-fail-fast
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 18.43s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.1
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.0
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.2
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=9.8
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.0
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=8.7
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.9
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.7
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 15.91s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.1
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.4
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=11.9
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=11.9
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.9
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=10.5
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.3
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
```

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 16.00s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.75s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-9b65167a4cb2eab7)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.03s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.38s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-66662286f7fb856d)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 30.96s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.09s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-5942c57d8ea2b063)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.90s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
```
