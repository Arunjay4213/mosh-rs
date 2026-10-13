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
// Ported from: upstream/src/network/transportfragment.h, upstream/src/network/transportfragment.cc

use prost::Message as _;

use super::compressor::{compress_str, uncompress_str};
use super::dos_detected;
use crate::crypto::CryptoError;
use crate::proto::transport_buffers::Instruction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fragment {
    pub id: u64,
    pub fragment_num: u16,
    pub final_: bool,
    pub initialized: bool,
    pub contents: Vec<u8>,
}

impl Default for Fragment {
    fn default() -> Fragment {
        Fragment {
            id: u64::MAX,
            fragment_num: u16::MAX,
            final_: false,
            initialized: false,
            contents: Vec::new(),
        }
    }
}

impl Fragment {
    pub const FRAG_HEADER_LEN: usize = 8 /* uint64_t */ + 2 /* uint16_t */;

    pub fn new(s_id: u64, s_fragment_num: u16, s_final: bool, s_contents: Vec<u8>) -> Fragment {
        Fragment {
            id: s_id,
            fragment_num: s_fragment_num,
            final_: s_final,
            initialized: true,
            contents: s_contents,
        }
    }

    /// `Fragment( const std::string& x )`: parse a received fragment.
    /// Upstream aborts (`fatal_assert`) on a fragment shorter than the
    /// header; here it is an error (PLAN.md section 9 item 8).
    pub fn from_bytes(x: &[u8]) -> Result<Fragment, CryptoError> {
        let (Some(id_bytes), Some(num_bytes)) = (x.first_chunk::<8>(), x.get(8..10)) else {
            return Err(dos_detected("x.size() >= frag_header_len", "Fragment"));
        };
        let contents = x[Fragment::FRAG_HEADER_LEN..].to_vec();
        let id = u64::from_be_bytes(*id_bytes);
        let mut fragment_num = u16::from_be_bytes([num_bytes[0], num_bytes[1]]);
        let final_ = ((fragment_num & 0x8000) >> 15) != 0;
        fragment_num &= 0x7FFF;
        Ok(Fragment {
            id,
            fragment_num,
            final_,
            initialized: true,
            contents,
        })
    }

    pub fn tostring(&self) -> Vec<u8> {
        assert!(self.initialized);
        let mut ret = Vec::with_capacity(Fragment::FRAG_HEADER_LEN + self.contents.len());
        ret.extend_from_slice(&self.id.to_be_bytes());
        /* effective limit on size of a terminal screen change or buffered user input */
        assert!(
            self.fragment_num & 0x8000 == 0,
            "!( fragment_num & 0x8000 )"
        );
        let combined_fragment_num: u16 = ((self.final_ as u16) << 15) | self.fragment_num;
        ret.extend_from_slice(&combined_fragment_num.to_be_bytes());
        assert!(ret.len() == Fragment::FRAG_HEADER_LEN);
        ret.extend_from_slice(&self.contents);
        ret
    }
}

pub struct FragmentAssembly {
    fragments: Vec<Fragment>,
    current_id: u64,
    fragments_arrived: i32,
    fragments_total: i32,
}

impl Default for FragmentAssembly {
    fn default() -> FragmentAssembly {
        FragmentAssembly::new()
    }
}

impl FragmentAssembly {
    pub fn new() -> FragmentAssembly {
        FragmentAssembly {
            fragments: Vec::new(),
            current_id: u64::MAX,
            fragments_arrived: 0,
            fragments_total: -1,
        }
    }

    /// Adds a received fragment; returns true when the instruction is complete.
    /// Input that upstream `assert`s on (a conflicting duplicate, a final
    /// fragment below an index already stored) is an error here; the partial
    /// instruction is discarded (PLAN.md section 9 item 8).
    pub fn add_fragment(&mut self, frag: &Fragment) -> Result<bool, CryptoError> {
        let ret = self.add_fragment_checked(frag);
        if ret.is_err() {
            self.reset();
        }
        ret
    }

    fn reset(&mut self) {
        self.fragments.clear();
        self.current_id = u64::MAX;
        self.fragments_arrived = 0;
        self.fragments_total = -1;
    }

