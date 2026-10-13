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
// Ported from: upstream/src/network/network.h, upstream/src/network/network.cc

use std::collections::VecDeque;
use std::fmt;
use std::net::{SocketAddr, UdpSocket};

pub use mosh_platform::socket::SocketHandle;
use mosh_platform::socket::{self as sys, last_errno, strerror, SysError};

use super::{dos_detected, Error};
use crate::crypto::{unique, Base64Key, CryptoError, Message, Nonce, Session};
use crate::timestamp::frozen_timestamp;

pub const MOSH_PROTOCOL_VERSION: u32 = 2; /* bumped for echo-ack */

pub fn timestamp() -> u64 {
    frozen_timestamp()
}

pub fn timestamp16() -> u16 {
    let mut ts = (timestamp() % 65536) as u16;
    if ts == u16::MAX {
        ts = ts.wrapping_add(1);
    }
    ts
}

pub fn timestamp_diff(tsnew: u16, tsold: u16) -> u16 {
    let mut diff = tsnew as i32 - tsold as i32;
    if diff < 0 {
        diff += 65536;
    }
    assert!(diff >= 0);
    assert!(diff <= 65535);
    diff as u16
}

/// `Network::NetworkException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkError {
    pub function: String,
    pub the_errno: i32,
}

impl NetworkError {
    pub fn new(s_function: impl Into<String>, s_errno: i32) -> NetworkError {
        NetworkError {
            function: s_function.into(),
            the_errno: s_errno,
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.function, strerror(self.the_errno))
    }
}

impl std::error::Error for NetworkError {}

impl From<SysError> for NetworkError {
    fn from(e: SysError) -> NetworkError {
        NetworkError::new(e.function, e.errno)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    ToServer = 0,
    ToClient = 1,
}

const DIRECTION_MASK: u64 = 1u64 << 63;
const SEQUENCE_MASK: u64 = u64::MAX ^ DIRECTION_MASK;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    pub seq: u64,
    pub direction: Direction,
    pub timestamp: u16,
    pub timestamp_reply: u16,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(
        s_direction: Direction,
        s_timestamp: u16,
        s_timestamp_reply: u16,
        s_payload: &[u8],
    ) -> Result<Packet, CryptoError> {
        Ok(Packet {
            seq: unique()?,
            direction: s_direction,
            timestamp: s_timestamp,
            timestamp_reply: s_timestamp_reply,
            payload: s_payload.to_vec(),
        })
    }

    /// Read in packet
    pub fn from_message(message: &Message) -> Result<Packet, CryptoError> {
        if message.text.len() < 2 * 2 {
            return Err(dos_detected(
                "message.text.size() >= 2 * sizeof( uint16_t )",
                "Packet",
            ));
        }
        let seq = message.nonce.val() & SEQUENCE_MASK;
        let direction = if message.nonce.val() & DIRECTION_MASK != 0 {
            Direction::ToClient
        } else {
            Direction::ToServer
        };
        let data = &message.text;
        Ok(Packet {
            seq,
            direction,
            timestamp: u16::from_be_bytes([data[0], data[1]]),
            timestamp_reply: u16::from_be_bytes([data[2], data[3]]),
            payload: data[4..].to_vec(),
        })
    }

    /// Output from packet
    pub fn to_message(&self) -> Message {
        let direction_seq =
            (((self.direction == Direction::ToClient) as u64) << 63) | (self.seq & SEQUENCE_MASK);
        let mut text = Vec::with_capacity(4 + self.payload.len());
        text.extend_from_slice(&self.timestamp.to_be_bytes());
        text.extend_from_slice(&self.timestamp_reply.to_be_bytes());
        text.extend_from_slice(&self.payload);
        Message::new(Nonce::new(direction_seq), text)
    }
}

pub struct Connection {
    socks: VecDeque<UdpSocket>,
    has_remote_addr: bool,
    remote_addr: Option<SocketAddr>,

    server: bool,

    mtu: i32, /* application datagram MTU */

    key: Base64Key,
    session: Session,

    direction: Direction,
    saved_timestamp: u16,
    saved_timestamp_received_at: u64,
    expected_receiver_seq: u64,

