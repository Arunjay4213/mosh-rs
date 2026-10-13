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
//
// Ported from: upstream/scripts/mosh.pl (`sub resolvename`, which calls
// Socket's `getaddrinfo` with AI_NUMERICHOST and AI_CANONNAME hints).

//! `getaddrinfo` for the launcher's port of mosh.pl `resolvename`: TCP
//! stream hints, an optional address family, the AI_NUMERICHOST and
//! AI_CANONNAME flags, and the canonical name of the first result. Unix
//! calls libc; Windows calls WinSock's `getaddrinfo`.

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

/// Address family hint, as mosh.pl's `--family=inet` / `inet6`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Family {
    /// AF_INET.
    Inet,
    /// AF_INET6.
    Inet6,
}

/// Which `ai_flags` to pass.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flags {
    /// No flags.
    None,
    /// AI_NUMERICHOST: the host must be a numeric address.
    NumericHost,
    /// AI_CANONNAME: also return the canonical name of the host.
    CanonName,
}

/// A failed lookup: the `gai_strerror` text and the `errno` value left
/// behind (Perl's `$!` after the call, which `die` uses as the exit code;
/// 0 when there is none, and always 0 on Windows).
#[derive(Debug)]
pub struct GaiError {
    /// `gai_strerror` text (on Windows, the WinSock error text).
    pub message: String,
    /// `errno` right after the call.
    pub errno: i32,
}

/// The results of a successful lookup, in resolver order.
#[derive(Debug, Default)]
pub struct AddrInfo {
    /// Every IPv4 and IPv6 result.
    pub addrs: Vec<SocketAddr>,
    /// `ai_canonname` of the first result, when AI_CANONNAME was asked for.
    pub canonname: Option<String>,
}

/// Decode a `sockaddr_in` or `sockaddr_in6` from its bytes. The layout is
/// the same on Linux and Windows: a 16-bit family, the port in network
/// order, then the IPv4 address, or the flow info, IPv6 address and scope.
fn sockaddr_from_bytes(b: &[u8], af_inet: i32, af_inet6: i32) -> Option<SocketAddr> {
    let family = i32::from(u16::from_ne_bytes([*b.first()?, *b.get(1)?]));
    let port = u16::from_be_bytes([*b.get(2)?, *b.get(3)?]);
    if family == af_inet && b.len() >= 8 {
        let ip = Ipv4Addr::new(b[4], b[5], b[6], b[7]);
        Some(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    } else if family == af_inet6 && b.len() >= 28 {
        // The raw field, as std and `socket.rs` keep it (`SocketAddrV6::flowinfo`).
        let flowinfo = u32::from_ne_bytes([b[4], b[5], b[6], b[7]]);
        let mut octets = [0u8; 16];
        octets.copy_from_slice(&b[8..24]);
        let scope_id = u32::from_ne_bytes([b[24], b[25], b[26], b[27]]);
        Some(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::from(octets),
            port,
            flowinfo,
            scope_id,
        )))
    } else {
        None
    }
}

