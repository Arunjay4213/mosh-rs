result: failure
commit: 111139f2d5cee959dcfb79875a3851f0a5139c80
subject: tools/bench: PLAN 5 performance bench against mosh 1.4.0; scripts/build-release.sh: static musl release build
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38382096969
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.26s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
interop-setup-loss: not run (opt-in: INTEROP_FULL=1 or --run)
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.8
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.2
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=4.8
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.4
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.8
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.7
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.5
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.2
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=36323
Server now attached to client at 127.0.0.1:38757
Server now attached to client at 127.0.0.1:36323
elapsed 10.59s; client: status=done elapsed_ms=10583 packets_received=15 acked_ts_advance=10505; server: packets_received=51 acked_ts_advance=9487 remote_addr=127.0.0.1:36323; state 6602 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.59s
proxy stats: c2s forwarded=51 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.59s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:34061
elapsed 10.37s; client: status=done elapsed_ms=10371 packets_received=40 acked_ts_advance=10030; server: packets_received=71 acked_ts_advance=9480 remote_addr=127.0.0.1:34061; state 5810 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.37s
proxy stats: c2s forwarded=71 dropped=7 reordered=8 s2c forwarded=40 dropped=1 reordered=1
PASS loss5: converged in 10.37s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:59438
elapsed 19.46s; client: status=done elapsed_ms=19457 packets_received=6 acked_ts_advance=19234; server: packets_received=86 acked_ts_advance=9178 remote_addr=127.0.0.1:59438; state 6399 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.46s
proxy stats: c2s forwarded=86 dropped=77 reordered=9 s2c forwarded=6 dropped=12 reordered=2
PASS loss30-blackout: converged in 19.46s, 1 client address(es)
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
PASS client=cxx server=cxx profile=clean scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=clean scenario=colors seconds=2.6
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=2.1
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.8
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=3.8
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.2
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.4
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=2.2
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.8
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.8
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.9
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.8
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=9.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.1
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=33871
Server now attached to client at 127.0.0.1:39446
Server now attached to client at 127.0.0.1:33871
elapsed 10.93s; client: status=done elapsed_ms=10927 packets_received=16 acked_ts_advance=10856; server: packets_received=47 acked_ts_advance=9929 remote_addr=127.0.0.1:33871; state 5675 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.93s
proxy stats: c2s forwarded=47 dropped=15 reordered=4 s2c forwarded=16 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.93s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:48258
elapsed 10.40s; client: status=done elapsed_ms=10398 packets_received=44 acked_ts_advance=10027; server: packets_received=71 acked_ts_advance=9877 remote_addr=127.0.0.1:48258; state 6295 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.40s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:58082
elapsed 10.74s; client: status=done elapsed_ms=10737 packets_received=7 acked_ts_advance=10598; server: packets_received=39 acked_ts_advance=9325 remote_addr=127.0.0.1:58082; state 6107 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.74s
proxy stats: c2s forwarded=39 dropped=46 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.74s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=244

run-all: cargo test plus 4 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
