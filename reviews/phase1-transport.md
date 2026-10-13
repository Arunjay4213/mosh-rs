# Phase 1 review: transport sender, transport, network, fragments, compressor, timestamp, crypto, proto

Reviewed tree: commits d9c8289 through 14e65f2 (agent A pushed 77311f8 and 14e65f2 while the review ran; agent B pushed f01fc0d and c182b85 after it).

Reviewer: independent review agent, 2026-10-13.
Method: each Rust file was read function by function beside the upstream C++ it was ported from.
For every function: constants, comparison operators and boundaries, order of operations and side effects, integer widths and wrap behavior, early returns, size 0/1/max behavior, error paths.
Allowed differences (PLAN section 9, `Result` for exceptions, indices for pointers, `SocketAddr` for `Addr`, the 1e9 ms clock offset, no ECN/TOS on Windows, non-blocking sockets instead of `MSG_DONTWAIT`, std-only sockets on Windows, the `State` trait without `compare`) are not listed.

Files reviewed (Rust against upstream):

- `crates/network/src/transport/transportsender.rs` against `upstream/src/network/transportsender.h`, `transportsender-impl.h`
- `crates/network/src/transport/networktransport.rs` against `upstream/src/network/networktransport.h`, `networktransport-impl.h`
- `crates/network/src/transport/transportstate.rs` against `upstream/src/network/transportstate.h`
- `crates/network/src/transport/network.rs` against `upstream/src/network/network.h`, `network.cc`
- `crates/network/src/transport/transportfragment.rs` against `upstream/src/network/transportfragment.h`, `transportfragment.cc`
- `crates/network/src/transport/compressor.rs` against `upstream/src/network/compressor.h`, `compressor.cc`
- `crates/network/src/timestamp.rs` against `upstream/src/util/timestamp.cc`
- `crates/network/src/crypto/mod.rs` against `upstream/src/crypto/crypto.h`, `crypto.cc` (plus `base64.rs` against `base64.cc`, `prng.rs` against `prng.h`, `ae.rs` against the `ae.h` interface)
- `crates/network/src/proto.rs` against `upstream/src/protobufs/transportinstruction.proto`, `userinput.proto`, `hostinput.proto`

