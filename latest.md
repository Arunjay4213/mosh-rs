result: failure
commit: ab74863b8353371e42b0bdd09be769149c7b109a
branch: laptop/windows-verified
pr: 44
subject: README: Windows sleep/wake, idle CPU and Ctrl-^ Ctrl-Z results; INTERVENTIONS entry
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38490684319
fmt: success
clippy: success
test: failure

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.69s
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
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 15.97s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.43s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-07158bb8782756dc)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.78s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.00s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-46a496b5b896b654)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test wrong_prediction_is_removed ... ok
test unicode_input_with_prediction ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 45.76s


shard client: all passed
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 20.57s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.11s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-9a2f539aa70c64aa)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.67s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.58s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-ba811c67a0c3c436)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test wrong_prediction_is_removed ... ok
test unicode_input_with_prediction ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 45.72s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
Server now attached to client at 127.0.0.1:51382
Server now attached to client at 127.0.0.1:55273
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.95s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.51s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-125a41875214585d)

running 5 tests
test fragment_round_trip ... ok
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:60016
test malformed_fragments_are_errors ... ok
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.24s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-2ac5a2a734e431d1)

running 1 test
Server now attached to client at 127.0.0.1:51114
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.04s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.03s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-8978fb5fa520a07a)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test rebind_changes_source_port ... ok
test loss_half_each_direction ... ok
test stats_and_sigterm ... ok
test delay_applies_both_directions ... ok
test reorder_delivers_out_of_sequence ... ok
test blackout_silences_then_resumes ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.44s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.14s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-2c3ee859dbf1619d)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.90s
     Running tests/launcher.rs (target/release/deps/launcher-cde95aa01a366849)

running 8 tests
test launches_session_on_dual_stack_name ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test missing_server_is_reported ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_with_user_at_ip_literal ... ok
test launches_session_on_localhost_with_command ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.20s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
test userstream_roundtrip ... ok
Server now attached to client at 127.0.0.1:43696
Server now attached to client at 127.0.0.1:45461
test malformed_diffs_through_transport ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.79s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.46s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-b238f7e5f5b36e4b)

running 5 tests
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:33576
test malformed_fragments_are_errors ... ok
test malformed_packets_through_transport ... ok
test fragment_round_trip ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.06s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-b7b5750f50087f0f)

running 1 test
Server now attached to client at 127.0.0.1:37069
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.07s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.03s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-9ad605cbe79a65c7)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test rebind_changes_source_port ... ok
test loss_half_each_direction ... ok
test stats_and_sigterm ... ok
test delay_applies_both_directions ... ok
test reorder_delivers_out_of_sequence ... ok
test blackout_silences_then_resumes ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.44s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.18s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-c355d62a37c99fed)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.83s
     Running tests/launcher.rs (target/release/deps/launcher-563bbc404d844ae8)

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

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.40s


shard conformance: all passed
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=239

```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=3.9
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.5
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=5.5
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.8
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.7
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (164 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.01s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=36132
Server now attached to client at 127.0.0.1:57051
Server now attached to client at 127.0.0.1:36132
elapsed 10.63s; client: status=done elapsed_ms=10632 packets_received=15 acked_ts_advance=10583; server: packets_received=50 acked_ts_advance=9705 remote_addr=127.0.0.1:36132; state 5988 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.64s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.63s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:41641
elapsed 10.36s; client: status=done elapsed_ms=10353 packets_received=41 acked_ts_advance=10121; server: packets_received=72 acked_ts_advance=9510 remote_addr=127.0.0.1:41641; state 6367 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.36s
proxy stats: c2s forwarded=72 dropped=7 reordered=7 s2c forwarded=41 dropped=1 reordered=2
PASS loss5: converged in 10.36s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:43763
elapsed 10.81s; client: status=done elapsed_ms=10804 packets_received=9 acked_ts_advance=10781; server: packets_received=34 acked_ts_advance=9588 remote_addr=127.0.0.1:43763; state 6203 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.81s
proxy stats: c2s forwarded=34 dropped=54 reordered=5 s2c forwarded=9 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.81s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=239

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
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=3.8
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.5
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.4
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=6.0
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.1
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.2
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.5
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (179 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS vim (2 s): lines=6007 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS clear (1 s): lines=3007 numbers=3000 first=1 last=3000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS altscreen (4 s): lines=6010 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=3 other=0
PASS resize (4 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (2 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (2 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (22 s): lines=2783 numbers=2777 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197223 before #1536'] prompts=0 other=0; notice: ^[[2m[mosh: 197223 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.01s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=39365
Server now attached to client at 127.0.0.1:39114
Server now attached to client at 127.0.0.1:39365
elapsed 10.40s; client: status=done elapsed_ms=10399 packets_received=19 acked_ts_advance=10010; server: packets_received=50 acked_ts_advance=9832 remote_addr=127.0.0.1:39365; state 6060 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=50 dropped=12 reordered=5 s2c forwarded=19 dropped=9 reordered=1
PASS loss30-rebind: converged in 10.40s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:54348
elapsed 10.34s; client: status=done elapsed_ms=10337 packets_received=43 acked_ts_advance=9985; server: packets_received=71 acked_ts_advance=9379 remote_addr=127.0.0.1:54348; state 6263 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.34s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.34s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:43190
elapsed 10.78s; client: status=done elapsed_ms=10781 packets_received=7 acked_ts_advance=10644; server: packets_received=39 acked_ts_advance=9364 remote_addr=127.0.0.1:43190; state 6339 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.78s
proxy stats: c2s forwarded=39 dropped=47 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.78s, 1 client address(es)
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
   Compiling mosh-platform v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 17.84s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.07s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-7626b7b0039291b8)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.88s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.87s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-626ce7029d02d391)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.58s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.58s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-1667738b282ea0cc)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=128 dropped=108 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.72s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.82s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-70e1b6f36796fdf1)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.30s


shard server: all passed
```

## log-test-server-ubuntu-24.04: failures
```
test command_output_and_exit ... FAILED

failures:

---- command_output_and_exit stdout ----
--
thread 'command_output_and_exit' (3241) panicked at tests/tests/server_e2e.rs:133:13:
timed out waiting for line "SERVER_OK"; screen:


[mosh is exiting.]
--
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 33.63s

error: test failed, to rerun pass `-p mosh-tests --test server_e2e`
== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.32s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-568ca713f7af071d)
--
shard server: FAILED: cargo test --release -p mosh-tests --test server_e2e
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```














note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    command_output_and_exit

test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 33.63s

error: test failed, to rerun pass `-p mosh-tests --test server_e2e`
== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.32s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-568ca713f7af071d)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.55s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.95s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-ba2cf48a6dedc9ee)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=128 dropped=125 reordered=0
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
test rendered_lines_match_upstream_cells ... ok
test seq_50000_random_snapshots ... ok
test seq_with_resizes ... ok
test small_backlog_drops_oldest_unpublished ... ok
test live::rust_server_clean ... ok
udp-impair stats: c2s forwarded=17 dropped=5 reordered=0 s2c forwarded=64 dropped=27 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.75s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.1 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.17s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-1eb049e1c661fabc)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.18s


shard server: FAILED: cargo test --release -p mosh-tests --test server_e2e
```
