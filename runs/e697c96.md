result: failure
commit: e697c963f4ec5e0d3846d31a0d006977f63c4faf
subject: STATUS: setup under loss (agent M)
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38370541707
fmt: success
clippy: success
test: failure

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 20.61s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.0
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.8
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.8
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.0
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.6
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=9.8
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.3
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=45977
Server now attached to client at 127.0.0.1:47055
Server now attached to client at 127.0.0.1:45977
elapsed 10.41s; client: status=done elapsed_ms=10411 packets_received=20 acked_ts_advance=10023; server: packets_received=49 acked_ts_advance=9842 remote_addr=127.0.0.1:45977; state 5878 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.41s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:37032
elapsed 10.45s; client: status=done elapsed_ms=10445 packets_received=44 acked_ts_advance=10073; server: packets_received=71 acked_ts_advance=9917 remote_addr=127.0.0.1:37032; state 5920 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.45s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.45s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:51345
elapsed 11.17s; client: status=done elapsed_ms=11165 packets_received=11 acked_ts_advance=10783; server: packets_received=38 acked_ts_advance=10254 remote_addr=127.0.0.1:51345; state 6315 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.17s
proxy stats: c2s forwarded=38 dropped=47 reordered=6 s2c forwarded=11 dropped=7 reordered=0
PASS loss30-blackout: converged in 11.17s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=239

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.8
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.9
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.8
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.2
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.7
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.8
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.8
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.8
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.9
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=14.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.9
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.2
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=13.2
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=18.4
interop: 30 cells, 30 passed, 0 failed, wall time 44s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=43817
Server now attached to client at 127.0.0.1:56110
Server now attached to client at 127.0.0.1:43817
elapsed 10.42s; client: status=done elapsed_ms=10420 packets_received=13 acked_ts_advance=10334; server: packets_received=48 acked_ts_advance=9408 remote_addr=127.0.0.1:43817; state 6053 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=48 dropped=15 reordered=6 s2c forwarded=13 dropped=5 reordered=0
PASS loss30-rebind: converged in 10.42s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:60248
elapsed 10.77s; client: status=done elapsed_ms=10770 packets_received=47 acked_ts_advance=10685; server: packets_received=75 acked_ts_advance=10051 remote_addr=127.0.0.1:60248; state 6043 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.77s
proxy stats: c2s forwarded=75 dropped=5 reordered=6 s2c forwarded=47 dropped=5 reordered=4
PASS loss5: converged in 10.77s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:40748
elapsed 10.79s; client: status=done elapsed_ms=10783 packets_received=7 acked_ts_advance=10656; server: packets_received=39 acked_ts_advance=9335 remote_addr=127.0.0.1:40748; state 5965 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.79s
proxy stats: c2s forwarded=39 dropped=45 reordered=5 s2c forwarded=7 dropped=5 reordered=1
PASS loss30-blackout: converged in 10.79s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=246

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
