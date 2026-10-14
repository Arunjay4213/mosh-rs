result: failure
commit: 760cd66b622e3cd0aa984aff5343770c382eaffe
branch: laptop/controlmaster
pr: 50
subject: STATUS: ControlMaster test results on devbox
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38593091336
fmt: success
clippy: success
test: failure

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.93s
```

## log-fmt: failures
```
```

## log-fmt (last 80 lines)
```
```

## log-test-client-ubuntu-24.04-arm: failures
```
```

## log-test-client-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 20.38s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.80s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-3b14c01bd37ee6e5)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
test killed_server_shows_last_contact ... ok
test no_server_times_out ... ok
test idle_cpu has been running for over 60 seconds
test idle_cpu ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.74s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.98s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-ac57dd13212b5c6e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test keystroke_shows_at_once_with_prediction ... ok
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed has been running for over 60 seconds
test wrong_prediction_is_removed ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.64s


shard client: all passed
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 31.58s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.32s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-b972b3dc43fc6b4f)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
test killed_server_shows_last_contact ... ok
test no_server_times_out ... ok
test idle_cpu has been running for over 60 seconds
test idle_cpu ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 62.46s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.60s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-5b963b9efd33ce5e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test keystroke_shows_at_once_with_prediction ... ok
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed has been running for over 60 seconds
test wrong_prediction_is_removed ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.71s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
test control_master_hint_and_remote_mode_reuses_socket ... FAILED
test options_and_messages_match_mosh_pl ... ok

failures:

--
thread 'control_master_hint_and_remote_mode_reuses_socket' (20302) panicked at tests/tests/launcher.rs:795:5:
no ControlMaster hint in default mode:
/home/runner/work/mosh-rs/mosh-rs/target/release/mosh: ssh has a ControlMaster s
ocket for localhost, but the default address discovery opens a new connection (s
sh -S none); use --experimental-remote-ip=remote to reuse the socket.
--
test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.94s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.09s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.46s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-c51facc5d19698ed)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.08s
     Running tests/launcher.rs (target/release/deps/launcher-c8c2a09a5034eb5f)

running 9 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test control_master_hint_and_remote_mode_reuses_socket ... FAILED
test options_and_messages_match_mosh_pl ... ok

failures:

---- control_master_hint_and_remote_mode_reuses_socket stdout ----
default mode final screen:
/home/runner/work/mosh-rs/mosh-rs/target/release/mosh: ssh has a ControlMaster s
ocket for localhost, but the default address discovery opens a new connection (s
sh -S none); use --experimental-remote-ip=remote to reuse the socket.


[mosh is exiting.]
MOSH_EXIT=0

thread 'control_master_hint_and_remote_mode_reuses_socket' (20302) panicked at tests/tests/launcher.rs:795:5:
no ControlMaster hint in default mode:
/home/runner/work/mosh-rs/mosh-rs/target/release/mosh: ssh has a ControlMaster s
ocket for localhost, but the default address discovery opens a new connection (s
sh -S none); use --experimental-remote-ip=remote to reuse the socket.


[mosh is exiting.]
MOSH_EXIT=0


















note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    control_master_hint_and_remote_mode_reuses_socket

test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.94s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-conformance-ubuntu-24.04: failures
```
test control_master_hint_and_remote_mode_reuses_socket ... FAILED
test options_and_messages_match_mosh_pl ... ok

failures:

--
thread 'control_master_hint_and_remote_mode_reuses_socket' (20664) panicked at tests/tests/launcher.rs:795:5:
no ControlMaster hint in default mode:
/home/runner/work/mosh-rs/mosh-rs/target/release/mosh: ssh has a ControlMaster s
ocket for localhost, but the default address discovery opens a new connection (s
sh -S none); use --experimental-remote-ip=remote to reuse the socket.
--
test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.21s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.17s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.15s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-ae583a58305a3ead)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.82s
     Running tests/launcher.rs (target/release/deps/launcher-a1cb4ceec230efd4)

running 9 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test control_master_hint_and_remote_mode_reuses_socket ... FAILED
test options_and_messages_match_mosh_pl ... ok

failures:

---- control_master_hint_and_remote_mode_reuses_socket stdout ----
default mode final screen:
/home/runner/work/mosh-rs/mosh-rs/target/release/mosh: ssh has a ControlMaster s
ocket for localhost, but the default address discovery opens a new connection (s
sh -S none); use --experimental-remote-ip=remote to reuse the socket.


[mosh is exiting.]
MOSH_EXIT=0

thread 'control_master_hint_and_remote_mode_reuses_socket' (20664) panicked at tests/tests/launcher.rs:795:5:
no ControlMaster hint in default mode:
/home/runner/work/mosh-rs/mosh-rs/target/release/mosh: ssh has a ControlMaster s
ocket for localhost, but the default address discovery opens a new connection (s
sh -S none); use --experimental-remote-ip=remote to reuse the socket.


[mosh is exiting.]
MOSH_EXIT=0


















note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    control_master_hint_and_remote_mode_reuses_socket

test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.21s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=238

