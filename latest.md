result: failure
commit: e30095d5b8623e29dd96a846d923ec934a1d72bf
subject: PLAN 9.6 colors and 9.11 clean client exits; NEEDS THE USER: overlay review, updated ci/terminfo
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38367788535
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.15s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
    `-p mosh-tests --test terminfo_table`
== interop.sh
interop: clients=(cxx) servers=(cxx) profiles=(blackout clean loss5 loss30 rebind) scenarios=(seq colors vim wide resize typing) blackout_ms=10000 jobs=10
PASS client=cxx server=cxx profile=clean scenario=seq seconds=0.7
PASS client=cxx server=cxx profile=clean scenario=wide seconds=0.8
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.0
PASS client=cxx server=cxx profile=clean scenario=vim seconds=2.2
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.7
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.6
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=3.9
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=4.6
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=5.5
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.7
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=13.8
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=0.9
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=7.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=1.9
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.1
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=7.1
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=16.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=9.6
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.7
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=21.7
interop: 30 cells, 30 passed, 0 failed, wall time 46s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=35203
Server now attached to client at 127.0.0.1:50966
Server now attached to client at 127.0.0.1:35203
elapsed 10.97s; client: status=done elapsed_ms=10965 packets_received=17 acked_ts_advance=10909; server: packets_received=48 acked_ts_advance=9800 remote_addr=127.0.0.1:35203; state 5914 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.97s
proxy stats: c2s forwarded=48 dropped=17 reordered=5 s2c forwarded=17 dropped=4 reordered=1
PASS loss30-rebind: converged in 10.97s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:41540
elapsed 10.44s; client: status=done elapsed_ms=10434 packets_received=43 acked_ts_advance=10053; server: packets_received=71 acked_ts_advance=9903 remote_addr=127.0.0.1:41540; state 6162 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.44s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.44s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:56378
elapsed 11.06s; client: status=done elapsed_ms=11060 packets_received=13 acked_ts_advance=10983; server: packets_received=38 acked_ts_advance=10091 remote_addr=127.0.0.1:56378; state 6042 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.06s
proxy stats: c2s forwarded=38 dropped=43 reordered=6 s2c forwarded=13 dropped=7 reordered=0
PASS loss30-blackout: converged in 11.06s, 1 client address(es)
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
PASS client=cxx server=cxx profile=clean scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=clean scenario=vim seconds=1.7
PASS client=cxx server=cxx profile=loss5 scenario=seq seconds=0.6
PASS client=cxx server=cxx profile=clean scenario=resize seconds=1.2
PASS client=cxx server=cxx profile=loss5 scenario=wide seconds=0.6
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.0
PASS client=cxx server=cxx profile=loss5 scenario=colors seconds=4.6
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=4.7
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.7
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=13.8
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=7.5
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=0.9
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.6
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.5
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=6.8
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=7.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=8.9
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=14.9
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.3
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=16.3
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.04s
    Finished `release` profile [optimized] target(s) in 0.06s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=36688
Server now attached to client at 127.0.0.1:59521
Server now attached to client at 127.0.0.1:36688
elapsed 10.44s; client: status=done elapsed_ms=10432 packets_received=16 acked_ts_advance=10353; server: packets_received=50 acked_ts_advance=9444 remote_addr=127.0.0.1:36688; state 6159 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.44s
proxy stats: c2s forwarded=50 dropped=14 reordered=4 s2c forwarded=16 dropped=7 reordered=2
PASS loss30-rebind: converged in 10.44s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:35463
elapsed 10.42s; client: status=done elapsed_ms=10415 packets_received=43 acked_ts_advance=10035; server: packets_received=71 acked_ts_advance=9884 remote_addr=127.0.0.1:35463; state 6253 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:33770
elapsed 10.73s; client: status=done elapsed_ms=10731 packets_received=7 acked_ts_advance=10591; server: packets_received=39 acked_ts_advance=9319 remote_addr=127.0.0.1:33770; state 6073 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.73s
proxy stats: c2s forwarded=39 dropped=46 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.73s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=234

run-all: cargo test plus 3 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
