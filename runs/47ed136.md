result: failure
commit: 47ed13639d9ecda94e553792c9e951a4ef7e6329
branch: agent/udp-impair-blackout-test
pr: 7
subject: tests: udp_impair blackout test drains in-flight replies and judges packets by send time
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38433763497
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.38s
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
    Finished `release` profile [optimized] target(s) in 35.48s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.48s
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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 65.13s

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
    Finished `release` profile [optimized] target(s) in 46.53s
== cargo test --release -p mosh-tests --test client_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.67s
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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 66.02s

== cargo test --release -p mosh-tests --test prediction_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.55s
     Running tests/prediction_e2e.rs (target/release/deps/prediction_e2e-12101d809925cbb3)

running 3 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 46.61s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
test options_and_messages_match_mosh_pl ... FAILED

failures:

---- options_and_messages_match_mosh_pl stdout ----
--
thread 'options_and_messages_match_mosh_pl' (20149) panicked at tests/tests/launcher.rs:532:5:
assertion `left == right` failed: launcher differs from mosh.pl
  left: 33
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
--
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.72s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
--- stderr
--- ssh argv
-n
-tt
-S
none
-o
ProxyCommand='/home/runner/work/mosh-rs/mosh-rs/target/release/mosh' '--family=prefer-inet' --fake-proxy -- %h %p
host
--
mosh-server 'new' '-c' '256' '-s' '-l' 'LANG=C.UTF-8'
--- client
arg -# --ssh=/tmp/mosh-rs-launcher-fakes-19787/fakessh --client=/tmp/mosh-rs-launcher-fakes-19787/fakeclient host |
arg 192.0.2.7
arg 60001
MOSH_KEY=AAAAAAAAAAAAAAAAAAAAAA
MOSH_PREDICTION_DISPLAY=never

DIFFERENT: ["-p", "60001:60010", "host"] [("FAKE_SSH_OUT", "MOSH IP 192.0.2.7\\nBad UDP port range\\n")]
=== mosh.pl
exit Some(10)
--- stdout
Bad UDP port range
--- stderr
$0: Server does not support UDP port range option.
--- ssh argv
-n
-tt
-S
none
-o
ProxyCommand='$0' '--family=prefer-inet' --fake-proxy -- %h %p
host
--
mosh-server 'new' '-c' '256' '-s' '-p' '60001:60010' '-l' 'LANG=C.UTF-8'
--- client
(none)
=== ours
exit Some(10)
--- stdout
Bad UDP port range
--- stderr
$0: Server does not support UDP port range option.
--- ssh argv
-n
-tt
-S
none
-o
ProxyCommand='/home/runner/work/mosh-rs/mosh-rs/target/release/mosh' '--family=prefer-inet' --fake-proxy -- %h %p
host
--
mosh-server 'new' '-c' '256' '-s' '-p' '60001:60010' '-l' 'LANG=C.UTF-8'
--- client
(none)

