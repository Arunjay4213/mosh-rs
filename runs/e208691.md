result: success
commit: e2086913610b82cfe42a9aa1ebd69e7f892cbfc9
branch: agent/macos-client-e2e
pr: 35
subject: tests: find the real mosh 1.4.0 on PATH (apt or Homebrew) with a skip message; client_e2e runs on macOS; CHANGELOG 0.1.1 unreleased
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38463836528
fmt: success
clippy: success
test: success

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.57s
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
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 18.43s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.36s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-05d18a1cd9e42bce)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.64s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.66s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-d1eec2fcc152967c)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test wrong_prediction_is_removed ... ok
test unicode_input_with_prediction ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 45.66s


shard client: all passed
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 24.50s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.73s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-8809f317143d72dc)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.93s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.34s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-07e09746500697f0)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test wrong_prediction_is_removed ... ok
test unicode_input_with_prediction ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 46.17s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
Server now attached to client at 127.0.0.1:60665
Server now attached to client at 127.0.0.1:54698
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.00s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-4fa039f18a2cf51b)

running 5 tests
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:40067
test malformed_fragments_are_errors ... ok
test malformed_packets_through_transport ... ok
test fragment_round_trip ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.59s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-3c3a06516293d34e)

running 1 test
Server now attached to client at 127.0.0.1:34785
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.00s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.43s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-4c5fcb5bf3694f3b)

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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.56s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-3a3d6d297de6e48e)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.19s
     Running tests/launcher.rs (target/release/deps/launcher-67bcc43b38dbb2a5)

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

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.36s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
Server now attached to client at 127.0.0.1:52280
Server now attached to client at 127.0.0.1:51681
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.64s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.14s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-4077e28d974d70cd)

running 5 tests
test instruction_wire_format ... ok
test malformed_fragments_are_errors ... ok
test fragment_round_trip ... ok
Server now attached to client at 127.0.0.1:43130
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.59s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-feeb43ceaccfcc46)

running 1 test
Server now attached to client at 127.0.0.1:56567
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.07s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.34s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-d4305531de144f13)

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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.46s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-6d5fadaabc1bc6ee)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.23s
     Running tests/launcher.rs (target/release/deps/launcher-6f8b902763393331)

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

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.03s


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
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=11.9
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.0
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=5.1
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=14.5
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.7
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=19.2
interop: 30 cells, 30 passed, 0 failed, wall time 44s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (148 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.20s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=45768
Server now attached to client at 127.0.0.1:44881
Server now attached to client at 127.0.0.1:45768
elapsed 10.63s; client: status=done elapsed_ms=10628 packets_received=15 acked_ts_advance=10578; server: packets_received=50 acked_ts_advance=9702 remote_addr=127.0.0.1:45768; state 6223 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.63s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.63s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:41760
elapsed 10.35s; client: status=done elapsed_ms=10345 packets_received=43 acked_ts_advance=9993; server: packets_received=71 acked_ts_advance=9387 remote_addr=127.0.0.1:41760; state 5848 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:43045
elapsed 10.93s; client: status=done elapsed_ms=10925 packets_received=8 acked_ts_advance=10878; server: packets_received=34 acked_ts_advance=9816 remote_addr=127.0.0.1:43045; state 5892 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.93s
proxy stats: c2s forwarded=34 dropped=48 reordered=5 s2c forwarded=8 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.93s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=238

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.7
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.2
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.7
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=10.0
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=16.9
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.5
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (149 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.75s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=46865
Server now attached to client at 127.0.0.1:54013
Server now attached to client at 127.0.0.1:46865
elapsed 13.36s; client: status=done elapsed_ms=13359 packets_received=21 acked_ts_advance=13320; server: packets_received=67 acked_ts_advance=9812 remote_addr=127.0.0.1:46865; state 6580 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.36s
proxy stats: c2s forwarded=67 dropped=21 reordered=6 s2c forwarded=21 dropped=9 reordered=2
PASS loss30-rebind: converged in 13.36s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:33596
elapsed 10.37s; client: status=done elapsed_ms=10369 packets_received=42 acked_ts_advance=10027; server: packets_received=71 acked_ts_advance=9868 remote_addr=127.0.0.1:33596; state 5906 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.37s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=42 dropped=1 reordered=2
PASS loss5: converged in 10.37s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:33427
elapsed 10.44s; client: status=done elapsed_ms=10442 packets_received=9 acked_ts_advance=10398; server: packets_received=35 acked_ts_advance=9402 remote_addr=127.0.0.1:33427; state 5855 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.44s
proxy stats: c2s forwarded=35 dropped=45 reordered=5 s2c forwarded=9 dropped=4 reordered=1
PASS loss30-blackout: converged in 10.44s, 1 client address(es)
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

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 20.82s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.09s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-9b65167a4cb2eab7)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.87s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.81s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-66662286f7fb856d)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.57s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.28s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-bd7c965b4cfb333e)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=129 dropped=116 reordered=0
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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.50s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-368ec471dc708a6e)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.20s


shard server: all passed
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 28.72s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.56s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-5942c57d8ea2b063)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.93s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.16s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-13d03daf73568ac2)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.61s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.66s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-017f21015f2f3ace)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=130 dropped=118 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.78s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.13s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-4c0f44b763adf657)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.29s


shard server: all passed
```