Checked and found identical (no finding): all timing constants (`SEND_INTERVAL_MIN/MAX`, `ACK_INTERVAL`, `ACK_DELAY`, `SHUTDOWN_RETRIES`, `ACTIVE_RETRY_TIMEOUT`, `SEND_MINDELAY = 8`), `send_interval()` (ceil of SRTT/2, clamped), `calculate_timers()` including the three-way `next_send_time` choice and the shutdown speed-up, `wait_time()`, `tick()`, `send_empty_ack()`, `send_to_receiver()` (new_num choice, timestamp refresh on resend, `assumed_receiver_state = back`), `update_assumed_receiver_state()` (`now - ts < timeout + ACK_DELAY`), `rationalize_states()`, `make_chaff()` (0..16 bytes), `send_in_fragments()` (field values, `shutdown_tries` incremented before fragmenting, MTU minus 12 minus 16), `add_sent_state()` (erase the element 16 from the end once the list exceeds 32; the index adjustment keeps the same element as upstream's list iterator), `process_acknowledgment_through()`, `shutdown_ack_timed_out()`, `attempt_prospective_resend_optimization()` (including the size_t wrap in `resend - proposed < 100`), the receiver in `Transport::recv()` (version check, ack processing, duplicate new_num, missing old_num, throwaway, the 1024 queue limit with the 15 s quench timer, sorted insert, `set_ack_num`, `remote_heard`, `set_data_ack` only for non-empty diffs), `get_remote_diff()`, `Packet` layout (direction bit 63, big-endian 16-bit timestamp and reply), `new_packet()` (1000 ms hold window, reply advanced by hold time, 16-bit wrap), `hop_port()`/`prune_sockets()` (`MAX_OLD_SOCKET_AGE`, `MAX_PORTS_OPEN`), `send()` (EMSGSIZE fallback to 500, `SERVER_ASSOCIATION_TIMEOUT`, `PORT_HOP_INTERVAL` on both last_port_choice and last_roundtrip_success), `recv()`/`recv_one()` (direction dos check, out-of-order seq returned but not used, `expected_receiver_seq = seq + 1`, saved timestamp and the 500 ms ECN penalty, RTT estimator with first-sample special case and alpha 1/8 beta 1/4 and the 5000 ms cutoff, server-only roaming), `timeout()` (SRTT + 4 RTTVAR clamped to 50..1000), `try_bind()` (60001..60999 default, port range parsing, IPV6_V6ONLY off for `::`), `parse_portrange()`, `timestamp16()`/`timestamp_diff()`, fragment header layout and the 0x8000 final bit, `FragmentAssembly::add_fragment()`/`get_assembly()`, `Fragmenter::make_fragments()` (instruction id bump rule, MTU minus 10), `Base64Key` parsing and printing, `Nonce` layout, `Session::encrypt()`/`decrypt()` (24-byte minimum, 2^47 block limit, tag appended), `unique()` wrap check, all protobuf field numbers, wire types and `optional` semantics (extensions encoded as plain fields with the same numbers).

A second reviewer pass independently re-read `transportsender.rs` in full against `transportsender-impl.h`, `networktransport.rs` against `networktransport-impl.h`, `Connection::send`/`recv`/`recv_one`/`new_packet`/`hop_port`/`prune_sockets`/`try_bind`/`timeout`/`parse_portrange`, `timestamp16`/`timestamp_diff`, `Packet`, `Session::encrypt`/`decrypt`, `Nonce`, the compressor, `proto.rs` and the fragment code, and confirmed the wildcard bind resolves through glibc `getaddrinfo` on unix so the server picks the same address family (AF_INET first on devbox) and therefore the same MTU as upstream.

### N-1  severity: low
file: crates/network/src/crypto/mod.rs:314
upstream: upstream/src/crypto/crypto.cc:259
deviation: `Session::decrypt` returns a new error "Ciphertext too long." when `body_len > RECEIVE_MTU`; upstream has `assert( body_len <= ciphertext_buffer.len() )` (abort, or buffer overflow with NDEBUG). The code comment cites "PLAN.md section 9 item 8", which does not exist.
effect: Unreachable through the network path (the receive buffer is `RECEIVE_MTU` = 2048 bytes and oversize datagrams are rejected before decrypt), so no wire or user-visible difference. Only a direct caller of `decrypt` with more than 2048 bytes (for example the crypto-check oracle) would see an error where upstream aborts. The comment should be fixed or the check removed so the deviation is not mistaken for an approved one.
status: answered. PLAN.md section 9 item 8 was added in f3cec0d while this review ran; it lists oversized ciphertext handed to Session::decrypt, so the comment now cites a real item.

### N-2  severity: low
file: crates/network/src/transport/networktransport.rs:178
upstream: upstream/src/network/networktransport-impl.h:135
deviation: After `process_throwaway_until()`, the Rust port looks the reference state up again and silently drops the packet if it is gone; upstream dereferences the `reference_state` iterator found before the throwaway, which is a dangling iterator if `throwaway_num > old_num`.
effect: Only reachable with a non-conforming or malicious peer (an honest sender always has `old_num >= throwaway_num`, since `old_num` is `assumed_receiver_state->num` and `throwaway_num` is `sent_states.front().num`). Upstream behavior there is undefined; the Rust behavior is a safe drop. No difference for real peers.
status: answered. PLAN.md section 9 item 9 (f3cec0d) lists the reference-state re-lookup as an allowed deviation that replaces upstream undefined behavior.

### N-3  severity: low
file: crates/network/src/transport/compressor.rs:47
upstream: upstream/src/network/compressor.cc:40
deviation: `compress_str` uses flate2 with the `zlib-rs` backend at `Compression::default()` (level 6, same as `Z_DEFAULT_COMPRESSION`), but zlib-rs does not guarantee byte-identical deflate output to the system zlib that upstream links.
effect: The compressed diff for the same `Instruction` can differ in length from upstream's by a few bytes, so fragment boundaries and packet sizes may differ from what a C++ peer would have produced. Both sides decompress each other's streams correctly (standard zlib format), so sync is unaffected. Observable only as packet length differences on the wire.
status: answered, no change. PLAN.md section 2 chose flate2 with the zlib-rs backend; peers only need a valid zlib stream, which any implementation decodes, so packet sizes may differ from a C++ peer but nothing on the wire breaks.

### N-4  severity: low
file: crates/network/src/transport/transportfragment.rs:184
upstream: upstream/src/network/transportfragment.cc:141
deviation: `get_assembly` panics via `expect` when the decompressed bytes do not parse as an `Instruction`; upstream `fatal_assert` aborts. Both terminate the process, but the Rust panic unwinds (`panic = "unwind"` in the release profile) and will run the client's panic hook (terminal restore) where upstream prints the assertion and calls `abort()`.
effect: Same outcome (session dies on a malformed-but-authenticated instruction, which only a peer holding the key can produce); only the exit path differs. Not reachable from a correct peer.
status: fixed in 77311f8. get_assembly maps the protobuf decode error to a dos_detected error (`rg -n "expect\(" crates/network/src/transport/transportfragment.rs` finds nothing).

### N-5  severity: low
file: crates/network/src/transport/transportfragment.rs:78, 169, 187, 197; crates/network/src/transport/networktransport.rs:245
upstream: upstream/src/network/transportfragment.cc:79, 105, 117, 122; upstream/src/network/networktransport-impl.h:191
deviation: Five process-killing checks became recoverable errors: `Fragment::from_bytes` on a datagram shorter than 10 bytes (upstream `fatal_assert`), the three `assert`s in `FragmentAssembly::add_fragment` (duplicate fragment with different contents, a fragment numbered past the final one, arrived count above total), and `process_throwaway_until` when `throwaway_num` is above every received state (upstream erases all states and then `fatal_assert`s).
Upstream is built with `-g -O2` and no `NDEBUG`, so all of these abort there; the Rust port returns a `dos_detected` error, resets the fragment assembly (or leaves `received_states` untouched) and the session continues.
effect: Only a peer holding the session key can trigger any of them (an honest sender never produces these inputs), so no difference for real peers.
When triggered, upstream's process dies; the Rust transport returns a `dos_detected` error to its caller instead, and what the user then sees depends on how the (not yet ported) frontend handles that error.
status: answered. PLAN.md section 9 item 8 (f3cec0d) lists exactly these five checks as errors that drop the packet.

files reviewed: 9, findings: high 0 medium 0 low 5
