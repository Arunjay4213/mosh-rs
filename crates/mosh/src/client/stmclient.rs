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
// Ported from: upstream/src/frontend/stmclient.h, upstream/src/frontend/stmclient.cc

//! `STMClient`: the client's terminal setup, event loop, keyboard handling
//! (including the escape key), and rendering. Port of upstream
//! `src/frontend/stmclient.{h,cc}`, in upstream's order.
//!
//! Upstream's exceptions are `Result`s: the loop body returns the
//! transport's `Error`, and `main` handles it as upstream's catch clauses
//! do. The saved terminal state lives in a thread-local, not in the struct,
//! so the panic hook (`restore_terminal_after_panic`) can reach it.
//! PLAN 9.5: every non-empty frame is wrapped in synchronized output.

use std::cell::RefCell;
use std::io::Write;

use mosh_network::{freeze_timestamp, timestamp, Error, Transport};
use mosh_platform::locale::{get_ctype, is_utf8_locale, locale_charset};
use mosh_platform::select::{Select, SIGCONT, SIGHUP, SIGINT, SIGPIPE, SIGTERM, SIGWINCH};
use mosh_platform::tty::{self, RawModeGuard};
use mosh_terminal::{Display, Framebuffer, Resize, UserByte};

use super::terminaloverlay::{swprintf128, DisplayPreference, OverlayManager};
use crate::statesync::{Complete, UserStream};

type NetworkType = Transport<UserStream, Complete>;

thread_local! {
    /// Upstream's `saved_termios`/`raw_termios` (as the platform guard),
    /// plus `display.close()`, while the terminal is in raw mode.
    static TERMINAL: RefCell<Option<(RawModeGuard, Vec<u8>)>> = const { RefCell::new(None) };
}

/// Called by the panic hook: leave the alternate screen and restore the
/// terminal driver state, so the panic message lands on a sane terminal.
/// Does nothing when the terminal is not in raw mode.
pub fn restore_terminal_after_panic() {
    TERMINAL.with(|t| {
        if let Ok(mut t) = t.try_borrow_mut() {
            if let Some((guard, close)) = t.take() {
                let _ = tty::write_all_stdout(&close);
                let _ = tty::raw_mode_off(guard);
            }
        }
    });
}

fn with_guard(f: impl FnOnce(&RawModeGuard) -> std::io::Result<()>) -> std::io::Result<()> {
    TERMINAL.with(|t| match &*t.borrow() {
        Some((guard, _)) => f(guard),
        None => Ok(()),
    })
}

/// `perror( s )`: `s: <system message>`, without the `(os error N)` that
/// `io::Error` adds.
fn perror(s: &str, e: &std::io::Error) {
    match e.raw_os_error() {
        Some(errno) => errln!("{s}: {}", mosh_platform::socket::strerror(errno)),
        None => errln!("{s}: {e}"),
    }
}

/// `swrite( STDOUT_FILENO, ... )`: report a failed write, as upstream.
fn swrite(bytes: &[u8]) {
    if let Err(e) = tty::write_all_stdout(bytes) {
        perror("write", &e);
    }
}

/// PLAN 9.5: synchronized output around each non-empty render.
fn swrite_frame(diff: &str) {
    if diff.is_empty() {
        return;
    }
    let mut out = Vec::with_capacity(diff.len() + 16);
    out.extend_from_slice(b"\x1b[?2026h");
    out.extend_from_slice(diff.as_bytes());
    out.extend_from_slice(b"\x1b[?2026l");
    swrite(&out);
}

/// What one pass of the main loop asks `main` to do next.
enum Step {
    Continue,
    Break,
    ReturnFalse,
}

/// The client.
pub struct STMClient {
    ip: String,
    port: String,
    key: String,

    escape_key: i32,
    escape_pass_key: i32,
    escape_pass_key2: i32,
    escape_requires_lf: bool,
    escape_key_help: String,

    window_size: (u16, u16),

    local_framebuffer: Framebuffer,
    new_state: Framebuffer,
    overlays: OverlayManager,
    network: Option<NetworkType>,
    display: Display,

    connecting_notification: String,
    repaint_requested: bool,
    lf_entered: bool,
    quit_sequence_started: bool,
    clean_shutdown: bool,
    verbose: u32,

