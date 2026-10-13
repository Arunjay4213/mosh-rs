result: failure
commit: 728e4689d87c5584bf26abb8de99d976009c327d
subject: scrollback: real alternate screen on the server, forwarded to the host terminal by the client
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38408629067
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.59s
```

## log-fmt (last 80 lines)
```
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=5.7
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.6
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.6
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.4
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=15.0
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.8
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== scrollback.sh
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (163 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=6007 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS clear (1 s): lines=3007 numbers=3000 first=1 last=3000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
FAIL altscreen (30 s): direct run: last screen line is not "$" within 30 s: "q$"
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
control at 5s: rebind -> ok rebind port=59625
Server now attached to client at 127.0.0.1:58511
Server now attached to client at 127.0.0.1:59625
elapsed 10.93s; client: status=done elapsed_ms=10927 packets_received=16 acked_ts_advance=10857; server: packets_received=47 acked_ts_advance=9929 remote_addr=127.0.0.1:59625; state 6194 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.93s
proxy stats: c2s forwarded=47 dropped=15 reordered=4 s2c forwarded=16 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.93s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:59361
elapsed 10.35s; client: status=done elapsed_ms=10348 packets_received=41 acked_ts_advance=10115; server: packets_received=72 acked_ts_advance=9504 remote_addr=127.0.0.1:59361; state 6034 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=72 dropped=7 reordered=7 s2c forwarded=41 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:57576
elapsed 10.84s; client: status=done elapsed_ms=10836 packets_received=8 acked_ts_advance=10817; server: packets_received=35 acked_ts_advance=9611 remote_addr=127.0.0.1:57576; state 6167 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.84s
proxy stats: c2s forwarded=35 dropped=51 reordered=5 s2c forwarded=8 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.84s, 1 client address(es)
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

run-all: cargo test plus 5 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast scrollback.sh
```

## log-test-ubuntu-latest (last 80 lines)
```
PASS client=cxx server=cxx profile=loss5 scenario=resize seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=7.0
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=6.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.7
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.7
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.5
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.7
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.9
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.5
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.6
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=13.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.6
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== scrollback.sh
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (154 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
control at 5s: rebind -> ok rebind port=58744
Server now attached to client at 127.0.0.1:57664
Server now attached to client at 127.0.0.1:58744
elapsed 10.95s; client: status=done elapsed_ms=10950 packets_received=18 acked_ts_advance=10891; server: packets_received=47 acked_ts_advance=9788 remote_addr=127.0.0.1:58744; state 6474 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.95s
proxy stats: c2s forwarded=47 dropped=18 reordered=4 s2c forwarded=18 dropped=3 reordered=2
PASS loss30-rebind: converged in 10.95s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:33467
elapsed 10.84s; client: status=done elapsed_ms=10833 packets_received=47 acked_ts_advance=10798; server: packets_received=73 acked_ts_advance=10044 remote_addr=127.0.0.1:33467; state 6440 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.84s
proxy stats: c2s forwarded=73 dropped=7 reordered=7 s2c forwarded=47 dropped=3 reordered=3
PASS loss5: converged in 10.84s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:52816
elapsed 10.47s; client: status=done elapsed_ms=10463 packets_received=9 acked_ts_advance=10263; server: packets_received=36 acked_ts_advance=9270 remote_addr=127.0.0.1:52816; state 6112 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.47s
proxy stats: c2s forwarded=36 dropped=41 reordered=6 s2c forwarded=9 dropped=6 reordered=0
PASS loss30-blackout: converged in 10.47s, 1 client address(es)
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

run-all: cargo test plus 5 shell driver(s)
run-all: FAILED: cargo test --release --workspace --no-fail-fast scrollback.sh
```
