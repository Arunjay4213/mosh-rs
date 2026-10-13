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
// Ported from: upstream/src/network/network.cc (the socket system calls of Connection)
// The Windows half is new (no upstream equivalent), using std sockets.

//! UDP socket calls for `mosh_network::Connection`, from upstream
//! `src/network/network.cc`: socket creation with its options
//! (`IP_MTU_DISCOVER`, `IP_TOS`, `IP_RECVTOS`), numeric address resolution
//! with upstream's `getaddrinfo` hints, binding, and `recvmsg` with the
//! source address and the ECN congestion mark.
//!
//! Unix follows upstream's BSD socket calls. Windows uses std sockets and
//! has no `IP_MTU_DISCOVER`, `IP_TOS` or ECN reception; a std socket cannot
//! exist unbound, so `new_socket` binds to the unspecified address and port
//! 0, which is what the first `sendto()` on an unbound socket does.
//!
//! Sockets are non-blocking: upstream passes `MSG_DONTWAIT` to every
//! `sendto()` and `recvmsg()`, which is the same thing per call.

use std::fmt;
use std::net::UdpSocket;

/// A failed system call: the name upstream puts in its `NetworkException`
/// (or a full message, for resolver errors) and the errno (0 if none).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysError {
    /// Function name or message, as upstream's `NetworkException::function`.
    pub function: String,
    /// errno, or 0.
    pub errno: i32,
}

impl SysError {
    fn new(function: impl Into<String>, errno: i32) -> SysError {
        SysError {
            function: function.into(),
            errno,
        }
    }
}

impl fmt::Display for SysError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.function, strerror(self.errno))
    }
}

/// The OS handle of a socket, for polling.
#[cfg(unix)]
pub type SocketHandle = std::os::fd::RawFd;
/// The OS handle of a socket, for polling.
#[cfg(windows)]
pub type SocketHandle = std::os::windows::io::RawSocket;

/// The OS handle of `sock`.
pub fn socket_handle(sock: &UdpSocket) -> SocketHandle {
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;
        sock.as_raw_fd()
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawSocket;
        sock.as_raw_socket()
    }
}

/// errno of the last failed system call (0 if none).
pub fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// errno value for "message too long" from `sendto()`.
#[cfg(unix)]
pub const EMSGSIZE: i32 = libc::EMSGSIZE;
/// errno value for "message too long" from `sendto()` (WSAEMSGSIZE).
#[cfg(windows)]
pub const EMSGSIZE: i32 = 10040;

/// C `strerror()`.
#[cfg(unix)]
pub fn strerror(e: i32) -> String {
    // SAFETY: strerror returns a pointer to a NUL-terminated string that
    // stays valid until the next strerror call; it is copied out at once.
    unsafe { std::ffi::CStr::from_ptr(libc::strerror(e)) }
        .to_string_lossy()
        .into_owned()
}

/// The system message for error code `e`, without Rust's " (os error N)"
/// suffix, as C `strerror()` gives it.
#[cfg(windows)]
pub fn strerror(e: i32) -> String {
    let text = std::io::Error::from_raw_os_error(e).to_string();
    match text.rfind(" (os error ") {
        Some(at) => text[..at].to_string(),
        None => text,
    }
}

#[cfg(unix)]
pub use unix::{bind, new_socket, recv_raw, resolve};
#[cfg(windows)]
pub use windows::{bind, new_socket, recv_raw, resolve};

/// The one decoder of a C socket address into a `SocketAddr`, used here
/// and by `netdb.rs`, so the two cannot read a `sockaddr` differently.
#[cfg(unix)]
pub(crate) use unix::from_sockaddr;
#[cfg(windows)]
pub(crate) use windows::from_sockaddr;

#[cfg(unix)]
mod unix {
    use super::{last_errno, SysError};
    use std::ffi::{CStr, CString};
    use std::mem::{size_of, zeroed};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
    use std::os::fd::{AsRawFd, FromRawFd};