    sel: Select,
}

impl STMClient {
    fn still_connecting(&self) -> bool {
        /* Initially, network == NULL */
        self.network
            .as_ref()
            .is_some_and(|n| n.get_remote_state_num() == 0)
    }

    /// Exits with status 1 on an unknown prediction mode, as upstream.
    pub fn new(
        s_ip: &str,
        s_port: &str,
        s_key: &str,
        predict_mode: Option<&str>,
        s_verbose: u32,
        predict_overwrite: Option<&str>,
    ) -> STMClient {
        let mut c = STMClient {
            ip: s_ip.to_string(),
            port: s_port.to_string(),
            key: s_key.to_string(),
            escape_key: 0x1E,
            escape_pass_key: '^' as i32,
            escape_pass_key2: '^' as i32,
            escape_requires_lf: false,
            escape_key_help: "?".to_string(),
            window_size: (0, 0),
            local_framebuffer: Framebuffer::new(1, 1),
            new_state: Framebuffer::new(1, 1),
            overlays: OverlayManager::new(),
            network: None,
            display: Display::new(true), /* use TERM environment var to initialize display */
            connecting_notification: String::new(),
            repaint_requested: false,
            lf_entered: false,
            quit_sequence_started: false,
            clean_shutdown: false,
            verbose: s_verbose,
            sel: Select::new(),
        };
        if let Some(predict_mode) = predict_mode {
            let pref = match predict_mode {
                "always" => DisplayPreference::Always,
                "never" => DisplayPreference::Never,
                "adaptive" => DisplayPreference::Adaptive,
                "experimental" => DisplayPreference::Experimental,
                _ => {
                    errln!("Unknown prediction mode {predict_mode}.");
                    std::process::exit(1);
                }
            };
            c.overlays
                .get_prediction_engine()
                .set_display_preference(pref);
        }
        if predict_overwrite == Some("yes") {
            c.overlays
                .get_prediction_engine()
                .set_predict_overwrite(true);
        }
        c
    }

    fn resume(&mut self) {
        /* Restore termios state */
        if let Err(e) = with_guard(|g| g.apply_raw()) {
            perror("tcsetattr", &e);
            std::process::exit(1);
        }

        /* Put terminal in application-cursor-key mode */
        swrite(&self.display.open());

        /* Flag that outer terminal state is unknown */
        self.repaint_requested = true;
    }

