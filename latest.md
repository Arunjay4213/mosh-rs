result: failure
commit: 045942bf7367fec9262a6306d04a994fb52ed36a
subject: STATUS: scrollback client half verified; vim and clear gaps being closed
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38401029264
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.48s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=5.7
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.5
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.7
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.8
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.4
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=12.1
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.5
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== scrollback.sh
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (171 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=5984 numbers=5977 first=1 last=6000 in_order=True duplicates=0 gaps=2978-3000 notices=none prompts=0 other=0
PASS clear (1 s): lines=2984 numbers=2977 first=1 last=3000 in_order=True duplicates=0 gaps=78-100 notices=none prompts=0 other=0
PASS resize (3 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (5 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (5 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (10 s): lines=2927 numbers=2921 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197079 before #1673'] prompts=0 other=0; notice: ^[[2m[mosh: 197079 lines of history were dropped]^[[0m
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.02s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=56128
Server now attached to client at 127.0.0.1:56655
Server now attached to client at 127.0.0.1:56128
elapsed 10.43s; client: status=done elapsed_ms=10430 packets_received=13 acked_ts_advance=10345; server: packets_received=48 acked_ts_advance=9412 remote_addr=127.0.0.1:56128; state 6098 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.43s
proxy stats: c2s forwarded=48 dropped=15 reordered=6 s2c forwarded=13 dropped=5 reordered=0
PASS loss30-rebind: converged in 10.43s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:36469
elapsed 10.41s; client: status=done elapsed_ms=10405 packets_received=40 acked_ts_advance=10064; server: packets_received=71 acked_ts_advance=9507 remote_addr=127.0.0.1:36469; state 6106 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=40 dropped=1 reordered=2
PASS loss5: converged in 10.41s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:47261
elapsed 10.74s; client: status=done elapsed_ms=10738 packets_received=7 acked_ts_advance=10599; server: packets_received=39 acked_ts_advance=9326 remote_addr=127.0.0.1:47261; state 6094 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.74s
proxy stats: c2s forwarded=39 dropped=44 reordered=5 s2c forwarded=7 dropped=6 reordered=1
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=238

run-all: cargo test plus 5 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```

## log-test-ubuntu-latest (last 80 lines)
```
PASS client=cxx server=cxx profile=loss5 scenario=vim seconds=2.0
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=5.9
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=7.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.8
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=14.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.7
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.0
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=2.2
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.8
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.7
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.9
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=12.8
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.5
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== scrollback.sh
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (154 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=5984 numbers=5977 first=1 last=6000 in_order=True duplicates=0 gaps=2978-3000 notices=none prompts=0 other=0
PASS clear (1 s): lines=2984 numbers=2977 first=1 last=3000 in_order=True duplicates=0 gaps=78-100 notices=none prompts=0 other=0
PASS resize (3 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (5 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (5 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (10 s): lines=2927 numbers=2921 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197079 before #1673'] prompts=0 other=0; notice: ^[[2m[mosh: 197079 lines of history were dropped]^[[0m
== transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
    Finished `release` profile [optimized] target(s) in 0.03s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=39420
Server now attached to client at 127.0.0.1:60811
Server now attached to client at 127.0.0.1:39420
elapsed 10.42s; client: status=done elapsed_ms=10420 packets_received=13 acked_ts_advance=10334; server: packets_received=48 acked_ts_advance=9409 remote_addr=127.0.0.1:39420; state 6331 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=48 dropped=15 reordered=6 s2c forwarded=13 dropped=5 reordered=0
PASS loss30-rebind: converged in 10.42s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:58069
elapsed 10.44s; client: status=done elapsed_ms=10438 packets_received=44 acked_ts_advance=10067; server: packets_received=71 acked_ts_advance=9911 remote_addr=127.0.0.1:58069; state 6232 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.44s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.44s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:56267
elapsed 10.32s; client: status=done elapsed_ms=10321 packets_received=13 acked_ts_advance=9977; server: packets_received=40 acked_ts_advance=9748 remote_addr=127.0.0.1:56267; state 6240 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.33s
proxy stats: c2s forwarded=40 dropped=43 reordered=4 s2c forwarded=13 dropped=7 reordered=2
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=246

run-all: cargo test plus 5 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast
```
