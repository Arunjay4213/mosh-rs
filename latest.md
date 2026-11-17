result: failure
commit: 2ad84f9944f71bbcdec6a0c77adfefbde9cd2393
branch: dependabot/cargo/prost-0.14.6
pr: 64
subject: Bump prost from 0.14.4 to 0.14.6
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/44116013619
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
error: could not compile `mosh-network` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `mosh-network` (lib test) due to 2 previous errors
```

## log-clippy (last 80 lines)
```
  Downloaded nix v0.31.3
  Downloaded itertools v0.14.0
  Downloaded libc v0.2.190
   Compiling libc v0.2.190
    Checking cfg-if v1.0.5
   Compiling getrandom v0.4.3
    Checking rand_core v0.10.1
   Compiling autocfg v1.5.1
   Compiling cfg_aliases v0.2.2
   Compiling nix v0.31.3
   Compiling memoffset v0.9.1
    Checking bitflags v2.13.2
    Checking cpufeatures v0.3.1
    Checking chacha20 v0.10.2
    Checking typenum v1.20.1
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.27
   Compiling anyhow v1.0.104
   Compiling either v1.19.0
   Compiling syn v2.0.119
   Compiling itertools v0.14.0
    Checking hybrid-array v0.4.15
    Checking inout v0.2.2
    Checking rand v0.10.3
    Checking crypto-common v0.2.2
    Checking cmov v0.5.4
    Checking ctutils v0.4.3
    Checking dbl v0.5.0
    Checking cpubits v0.1.1
    Checking zlib-rs v0.6.8
    Checking cipher v0.5.2
    Checking aead v0.6.1
    Checking zeroize v1.9.1
    Checking bytes v1.12.1
    Checking aes v0.9.3
   Compiling prost-derive v0.14.6
    Checking flate2 v1.1.10
    Checking ocb3 v0.2.0
    Checking mosh-terminal v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-platform v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-tools v0.1.4 (/home/runner/work/mosh-rs/mosh-rs/tools)
    Checking prost v0.14.6
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

error: could not compile `mosh-network` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `mosh-network` (lib test) due to 2 previous errors
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
    Finished `release` profile [optimized] target(s) in 6.31s
     Running tests/prediction_e2e.rs (target/release/build/mosh-tests/5ad9110e8e0b45df/out/prediction_e2e-5ad9110e8e0b45df)

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

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.13s


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
    Finished `release` profile [optimized] target(s) in 6.58s
     Running tests/prediction_e2e.rs (target/release/build/mosh-tests/3677fb31fba85a03/out/prediction_e2e-3677fb31fba85a03)

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

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.38s


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
    Finished `release` profile [optimized] target(s) in 6.97s
     Running tests/launcher.rs (target/release/build/mosh-tests/10b7e0c157e20fe6/out/launcher-10b7e0c157e20fe6)

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

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.32s


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
    Finished `release` profile [optimized] target(s) in 5.67s
     Running tests/launcher.rs (target/release/build/mosh-tests/e0b67237e4e357d4/out/launcher-e0b67237e4e357d4)

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
test launches_session_with_user_at_ip_literal ... ok
test numeric_addresses_resolve_the_same_in_both_paths ... ok
test missing_server_is_reported ... ok
test proxy_resolves_canonical_name_like_mosh_pl ... ok
test windows_proxy_command_quoting ... ok
test launches_session_on_localhost_with_command ... ok
test control_master_hint_and_remote_mode_reuses_socket ... ok
test options_and_messages_match_mosh_pl ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.52s