    last_heard: u64,
    last_port_choice: u64,
    last_roundtrip_success: u64, /* transport layer needs to tell us this */

    rtt_hit: bool,
    srtt: f64,
    rttvar: f64,

    /* Error from send()/sendto(). */
    send_error: String,
}

impl Connection {
    /*
     * For IPv4, guess the typical (minimum) header length;
     * fragmentation is not dangerous, just inefficient.
     */
    const IPV4_HEADER_LEN: i32 = 20 /* base IP header */ + 8 /* UDP */;

    /*
     * For IPv6, we don't want to ever have MTU issues, so make a
     * conservative guess about header size.
     */
    const IPV6_HEADER_LEN: i32 = 40 /* base IPv6 header */ + 16 /* 2 minimum-sized extension headers */ + 8 /* UDP */;

    /* Application datagram MTU. For constructors and fallback. */
    const DEFAULT_SEND_MTU: i32 = 500;

    /*
     * IPv4 MTU. Don't use full Ethernet-derived MTU,
     * mobile networks have high tunneling overhead.
     *
     * As of July 2016, VPN traffic over Amtrak Acela wifi seems to be
     * dropped if tunnelled packets are 1320 bytes or larger.  Use a
     * 1280-byte IPv4 MTU for now.
     *
     * We may have to implement ICMP-less PMTUD (RFC 4821) eventually.
     */
    const DEFAULT_IPV4_MTU: i32 = 1280;

    /* IPv6 MTU. Use the guaranteed minimum to avoid fragmentation. */
    const DEFAULT_IPV6_MTU: i32 = 1280;

    const MIN_RTO: u64 = 50; /* ms */
    const MAX_RTO: u64 = 1000; /* ms */

    const PORT_RANGE_LOW: i32 = 60001;
    const PORT_RANGE_HIGH: i32 = 60999;

    const SERVER_ASSOCIATION_TIMEOUT: u64 = 40000;
    const PORT_HOP_INTERVAL: u64 = 10000;

    const MAX_PORTS_OPEN: usize = 10;
    const MAX_OLD_SOCKET_AGE: u64 = 60000;

    const CONGESTION_TIMESTAMP_PENALTY: u16 = 500; /* ms */

    /* Network transport overhead. */
    pub const ADDED_BYTES: i32 = 8 /* seqno/nonce */ + 4 /* timestamps */;

    fn blank(server: bool, key: Base64Key) -> Connection {
        Connection {
            socks: VecDeque::new(),
            has_remote_addr: false,
            remote_addr: None,
            server,
            mtu: Connection::DEFAULT_SEND_MTU,
            key,
            session: Session::new(key),
            direction: if server {
                Direction::ToClient
            } else {
                Direction::ToServer
            },
            saved_timestamp: u16::MAX,
            saved_timestamp_received_at: 0,
            expected_receiver_seq: 0,
            last_heard: u64::MAX,
            last_port_choice: u64::MAX,
            last_roundtrip_success: u64::MAX,
            rtt_hit: false,
            srtt: 1000.0,
            rttvar: 500.0,
            send_error: String::new(),
        }
    }

    /// Server constructor.
    pub fn new_server(
        desired_ip: Option<&str>,
        desired_port: Option<&str>,
    ) -> Result<Connection, Error> {
        let mut c = Connection::blank(true, Base64Key::new()?);
        c.setup();

        /* The mosh wrapper always gives an IP request, in order
        to deal with multihomed servers. The port is optional. */

        /* If an IP request is given, we try to bind to that IP, but we also
        try INADDR_ANY. If a port request is given, we bind only to that port. */

        /* convert port numbers */
        let mut desired_port_low = -1;
        let mut desired_port_high = -1;

        if let Some(desired_port) = desired_port {
            if !Connection::parse_portrange(
                desired_port,
                &mut desired_port_low,
                &mut desired_port_high,
            ) {
                return Err(NetworkError::new("Invalid port range", 0).into());
            }
        }

        /* try to bind to desired IP first */
        if let Some(desired_ip) = desired_ip {
            match c.try_bind(Some(desired_ip), desired_port_low, desired_port_high) {
                Ok(true) => return Ok(c),
                Ok(false) => {}
                Err(e) => eprintln!("Error binding to IP {desired_ip}: {e}"),
            }
        }

        /* now try any local interface */
        match c.try_bind(None, desired_port_low, desired_port_high) {
            Ok(true) => return Ok(c),
            Ok(false) => {}
            Err(e) => {
                eprintln!("Error binding to any interface: {e}");
                return Err(e.into()); /* this time it's fatal */
            }
        }

        Err(NetworkError::new("Could not bind", last_errno()).into())
    }

