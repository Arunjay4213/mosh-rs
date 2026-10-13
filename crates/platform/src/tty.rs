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
// Ported from: upstream/src/frontend/stmclient.cc (init, shutdown, resume:
// tcgetattr/IUTF8/cfmakeraw/tcsetattr; TIOCGWINSZ in main_init and
// process_resize), upstream/src/util/swrite.cc (swrite).
// The Windows half is new (no upstream equivalent), following PLAN section 7.

//! The local terminal, ported from upstream `src/frontend/stmclient.cc` and
//! `src/util/swrite.cc`: raw mode on and off, window size, stdin reads and
//! complete stdout writes. Windows uses console modes (PLAN section 7).

/// `kill( 0, SIGSTOP )`: stop this process group, for the client's
/// Ctrl-^ Ctrl-Z suspend (STMClient::process_user_input). Returns after
/// SIGCONT. Windows has no job control, so this returns at once there.
pub fn suspend_self() {
    #[cfg(unix)]
    // SAFETY: plain kill(2) call on our own process group.
    unsafe {
        libc::kill(0, libc::SIGSTOP);
    }
}

/// On Windows, sets `TERM=xterm-256color` for this process (and so for
/// the `ssh` and `mosh-client` it starts) when `TERM` is unset or empty
/// (PLAN.md 9.12). Windows normally has no `TERM`, and without one the
/// client's color count fails ("TERM environment variable not set.") and
/// the launcher stops with "Can't count colors"; Windows Terminal and
/// conhost with VT processing implement xterm-256color. On unix this does
/// nothing: an unset `TERM` keeps upstream's behavior.
///
/// Call it first thing in `main`, before any thread starts: setting the
/// environment is only sound while no other thread can read it (which is
/// why `set_var` is `unsafe` in edition 2024; this crate is edition 2021).
pub fn default_term_if_unset() {
    #[cfg(windows)]
    {
        let unset = match std::env::var_os("TERM") {
            None => true,
            Some(term) => term.is_empty(),
        };
        if unset {
            std::env::set_var("TERM", "xterm-256color");
        }
    }
}

#[cfg(unix)]
pub use self::unix::*;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(unix)]
mod unix {
    use std::io;
    use std::mem::MaybeUninit;

    /// What `Select` and `read_stdin` use for the local terminal input.
    pub type Fd = std::os::fd::RawFd;

    /// The fd to watch in `Select` for terminal input.
    pub fn stdin_fd() -> Fd {
        libc::STDIN_FILENO
    }

    /// Holds STMClient's `saved_termios` and `raw_termios`. Dropping it
    /// restores `saved_termios`, so a panic or early return cannot leave the
    /// user's terminal raw.
    pub struct RawModeGuard {
        saved_termios: libc::termios,
        raw_termios: libc::termios,
        restored: bool,
    }