    /// `sizeof` of a socket address structure as `socklen_t`. The structures
    /// are at most 128 bytes, so the conversion is lossless.
    fn socklen_of<T>() -> libc::socklen_t {
        libc::socklen_t::try_from(size_of::<T>()).expect("socket structures are small")
    }

    fn setsockopt_int(
        fd: libc::c_int,
        level: libc::c_int,
        name: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int {
        // SAFETY: passes a pointer to a live local c_int together with its size.
        unsafe {
            libc::setsockopt(
                fd,
                level,
                name,
                std::ptr::from_ref(&value).cast::<libc::c_void>(),
                socklen_of::<libc::c_int>(),
            )
        }
    }

    /// `Connection::Socket::Socket( int family )`: a UDP socket of the
    /// family of IPv6 (`ipv6`) or IPv4, with upstream's socket options.
    pub fn new_socket(ipv6: bool) -> Result<UdpSocket, SysError> {
        let family = if ipv6 { libc::AF_INET6 } else { libc::AF_INET };
        // SAFETY: plain socket(2) call with constant arguments.
        let fd = unsafe { libc::socket(family, libc::SOCK_DGRAM, 0) };
        if fd < 0 {
            return Err(SysError::new("socket", last_errno()));
        }
        // SAFETY: fd is a freshly created socket that nothing else owns;
        // the UdpSocket takes ownership and closes it on drop.
        let sock = unsafe { UdpSocket::from_raw_fd(fd) };

        /* Disable path MTU discovery */
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            let flag = libc::IP_PMTUDISC_DONT;
            if setsockopt_int(fd, libc::IPPROTO_IP, libc::IP_MTU_DISCOVER, flag) < 0 {
                return Err(SysError::new("setsockopt", last_errno()));
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
                crate::stdio::perror("setsockopt( IP_RECVTOS )", &std::io::Error::last_os_error());
            }
        }

        /* upstream passes MSG_DONTWAIT to every sendto() and recvmsg() */
        if let Err(e) = sock.set_nonblocking(true) {
            return Err(SysError::new("fcntl", e.raw_os_error().unwrap_or(0)));
        }

        Ok(sock)
    }

