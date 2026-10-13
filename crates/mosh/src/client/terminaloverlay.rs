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
// Ported from: upstream/src/frontend/terminaloverlay.h, upstream/src/frontend/terminaloverlay.cc

//! The client's local overlays on top of the server's framebuffer: the
//! notification bar, the window title prefix, and the prediction engine
//! (local echo of typed characters, confirmed or killed when the server's
//! frame arrives). Port of upstream `src/frontend/terminaloverlay.{h,cc}`,
//! in upstream's order.
//!
//! Upstream's class hierarchy becomes plain structs: `ConditionalOverlay` is
//! embedded as `base` in `ConditionalCursorMove` and `ConditionalOverlayCell`
//! (their `apply` and `get_validity` are never called through the base
//! class). `std::list` becomes `Vec`, and list iterators that upstream keeps
//! across calls that change other elements become indices.
//! All `uint64_t` arithmetic wraps, as it does in C++.

use mosh_network::timestamp;
use mosh_network::transport::transportsender::ACK_INTERVAL;
use mosh_terminal::terminalframebuffer::{attribute_type, title_type};
use mosh_terminal::wcwidth::wcwidth;
use mosh_terminal::{ActionType, Actions, Cell, Framebuffer, UTF8Parser};

/// `INT_MAX`.
const INT_MAX: i32 = i32::MAX;

/// How a prediction compares with the server's framebuffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Validity {
    /// Not confirmed or refuted yet.
    Pending,
    /// The server shows what was predicted.
    Correct,
    /// Correct, but no evidence that prediction works (blank, unknown, or unchanged).
    CorrectNoCredit,
    /// The server shows something else, or the prediction is off screen.
    IncorrectOrExpired,
    /// Not a prediction at all.
    Inactive,
}

/// What every prediction has (upstream base class `ConditionalOverlay`).
#[derive(Clone, Debug)]
pub struct ConditionalOverlay {
    /// The prediction is checked once the server has echoed this frame.
    pub expiration_frame: u64,
    /// Column.
    pub col: i32,
    /// represents a prediction at all
    pub active: bool,
    /// when to show
    pub tentative_until_epoch: u64,
    /// used to find long-pending predictions
    pub prediction_time: u64,
}

impl ConditionalOverlay {
    fn new(s_exp: u64, s_col: i32, s_tentative: u64) -> ConditionalOverlay {
        ConditionalOverlay {
            expiration_frame: s_exp,
            col: s_col,
            active: false,
            tentative_until_epoch: s_tentative,
            prediction_time: u64::MAX,
        }
    }

    /// Whether the prediction belongs to an epoch not confirmed yet.
    pub fn tentative(&self, confirmed_epoch: u64) -> bool {
        self.tentative_until_epoch > confirmed_epoch
    }

    /// No prediction.
    pub fn reset(&mut self) {
        self.expiration_frame = u64::MAX;
        self.tentative_until_epoch = u64::MAX;
        self.active = false;
    }

    /// Checked once frame `s_exp` is echoed; predicted at `now`.
    pub fn expire(&mut self, s_exp: u64, now: u64) {
        self.expiration_frame = s_exp;
        self.prediction_time = now;
    }
}

/// A predicted cursor position.
#[derive(Clone, Debug)]
pub struct ConditionalCursorMove {
    /// The common prediction fields (`col` is the predicted column).
    pub base: ConditionalOverlay,
    /// Predicted row.
    pub row: i32,
}

/// A predicted cell.
#[derive(Clone, Debug)]
pub struct ConditionalOverlayCell {
    /// The common prediction fields.
    pub base: ConditionalOverlay,
    /// What the cell is predicted to show.
    pub replacement: Cell,
    /// The cell will change, but to what is not known.
    pub unknown: bool,
    /// we don't give credit for correct predictions that match the original contents
    pub original_contents: Vec<Cell>,
}

/// The predicted cells of one row.
#[derive(Clone, Debug)]
pub struct ConditionalOverlayRow {
    /// Row number.
    pub row_num: i32,
    /// One cell prediction per column.
    pub overlay_cells: Vec<ConditionalOverlayCell>,
}

impl ConditionalOverlayCell {
    fn new(s_exp: u64, s_col: i32, s_tentative: u64) -> ConditionalOverlayCell {
        ConditionalOverlayCell {
            base: ConditionalOverlay::new(s_exp, s_col, s_tentative),
            replacement: Cell::new(0),
            unknown: false,
            original_contents: Vec::new(),
        }
    }

    /// No prediction, and forget the original contents.
    pub fn reset(&mut self) {
        self.unknown = false;
        self.original_contents.clear();
        self.base.reset();
    }

    /// No prediction, but remember the replacement as an original content.
    pub fn reset_with_orig(&mut self) {
        if (!self.base.active) || self.unknown {
            self.reset();
            return;
        }

        self.original_contents.push(self.replacement.clone());
        self.base.reset();
    }
}

