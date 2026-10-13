result: failure
commit: cb97dead499499674247b67e7043f945243d4147
branch: agent/launcher-ci
pr: 6
subject: launcher test: compare against the canonical binary path (CI runners use target/../); list every differing case in the failure message
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38432847587
fmt: success
clippy: success
test: failure

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 12.56s
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
    Finished `release` profile [optimized] target(s) in 33.18s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.06s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-314fc7e2b9dd1429)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test command_output_and_exit ... ok
test interactive_shell_and_resize ... ok
test killed_server_shows_last_contact ... ok
test idle_cpu has been running for over 60 seconds
test idle_cpu ... ok
test no_server_times_out ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 65.14s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.66s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-66923b123ab4d06e)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 45.78s


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
    Finished `release` profile [optimized] target(s) in 59.61s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.22s
     Running tests/client_e2e.rs (target/release/deps/client_e2e-904bae8030d6b876)

running 7 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test escape_quit_restores_terminal ... ok
test escape_suspend_and_resume ... ok
test command_output_and_exit ... ok
test interactive_shell_and_resize ... ok
test killed_server_shows_last_contact ... ok
test idle_cpu has been running for over 60 seconds
test idle_cpu ... ok
test no_server_times_out ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 66.17s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.14s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-12101d809925cbb3)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 46.29s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
running 3 tests
Server now attached to client at 127.0.0.1:39713
Server now attached to client at 127.0.0.1:43503
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.99s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.69s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-83c2326efb456c98)

running 5 tests
test fragment_round_trip ... ok
test instruction_wire_format ... ok
test malformed_fragments_are_errors ... ok
Server now attached to client at 127.0.0.1:49644
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.40s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-64b017b81515f1cf)

running 1 test
Server now attached to client at 127.0.0.1:44362
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.00s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.17s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-05ef8c9a34b34126)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test blackout_silences_then_resumes ... ok
test stats_and_sigterm ... ok
test reorder_delivers_out_of_sequence ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.68s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.37s
     Running tests/platform_tty.rs (target/release/deps/platform_tty-d09b608915fcb24e)

running 1 test
test raw_mode_resize_and_large_write_through_a_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

== cargo test --release -p mosh-tests --test launcher
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.90s
     Running tests/launcher.rs (target/release/deps/launcher-4521131ad8fc8c2b)

running 7 tests
test launches_session_on_dual_stack_name ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
test launches_session_with_user_at_ip_literal ... ok
test missing_server_is_reported ... ok
test launches_session_on_localhost_with_command ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.79s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
running 3 tests
Server now attached to client at 127.0.0.1:50017
Server now attached to client at 127.0.0.1:50890
test malformed_diffs_through_transport ... ok
test userstream_roundtrip ... ok
test complete_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.92s

== cargo test --release -p mosh-tests --test transport_roundtrip
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 8.64s
     Running tests/transport_roundtrip.rs (target/release/deps/transport_roundtrip-008fd545209037fd)

running 5 tests
test fragment_round_trip ... ok
test instruction_wire_format ... ok
test malformed_fragments_are_errors ... ok
Server now attached to client at 127.0.0.1:34080
test malformed_packets_through_transport ... ok
test compressor_round_trip ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

== cargo test --release -p mosh-tests --test transport_sync
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.24s
     Running tests/transport_sync.rs (target/release/deps/transport_sync-f887e0d631d090ac)

running 1 test
Server now attached to client at 127.0.0.1:44911
test transport_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.04s

== cargo test --release -p mosh-tests --test udp_impair
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.68s
     Running tests/udp_impair.rs (target/release/deps/udp_impair-8558686294110e71)

running 6 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test delay_applies_both_directions ... ok
test loss_half_each_direction ... ok
test rebind_changes_source_port ... ok
test blackout_silences_then_resumes ... ok
test stats_and_sigterm ... ok
test reorder_delivers_out_of_sequence ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.81s

== cargo test --release -p mosh-tests --test platform_tty
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.11s
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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.24s


shard conformance: all passed
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
FAIL altscreen (122 s): screen differs from direct-vim.txt: 0a1|> 2978|23,24c24|< $ vim -u NONE -N|< $|---|
  screen diff, expected (direct-vim.txt) vs shown:
  | 0a1
  | > 2978
  | 23,24c24
--
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=239

shard scrollback-sh: FAILED: tests/scrollback.sh
```

## log-test-scrollback-sh-ubuntu-24.04-arm (last 80 lines)
```
  | ---
  | > $ $
  screen at the failure:
  | 2978
  | 2979
  | 2980
  | 2981
  | 2982
  | 2983
  | 2984
  | 2985
  | 2986
  | 2987
  | 2988
  | 2989
  | 2990
  | 2991
  | 2992
  | 2993
  | 2994
  | 2995
  | 2996
  | 2997
  | 2998
  | 2999
  | 3000
  | $ $
  (history capture: /home/runner/work/mosh-rs/mosh-rs/target/scrollback/altscreen/capture.txt, 3006 lines)
