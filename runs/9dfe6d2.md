result: failure
commit: 9dfe6d2f3720f7f8ebaecafb1d5f9e0c131d3790
subject: scrollback, server side: capability, alternate-screen flag, capture ring, ScrolledLines
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38381385956
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.75s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.4
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.5
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=9.0
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=10.8
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=8.7
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=18.2
interop: 30 cells, 30 passed, 0 failed, wall time 43s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=41081
Server now attached to client at 127.0.0.1:36293
Server now attached to client at 127.0.0.1:41081
elapsed 10.41s; client: status=done elapsed_ms=10407 packets_received=20 acked_ts_advance=10019; server: packets_received=49 acked_ts_advance=9842 remote_addr=127.0.0.1:41081; state 5956 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.41s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:39218
elapsed 10.34s; client: status=done elapsed_ms=10337 packets_received=43 acked_ts_advance=9985; server: packets_received=71 acked_ts_advance=9379 remote_addr=127.0.0.1:39218; state 6548 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.34s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.34s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:36866
elapsed 10.81s; client: status=done elapsed_ms=10803 packets_received=9 acked_ts_advance=10781; server: packets_received=34 acked_ts_advance=9589 remote_addr=127.0.0.1:36866; state 6250 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.81s
proxy stats: c2s forwarded=34 dropped=48 reordered=5 s2c forwarded=9 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.81s, 1 client address(es)
transport-impaired: all 3 runs passed
== upstream-check.sh
upstream-check: client=cxx (/home/runner/src/mosh/src/frontend/mosh-client) server=cxx (/home/runner/src/mosh/src/frontend/mosh-server)
upstream-check: copying /home/runner/src/mosh to /home/runner/work/mosh-rs/mosh-rs/target/upstream-check/mosh
upstream-check: make -n -C src/frontend mosh-client mosh-server:
  make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/target/upstream-check/mosh/src/frontend'
  make: 'mosh-client' is up to date.
  make: 'mosh-server' is up to date.
  make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/target/upstream-check/mosh/src/frontend'
upstream-check: make -q says both binaries are up to date
upstream-check: running make check (log: /home/runner/work/mosh-rs/mosh-rs/target/upstream-check/make-check.log)
# TOTAL: 32
# PASS:  30
# SKIP:  0
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=236

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.5
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=2.1
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.5
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.0
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=2.1
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.7
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.7
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=5.8
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.5
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.9
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.5
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=53805
Server now attached to client at 127.0.0.1:50050
Server now attached to client at 127.0.0.1:53805
elapsed 12.34s; client: status=done elapsed_ms=12334 packets_received=21 acked_ts_advance=11887; server: packets_received=51 acked_ts_advance=10136 remote_addr=127.0.0.1:53805; state 6337 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.34s
proxy stats: c2s forwarded=51 dropped=19 reordered=4 s2c forwarded=21 dropped=6 reordered=3
PASS loss30-rebind: converged in 12.34s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:53772
elapsed 10.42s; client: status=done elapsed_ms=10418 packets_received=43 acked_ts_advance=10038; server: packets_received=71 acked_ts_advance=9888 remote_addr=127.0.0.1:53772; state 6142 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:60417
elapsed 10.45s; client: status=done elapsed_ms=10443 packets_received=10 acked_ts_advance=10424; server: packets_received=43 acked_ts_advance=9512 remote_addr=127.0.0.1:60417; state 6112 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.45s
proxy stats: c2s forwarded=43 dropped=50 reordered=5 s2c forwarded=10 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.45s, 1 client address(es)
transport-impaired: all 3 runs passed
== upstream-check.sh
upstream-check: client=cxx (/home/runner/src/mosh/src/frontend/mosh-client) server=cxx (/home/runner/src/mosh/src/frontend/mosh-server)
upstream-check: copying /home/runner/src/mosh to /home/runner/work/mosh-rs/mosh-rs/target/upstream-check/mosh
upstream-check: make -n -C src/frontend mosh-client mosh-server:
  make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/target/upstream-check/mosh/src/frontend'
  make: 'mosh-client' is up to date.
  make: 'mosh-server' is up to date.
  make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/target/upstream-check/mosh/src/frontend'
upstream-check: make -q says both binaries are up to date
upstream-check: running make check (log: /home/runner/work/mosh-rs/mosh-rs/target/upstream-check/make-check.log)
# TOTAL: 32
# PASS:  30
# SKIP:  0
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=245

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
