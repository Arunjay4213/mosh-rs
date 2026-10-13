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
// Ported from: upstream/src/protobufs/transportinstruction.proto, userinput.proto, hostinput.proto

//! Port of upstream `src/protobufs/transportinstruction.proto`, `userinput.proto` and `hostinput.proto`.
//!
//! Hand-written prost messages with proto2 semantics (every scalar is
//! `optional`, absent fields are not encoded, unknown fields are skipped
//! on decode).
//!
//! `userinput.proto` and `hostinput.proto` declare an empty
//! `Instruction { extensions 2 to max; }` and `extend` it. An extension
//! field is encoded exactly like a regular field with the same number,
//! so the extensions are ordinary optional fields here.

/// `package TransportBuffers`
pub mod transport_buffers {
    /// `TransportBuffers::Instruction`: one SSP message, the diff from state `old_num` to `new_num`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Instruction {
        /// `MOSH_PROTOCOL_VERSION` of the sender.
        #[prost(uint32, optional, tag = "1")]
        pub protocol_version: Option<u32>,
        /// State number the diff applies to.
        #[prost(uint64, optional, tag = "2")]
        pub old_num: Option<u64>,
        /// State number the diff produces.
        #[prost(uint64, optional, tag = "3")]
        pub new_num: Option<u64>,
        /// Highest state number of the peer that the sender has received.
        #[prost(uint64, optional, tag = "4")]
        pub ack_num: Option<u64>,
        /// Received states below this number can be discarded.
        #[prost(uint64, optional, tag = "5")]
        pub throwaway_num: Option<u64>,
        /// The state diff.
        #[prost(bytes = "vec", optional, tag = "6")]
        pub diff: Option<Vec<u8>>,
        /// Random padding that hides the instruction length.
        #[prost(bytes = "vec", optional, tag = "7")]
        pub chaff: Option<Vec<u8>>,
    }
}

/// `package ClientBuffers`
pub mod client_buffers {
    /// `ClientBuffers::UserMessage`: the client-to-server state diff.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserMessage {
        /// User events in order.
        #[prost(message, repeated, tag = "1")]
        pub instruction: Vec<Instruction>,
    }

    /// `ClientBuffers::Instruction`: one user event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Instruction {
        /// extension `keystroke = 2`
        #[prost(message, optional, tag = "2")]
        pub keystroke: Option<Keystroke>,
        /// extension `resize = 3`
        #[prost(message, optional, tag = "3")]
        pub resize: Option<ResizeMessage>,
    }

    /// `ClientBuffers::Keystroke`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Keystroke {
        /// Bytes typed by the user.
        #[prost(bytes = "vec", optional, tag = "4")]
        pub keys: Option<Vec<u8>>,
    }

    /// `ClientBuffers::ResizeMessage`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResizeMessage {
        /// New width in columns.
        #[prost(int32, optional, tag = "5")]
        pub width: Option<i32>,
        /// New height in rows.
        #[prost(int32, optional, tag = "6")]
        pub height: Option<i32>,
    }
}

/// `package HostBuffers`
pub mod host_buffers {
    /// `HostBuffers::HostMessage`: the server-to-client state diff.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostMessage {
        /// Host events in order.
        #[prost(message, repeated, tag = "1")]
        pub instruction: Vec<Instruction>,
    }

    /// `HostBuffers::Instruction`: one host event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Instruction {
        /// extension `hostbytes = 2`
        #[prost(message, optional, tag = "2")]
        pub hostbytes: Option<HostBytes>,
        /// extension `resize = 3`
        #[prost(message, optional, tag = "3")]
        pub resize: Option<ResizeMessage>,
        /// extension `echoack = 7`
        #[prost(message, optional, tag = "7")]
        pub echoack: Option<EchoAck>,
    }

    /// `HostBuffers::HostBytes`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostBytes {
        /// Terminal output that redraws the client's screen.
        #[prost(bytes = "vec", optional, tag = "4")]
        pub hoststring: Option<Vec<u8>>,
    }

    /// `HostBuffers::ResizeMessage`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResizeMessage {
        /// New width in columns.
        #[prost(int32, optional, tag = "5")]
        pub width: Option<i32>,
        /// New height in rows.
        #[prost(int32, optional, tag = "6")]
        pub height: Option<i32>,
    }

    /// `HostBuffers::EchoAck`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EchoAck {
        /// Number of the last user input the server has echoed.
        #[prost(uint64, optional, tag = "8")]
        pub echo_ack_num: Option<u64>,
    }
}
