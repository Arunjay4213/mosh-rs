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
// Ported from: upstream/src/network/transportsender.h, upstream/src/network/transportsender-impl.h

//! Port of upstream `src/network/transportsender.h` and `transportsender-impl.h`.
//!
//! The sender half of SSP. Upstream holds a `Connection*`; here the
//! owning `Transport` passes the connection into each call instead.

use super::network::{timestamp, Connection, MOSH_PROTOCOL_VERSION};
use super::transportfragment::Fragmenter;
use super::transportstate::{State, TimestampedState};
use super::Error;
use crate::crypto::prng::Prng;
use crate::crypto::{CryptoError, Session};
use crate::proto::transport_buffers::Instruction;
use mosh_platform::errln;

/* timing parameters */
/// Minimum ms between frames.
pub const SEND_INTERVAL_MIN: i32 = 20; /* ms between frames */
/// Maximum ms between frames.
pub const SEND_INTERVAL_MAX: i32 = 250; /* ms between frames */
/// Ms between empty acks.
pub const ACK_INTERVAL: u64 = 3000; /* ms between empty acks */
/// Ms before a delayed ack.
pub const ACK_DELAY: u64 = 100; /* ms before delayed ack */
/// Shutdown packets to send before giving up.
pub const SHUTDOWN_RETRIES: i32 = 16; /* number of shutdown packets to send before giving up */
/// Ms to keep resending at frame rate after last hearing from the peer.
pub const ACTIVE_RETRY_TIMEOUT: u64 = 10000; /* attempt to resend at frame rate */

/// The sending half of SSP: state history, timers, and acks (`Network::TransportSender`).
pub struct TransportSender<MyState: State> {
    /* state of sender */
    current_state: MyState,

    /* first element: known, acknowledged receiver state */
    /* last element: last sent state */
    sent_states: Vec<TimestampedState<MyState>>,

    /* somewhere in the middle: the assumed state of the receiver */
    assumed_receiver_state: usize,

    /* for fragment creation */
    fragmenter: Fragmenter,

    /* timing state */
    next_ack_time: u64,
    next_send_time: u64,

    verbose: u32,
    shutdown_in_progress: bool,
    shutdown_tries: i32,
    shutdown_start: u64,

    /* information about receiver state */
    ack_num: u64,
    pending_data_ack: bool,

    send_mindelay: u32, /* ms to collect all input */

    last_heard: u64, /* last time received new state */

    /* chaff to disguise instruction length */
    prng: Prng,

    mindelay_clock: u64, /* time of first pending change to current state */
}

impl<MyState: State> TransportSender<MyState> {
    /// A sender whose first state is `initial_state`, number 0.
    pub fn new(initial_state: &MyState) -> TransportSender<MyState> {
        TransportSender {
            current_state: initial_state.clone(),
            sent_states: vec![TimestampedState::new(timestamp(), 0, initial_state.clone())],
            assumed_receiver_state: 0,
            fragmenter: Fragmenter::new(),
            next_ack_time: timestamp(),
            next_send_time: timestamp(),
            verbose: 0,
            shutdown_in_progress: false,
            shutdown_tries: 0,
            shutdown_start: u64::MAX,
            ack_num: 0,
            pending_data_ack: false,
            send_mindelay: 8,
            last_heard: 0,
            prng: Prng::new(),
            mindelay_clock: u64::MAX,
        }
    }

    /// Try to send roughly two frames per RTT, bounded by limits on frame rate
    pub fn send_interval(&self, connection: &Connection) -> u32 {
        /* f64 to i32 saturates; SRTT is a non-negative RTT under 5000 ms, as upstream's lrint() to int */
        let send_interval = (connection.get_srtt() / 2.0).ceil() as i32;
        /* clamped to 20..=250, so the u32 is exact */
        send_interval.clamp(SEND_INTERVAL_MIN, SEND_INTERVAL_MAX) as u32
    }

