result: success
commit: 76767cfb77efd3cae343983a25c3a178b1605c87
branch: laptop/windows-static-crt
pr: 60
subject: Version 0.1.4; changelog and README version references
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/42107887159
fmt: success
clippy: success
test: success

## log-clippy: failures
```
```

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
    Checking mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.08s
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
   Compiling mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 15.37s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.30s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-50f7a51c37e5378a)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.61s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.00s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-d05a9bbe88c5f914)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction has been running for over 60 seconds
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.72s


shard client: all passed
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 37.72s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.15s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-22901d73251ffa58)

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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 62.19s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.61s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-6c511527aa7d73a8)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction has been running for over 60 seconds
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.20s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
Server now attached to client at 127.0.0.1:44677
Server now attached to client at 127.0.0.1:49294
test malformed_diffs_through_transport ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.09s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.68s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-7a6342d0413fe7a3)

running 5 tests
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:41675
test malformed_packets_through_transport ... ok
test fragment_round_trip ... ok
test malformed_fragments_are_errors ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.40s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-0d7cb58998db1c2c)

running 1 test
Server now attached to client at 127.0.0.1:54405
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.00s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.10s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-3558cc549c4cc59e)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test blackout_silences_then_resumes ... ok
test loss_half_each_direction ... ok
test delay_applies_both_directions ... ok
test rebind_changes_source_port ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.06s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.30s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-df0e1c94bb1dc0b7)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.01s
     Running tests/launcher.rs (target/release/deps/launcher-aa75b8bbfa3fb078)

running 9 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test options_and_messages_match_mosh_pl ... ok
test control_master_hint_and_remote_mode_reuses_socket ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.00s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
Server now attached to client at 127.0.0.1:59494
Server now attached to client at 127.0.0.1:33906
test malformed_diffs_through_transport ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.84s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.76s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-2f17b2e8221b8787)

running 5 tests
test instruction_wire_format ... ok
Server now attached to client at 127.0.0.1:48859
test malformed_fragments_are_errors ... ok
test malformed_packets_through_transport ... ok
test fragment_round_trip ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.45s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-42ffa0ddf55a1748)

running 1 test
Server now attached to client at 127.0.0.1:38546
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.00s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.03s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-fb77006e9b07af06)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test blackout_silences_then_resumes ... ok
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test reorder_delivers_out_of_sequence ... ok
test stats_and_sigterm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.16s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.19s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-e2710f9a466e807a)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.08s
     Running tests/launcher.rs (target/release/deps/launcher-e0a5342ed1ee2fe6)

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

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.27s


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
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=5.2
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.5
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.4
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.3
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.3
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.2
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=5.9
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=7.3
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=16.0
interop: 30 cells, 30 passed, 0 failed, wall time 41s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (170 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.15s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=49389
Server now attached to client at 127.0.0.1:39201
Server now attached to client at 127.0.0.1:49389
elapsed 16.80s; client: status=done elapsed_ms=16797 packets_received=16 acked_ts_advance=16796; server: packets_received=91 acked_ts_advance=9732 remote_addr=127.0.0.1:49389; state 6366 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.80s
proxy stats: c2s forwarded=91 dropped=24 reordered=9 s2c forwarded=16 dropped=13 reordered=3
PASS loss30-rebind: converged in 16.80s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:42531
elapsed 10.38s; client: status=done elapsed_ms=10379 packets_received=42 acked_ts_advance=10026; server: packets_received=72 acked_ts_advance=9889 remote_addr=127.0.0.1:42531; state 6246 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.38s
proxy stats: c2s forwarded=72 dropped=6 reordered=7 s2c forwarded=42 dropped=2 reordered=2
PASS loss5: converged in 10.38s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:43770
elapsed 10.79s; client: status=done elapsed_ms=10785 packets_received=7 acked_ts_advance=10647; server: packets_received=39 acked_ts_advance=9366 remote_addr=127.0.0.1:43770; state 6199 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.79s
proxy stats: c2s forwarded=39 dropped=48 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.79s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=235

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=loss30 scenario=seq seconds=5.1
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=12.4
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.0
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.1
PASS client=cxx server=cxx profile=clean scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.2
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.3
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.5
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.1
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.6
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=12.0
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.5
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.3
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=5.7
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=13.6
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.1
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=15.4
interop: 30 cells, 30 passed, 0 failed, wall time 40s
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
PASS dropped (22 s): lines=2783 numbers=2777 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197223 before #1536'] prompts=0 other=0; notice: ^[[2m[mosh: 197223 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.76s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=52599
Server now attached to client at 127.0.0.1:40453
Server now attached to client at 127.0.0.1:52599
elapsed 10.46s; client: status=done elapsed_ms=10458 packets_received=18 acked_ts_advance=10208; server: packets_received=49 acked_ts_advance=9477 remote_addr=127.0.0.1:52599; state 6394 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s
proxy stats: c2s forwarded=49 dropped=15 reordered=6 s2c forwarded=18 dropped=6 reordered=0
PASS loss30-rebind: converged in 10.46s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:40519
elapsed 13.36s; client: status=done elapsed_ms=13362 packets_received=46 acked_ts_advance=13047; server: packets_received=77 acked_ts_advance=10086 remote_addr=127.0.0.1:40519; state 5757 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.36s
proxy stats: c2s forwarded=77 dropped=7 reordered=6 s2c forwarded=46 dropped=3 reordered=4
PASS loss5: converged in 13.36s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:52596
elapsed 10.70s; client: status=done elapsed_ms=10698 packets_received=7 acked_ts_advance=10558; server: packets_received=39 acked_ts_advance=9287 remote_addr=127.0.0.1:52596; state 6272 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.70s
proxy stats: c2s forwarded=39 dropped=45 reordered=5 s2c forwarded=7 dropped=6 reordered=1
PASS loss30-blackout: converged in 10.70s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=235

shard scrollback-sh: all passed
```

## log-test-server-ubuntu-24.04-arm: failures
```
```

## log-test-server-ubuntu-24.04-arm (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 20.74s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.89s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-53968afde2d38515)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.89s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.82s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-c6b53e04e17f9f97)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.64s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.35s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-d14c56893e122e76)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=128 dropped=113 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.74s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.55s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-ffc3d15d940c0803)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.24s


shard server: all passed
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
    Updating crates.io index
   Compiling mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
   Compiling mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
   Compiling mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh-network v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 27.74s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.70s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-5cdabf90956d0744)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test typing_and_resize ... ok
test command_output_and_exit ... ok
test network_timeout ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.93s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.26s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-a986b7b1dbbda56b)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.74s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.78s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-de90f7d683a9dbef)

running 13 tests
test capability_wire_format ... ok
    Finished `release` profile [optimized] target(s) in 0.03s
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=129 dropped=118 reordered=0
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

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.79s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.05s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-613927f7f06f8134)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.23s


shard server: all passed
```
