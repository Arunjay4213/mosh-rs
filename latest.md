result: failure
commit: 60a0448e53aeb387a60b0b0d02587dc7f27eae35
subject: STATUS: detached-session warning verified for C++, glibc and musl servers
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38391698240
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.31s
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
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.3
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.8
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=15.3
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.3
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.01s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=52217
Server now attached to client at 127.0.0.1:33307
Server now attached to client at 127.0.0.1:52217
elapsed 10.97s; client: status=done elapsed_ms=10968 packets_received=17 acked_ts_advance=10909; server: packets_received=48 acked_ts_advance=9806 remote_addr=127.0.0.1:52217; state 6106 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.97s
proxy stats: c2s forwarded=48 dropped=17 reordered=5 s2c forwarded=17 dropped=4 reordered=1
PASS loss30-rebind: converged in 10.97s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:40121
elapsed 10.35s; client: status=done elapsed_ms=10344 packets_received=41 acked_ts_advance=10112; server: packets_received=72 acked_ts_advance=9500 remote_addr=127.0.0.1:40121; state 6307 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=72 dropped=7 reordered=7 s2c forwarded=41 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:36996
elapsed 10.46s; client: status=done elapsed_ms=10455 packets_received=11 acked_ts_advance=10059; server: packets_received=38 acked_ts_advance=9809 remote_addr=127.0.0.1:36996; state 6205 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=38 dropped=53 reordered=4 s2c forwarded=11 dropped=6 reordered=2
PASS loss30-blackout: converged in 10.46s, 1 client address(es)
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
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.4
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.5
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.9
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.5
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.6
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.7
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.8
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.9
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=6.0
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.6
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=12.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.4
interop: 30 cells, 30 passed, 0 failed, wall time 43s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.02s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=44155
Server now attached to client at 127.0.0.1:44616
Server now attached to client at 127.0.0.1:44155
elapsed 10.63s; client: status=done elapsed_ms=10624 packets_received=15 acked_ts_advance=10574; server: packets_received=50 acked_ts_advance=9697 remote_addr=127.0.0.1:44155; state 6139 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.63s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.63s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:48894
elapsed 10.38s; client: status=done elapsed_ms=10373 packets_received=42 acked_ts_advance=10031; server: packets_received=71 acked_ts_advance=9872 remote_addr=127.0.0.1:48894; state 5726 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.38s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=42 dropped=1 reordered=2
PASS loss5: converged in 10.38s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:49822
elapsed 11.02s; client: status=done elapsed_ms=11015 packets_received=9 acked_ts_advance=10984; server: packets_received=55 acked_ts_advance=9784 remote_addr=127.0.0.1:49822; state 6701 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.02s
proxy stats: c2s forwarded=55 dropped=57 reordered=5 s2c forwarded=9 dropped=6 reordered=1
PASS loss30-blackout: converged in 11.02s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=243

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
