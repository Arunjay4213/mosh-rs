result: success
commit: bcc1faf993d33314a1e2f83ee407b0e32349f4ed
branch: laptop/review-platform
pr: 20
subject: Add independent review of the platform crate (Phase 3)
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38446272689
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
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.18s
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
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 35.88s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.46s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-314fc7e2b9dd1429)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test command_output_and_exit ... ok
test interactive_shell_and_resize ... ok
test killed_server_shows_last_contact ... ok
test idle_cpu has been running for over 60 seconds
test idle_cpu ... ok
test no_server_times_out ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 65.50s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.00s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-66923b123ab4d06e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 45.99s


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
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 50.42s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.86s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-904bae8030d6b876)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.04s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test command_output_and_exit ... ok
test interactive_shell_and_resize ... ok
test killed_server_shows_last_contact ... ok
test idle_cpu has been running for over 60 seconds
test no_server_times_out ... ok
test idle_cpu ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 66.30s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.42s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-12101d809925cbb3)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 46.66s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
running 3 tests
Server now attached to client at 127.0.0.1:42723
Server now attached to client at 127.0.0.1:52939
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.99s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.76s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-83c2326efb456c98)

running 5 tests
test fragment_round_trip ... ok
test instruction_wire_format ... ok
test malformed_fragments_are_errors ... ok
Server now attached to client at 127.0.0.1:44159
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.61s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-64b017b81515f1cf)

running 1 test
Server now attached to client at 127.0.0.1:59308
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.04s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.25s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-05ef8c9a34b34126)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test blackout_silences_then_resumes ... ok
test stats_and_sigterm ... ok
test reorder_delivers_out_of_sequence ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.66s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.54s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-d09b608915fcb24e)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.00s
     Running tests/launcher.rs (target/release/deps/launcher-4521131ad8fc8c2b)

running 7 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test launches_session_with_user_at_ip_literal ... ok
test missing_server_is_reported ... ok
test launches_session_on_localhost_with_command ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.86s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
running 3 tests
Server now attached to client at 127.0.0.1:37893
Server now attached to client at 127.0.0.1:47757
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.90s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.35s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-008fd545209037fd)

running 5 tests
test fragment_round_trip ... ok
test instruction_wire_format ... ok
test malformed_fragments_are_errors ... ok
Server now attached to client at 127.0.0.1:39000
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.09s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-f887e0d631d090ac)

running 1 test
Server now attached to client at 127.0.0.1:59633
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.07s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.84s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-8558686294110e71)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test blackout_silences_then_resumes ... ok
test stats_and_sigterm ... ok
test reorder_delivers_out_of_sequence ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.80s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.06s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-9506cf8a1d20952d)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.53s
     Running tests/launcher.rs (target/release/deps/launcher-9a1b47fd88e0c184)

