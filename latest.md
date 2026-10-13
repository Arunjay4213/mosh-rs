result: failure
commit: e6257412b443849f83deb811eaf172c46ae2ba61
subject: statesync, network: subtract_self without copying (Complete: nothing, UserStream: clear); State::apply_string returns an error for malformed diffs and Transport::recv drops the packet (PLAN 9.8)
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38363168372
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.75s
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
    Finished `release` profile [optimized] target(s) in 0.01s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=48530
Server now attached to client at 127.0.0.1:58454
Server now attached to client at 127.0.0.1:48530
elapsed 10.41s; client: status=done elapsed_ms=10405 packets_received=20 acked_ts_advance=10016; server: packets_received=49 acked_ts_advance=9838 remote_addr=127.0.0.1:48530; state 6057 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.41s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:54605
elapsed 13.44s; client: status=done elapsed_ms=13440 packets_received=47 acked_ts_advance=13126; server: packets_received=76 acked_ts_advance=10166 remote_addr=127.0.0.1:54605; state 5849 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.44s
proxy stats: c2s forwarded=76 dropped=9 reordered=7 s2c forwarded=47 dropped=1 reordered=3
PASS loss5: converged in 13.44s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:58018
elapsed 10.47s; client: status=done elapsed_ms=10466 packets_received=9 acked_ts_advance=10266; server: packets_received=36 acked_ts_advance=9273 remote_addr=127.0.0.1:58018; state 5920 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.47s
proxy stats: c2s forwarded=36 dropped=39 reordered=6 s2c forwarded=9 dropped=6 reordered=0
PASS loss30-blackout: converged in 10.47s, 1 client address(es)
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
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=40602
Server now attached to client at 127.0.0.1:46798
Server now attached to client at 127.0.0.1:40602
elapsed 10.46s; client: status=done elapsed_ms=10454 packets_received=16 acked_ts_advance=10375; server: packets_received=50 acked_ts_advance=9465 remote_addr=127.0.0.1:40602; state 5919 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=50 dropped=14 reordered=4 s2c forwarded=16 dropped=7 reordered=2
PASS loss30-rebind: converged in 10.46s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:52221
elapsed 10.42s; client: status=done elapsed_ms=10417 packets_received=43 acked_ts_advance=10037; server: packets_received=71 acked_ts_advance=9887 remote_addr=127.0.0.1:52221; state 6433 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:38670
elapsed 10.80s; client: status=done elapsed_ms=10800 packets_received=9 acked_ts_advance=10779; server: packets_received=34 acked_ts_advance=9586 remote_addr=127.0.0.1:38670; state 5895 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.80s
proxy stats: c2s forwarded=34 dropped=50 reordered=5 s2c forwarded=9 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.80s, 1 client address(es)
transport-impaired: all 3 runs passed

run-all: cargo test plus 1 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
