result: failure
commit: afa5b47d013b4a97f16a9b26ca9a3750a546c6f6
subject: STATUS: spawn agent F for the Phase 2 cleanup cluster
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38371963257
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 12.99s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.1
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.5
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.0
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.7
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=8.6
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=16.0
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.3
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=57555
Server now attached to client at 127.0.0.1:49329
Server now attached to client at 127.0.0.1:57555
elapsed 10.64s; client: status=done elapsed_ms=10642 packets_received=14 acked_ts_advance=10594; server: packets_received=51 acked_ts_advance=9711 remote_addr=127.0.0.1:57555; state 6188 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.64s
proxy stats: c2s forwarded=51 dropped=15 reordered=5 s2c forwarded=14 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.64s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:46344
elapsed 10.34s; client: status=done elapsed_ms=10334 packets_received=43 acked_ts_advance=9983; server: packets_received=71 acked_ts_advance=9377 remote_addr=127.0.0.1:46344; state 5884 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.34s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.34s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:52978
elapsed 10.78s; client: status=done elapsed_ms=10781 packets_received=7 acked_ts_advance=10644; server: packets_received=39 acked_ts_advance=9364 remote_addr=127.0.0.1:52978; state 6270 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.78s
proxy stats: c2s forwarded=39 dropped=49 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.78s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=237

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.7
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.3
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.7
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.4
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.6
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.2
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.7
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.3
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=47685
Server now attached to client at 127.0.0.1:60523
Server now attached to client at 127.0.0.1:47685
elapsed 10.93s; client: status=done elapsed_ms=10925 packets_received=16 acked_ts_advance=10855; server: packets_received=47 acked_ts_advance=9928 remote_addr=127.0.0.1:47685; state 6160 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.93s
proxy stats: c2s forwarded=47 dropped=15 reordered=4 s2c forwarded=16 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.93s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:46767
elapsed 10.40s; client: status=done elapsed_ms=10401 packets_received=43 acked_ts_advance=10021; server: packets_received=71 acked_ts_advance=9871 remote_addr=127.0.0.1:46767; state 6261 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.40s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:56462
elapsed 10.46s; client: status=done elapsed_ms=10453 packets_received=11 acked_ts_advance=10058; server: packets_received=38 acked_ts_advance=9808 remote_addr=127.0.0.1:56462; state 6347 bytes
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