    /// Check the locale, put the terminal in raw mode, and read the escape key settings.
    pub fn init(&mut self) {
        if !is_utf8_locale() {
            let native_ctype = get_ctype();
            let native_charset = locale_charset();

            err!(
                "mosh-client needs a UTF-8 native locale to run.\n\n\
                 Unfortunately, the client's environment ({}) specifies\n\
                 the character set \"{}\".\n\n",
                native_ctype.str(),
                native_charset
            );
            let _ = std::process::Command::new("locale").status();
            std::process::exit(1);
        }

        /* Verify terminal configuration, put terminal driver in raw mode */
        let guard = match tty::raw_mode_on() {
            Ok(g) => g,
            Err(e) => {
                perror("tcgetattr", &e);
                std::process::exit(1);
            }
        };
        TERMINAL.with(|t| *t.borrow_mut() = Some((guard, self.display.close())));

        /* Put terminal in application-cursor-key mode */
        swrite(&self.display.open());

        /* Add our name to window title */
        if std::env::var_os("MOSH_TITLE_NOPREFIX").is_none() {
            self.overlays.set_title_prefix("[mosh] ");
        }

        /* Set terminal escape key. */
        if let Some(escape_key_env) = std::env::var_os("MOSH_ESCAPE_KEY") {
            let escape_key_env = escape_key_env.as_encoded_bytes();
            if escape_key_env.len() == 1 {
                self.escape_key = i32::from(escape_key_env[0]);
                if self.escape_key > 0 && self.escape_key < 128 {
                    if self.escape_key < 32 {
                        /* If escape is ctrl-something, pass it with repeating the key without ctrl. */
                        self.escape_pass_key = self.escape_key + '@' as i32;
                    } else {
                        /* If escape is something else, pass it with repeating the key itself. */
                        self.escape_pass_key = self.escape_key;
                    }
                    if self.escape_pass_key >= 'A' as i32 && self.escape_pass_key <= 'Z' as i32 {
                        /* If escape pass is an upper case character, define optional version
                        as lower case of the same. */
                        self.escape_pass_key2 = self.escape_pass_key + 'a' as i32 - 'A' as i32;
                    } else {
                        self.escape_pass_key2 = self.escape_pass_key;
                    }
                } else {
                    self.escape_key = 0x1E;
                    self.escape_pass_key = '^' as i32;
                    self.escape_pass_key2 = '^' as i32;
                }
            } else if escape_key_env.is_empty() {
                self.escape_key = -1;
            } else {
                self.escape_key = 0x1E;
                self.escape_pass_key = '^' as i32;
                self.escape_pass_key2 = '^' as i32;
            }
        } else {
            self.escape_key = 0x1E;
            self.escape_pass_key = '^' as i32;
            self.escape_pass_key2 = '^' as i32;
        }

        /* There are so many better ways to shoot oneself into leg than
        setting escape key to Ctrl-C, Ctrl-D, NewLine, Ctrl-L or CarriageReturn
        that we just won't allow that. */
        if [0x03, 0x04, 0x0A, 0x0C, 0x0D].contains(&self.escape_key) {
            self.escape_key = 0x1E;
            self.escape_pass_key = '^' as i32;
            self.escape_pass_key2 = '^' as i32;
        }

        /* Adjust escape help differently if escape is a control character. */
        if self.escape_key > 0 {
            // Both keys are below 128 here, so they are ASCII characters.
            let pass_char = char::from(self.escape_pass_key as u8);
            let escape_pass_name = format!("\"{pass_char}\"");
            let escape_key_name = if self.escape_key < 32 {
                self.escape_requires_lf = false;
                format!("Ctrl-{pass_char}")
            } else {
                self.escape_requires_lf = true;
                format!("\"{}\"", char::from(self.escape_key as u8))
            };
            self.escape_key_help = format!(
                "Commands: Ctrl-Z suspends, \".\" quits, {escape_pass_name} gives literal {escape_key_name}"
            );
            self.overlays
                .get_notification_engine()
                .set_escape_key_string(&escape_key_name);
        }
        self.connecting_notification = swprintf128(format!(
            "Nothing received from server on UDP port {}.",
            self.port
        ));
    }

    /// Restore the screen and the terminal, and explain an unclean exit.
    pub fn shutdown(&mut self) {
        /* Restore screen state */
        self.overlays
            .get_notification_engine()
            .set_notification_string("", false, true);
        self.overlays
            .get_notification_engine()
            .server_heard(timestamp());
        self.overlays.set_title_prefix("");
        self.output_new_frame();

        /* Restore terminal and terminal-driver state */
        swrite(&self.display.close());

        if let Some((guard, _)) = TERMINAL.with(|t| t.borrow_mut().take()) {
            if let Err(e) = tty::raw_mode_off(guard) {
                perror("tcsetattr", &e);
                std::process::exit(1);
            }
        }

        if self.still_connecting() {
            err!(
                "\nmosh did not make a successful connection to {}:{}.\n\
                 Please verify that UDP port {} is not firewalled and can reach the server.\n\n\
                 (By default, mosh uses a UDP port between 60000 and 61000. The -p option\n\
                 selects a specific UDP port number.)\n",
                self.ip,
                self.port,
                self.port
            );
        } else if self.network.is_some() && !self.clean_shutdown {
            err!(
                "\n\nmosh did not shut down cleanly. Please note that the\n\
                 mosh-server process may still be running on the server.\n"
            );
        }
    }

