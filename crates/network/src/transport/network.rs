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

#[cfg(unix)]
fn strerror(e: i32) -> String {
    // SAFETY: strerror returns a pointer to a NUL-terminated string.
    unsafe { std::ffi::CStr::from_ptr(libc::strerror(e)) }
        .to_string_lossy()
        .into_owned()
}

#[cfg(windows)]
fn strerror(e: i32) -> String {
    std::io::Error::from_raw_os_error(e).to_string()
}

fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
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

#[cfg(unix)]
pub type SocketHandle = std::os::fd::RawFd;
#[cfg(windows)]
pub type SocketHandle = std::os::windows::io::RawSocket;

struct Socket {
    sock: UdpSocket,
}

impl Socket {
    fn fd(&self) -> SocketHandle {
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            self.sock.as_raw_fd()
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawSocket;
            self.sock.as_raw_socket()
        }
    }
}

pub struct Connection {
    socks: VecDeque<Socket>,
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

        #[cfg(unix)]
        self.socks.push_back(sys::new_socket(local_addr.is_ipv6())?);

        let mut saved_errno = 0;
        for i in search_low..=search_high {
            local_addr.set_port(i as u16);

            match sys::bind(&mut self.socks, &local_addr) {
                Ok(()) => {
                    self.set_mtu(local_addr.is_ipv6());
                    return Ok(true);
                }
                Err(e) => saved_errno = e,
            } // else fallthrough to below code, on last iteration.
        }

        #[cfg(unix)]
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