shard conformance: all passed
```

## log-test-scrollback-sh-ubuntu-24.04-arm: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=242

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
    Finished `release` profile [optimized] target(s) in 5.85s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=43842
Server now attached to client at 127.0.0.1:54581
Server now attached to client at 127.0.0.1:43842
elapsed 10.94s; client: status=done elapsed_ms=10937 packets_received=16 acked_ts_advance=10867; server: packets_received=47 acked_ts_advance=9939 remote_addr=127.0.0.1:43842; state 6191 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.94s
proxy stats: c2s forwarded=47 dropped=15 reordered=4 s2c forwarded=16 dropped=6 reordered=2
PASS loss30-rebind: converged in 10.94s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:53251
elapsed 10.35s; client: status=done elapsed_ms=10351 packets_received=43 acked_ts_advance=9999; server: packets_received=71 acked_ts_advance=9394 remote_addr=127.0.0.1:53251; state 6146 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.35s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=43 dropped=1 reordered=2
PASS loss5: converged in 10.35s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:37913
elapsed 10.32s; client: status=done elapsed_ms=10322 packets_received=13 acked_ts_advance=9977; server: packets_received=40 acked_ts_advance=9749 remote_addr=127.0.0.1:37913; state 5976 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.33s
proxy stats: c2s forwarded=40 dropped=42 reordered=4 s2c forwarded=13 dropped=7 reordered=2
PASS loss30-blackout: converged in 10.32s, 1 client address(es)
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

## log-test-scrollback-sh-ubuntu-24.04: failures
```
# XFAIL: 2
# FAIL:  0
# XPASS: 0
# ERROR: 0
PASS upstream-check client=cxx server=cxx total=32 pass=30 xfail=2 fail=0 error=0 xpass=0 seconds=241

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
    Finished `release` profile [optimized] target(s) in 6.48s
--- loss30-rebind: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 5s: rebind -> ok rebind port=47735
Server now attached to client at 127.0.0.1:57838
Server now attached to client at 127.0.0.1:47735
elapsed 10.37s; client: status=done elapsed_ms=10363 packets_received=21 acked_ts_advance=9975; server: packets_received=48 acked_ts_advance=9794 remote_addr=127.0.0.1:47735; state 6284 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.37s
proxy stats: c2s forwarded=48 dropped=14 reordered=3 s2c forwarded=21 dropped=7 reordered=3
PASS loss30-rebind: converged in 10.37s, 2 client address(es)
--- loss5: proxy --loss 0.05 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
Server now attached to client at 127.0.0.1:59678
elapsed 10.40s; client: status=done elapsed_ms=10395 packets_received=40 acked_ts_advance=10054; server: packets_received=71 acked_ts_advance=9498 remote_addr=127.0.0.1:59678; state 6238 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.40s
proxy stats: c2s forwarded=71 dropped=7 reordered=7 s2c forwarded=40 dropped=1 reordered=2
PASS loss5: converged in 10.40s, 1 client address(es)
--- loss30-blackout: proxy --loss 0.30 --delay-ms 100 --jitter-ms 50 --reorder 0.10 --seed 7
control at 4s: blackout 3000 -> ok
Server now attached to client at 127.0.0.1:41192
elapsed 10.83s; client: status=done elapsed_ms=10831 packets_received=8 acked_ts_advance=10814; server: packets_received=35 acked_ts_advance=9607 remote_addr=127.0.0.1:41192; state 6296 bytes
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.83s
proxy stats: c2s forwarded=35 dropped=51 reordered=5 s2c forwarded=8 dropped=5 reordered=0
PASS loss30-blackout: converged in 10.83s, 1 client address(es)
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
    Finished `release` profile [optimized] target(s) in 5.91s
     Running tests/scrollback_hostile.rs (target/release/build/mosh-tests/c76b8f6d7846ff7c/out/scrollback_hostile-c76b8f6d7846ff7c)

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.18s


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
    Finished `release` profile [optimized] target(s) in 5.15s
     Running tests/scrollback_hostile.rs (target/release/build/mosh-tests/1df379235309cd75/out/scrollback_hostile-1df379235309cd75)

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
    Finished `release` profile [optimized] target(s) in 0.01s
test hostile_history_lines_stay_bounded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.15s


shard server: all passed
```
