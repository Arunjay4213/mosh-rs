result: failure
commit: 21529701c73f05859d20a938f8b881cc1d34e3c6
branch: laptop/controlmaster
pr: 50
subject: tests: ControlMaster test searches the pane with line wraps removed; INTERVENTIONS entry
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38595613816
fmt: success
clippy: cancelled
test: cancelled

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.59s
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
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 20.72s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.58s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-3b14c01bd37ee6e5)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.01s
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
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 25.96s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.15s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-b972b3dc43fc6b4f)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.03s
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
   Doc-tests mosh_terminal

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tools

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --lib --bins --no-fail-fast
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 11.25s
     Running unittests src/lib.rs (target/release/deps/mosh_tests-4bdad211b4512352)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-74147b760c6fae29)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/hostile-scrollback-server.rs (target/release/deps/hostile_scrollback_server-3ab7d92c1d4f5a60)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/scrollback-probe.rs (target/release/deps/scrollback_probe-9f05586dec480899)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync/main.rs (target/release/deps/toy_sync-8bb45ea8dc5069e2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-e0e3db6113d24600)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-440ed66e36ca8bfc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-29b27128c7131cac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

== cargo test --release -p mosh-tests --test crypto_conformance
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.02s
     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-17a90ee4dac23a9c)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests  -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests  -g -O2 -Wall -Dmain=ocb_aes_main -c -o ocb-aes.o ../../upstream/src/tests/ocb-aes.cc
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests  -g -O2 -Wall -c -o test_utils.o ../../upstream/src/tests/test_utils.cc
c++ -std=gnu++17 -g -O2 -Wall -o ref-crypto ref-crypto.o ocb-aes.o test_utils.o ../../upstream/src/network/libmoshnetwork.a ../../upstream/src/crypto/libmoshcrypto.a ../../upstream/src/util/libmoshutil.a -lssl -lcrypto -lcrypto
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.16s

== cargo test --release -p mosh-tests --test terminal_conformance
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
     Running unittests src/bin/mosh-server.rs (target/release/deps/mosh_server-e9b00254082660a7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_network-0e445388c486d673)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-0fb65a311d4eb9f8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-e2218486105f8e15)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tools-d9d7f2a9e6fd3e08)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/bench/main.rs (target/release/deps/bench-27a18bee58d64845)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/pty-record/main.rs (target/release/deps/pty_record-525c0151b757d316)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/udp-impair/main.rs (target/release/deps/udp_impair-1bdc7ee371e3e7b1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

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
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 23.21s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.1
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.6
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.0
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.4
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.1
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 23.01s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.8
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.9
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=clean scenario=typing seconds=11.6
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=11.7
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=6.6
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=9.6
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=11.8
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=14.8
interop: 30 cells, 30 passed, 0 failed, wall time 39s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
```

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 16.02s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.77s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-69dd234075438fc1)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.84s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.62s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-219dcd6c3e572b42)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 24.20s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.31s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-936cf87a3ec1e824)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.90s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.98s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-302f71dd22d1993a)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
```
