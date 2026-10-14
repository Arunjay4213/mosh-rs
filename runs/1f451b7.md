result: success
commit: 1f451b7ff679cd6d72b3b847d6bddfe8a8b3fa52
branch: laptop/brew-verified
pr: 49
subject: STATUS, INTERVENTIONS: Homebrew tap verified on macOS 15.8
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38514515762
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.89s
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
    Finished `release` profile [optimized] target(s) in 17.20s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.25s
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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.60s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.84s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-ac57dd13212b5c6e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test keystroke_shows_at_once_with_prediction ... ok
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed has been running for over 60 seconds
test wrong_prediction_is_removed ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.42s


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
    Finished `release` profile [optimized] target(s) in 29.09s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.47s
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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 62.06s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.03s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-5b963b9efd33ce5e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test keystroke_shows_at_once_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test unicode_input_with_prediction has been running for over 60 seconds
test unicode_input_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 65.04s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
test userstream_roundtrip ... ok
Server now attached to client at 127.0.0.1:35651
Server now attached to client at 127.0.0.1:39414
test malformed_diffs_through_transport ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.98s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.68s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-cc62286ad719b2f9)

running 5 tests
test fragment_round_trip ... ok
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:37307
test malformed_fragments_are_errors ... ok
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.18s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-85c6987c331b6535)

running 1 test
Server now attached to client at 127.0.0.1:39811
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.97s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.11s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-ec52cb020c98762f)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test blackout_silences_then_resumes ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test delay_applies_both_directions ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.06s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.25s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-c51facc5d19698ed)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.88s
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

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.26s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
test userstream_roundtrip ... ok
Server now attached to client at 127.0.0.1:55969
Server now attached to client at 127.0.0.1:36619
test malformed_diffs_through_transport ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.80s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.41s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-a34a8193ced1844f)

running 5 tests
test malformed_fragments_are_errors ... ok
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:60284
test fragment_round_trip ... ok
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.09s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-5e15975671f6f1c7)

running 1 test
Server now attached to client at 127.0.0.1:51265
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.00s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.73s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-6453a427fc1d2bab)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test blackout_silences_then_resumes ... ok
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.18s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.87s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-ae583a58305a3ead)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.43s
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

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.30s


shard conformance: all passed
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
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.2
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=12.9
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.2
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.0
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.4
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=10.7
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=9.5
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.8
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (118 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
    Finished `release` profile [optimized] target(s) in 5.97s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=41184
Server now attached to client at 127.0.0.1:53152
Server now attached to client at 127.0.0.1:41184
elapsed 16.79s; client: status=done elapsed_ms=16783 packets_received=16 acked_ts_advance=16643; server: packets_received=91 acked_ts_advance=9719 remote_addr=127.0.0.1:41184; state 6449 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.79s
proxy stats: c2s forwarded=91 dropped=24 reordered=9 s2c forwarded=16 dropped=13 reordered=3
PASS loss30-rebind: converged in 16.79s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:46385
elapsed 13.38s; client: status=done elapsed_ms=13381 packets_received=46 acked_ts_advance=13067; server: packets_received=77 acked_ts_advance=10105 remote_addr=127.0.0.1:46385; state 6092 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.38s
proxy stats: c2s forwarded=77 dropped=7 reordered=6 s2c forwarded=46 dropped=3 reordered=4
PASS loss5: converged in 13.38s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:42910
elapsed 10.33s; client: status=done elapsed_ms=10326 packets_received=13 acked_ts_advance=9981; server: packets_received=40 acked_ts_advance=9753 remote_addr=127.0.0.1:42910; state 6223 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.33s
proxy stats: c2s forwarded=40 dropped=43 reordered=4 s2c forwarded=13 dropped=7 reordered=2
PASS loss30-blackout: converged in 10.33s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=243

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.3
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.3
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.4
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.0
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=20.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.6
interop: 30 cells, 30 passed, 0 failed, wall time 40s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (177 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
    Finished `release` profile [optimized] target(s) in 6.90s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=44402
Server now attached to client at 127.0.0.1:33960
Server now attached to client at 127.0.0.1:44402
elapsed 10.99s; client: status=done elapsed_ms=10983 packets_received=15 acked_ts_advance=10916; server: packets_received=48 acked_ts_advance=9985 remote_addr=127.0.0.1:44402; state 6093 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.99s
proxy stats: c2s forwarded=48 dropped=15 reordered=4 s2c forwarded=15 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.99s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:43982
elapsed 10.83s; client: status=done elapsed_ms=10825 packets_received=47 acked_ts_advance=10790; server: packets_received=73 acked_ts_advance=10036 remote_addr=127.0.0.1:43982; state 6413 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.83s
proxy stats: c2s forwarded=73 dropped=7 reordered=7 s2c forwarded=47 dropped=3 reordered=3
PASS loss5: converged in 10.83s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:39726
elapsed 10.78s; client: status=done elapsed_ms=10781 packets_received=7 acked_ts_advance=10644; server: packets_received=39 acked_ts_advance=9365 remote_addr=127.0.0.1:39726; state 6616 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.78s
proxy stats: c2s forwarded=39 dropped=48 reordered=5 s2c forwarded=7 dropped=6 reordered=1
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=243

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
    Finished `release` profile [optimized] target(s) in 19.84s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.92s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-69dd234075438fc1)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.19s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.65s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-219dcd6c3e572b42)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.64s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.81s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-83adf5d4726fd392)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=117 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.77s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.45s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-726aa65b474b5b67)

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
   Compiling mosh-platform v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 23.72s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.58s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-936cf87a3ec1e824)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.90s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.89s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-302f71dd22d1993a)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.67s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.31s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-c9e9e20f1be4041c)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=131 dropped=115 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.76s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.2 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.90s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-33aa1bab0c62d2a0)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.24s


shard server: all passed
```