    /// Returns false where upstream's `main_init` returns early (the window
    /// size cannot be read); upstream then dereferences a null network.
    fn main_init(&mut self) -> Result<bool, Error> {
        self.sel.add_signal(SIGWINCH);
        self.sel.add_signal(SIGTERM);
        self.sel.add_signal(SIGINT);
        self.sel.add_signal(SIGHUP);
        self.sel.add_signal(SIGPIPE);
        self.sel.add_signal(SIGCONT);

        /* get initial window size */
        self.window_size = match tty::window_size() {
            Ok(ws) => ws,
            Err(e) => {
                perror("ioctl TIOCGWINSZ", &e);
                return Ok(false);
            }
        };
        let (ws_col, ws_row) = self.window_size;

        /* local state */
        self.local_framebuffer = Framebuffer::new(i32::from(ws_col), i32::from(ws_row));
        self.new_state = Framebuffer::new(1, 1);

        /* initialize screen */
        let init = self
            .display
            .new_frame(false, &self.local_framebuffer, &self.local_framebuffer);
        swrite_frame(&init);

        /* open network */
        let blank = UserStream::new();
        let local_terminal = Complete::new(usize::from(ws_col), usize::from(ws_row));
        let mut network =
            NetworkType::new_client(&blank, &local_terminal, &self.key, &self.ip, &self.port)?;

        network.set_send_delay(1); /* minimal delay on outgoing keystrokes */

        /* tell server the size of the terminal */
        network
            .get_current_state()
            .push_back_resize(Resize::new(usize::from(ws_col), usize::from(ws_row)));

        /* be noisy as necessary */
        network.set_verbose(self.verbose);
        Select::set_verbose(self.verbose);

        self.network = Some(network);
        Ok(true)
    }

    fn output_new_frame(&mut self) {
        let Some(network) = &self.network else {
            /* clean shutdown even when not initialized */
            return;
        };

        /* fetch target state */
        self.new_state = network.get_latest_remote_state().state.get_fb().clone();

        /* apply local overlays */
        self.overlays.apply(&mut self.new_state);

        /* calculate minimal difference from where we are */
        let diff = self.display.new_frame(
            !self.repaint_requested,
            &self.local_framebuffer,
            &self.new_state,
        );
        swrite_frame(&diff);

        self.repaint_requested = false;

        // `local_framebuffer = new_state`; new_state is replaced before its next use.
        std::mem::swap(&mut self.local_framebuffer, &mut self.new_state);
    }

    fn process_network_input(&mut self) -> Result<(), Error> {
        let network = self.network.as_mut().expect("network initialized");
        network.recv()?;

        /* Now give hints to the overlays */
        let notifications = self.overlays.get_notification_engine();
        notifications.server_heard(network.get_latest_remote_state().timestamp);
        notifications.server_acked(network.get_sent_state_acked_timestamp());

        let predictions = self.overlays.get_prediction_engine();
        predictions.set_local_frame_acked(network.get_sent_state_acked());
        predictions.set_send_interval(network.send_interval());
        predictions
            .set_local_frame_late_acked(network.get_latest_remote_state().state.get_echo_ack());
        Ok(())
    }

    fn net(&mut self) -> &mut NetworkType {
        self.network.as_mut().expect("network initialized")
    }