```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.0
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=clean scenario=typing seconds=11.8
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.9
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=4.7
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=10.7
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=18.6
interop: 30 cells, 30 passed, 0 failed, wall time 43s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (110 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=6007 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS clear (1 s): lines=3007 numbers=3000 first=1 last=3000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS altscreen (4 s): lines=6010 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=3 other=0
PASS resize (3 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (2 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (2 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (22 s): lines=2783 numbers=2777 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197223 before #1536'] prompts=0 other=0; notice: ^[[2m[mosh: 197223 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.81s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=33718
Server now attached to client at 127.0.0.1:38323
Server now attached to client at 127.0.0.1:33718
elapsed 10.62s; client: status=done elapsed_ms=10621 packets_received=17 acked_ts_advance=10541; server: packets_received=49 acked_ts_advance=9522 remote_addr=127.0.0.1:33718; state 6159 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.62s
proxy stats: c2s forwarded=49 dropped=14 reordered=4 s2c forwarded=17 dropped=7 reordered=2
PASS loss30-rebind: converged in 10.62s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:60946
elapsed 10.34s; client: status=done elapsed_ms=10341 packets_received=42 acked_ts_advance=10106; server: packets_received=71 acked_ts_advance=9494 remote_addr=127.0.0.1:60946; state 6221 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.34s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=42 dropped=1 reordered=2
PASS loss5: converged in 10.34s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:56026
elapsed 10.61s; client: status=done elapsed_ms=10609 packets_received=7 acked_ts_advance=10575; server: packets_received=37 acked_ts_advance=9439 remote_addr=127.0.0.1:56026; state 6712 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.61s
proxy stats: c2s forwarded=37 dropped=47 reordered=6 s2c forwarded=7 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.61s, 1 client address(es)
transport-impaired: all 3 runs passed
== tests/upstream-check.sh
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

shard scrollback-sh: all passed
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=242

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.7
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=15.3
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.9
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (137 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=6007 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS clear (1 s): lines=3007 numbers=3000 first=1 last=3000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS altscreen (4 s): lines=6010 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=3 other=0
PASS resize (3 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (2 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (2 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (22 s): lines=2927 numbers=2921 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197079 before #1673'] prompts=0 other=0; notice: ^[[2m[mosh: 197079 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.41s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=60991
Server now attached to client at 127.0.0.1:47149
Server now attached to client at 127.0.0.1:60991
elapsed 10.99s; client: status=done elapsed_ms=10984 packets_received=17 acked_ts_advance=10925; server: packets_received=48 acked_ts_advance=9822 remote_addr=127.0.0.1:60991; state 6159 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.99s
proxy stats: c2s forwarded=48 dropped=17 reordered=5 s2c forwarded=17 dropped=4 reordered=1
PASS loss30-rebind: converged in 10.99s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:47171
elapsed 10.80s; client: status=done elapsed_ms=10798 packets_received=46 acked_ts_advance=10709; server: packets_received=76 acked_ts_advance=10079 remote_addr=127.0.0.1:47171; state 5838 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.80s
proxy stats: c2s forwarded=76 dropped=5 reordered=8 s2c forwarded=46 dropped=5 reordered=2
PASS loss5: converged in 10.80s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:53641
elapsed 10.84s; client: status=done elapsed_ms=10832 packets_received=8 acked_ts_advance=10814; server: packets_received=35 acked_ts_advance=9607 remote_addr=127.0.0.1:53641; state 6417 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.84s
proxy stats: c2s forwarded=35 dropped=52 reordered=5 s2c forwarded=8 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.84s, 1 client address(es)
transport-impaired: all 3 runs passed
== tests/upstream-check.sh
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=242

shard scrollback-sh: all passed
```

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 15.98s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.98s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-69dd234075438fc1)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.86s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.66s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-219dcd6c3e572b42)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.74s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.37s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-83adf5d4726fd392)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=129 dropped=115 reordered=0
test live::rust_server_blackout_10s ... ok
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
test live::cxx_server_ignores_capability ... ok
test partial_scroll_region_is_not_captured ... ok
test old_peer_diffs_unchanged ... ok
test seq_50000_random_snapshots ... ok
test seq_with_resizes ... ok
test small_backlog_drops_oldest_unpublished ... ok
test rendered_lines_match_upstream_cells ... ok
test live::rust_server_clean ... ok
udp-impair stats: c2s forwarded=17 dropped=5 reordered=0 s2c forwarded=64 dropped=27 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.75s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.85s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-726aa65b474b5b67)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.28s


shard server: all passed
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 41.25s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.73s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-936cf87a3ec1e824)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.12s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.62s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-302f71dd22d1993a)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.64s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 10.54s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-c9e9e20f1be4041c)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=119 reordered=0
test live::rust_server_blackout_10s ... ok
test live::cxx_server_ignores_capability ... ok
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
test partial_scroll_region_is_not_captured ... ok
test old_peer_diffs_unchanged ... ok
test seq_50000_random_snapshots ... ok
test seq_with_resizes ... ok
test small_backlog_drops_oldest_unpublished ... ok
test live::rust_server_clean ... ok
test rendered_lines_match_upstream_cells ... ok
udp-impair stats: c2s forwarded=17 dropped=5 reordered=0 s2c forwarded=64 dropped=27 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.78s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.46s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-33aa1bab0c62d2a0)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.28s


shard server: all passed
```