    fn add_fragment_checked(&mut self, frag: &Fragment) -> Result<bool, CryptoError> {
        let num = frag.fragment_num as usize;
        /* see if this is a totally new packet */
        if self.current_id != frag.id {
            self.fragments.clear();
            self.fragments.resize(num + 1, Fragment::default());
            self.fragments[num] = frag.clone();
            self.fragments_arrived = 1;
            self.fragments_total = -1; /* unknown */
            self.current_id = frag.id;
        } else {
            /* not a new packet */
            /* see if we already have this fragment */
            if self.fragments.len() > num && self.fragments[num].initialized {
                /* make sure new version is same as what we already have */
                if self.fragments[num] != *frag {
                    return Err(dos_detected(
                        "fragments.at( frag.fragment_num ) == frag",
                        "add_fragment",
                    ));
                }
            } else {
                if self.fragments.len() < num + 1 {
                    self.fragments.resize(num + 1, Fragment::default());
                }
                self.fragments[num] = frag.clone();
                self.fragments_arrived += 1;
            }
        }

        if frag.final_ {
            /* num < 0x8000, so the i32 cannot overflow */
            self.fragments_total = num as i32 + 1;
            if self.fragments.len() > num + 1 {
                return Err(dos_detected(
                    "(int)fragments.size() <= fragments_total",
                    "add_fragment",
                ));
            }
            self.fragments
                /* fragments_total = num + 1 here, positive */
                .resize(self.fragments_total as usize, Fragment::default());
        }

        if self.fragments_total != -1 && self.fragments_arrived > self.fragments_total {
            return Err(dos_detected(
                "fragments_arrived <= fragments_total",
                "add_fragment",
            ));
        }

        /* see if we're done */
        Ok(self.fragments_arrived == self.fragments_total)
    }

    pub fn get_assembly(&mut self) -> Result<Instruction, CryptoError> {
        assert!(self.fragments_arrived == self.fragments_total);

        let mut encoded = Vec::new();
        /* positive: get_assembly is only called after add_fragment set a total */
        for i in 0..self.fragments_total as usize {
            assert!(self.fragments[i].initialized);
            encoded.extend_from_slice(&self.fragments[i].contents);
        }

        /* upstream: fatal_assert( ret.ParseFromString( ... ) ); an error here */
        let ret = uncompress_str(&encoded).and_then(|raw| {
            Instruction::decode(raw.as_slice()).map_err(|_| {
                dos_detected(
                    "ret.ParseFromString( get_compressor().uncompress_str( encoded ) )",
                    "get_assembly",
                )
            })
        });

        self.fragments.clear();
        self.fragments_arrived = 0;
        self.fragments_total = -1;

        ret
    }
}

pub struct Fragmenter {
    next_instruction_id: u64,
    last_instruction: Instruction,
    last_mtu: usize,
}

impl Default for Fragmenter {
    fn default() -> Fragmenter {
        Fragmenter::new()
    }
}

impl Fragmenter {
    pub fn new() -> Fragmenter {
        Fragmenter {
            next_instruction_id: 0,
            last_instruction: Instruction {
                old_num: Some(u64::MAX),
                new_num: Some(u64::MAX),
                ..Default::default()
            },
            last_mtu: usize::MAX,
        }
    }

    pub fn make_fragments(
        &mut self,
        inst: &Instruction,
        mtu: usize,
    ) -> Result<Vec<Fragment>, CryptoError> {
        let mtu = mtu.wrapping_sub(Fragment::FRAG_HEADER_LEN);
        let last = &self.last_instruction;
        if inst.old_num() != last.old_num()
            || inst.new_num() != last.new_num()
            || inst.ack_num() != last.ack_num()
            || inst.throwaway_num() != last.throwaway_num()
            || inst.chaff() != last.chaff()
            || inst.protocol_version() != last.protocol_version()
            || self.last_mtu != mtu
        {
            self.next_instruction_id += 1;
        }

        if inst.old_num() == last.old_num() && inst.new_num() == last.new_num() {
            assert!(
                inst.diff() == last.diff(),
                "inst.diff() == last_instruction.diff()"
            );
        }

        self.last_instruction = inst.clone();
        self.last_mtu = mtu;

        let mut payload: &[u8] = &compress_str(&inst.encode_to_vec())?;
        let mut fragment_num: u16 = 0;
        let mut ret = Vec::new();

        while !payload.is_empty() {
            let this_fragment;
            let mut final_ = false;

            if payload.len() > mtu {
                this_fragment = &payload[..mtu];
                payload = &payload[mtu..];
            } else {
                this_fragment = payload;
                payload = &[];
                final_ = true;
            }

            ret.push(Fragment::new(
                self.next_instruction_id,
                fragment_num,
                final_,
                this_fragment.to_vec(),
            ));
            fragment_num = fragment_num.wrapping_add(1);
        }

        Ok(ret)
    }

    pub fn last_ack_sent(&self) -> u64 {
        self.last_instruction.ack_num()
    }
}