running 7 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test launches_session_with_user_at_ip_literal ... ok
test missing_server_is_reported ... ok
test launches_session_on_localhost_with_command ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.08s


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
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=6.4
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.5
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.4
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=4.6
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.8
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.1
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.9
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.0
interop: 30 cells, 30 passed, 0 failed, wall time 42s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (135 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
    Finished `release` profile [optimized] target(s) in 6.05s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=55720
Server now attached to client at 127.0.0.1:55836
Server now attached to client at 127.0.0.1:55720
elapsed 10.42s; client: status=done elapsed_ms=10416 packets_received=20 acked_ts_advance=10027; server: packets_received=49 acked_ts_advance=9847 remote_addr=127.0.0.1:55720; state 6091 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.42s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:57952
elapsed 10.46s; client: status=done elapsed_ms=10459 packets_received=44 acked_ts_advance=10087; server: packets_received=71 acked_ts_advance=9937 remote_addr=127.0.0.1:57952; state 6117 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.46s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:55107
elapsed 10.74s; client: status=done elapsed_ms=10738 packets_received=7 acked_ts_advance=10599; server: packets_received=39 acked_ts_advance=9327 remote_addr=127.0.0.1:55107; state 6264 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.74s
proxy stats: c2s forwarded=39 dropped=46 reordered=5 s2c forwarded=7 dropped=6 reordered=1
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=239

shard scrollback-sh: all passed
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=249

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.9
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.7
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=14.0
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.7
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.8
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=15.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=14.7
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.7
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.9
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=14.3
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.7
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=16.8
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=13.0
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=18.2
interop: 30 cells, 30 passed, 0 failed, wall time 44s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (166 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
    Finished `release` profile [optimized] target(s) in 0.02s
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.36s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=39105
Server now attached to client at 127.0.0.1:47806
Server now attached to client at 127.0.0.1:39105
elapsed 10.93s; client: status=done elapsed_ms=10926 packets_received=16 acked_ts_advance=10855; server: packets_received=47 acked_ts_advance=9928 remote_addr=127.0.0.1:39105; state 6050 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.93s
proxy stats: c2s forwarded=47 dropped=15 reordered=4 s2c forwarded=16 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.93s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:43539
elapsed 10.42s; client: status=done elapsed_ms=10417 packets_received=43 acked_ts_advance=10037; server: packets_received=71 acked_ts_advance=9887 remote_addr=127.0.0.1:43539; state 5871 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:43984
elapsed 11.06s; client: status=done elapsed_ms=11061 packets_received=13 acked_ts_advance=10984; server: packets_received=38 acked_ts_advance=10093 remote_addr=127.0.0.1:43984; state 6094 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.07s
proxy stats: c2s forwarded=38 dropped=42 reordered=6 s2c forwarded=13 dropped=7 reordered=0
PASS loss30-blackout: converged in 11.06s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=249

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
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 35.37s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.99s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-47f978d2a268e604)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test command_output_and_exit ... ok
test network_timeout ... ok
test typing_and_resize ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.47s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.73s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-fda447bf2e58152f)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.72s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.57s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-f795f198656a1e36)

running 13 tests
test capability_wire_format ... ok
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
test live::cxx_server_ignores_capability ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=119 reordered=0
test live::rust_server_blackout_10s ... ok
test live::rust_server_clean ... ok
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
test old_peer_diffs_unchanged ... ok
test partial_scroll_region_is_not_captured ... ok
test rendered_lines_match_upstream_cells ... ok
test seq_50000_random_snapshots ... ok
test seq_with_resizes ... ok
test small_backlog_drops_oldest_unpublished ... ok
udp-impair stats: c2s forwarded=17 dropped=5 reordered=0 s2c forwarded=64 dropped=27 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.79s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.72s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-5eea0241760472e3)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.21s


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
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 35.82s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.37s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-b5aa7fe1a8b3b615)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test command_output_and_exit ... ok
test network_timeout ... ok
test typing_and_resize ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.58s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.96s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-ef7d8786a0b0cdb2)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.64s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.81s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-1f11e0621a67ec02)

running 13 tests
test capability_wire_format ... ok
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
test live::cxx_server_ignores_capability ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=126 dropped=117 reordered=0
test live::rust_server_blackout_10s ... ok
test live::rust_server_clean ... ok
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
Contents: ' ̂' [0xc2, 0xa0, 0xcc, 0x82] (4) vs. ' ̂' [0xcc, 0x82] (2)
fallback: 0 vs. 1
test old_peer_diffs_unchanged ... ok
test partial_scroll_region_is_not_captured ... ok
test rendered_lines_match_upstream_cells ... ok
test seq_50000_random_snapshots ... ok
test seq_with_resizes ... ok
test small_backlog_drops_oldest_unpublished ... ok
udp-impair stats: c2s forwarded=17 dropped=5 reordered=0 s2c forwarded=64 dropped=27 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.81s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.01s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-707edeeb36edf4c0)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.22s


shard server: all passed
```
