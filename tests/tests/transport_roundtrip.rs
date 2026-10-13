// Mosh: the mobile shell
// Copyright 2012 Keith Winstein
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// In addition, as a special exception, the copyright holders give
// permission to link the code of portions of this program with the
// OpenSSL library under certain conditions as described in each
// individual source file, and distribute linked combinations including
// the two.
//
// You must obey the GNU General Public License in all respects for all
// of the code used other than OpenSSL. If you modify file(s) with this
// exception, you may extend this exception to your version of the
// file(s), but you are not obligated to do so. If you do not wish to do
// so, delete this exception statement from your version. If you delete
// this exception statement from all source files in the program, then
// also delete it here.
// Ported from: (none; end-to-end check of proto, compressor and fragment layers)

//! Protocol buffers, zlib compression, and fragmentation round trips.

use mosh_network::proto::{client_buffers, host_buffers, transport_buffers};
use mosh_network::transport::compressor::{compress_str, uncompress_str, BUFFER_SIZE};
use mosh_network::transport::transportfragment::{Fragment, FragmentAssembly, Fragmenter};
use mosh_network::{Connection, Session};
use mosh_network::prost::Message;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn unhex(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex")).collect()
}

#[test]
fn instruction_wire_format() {
    /* TransportBuffers::Instruction with every field set; the expected bytes
       are the proto2 encoding in field-number order, as C++ protobuf emits it */
    let inst = transport_buffers::Instruction {
        protocol_version: Some(2),
        old_num: Some(1),
        new_num: Some(300),
        ack_num: Some(0),
        throwaway_num: Some(u64::MAX),
        diff: Some(b"ab".to_vec()),
        chaff: Some(Vec::new()),
    };
    let bytes = inst.encode_to_vec();
    assert_eq!(bytes, unhex("0802 1001 18ac02 2000 28ffffffffffffffffff01 32026162 3a00"));
    assert_eq!(transport_buffers::Instruction::decode(bytes.as_slice()).unwrap(), inst);

    /* absent fields are not encoded and read back as defaults */
    let empty = transport_buffers::Instruction::default();
    assert!(empty.encode_to_vec().is_empty());
    assert_eq!(empty.old_num(), 0);
    assert_eq!(empty.diff(), b"");

    /* unknown field 9 (length-delimited) is skipped on decode */
    let mut with_unknown = bytes.clone();
    with_unknown.extend_from_slice(&unhex("4a03 78797a"));
    with_unknown.splice(0..0, unhex("4a00"));
    assert_eq!(transport_buffers::Instruction::decode(with_unknown.as_slice()).unwrap(), inst);

    /* HostBuffers: extensions 2, 3, 7 inside a HostMessage */
    let host = host_buffers::HostMessage {
        instruction: vec![
            host_buffers::Instruction {
                hostbytes: Some(host_buffers::HostBytes {
                    hoststring: Some(b"hi".to_vec()),
                }),
                resize: None,
                echoack: None,
            },
            host_buffers::Instruction {
                hostbytes: None,
                resize: Some(host_buffers::ResizeMessage {
                    width: Some(80),
                    height: Some(24),
                }),
                echoack: Some(host_buffers::EchoAck { echo_ack_num: Some(5) }),
            },
        ],
    };
    let bytes = host.encode_to_vec();
    assert_eq!(bytes, unhex("0a06 1204 22026869 0a0a 1a04 28503018 3a02 4005"));
    assert_eq!(host_buffers::HostMessage::decode(bytes.as_slice()).unwrap(), host);

    /* the same with an unknown field 9 inside an Instruction and inside HostMessage */
    let with_unknown = unhex("0a0a 1204 22026869 4a02 0102 0a0a 1a04 28503018 3a02 4005 4a01 ff");
    assert_eq!(host_buffers::HostMessage::decode(with_unknown.as_slice()).unwrap(), host);

    /* ClientBuffers: extensions 2 and 3 */
    let user = client_buffers::UserMessage {
        instruction: vec![
            client_buffers::Instruction {
                keystroke: Some(client_buffers::Keystroke {
                    keys: Some(b"ls\r".to_vec()),
                }),
                resize: None,
            },
            client_buffers::Instruction {
                keystroke: None,
                resize: Some(client_buffers::ResizeMessage {
                    width: Some(132),
                    height: Some(-1),
                }),
            },
        ],
    };
    let bytes = user.encode_to_vec();
    assert_eq!(
        bytes,
        unhex("0a07 1205 2203 6c730d 0a10 1a0e 288401 30ffffffffffffffffff01")
    );
    assert_eq!(client_buffers::UserMessage::decode(bytes.as_slice()).unwrap(), user);
    let mut with_unknown = unhex("0a0b 4a02 aabb 1205 2203 6c730d");
    with_unknown.extend_from_slice(&unhex("0a10 1a0e 288401 30ffffffffffffffffff01"));
    assert_eq!(client_buffers::UserMessage::decode(with_unknown.as_slice()).unwrap(), user);
}