    fn sock(&self) -> &Socket {
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
        let sent = self.sock().sock.send_to(&p, remote_addr);
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
            match sys::recv_raw(&self.socks[sock_index].sock)? {
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
        self.socks.iter().map(Socket::fd).collect()
    }

    pub fn get_mtu(&self) -> i32 {
        self.mtu
    }

    pub fn port(&self) -> Result<String, NetworkError> {
        match self.sock().sock.local_addr() {
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

/// Platform socket calls. Unix follows upstream's BSD socket calls
/// (getaddrinfo, socket options, recvmsg with the TOS control message).
#[cfg(unix)]
mod sys {
    use super::{last_errno, NetworkError, Socket};
    use std::collections::VecDeque;
    use std::ffi::{CStr, CString};
    use std::mem::{size_of, zeroed};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
    use std::os::fd::{AsRawFd, FromRawFd};

    use crate::crypto::Session;

    pub const EMSGSIZE: i32 = libc::EMSGSIZE;

    fn setsockopt_int(
        fd: libc::c_int,
        level: libc::c_int,
        name: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int {
        // SAFETY: passes a pointer to a local c_int with its size.
        unsafe {
            libc::setsockopt(
                fd,
                level,
                name,
                &value as *const libc::c_int as *const libc::c_void,
                size_of::<libc::c_int>() as libc::socklen_t,
            )
        }
    }

    /// `Connection::Socket::Socket( int family )`.
    pub fn new_socket(ipv6: bool) -> Result<Socket, NetworkError> {
        let family = if ipv6 { libc::AF_INET6 } else { libc::AF_INET };
        // SAFETY: plain socket(2) call.
        let fd = unsafe { libc::socket(family, libc::SOCK_DGRAM, 0) };
        if fd < 0 {
            return Err(NetworkError::new("socket", last_errno()));
        }
        // SAFETY: fd is a freshly created socket that nothing else owns.
        let sock = unsafe { UdpSocket::from_raw_fd(fd) };

        /* Disable path MTU discovery */
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            let flag = libc::IP_PMTUDISC_DONT;
            if setsockopt_int(fd, libc::IPPROTO_IP, libc::IP_MTU_DISCOVER, flag) < 0 {
                return Err(NetworkError::new("setsockopt", last_errno()));
            }
        }

        //  int dscp = 0x92; /* OS X does not have IPTOS_DSCP_AF42 constant */
        let dscp = 0x02; /* ECN-capable transport only */
        if setsockopt_int(fd, libc::IPPROTO_IP, libc::IP_TOS, dscp) < 0 {
            //    perror( "setsockopt( IP_TOS )" );
        }

        /* request explicit congestion notification on received datagrams */
        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_vendor = "apple",
            target_os = "freebsd"
        ))]
        {
            let tosflag = 1;
            if setsockopt_int(fd, libc::IPPROTO_IP, libc::IP_RECVTOS, tosflag) < 0
                && family == libc::IPPROTO_IP
            {
                /* FreeBSD disallows this option on IPv6 sockets. */
                eprintln!(
                    "setsockopt( IP_RECVTOS ): {}",
                    std::io::Error::last_os_error()
                );
            }
        }

        /* upstream passes MSG_DONTWAIT to every sendto() and recvmsg() */
        if let Err(e) = sock.set_nonblocking(true) {
            return Err(NetworkError::new("fcntl", e.raw_os_error().unwrap_or(0)));
        }

        Ok(Socket { sock })
    }

    fn to_sockaddr(addr: &SocketAddr) -> (libc::sockaddr_storage, libc::socklen_t) {
        // SAFETY: sockaddr_storage is plain data; all-zero is a valid value.
        let mut ss: libc::sockaddr_storage = unsafe { zeroed() };
        let len = match addr {
            SocketAddr::V4(a) => {
                // SAFETY: sockaddr_storage is large enough and aligned for sockaddr_in.
                let sin = unsafe {
                    &mut *(&mut ss as *mut libc::sockaddr_storage as *mut libc::sockaddr_in)
                };
                sin.sin_family = libc::AF_INET as libc::sa_family_t;
                sin.sin_port = a.port().to_be();
                sin.sin_addr = libc::in_addr {
                    s_addr: u32::from_ne_bytes(a.ip().octets()),
                };
                #[cfg(any(
                    target_vendor = "apple",
                    target_os = "freebsd",
                    target_os = "openbsd",
                    target_os = "netbsd",
                    target_os = "dragonfly"
                ))]
                {
                    sin.sin_len = size_of::<libc::sockaddr_in>() as u8;
                }
                size_of::<libc::sockaddr_in>()
            }
            SocketAddr::V6(a) => {
                // SAFETY: sockaddr_storage is large enough and aligned for sockaddr_in6.
                let sin6 = unsafe {
                    &mut *(&mut ss as *mut libc::sockaddr_storage as *mut libc::sockaddr_in6)
                };
                sin6.sin6_family = libc::AF_INET6 as libc::sa_family_t;
                sin6.sin6_port = a.port().to_be();
                sin6.sin6_flowinfo = a.flowinfo();
                sin6.sin6_addr = libc::in6_addr {
                    s6_addr: a.ip().octets(),
                };
                sin6.sin6_scope_id = a.scope_id();
                #[cfg(any(
                    target_vendor = "apple",
                    target_os = "freebsd",
                    target_os = "openbsd",
                    target_os = "netbsd",
                    target_os = "dragonfly"
                ))]
                {
                    sin6.sin6_len = size_of::<libc::sockaddr_in6>() as u8;
                }
                size_of::<libc::sockaddr_in6>()
            }
        };
        (ss, len as libc::socklen_t)
    }

    /// # Safety
    /// `sa` must point to a socket address of at least `len` bytes.
    unsafe fn from_sockaddr(sa: *const libc::sockaddr, len: libc::socklen_t) -> Option<SocketAddr> {
        // SAFETY: caller guarantees sa points to len valid bytes; family is read first.
        unsafe {
            match (*sa).sa_family as libc::c_int {
                libc::AF_INET if len as usize >= size_of::<libc::sockaddr_in>() => {
                    let sin = &*(sa as *const libc::sockaddr_in);
                    Some(SocketAddr::V4(SocketAddrV4::new(
                        Ipv4Addr::from(sin.sin_addr.s_addr.to_ne_bytes()),
                        u16::from_be(sin.sin_port),
                    )))
                }
                libc::AF_INET6 if len as usize >= size_of::<libc::sockaddr_in6>() => {
                    let sin6 = &*(sa as *const libc::sockaddr_in6);
                    Some(SocketAddr::V6(SocketAddrV6::new(
                        Ipv6Addr::from(sin6.sin6_addr.s6_addr),
                        u16::from_be(sin6.sin6_port),
                        sin6.sin6_flowinfo,
                        sin6.sin6_scope_id,
                    )))
                }
                _ => None,
            }
        }
    }

    /// `AddrInfo( node, service, hints )` with upstream's hints, first result.
    /// `passive` selects the server hints (AI_PASSIVE).
    pub fn resolve(
        node: Option<&str>,
        service: &str,
        passive: bool,
    ) -> Result<SocketAddr, NetworkError> {
        let bad = |msg: String| {
            NetworkError::new(
                format!("Bad IP address ({}): {}", node.unwrap_or("(null)"), msg),
                0,
            )
        };
        let node_c = match node.map(CString::new).transpose() {
            Ok(n) => n,
            Err(_) => return Err(bad("embedded NUL".to_string())),
        };
        let service_c = CString::new(service).map_err(|_| bad("embedded NUL".to_string()))?;

        // SAFETY: addrinfo is plain data; all-zero is the documented way to init hints.
        let mut hints: libc::addrinfo = unsafe { zeroed() };
        hints.ai_family = libc::AF_UNSPEC;
        hints.ai_socktype = libc::SOCK_DGRAM;
        hints.ai_flags = libc::AI_NUMERICHOST | libc::AI_NUMERICSERV;
        if passive {
            hints.ai_flags |= libc::AI_PASSIVE;
        }

        let mut res: *mut libc::addrinfo = std::ptr::null_mut();
        // SAFETY: valid C strings (or NULL node), valid hints, out-pointer to res.
        let errcode = unsafe {
            libc::getaddrinfo(
                node_c.as_ref().map_or(std::ptr::null(), |n| n.as_ptr()),
                service_c.as_ptr(),
                &hints,
                &mut res,
            )
        };
        if errcode != 0 {
            // SAFETY: gai_strerror returns a static NUL-terminated string.
            let msg = unsafe { CStr::from_ptr(libc::gai_strerror(errcode)) }
                .to_string_lossy()
                .into_owned();
            return Err(bad(msg));
        }
        // SAFETY: getaddrinfo succeeded, so res points to at least one addrinfo.
        let addr = unsafe {
            let first = &*res;
            let a = from_sockaddr(first.ai_addr, first.ai_addrlen);
            libc::freeaddrinfo(res);
            a
        };
        addr.ok_or_else(|| NetworkError::new("Unknown address family", 0))
    }

    /// Binds the newest socket (pushed by `try_bind`) to `local_addr`.
    /// Returns errno on failure.
    pub fn bind(socks: &mut VecDeque<Socket>, local_addr: &SocketAddr) -> Result<(), i32> {
        let fd = socks
            .back()
            .expect("socket pushed by try_bind")
            .sock
            .as_raw_fd();
        if let SocketAddr::V6(a) = local_addr {
            if a.ip().is_unspecified() {
                let off = 0;
                if setsockopt_int(fd, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY, off) != 0 {
                    eprintln!(
                        "setsockopt( IPV6_V6ONLY, off ): {}",
                        std::io::Error::last_os_error()
                    );
                }
            }
        }
        let (ss, len) = to_sockaddr(local_addr);
        // SAFETY: ss holds a valid sockaddr of length len.
        if unsafe {
            libc::bind(
                fd,
                &ss as *const libc::sockaddr_storage as *const libc::sockaddr,
                len,
            )
        } == 0
        {
            Ok(())
        } else {
            Err(last_errno())
        }
    }

    /// Receive one datagram with its source address and ECN mark.
    /// `Ok(None)` means EAGAIN/EWOULDBLOCK.
    pub fn recv_raw(sock: &UdpSocket) -> Result<Option<(Vec<u8>, SocketAddr, bool)>, NetworkError> {
        /* receive source address, ECN, and payload in msghdr structure */
        // SAFETY: plain data structures, zero-initialized.
        let mut packet_remote_addr: libc::sockaddr_storage = unsafe { zeroed() };
        let mut header: libc::msghdr = unsafe { zeroed() };
        let mut msg_payload = [0u8; Session::RECEIVE_MTU];
        /* u64 elements keep the control buffer aligned for cmsghdr */
        let mut msg_control = [0u64; Session::RECEIVE_MTU / 8];
        let mut msg_iovec = libc::iovec {
            iov_base: msg_payload.as_mut_ptr() as *mut libc::c_void,
            iov_len: msg_payload.len(),
        };

        /* receive source address */
        header.msg_name =
            &mut packet_remote_addr as *mut libc::sockaddr_storage as *mut libc::c_void;
        header.msg_namelen = size_of::<libc::sockaddr_storage>() as libc::socklen_t;

        /* receive payload */
        header.msg_iov = &mut msg_iovec;
        header.msg_iovlen = 1;

        /* receive explicit congestion notification */
        header.msg_control = msg_control.as_mut_ptr() as *mut libc::c_void;
        header.msg_controllen = (msg_control.len() * 8) as _;

        /* receive flags */
        header.msg_flags = 0;

        // SAFETY: header points at live buffers declared above.
        let received_len =
            unsafe { libc::recvmsg(sock.as_raw_fd(), &mut header, libc::MSG_DONTWAIT) };

        if received_len < 0 {
            let errno = last_errno();
            if errno == libc::EAGAIN || errno == libc::EWOULDBLOCK {
                return Ok(None);
            }
            return Err(NetworkError::new("recvmsg", errno));
        }

        if header.msg_flags & libc::MSG_TRUNC != 0 {
            return Err(NetworkError::new(
                "Received oversize datagram",
                last_errno(),
            ));
        }

        /* receive ECN */
        let mut congestion_experienced = false;

        // SAFETY: header was filled in by recvmsg; CMSG_FIRSTHDR/CMSG_DATA stay within msg_control.
        unsafe {
            let ecn_hdr = libc::CMSG_FIRSTHDR(&header);
            if !ecn_hdr.is_null()
                && (*ecn_hdr).cmsg_level == libc::IPPROTO_IP
                && is_tos_type((*ecn_hdr).cmsg_type)
            {
                /* got one */
                let ecn_octet_p = libc::CMSG_DATA(ecn_hdr);
                assert!(!ecn_octet_p.is_null());
                congestion_experienced = (*ecn_octet_p & 0x03) == 0x03;
            }
        }

        // SAFETY: recvmsg wrote msg_namelen bytes of address into packet_remote_addr.
        let addr = unsafe {
            from_sockaddr(
                &packet_remote_addr as *const libc::sockaddr_storage as *const libc::sockaddr,
                header.msg_namelen,
            )
        };
        let addr = addr.ok_or_else(|| NetworkError::new("recv_one: unknown address family", 0))?;

        Ok(Some((
            msg_payload[..received_len as usize].to_vec(),
            addr,
            congestion_experienced,
        )))
    }

    fn is_tos_type(t: libc::c_int) -> bool {
        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_vendor = "apple",
            target_os = "freebsd"
        ))]
        {
            t == libc::IP_TOS || t == libc::IP_RECVTOS
        }
        #[cfg(not(any(
            target_os = "linux",
            target_os = "android",
            target_vendor = "apple",
            target_os = "freebsd"
        )))]
        {
            t == libc::IP_TOS
        }
    }
}

