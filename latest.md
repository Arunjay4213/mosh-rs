result: failure
commit: 637682981036377f284935f88d379a9699af1b15
subject: CI: capture job logs and publish results to the ci-status branch; add ci-status script
run: https://github.com/Arunjay4213/mosh-rs/actions/runs/38355990121
fmt: failure
clippy: failure
test: failure

## log-clippy (last 80 lines)
```
    Updating crates.io index
    Checking mosh-network v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/network)
    Checking mosh-platform v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/platform)
    Checking mosh-terminal v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/crates/terminal)
    Checking mosh-tools v0.1.0 (/home/runner/work/mosh-rs/mosh-rs/tools)
error: this comparison is always false
   --> crates/terminal/src/parser.rs:110:15
    |
110 |     } else if (c & 0xFC) == 0xF9 {
    |               ^^^^^^^^^^^^^^^^^^
    |
    = note: `0xfc` is missing bits contained in the compared constant `0xf9`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#bad_bit_mask
    = note: `#[deny(clippy::bad_bit_mask)]` on by default

error: could not compile `mosh-terminal` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

## log-fmt (last 80 lines)
```
     if proxy.is_some() {
-        assert!(server_port.is_some(), "MOSH_IMPAIR_PROXY needs MOSH_SYNC_SERVER_PORT (the proxy's target port)");
+        assert!(
+            server_port.is_some(),
+            "MOSH_IMPAIR_PROXY needs MOSH_SYNC_SERVER_PORT (the proxy's target port)"
+        );
     }
     let seed: u64 = match std::env::var("SYNC_SEED") {
         Ok(s) => s.parse().expect("SYNC_SEED is a u64"),
Diff in /home/runner/work/mosh-rs/mosh-rs/tests/tests/transport_sync.rs:83:
     let mut line = String::new();
     server_out.read_line(&mut line).expect("read MOSH CONNECT");
     let words: Vec<&str> = line.split_whitespace().collect();
-    assert!(words.len() == 4 && words[0] == "MOSH" && words[1] == "CONNECT", "bad server line {line:?}");
+    assert!(
+        words.len() == 4 && words[0] == "MOSH" && words[1] == "CONNECT",
+        "bad server line {line:?}"
+    );
     let (port, key) = (words[2].to_string(), words[3].to_string());
     let port_num: u16 = port.parse().expect("port");
-    assert!((60001..=60999).contains(&port_num) || server_port.is_some(), "port {port} outside mosh range");
+    assert!(
+        (60001..=60999).contains(&port_num) || server_port.is_some(),
+        "port {port} outside mosh range"
+    );
 
     let (ip, cport) = match &proxy {
         Some(p) => {
Diff in /home/runner/work/mosh-rs/mosh-rs/tests/tests/transport_sync.rs:110:
     /* closing the server's stdin tells it to report and exit */
     drop(server.stdin.take());
     let mut server_line = String::new();
-    server_out.read_line(&mut server_line).expect("read server report");
+    server_out
+        .read_line(&mut server_line)
+        .expect("read server report");
     let server_line = server_line.trim_end();
-    assert!(server.wait().expect("wait server").success(), "server failed");
+    assert!(
+        server.wait().expect("wait server").success(),
+        "server failed"
+    );
     let elapsed = start.elapsed();
 
     let client_log = field(client_line, "log");
Diff in /home/runner/work/mosh-rs/mosh-rs/tests/tests/transport_sync.rs:130:
         client_log.len() / 2,
     );
 
-    assert_eq!(field(client_line, "status"), "done", "client did not get everything acknowledged");
+    assert_eq!(
+        field(client_line, "status"),
+        "done",
+        "client did not get everything acknowledged"
+    );
     assert!(client.status.success(), "client failed");
     assert!(elapsed < Duration::from_secs(60), "took {elapsed:?}");
     assert!(!client_log.is_empty());
Diff in /home/runner/work/mosh-rs/mosh-rs/tests/tests/transport_sync.rs:137:
-    assert_eq!(server_remote, client_log, "server's received state differs from the client's state");
+    assert_eq!(
+        server_remote, client_log,
+        "server's received state differs from the client's state"
+    );
     let client_adv: u64 = field(client_line, "acked_ts_advance").parse().unwrap();
     let server_adv: u64 = field(server_line, "acked_ts_advance").parse().unwrap();
     assert!(client_adv > 0, "client's acked timestamp did not advance");
Diff in /home/runner/work/mosh-rs/mosh-rs/tests/tests/transport_sync.rs:141:
     assert!(server_adv > 0, "server's acked timestamp did not advance");
-    let addr: SocketAddr = field(server_line, "remote_addr").parse().expect("server remote_addr is an address");
-    assert!(addr.port() != 0 && !addr.ip().is_unspecified(), "bad remote addr {addr}");
+    let addr: SocketAddr = field(server_line, "remote_addr")
+        .parse()
+        .expect("server remote_addr is an address");
+    assert!(
+        addr.port() != 0 && !addr.ip().is_unspecified(),
+        "bad remote addr {addr}"
+    );
 }
 
```