#[test]
fn compressor_round_trip() {
    let mut rng = StdRng::seed_from_u64(1);
    let mut random = vec![0u8; 1 << 20];
    rng.fill_bytes(&mut random);
    let inputs: Vec<Vec<u8>> = vec![
        Vec::new(),
        b"a".to_vec(),
        b"\x1b[H\x1b[2J".repeat(1000),
        random,
        vec![0u8; BUFFER_SIZE],
    ];
    for input in &inputs {
        let c = compress_str(input).unwrap();
        assert_eq!(&uncompress_str(&c).unwrap(), input, "round trip of {} bytes", input.len());
        /* bytes after the zlib stream are ignored, as zlib's uncompress() does */
        let mut trailing = c.clone();
        trailing.extend_from_slice(b"junk");
        assert_eq!(&uncompress_str(&trailing).unwrap(), input);
        /* truncated input is rejected */
        assert!(uncompress_str(&c[..c.len() - 1]).is_err(), "truncated stream accepted");
    }
    /* output larger than the 4 MiB buffer is rejected */
    let too_big = compress_str(&vec![0u8; BUFFER_SIZE + 1]).unwrap();
    assert!(uncompress_str(&too_big).is_err(), "oversize output accepted");
    /* garbage is rejected */
    assert!(uncompress_str(b"not zlib at all").is_err());
}

/// The fragment MTU a client over IPv4 uses (TransportSender::send_in_fragments).
fn fragment_mtu() -> usize {
    let conn = Connection::new_client("AAAAAAAAAAAAAAAAAAAAAA", "127.0.0.1", "60001").unwrap();
    (conn.get_mtu() - Connection::ADDED_BYTES - Session::ADDED_BYTES as i32) as usize
}

fn instruction(rng: &mut StdRng, num: u64, diff_len: usize) -> transport_buffers::Instruction {
    let mut diff = vec![0u8; diff_len];
    rng.fill_bytes(&mut diff); /* incompressible, so the diff length sets the fragment count */
    transport_buffers::Instruction {
        protocol_version: Some(2),
        old_num: Some(num),
        new_num: Some(num + 1),
        ack_num: Some(num),
        throwaway_num: Some(num),
        diff: Some(diff),
        chaff: Some(vec![7u8; (num % 17) as usize]),
    }
}

#[test]
fn fragment_round_trip() {
    let mtu = fragment_mtu();
    assert_eq!(mtu, 1280 - 28 - 12 - 16);
    let mut rng = StdRng::seed_from_u64(2);
    let mut fragmenter = Fragmenter::new();
    let mut assembly = FragmentAssembly::new();

    for (n, size) in [0, 1, mtu - 1, mtu, 10 * mtu].into_iter().enumerate() {
        let inst = instruction(&mut rng, n as u64, size);
        let frags = fragmenter.make_fragments(&inst, mtu).unwrap();
        let expected_count = size / (mtu - Fragment::FRAG_HEADER_LEN) + 1;
        assert!(
            frags.len() == expected_count || frags.len() == expected_count + 1,
            "size {size}: {} fragments",
            frags.len()
        );
        for (i, f) in frags.iter().enumerate() {
            let wire = f.tostring();
            assert!(wire.len() <= mtu, "fragment longer than MTU");
            /* header: u64 id BE, then u16 fragment number BE with the top bit = final */
            assert_eq!(wire[0..8], f.id.to_be_bytes());
            let num = u16::from_be_bytes([wire[8], wire[9]]);
            assert_eq!(num & 0x7fff, i as u16);
            assert_eq!(num & 0x8000 != 0, i == frags.len() - 1);
            assert_eq!(&Fragment::from_bytes(&wire), f);
        }

        /* deliver over the wire format in reverse order; every fragment but the
           completing one (fragment 0) is delivered twice, and the first one again
           just before the end */
        let received: Vec<Fragment> = frags.iter().rev().map(|f| Fragment::from_bytes(&f.tostring())).collect();
        let (completing, rest) = received.split_last().expect("at least one fragment");
        let mut deliveries: Vec<&Fragment> = rest.iter().flat_map(|f| [f, f]).collect();
        if let Some(first) = rest.first() {
            deliveries.push(first);
        }
        deliveries.push(completing);
        let mut done = 0;
        for f in deliveries {
            if assembly.add_fragment(f) {
                done += 1;
                assert_eq!(assembly.get_assembly().unwrap(), inst, "size {size}");
            }
        }
        assert_eq!(done, 1, "size {size}: assembled {done} times");

        /* a late duplicate of the completing fragment: upstream starts a new
           assembly, which completes again only for a one-fragment instruction
           (Transport::recv then drops it because new_num is already known) */
        let again = assembly.add_fragment(completing);
        assert_eq!(again, frags.len() == 1, "size {size}: late duplicate");
        if again {
            assert_eq!(assembly.get_assembly().unwrap(), inst);
        }
        println!("size {size}: {} fragments", frags.len());
    }

    /* in order, with an unrelated stale fragment in between that starts a new id */
    let inst = instruction(&mut rng, 100, 3 * mtu);
    let frags = fragmenter.make_fragments(&inst, mtu).unwrap();
    assert!(frags.len() >= 3);
    let mut assembly = FragmentAssembly::new();
    assert!(!assembly.add_fragment(&frags[0]));
    assert!(!assembly.add_fragment(&frags[1]));
    for f in &frags {
        if assembly.add_fragment(f) {
            assert_eq!(assembly.get_assembly().unwrap(), inst);
        }
    }

    /* a retransmission of the same instruction keeps its id; a new one gets the next id */
    let again = fragmenter.make_fragments(&inst, mtu).unwrap();
    assert_eq!(again[0].id, frags[0].id);
    let next = fragmenter.make_fragments(&instruction(&mut rng, 101, 1), mtu).unwrap();
    assert_eq!(next[0].id, frags[0].id + 1);
}
