result: failure
commit: 31837bcd0375e18b2e6c53d85fcb12917f08a51e
subject: STATUS: server gates verified by the orchestrator
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38373072245
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.65s
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
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=vim seconds=1.8
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.1
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.1
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.5
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.4
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.5
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.5
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.8
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.2
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=4.7
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.5
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=60182
Server now attached to client at 127.0.0.1:47984
Server now attached to client at 127.0.0.1:60182
elapsed 10.63s; client: status=done elapsed_ms=10623 packets_received=15 acked_ts_advance=10573; server: packets_received=50 acked_ts_advance=9697 remote_addr=127.0.0.1:60182; state 6185 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.63s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.63s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:57452
elapsed 10.41s; client: status=done elapsed_ms=10412 packets_received=44 acked_ts_advance=10040; server: packets_received=71 acked_ts_advance=9884 remote_addr=127.0.0.1:57452; state 5797 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=71 dropped=7 reordered=6 s2c forwarded=44 dropped=1 reordered=3
PASS loss5: converged in 10.41s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:35091
elapsed 10.40s; client: status=done elapsed_ms=10398 packets_received=9 acked_ts_advance=10070; server: packets_received=34 acked_ts_advance=9832 remote_addr=127.0.0.1:35091; state 6171 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=34 dropped=50 reordered=5 s2c forwarded=9 dropped=6 reordered=0
PASS loss30-blackout: converged in 10.40s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=240

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.5
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.6
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=2.0
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=5.0
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.7
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.8
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.7
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=14.9
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.9
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.7
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.2
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=15.0
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.0
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=51363
Server now attached to client at 127.0.0.1:60307
Server now attached to client at 127.0.0.1:51363
elapsed 10.46s; client: status=done elapsed_ms=10455 packets_received=16 acked_ts_advance=10375; server: packets_received=50 acked_ts_advance=9465 remote_addr=127.0.0.1:51363; state 6082 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=50 dropped=14 reordered=4 s2c forwarded=16 dropped=7 reordered=2
PASS loss30-rebind: converged in 10.46s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:40306
elapsed 10.42s; client: status=done elapsed_ms=10415 packets_received=43 acked_ts_advance=10035; server: packets_received=71 acked_ts_advance=9885 remote_addr=127.0.0.1:40306; state 6009 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:47678
elapsed 10.41s; client: status=done elapsed_ms=10402 packets_received=10 acked_ts_advance=10385; server: packets_received=43 acked_ts_advance=9464 remote_addr=127.0.0.1:47678; state 6446 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=43 dropped=35 reordered=5 s2c forwarded=10 dropped=8 reordered=1
PASS loss30-blackout: converged in 10.41s, 1 client address(es)
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
