result: failure
commit: 1cb0eb6ce9903ec2e4e834e81a773b1b33a7afba
subject: INTERVENTIONS: laptop pushed ci/terminfo
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38416493370
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.25s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=7.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=9.1
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.7
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.2
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=9.6
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.4
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.8
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== scrollback.sh
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (155 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
server exited unexpectedly
FAIL vim (0 s): 
PASS clear (1 s): lines=3007 numbers=3000 first=1 last=3000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
FAIL altscreen (31 s): direct run: last screen line is not "$" within 30 s: "q$"
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
control at 5s: rebind -> ok rebind port=53469
Server now attached to client at 127.0.0.1:45888
Server now attached to client at 127.0.0.1:53469
elapsed 10.39s; client: status=done elapsed_ms=10385 packets_received=17 acked_ts_advance=10175; server: packets_received=51 acked_ts_advance=9299 remote_addr=127.0.0.1:53469; state 6107 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.39s
proxy stats: c2s forwarded=51 dropped=14 reordered=6 s2c forwarded=17 dropped=7 reordered=0
PASS loss30-rebind: converged in 10.39s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:47924
elapsed 10.42s; client: status=done elapsed_ms=10413 packets_received=43 acked_ts_advance=10033; server: packets_received=71 acked_ts_advance=9883 remote_addr=127.0.0.1:47924; state 5839 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:47850
elapsed 10.42s; client: status=done elapsed_ms=10419 packets_received=11 acked_ts_advance=10072; server: packets_received=33 acked_ts_advance=9123 remote_addr=127.0.0.1:47850; state 5990 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=33 dropped=48 reordered=5 s2c forwarded=11 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.42s, 1 client address(es)
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

run-all: cargo test plus 5 shell driver(s)
run-all: FAILED: scrollback.sh
```

## log-test-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=6.2
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.8
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.2
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=8.7
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.4
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.7
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=14.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.1
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=15.8
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=13.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.7
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== scrollback.sh
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (147 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=6007 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS clear (1 s): lines=3007 numbers=3000 first=1 last=3000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
FAIL altscreen (31 s): direct run: last screen line is not "$" within 30 s: "q$"
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
control at 5s: rebind -> ok rebind port=58752
Server now attached to client at 127.0.0.1:52757
Server now attached to client at 127.0.0.1:58752
elapsed 25.54s; client: status=done elapsed_ms=25541 packets_received=20 acked_ts_advance=25491; server: packets_received=106 acked_ts_advance=9368 remote_addr=127.0.0.1:58752; state 6024 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 25.54s
proxy stats: c2s forwarded=106 dropped=36 reordered=10 s2c forwarded=20 dropped=14 reordered=3
PASS loss30-rebind: converged in 25.54s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:53717
elapsed 10.42s; client: status=done elapsed_ms=10414 packets_received=43 acked_ts_advance=10034; server: packets_received=71 acked_ts_advance=9884 remote_addr=127.0.0.1:53717; state 6151 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:47223
elapsed 10.74s; client: status=done elapsed_ms=10736 packets_received=7 acked_ts_advance=10597; server: packets_received=39 acked_ts_advance=9324 remote_addr=127.0.0.1:47223; state 6396 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.74s
proxy stats: c2s forwarded=39 dropped=47 reordered=5 s2c forwarded=7 dropped=6 reordered=1
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=246

run-all: cargo test plus 5 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast scrollback.sh
```