    /// Housekeeping routine to calculate next send and ack times
    fn calculate_timers(&mut self, connection: &Connection) {
        let now = timestamp();

        /* Update assumed receiver state */
        self.update_assumed_receiver_state(connection);

        /* Cut out common prefix of all states */
        self.rationalize_states();

        if self.pending_data_ack && (self.next_ack_time > now + ACK_DELAY) {
            self.next_ack_time = now + ACK_DELAY;
        }

        let back = self.sent_states.last().expect("sent_states not empty");
        if self.current_state != back.state {
            if self.mindelay_clock == u64::MAX {
                self.mindelay_clock = now;
            }

            self.next_send_time = std::cmp::max(
                self.mindelay_clock.wrapping_add(self.send_mindelay as u64),
                back.timestamp + self.send_interval(connection) as u64,
            );
        } else if self.current_state != self.sent_states[self.assumed_receiver_state].state
            && self.last_heard + ACTIVE_RETRY_TIMEOUT > now
        {
            self.next_send_time = back.timestamp + self.send_interval(connection) as u64;
            if self.mindelay_clock != u64::MAX {
                self.next_send_time = std::cmp::max(
                    self.next_send_time,
                    self.mindelay_clock.wrapping_add(self.send_mindelay as u64),
                );
            }
        } else if self.current_state != self.sent_states[0].state
            && self.last_heard + ACTIVE_RETRY_TIMEOUT > now
        {
            self.next_send_time = back.timestamp + connection.timeout() + ACK_DELAY;
        } else {
            self.next_send_time = u64::MAX;
        }

        /* speed up shutdown sequence */
        if self.shutdown_in_progress || (self.ack_num == u64::MAX) {
            self.next_ack_time = back.timestamp + self.send_interval(connection) as u64;
        }
    }

    /// How many ms to wait until next event
    pub fn wait_time(&mut self, connection: &Connection) -> i32 {
        self.calculate_timers(connection);

        let mut next_wakeup = self.next_ack_time;
        if self.next_send_time < next_wakeup {
            next_wakeup = self.next_send_time;
        }

        let now = timestamp();

        if !connection.get_has_remote_addr() {
            return i32::MAX;
        }

        if next_wakeup > now {
            /* next_wakeup is at most next_ack_time, a few seconds ahead; upstream returns the same difference as int */
            (next_wakeup - now) as i32
        } else {
            0
        }
    }

    /// Send data or an empty ack if necessary
    pub fn tick(&mut self, connection: &mut Connection) -> Result<(), Error> {
        self.calculate_timers(connection); /* updates assumed receiver state and rationalizes */

        if !connection.get_has_remote_addr() {
            return Ok(());
        }

        let now = timestamp();

        if (now < self.next_ack_time) && (now < self.next_send_time) {
            return Ok(());
        }

        /* Determine if a new diff or empty ack needs to be sent */

        let mut diff = self
            .current_state
            .diff_from(&self.sent_states[self.assumed_receiver_state].state);

        self.attempt_prospective_resend_optimization(&mut diff);

        if self.verbose != 0 {
            /* verify diff has round-trip identity (modulo Unicode fallback rendering) */
            let mut newstate = self.sent_states[self.assumed_receiver_state].state.clone();
            if newstate.apply_string(&diff).is_err() {
                errln!("Warning, generated diff does not apply to the assumed receiver state!");
            }
            if self.current_state.compare(&newstate) {
                errln!("Warning, round-trip Instruction verification failed!");
            }

            /* Also verify that both the original frame and generated frame have the same initial diff. */
            let current_diff = self.current_state.init_diff();
            let new_diff = newstate.init_diff();
            if current_diff != new_diff {
                errln!("Warning, target state Instruction verification failed!");
            }
        }

        if diff.is_empty() {
            if now >= self.next_ack_time {
                self.send_empty_ack(connection)?;
                self.mindelay_clock = u64::MAX;
            }
            if now >= self.next_send_time {
                self.next_send_time = u64::MAX;
                self.mindelay_clock = u64::MAX;
            }
        } else if (now >= self.next_send_time) || (now >= self.next_ack_time) {
            /* Send diffs or ack */
            self.send_to_receiver(connection, &diff)?;
            self.mindelay_clock = u64::MAX;
        }
        Ok(())
    }

    fn send_empty_ack(&mut self, connection: &mut Connection) -> Result<(), Error> {
        let now = timestamp();

        assert!(now >= self.next_ack_time);

        let mut new_num = self.sent_states.last().expect("sent_states not empty").num + 1;

        /* special case for shutdown sequence */
        if self.shutdown_in_progress {
            new_num = u64::MAX;
        }

        //  sent_states.push_back( TimestampedState<MyState>( sent_states.back().timestamp, new_num, current_state ) );
        let state = self.current_state.clone();
        self.add_sent_state(now, new_num, state);
        self.send_in_fragments(connection, b"", new_num)?;

        self.next_ack_time = now + ACK_INTERVAL;
        self.next_send_time = u64::MAX;
        Ok(())
    }

