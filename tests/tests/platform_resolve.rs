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
// Ported from: (new; no upstream equivalent)
//
// The two places mosh resolves numeric addresses must agree: the network
// code's `socket::resolve` (upstream network.cc, used by mosh-client and
// mosh-server for the address on their command line) and the launcher's
// `netdb::getaddrinfo` (upstream mosh.pl `resolvename`). Review P10 found
// they decoded `sockaddr_in6` differently; a scoped IPv6 literal such as a
// link-local server address goes through both, so both must give the same
// address, port, scope id and flow information.

#![cfg(unix)]

use std::net::SocketAddr;

use mosh_platform::{netdb, socket};

#[test]
fn numeric_addresses_resolve_the_same_in_both_paths() {
    // `lo` is interface 1 on Linux; the numeric zone works everywhere.
    let cases = [
        ("fe80::1%1", "60001"),
        ("fe80::1%lo", "60001"),
        ("fe80::abcd:1234%1", "22"),
        ("::1", "60001"),
        ("127.0.0.1", "60001"),
    ];
    for (host, port) in cases {
        let via_socket = socket::resolve(Some(host), port, false)
            .unwrap_or_else(|e| panic!("socket::resolve({host}): {e:?}"));
        let via_netdb = netdb::getaddrinfo(host, port, None, netdb::Flags::NumericHost)
            .unwrap_or_else(|e| panic!("netdb::getaddrinfo({host}): {}", e.message));
        println!(
            "{host:<20} socket.rs {via_socket:?}  netdb.rs {:?}",
            via_netdb.addrs.first()
        );
        assert_eq!(via_netdb.addrs.first(), Some(&via_socket), "{host}");
        if let SocketAddr::V6(a) = via_socket {
            if host.contains('%') {
                assert_eq!(
                    a.scope_id(),
                    1,
                    "{host}: the zone must become scope id 1 (lo)"
                );
            }
            // getaddrinfo never sets a flow label; both paths read the raw
            // `sin6_flowinfo` field, as std's SocketAddrV6 does.
            assert_eq!(a.flowinfo(), 0, "{host}");
        }
    }
}
