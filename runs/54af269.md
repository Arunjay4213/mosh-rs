result: success
commit: 54af269c7a3b4ca5834a0eb4023f6aac5a1e6b20
branch: laptop/packaging
pr: 47
subject: README: package manager installs (crates.io, Homebrew, Scoop, winget pending); fix v0.1.1 download links
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38509807630
fmt: success
clippy: success
test: success

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.40s
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
    Finished `release` profile [optimized] target(s) in 19.87s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.43s
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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.66s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.10s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-ac57dd13212b5c6e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test keystroke_shows_at_once_with_prediction ... ok
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed has been running for over 60 seconds
test wrong_prediction_is_removed ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.56s


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
    Finished `release` profile [optimized] target(s) in 21.59s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.95s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-b972b3dc43fc6b4f)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test interactive_shell_and_resize ... ok
test command_output_and_exit ... ok
test killed_server_shows_last_contact ... ok
test no_server_times_out ... ok
test idle_cpu has been running for over 60 seconds
test idle_cpu ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.90s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.36s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-5b963b9efd33ce5e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test keystroke_shows_at_once_with_prediction ... ok
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed has been running for over 60 seconds
test wrong_prediction_is_removed ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.03s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
Server now attached to client at 127.0.0.1:35598
Server now attached to client at 127.0.0.1:60331
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.08s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.72s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-cc62286ad719b2f9)

running 5 tests
test malformed_fragments_are_errors ... ok
test fragment_round_trip ... ok
Server now attached to client at test instruction_wire_format ... ok
127.0.0.1:51703
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.43s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-85c6987c331b6535)

running 1 test
Server now attached to client at 127.0.0.1:54133
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.04s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.52s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-ec52cb020c98762f)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test rebind_changes_source_port ... ok
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test blackout_silences_then_resumes ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.06s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.52s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-c51facc5d19698ed)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.40s
     Running tests/launcher.rs (target/release/deps/launcher-c8c2a09a5034eb5f)

running 8 tests
test launches_session_on_dual_stack_name ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test missing_server_is_reported ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_with_user_at_ip_literal ... ok
test launches_session_on_localhost_with_command ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.35s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
test userstream_roundtrip ... ok
Server now attached to client at 127.0.0.1:47987
Server now attached to client at 127.0.0.1:58203
test malformed_diffs_through_transport ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.73s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.16s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-a34a8193ced1844f)

running 5 tests
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:36732
test malformed_packets_through_transport ... ok
test malformed_fragments_are_errors ... ok
test fragment_round_trip ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.84s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-5e15975671f6f1c7)

running 1 test
Server now attached to client at 127.0.0.1:35768
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.07s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.67s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-6453a427fc1d2bab)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test blackout_silences_then_resumes ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test delay_applies_both_directions ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.17s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.81s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-ae583a58305a3ead)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.48s
     Running tests/launcher.rs (target/release/deps/launcher-a1cb4ceec230efd4)

running 8 tests
test launches_session_on_dual_stack_name ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test missing_server_is_reported ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_with_user_at_ip_literal ... ok
test launches_session_on_localhost_with_command ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.34s


shard conformance: all passed
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=240

```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=4.9
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.5
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.0
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=18.2
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.2
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (111 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
    Finished `release` profile [optimized] target(s) in 5.57s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=43318
Server now attached to client at 127.0.0.1:51613
Server now attached to client at 127.0.0.1:43318
elapsed 10.65s; client: status=done elapsed_ms=10644 packets_received=14 acked_ts_advance=10595; server: packets_received=51 acked_ts_advance=9712 remote_addr=127.0.0.1:43318; state 6217 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.65s
proxy stats: c2s forwarded=51 dropped=15 reordered=5 s2c forwarded=14 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.65s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:49361
elapsed 10.43s; client: status=done elapsed_ms=10424 packets_received=43 acked_ts_advance=10043; server: packets_received=71 acked_ts_advance=9893 remote_addr=127.0.0.1:49361; state 6245 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.43s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.43s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:57795
elapsed 10.74s; client: status=done elapsed_ms=10741 packets_received=7 acked_ts_advance=10601; server: packets_received=39 acked_ts_advance=9329 remote_addr=127.0.0.1:57795; state 6003 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.74s
proxy stats: c2s forwarded=39 dropped=45 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.74s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=240

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
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=8.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.5
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.8
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (126 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
    Finished `release` profile [optimized] target(s) in 6.17s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=58996
Server now attached to client at 127.0.0.1:45219
Server now attached to client at 127.0.0.1:58996
elapsed 10.63s; client: status=done elapsed_ms=10625 packets_received=15 acked_ts_advance=10575; server: packets_received=50 acked_ts_advance=9698 remote_addr=127.0.0.1:58996; state 5986 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.63s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.63s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:48485
elapsed 10.43s; client: status=done elapsed_ms=10429 packets_received=44 acked_ts_advance=10058; server: packets_received=71 acked_ts_advance=9907 remote_addr=127.0.0.1:48485; state 5769 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.43s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.43s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:41257
elapsed 10.40s; client: status=done elapsed_ms=10400 packets_received=10 acked_ts_advance=10384; server: packets_received=43 acked_ts_advance=9464 remote_addr=127.0.0.1:41257; state 6339 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=43 dropped=37 reordered=5 s2c forwarded=10 dropped=8 reordered=1
PASS loss30-blackout: converged in 10.40s, 1 client address(es)
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
    Finished `release` profile [optimized] target(s) in 17.03s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.91s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-69dd234075438fc1)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.78s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.52s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-219dcd6c3e572b42)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.72s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.99s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-83adf5d4726fd392)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=129 dropped=114 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.73s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.51s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-726aa65b474b5b67)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.11s


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
    Finished `release` profile [optimized] target(s) in 19.98s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.09s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-936cf87a3ec1e824)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.82s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.23s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-302f71dd22d1993a)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.59s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.55s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-c9e9e20f1be4041c)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=118 reordered=0
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
    Finished `release` profile [optimized] target(s) in 4.15s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-33aa1bab0c62d2a0)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.16s


shard server: all passed
```
