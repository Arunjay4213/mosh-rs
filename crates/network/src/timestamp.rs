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
// Ported from: upstream/src/util/timestamp.cc

//! Millisecond monotonic clock with a per-loop frozen value.
//!
//! Upstream reads `CLOCK_MONOTONIC` (milliseconds since boot) and caches
//! it in `freeze_timestamp()`. Here the clock is `std::time::Instant`
//! measured from the first call, plus `CLOCK_BASE_MS`. The base keeps the
//! value far from zero, as upstream's since-boot value is: the transport
//! initializes some times to 0 and compares `now - 0` against thresholds
//! of up to 60 s (`Connection::new_packet`, `TransportSender::calculate_timers`),
//! so a clock that starts at 0 would change behavior in the first minute.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::Instant;

const CLOCK_BASE_MS: u64 = 1_000_000_000;

static CLOCK_START: OnceLock<Instant> = OnceLock::new();
static MILLIS_CACHE: AtomicU64 = AtomicU64::new(u64::MAX);

pub fn frozen_timestamp() -> u64 {
    if MILLIS_CACHE.load(Ordering::Relaxed) == u64::MAX {
        freeze_timestamp();
    }
    MILLIS_CACHE.load(Ordering::Relaxed)
}

pub fn freeze_timestamp() {
    let start = CLOCK_START.get_or_init(Instant::now);
    /* u128 to u64: overflows only after 584 million years */
    let millis = CLOCK_BASE_MS + start.elapsed().as_millis() as u64;
    MILLIS_CACHE.store(millis, Ordering::Relaxed);
}