    fn add_sent_state(&mut self, the_timestamp: u64, num: u64, state: MyState) {
        self.sent_states
            .push(TimestampedState::new(the_timestamp, num, state));
        if self.sent_states.len() > 32 {
            /* limit on state queue */
            let last = self.sent_states.len() - 16;
            self.sent_states.remove(last); /* erase state from middle of queue */
            /* upstream's list iterator keeps pointing at the same element;
            keep the index on it too (a dangling iterator upstream if it was
            the erased element; here it moves to the previous one) */
            if self.assumed_receiver_state >= last && self.assumed_receiver_state > 0 {
                self.assumed_receiver_state -= 1;
            }
        }
    }

    fn send_to_receiver(&mut self, connection: &mut Connection, diff: &[u8]) -> Result<(), Error> {
        let back = self.sent_states.last().expect("sent_states not empty");
        let mut new_num = if self.current_state == back.state {
            /* previously sent */
            back.num
        } else {
            /* new state */
            back.num + 1
        };

        /* special case for shutdown sequence */
        if self.shutdown_in_progress {
            new_num = u64::MAX;
        }

        if new_num == back.num {
            self.sent_states
                .last_mut()
                .expect("sent_states not empty")
                .timestamp = timestamp();
        } else {
            let state = self.current_state.clone();
            self.add_sent_state(timestamp(), new_num, state);
        }

        self.send_in_fragments(connection, diff, new_num)?; // Can throw NetworkException

        /* successfully sent, probably */
        /* ("probably" because the FIRST size-exceeded datagram doesn't get an error) */
        self.assumed_receiver_state = self.sent_states.len() - 1;
        self.next_ack_time = timestamp() + ACK_INTERVAL;
        self.next_send_time = u64::MAX;
        Ok(())
    }

    fn update_assumed_receiver_state(&mut self, connection: &Connection) {
        let now = timestamp();

        /* start from what is known and give benefit of the doubt to unacknowledged states
        transmitted recently enough ago */
        self.assumed_receiver_state = 0;

        for i in 1..self.sent_states.len() {
            assert!(now >= self.sent_states[i].timestamp);

            if now - self.sent_states[i].timestamp < connection.timeout() + ACK_DELAY {
                self.assumed_receiver_state = i;
            } else {
                return;
            }
        }
    }

    fn rationalize_states(&mut self) {
        let (known_receiver_state, rest) = self
            .sent_states
            .split_first_mut()
            .expect("sent_states always holds the acknowledged state");

        self.current_state.subtract(&known_receiver_state.state);

        for i in rest.iter_mut().rev() {
            i.state.subtract(&known_receiver_state.state);
        }
        /* last in upstream's reverse loop: the known state itself */
        known_receiver_state.state.subtract_self();
    }

    fn make_chaff(&mut self) -> Result<Vec<u8>, CryptoError> {
        const CHAFF_MAX: usize = 16;
        let chaff_len = self.prng.uint8()? as usize % (CHAFF_MAX + 1);

        let mut chaff = vec![0u8; chaff_len];
        self.prng.fill(&mut chaff)?;
        Ok(chaff)
    }

    fn send_in_fragments(
        &mut self,
        connection: &mut Connection,
        diff: &[u8],
        new_num: u64,
    ) -> Result<(), Error> {
        let inst = Instruction {
            protocol_version: Some(MOSH_PROTOCOL_VERSION),
            old_num: Some(self.sent_states[self.assumed_receiver_state].num),
            new_num: Some(new_num),
            ack_num: Some(self.ack_num),
            throwaway_num: Some(self.sent_states[0].num),
            diff: Some(diff.to_vec()),
            chaff: Some(self.make_chaff()?),
        };

        if new_num == u64::MAX {
            self.shutdown_tries += 1;
        }

        let fragments = self.fragmenter.make_fragments(
            &inst,
            /* MTU is at least DEFAULT_SEND_MTU (500), so the difference is positive */
            (connection.get_mtu() - Connection::ADDED_BYTES - Session::ADDED_BYTES as i32) as usize,
        )?;
        for i in &fragments {
            connection.send(&i.tostring())?;

            if self.verbose != 0 {
                errln!(
                    /* the as i32 casts below mirror upstream's (int) casts in this message: u64::MAX prints as -1 */
                    "[{}] Sent [{}=>{}] id {}, frag {} ack={}, throwaway={}, len={}, frame rate={:.2}, timeout={}, srtt={:.1}",
                    (timestamp() % 100000) as u32,
                    inst.old_num() as i32,
                    inst.new_num() as i32,
                    i.id as i32,
                    i.fragment_num as i32,
                    inst.ack_num() as i32,
                    inst.throwaway_num() as i32,
                    i.contents.len() as i32,
                    1000.0 / self.send_interval(connection) as f64,
                    connection.timeout() as i32,
                    connection.get_srtt()
                );
            }
        }

        self.pending_data_ack = false;
        Ok(())
    }