same: ["--experimental-remote-ip=remote", "host"] [("FAKE_SSH_OUT", "MOSH SSH_CONNECTION 1 2 3\\n")] -> exit Some(29)
same: ["--experimental-remote-ip=remote", "-6", "host"] [("FAKE_SSH_OUT", "\\r\\nMOSH SSH_CONNECTION 192.0.2.1 5555 192.0.2.9 22\\r\\nMOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\\r\\n")] -> exit Some(0)
same: ["--experimental-remote-ip=local", "me@127.0.0.1", "--", "ls"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(0)
same: ["--e=local", "a@b@127.0.0.1"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(0)
same: ["--experimental-remote-ip=local", "-6", "127.0.0.1"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(255)
same: ["--experimental-remote-ip=local", "nosuch.invalid"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(255)
same: ["--local", "--server=/tmp/mosh-rs-launcher-fakes-19787/fakessh", "127.0.0.1"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(0)
73 cases, 33 different

thread 'options_and_messages_match_mosh_pl' (20149) panicked at tests/tests/launcher.rs:532:5:
assertion `left == right` failed: launcher differs from mosh.pl
  left: 33
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    options_and_messages_match_mosh_pl

test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.72s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-conformance-ubuntu-24.04: failures
```
test options_and_messages_match_mosh_pl ... FAILED

failures:

---- options_and_messages_match_mosh_pl stdout ----
--
thread 'options_and_messages_match_mosh_pl' (20256) panicked at tests/tests/launcher.rs:532:5:
assertion `left == right` failed: launcher differs from mosh.pl
  left: 33
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
--
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.95s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
--- stderr
--- ssh argv
-n
-tt
-S
none
-o
ProxyCommand='/home/runner/work/mosh-rs/mosh-rs/target/release/mosh' '--family=prefer-inet' --fake-proxy -- %h %p
host
--
mosh-server 'new' '-c' '256' '-s' '-l' 'LANG=C.UTF-8'
--- client
arg -# --ssh=/tmp/mosh-rs-launcher-fakes-19888/fakessh --client=/tmp/mosh-rs-launcher-fakes-19888/fakeclient host |
arg 192.0.2.7
arg 60001
MOSH_KEY=AAAAAAAAAAAAAAAAAAAAAA
MOSH_PREDICTION_DISPLAY=never

DIFFERENT: ["-p", "60001:60010", "host"] [("FAKE_SSH_OUT", "MOSH IP 192.0.2.7\\nBad UDP port range\\n")]
=== mosh.pl
exit Some(10)
--- stdout
Bad UDP port range
--- stderr
$0: Server does not support UDP port range option.
--- ssh argv
-n
-tt
-S
none
-o
ProxyCommand='$0' '--family=prefer-inet' --fake-proxy -- %h %p
host
--
mosh-server 'new' '-c' '256' '-s' '-p' '60001:60010' '-l' 'LANG=C.UTF-8'
--- client
(none)
=== ours
exit Some(10)
--- stdout
Bad UDP port range
--- stderr
$0: Server does not support UDP port range option.
--- ssh argv
-n
-tt
-S
none
-o
ProxyCommand='/home/runner/work/mosh-rs/mosh-rs/target/release/mosh' '--family=prefer-inet' --fake-proxy -- %h %p
host
--
mosh-server 'new' '-c' '256' '-s' '-p' '60001:60010' '-l' 'LANG=C.UTF-8'
--- client
(none)

same: ["--experimental-remote-ip=remote", "host"] [("FAKE_SSH_OUT", "MOSH SSH_CONNECTION 1 2 3\\n")] -> exit Some(29)
same: ["--experimental-remote-ip=remote", "-6", "host"] [("FAKE_SSH_OUT", "\\r\\nMOSH SSH_CONNECTION 192.0.2.1 5555 192.0.2.9 22\\r\\nMOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\\r\\n")] -> exit Some(0)
same: ["--experimental-remote-ip=local", "me@127.0.0.1", "--", "ls"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(0)
same: ["--e=local", "a@b@127.0.0.1"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(0)
same: ["--experimental-remote-ip=local", "-6", "127.0.0.1"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(255)
same: ["--experimental-remote-ip=local", "nosuch.invalid"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(255)
same: ["--local", "--server=/tmp/mosh-rs-launcher-fakes-19888/fakessh", "127.0.0.1"] [("FAKE_SSH_OUT", "MOSH CONNECT 60001 AAAAAAAAAAAAAAAAAAAAAA\n")] -> exit Some(0)
73 cases, 33 different

thread 'options_and_messages_match_mosh_pl' (20256) panicked at tests/tests/launcher.rs:532:5:
assertion `left == right` failed: launcher differs from mosh.pl
  left: 33
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    options_and_messages_match_mosh_pl

test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.95s

error: test failed, to rerun pass `-p mosh-tests --test launcher`

shard conformance: FAILED: cargo test --release -p mosh-tests --test launcher
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=241

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
PASS oldserver (1 s): alternate_on=1, notice shown, 24 lines on the main screen after exit, none from the session
PASS dropped (22 s): lines=2783 numbers=2777 first=1 last=200000 in_order=True duplicates=0 gaps=none notices=['197223 before #1536'] prompts=0 other=0; notice: ^[[2m[mosh: 197223 lines of history were dropped]^[[0m
== tests/transport-impaired.sh
    Finished `release` profile [optimized] target(s) in 0.01s
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.99s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=53070
Server now attached to client at 127.0.0.1:56562
Server now attached to client at 127.0.0.1:53070
elapsed 10.44s; client: status=done elapsed_ms=10435 packets_received=13 acked_ts_advance=10350; server: packets_received=48 acked_ts_advance=9417 remote_addr=127.0.0.1:53070; state 6354 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.44s
proxy stats: c2s forwarded=48 dropped=15 reordered=6 s2c forwarded=13 dropped=5 reordered=0
PASS loss30-rebind: converged in 10.44s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:43178
elapsed 10.25s; client: status=done elapsed_ms=10245 packets_received=41 acked_ts_advance=10030; server: packets_received=71 acked_ts_advance=9884 remote_addr=127.0.0.1:43178; state 6064 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.25s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=41 dropped=1 reordered=2
PASS loss5: converged in 10.25s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:46682
elapsed 10.80s; client: status=done elapsed_ms=10801 packets_received=9 acked_ts_advance=10779; server: packets_received=34 acked_ts_advance=9587 remote_addr=127.0.0.1:46682; state 5961 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.80s
proxy stats: c2s forwarded=34 dropped=48 reordered=5 s2c forwarded=9 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.80s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=241

shard scrollback-sh: FAILED: tests/scrollback.sh
```

## log-test-scrollback-sh-ubuntu-24.04: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=240

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
PASS client=cxx server=cxx profile=blackout scenario=seq seconds=13.4
PASS client=cxx server=cxx profile=blackout scenario=wide seconds=13.4
PASS client=cxx server=cxx profile=loss30 scenario=colors seconds=7.5
PASS client=cxx server=cxx profile=blackout scenario=colors seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=resize seconds=13.6
PASS client=cxx server=cxx profile=blackout scenario=typing seconds=13.9
PASS client=cxx server=cxx profile=blackout scenario=vim seconds=14.6
PASS client=cxx server=cxx profile=clean scenario=typing seconds=13.6
PASS client=cxx server=cxx profile=rebind scenario=seq seconds=1.7
PASS client=cxx server=cxx profile=rebind scenario=wide seconds=1.3
PASS client=cxx server=cxx profile=rebind scenario=resize seconds=1.5
PASS client=cxx server=cxx profile=rebind scenario=vim seconds=2.7
PASS client=cxx server=cxx profile=loss30 scenario=wide seconds=3.9
PASS client=cxx server=cxx profile=rebind scenario=colors seconds=4.4
PASS client=cxx server=cxx profile=loss5 scenario=typing seconds=13.5
PASS client=cxx server=cxx profile=loss30 scenario=resize seconds=6.8
PASS client=cxx server=cxx profile=loss30 scenario=vim seconds=17.4
PASS client=cxx server=cxx profile=rebind scenario=typing seconds=12.6
PASS client=cxx server=cxx profile=loss30 scenario=typing seconds=17.2
interop: 30 cells, 30 passed, 0 failed, wall time 43s
== tests/scrollback.sh
scrollback: variants: clean loss30 vim clear altscreen resize shrink grow oldserver dropped
scrollback: tmux 3.4
PASS clean (10 s): lines=50007 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
PASS loss30 (139 s): lines=50005 numbers=50000 first=1 last=50000 in_order=True duplicates=0 gaps=none notices=none prompts=0 other=0
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
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.08s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=55402
Server now attached to client at 127.0.0.1:42796
Server now attached to client at 127.0.0.1:55402
elapsed 10.41s; client: status=done elapsed_ms=10404 packets_received=20 acked_ts_advance=10016; server: packets_received=49 acked_ts_advance=9838 remote_addr=127.0.0.1:55402; state 5967 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.41s
proxy stats: c2s forwarded=49 dropped=12 reordered=4 s2c forwarded=20 dropped=9 reordered=2
PASS loss30-rebind: converged in 10.41s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:59255
elapsed 10.48s; client: status=done elapsed_ms=10481 packets_received=45 acked_ts_advance=10478; server: packets_received=72 acked_ts_advance=9856 remote_addr=127.0.0.1:59255; state 6038 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.48s
proxy stats: c2s forwarded=73 dropped=6 reordered=7 s2c forwarded=45 dropped=3 reordered=3
PASS loss5: converged in 10.48s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:44013
elapsed 10.78s; client: status=done elapsed_ms=10781 packets_received=7 acked_ts_advance=10644; server: packets_received=39 acked_ts_advance=9365 remote_addr=127.0.0.1:44013; state 6301 bytes
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=240

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
    Finished `release` profile [optimized] target(s) in 37.50s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.36s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-47f978d2a268e604)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.02s
test command_output_and_exit ... ok
test network_timeout ... ok
test typing_and_resize ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.50s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.89s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-fda447bf2e58152f)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.01s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.57s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 9.58s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-f795f198656a1e36)

running 13 tests
test capability_wire_format ... ok
test alternate_screen_is_not_captured ... ok
test clear_keeps_the_screen_as_history ... ok
test live::cxx_server_ignores_capability ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=127 dropped=117 reordered=0
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
udp-impair stats: c2s forwarded=83 dropped=50 reordered=0 s2c forwarded=602 dropped=262 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 220.38s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.70s
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
    Finished `release` profile [optimized] target(s) in 54.44s
== cargo test --release -p mosh-tests --test server_e2e
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 7.07s
     Running tests/server_e2e.rs (target/release/deps/server_e2e-b5aa7fe1a8b3b615)

running 4 tests
    Finished `release` profile [optimized] target(s) in 0.03s
test command_output_and_exit ... ok
test network_timeout ... ok
test typing_and_resize ... ok
test signal_timeout ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.91s

== cargo test --release -p mosh-tests --test detached_warning
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.51s
     Running tests/detached_warning.rs (target/release/deps/detached_warning-ef7d8786a0b0cdb2)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test detached_session_is_reported ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.61s

== cargo test --release -p mosh-tests --test scrollback_server
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 10.44s
     Running tests/scrollback_server.rs (target/release/deps/scrollback_server-1f11e0621a67ec02)

running 13 tests
test capability_wire_format ... ok
test clear_keeps_the_screen_as_history ... ok
test alternate_screen_is_not_captured ... ok
test live::cxx_server_ignores_capability ... ok
    Finished `release` profile [optimized] target(s) in 0.02s
udp-impair stats: c2s forwarded=84 dropped=4 reordered=0 s2c forwarded=129 dropped=117 reordered=0
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
udp-impair stats: c2s forwarded=83 dropped=40 reordered=0 s2c forwarded=536 dropped=237 reordered=0
test live::rust_server_loss30_delay100 ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 192.48s

== cargo test --release -p mosh-tests --test scrollback_hostile
   Compiling mosh-tests v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.42s
     Running tests/scrollback_hostile.rs (target/release/deps/scrollback_hostile-707edeeb36edf4c0)

running 1 test
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.15s


shard server: all passed
```
