result: failure
commit: c48a54f89358392acd0ed8b727bdb7822fb004e6
subject: STATUS: client gates verified by the orchestrator
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38368851715
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.37s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
    `-p mosh-tests --test terminfo_table`
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=0.8
PASS client=cxx server=cxx profile=clean scenario=wide seconds=0.8
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=clean scenario=vim seconds=1.8
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.8
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=1.1
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.7
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=4.5
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=3.9
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=1.7
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=2.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.3
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=13.8
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.1
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=3.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=8.0
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=4.1
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=4.7
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=20.3
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=15.4
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.02s
    Finished `release` profile [optimized] target(s) in 0.05s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=55215
Server now attached to client at 127.0.0.1:36724
Server now attached to client at 127.0.0.1:55215
elapsed 10.60s; client: status=done elapsed_ms=10594 packets_received=15 acked_ts_advance=10544; server: packets_received=50 acked_ts_advance=9668 remote_addr=127.0.0.1:55215; state 6188 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.60s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.60s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:38760
elapsed 10.42s; client: status=done elapsed_ms=10416 packets_received=43 acked_ts_advance=10036; server: packets_received=71 acked_ts_advance=9886 remote_addr=127.0.0.1:38760; state 6138 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:36332
elapsed 10.32s; client: status=done elapsed_ms=10321 packets_received=13 acked_ts_advance=9977; server: packets_received=40 acked_ts_advance=9748 remote_addr=127.0.0.1:36332; state 6574 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.32s
proxy stats: c2s forwarded=40 dropped=42 reordered=4 s2c forwarded=13 dropped=7 reordered=2
PASS loss30-blackout: converged in 10.32s, 1 client address(es)
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

run-all: cargo test plus 3 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
    `-p mosh-tests --test terminfo_table`
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=wide seconds=0.7
PASS client=cxx server=cxx profile=clean scenario=seq seconds=0.8
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=clean scenario=vim seconds=1.8
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.0
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.6
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=0.9
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.1
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=4.0
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=1.9
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.2
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.3
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=13.6
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.1
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.0
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.8
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=4.6
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=4.9
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=15.9
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=14.9
interop: 30 cells, 30 passed, 0 failed, wall time 39s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.04s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=51842
Server now attached to client at 127.0.0.1:59438
Server now attached to client at 127.0.0.1:51842
elapsed 10.59s; client: status=done elapsed_ms=10586 packets_received=15 acked_ts_advance=10536; server: packets_received=50 acked_ts_advance=9661 remote_addr=127.0.0.1:51842; state 6619 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.59s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.59s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:59286
elapsed 10.40s; client: status=done elapsed_ms=10399 packets_received=43 acked_ts_advance=10019; server: packets_received=71 acked_ts_advance=9868 remote_addr=127.0.0.1:59286; state 6260 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.40s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:59501
elapsed 10.37s; client: status=done elapsed_ms=10371 packets_received=11 acked_ts_advance=9978; server: packets_received=38 acked_ts_advance=9734 remote_addr=127.0.0.1:59501; state 6157 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.37s
proxy stats: c2s forwarded=38 dropped=47 reordered=5 s2c forwarded=11 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.37s, 1 client address(es)
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

run-all: cargo test plus 3 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
