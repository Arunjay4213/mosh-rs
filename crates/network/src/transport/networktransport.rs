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
// Ported from: upstream/src/network/networktransport.h, upstream/src/network/networktransport-impl.h

use std::net::SocketAddr;

use super::network::{timestamp, Connection, NetworkError, SocketHandle, MOSH_PROTOCOL_VERSION};
use super::transportfragment::{Fragment, FragmentAssembly};
use super::transportsender::TransportSender;
use super::transportstate::{State, TimestampedState};
use super::{dos_detected, Error};
use crate::crypto::CryptoError;

pub struct Transport<MyState: State, RemoteState: State> {
    /* the underlying, encrypted network connection */
    connection: Connection,

    /* sender side */
    sender: TransportSender<MyState>,

    /* simple receiver */
    received_states: Vec<TimestampedState<RemoteState>>,
    receiver_quench_timer: u64,
    last_receiver_state: RemoteState, /* the state we were in when user last queried state */
    fragments: FragmentAssembly,
    verbose: u32,
}

impl<MyState: State, RemoteState: State> Transport<MyState, RemoteState> {
    /// Server.
    pub fn new_server(
        initial_state: &MyState,
        initial_remote: &RemoteState,
        desired_ip: Option<&str>,
        desired_port: Option<&str>,
    ) -> Result<Transport<MyState, RemoteState>, Error> {
        let connection = Connection::new_server(desired_ip, desired_port)?;
        Ok(Transport::with_connection(
            connection,
            initial_state,
            initial_remote,
        ))
    }

    /// Client.
    pub fn new_client(
        initial_state: &MyState,
        initial_remote: &RemoteState,
        key_str: &str,
        ip: &str,
        port: &str,
    ) -> Result<Transport<MyState, RemoteState>, Error> {
        let connection = Connection::new_client(key_str, ip, port)?;
        Ok(Transport::with_connection(
            connection,
            initial_state,
            initial_remote,
        ))
    }

    fn with_connection(
        connection: Connection,
        initial_state: &MyState,
        initial_remote: &RemoteState,
    ) -> Transport<MyState, RemoteState> {
        let sender = TransportSender::new(initial_state);
        Transport {
            connection,
            sender,
            received_states: vec![TimestampedState::new(
                timestamp(),
                0,
                initial_remote.clone(),
            )],
            receiver_quench_timer: 0,
            last_receiver_state: initial_remote.clone(),
            fragments: FragmentAssembly::new(),
            verbose: 0,
        }
    }

    /// Send data or an ack if necessary.
    pub fn tick(&mut self) -> Result<(), Error> {
        self.sender.tick(&mut self.connection)
    }

    /// Returns the number of ms to wait until next possible event.
    pub fn wait_time(&mut self) -> i32 {
        self.sender.wait_time(&self.connection)
    }

    /// Blocks waiting for a packet.
    pub fn recv(&mut self) -> Result<(), Error> {
        let s = self.connection.recv()?;
        let frag = Fragment::from_bytes(&s)?;

        if self.fragments.add_fragment(&frag)? {
            /* complete packet */
            let inst = self.fragments.get_assembly()?;

            if inst.protocol_version() != MOSH_PROTOCOL_VERSION {
                return Err(NetworkError::new("mosh protocol version mismatch", 0).into());
            }

            self.sender.process_acknowledgment_through(inst.ack_num());

            /* inform network layer of roundtrip (end-to-end-to-end) connectivity */
            self.connection
                .set_last_roundtrip_success(self.sender.get_sent_state_acked_timestamp());

            /* first, make sure we don't already have the new state */
            if self.received_states.iter().any(|i| inst.new_num() == i.num) {
                return Ok(());
            }

            /* now, make sure we do have the old state */
            if !self.received_states.iter().any(|i| inst.old_num() == i.num) {
                //    fprintf( stderr, "Ignoring out-of-order packet. Reference state %d has been discarded or hasn't yet been
                //    received.\n", int(inst.old_num) );
                return Ok(()); /* this is security-sensitive and part of how we enforce idempotency */
            }

            /* Do not accept state if our queue is full */
            /* This is better than dropping states from the middle of the
            queue (as sender does), because we don't want to ACK a state
            and then discard it later. */

            self.process_throwaway_until(inst.throwaway_num())?;

            if self.received_states.len() > 1024 {
                /* limit on state queue */
                let now = timestamp();
                if now < self.receiver_quench_timer {
                    /* deny letting state grow further */
                    if self.verbose != 0 {
                        eprintln!(
                            "[{}] Receiver queue full, discarding {} (malicious sender or long-unidirectional connectivity?)",
                            (timestamp() % 100000) as u32,
                            inst.new_num() as i32
                        );
                    }
                    return Ok(());
                } else {
                    self.receiver_quench_timer = now + 15000;
                }
            }

            /* apply diff to reference state */
            /* upstream dereferences an iterator found before process_throwaway_until();
            look the reference state up again so a throwaway past old_num cannot
            leave it dangling */
            let reference_state = match self
                .received_states
                .iter()
                .find(|i| inst.old_num() == i.num)
            {
                Some(r) => r,
                None => return Ok(()),
            };
            let mut new_state = reference_state.clone();
            new_state.timestamp = timestamp();
            new_state.num = inst.new_num();

            if !inst.diff().is_empty() {
                new_state.state.apply_string(inst.diff());
            }

            /* Insert new state in sorted place */
            if let Some(pos) = self
                .received_states
                .iter()
                .position(|i| i.num > new_state.num)
            {
                let num = new_state.num;
                self.received_states.insert(pos, new_state);
                if self.verbose != 0 {
                    eprintln!(
                        "[{}] Received OUT-OF-ORDER state {} [ack {}]",
                        (timestamp() % 100000) as u32,
                        num as i32,
                        inst.ack_num() as i32
                    );
                }
                return Ok(());
            }
            if self.verbose != 0 {
                eprintln!(
                    "[{}] Received state {} [coming from {}, ack {}]",
                    (timestamp() % 100000) as u32,
                    new_state.num as i32,
                    inst.old_num() as i32,
                    inst.ack_num() as i32
                );
            }
            let new_state_timestamp = new_state.timestamp;
            self.received_states.push(new_state);
            self.sender.set_ack_num(
                self.received_states
                    .last()
                    .expect("received_states not empty")
                    .num,
            );

            self.sender.remote_heard(new_state_timestamp);
            if !inst.diff().is_empty() {
                self.sender.set_data_ack();
            }
        }
        Ok(())
    }