PASS resize (4 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (2 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (2 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (0 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (22 s): lines=2783 numbers=2777 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197223 before #1536'] prompts=0 other=0; notice: ^[[2m[mosh: 197223 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.85s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=56286
Server now attached to client at 127.0.0.1:35104
Server now attached to client at 127.0.0.1:56286
elapsed 10.93s; client: status=done elapsed_ms=10931 packets_received=16 acked_ts_advance=10860; server: packets_received=47 acked_ts_advance=9932 remote_addr=127.0.0.1:56286; state 5805 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.93s
proxy stats: c2s forwarded=47 dropped=15 reordered=4 s2c forwarded=16 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.93s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:46929
elapsed 10.35s; client: status=done elapsed_ms=10343 packets_received=43 acked_ts_advance=9992; server: packets_received=71 acked_ts_advance=9385 remote_addr=127.0.0.1:46929; state 6277 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:54680
elapsed 10.47s; client: status=done elapsed_ms=10464 packets_received=9 acked_ts_advance=10263; server: packets_received=36 acked_ts_advance=9269 remote_addr=127.0.0.1:54680; state 5812 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.47s
proxy stats: c2s forwarded=36 dropped=41 reordered=6 s2c forwarded=9 dropped=6 reordered=0
PASS loss30-blackout: converged in 10.47s, 1 client address(es)
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

shard scrollback-sh: FAILED: tests/scrollback.sh
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
FAIL altscreen (122 s): screen differs from direct-vim.txt: 0a1|> 2978|23,24c24|< $ vim -u NONE -N|< $|---|
  screen diff, expected (direct-vim.txt) vs shown:
  | 0a1
  | > 2978
  | 23,24c24
--
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=234

shard scrollback-sh: FAILED: tests/scrollback.sh
```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
  | ---
  | > $ $
  screen at the failure:
  | 2978
  | 2979
  | 2980
  | 2981
  | 2982
  | 2983
  | 2984
  | 2985
  | 2986
  | 2987
  | 2988
  | 2989
  | 2990
  | 2991
  | 2992
  | 2993
  | 2994
  | 2995
  | 2996
  | 2997
  | 2998
  | 2999
  | 3000
  | $ $
  (history capture: /home/runner/work/mosh-rs/mosh-rs/target/scrollback/altscreen/capture.txt, 3006 lines)
PASS resize (4 s): lines=6008 numbers=6000 first=1 last=6000 in_order=True duplicates=0 gaps=none notices=none prompts=1 other=0
PASS shrink (2 s): lines=6008 numbers=6000 first=1 last=6000 in_order=False duplicates=6 (2978,2979,2980,2981,2982,2983) gaps=2995-3000 notices=none prompts=0 other=1
  unexpected line 3005: '29seq 3001 6000; echo DONE-SEQ'
PASS grow (2 s): lines=6002 numbers=5994 first=1 last=6000 in_order=True duplicates=0 gaps=2972-2977 notices=none prompts=1 other=0
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (22 s): lines=2783 numbers=2777 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197223 before #1536'] prompts=0 other=0; notice: ^[[2m[mosh: 197223 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.01s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=41128
Server now attached to client at 127.0.0.1:43792
Server now attached to client at 127.0.0.1:41128
elapsed 10.58s; client: status=done elapsed_ms=10582 packets_received=15 acked_ts_advance=10504; server: packets_received=51 acked_ts_advance=9485 remote_addr=127.0.0.1:41128; state 6343 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.58s
proxy stats: c2s forwarded=51 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.58s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:60079
elapsed 13.36s; client: status=done elapsed_ms=13362 packets_received=46 acked_ts_advance=13048; server: packets_received=77 acked_ts_advance=10086 remote_addr=127.0.0.1:60079; state 5964 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.36s
proxy stats: c2s forwarded=77 dropped=7 reordered=6 s2c forwarded=46 dropped=3 reordered=4
PASS loss5: converged in 13.36s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:41023
elapsed 10.31s; client: status=done elapsed_ms=10311 packets_received=13 acked_ts_advance=9967; server: packets_received=40 acked_ts_advance=9739 remote_addr=127.0.0.1:41023; state 6250 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.31s
proxy stats: c2s forwarded=40 dropped=43 reordered=4 s2c forwarded=13 dropped=7 reordered=2
PASS loss30-blackout: converged in 10.31s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=234

shard scrollback-sh: FAILED: tests/scrollback.sh
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
    Finished `release` profile [optimized] target(s) in 34.95s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.83s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-47f978d2a268e604)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.01s
test command_output_and_exit ... ok
test network_timeout ... ok
test typing_and_resize ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.44s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.45s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-fda447bf2e58152f)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.61s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.07s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-f795f198656a1e36)

running 13 tests
test capability_wire_format ... ok
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
test live::cxx_server_ignores_capability ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=118 reordered=0
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
test live::rust_server_loss30_delay100 has been running for over 60 seconds
udp-impair stats: c2s forwarded=83 dropped=44 reordered=0 s2c forwarded=570 dropped=253 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 206.98s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.52s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-5eea0241760472e3)

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
   Compiling mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
   Compiling mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
   Compiling mosh v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/mosh)
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 34.82s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.85s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-b5aa7fe1a8b3b615)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test command_output_and_exit ... ok
test network_timeout ... ok
test typing_and_resize ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.48s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.68s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-ef7d8786a0b0cdb2)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.59s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.41s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-1f11e0621a67ec02)

running 13 tests
test capability_wire_format ... ok
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
test live::cxx_server_ignores_capability ... ok
    Finished `release` profile [optimized] target(s) in 0.01s
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=128 dropped=112 reordered=0
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
test live::rust_server_loss30_delay100 has been running for over 60 seconds
udp-impair stats: c2s forwarded=83 dropped=46 reordered=0 s2c forwarded=561 dropped=247 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 207.66s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 4.35s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-707edeeb36edf4c0)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.19s


shard server: all passed
```