    fn try_bind(
        &mut self,
        addr: Option<&str>,
        port_low: i32,
        port_high: i32,
    ) -> Result<bool, NetworkError> {
        let mut local_addr = sys::resolve(addr, "0", true)?;

        let mut search_low = Connection::PORT_RANGE_LOW;
        let mut search_high = Connection::PORT_RANGE_HIGH;

        if port_low != -1 {
            /* low port preference */
            search_low = port_low;
        }
        if port_high != -1 {
            /* high port preference */
            search_high = port_high;
        }

        self.socks.push_back(sys::new_socket(local_addr.is_ipv6())?);

        let mut saved_errno = 0;
        for i in search_low..=search_high {
            local_addr.set_port(i as u16);

            let sock = self.socks.back_mut().expect("socket pushed above");
            match sys::bind(sock, &local_addr) {
                Ok(()) => {
                    self.set_mtu(local_addr.is_ipv6());
                    return Ok(true);
                }
                Err(e) => saved_errno = e,
            } // else fallthrough to below code, on last iteration.
        }

        self.socks.pop_back();

        eprintln!(
            "Failed binding to {}:{}",
            local_addr.ip(),
            local_addr.port()
        );
        Err(NetworkError::new("bind", saved_errno))
    }

    /// Client constructor.
    pub fn new_client(key_str: &str, ip: &str, port: &str) -> Result<Connection, Error> {
        let mut c = Connection::blank(false, Base64Key::from_printable(key_str)?);
        c.setup();

        /* associate socket with remote host and port */
        let remote_addr = sys::resolve(Some(ip), port, false)?;
        c.remote_addr = Some(remote_addr);
        c.has_remote_addr = true;

        c.socks.push_back(sys::new_socket(remote_addr.is_ipv6())?);

        c.set_mtu(remote_addr.is_ipv6());
        Ok(c)
    }

    fn set_mtu(&mut self, ipv6: bool) {
        self.mtu = if ipv6 {
            Connection::DEFAULT_IPV6_MTU - Connection::IPV6_HEADER_LEN
        } else {
            Connection::DEFAULT_IPV4_MTU - Connection::IPV4_HEADER_LEN
        };
    }

    fn setup(&mut self) {
        self.last_port_choice = timestamp();
    }

    fn sock(&self) -> &UdpSocket {
        assert!(!self.socks.is_empty());
        self.socks.back().expect("socks not empty")
    }

    fn new_packet(&mut self, s_payload: &[u8]) -> Result<Packet, CryptoError> {
        let mut outgoing_timestamp_reply = u16::MAX;

        let now = timestamp();

        if now.wrapping_sub(self.saved_timestamp_received_at) < 1000 {
            /* we have a recent received timestamp */
            /* send "corrected" timestamp advanced by how long we held it */
            outgoing_timestamp_reply = self
                .saved_timestamp
                .wrapping_add(now.wrapping_sub(self.saved_timestamp_received_at) as u16);
            self.saved_timestamp = u16::MAX;
            self.saved_timestamp_received_at = 0;
        }

        Packet::new(
            self.direction,
            timestamp16(),
            outgoing_timestamp_reply,
            s_payload,
        )
    }

    fn hop_port(&mut self) -> Result<(), NetworkError> {
        assert!(!self.server);

        self.setup();
        let remote_addr = self.remote_addr.expect("remote_addr_len != 0");

        self.socks
            .push_back(sys::new_socket(remote_addr.is_ipv6())?);

        self.prune_sockets();
        Ok(())
    }

