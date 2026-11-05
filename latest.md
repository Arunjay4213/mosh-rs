result: failure
commit: c76c1c0ac269db26807d9efad5a6e3da706e8f93
branch: laptop/windows-static-crt
pr: 60
subject: README: Windows executables need no VC++ redistributable
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/42107190586
fmt: cancelled
clippy: cancelled
test: cancelled

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.81s
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
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 19.60s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.62s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-08d72246e2035d2b)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 25.69s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.97s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-2a96afc4d9ee4071)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
     Running unittests src/bin/mosh-server.rs (target/release/deps/mosh_server-c34f69d780ac7201)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_network-89cf6380220114b2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-77ab1100a5f6c2dc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-ce2d96a3eeeab714)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tools-de3717d88eecfd90)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/bench/main.rs (target/release/deps/bench-ac6f389e9a6584f7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/pty-record/main.rs (target/release/deps/pty_record-1e82ab66e571eb47)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/udp-impair/main.rs (target/release/deps/udp_impair-be46261dad5c3f19)

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
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 18.76s
== tests/interop-setup-loss.sh
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== tests/interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.0
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.4
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.6
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
```
