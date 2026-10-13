result: failure
commit: c5102574fa720d953ab92511f44004121c4b1a4d
subject: Spawn agent P: bench tooling and static musl build, started early
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38378437984
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.47s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.0
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.2
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.5
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.6
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=9.9
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.9
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.8
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=37659
Server now attached to client at 127.0.0.1:34819
Server now attached to client at 127.0.0.1:37659
elapsed 10.98s; client: status=done elapsed_ms=10978 packets_received=17 acked_ts_advance=10922; server: packets_received=48 acked_ts_advance=9813 remote_addr=127.0.0.1:37659; state 5819 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.98s
proxy stats: c2s forwarded=48 dropped=17 reordered=5 s2c forwarded=17 dropped=4 reordered=1
PASS loss30-rebind: converged in 10.98s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:33927
elapsed 10.35s; client: status=done elapsed_ms=10349 packets_received=41 acked_ts_advance=10116; server: packets_received=72 acked_ts_advance=9505 remote_addr=127.0.0.1:33927; state 6113 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=72 dropped=7 reordered=7 s2c forwarded=41 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:47553
elapsed 10.79s; client: status=done elapsed_ms=10784 packets_received=7 acked_ts_advance=10656; server: packets_received=39 acked_ts_advance=9334 remote_addr=127.0.0.1:47553; state 5925 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.79s
proxy stats: c2s forwarded=39 dropped=47 reordered=5 s2c forwarded=7 dropped=5 reordered=1
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=238

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.5
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.6
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.8
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=5.6
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.7
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.8
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=14.7
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.6
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.8
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.0
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=16.5
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.9
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.3
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=50108
Server now attached to client at 127.0.0.1:37966
Server now attached to client at 127.0.0.1:50108
elapsed 12.29s; client: status=done elapsed_ms=12284 packets_received=21 acked_ts_advance=11837; server: packets_received=51 acked_ts_advance=10091 remote_addr=127.0.0.1:50108; state 5884 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.29s
proxy stats: c2s forwarded=51 dropped=18 reordered=4 s2c forwarded=21 dropped=7 reordered=3
PASS loss30-rebind: converged in 12.29s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:60758
elapsed 10.42s; client: status=done elapsed_ms=10417 packets_received=43 acked_ts_advance=10037; server: packets_received=71 acked_ts_advance=9887 remote_addr=127.0.0.1:60758; state 6079 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:58517
elapsed 11.07s; client: status=done elapsed_ms=11063 packets_received=13 acked_ts_advance=10985; server: packets_received=38 acked_ts_advance=10093 remote_addr=127.0.0.1:58517; state 6044 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.07s
proxy stats: c2s forwarded=38 dropped=40 reordered=6 s2c forwarded=13 dropped=7 reordered=0
PASS loss30-blackout: converged in 11.07s, 1 client address(es)
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