    /// The sender uses throwaway_num to tell us the earliest received state that we need to keep around
    /// Upstream `fatal_assert`s that a state survives; here a `throwaway_num`
    /// above every received state is an error and nothing is erased
    /// (PLAN.md section 9 item 8).
    fn process_throwaway_until(&mut self, throwaway_num: u64) -> Result<(), CryptoError> {
        if !self.received_states.iter().any(|i| i.num >= throwaway_num) {
            return Err(dos_detected(
                "received_states.size() > 0",
                "process_throwaway_until",
            ));
        }
        self.received_states.retain(|i| i.num >= throwaway_num);
        Ok(())
    }

    /// Find diff between last receiver state and current remote state, then rationalize states.
    pub fn get_remote_diff(&mut self) -> Vec<u8> {
        let ret = self
            .received_states
            .last()
            .expect("received_states not empty")
            .state
            .diff_from(&self.last_receiver_state);

        let oldest_receiver_state = self.received_states[0].state.clone();
        for i in self.received_states.iter_mut().rev() {
            i.state.subtract(&oldest_receiver_state);
        }

        self.last_receiver_state = self
            .received_states
            .last()
            .expect("received_states not empty")
            .state
            .clone();
        ret
    }

    /// Shut down other side of connection.
    /// Illegal to change current_state after this.
    pub fn start_shutdown(&mut self) {
        self.sender.start_shutdown();
    }

    pub fn shutdown_in_progress(&self) -> bool {
        self.sender.get_shutdown_in_progress()
    }

    pub fn shutdown_acknowledged(&self) -> bool {
        self.sender.get_shutdown_acknowledged()
    }

    pub fn shutdown_ack_timed_out(&self) -> bool {
        self.sender.shutdown_ack_timed_out()
    }

    pub fn has_remote_addr(&self) -> bool {
        self.connection.get_has_remote_addr()
    }

    /// Other side has requested shutdown and we have sent one ACK
    pub fn counterparty_shutdown_ack_sent(&self) -> bool {
        self.sender.get_counterparty_shutdown_acknowledged()
    }

    pub fn port(&self) -> Result<String, NetworkError> {
        self.connection.port()
    }

    pub fn get_key(&self) -> String {
        self.connection.get_key()
    }

    pub fn get_current_state(&mut self) -> &mut MyState {
        self.sender.get_current_state()
    }

    pub fn set_current_state(&mut self, x: &MyState) {
        self.sender.set_current_state(x);
    }

    pub fn get_remote_state_num(&self) -> u64 {
        self.received_states
            .last()
            .expect("received_states not empty")
            .num
    }

    pub fn get_latest_remote_state(&self) -> &TimestampedState<RemoteState> {
        self.received_states
            .last()
            .expect("received_states not empty")
    }

    pub fn fds(&self) -> Vec<SocketHandle> {
        self.connection.fds()
    }

    pub fn set_verbose(&mut self, s_verbose: u32) {
        self.sender.set_verbose(s_verbose);
        self.verbose = s_verbose;
    }

    pub fn set_send_delay(&mut self, new_delay: u32) {
        self.sender.set_send_delay(new_delay);
    }

    pub fn get_sent_state_acked_timestamp(&self) -> u64 {
        self.sender.get_sent_state_acked_timestamp()
    }

    pub fn get_sent_state_acked(&self) -> u64 {
        self.sender.get_sent_state_acked()
    }

    pub fn get_sent_state_last(&self) -> u64 {
        self.sender.get_sent_state_last()
    }

    pub fn send_interval(&self) -> u32 {
        self.sender.send_interval(&self.connection)
    }

    pub fn get_remote_addr(&self) -> Option<SocketAddr> {
        self.connection.get_remote_addr()
    }

    pub fn get_send_error(&mut self) -> &mut String {
        self.connection.get_send_error()
    }
}