impl ConditionalOverlayCell {
    /// Draw the prediction into `fb` (underlined when `flag`).
    pub fn apply(&self, fb: &mut Framebuffer, confirmed_epoch: u64, row: i32, mut flag: bool) {
        let col = self.base.col;
        if (!self.base.active) || (row >= fb.ds.get_height()) || (col >= fb.ds.get_width()) {
            return;
        }

        if self.base.tentative(confirmed_epoch) {
            return;
        }

        if self.replacement.is_blank() && fb.get_cell(row, col).is_blank() {
            flag = false;
        }

        if self.unknown {
            if flag && (col != fb.ds.get_width() - 1) {
                fb.get_mutable_cell(row, col)
                    .get_renditions_mut()
                    .set_attribute(attribute_type::underlined, true);
            }
            return;
        }

        if *fb.get_cell(row, col) != self.replacement {
            *fb.get_mutable_cell(row, col) = self.replacement.clone();
            if flag {
                fb.get_mutable_cell(row, col)
                    .get_renditions_mut()
                    .set_attribute(attribute_type::underlined, true);
            }
        }
    }

    /// Compare the prediction with the server's framebuffer.
    pub fn get_validity(
        &self,
        fb: &Framebuffer,
        row: i32,
        _early_ack: u64,
        late_ack: u64,
    ) -> Validity {
        if !self.base.active {
            return Validity::Inactive;
        }

        let col = self.base.col;
        if (row >= fb.ds.get_height()) || (col >= fb.ds.get_width()) {
            return Validity::IncorrectOrExpired;
        }

        let current = fb.get_cell(row, col);

        /* see if it hasn't been updated yet */
        if late_ack < self.base.expiration_frame {
            return Validity::Pending;
        }

        if self.unknown {
            return Validity::CorrectNoCredit;
        }

        if self.replacement.is_blank() {
            /* too easy for this to trigger falsely */
            return Validity::CorrectNoCredit;
        }

        if current.contents_match(&self.replacement) {
            if !self
                .original_contents
                .iter()
                .any(|it| it.contents_match(&self.replacement))
            {
                return Validity::Correct;
            }
            return Validity::CorrectNoCredit;
        }
        Validity::IncorrectOrExpired
    }
}

impl ConditionalCursorMove {
    fn new(s_exp: u64, s_row: i32, s_col: i32, s_tentative: u64) -> ConditionalCursorMove {
        ConditionalCursorMove {
            base: ConditionalOverlay::new(s_exp, s_col, s_tentative),
            row: s_row,
        }
    }

    /// Compare the predicted cursor with the server's cursor.
    pub fn get_validity(&self, fb: &Framebuffer, _early_ack: u64, late_ack: u64) -> Validity {
        if !self.base.active {
            return Validity::Inactive;
        }

        if (self.row >= fb.ds.get_height()) || (self.base.col >= fb.ds.get_width()) {
            return Validity::IncorrectOrExpired;
        }

        if late_ack >= self.base.expiration_frame {
            if (fb.ds.get_cursor_col() == self.base.col) && (fb.ds.get_cursor_row() == self.row) {
                return Validity::Correct;
            }
            return Validity::IncorrectOrExpired;
        }

        Validity::Pending
    }

    /// Move `fb`'s cursor to the prediction.
    pub fn apply(&self, fb: &mut Framebuffer, confirmed_epoch: u64) {
        if !self.base.active {
            return;
        }

        if self.base.tentative(confirmed_epoch) {
            return;
        }

        assert!(self.row < fb.ds.get_height());
        assert!(self.base.col < fb.ds.get_width());
        assert!(!fb.ds.origin_mode);

        fb.ds.move_row(self.row, false);
        fb.ds.move_col(self.base.col, false, false);
    }
}

/// The notification bar: connection trouble and messages.
#[derive(Clone, Debug)]
pub struct NotificationEngine {
    last_word_from_server: u64,
    last_acked_state: u64,
    escape_key_string: String,
    message: String,
    message_is_network_error: bool,
    message_expiration: u64,
    show_quit_keystroke: bool,
}

impl Default for NotificationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// `swprintf( tmp, 128, ... )`: glibc keeps at most 127 wide characters
/// when the result does not fit (checked on devbox).
pub(crate) fn swprintf128(s: String) -> String {
    match s.char_indices().nth(127) {
        Some((i, _)) => s[..i].to_string(),
        None => s,
    }
}

impl NotificationEngine {
    fn server_late(&self, ts: u64) -> bool {
        ts.wrapping_sub(self.last_word_from_server) > 6500
    }

    fn reply_late(&self, ts: u64) -> bool {
        ts.wrapping_sub(self.last_acked_state) > 10000
    }

    fn need_countup(&self, ts: u64) -> bool {
        self.server_late(ts) || self.reply_late(ts)
    }

    /// The current message (empty when none).
    pub fn get_notification_string(&self) -> &str {
        &self.message
    }

    /// The server was last heard at `s_last_word`.
    pub fn server_heard(&mut self, s_last_word: u64) {
        self.last_word_from_server = s_last_word;
    }

    /// The server last acknowledged a state sent at `s_last_acked`.
    pub fn server_acked(&mut self, s_last_acked: u64) {
        self.last_acked_state = s_last_acked;
    }

