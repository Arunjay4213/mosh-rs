result: failure
commit: f3806ac28c9482ac702fea2918d501175d7586fd
subject: mosh-server: port of mosh-server.cc; platform server syscalls; server_e2e against C++ client
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38364726057
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.38s
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
control at 5s: rebind -> ok rebind port=54615
Server now attached to client at 127.0.0.1:53412
Server now attached to client at 127.0.0.1:54615
elapsed 11.03s; client: status=done elapsed_ms=11030 packets_received=15 acked_ts_advance=10975; server: packets_received=50 acked_ts_advance=9861 remote_addr=127.0.0.1:54615; state 6091 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.03s
proxy stats: c2s forwarded=50 dropped=15 reordered=6 s2c forwarded=15 dropped=6 reordered=0
PASS loss30-rebind: converged in 11.03s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:51188
elapsed 10.37s; client: status=done elapsed_ms=10364 packets_received=41 acked_ts_advance=10022; server: packets_received=72 acked_ts_advance=9865 remote_addr=127.0.0.1:51188; state 6153 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.37s
proxy stats: c2s forwarded=72 dropped=6 reordered=7 s2c forwarded=41 dropped=2 reordered=2
PASS loss5: converged in 10.37s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:55219
elapsed 10.47s; client: status=done elapsed_ms=10466 packets_received=9 acked_ts_advance=10266; server: packets_received=36 acked_ts_advance=9271 remote_addr=127.0.0.1:55219; state 5822 bytes
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
control at 5s: rebind -> ok rebind port=52907
Server now attached to client at 127.0.0.1:33688
Server now attached to client at 127.0.0.1:52907
elapsed 10.41s; client: status=done elapsed_ms=10404 packets_received=20 acked_ts_advance=10016; server: packets_received=49 acked_ts_advance=9839 remote_addr=127.0.0.1:52907; state 6250 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.41s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:43433
elapsed 10.46s; client: status=done elapsed_ms=10457 packets_received=46 acked_ts_advance=10023; server: packets_received=73 acked_ts_advance=9880 remote_addr=127.0.0.1:43433; state 6041 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=73 dropped=5 reordered=8 s2c forwarded=46 dropped=4 reordered=2
PASS loss5: converged in 10.46s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:56234
elapsed 10.46s; client: status=done elapsed_ms=10453 packets_received=11 acked_ts_advance=10058; server: packets_received=38 acked_ts_advance=9809 remote_addr=127.0.0.1:56234; state 6313 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=38 dropped=52 reordered=4 s2c forwarded=11 dropped=6 reordered=2
PASS loss30-blackout: converged in 10.46s, 1 client address(es)
transport-impaired: all 3 runs passed

run-all: cargo test plus 1 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