    fn prune_sockets(&mut self) {
        /* don't keep old sockets if the new socket has been working for long enough */
        if self.socks.len() > 1 {
            if timestamp().wrapping_sub(self.last_port_choice) > Connection::MAX_OLD_SOCKET_AGE {
                let num_to_kill = self.socks.len() - 1;
                for _ in 0..num_to_kill {
                    self.socks.pop_front();
                }
            }
        } else {
            return;
        }

        /* make sure we don't have too many receive sockets open */
        if self.socks.len() > Connection::MAX_PORTS_OPEN {
            let num_to_kill = self.socks.len() - Connection::MAX_PORTS_OPEN;
            for _ in 0..num_to_kill {
                self.socks.pop_front();
            }
        }
    }

    pub fn send(&mut self, s: &[u8]) -> Result<(), Error> {
        if !self.has_remote_addr {
            return Ok(());
        }

        let px = self.new_packet(s)?;

        let p = self.session.encrypt(&px.to_message())?;

        let remote_addr = self.remote_addr.expect("has_remote_addr");
        let sent = self.sock().send_to(&p, remote_addr);
        let failed_errno = match sent {
            Ok(n) if n == p.len() => None,
            Ok(_) => Some(last_errno()),
            Err(e) => Some(e.raw_os_error().unwrap_or(0)),
        };
        if let Some(errno) = failed_errno {
            /* Make sendto() failure available to the frontend. */
            self.send_error = format!("sendto: {}", strerror(errno));

            if errno == sys::EMSGSIZE {
                self.mtu = Connection::DEFAULT_SEND_MTU; /* payload MTU of last resort */
            }
        }

        let now = timestamp();
        if self.server {
            if now.wrapping_sub(self.last_heard) > Connection::SERVER_ASSOCIATION_TIMEOUT {
                self.has_remote_addr = false;
                eprintln!("Server now detached from client.");
            }
        } else {
            /* client */
            if now.wrapping_sub(self.last_port_choice) > Connection::PORT_HOP_INTERVAL
                && now.wrapping_sub(self.last_roundtrip_success) > Connection::PORT_HOP_INTERVAL
            {
                self.hop_port()?;
            }
        }
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Vec<u8>, Error> {
        assert!(!self.socks.is_empty());
        for i in 0..self.socks.len() {
            /* a would-block result (EAGAIN/EWOULDBLOCK) moves on to the next socket */
            if let Some(payload) = self.recv_one(i)? {
                /* succeeded */
                self.prune_sockets();
                return Ok(payload);
            }
        }
        Err(NetworkError::new("No packet received", 0).into())
    }

    /// Returns `Ok(None)` where upstream's `recv_one` throws EAGAIN/EWOULDBLOCK.
    fn recv_one(&mut self, sock_index: usize) -> Result<Option<Vec<u8>>, Error> {
        let (msg_payload, packet_remote_addr, congestion_experienced) =
            match sys::recv_raw(&self.socks[sock_index], Session::RECEIVE_MTU)? {
                None => return Ok(None),
                Some(r) => r,
            };

        let p = Packet::from_message(&self.session.decrypt(&msg_payload)?)?;

        /* prevent malicious playback to sender */
        let expected_direction = if self.server {
            Direction::ToServer
        } else {
            Direction::ToClient
        };
        if p.direction != expected_direction {
            return Err(dos_detected(
                "p.direction == ( server ? TO_SERVER : TO_CLIENT )",
                "recv_one",
            )
            .into());
        }

        if p.seq < self.expected_receiver_seq {
            /* don't use (but do return) out-of-order packets for timestamp or targeting */
            return Ok(Some(p.payload));
        }
        /* this is security-sensitive because a replay attack could otherwise
        screw up the timestamp and targeting */
        self.expected_receiver_seq = p.seq + 1;

        if p.timestamp != u16::MAX {
            self.saved_timestamp = p.timestamp;
            self.saved_timestamp_received_at = timestamp();

            if congestion_experienced {
                /* signal counterparty to slow down */
                /* this will gradually slow the counterparty down to the minimum frame rate */
                self.saved_timestamp = self
                    .saved_timestamp
                    .wrapping_sub(Connection::CONGESTION_TIMESTAMP_PENALTY);
                if self.server {
                    eprintln!("Received explicit congestion notification.");
                }
            }
        }

        if p.timestamp_reply != u16::MAX {
            let now = timestamp16();
            let r = timestamp_diff(now, p.timestamp_reply) as f64;

            if r < 5000.0 {
                /* ignore large values, e.g. server was Ctrl-Zed */
                if !self.rtt_hit {
                    /* first measurement */
                    self.srtt = r;
                    self.rttvar = r / 2.0;
                    self.rtt_hit = true;
                } else {
                    let alpha = 1.0 / 8.0;
                    let beta = 1.0 / 4.0;

                    self.rttvar = (1.0 - beta) * self.rttvar + (beta * (self.srtt - r).abs());
                    self.srtt = (1.0 - alpha) * self.srtt + (alpha * r);
                }
            }
        }

        /* auto-adjust to remote host */
        self.has_remote_addr = true;
        self.last_heard = timestamp();

        if self.server /* only client can roam */ && self.remote_addr != Some(packet_remote_addr) {
            self.remote_addr = Some(packet_remote_addr);
            eprintln!(
                "Server now attached to client at {}:{}",
                packet_remote_addr.ip(),
                packet_remote_addr.port()
            );
        }
        Ok(Some(p.payload))
    }

    pub fn fds(&self) -> Vec<SocketHandle> {
        self.socks.iter().map(sys::socket_handle).collect()
    }

    pub fn get_mtu(&self) -> i32 {
        self.mtu
    }

    pub fn port(&self) -> Result<String, NetworkError> {
        match self.sock().local_addr() {
            Ok(a) => Ok(a.port().to_string()),
            Err(e) => Err(NetworkError::new(
                "getsockname",
                e.raw_os_error().unwrap_or(0),
            )),
        }
    }

    pub fn get_key(&self) -> String {
        self.key.printable_key()
    }

    pub fn get_has_remote_addr(&self) -> bool {
        self.has_remote_addr
    }

    pub fn timeout(&self) -> u64 {
        let rto = (self.srtt + 4.0 * self.rttvar).ceil() as u64;
        rto.clamp(Connection::MIN_RTO, Connection::MAX_RTO)
    }

    pub fn get_srtt(&self) -> f64 {
        self.srtt
    }

    /// `None` before a server has heard from any client
    /// (upstream: `remote_addr_len == 0`).
    pub fn get_remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    pub fn get_send_error(&mut self) -> &mut String {
        &mut self.send_error
    }

    pub fn set_last_roundtrip_success(&mut self, s_success: u64) {
        self.last_roundtrip_success = s_success;
    }

    pub fn parse_portrange(
        desired_port: &str,
        desired_port_low: &mut i32,
        desired_port_high: &mut i32,
    ) -> bool {
        /* parse "port" or "portlow:porthigh" */
        *desired_port_low = 0;
        *desired_port_high = 0;
        let s = desired_port.as_bytes();

        /* parse first (only?) port */
        let (value, end, overflow) = crate::crypto::strtol(s);
        if overflow || (end != s.len() && s[end] != b':') {
            eprintln!("Invalid (low) port number ({desired_port})");
            return false;
        }
        if !(0..=65535).contains(&value) {
            eprintln!("(Low) port number {value} outside valid range [0..65535]");
            return false;
        }

        *desired_port_low = value as i32;
        if end == s.len() {
            /* not a port range */
            *desired_port_high = *desired_port_low;
            return true;
        }
        /* port range; parse high port */
        let cp = &desired_port[end + 1..];
        let (value, end, overflow) = crate::crypto::strtol(cp.as_bytes());
        if overflow || end != cp.len() {
            eprintln!("Invalid high port number ({cp})");
            return false;
        }
        if !(0..=65535).contains(&value) {
            eprintln!("High port number {value} outside valid range [0..65535]");
            return false;
        }

        *desired_port_high = value as i32;
        if *desired_port_low > *desired_port_high {
            eprintln!(
                "Low port {} greater than high port {}",
                *desired_port_low, *desired_port_high
            );
            return false;
        }

        if *desired_port_low == 0 {
            eprintln!("Low port 0 incompatible with port ranges");
            return false;
        }

        true
    }
}