    fn tcsetattr(termios: &libc::termios) -> io::Result<()> {
        // SAFETY: termios is a fully initialized struct from tcgetattr.
        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, termios) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// STMClient::init: save the terminal driver state, then put it in raw
    /// mode (with IUTF8 set where the platform has it).
    pub fn raw_mode_on() -> io::Result<RawModeGuard> {
        /* Verify terminal configuration */
        let mut saved = MaybeUninit::<libc::termios>::uninit();
        // SAFETY: tcgetattr fills the struct on success.
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, saved.as_mut_ptr()) } < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: initialized by the successful tcgetattr above.
        let saved_termios = unsafe { saved.assume_init() };

        /* Put terminal driver in raw mode */
        let mut raw_termios = saved_termios;
        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "macos",
            target_os = "cygwin"
        ))]
        if raw_termios.c_iflag & libc::IUTF8 == 0 {
            /* Probably not really necessary since we are putting terminal driver into raw mode anyway. */
            raw_termios.c_iflag |= libc::IUTF8;
        }
        // SAFETY: raw_termios is initialized.
        unsafe { libc::cfmakeraw(&mut raw_termios) };
        tcsetattr(&raw_termios)?;
        Ok(RawModeGuard {
            saved_termios,
            raw_termios,
            restored: false,
        })
    }

    impl RawModeGuard {
        /// Restore `saved_termios` without giving up the guard (the
        /// Ctrl-^ Ctrl-Z suspend path in STMClient::process_user_input).
        pub fn restore_saved(&self) -> io::Result<()> {
            tcsetattr(&self.saved_termios)
        }

        /// Re-apply `raw_termios` (STMClient::resume after SIGCONT).
        pub fn apply_raw(&self) -> io::Result<()> {
            tcsetattr(&self.raw_termios)
        }
    }

    /// STMClient::shutdown: restore `saved_termios`.
    pub fn raw_mode_off(mut guard: RawModeGuard) -> io::Result<()> {
        guard.restored = true;
        tcsetattr(&guard.saved_termios)
    }

    impl Drop for RawModeGuard {
        fn drop(&mut self) {
            if !self.restored {
                let _ = tcsetattr(&self.saved_termios);
            }
        }
    }

    /// `ioctl( STDIN_FILENO, TIOCGWINSZ, &window_size )`, as (cols, rows).
    pub fn window_size() -> io::Result<(u16, u16)> {
        let mut ws = MaybeUninit::<libc::winsize>::zeroed();
        // SAFETY: TIOCGWINSZ writes one winsize struct.
        if unsafe { libc::ioctl(libc::STDIN_FILENO, libc::TIOCGWINSZ, ws.as_mut_ptr()) } < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: zero-initialized and filled by the ioctl.
        let ws = unsafe { ws.assume_init() };
        Ok((ws.ws_col, ws.ws_row))
    }

    /// One `read( STDIN_FILENO, buf, size )`, as STMClient::process_user_input.
    /// Ok(0) is end of file.
    pub fn read_stdin(buf: &mut [u8]) -> io::Result<usize> {
        // SAFETY: buf is valid for buf.len() bytes.
        let n = unsafe { libc::read(libc::STDIN_FILENO, buf.as_mut_ptr().cast(), buf.len()) };
        if n < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(n as usize)
    }

    /// swrite( STDOUT_FILENO, ... ): write everything or fail. Like upstream,
    /// a short or failed write is not retried on EINTR.
    pub fn write_all_stdout(bytes: &[u8]) -> io::Result<()> {
        let mut total_bytes_written = 0;
        while total_bytes_written < bytes.len() {
            let rest = &bytes[total_bytes_written..];
            // SAFETY: rest is valid for rest.len() bytes.
            let bytes_written =
                unsafe { libc::write(libc::STDOUT_FILENO, rest.as_ptr().cast(), rest.len()) };
            if bytes_written <= 0 {
                return Err(if bytes_written == 0 {
                    io::Error::from(io::ErrorKind::WriteZero)
                } else {
                    io::Error::last_os_error()
                });
            }
            total_bytes_written += bytes_written as usize;
        }
        Ok(())
    }
}