/// Windows: std sockets. No IP_MTU_DISCOVER, IP_TOS or ECN (TOS) reception.
#[cfg(windows)]
mod sys {
    use super::{NetworkError, Socket};
    use std::collections::VecDeque;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

    use crate::crypto::Session;

    const WSAEWOULDBLOCK: i32 = 10035;
    const WSAEMSGSIZE: i32 = 10040;
    pub const EMSGSIZE: i32 = WSAEMSGSIZE;

    fn errno(e: &std::io::Error) -> i32 {
        e.raw_os_error().unwrap_or(0)
    }

    /// An unbound socket cannot be created with std; bind to the
    /// unspecified address and port 0, which is what the first sendto()
    /// on an unbound socket does implicitly.
    pub fn new_socket(ipv6: bool) -> Result<Socket, NetworkError> {
        let any: SocketAddr = if ipv6 {
            (Ipv6Addr::UNSPECIFIED, 0).into()
        } else {
            (Ipv4Addr::UNSPECIFIED, 0).into()
        };
        let sock = UdpSocket::bind(any).map_err(|e| NetworkError::new("socket", errno(&e)))?;
        sock.set_nonblocking(true)
            .map_err(|e| NetworkError::new("ioctlsocket", errno(&e)))?;
        Ok(Socket { sock })
    }

