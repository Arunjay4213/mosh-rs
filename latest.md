result: success
commit: 6f2a510137d13d1a05dd3f3947d3e9badc0da989
branch: dependabot/github_actions/actions/upload-artifact-7
pr: 53
subject: Bump actions/upload-artifact from 4 to 7
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38611763581
fmt: success
clippy: success
test: success

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.08s
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
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 24.90s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.23s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-9ce6e111dfd7552e)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.87s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.69s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-85e2fc988cd7f06b)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction has been running for over 60 seconds
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.95s


shard client: all passed
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 27.08s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.19s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-1faf7ffdc21a65f6)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 62.28s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.56s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-6f8d415a6700b310)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test keystroke_shows_at_once_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test unicode_input_with_prediction has been running for over 60 seconds
test unicode_input_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.57s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
Server now attached to client at 127.0.0.1:36505
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.02s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.84s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-851340c557c2d991)

running 5 tests
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:51134
test malformed_fragments_are_errors ... ok
test malformed_packets_through_transport ... ok
test fragment_round_trip ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.67s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-da84df6f89db10d6)

running 1 test
Server now attached to client at 127.0.0.1:36751
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.00s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.31s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-79cf2d46a77088c8)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test loss_half_each_direction ... ok
test blackout_silences_then_resumes ... ok
test rebind_changes_source_port ... ok
test delay_applies_both_directions ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.07s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.48s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-e3bac9f2651bcd35)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.30s
     Running tests/launcher.rs (target/release/deps/launcher-b23aeb2b3f0e0ef1)

running 9 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test options_and_messages_match_mosh_pl ... ok
test control_master_hint_and_remote_mode_reuses_socket ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.09s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
Server now attached to client at 127.0.0.1:57293
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.53s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.41s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-49a0666cb5a89f04)

running 5 tests
test instruction_wire_format ... ok
test malformed_fragments_are_errors ... ok
Server now attached to client at 127.0.0.1:51529
test malformed_packets_through_transport ... ok
test fragment_round_trip ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.98s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-c402dc70e8e68d22)

running 1 test
Server now attached to client at 127.0.0.1:49306
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.07s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.75s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-6a4f634d8b6e4ed5)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test blackout_silences_then_resumes ... ok
test loss_half_each_direction ... ok
test delay_applies_both_directions ... ok
test rebind_changes_source_port ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.18s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.96s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-57bd1582beb2872d)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.68s
     Running tests/launcher.rs (target/release/deps/launcher-45f9c0251938de1a)

running 9 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test control_master_hint_and_remote_mode_reuses_socket ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.18s


shard conformance: all passed
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=244

```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.6
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.2
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=5.0
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.9
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.4
interop: 30 cells, 30 passed, 0 failed, wall time 40s
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
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.38s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=37936
Server now attached to client at 127.0.0.1:56644
Server now attached to client at 127.0.0.1:37936
elapsed 10.42s; client: status=done elapsed_ms=10419 packets_received=20 acked_ts_advance=10030; server: packets_received=49 acked_ts_advance=9850 remote_addr=127.0.0.1:37936; state 5992 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.42s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:59139
elapsed 10.45s; client: status=done elapsed_ms=10443 packets_received=44 acked_ts_advance=10072; server: packets_received=71 acked_ts_advance=9918 remote_addr=127.0.0.1:59139; state 6272 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.45s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.45s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:60543
elapsed 10.44s; client: status=done elapsed_ms=10441 packets_received=9 acked_ts_advance=10399; server: packets_received=35 acked_ts_advance=9402 remote_addr=127.0.0.1:60543; state 6066 bytes
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=244

shard scrollback-sh: all passed
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=246

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.8
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.4
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=11.2
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=13.1
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=19.0
interop: 30 cells, 30 passed, 0 failed, wall time 44s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (141 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.91s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=43888
Server now attached to client at 127.0.0.1:37899
Server now attached to client at 127.0.0.1:43888
elapsed 25.55s; client: status=done elapsed_ms=25543 packets_received=20 acked_ts_advance=25494; server: packets_received=106 acked_ts_advance=9373 remote_addr=127.0.0.1:43888; state 6343 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 25.55s
proxy stats: c2s forwarded=106 dropped=36 reordered=10 s2c forwarded=20 dropped=14 reordered=3
PASS loss30-rebind: converged in 25.55s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:46512
elapsed 10.35s; client: status=done elapsed_ms=10346 packets_received=41 acked_ts_advance=10114; server: packets_received=72 acked_ts_advance=9503 remote_addr=127.0.0.1:46512; state 6370 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=72 dropped=7 reordered=7 s2c forwarded=41 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:45442
elapsed 10.46s; client: status=done elapsed_ms=10453 packets_received=11 acked_ts_advance=10058; server: packets_received=38 acked_ts_advance=9808 remote_addr=127.0.0.1:45442; state 6319 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=38 dropped=54 reordered=4 s2c forwarded=11 dropped=6 reordered=2
PASS loss30-blackout: converged in 10.46s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=246

shard scrollback-sh: all passed
```

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 21.89s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.38s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-84a20622c1007c10)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.85s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.76s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-6ba97842804857bf)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.57s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.61s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-0ad4f7e742b081ab)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=116 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.81s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.73s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-6191b1bbbbea4b8e)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.15s


shard server: all passed
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 22.07s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.21s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-acf21c1f6e8cb8cf)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.85s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.80s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-70c23f908f5a88a5)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.59s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.68s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-7a2729bb0d71efaa)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=120 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.82s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.3 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.73s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-2777a3349a68f3df)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.27s


shard server: all passed
```
