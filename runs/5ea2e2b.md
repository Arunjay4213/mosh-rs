result: failure
commit: 5ea2e2b152eb7a3ecc9a30802ea702132d528975
branch: dependabot/cargo/rand-0.10.4
pr: 65
subject: Bump rand from 0.10.3 to 0.10.4
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/45427237025
fmt: success
clippy: failure
test: success

## log-clippy: failures
```
error: this `if` guard returns a bool literal and is followed by another
  --> crates/network/src/crypto/base64.rs:82:5
   |
82 | /     if b64[22] != b'=' || b64[23] != b'=' {
83 | |         return false;
--
error: the loop variable `i` is only used to index `self.fragments`
   --> crates/network/src/transport/transportfragment.rs:229:18
    |
229 |         for i in 0..self.fragments_total as usize {
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
--
error: could not compile `mosh-network` (lib test) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `mosh-network` (lib) due to 2 previous errors
```

## log-clippy (last 80 lines)
```
  Downloaded zlib-rs v0.6.8
  Downloaded nix v0.31.3
  Downloaded libc v0.2.190
   Compiling libc v0.2.190
    Checking cfg-if v1.0.5
   Compiling getrandom v0.4.3
    Checking rand_core v0.10.1
   Compiling autocfg v1.5.1
   Compiling cfg_aliases v0.2.2
   Compiling nix v0.31.3
   Compiling memoffset v0.9.1
    Checking cpufeatures v0.3.1
    Checking bitflags v2.13.2
    Checking chacha20 v0.10.2
    Checking typenum v1.20.1
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.27
    Checking hybrid-array v0.4.15
   Compiling anyhow v1.0.104
   Compiling either v1.19.0
   Compiling itertools v0.14.0
    Checking inout v0.2.2
   Compiling syn v2.0.119
    Checking cmov v0.5.4
    Checking ctutils v0.4.3
    Checking dbl v0.5.0
    Checking zlib-rs v0.6.8
    Checking rand v0.10.4
    Checking crypto-common v0.2.2
    Checking cpubits v0.1.1
    Checking zeroize v1.9.1
    Checking bytes v1.12.1
    Checking cipher v0.5.2
    Checking aead v0.6.1
    Checking flate2 v1.1.10
    Checking aes v0.9.3
    Checking ocb3 v0.2.0
   Compiling prost-derive v0.14.4
    Checking mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking prost v0.14.4
    Checking mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking mosh-network v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
error: this `if` guard returns a bool literal and is followed by another
  --> crates/network/src/crypto/base64.rs:82:5
   |
82 | /     if b64[22] != b'=' || b64[23] != b'=' {
83 | |         return false;
84 | |     }
85 | |     true
   | |________^ help: you can reduce it to: `!(b64[22] != b'=' || b64[23] != b'=')`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#needless_bool
   = note: `-D clippy::needless-bool` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_bool)]`

error: the loop variable `i` is only used to index `self.fragments`
   --> crates/network/src/transport/transportfragment.rs:229:18
    |
229 |         for i in 0..self.fragments_total as usize {
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: for this index operation
   --> crates/network/src/transport/transportfragment.rs:231:40
    |
231 |             encoded.extend_from_slice(&self.fragments[i].contents);
    |                                        ^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#needless_range_loop
    = note: `-D clippy::needless-range-loop` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_range_loop)]`
help: consider using an iterator
    |
229 -         for i in 0..self.fragments_total as usize {
229 +         for <item> in self.fragments.iter().take(self.fragments_total as usize) {
    |

error: could not compile `mosh-network` (lib test) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `mosh-network` (lib) due to 2 previous errors
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
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.30s
     Running tests/prediction_e2e.rs (target/release/build/mosh-tests/e3b88e770e824eb3/out/prediction_e2e-e3b88e770e824eb3)

running 3 tests
warning: `package.homepage` is redundant with `package.repository`
 --> crates/network/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.02s
test keystroke_shows_at_once_with_prediction ... ok
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed has been running for over 60 seconds
test wrong_prediction_is_removed ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.24s


shard client: all passed
```

## log-test-client-ubuntu-24.04: failures
```
```

## log-test-client-ubuntu-24.04 (last 80 lines)
```
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.65s
     Running tests/prediction_e2e.rs (target/release/build/mosh-tests/b032c09f5de90542/out/prediction_e2e-b032c09f5de90542)

running 3 tests
warning: `package.homepage` is redundant with `package.repository`
 --> crates/network/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.01s
test unicode_input_with_prediction ... ok
test wrong_prediction_is_removed ... ok
test keystroke_shows_at_once_with_prediction has been running for over 60 seconds
test keystroke_shows_at_once_with_prediction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.23s


shard client: all passed
```

## log-test-conformance-ubuntu-24.04-arm: failures
```
```