    fn process_user_input(&mut self) -> bool {
        const BUF_SIZE: usize = 16384;
        let mut buf = [0u8; BUF_SIZE];

        /* fill buffer if possible */
        let bytes_read = match tty::read_stdin(&mut buf) {
            Ok(0) => return false, /* EOF */
            Ok(n) => n,
            Err(e) => {
                perror("read", &e);
                return false;
            }
        };

        if self.net().shutdown_in_progress() {
            return true;
        }
        let sent_state_last = self.net().get_sent_state_last();
        self.overlays
            .get_prediction_engine()
            .set_local_frame_sent(sent_state_last);

        /* Don't predict for bulk data. */
        let paste = bytes_read > 100;
        if paste {
            self.overlays.get_prediction_engine().reset();
        }

        for &the_byte in &buf[..bytes_read] {
            if !paste {
                self.overlays
                    .get_prediction_engine()
                    .new_user_byte(the_byte, &self.local_framebuffer);
            }

            let byte = i32::from(the_byte);

            if self.quit_sequence_started {
                if the_byte == b'.' {
                    /* Quit sequence is Ctrl-^ . */
                    if self.net().has_remote_addr() && (!self.net().shutdown_in_progress()) {
                        self.overlays
                            .get_notification_engine()
                            .set_notification_string("Exiting on user request...", true, true);
                        self.net().start_shutdown();
                        return true;
                    }
                    return false;
                } else if the_byte == 0x1a {
                    /* Suspend sequence is escape_key Ctrl-Z */
                    /* Restore terminal and terminal-driver state */
                    swrite(&self.display.close());

                    if let Err(e) = with_guard(|g| g.restore_saved()) {
                        perror("tcsetattr", &e);
                        std::process::exit(1);
                    }

                    out!("\n\x1b[37;44m[mosh is suspended.]\x1b[m\n");

                    let _ = std::io::stdout().flush();
                    let _ = std::io::stderr().flush();

                    /* actually suspend */
                    tty::suspend_self();

                    self.resume();
                } else if (byte == self.escape_pass_key) || (byte == self.escape_pass_key2) {
                    /* Emulation sequence to type escape_key is escape_key +
                    escape_pass_key (that is escape key without Ctrl) */
                    let escape_key = UserByte::new(self.escape_key as u8);
                    self.net()
                        .get_current_state()
                        .push_back_userbyte(escape_key);
                } else {
                    /* Escape key followed by anything other than . and ^ gets sent literally */
                    let escape_key = UserByte::new(self.escape_key as u8);
                    let state = self.net().get_current_state();
                    state.push_back_userbyte(escape_key);
                    state.push_back_userbyte(UserByte::new(the_byte));
                }

                self.quit_sequence_started = false;

                if self
                    .overlays
                    .get_notification_engine()
                    .get_notification_string()
                    == self.escape_key_help
                {
                    self.overlays
                        .get_notification_engine()
                        .set_notification_string("", false, true);
                }

                continue;
            }

            self.quit_sequence_started = (self.escape_key > 0)
                && (byte == self.escape_key)
                && (self.lf_entered || (!self.escape_requires_lf));
            if self.quit_sequence_started {
                self.lf_entered = false;
                let help = self.escape_key_help.clone();
                self.overlays
                    .get_notification_engine()
                    .set_notification_string(&help, true, false);
                continue;
            }

            /* LineFeed, Ctrl-J, '\n' or CarriageReturn, Ctrl-M, '\r' */
            self.lf_entered = (the_byte == 0x0A) || (the_byte == 0x0D);

            if the_byte == 0x0C {
                /* Ctrl-L */
                self.repaint_requested = true;
            }

            self.net()
                .get_current_state()
                .push_back_userbyte(UserByte::new(the_byte));
        }

        true
    }

    fn process_resize(&mut self) -> bool {
        /* get new size */
        self.window_size = match tty::window_size() {
            Ok(ws) => ws,
            Err(e) => {
                perror("ioctl TIOCGWINSZ", &e);
                return false;
            }
        };

        /* tell remote emulator */
        let (ws_col, ws_row) = self.window_size;
        let res = Resize::new(usize::from(ws_col), usize::from(ws_row));

        if !self.net().shutdown_in_progress() {
            self.net().get_current_state().push_back_resize(res);
        }

        /* note remote emulator will probably reply with its own Resize to adjust our state */

        /* tell prediction engine */
        self.overlays.get_prediction_engine().reset();

        true
    }

    /// The event loop. Ok(true) after a clean shutdown; Err for what
    /// upstream throws out of `main` (network setup errors, fatal crypto errors).
    pub fn main(&mut self) -> Result<bool, Error> {
        /* initialize signal handling and structures */
        if !self.main_init()? {
            return Ok(false);
        }

        loop {
            match self.main_loop_body() {
                Ok(Step::Continue) => {}
                Ok(Step::Break) => break,
                Ok(Step::ReturnFalse) => return Ok(false),
                Err(Error::Network(e)) => {
                    if !self.net().shutdown_in_progress() {
                        self.overlays
                            .get_notification_engine()
                            .set_network_error(&e.to_string());
                    }

                    std::thread::sleep(std::time::Duration::from_millis(200)); /* 0.2 sec */
                    freeze_timestamp();
                }
                Err(Error::Crypto(e)) => {
                    if e.fatal {
                        return Err(Error::Crypto(e));
                    }
                    let tmp = swprintf128(format!("Crypto exception: {e}"));
                    self.overlays
                        .get_notification_engine()
                        .set_notification_string(&tmp, false, true);
                }
            }
        }
        Ok(self.clean_shutdown)
    }