    /// Show `s_message`; it expires after 1 s unless `permanent`.
    /// Upstream's defaults are `permanent = false`, `s_show_quit_keystroke = true`.
    pub fn set_notification_string(
        &mut self,
        s_message: &str,
        permanent: bool,
        s_show_quit_keystroke: bool,
    ) {
        self.message = s_message.to_string();
        if permanent {
            self.message_expiration = u64::MAX;
        } else {
            self.message_expiration = timestamp() + 1000;
        }
        self.message_is_network_error = false;
        self.show_quit_keystroke = s_show_quit_keystroke;
    }

    /// The quit hint, e.g. ` [To quit: Ctrl-^ .]`.
    pub fn set_escape_key_string(&mut self, s_name: &str) {
        self.escape_key_string = format!(" [To quit: {s_name} .]");
    }

    /// Show a network error until the next empty-ack interval passes.
    pub fn set_network_error(&mut self, s: &str) {
        self.message = swprintf128(s.to_string());
        self.message_is_network_error = true;
        self.message_expiration = timestamp() + ACK_INTERVAL + 100;
    }

    /// Let a network error expire within a second.
    pub fn clear_network_error(&mut self) {
        if self.message_is_network_error {
            self.message_expiration = self.message_expiration.min(timestamp() + 1000);
        }
    }

    /// A fresh engine: the server was just heard from and nothing is shown.
    pub fn new() -> NotificationEngine {
        NotificationEngine {
            last_word_from_server: timestamp(),
            last_acked_state: timestamp(),
            escape_key_string: String::new(),
            message: String::new(),
            message_is_network_error: false,
            message_expiration: u64::MAX,
            show_quit_keystroke: true,
        }
    }
}

fn human_readable_duration(num_seconds: i32, seconds_abbr: &str) -> String {
    if num_seconds < 60 {
        format!("{num_seconds} {seconds_abbr}")
    } else if num_seconds < 3600 {
        format!("{}:{:02}", num_seconds / 60, num_seconds % 60)
    } else {
        format!(
            "{}:{:02}:{:02}",
            num_seconds / 3600,
            (num_seconds / 60) % 60,
            num_seconds % 60
        )
    }
}

impl NotificationEngine {
    /// Draw the notification bar on the top row of `fb`.
    pub fn apply(&self, fb: &mut Framebuffer) {
        let now = timestamp();

        let time_expired = self.need_countup(now);

        if self.message.is_empty() && !time_expired {
            return;
        }

        assert!(fb.ds.get_width() > 0);
        assert!(fb.ds.get_height() > 0);

        /* hide cursor if necessary */
        if fb.ds.get_cursor_row() == 0 {
            fb.ds.cursor_visible = false;
        }

        /* draw bar across top of screen */
        let mut notification_bar = Cell::new(0);
        notification_bar
            .get_renditions_mut()
            .set_foreground_color(7);
        notification_bar
            .get_renditions_mut()
            .set_background_color(4);
        notification_bar.append(0x20);

        for i in 0..fb.ds.get_width() {
            *fb.get_mutable_cell(0, i) = notification_bar.clone();
        }

        /* write message */

        /* We want to prefer the "last contact" message if we simply haven't
        heard from the server in a while, but print the "last reply" message
        if the problem is uplink-only. */

        let since_heard = now.wrapping_sub(self.last_word_from_server) as f64 / 1000.0;
        let since_ack = now.wrapping_sub(self.last_acked_state) as f64 / 1000.0;
        let server_message = "contact";
        let reply_message = "reply";

        let mut time_elapsed = since_heard;
        let mut explanation = server_message;

        if self.reply_late(now) && (!self.server_late(now)) {
            time_elapsed = since_ack;
            explanation = reply_message;
        }

        let keystroke_str = if self.show_quit_keystroke {
            self.escape_key_string.as_str()
        } else {
            ""
        };

        if self.message.is_empty() && (!time_expired) {
            return;
        }
        let tmp = if self.message.is_empty() && time_expired {
            format!(
                "mosh: Last {} {} ago.{}",
                explanation,
                human_readable_duration(time_elapsed as i32, "seconds"),
                keystroke_str
            )
        } else if (!self.message.is_empty()) && (!time_expired) {
            format!("mosh: {}{}", self.message, keystroke_str)
        } else {
            format!(
                "mosh: {} ({} without {}.){}",
                self.message,
                human_readable_duration(time_elapsed as i32, "s"),
                explanation,
                keystroke_str
            )
        };

        let string_to_draw = swprintf128(tmp);

        let mut overlay_col = 0;

        // Upstream's `Cell* combining_cell`, as a column of row 0.
        let mut combining_cell: Option<i32> = Some(0);

        /* We unfortunately duplicate the terminal's logic for how to render a Unicode sequence into graphemes */
        for ch in string_to_draw.chars() {
            if overlay_col >= fb.ds.get_width() {
                break;
            }

            let ch = ch as u32;
            let chwidth = if ch == 0 { -1 } else { wcwidth(ch) };

            match chwidth {
                1 | 2 => {
                    /* normal character, wide character */
                    fb.reset_cell(0, overlay_col);
                    let this_cell = fb.get_mutable_cell(0, overlay_col);
                    this_cell
                        .get_renditions_mut()
                        .set_attribute(attribute_type::bold, true);
                    this_cell.get_renditions_mut().set_foreground_color(7);
                    this_cell.get_renditions_mut().set_background_color(4);

                    this_cell.append(ch);
                    this_cell.set_wide(chwidth == 2);
                    combining_cell = Some(overlay_col);

                    overlay_col += chwidth;
                }
                0 => {
                    /* combining character */
                    let Some(cc) = combining_cell else {
                        continue;
                    };
                    let combining_cell = fb.get_mutable_cell(0, cc);

                    if combining_cell.empty() {
                        assert!(!combining_cell.get_wide());
                        combining_cell.set_fallback(true);
                        overlay_col += 1;
                    }

                    if !combining_cell.full() {
                        combining_cell.append(ch);
                    }
                }
                -1 => { /* unprintable character */ }
                _ => panic!("unexpected character width from wcwidth()"),
            }
        }
    }