/// `getaddrinfo( host, port, { socktype => SOCK_STREAM, protocol =>
/// IPPROTO_TCP, family, flags } )` as mosh.pl calls it.
#[cfg(unix)]
pub fn getaddrinfo(
    host: &str,
    port: &str,
    family: Option<Family>,
    flags: Flags,
) -> Result<AddrInfo, GaiError> {
    use std::ffi::{CStr, CString};
    let bad = |what: &str| GaiError {
        message: format!("{what} contains a NUL byte"),
        errno: 0,
    };
    let c_host = CString::new(host).map_err(|_| bad("host"))?;
    let c_port = CString::new(port).map_err(|_| bad("port"))?;
    // SAFETY: an all-zero addrinfo is a valid "no hints" value.
    let mut hints: libc::addrinfo = unsafe { std::mem::zeroed() };
    hints.ai_socktype = libc::SOCK_STREAM;
    hints.ai_protocol = libc::IPPROTO_TCP;
    hints.ai_family = match family {
        None => libc::AF_UNSPEC,
        Some(Family::Inet) => libc::AF_INET,
        Some(Family::Inet6) => libc::AF_INET6,
    };
    hints.ai_flags = match flags {
        Flags::None => 0,
        Flags::NumericHost => libc::AI_NUMERICHOST,
        Flags::CanonName => libc::AI_CANONNAME,
    };
    let mut res: *mut libc::addrinfo = std::ptr::null_mut();
    // SAFETY: valid C strings and hints; res receives a list we free below.
    let rc = unsafe { libc::getaddrinfo(c_host.as_ptr(), c_port.as_ptr(), &hints, &mut res) };
    if rc != 0 {
        let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        // SAFETY: gai_strerror returns a static NUL-terminated string.
        let message = unsafe { CStr::from_ptr(libc::gai_strerror(rc)) }
            .to_string_lossy()
            .into_owned();
        return Err(GaiError { message, errno });
    }
    let mut out = AddrInfo::default();
    let mut p = res;
    while !p.is_null() {
        // SAFETY: p is a node of the list getaddrinfo returned.
        let ai = unsafe { &*p };
        if out.canonname.is_none() && p == res && !ai.ai_canonname.is_null() {
            // SAFETY: ai_canonname is a NUL-terminated string owned by the list.
            out.canonname = Some(
                unsafe { CStr::from_ptr(ai.ai_canonname) }
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        if !ai.ai_addr.is_null() {
            // SAFETY: ai_addr points to ai_addrlen bytes owned by the list.
            let bytes = unsafe {
                std::slice::from_raw_parts(ai.ai_addr.cast::<u8>(), ai.ai_addrlen as usize)
            };
            out.addrs
                .extend(sockaddr_from_bytes(bytes, libc::AF_INET, libc::AF_INET6));
        }
        p = ai.ai_next;
    }
    // SAFETY: res came from a successful getaddrinfo and is freed once.
    unsafe { libc::freeaddrinfo(res) };
    Ok(out)
}

/// `getaddrinfo` as mosh.pl calls it, through WinSock.
#[cfg(windows)]
pub fn getaddrinfo(
    host: &str,
    port: &str,
    family: Option<Family>,
    flags: Flags,
) -> Result<AddrInfo, GaiError> {
    use std::ffi::{CStr, CString};
    use windows_sys::Win32::Networking::WinSock as ws;
    static WSA: std::sync::Once = std::sync::Once::new();
    WSA.call_once(|| {
        // SAFETY: zeroed WSADATA is a valid out buffer; version 2.2.
        let mut data: ws::WSADATA = unsafe { std::mem::zeroed() };
        // SAFETY: plain call; a failure shows up as a getaddrinfo error below.
        unsafe { ws::WSAStartup(0x0202, &mut data) };
    });
    let bad = |what: &str| GaiError {
        message: format!("{what} contains a NUL byte"),
        errno: 0,
    };
    let c_host = CString::new(host).map_err(|_| bad("host"))?;
    let c_port = CString::new(port).map_err(|_| bad("port"))?;
    let hints = ws::ADDRINFOA {
        ai_socktype: ws::SOCK_STREAM,
        ai_protocol: ws::IPPROTO_TCP,
        ai_family: match family {
            None => i32::from(ws::AF_UNSPEC),
            Some(Family::Inet) => i32::from(ws::AF_INET),
            Some(Family::Inet6) => i32::from(ws::AF_INET6),
        },
        // The flag values are small positive constants.
        ai_flags: match flags {
            Flags::None => 0,
            Flags::NumericHost => ws::AI_NUMERICHOST as i32,
            Flags::CanonName => ws::AI_CANONNAME as i32,
        },
        ..Default::default()
    };
    let mut res: *mut ws::ADDRINFOA = std::ptr::null_mut();
    // SAFETY: valid C strings and hints; res receives a list we free below.
    let rc = unsafe {
        ws::getaddrinfo(
            c_host.as_ptr().cast(),
            c_port.as_ptr().cast(),
            &hints,
            &mut res,
        )
    };
    if rc != 0 {
        return Err(GaiError {
            message: std::io::Error::from_raw_os_error(rc).to_string(),
            errno: 0,
        });
    }
    let mut out = AddrInfo::default();
    let mut p = res;
    while !p.is_null() {
        // SAFETY: p is a node of the list getaddrinfo returned.
        let ai = unsafe { &*p };
        if p == res && !ai.ai_canonname.is_null() {
            // SAFETY: ai_canonname is a NUL-terminated string owned by the list.
            out.canonname = Some(
                unsafe { CStr::from_ptr(ai.ai_canonname.cast()) }
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        if !ai.ai_addr.is_null() {
            // SAFETY: ai_addr points to ai_addrlen bytes owned by the list.
            let bytes =
                unsafe { std::slice::from_raw_parts(ai.ai_addr.cast::<u8>(), ai.ai_addrlen) };
            out.addrs.extend(sockaddr_from_bytes(
                bytes,
                i32::from(ws::AF_INET),
                i32::from(ws::AF_INET6),
            ));
        }
        p = ai.ai_next;
    }
    // SAFETY: res came from a successful getaddrinfo and is freed once.
    unsafe { ws::freeaddrinfo(res) };
    Ok(out)
}
