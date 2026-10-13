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
// Ported from: upstream/src/network (module root; no single upstream file)

//! Port of upstream `src/network`: the SSP transport. This file holds the
//! error type shared by the submodules (upstream throws `NetworkException`
//! or `CryptoException`) and `dos_detected` (upstream `dos_assert`).

pub mod compressor;
/// Port of `network.h`/`network.cc`: packets, timestamps, and the UDP `Connection`.
pub mod network;
/// Port of `networktransport.h`/`networktransport-impl.h`: `Transport`, sender plus receiver.
pub mod networktransport;
/// Port of `transportfragment.h`/`transportfragment.cc`.
pub mod transportfragment;
pub mod transportsender;
/// Port of `transportstate.h`, plus the `State` trait.
pub mod transportstate;

use std::fmt;

use crate::crypto::CryptoError;
use network::NetworkError;

/// The two exception types upstream's network layer can throw
/// (`Network::NetworkException` and `Crypto::CryptoException`).
/// Frontends handle them differently, as upstream's catch clauses do.
#[derive(Debug, Clone)]
pub enum Error {
    /// A `NetworkException`.
    Network(NetworkError),
    /// A `CryptoException`.
    Crypto(CryptoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(e) => e.fmt(f),
            Error::Crypto(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<NetworkError> for Error {
    fn from(e: NetworkError) -> Error {
        Error::Network(e)
    }
}

impl From<mosh_platform::socket::SysError> for Error {
    fn from(e: mosh_platform::socket::SysError) -> Error {
        Error::Network(e.into())
    }
}

impl From<CryptoError> for Error {
    fn from(e: CryptoError) -> Error {
        Error::Crypto(e)
    }
}

/// `dos_assert( expr )`: illegal counterparty input raises a non-fatal
/// CryptoException. Public so `State` implementations report malformed diffs
/// the same way.
pub fn dos_detected(expression: &str, function: &str) -> CryptoError {
    CryptoError::new(format!(
        "Illegal counterparty input (possible denial of service) in function {function}, failed test: {expression}\n"
    ))
}