    /// Drop the message once it has expired.
    pub fn adjust_message(&mut self) {
        if timestamp() >= self.message_expiration {
            self.message.clear();
        }
    }

    /// Milliseconds until the bar needs redrawing.
    pub fn wait_time(&self) -> i32 {
        let mut next_expiry: u64 = INT_MAX as u64;

        let now = timestamp();

        next_expiry = next_expiry.min(self.message_expiration.wrapping_sub(now));

        if self.need_countup(now) {
            let mut countup_interval: u64 = 1000;
            if now.wrapping_sub(self.last_word_from_server) > 60000 {
                /* If we've been disconnected for 60 seconds, save power by updating the
                display less often.  See #243. */
                countup_interval = ACK_INTERVAL;
            }
            next_expiry = next_expiry.min(countup_interval);
        }

        // At most INT_MAX, so the conversion is exact.
        next_expiry as i32
    }
}

/// When to show predictions (`MOSH_PREDICTION_DISPLAY`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayPreference {
    /// Always show.
    Always,
    /// Never predict.
    Never,
    /// Show when the connection is slow (the default).
    Adaptive,
    /// No epochs; wrong cells are dropped one by one.
    Experimental,
}

/// The prediction engine: local echo of typed characters and cursor moves.
#[derive(Clone, Debug)]
pub struct PredictionEngine {
    last_byte: u8,
    parser: UTF8Parser,

    overlays: Vec<ConditionalOverlayRow>,

    cursors: Vec<ConditionalCursorMove>,

    local_frame_sent: u64,
    local_frame_acked: u64,
    local_frame_late_acked: u64,

    prediction_epoch: u64,
    confirmed_epoch: u64,

    /// whether we are underlining predictions
    flagging: bool,
    /// show predictions because of slow round trip time
    srtt_trigger: bool,
    /// show predictions temporarily because of long-pending prediction
    glitch_trigger: u32,
    last_quick_confirmation: u64,

    send_interval: u32,

    last_height: i32,
    last_width: i32,

    display_preference: DisplayPreference,
    predict_overwrite: bool,
}

impl Default for PredictionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PredictionEngine {
    /// <= ms cures SRTT trigger to show predictions
    const SRTT_TRIGGER_LOW: u64 = 20;
    /// > ms starts SRTT trigger
    const SRTT_TRIGGER_HIGH: u64 = 30;

    /// <= ms cures flagging
    const FLAG_TRIGGER_LOW: u64 = 50;
    /// > ms starts flagging
    const FLAG_TRIGGER_HIGH: u64 = 80;

    /// prediction outstanding this long is glitch
    const GLITCH_THRESHOLD: u64 = 250;
    /// non-glitches required to cure glitch trigger
    const GLITCH_REPAIR_COUNT: u64 = 10;
    /// required time in between non-glitches
    const GLITCH_REPAIR_MININTERVAL: u64 = 150;

    /// prediction outstanding this long => underline
    const GLITCH_FLAG_THRESHOLD: u64 = 5000;

    fn cursor(&mut self) -> &mut ConditionalCursorMove {
        self.cursors.last_mut().expect("cursors not empty")
    }

    fn timing_tests_necessary(&self) -> bool {
        /* Are there any timing-based triggers that haven't fired yet? */
        !(self.glitch_trigger != 0 && self.flagging)
    }

    /// Set the display preference.
    pub fn set_display_preference(&mut self, s_pref: DisplayPreference) {
        self.display_preference = s_pref;
    }

    /// Predict overwriting instead of inserting.
    pub fn set_predict_overwrite(&mut self, overwrite: bool) {
        self.predict_overwrite = overwrite;
    }

    /// Number of the newest state sent to the server.
    pub fn set_local_frame_sent(&mut self, x: u64) {
        self.local_frame_sent = x;
    }

    /// Number of the newest state the server acknowledged.
    pub fn set_local_frame_acked(&mut self, x: u64) {
        self.local_frame_acked = x;
    }

    /// Number of the newest state the server's screen reflects (echo ack).
    pub fn set_local_frame_late_acked(&mut self, x: u64) {
        self.local_frame_late_acked = x;
    }