## log-test-ubuntu-24.04-arm (last 80 lines)
```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-ad4d990e572312f3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-f093f1482b687e89)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tests-61d233b33c336685)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-4d0a991918ea36bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync.rs (target/release/deps/toy_sync-3ca43513607827d0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-154069677c0bb766)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-21d2b4854e3dfdcb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-75be59f05b5a6b32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-a3a42611cdc0a2ed)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
ref-crypto.cc:48:10: fatal error: src/crypto/crypto.h: No such file or directory
   48 | #include "src/crypto/crypto.h"
      |          ^~~~~~~~~~~~~~~~~~~~~
compilation terminated.
make: *** [Makefile:20: ref-crypto.o] Error 1
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... FAILED

failures:

---- crypto_conformance stdout ----

thread 'crypto_conformance' (2862) panicked at tests/tests/crypto_conformance.rs:147:5:
make -C ref/ref-crypto failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    crypto_conformance

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.16s

error: test failed, to rerun pass `-p mosh-tests --test crypto_conformance`
```

## log-test-ubuntu-latest (last 80 lines)
```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_platform-6dfede581caad4b7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_terminal-936453b01bc39a3a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/release/deps/mosh_tests-de153860463d1424)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/crypto-check.rs (target/release/deps/crypto_check-5f3c6c303bd92025)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/toy-sync.rs (target/release/deps/toy_sync-a7e8633597707ea4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tty-probe.rs (target/release/deps/tty_probe-c683db55508d9b09)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-check.rs (target/release/deps/vt_check-c53c7a329874620b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/vt-fuzz.rs (target/release/deps/vt_fuzz-dd51831fcbd707af)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/crypto_conformance.rs (target/release/deps/crypto_conformance-2c4bb4d6dc0b5847)

running 1 test
make: Entering directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
c++ -std=gnu++17 -DHAVE_CONFIG_H -I../../upstream -I../../upstream/src/include -I../../upstream/src/crypto -I../../upstream/src/util -I../../upstream/src/network -I../../upstream/src/tests -g -O2 -Wall -c -o ref-crypto.o ref-crypto.cc
ref-crypto.cc:48:10: fatal error: src/crypto/crypto.h: No such file or directory
   48 | #include "src/crypto/crypto.h"
      |          ^~~~~~~~~~~~~~~~~~~~~
compilation terminated.
make: *** [Makefile:20: ref-crypto.o] Error 1
make: Leaving directory '/home/runner/work/mosh-rs/mosh-rs/ref/ref-crypto'
test crypto_conformance ... FAILED

failures:

---- crypto_conformance stdout ----

thread 'crypto_conformance' (2962) panicked at tests/tests/crypto_conformance.rs:147:5:
make -C ref/ref-crypto failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    crypto_conformance

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.10s

error: test failed, to rerun pass `-p mosh-tests --test crypto_conformance`
```