    fn to_sockaddr(addr: &SocketAddr) -> (libc::sockaddr_storage, libc::socklen_t) {
        // SAFETY: sockaddr_storage is plain data; all-zero is a valid value.
        let mut ss: libc::sockaddr_storage = unsafe { zeroed() };
        let len = match addr {
            SocketAddr::V4(a) => {
                // SAFETY: sockaddr_storage is large enough and aligned for sockaddr_in.
                let sin = unsafe { &mut *std::ptr::from_mut(&mut ss).cast::<libc::sockaddr_in>() };
                /* AF_INET is 2: fits sa_family_t */
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
                    /* 16 bytes: fits u8 */
                    sin.sin_len = size_of::<libc::sockaddr_in>() as u8;
                }
                socklen_of::<libc::sockaddr_in>()
            }
            SocketAddr::V6(a) => {
                // SAFETY: sockaddr_storage is large enough and aligned for sockaddr_in6.
                let sin6 =
                    unsafe { &mut *std::ptr::from_mut(&mut ss).cast::<libc::sockaddr_in6>() };
                /* AF_INET6 is 10 or 30: fits sa_family_t */
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
                    /* 28 bytes: fits u8 */
                    sin6.sin6_len = size_of::<libc::sockaddr_in6>() as u8;
                }
                socklen_of::<libc::sockaddr_in6>()
            }
        };
        (ss, len)
    }

    /// Decodes a `sockaddr_in` or `sockaddr_in6` through libc's structs, so
    /// the family is read from `sa_family` wherever the platform puts it
    /// (after `sa_len` on macOS and the BSDs, first on Linux).
    ///
    /// # Safety
    /// `sa` must point to a socket address of at least `len` readable bytes.
    pub(crate) unsafe fn from_sockaddr(
        sa: *const libc::sockaddr,
        len: libc::socklen_t,
    ) -> Option<SocketAddr> {
        let len = usize::try_from(len).ok()?;
        // SAFETY: the caller guarantees sa points to len readable bytes; the
        // family is read first and the length checked before each cast.
        unsafe {
            match libc::c_int::from((*sa).sa_family) {
                libc::AF_INET if len >= size_of::<libc::sockaddr_in>() => {
                    let sin = &*sa.cast::<libc::sockaddr_in>();
                    Some(SocketAddr::V4(SocketAddrV4::new(
                        Ipv4Addr::from(sin.sin_addr.s_addr.to_ne_bytes()),
                        u16::from_be(sin.sin_port),
                    )))
                }
                libc::AF_INET6 if len >= size_of::<libc::sockaddr_in6>() => {
                    let sin6 = &*sa.cast::<libc::sockaddr_in6>();
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

    /// `AddrInfo( node, service, hints )` with upstream's hints
    /// (`AF_UNSPEC`, `SOCK_DGRAM`, `AI_NUMERICHOST | AI_NUMERICSERV`, plus
    /// `AI_PASSIVE` when `passive`); returns the first result.
    pub fn resolve(
        node: Option<&str>,
        service: &str,
        passive: bool,
    ) -> Result<SocketAddr, SysError> {
        let bad = |msg: &str| {
            SysError::new(
                format!("Bad IP address ({}): {}", node.unwrap_or("(null)"), msg),
                0,
            )
        };
        let Ok(node_c) = node.map(CString::new).transpose() else {
            return Err(bad("embedded NUL"));
        };
        let service_c = CString::new(service).map_err(|_| bad("embedded NUL"))?;

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
            let msg = unsafe { CStr::from_ptr(libc::gai_strerror(errcode)) }.to_string_lossy();
            return Err(bad(&msg));
        }
        // SAFETY: getaddrinfo succeeded, so res points to at least one addrinfo
        // whose ai_addr holds ai_addrlen bytes; res is freed exactly once.
        let addr = unsafe {
            let first = &*res;
            let a = from_sockaddr(first.ai_addr, first.ai_addrlen);
            libc::freeaddrinfo(res);
            a
        };
        addr.ok_or_else(|| SysError::new("Unknown address family", 0))
    }

    /// Binds `sock` to `local_addr`, first turning `IPV6_V6ONLY` off for the
    /// IPv6 wildcard address as upstream does. Returns errno on failure.
    pub fn bind(sock: &mut UdpSocket, local_addr: &SocketAddr) -> Result<(), i32> {
        let fd = sock.as_raw_fd();
        if let SocketAddr::V6(a) = local_addr {
            if a.ip().is_unspecified() {
                let off = 0;
                if setsockopt_int(fd, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY, off) != 0 {
                    crate::stdio::perror(
                        "setsockopt( IPV6_V6ONLY, off )",
                        &std::io::Error::last_os_error(),
                    );
                }
            }
        }
        let (ss, len) = to_sockaddr(local_addr);
        // SAFETY: ss holds a valid sockaddr of length len.
        let rc = unsafe { libc::bind(fd, std::ptr::from_ref(&ss).cast::<libc::sockaddr>(), len) };
        if rc == 0 {
            Ok(())
        } else {
            Err(last_errno())
        }
    }

    /// Receives one datagram of at most `max_len` bytes with its source
    /// address and whether it carries the ECN congestion-experienced mark.
    /// `Ok(None)` means EAGAIN/EWOULDBLOCK.
    pub fn recv_raw(
        sock: &UdpSocket,
        max_len: usize,
    ) -> Result<Option<(Vec<u8>, SocketAddr, bool)>, SysError> {
        /* receive source address, ECN, and payload in msghdr structure */
        // SAFETY: sockaddr_storage and msghdr are plain data; all-zero is valid.
        let mut packet_remote_addr: libc::sockaddr_storage = unsafe { zeroed() };
        // SAFETY: as above.
        let mut header: libc::msghdr = unsafe { zeroed() };
        let mut msg_payload = vec![0u8; max_len];
        /* u64 elements keep the control buffer aligned for cmsghdr */
        let mut msg_control = [0u64; 256];
        let mut msg_iovec = libc::iovec {
            iov_base: msg_payload.as_mut_ptr().cast::<libc::c_void>(),
            iov_len: msg_payload.len(),
        };

        /* receive source address */
        header.msg_name = std::ptr::from_mut(&mut packet_remote_addr).cast::<libc::c_void>();
        header.msg_namelen = socklen_of::<libc::sockaddr_storage>();

        /* receive payload */
        header.msg_iov = &mut msg_iovec;
        header.msg_iovlen = 1;

        /* receive explicit congestion notification */
        header.msg_control = msg_control.as_mut_ptr().cast::<libc::c_void>();
        /* 2048 bytes: fits msg_controllen on every platform */
        header.msg_controllen = (msg_control.len() * 8) as _;

        /* receive flags */
        header.msg_flags = 0;

        // SAFETY: header points at the live buffers declared above.
        let received_len =
            unsafe { libc::recvmsg(sock.as_raw_fd(), &mut header, libc::MSG_DONTWAIT) };

        /* negative means error; otherwise it is a length within max_len */
        let Ok(received_len) = usize::try_from(received_len) else {
            let errno = last_errno();
            if errno == libc::EAGAIN || errno == libc::EWOULDBLOCK {
                return Ok(None);
            }
            return Err(SysError::new("recvmsg", errno));
        };

        if header.msg_flags & libc::MSG_TRUNC != 0 {
            return Err(SysError::new("Received oversize datagram", last_errno()));
        }

        /* receive ECN */
        let mut congestion_experienced = false;

        // SAFETY: header was filled in by recvmsg; CMSG_FIRSTHDR returns null
        // or a header inside msg_control, and CMSG_DATA stays inside it.
        unsafe {
            let ecn_hdr = libc::CMSG_FIRSTHDR(&header);
            if !ecn_hdr.is_null()
                && (*ecn_hdr).cmsg_level == libc::IPPROTO_IP
                && is_tos_type((*ecn_hdr).cmsg_type)
            {
                /* got one */
                let ecn_octet_p = libc::CMSG_DATA(ecn_hdr);
                congestion_experienced = (*ecn_octet_p & 0x03) == 0x03;
            }
        }

        // SAFETY: recvmsg wrote msg_namelen bytes of address into packet_remote_addr.
        let addr = unsafe {
            from_sockaddr(
                std::ptr::from_ref(&packet_remote_addr).cast::<libc::sockaddr>(),
                header.msg_namelen,
            )
        };
        let addr = addr.ok_or_else(|| SysError::new("recv_one: unknown address family", 0))?;

        msg_payload.truncate(received_len);
        Ok(Some((msg_payload, addr, congestion_experienced)))
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

#[cfg(windows)]
mod windows {
    use super::SysError;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6, UdpSocket};

    /// Decodes a WinSock `SOCKADDR_IN` or `SOCKADDR_IN6` through its typed
    /// fields (Windows has no `sa_len`; the family is the first field).
    ///
    /// # Safety
    /// `sa` must point to a socket address of at least `len` readable bytes.
    pub(crate) unsafe fn from_sockaddr(
        sa: *const windows_sys::Win32::Networking::WinSock::SOCKADDR,
        len: usize,
    ) -> Option<SocketAddr> {
        use std::net::SocketAddrV4;
        use windows_sys::Win32::Networking::WinSock::{
            AF_INET, AF_INET6, SOCKADDR_IN, SOCKADDR_IN6,
        };
        // SAFETY: the caller guarantees sa points to len readable bytes; the
        // family is read first and the length checked before each cast.
        unsafe {
            match (*sa).sa_family {
                AF_INET if len >= size_of::<SOCKADDR_IN>() => {
                    let sin = &*sa.cast::<SOCKADDR_IN>();
                    Some(SocketAddr::V4(SocketAddrV4::new(
                        Ipv4Addr::from(sin.sin_addr.S_un.S_addr.to_ne_bytes()),
                        u16::from_be(sin.sin_port),
                    )))
                }
                AF_INET6 if len >= size_of::<SOCKADDR_IN6>() => {
                    let sin6 = &*sa.cast::<SOCKADDR_IN6>();
                    Some(SocketAddr::V6(SocketAddrV6::new(
                        Ipv6Addr::from(sin6.sin6_addr.u.Byte),
                        u16::from_be(sin6.sin6_port),
                        sin6.sin6_flowinfo,
                        sin6.Anonymous.sin6_scope_id,
                    )))
                }
                _ => None,
            }
        }
    }

    const WSAEWOULDBLOCK: i32 = 10035;
    const WSAEMSGSIZE: i32 = 10040;

    fn errno(e: &std::io::Error) -> i32 {
        e.raw_os_error().unwrap_or(0)
    }

    /// A non-blocking UDP socket of the given family, bound to the
    /// unspecified address and port 0.
    pub fn new_socket(ipv6: bool) -> Result<UdpSocket, SysError> {
        let any: SocketAddr = if ipv6 {
            (Ipv6Addr::UNSPECIFIED, 0).into()
        } else {
            (Ipv4Addr::UNSPECIFIED, 0).into()
        };
        let sock = UdpSocket::bind(any).map_err(|e| SysError::new("socket", errno(&e)))?;
        sock.set_nonblocking(true)
            .map_err(|e| SysError::new("ioctlsocket", errno(&e)))?;
        Ok(sock)
    }

    /// Numeric host and port only, as upstream's `AI_NUMERICHOST | AI_NUMERICSERV`.
    /// No host means the IPv4 wildcard.
    pub fn resolve(
        node: Option<&str>,
        service: &str,
        _passive: bool,
    ) -> Result<SocketAddr, SysError> {
        let bad = || {
            SysError::new(
                format!(
                    "Bad IP address ({}): Name or service not known",
                    node.unwrap_or("(null)")
                ),
                0,
            )
        };
        let port: u16 = service.parse().map_err(|_| bad())?;
        let Some(n) = node else {
            return Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port));
        };
        /* An IPv6 literal may carry a zone, `fe80::1%3`, which getaddrinfo
        with AI_NUMERICHOST accepts as the interface index (the unix half
        accepts it the same way). */
        if let Some((addr, zone)) = n.split_once('%') {
            let ip: Ipv6Addr = addr.parse().map_err(|_| bad())?;
            let scope_id: u32 = zone.parse().map_err(|_| bad())?;
            return Ok(SocketAddr::V6(SocketAddrV6::new(ip, port, 0, scope_id)));
        }
        let ip: IpAddr = n.parse().map_err(|_| bad())?;
        Ok(SocketAddr::new(ip, port))
    }

    /// A std socket cannot be re-bound, so this replaces `sock` with a new
    /// non-blocking socket bound to `local_addr`. Returns the error code on failure.
    pub fn bind(sock: &mut UdpSocket, local_addr: &SocketAddr) -> Result<(), i32> {
        let bound = UdpSocket::bind(local_addr).map_err(|e| errno(&e))?;
        bound.set_nonblocking(true).map_err(|e| errno(&e))?;
        *sock = bound;
        Ok(())
    }

    /// Receives one datagram of at most `max_len` bytes with its source
    /// address; there is no ECN mark on Windows. `Ok(None)` means WSAEWOULDBLOCK.
    pub fn recv_raw(
        sock: &UdpSocket,
        max_len: usize,
    ) -> Result<Option<(Vec<u8>, SocketAddr, bool)>, SysError> {
        let mut msg_payload = vec![0u8; max_len];
        match sock.recv_from(&mut msg_payload) {
            Ok((n, addr)) => {
                msg_payload.truncate(n);
                Ok(Some((msg_payload, addr, false)))
            }
            Err(e) if errno(&e) == WSAEWOULDBLOCK => Ok(None),
            Err(e) if errno(&e) == WSAEMSGSIZE => {
                Err(SysError::new("Received oversize datagram", errno(&e)))
            }
            Err(e) => Err(SysError::new("recvmsg", errno(&e))),
        }
    }
}