    /// The transport's current send interval in ms.
    pub fn set_send_interval(&mut self, x: u32) {
        self.send_interval = x;
    }

    /// Milliseconds until the timing triggers need checking again.
    pub fn wait_time(&self) -> i32 {
        if self.timing_tests_necessary() && self.active() {
            50
        } else {
            INT_MAX
        }
    }

    /// Adaptive mode, nothing predicted.
    pub fn new() -> PredictionEngine {
        PredictionEngine {
            last_byte: 0,
            parser: UTF8Parser::new(),
            overlays: Vec::new(),
            cursors: Vec::new(),
            local_frame_sent: 0,
            local_frame_acked: 0,
            local_frame_late_acked: 0,
            prediction_epoch: 1,
            confirmed_epoch: 0,
            flagging: false,
            srtt_trigger: false,
            glitch_trigger: 0,
            last_quick_confirmation: 0,
            send_interval: 250,
            last_height: 0,
            last_width: 0,
            display_preference: DisplayPreference::Adaptive,
            predict_overwrite: false,
        }
    }
}

/// The window title prefix (`[mosh] `).
#[derive(Clone, Debug, Default)]
pub struct TitleEngine {
    prefix: title_type,
}

impl TitleEngine {
    /// Prefix `fb`'s window title.
    pub fn apply(&self, fb: &mut Framebuffer) {
        fb.prefix_window_title(&self.prefix);
    }

    /// No prefix.
    pub fn new() -> TitleEngine {
        TitleEngine { prefix: Vec::new() }
    }
}

/// All overlays, applied in order to each frame before it is drawn.
#[derive(Clone, Debug, Default)]
pub struct OverlayManager {
    notifications: NotificationEngine,
    predictions: PredictionEngine,
    title: TitleEngine,
}

impl OverlayManager {
    /// The notification bar.
    pub fn get_notification_engine(&mut self) -> &mut NotificationEngine {
        &mut self.notifications
    }

    /// The prediction engine.
    pub fn get_prediction_engine(&mut self) -> &mut PredictionEngine {
        &mut self.predictions
    }

    /// Set the window title prefix.
    pub fn set_title_prefix(&mut self, s: &str) {
        self.title.set_prefix(s);
    }

    /// No overlays.
    pub fn new() -> OverlayManager {
        OverlayManager {
            notifications: NotificationEngine::new(),
            predictions: PredictionEngine::new(),
            title: TitleEngine::new(),
        }
    }

    /// Milliseconds until an overlay needs redrawing.
    pub fn wait_time(&self) -> i32 {
        self.notifications
            .wait_time()
            .min(self.predictions.wait_time())
    }
}

impl OverlayManager {
    /// Apply every overlay to `fb`.
    pub fn apply(&mut self, fb: &mut Framebuffer) {
        self.predictions.cull(fb);
        self.predictions.apply(fb);
        self.notifications.adjust_message();
        self.notifications.apply(fb);
        self.title.apply(fb);
    }
}

impl TitleEngine {
    /// Set the prefix.
    pub fn set_prefix(&mut self, s: &str) {
        self.prefix = s.chars().map(|c| c as u32).collect();
    }
}

impl ConditionalOverlayRow {
    fn new(s_row_num: i32) -> ConditionalOverlayRow {
        ConditionalOverlayRow {
            row_num: s_row_num,
            overlay_cells: Vec::new(),
        }
    }

    /// Draw the row's predictions.
    pub fn apply(&self, fb: &mut Framebuffer, confirmed_epoch: u64, flag: bool) {
        for it in self.overlay_cells.iter() {
            it.apply(fb, confirmed_epoch, self.row_num, flag);
        }
    }
}

impl PredictionEngine {
    /// Draw the predictions into `fb`, if they are to be shown.
    pub fn apply(&self, fb: &mut Framebuffer) {
        if (self.display_preference == DisplayPreference::Never)
            || !(self.srtt_trigger
                || self.glitch_trigger != 0
                || (self.display_preference == DisplayPreference::Always)
                || (self.display_preference == DisplayPreference::Experimental))
        {
            return;
        }

        for it in self.cursors.iter() {
            it.apply(fb, self.confirmed_epoch);
        }

        for it in self.overlays.iter() {
            it.apply(fb, self.confirmed_epoch, self.flagging);
        }
    }

    fn kill_epoch(&mut self, epoch: u64, fb: &Framebuffer) {
        self.cursors
            .retain(|it| !it.base.tentative(epoch.wrapping_sub(1)));

        self.cursors.push(ConditionalCursorMove::new(
            self.local_frame_sent + 1,
            fb.ds.get_cursor_row(),
            fb.ds.get_cursor_col(),
            self.prediction_epoch,
        ));
        self.cursor().base.active = true;

        for i in self.overlays.iter_mut() {
            for j in i.overlay_cells.iter_mut() {
                if j.base.tentative(epoch.wrapping_sub(1)) {
                    j.reset();
                }
            }
        }

        self.become_tentative();
    }

