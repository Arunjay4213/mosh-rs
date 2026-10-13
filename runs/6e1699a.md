result: failure
commit: 6e1699a6ff2852d55fa7144d7d4c43be0da6b6a8
subject: review T-1, T-2 resolved; STATUS and INTERVENTIONS: generated terminfo table, kitty-terminfo on devbox
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38361005210
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.55s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/pty-record.rs (target/release/deps/pty_record-a2763de2ee3ea8db)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/udp-impair.rs (target/release/deps/udp_impair-c3da5e153e8983fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_network

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_platform

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_terminal

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tools

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: 1 target failed:
    `-p mosh-tests --test terminfo_table`
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=52218
Server now attached to client at 127.0.0.1:38495
Server now attached to client at 127.0.0.1:52218
elapsed 10.41s; client: status=done elapsed_ms=10403 packets_received=20 acked_ts_advance=10015; server: packets_received=49 acked_ts_advance=9837 remote_addr=127.0.0.1:52218; state 5981 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.41s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:51417
elapsed 10.37s; client: status=done elapsed_ms=10368 packets_received=42 acked_ts_advance=10026; server: packets_received=71 acked_ts_advance=9866 remote_addr=127.0.0.1:51417; state 5973 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.37s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=42 dropped=1 reordered=2
PASS loss5: converged in 10.37s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:41569
elapsed 10.32s; client: status=done elapsed_ms=10322 packets_received=13 acked_ts_advance=9977; server: packets_received=40 acked_ts_advance=9749 remote_addr=127.0.0.1:41569; state 6523 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.32s
proxy stats: c2s forwarded=40 dropped=47 reordered=4 s2c forwarded=13 dropped=7 reordered=2
PASS loss30-blackout: converged in 10.32s, 1 client address(es)
transport-impaired: all 3 runs passed

run-all: cargo test plus 1 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/pty-record.rs (target/release/deps/pty_record-b9b1ffcf70908418)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/udp-impair.rs (target/release/deps/udp_impair-2c17e91983d2db3a)

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

   Doc-tests mosh_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mosh_tools

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: 1 target failed:
    `-p mosh-tests --test terminfo_table`
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=45348
Server now attached to client at 127.0.0.1:33138
Server now attached to client at 127.0.0.1:45348
elapsed 12.28s; client: status=done elapsed_ms=12281 packets_received=21 acked_ts_advance=11835; server: packets_received=51 acked_ts_advance=10102 remote_addr=127.0.0.1:45348; state 5874 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.28s
proxy stats: c2s forwarded=51 dropped=19 reordered=4 s2c forwarded=21 dropped=6 reordered=3
PASS loss30-rebind: converged in 12.28s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:56524
elapsed 10.40s; client: status=done elapsed_ms=10398 packets_received=44 acked_ts_advance=10027; server: packets_received=71 acked_ts_advance=9876 remote_addr=127.0.0.1:56524; state 6064 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.40s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:56330
elapsed 10.73s; client: status=done elapsed_ms=10732 packets_received=7 acked_ts_advance=10592; server: packets_received=39 acked_ts_advance=9320 remote_addr=127.0.0.1:56330; state 6174 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.73s
proxy stats: c2s forwarded=39 dropped=49 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.73s, 1 client address(es)
transport-impaired: all 3 runs passed

run-all: cargo test plus 1 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