## log-test-conformance-ubuntu-24.04-arm (last 80 lines)
```
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.39s
     Running tests/launcher.rs (target/release/build/mosh-tests/d5a68bebafc2dca6/out/launcher-d5a68bebafc2dca6)

running 9 tests
test launches_session_on_dual_stack_name ... ok
warning: `package.homepage` is redundant with `package.repository`
 --> crates/network/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.02s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test control_master_hint_and_remote_mode_reuses_socket ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.16s


shard conformance: all passed
```

## log-test-conformance-ubuntu-24.04: failures
```
```

## log-test-conformance-ubuntu-24.04 (last 80 lines)
```
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.92s
     Running tests/launcher.rs (target/release/build/mosh-tests/5dd6d93b6e63a798/out/launcher-5dd6d93b6e63a798)

running 9 tests
test launches_session_on_dual_stack_name ... ok
warning: `package.homepage` is redundant with `package.repository`
 --> crates/network/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.03s
test missing_server_is_reported ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test launches_session_with_user_at_ip_literal ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test control_master_hint_and_remote_mode_reuses_socket ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.26s


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
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.58s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=56474
Server now attached to client at 127.0.0.1:48189
Server now attached to client at 127.0.0.1:56474
elapsed 10.63s; client: status=done elapsed_ms=10624 packets_received=15 acked_ts_advance=10574; server: packets_received=50 acked_ts_advance=9697 remote_addr=127.0.0.1:56474; state 6459 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.63s
proxy stats: c2s forwarded=50 dropped=15 reordered=5 s2c forwarded=15 dropped=6 reordered=1
PASS loss30-rebind: converged in 10.63s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:60000
elapsed 10.38s; client: status=done elapsed_ms=10374 packets_received=42 acked_ts_advance=10032; server: packets_received=71 acked_ts_advance=9874 remote_addr=127.0.0.1:60000; state 6032 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.38s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=42 dropped=1 reordered=2
PASS loss5: converged in 10.38s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:35052
elapsed 11.04s; client: status=done elapsed_ms=11038 packets_received=11 acked_ts_advance=11014; server: packets_received=47 acked_ts_advance=9911 remote_addr=127.0.0.1:35052; state 6741 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.04s
proxy stats: c2s forwarded=47 dropped=35 reordered=4 s2c forwarded=11 dropped=10 reordered=2
PASS loss30-blackout: converged in 11.04s, 1 client address(es)
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
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=235

```

## log-test-scrollback-sh-ubuntu-24.04 (last 80 lines)
```
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.95s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=54415
Server now attached to client at 127.0.0.1:39906
Server now attached to client at 127.0.0.1:54415
elapsed 10.43s; client: status=done elapsed_ms=10428 packets_received=13 acked_ts_advance=10343; server: packets_received=48 acked_ts_advance=9410 remote_addr=127.0.0.1:54415; state 5993 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.43s
proxy stats: c2s forwarded=48 dropped=15 reordered=6 s2c forwarded=13 dropped=5 reordered=0
PASS loss30-rebind: converged in 10.43s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:46118
elapsed 10.42s; client: status=done elapsed_ms=10414 packets_received=44 acked_ts_advance=10043; server: packets_received=71 acked_ts_advance=9893 remote_addr=127.0.0.1:46118; state 6141 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.42s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=44 dropped=1 reordered=2
PASS loss5: converged in 10.42s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:49473
elapsed 10.43s; client: status=done elapsed_ms=10430 packets_received=11 acked_ts_advance=10082; server: packets_received=33 acked_ts_advance=9854 remote_addr=127.0.0.1:49473; state 6073 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.43s
proxy stats: c2s forwarded=33 dropped=48 reordered=5 s2c forwarded=11 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.43s, 1 client address(es)
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
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 5.49s
     Running tests/scrollback_hostile.rs (target/release/build/mosh-tests/435050ca82ebe5c3/out/scrollback_hostile-435050ca82ebe5c3)

running 1 test
warning: `package.homepage` is redundant with `package.repository`
 --> crates/network/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.14s


shard server: all passed
```

## log-test-server-ubuntu-24.04: failures
```
```

## log-test-server-ubuntu-24.04 (last 80 lines)
```
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
   Compiling mosh-tests v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tests)
    Finished `release` profile [optimized] target(s) in 6.70s
     Running tests/scrollback_hostile.rs (target/release/build/mosh-tests/1497a46ed5a18ad0/out/scrollback_hostile-1497a46ed5a18ad0)

running 1 test
warning: `package.homepage` is redundant with `package.repository`
 --> crates/network/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-network` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/terminal/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-terminal` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
  --> crates/mosh/Cargo.toml:11:1
   |
10 | repository.workspace = true
   | ----------
11 | homepage.workspace = true
   | ^^^^^^^^
   |
   = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh` (manifest) generated 1 warning
warning: `package.homepage` is redundant with `package.repository`
 --> crates/platform/Cargo.toml:8:1
  |
7 | repository.workspace = true
  | ----------
8 | homepage.workspace = true
  | ^^^^^^^^
  |
  = note: `cargo::redundant_homepage` is set to `warn` by default
help: consider removing `package.homepage`
warning: `mosh-platform` (manifest) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.02s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.22s


shard server: all passed
```