    /// Forget every prediction.
    pub fn reset(&mut self) {
        self.cursors.clear();
        self.overlays.clear();
        self.become_tentative();
    }

    fn init_cursor(&mut self, fb: &Framebuffer) {
        if self.cursors.is_empty() {
            /* initialize new cursor prediction */

            self.cursors.push(ConditionalCursorMove::new(
                self.local_frame_sent + 1,
                fb.ds.get_cursor_row(),
                fb.ds.get_cursor_col(),
                self.prediction_epoch,
            ));

            self.cursor().base.active = true;
        } else if self.cursor().base.tentative_until_epoch != self.prediction_epoch {
            let (row, col) = (self.cursor().row, self.cursor().base.col);
            self.cursors.push(ConditionalCursorMove::new(
                self.local_frame_sent + 1,
                row,
                col,
                self.prediction_epoch,
            ));

            self.cursor().base.active = true;
        }
    }

    /// Check the predictions against the server's framebuffer `fb`:
    /// confirm or kill them, and update the timing triggers.
    pub fn cull(&mut self, fb: &Framebuffer) {
        if self.display_preference == DisplayPreference::Never {
            return;
        }

        if (self.last_height != fb.ds.get_height()) || (self.last_width != fb.ds.get_width()) {
            self.last_height = fb.ds.get_height();
            self.last_width = fb.ds.get_width();
            self.reset();
        }

        let now = timestamp();
        let send_interval = u64::from(self.send_interval);

        /* control srtt_trigger with hysteresis */
        if send_interval > Self::SRTT_TRIGGER_HIGH {
            self.srtt_trigger = true;
        } else if self.srtt_trigger
            && (send_interval <= Self::SRTT_TRIGGER_LOW) /* 20 ms is current minimum value */
            && (!self.active())
        {
            /* only turn off when no predictions being shown */
            self.srtt_trigger = false;
        }

        /* control underlining with hysteresis */
        if send_interval > Self::FLAG_TRIGGER_HIGH {
            self.flagging = true;
        } else if send_interval <= Self::FLAG_TRIGGER_LOW {
            self.flagging = false;
        }

        /* really big glitches also activate underlining */
        if u64::from(self.glitch_trigger) > Self::GLITCH_REPAIR_COUNT {
            self.flagging = true;
        }

        /* go through cell predictions */

        let mut i = 0;
        while i < self.overlays.len() {
            let row_num = self.overlays[i].row_num;
            if (row_num < 0) || (row_num >= fb.ds.get_height()) {
                self.overlays.remove(i);
                continue;
            }

            for j in 0..self.overlays[i].overlay_cells.len() {
                let validity = self.overlays[i].overlay_cells[j].get_validity(
                    fb,
                    row_num,
                    self.local_frame_acked,
                    self.local_frame_late_acked,
                );
                match validity {
                    Validity::IncorrectOrExpired => {
                        let cell = &mut self.overlays[i].overlay_cells[j];
                        if cell.base.tentative(self.confirmed_epoch) {
                            if self.display_preference == DisplayPreference::Experimental {
                                cell.reset();
                            } else {
                                let epoch = cell.base.tentative_until_epoch;
                                self.kill_epoch(epoch, fb);
                            }
                        } else if self.display_preference == DisplayPreference::Experimental {
                            cell.reset();
                        } else {
                            self.reset();
                            return;
                        }
                    }
                    Validity::Correct | Validity::CorrectNoCredit => {
                        if validity == Validity::Correct {
                            let cell = &self.overlays[i].overlay_cells[j];
                            if cell.base.tentative_until_epoch > self.confirmed_epoch {
                                self.confirmed_epoch = cell.base.tentative_until_epoch;
                            }

                            /* When predictions come in quickly, slowly take away the glitch trigger. */
                            if now.wrapping_sub(cell.base.prediction_time) < Self::GLITCH_THRESHOLD
                                && (self.glitch_trigger > 0
                                    && now.wrapping_sub(Self::GLITCH_REPAIR_MININTERVAL)
                                        >= self.last_quick_confirmation)
                            {
                                self.glitch_trigger -= 1;
                                self.last_quick_confirmation = now;
                            }

                            /* match rest of row to the actual renditions */
                            let actual_renditions =
                                *fb.get_cell(row_num, cell.base.col).get_renditions();
                            for k in self.overlays[i].overlay_cells[j..].iter_mut() {
                                *k.replacement.get_renditions_mut() = actual_renditions;
                            }
                        }

                        /* Correct falls through to CorrectNoCredit */
                        self.overlays[i].overlay_cells[j].reset();
                    }
                    Validity::Pending => {
                        /* When a prediction takes a long time to be confirmed, we
                        activate the predictions even if SRTT is low */
                        let pending_for = now
                            .wrapping_sub(self.overlays[i].overlay_cells[j].base.prediction_time);
                        if pending_for >= Self::GLITCH_FLAG_THRESHOLD {
                            /* display and underline */
                            self.glitch_trigger = (Self::GLITCH_REPAIR_COUNT * 2) as u32;
                        } else if (pending_for >= Self::GLITCH_THRESHOLD)
                            && (u64::from(self.glitch_trigger) < Self::GLITCH_REPAIR_COUNT)
                        {
                            /* just display */
                            self.glitch_trigger = Self::GLITCH_REPAIR_COUNT as u32;
                        }
                    }
                    Validity::Inactive => {}
                }
            }

            i += 1;
        }

        /* go through cursor predictions */
        let (early_ack, late_ack) = (self.local_frame_acked, self.local_frame_late_acked);
        if !self.cursors.is_empty()
            && self.cursor().get_validity(fb, early_ack, late_ack) == Validity::IncorrectOrExpired
        {
            if self.display_preference == DisplayPreference::Experimental {
                self.cursors.clear();
            } else {
                self.reset();
                return;
            }
        }

        self.cursors
            .retain(|it| it.get_validity(fb, early_ack, late_ack) == Validity::Pending);
    }