    /// Numeric host and port only, as upstream's AI_NUMERICHOST | AI_NUMERICSERV.
    pub fn resolve(
        node: Option<&str>,
        service: &str,
        _passive: bool,
    ) -> Result<SocketAddr, NetworkError> {
        let bad = || {
            NetworkError::new(
                format!(
                    "Bad IP address ({}): Name or service not known",
                    node.unwrap_or("(null)")
                ),
                0,
            )
        };
        let ip: IpAddr = match node {
            None => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            Some(n) => n.parse().map_err(|_| bad())?,
        };
        let port: u16 = service.parse().map_err(|_| bad())?;
        Ok(SocketAddr::new(ip, port))
    }

    /// Creates and binds a socket at `local_addr`, pushing it on success.
    pub fn bind(socks: &mut VecDeque<Socket>, local_addr: &SocketAddr) -> Result<(), i32> {
        let sock = UdpSocket::bind(local_addr).map_err(|e| errno(&e))?;
        sock.set_nonblocking(true).map_err(|e| errno(&e))?;
        socks.push_back(Socket { sock });
        Ok(())
    }

    pub fn recv_raw(sock: &UdpSocket) -> Result<Option<(Vec<u8>, SocketAddr, bool)>, NetworkError> {
        let mut msg_payload = [0u8; Session::RECEIVE_MTU];
        match sock.recv_from(&mut msg_payload) {
            Ok((n, addr)) => Ok(Some((msg_payload[..n].to_vec(), addr, false))),
            Err(e) if errno(&e) == WSAEWOULDBLOCK => Ok(None),
            Err(e) if errno(&e) == WSAEMSGSIZE => {
                Err(NetworkError::new("Received oversize datagram", errno(&e)))
            }
            Err(e) => Err(NetworkError::new("recvmsg", errno(&e))),
        }
    }
}