#[cfg(windows)]
mod windows {
    use std::io::{self, Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::os::windows::io::AsRawSocket;
    use std::sync::OnceLock;

    use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{ReadFile, WriteFile};
    use windows_sys::Win32::System::Console::{
        GetConsoleCP, GetConsoleMode, GetConsoleOutputCP, GetConsoleScreenBufferInfo, GetStdHandle,
        ReadConsoleW, SetConsoleCP, SetConsoleMode, SetConsoleOutputCP, CONSOLE_MODE,
        CONSOLE_SCREEN_BUFFER_INFO, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT,
        ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_PROCESSED_OUTPUT,
        ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_INPUT_HANDLE,
        STD_OUTPUT_HANDLE,
    };

    const CP_UTF8: u32 = 65001;

    /// A socket: on Windows `Select` polls sockets only, so console input is
    /// read by a thread and forwarded over a loopback TCP connection whose
    /// receiving end is this socket.
    pub type Fd = std::os::windows::io::RawSocket;

    fn std_handle(which: u32) -> io::Result<HANDLE> {
        // SAFETY: GetStdHandle has no preconditions.
        let h = unsafe { GetStdHandle(which) };
        if h.is_null() || h == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        Ok(h)
    }

    fn console_mode(h: HANDLE) -> io::Result<CONSOLE_MODE> {
        let mut mode: CONSOLE_MODE = 0;
        // SAFETY: mode is a valid out pointer.
        if unsafe { GetConsoleMode(h, &mut mode) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(mode)
    }

    fn set_console_mode(h: HANDLE, mode: CONSOLE_MODE) -> io::Result<()> {
        // SAFETY: plain call on a console handle.
        if unsafe { SetConsoleMode(h, mode) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    fn set_code_pages(input_cp: u32, output_cp: u32) -> io::Result<()> {
        // SAFETY: plain calls.
        if unsafe { SetConsoleCP(input_cp) } == 0 || unsafe { SetConsoleOutputCP(output_cp) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Saved console modes and code pages; the Windows counterpart of
    /// `saved_termios`. Dropping it restores them.
    pub struct RawModeGuard {
        input: HANDLE,
        output: HANDLE,
        saved_input_mode: CONSOLE_MODE,
        saved_output_mode: CONSOLE_MODE,
        saved_input_cp: u32,
        saved_output_cp: u32,
        restored: bool,
    }

    fn raw_input_mode(saved: CONSOLE_MODE) -> CONSOLE_MODE {
        (saved | ENABLE_VIRTUAL_TERMINAL_INPUT)
            & !(ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT)
    }

    fn raw_output_mode(saved: CONSOLE_MODE) -> CONSOLE_MODE {
        /* VT processing needs ENABLE_PROCESSED_OUTPUT (SetConsoleMode documentation);
        it is on by default, but a parent program may have cleared it. */
        saved
            | ENABLE_PROCESSED_OUTPUT
            | ENABLE_VIRTUAL_TERMINAL_PROCESSING
            | DISABLE_NEWLINE_AUTO_RETURN
    }

    /// PLAN section 7: VT processing on output, VT input with processed,
    /// line and echo input off, and UTF-8 code pages. After this, keys
    /// arrive as the same bytes a unix tty in raw mode produces.
    pub fn raw_mode_on() -> io::Result<RawModeGuard> {
        let input = std_handle(STD_INPUT_HANDLE)?;
        let output = std_handle(STD_OUTPUT_HANDLE)?;
        let saved_input_mode = console_mode(input)?;
        let saved_output_mode = console_mode(output)?;
        // SAFETY: plain calls; both return 0 on failure.
        let (saved_input_cp, saved_output_cp) = unsafe { (GetConsoleCP(), GetConsoleOutputCP()) };
        if saved_input_cp == 0 || saved_output_cp == 0 {
            return Err(io::Error::last_os_error());
        }
        let guard = RawModeGuard {
            input,
            output,
            saved_input_mode,
            saved_output_mode,
            saved_input_cp,
            saved_output_cp,
            restored: false,
        };
        guard.apply_raw()?;
        /* The console input thread starts here, after line and echo input
        are off (a ReadConsoleW started in line mode would wait for Enter).
        If it cannot start, the error is returned like a failed tcgetattr
        and dropping the guard restores the console. */
        if STDIN.get().is_none() {
            let receiver = start_stdin_thread()?;
            let _ = STDIN.set(receiver);
        }
        Ok(guard)
    }

    impl RawModeGuard {
        /// Restore the saved console modes and code pages.
        pub fn restore_saved(&self) -> io::Result<()> {
            set_console_mode(self.input, self.saved_input_mode)?;
            set_console_mode(self.output, self.saved_output_mode)?;
            set_code_pages(self.saved_input_cp, self.saved_output_cp)
        }

        /// Apply the raw console modes and UTF-8 code pages again.
        pub fn apply_raw(&self) -> io::Result<()> {
            set_console_mode(self.input, raw_input_mode(self.saved_input_mode))?;
            set_console_mode(self.output, raw_output_mode(self.saved_output_mode))?;
            set_code_pages(CP_UTF8, CP_UTF8)
        }
    }

    /// Restore the saved console state.
    pub fn raw_mode_off(mut guard: RawModeGuard) -> io::Result<()> {
        guard.restored = true;
        guard.restore_saved()
    }

    impl Drop for RawModeGuard {
        fn drop(&mut self) {
            if !self.restored {
                let _ = self.restore_saved();
            }
        }
    }

    /// Visible window of the console screen buffer, as (cols, rows).
    pub fn window_size() -> io::Result<(u16, u16)> {
        let output = std_handle(STD_OUTPUT_HANDLE)?;
        // SAFETY: zeroed is a valid CONSOLE_SCREEN_BUFFER_INFO; filled on success.
        let mut info: CONSOLE_SCREEN_BUFFER_INFO = unsafe { std::mem::zeroed() };
        if unsafe { GetConsoleScreenBufferInfo(output, &mut info) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let cols = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
        let rows = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;
        Ok((cols, rows))
    }

    /// Receiving end of the stdin forwarding connection, set by `raw_mode_on`.
    static STDIN: OnceLock<TcpStream> = OnceLock::new();

    fn start_stdin_thread() -> io::Result<TcpStream> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let mut sender = TcpStream::connect(listener.local_addr()?)?;
        let receiver = loop {
            let (stream, peer) = listener.accept()?;
            // Ignore anything else on the machine that raced to connect.
            if peer == sender.local_addr()? {
                break stream;
            }
        };
        sender.set_nodelay(true)?;
        let input = std_handle(STD_INPUT_HANDLE)? as usize;
        std::thread::spawn(move || {
            let input = input as HANDLE;
            let mut buf = [0u8; 16384];
            loop {
                // Dropping `sender` gives the reader end of file.
                match read_console_bytes(input, &mut buf) {
                    Ok(0) => break,
                    Err(e) => {
                        /* As the unix client reports a failed read( STDIN_FILENO ). */
                        crate::stdio::perror("read", &e);
                        break;
                    }
                    Ok(n) => {
                        if sender.write_all(&buf[..n]).is_err() {
                            break;
                        }
                    }
                }
            }
        });
        Ok(receiver)
    }

    /// One blocking read of console input as UTF-8 bytes. A console is read
    /// with ReadConsoleW (UTF-16, converted here); anything else (a pipe or
    /// file) with ReadFile.
    fn read_console_bytes(input: HANDLE, out: &mut [u8; 16384]) -> io::Result<usize> {
        if console_mode(input).is_err() {
            let mut n: u32 = 0;
            // SAFETY: out is valid for out.len() bytes.
            if unsafe {
                ReadFile(
                    input,
                    out.as_mut_ptr(),
                    out.len() as u32,
                    &mut n,
                    std::ptr::null_mut(),
                )
            } == 0
            {
                return Err(io::Error::last_os_error());
            }
            return Ok(n as usize);
        }
        // At most 4096 UTF-16 units become at most 3 bytes each (a surrogate
        // pair is 2 units and 4 bytes), so 12288 bytes fit in `out`.
        let mut wide = [0u16; 4096];
        let mut len = 0usize;
        loop {
            let mut n: u32 = 0;
            // SAFETY: wide[len..] is valid for the requested number of units.
            if unsafe {
                ReadConsoleW(
                    input,
                    wide[len..].as_mut_ptr().cast(),
                    (wide.len() - len) as u32,
                    &mut n,
                    std::ptr::null(),
                )
            } == 0
            {
                return Err(io::Error::last_os_error());
            }
            len += n as usize;
            // Do not split a surrogate pair across two reads.
            if len > 0 && (0xD800..0xDC00).contains(&wide[len - 1]) && len < wide.len() {
                continue;
            }
            break;
        }
        let mut written = 0;
        for c in char::decode_utf16(wide[..len].iter().copied()) {
            let c = c.unwrap_or(char::REPLACEMENT_CHARACTER);
            written += c.encode_utf8(&mut out[written..]).len();
        }
        Ok(written)
    }

    /// The socket to watch in `Select` for console input. Before a
    /// successful `raw_mode_on` there is none and this is INVALID_SOCKET, so
    /// a `select` on it fails with an error rather than waiting forever.
    pub fn stdin_fd() -> Fd {
        STDIN.get().map_or(
            windows_sys::Win32::Networking::WinSock::INVALID_SOCKET as Fd,
            |s| s.as_raw_socket(),
        )
    }

    /// Read bytes typed at the console. Ok(0) is end of file. Before a
    /// successful `raw_mode_on` there is no console input to read.
    pub fn read_stdin(buf: &mut [u8]) -> io::Result<usize> {
        match STDIN.get() {
            Some(mut socket) => socket.read(buf),
            None => Err(io::Error::from(io::ErrorKind::NotConnected)),
        }
    }

    /// swrite( STDOUT_FILENO, ... ) with WriteFile, so bytes reach the
    /// console unchanged (UTF-8 code page, VT processing on).
    pub fn write_all_stdout(bytes: &[u8]) -> io::Result<()> {
        let output = std_handle(STD_OUTPUT_HANDLE)?;
        let mut total_bytes_written = 0;
        while total_bytes_written < bytes.len() {
            let rest = &bytes[total_bytes_written..];
            let chunk = rest.len().min(u32::MAX as usize) as u32;
            let mut bytes_written: u32 = 0;
            // SAFETY: rest is valid for chunk bytes.
            if unsafe {
                WriteFile(
                    output,
                    rest.as_ptr(),
                    chunk,
                    &mut bytes_written,
                    std::ptr::null_mut(),
                )
            } == 0
            {
                return Err(io::Error::last_os_error());
            }
            if bytes_written == 0 {
                return Err(io::Error::from(io::ErrorKind::WriteZero));
            }
            total_bytes_written += bytes_written as usize;
        }
        Ok(())
    }
}