    /// Index in `overlays` of row `row_num`, made with `num_cols` cells if missing.
    fn get_or_make_row(&mut self, row_num: i32, num_cols: i32) -> usize {
        if let Some(it) = self.overlays.iter().position(|it| it.row_num == row_num) {
            return it;
        }
        /* make row */
        let mut r = ConditionalOverlayRow::new(row_num);
        r.overlay_cells.reserve(num_cols.max(0) as usize);
        for i in 0..num_cols {
            r.overlay_cells
                .push(ConditionalOverlayCell::new(0, i, self.prediction_epoch));
            assert!(r.overlay_cells[i as usize].base.col == i);
        }
        self.overlays.push(r);
        self.overlays.len() - 1
    }

    /// Predict the effect on `fb` of one byte typed by the user.
    pub fn new_user_byte(&mut self, mut the_byte: u8, fb: &Framebuffer) {
        if self.display_preference == DisplayPreference::Never {
            return;
        }
        if self.display_preference == DisplayPreference::Experimental {
            self.prediction_epoch = self.confirmed_epoch;
        }

        self.cull(fb);

        let now = timestamp();

        /* translate application-mode cursor control function to ANSI cursor control sequence */
        if (self.last_byte == 0x1b) && (the_byte == b'O') {
            the_byte = b'[';
        }
        self.last_byte = the_byte;

        let mut actions = Actions::new();
        self.parser.input(the_byte, &mut actions);

        let width = fb.ds.get_width();

        for act in actions.iter() {
            match act.kind {
                ActionType::Print => {
                    /* make new prediction */

                    self.init_cursor(fb);

                    assert!(act.char_present);

                    let ch = act.ch;
                    /* XXX handle wide characters */

                    if ch == 0x7f {
                        /* backspace */
                        let crow = self.cursor().row;
                        let the_row = self.get_or_make_row(crow, width);

                        if self.cursor().base.col > 0 {
                            let sent = self.local_frame_sent + 1;
                            self.cursor().base.col -= 1;
                            self.cursor().base.expire(sent, now);
                            let ccol = self.cursor().base.col;
                            let prediction_epoch = self.prediction_epoch;

                            if self.predict_overwrite {
                                let cell = &mut self.overlays[the_row].overlay_cells[ccol as usize];
                                cell.reset_with_orig();
                                cell.base.active = true;
                                cell.base.tentative_until_epoch = prediction_epoch;
                                cell.base.expire(sent, now);
                                let orig_cell = fb.get_cell(-1, -1).clone();
                                cell.original_contents.push(orig_cell.clone());
                                cell.replacement = orig_cell;
                                cell.replacement.clear();
                                cell.replacement.append(' ' as u32);
                            } else {
                                for i in ccol..width {
                                    let cells = &mut self.overlays[the_row].overlay_cells;
                                    // next_cell is read before cell is written; they differ.
                                    let next = (i + 2 < width).then(|| {
                                        let next_cell = &cells[(i + 1) as usize];
                                        (
                                            next_cell.base.active,
                                            next_cell.unknown,
                                            next_cell.replacement.clone(),
                                        )
                                    });
                                    let cell = &mut cells[i as usize];

                                    cell.reset_with_orig();
                                    cell.base.active = true;
                                    cell.base.tentative_until_epoch = prediction_epoch;
                                    cell.base.expire(sent, now);
                                    cell.original_contents.push(fb.get_cell(crow, i).clone());

                                    if let Some((next_active, next_unknown, next_replacement)) =
                                        next
                                    {
                                        if next_active {
                                            if next_unknown {
                                                cell.unknown = true;
                                            } else {
                                                cell.unknown = false;
                                                cell.replacement = next_replacement;
                                            }
                                        } else {
                                            cell.unknown = false;
                                            cell.replacement = fb.get_cell(crow, i + 1).clone();
                                        }
                                    } else {
                                        cell.unknown = true;
                                    }
                                }
                            }
                        }
                    } else if (ch < 0x20) || (wcwidth(ch) != 1) {
                        /* unknown print */
                        self.become_tentative();
                    } else {
                        let (crow, ccol) = (self.cursor().row, self.cursor().base.col);
                        assert!(crow >= 0);
                        assert!(ccol >= 0);
                        assert!(crow < fb.ds.get_height());
                        assert!(ccol < width);

                        let the_row = self.get_or_make_row(crow, width);

                        if ccol + 1 >= width {
                            /* prediction in the last column is tricky */
                            /* e.g., emacs will show wrap character, shell will just put the character there */
                            self.become_tentative();
                        }

                        let sent = self.local_frame_sent + 1;
                        let prediction_epoch = self.prediction_epoch;
                        let cells = &mut self.overlays[the_row].overlay_cells;

                        /* do the insert */
                        let rightmost_column = if self.predict_overwrite {
                            ccol
                        } else {
                            width - 1
                        };
                        for i in (ccol + 1..=rightmost_column).rev() {
                            // prev_cell is read before cell is written; they differ.
                            let prev_cell = &cells[(i - 1) as usize];
                            let (prev_active, prev_unknown, prev_replacement) = (
                                prev_cell.base.active,
                                prev_cell.unknown,
                                prev_cell.replacement.clone(),
                            );
                            let cell = &mut cells[i as usize];
                            cell.reset_with_orig();
                            cell.base.active = true;
                            cell.base.tentative_until_epoch = prediction_epoch;
                            cell.base.expire(sent, now);
                            cell.original_contents.push(fb.get_cell(crow, i).clone());

                            if i == width - 1 {
                                cell.unknown = true;
                            } else if prev_active {
                                if prev_unknown {
                                    cell.unknown = true;
                                } else {
                                    cell.unknown = false;
                                    cell.replacement = prev_replacement;
                                }
                            } else {
                                cell.unknown = false;
                                cell.replacement = fb.get_cell(crow, i - 1).clone();
                            }
                        }

                        /* heuristic: match renditions of character to the left */
                        let left_renditions = (ccol > 0).then(|| {
                            let prev_cell = &cells[(ccol - 1) as usize];
                            if prev_cell.base.active && (!prev_cell.unknown) {
                                *prev_cell.replacement.get_renditions()
                            } else {
                                *fb.get_cell(crow, ccol - 1).get_renditions()
                            }
                        });

                        let cell = &mut cells[ccol as usize];
                        cell.reset_with_orig();
                        cell.base.active = true;
                        cell.base.tentative_until_epoch = prediction_epoch;
                        cell.base.expire(sent, now);
                        *cell.replacement.get_renditions_mut() = *fb.ds.get_renditions();

                        if let Some(r) = left_renditions {
                            *cell.replacement.get_renditions_mut() = r;
                        }

                        cell.replacement.clear();
                        cell.replacement.append(ch);
                        cell.original_contents.push(fb.get_cell(crow, ccol).clone());

                        self.cursor().base.expire(sent, now);

                        /* do we need to wrap? */
                        if ccol < width - 1 {
                            self.cursor().base.col += 1;
                        } else {
                            self.become_tentative();
                            self.newline_carriage_return(fb);
                        }
                    }
                }
                ActionType::Execute => {
                    if act.char_present && (act.ch == 0x0d) {
                        /* CR */
                        self.become_tentative();
                        self.newline_carriage_return(fb);
                    } else {
                        self.become_tentative();
                    }
                }
                ActionType::Esc_Dispatch => {
                    self.become_tentative();
                }
                ActionType::CSI_Dispatch => {
                    let sent = self.local_frame_sent + 1;
                    if act.char_present && (act.ch == 'C' as u32) {
                        /* right arrow */
                        self.init_cursor(fb);
                        if self.cursor().base.col < width - 1 {
                            self.cursor().base.col += 1;
                            self.cursor().base.expire(sent, now);
                        }
                    } else if act.char_present && (act.ch == 'D' as u32) {
                        /* left arrow */
                        self.init_cursor(fb);

                        if self.cursor().base.col > 0 {
                            self.cursor().base.col -= 1;
                            self.cursor().base.expire(sent, now);
                        }
                    } else {
                        self.become_tentative();
                    }
                }
                _ => {}
            }
        }
    }

    fn newline_carriage_return(&mut self, fb: &Framebuffer) {
        let now = timestamp();
        self.init_cursor(fb);
        self.cursor().base.col = 0;
        if self.cursor().row == fb.ds.get_height() - 1 {
            /* Don't try to predict scroll until we have versioned cell predictions */

            /* make blank prediction for last row */
            let crow = self.cursor().row;
            let the_row = self.get_or_make_row(crow, fb.ds.get_width());
            let (prediction_epoch, sent) = (self.prediction_epoch, self.local_frame_sent + 1);
            for j in self.overlays[the_row].overlay_cells.iter_mut() {
                j.base.active = true;
                j.base.tentative_until_epoch = prediction_epoch;
                j.base.expire(sent, now);
                j.replacement.clear();
            }
        } else {
            self.cursor().row += 1;
        }
    }

    fn become_tentative(&mut self) {
        if self.display_preference != DisplayPreference::Experimental {
            self.prediction_epoch += 1;
        }
    }

    fn active(&self) -> bool {
        if !self.cursors.is_empty() {
            return true;
        }

        self.overlays
            .iter()
            .any(|i| i.overlay_cells.iter().any(|j| j.base.active))
    }
}