    /// One pass of upstream's `while ( 1 ) { try { ... } }`.
    fn main_loop_body(&mut self) -> Result<Step, Error> {
        self.output_new_frame();

        let mut wait_time = self.net().wait_time().min(self.overlays.wait_time());

        /* Handle startup "Connecting..." message */
        if self.still_connecting() {
            wait_time = wait_time.min(250);
        }

        /* poll for events */
        /* network->fd() can in theory change over time */
        self.sel.clear_fds();
        let fd_list = self.net().fds();
        for &fd in fd_list.iter() {
            self.sel.add_fd(fd);
        }
        self.sel.add_fd(tty::stdin_fd());

        let active_fds = self.sel.select(wait_time);
        // Upstream's Select::select freezes the clock before returning.
        freeze_timestamp();
        if let Err(e) = active_fds {
            perror("select", &e);
            return Ok(Step::Break);
        }

        let mut network_ready_to_read = false;

        for &fd in fd_list.iter() {
            if self.sel.read(fd) {
                /* packet received from the network */
                /* we only read one socket each run */
                network_ready_to_read = true;
            }
        }

        if network_ready_to_read {
            self.process_network_input()?;
        }

        if self.sel.read(tty::stdin_fd()) && !self.process_user_input() {
            /* input from the user needs to be fed to the network */
            if !self.net().has_remote_addr() {
                return Ok(Step::Break);
            } else if !self.net().shutdown_in_progress() {
                self.overlays
                    .get_notification_engine()
                    .set_notification_string("Exiting...", true, true);
                self.net().start_shutdown();
            }
        }

        if self.sel.signal(SIGWINCH) && !self.process_resize() {
            /* resize */
            return Ok(Step::ReturnFalse);
        }

        if self.sel.signal(SIGCONT) {
            self.resume();
        }

        // Upstream's `||` stops at the first signal it consumes; so does this.
        if self.sel.signal(SIGTERM)
            || self.sel.signal(SIGINT)
            || self.sel.signal(SIGHUP)
            || self.sel.signal(SIGPIPE)
        {
            /* shutdown signal */
            if !self.net().has_remote_addr() {
                return Ok(Step::Break);
            } else if !self.net().shutdown_in_progress() {
                self.overlays
                    .get_notification_engine()
                    .set_notification_string("Signal received, shutting down...", true, true);
                self.net().start_shutdown();
            }
        }

        /* quit if our shutdown has been acknowledged */
        if self.net().shutdown_in_progress() && self.net().shutdown_acknowledged() {
            self.clean_shutdown = true;
            return Ok(Step::Break);
        }

        /* quit after shutdown acknowledgement timeout */
        if self.net().shutdown_in_progress() && self.net().shutdown_ack_timed_out() {
            return Ok(Step::Break);
        }

        /* quit if we received and acknowledged a shutdown request */
        if self.net().counterparty_shutdown_ack_sent() {
            self.clean_shutdown = true;
            return Ok(Step::Break);
        }

        /* write diagnostic message if can't reach server */
        let since_remote_state =
            timestamp().wrapping_sub(self.net().get_latest_remote_state().timestamp);
        if self.still_connecting()
            && (!self.net().shutdown_in_progress())
            && (since_remote_state > 250)
        {
            if since_remote_state > 15000 {
                if !self.net().shutdown_in_progress() {
                    self.overlays
                        .get_notification_engine()
                        .set_notification_string("Timed out waiting for server...", true, true);
                    self.net().start_shutdown();
                }
            } else {
                let tmp = self.connecting_notification.clone();
                self.overlays
                    .get_notification_engine()
                    .set_notification_string(&tmp, false, true);
            }
        } else if (self.net().get_remote_state_num() != 0)
            && (self
                .overlays
                .get_notification_engine()
                .get_notification_string()
                == self.connecting_notification)
        {
            self.overlays
                .get_notification_engine()
                .set_notification_string("", false, true);
        }

        self.net().tick()?;

        let send_error = self.net().get_send_error();
        if !send_error.is_empty() {
            let e = std::mem::take(send_error);
            self.overlays
                .get_notification_engine()
                .set_network_error(&e);
        } else {
            self.overlays
                .get_notification_engine()
                .clear_network_error();
        }

        Ok(Step::Continue)
    }
}