    /// Executed upon receipt of ack
    pub fn process_acknowledgment_through(&mut self, ack_num: u64) {
        /* Ignore ack if we have culled the state it's acknowledging */

        if self.sent_states.iter().any(|i| i.num == ack_num) {
            let mut erased_before_assumed = 0;
            for (idx, s) in std::mem::take(&mut self.sent_states)
                .into_iter()
                .enumerate()
            {
                if s.num < ack_num {
                    if idx < self.assumed_receiver_state {
                        erased_before_assumed += 1;
                    }
                } else {
                    self.sent_states.push(s);
                }
            }
            /* keep the index on the same element, as upstream's list iterator
            does (it is recomputed by calculate_timers() before its next use) */
            self.assumed_receiver_state = self
                .assumed_receiver_state
                .saturating_sub(erased_before_assumed);
        }

        assert!(!self.sent_states.is_empty());
    }

    /// give up on getting acknowledgement for shutdown
    pub fn shutdown_ack_timed_out(&self) -> bool {
        self.shutdown_in_progress
            && (self.shutdown_tries >= SHUTDOWN_RETRIES
                || timestamp().wrapping_sub(self.shutdown_start) >= ACTIVE_RETRY_TIMEOUT)
    }

    /// Executed upon entry to new receiver state
    pub fn set_ack_num(&mut self, s_ack_num: u64) {
        self.ack_num = s_ack_num;
    }

    /// Investigate diff against known receiver state instead
    /// Mutates proposed_diff
    fn attempt_prospective_resend_optimization(&mut self, proposed_diff: &mut Vec<u8>) {
        if self.assumed_receiver_state == 0 {
            return;
        }

        let resend_diff = self.current_state.diff_from(&self.sent_states[0].state);

        /* We do a prophylactic resend if it would make the diff shorter,
        or if it would lengthen it by no more than 100 bytes and still be
        less than 1000 bytes. */

        if (resend_diff.len() <= proposed_diff.len())
            || ((resend_diff.len() < 1000)
                && (resend_diff.len().wrapping_sub(proposed_diff.len()) < 100))
        {
            self.assumed_receiver_state = 0;
            *proposed_diff = resend_diff;
        }
    }

    /// Accelerate reply ack
    pub fn set_data_ack(&mut self) {
        self.pending_data_ack = true;
    }

    /// Received something
    pub fn remote_heard(&mut self, ts: u64) {
        self.last_heard = ts;
    }

    /// Starts shutdown sequence
    pub fn start_shutdown(&mut self) {
        if !self.shutdown_in_progress {
            self.shutdown_start = timestamp();
            self.shutdown_in_progress = true;
        }
    }

    /// Cannot modify current_state while shutdown in progress
    pub fn get_current_state(&mut self) -> &mut MyState {
        assert!(!self.shutdown_in_progress);
        &mut self.current_state
    }

    /// Replaces the current state and calls its `reset_input()`.
    pub fn set_current_state(&mut self, x: &MyState) {
        assert!(!self.shutdown_in_progress);
        self.current_state = x.clone();
        self.current_state.reset_input();
    }

    /// Verbosity of diagnostics on stderr (0 is quiet).
    pub fn set_verbose(&mut self, s_verbose: u32) {
        self.verbose = s_verbose;
    }

    /// Whether shutdown has started.
    pub fn get_shutdown_in_progress(&self) -> bool {
        self.shutdown_in_progress
    }

    /// Whether the peer acknowledged our shutdown state.
    pub fn get_shutdown_acknowledged(&self) -> bool {
        self.sent_states[0].num == u64::MAX
    }

    /// Whether we acknowledged the peer's shutdown.
    pub fn get_counterparty_shutdown_acknowledged(&self) -> bool {
        self.fragmenter.last_ack_sent() == u64::MAX
    }

    /// When the newest acknowledged state was sent.
    pub fn get_sent_state_acked_timestamp(&self) -> u64 {
        self.sent_states[0].timestamp
    }

    /// Number of the newest acknowledged state.
    pub fn get_sent_state_acked(&self) -> u64 {
        self.sent_states[0].num
    }

    /// Number of the newest sent state.
    pub fn get_sent_state_last(&self) -> u64 {
        self.sent_states.last().expect("sent_states not empty").num
    }

    /// Minimum ms to collect changes before sending.
    pub fn set_send_delay(&mut self, new_delay: u32) {
        self.send_mindelay = new_delay;
    }
}
