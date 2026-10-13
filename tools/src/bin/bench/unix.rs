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
// Ported from: (new; test tooling)

//! The unix implementation of `bench` (see `main.rs` and tools/bench/README.md).
//!
//! Every session is started the way tests/interop.sh starts one:
//! `mosh-server new -p PORT -- COMMAND` prints `MOSH CONNECT PORT KEY` and
//! detaches; the client gets the key in `MOSH_KEY`. Here the client runs on
//! a pseudoterminal this program owns, so every write the client makes to
//! its terminal is read back with a timestamp.

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::UdpSocket;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{exit, Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep, JoinHandle};
use std::time::{Duration, Instant};

const PORT_FIRST: u16 = 61400;
const PORT_LAST: u16 = 61499;
const SLOT_PORTS: u16 = 4;
const LOCK_FILE: &str = "/tmp/mosh-rs-interop.lock";

const COLS: u16 = 80;
const ROWS: u16 = 24;

// Verdict thresholds; tools/bench/README.md explains each one.
const IDLE_CPU_MAX_PCT: f64 = 0.05;
const BINARY_MAX_BYTES: u64 = 4_000_000;
const STARTUP_MAX_MS: f64 = 20.0;
const KEYSTROKE_MAX_MS: f64 = 16.0;
/// seq output arrives in frames at least SEND_INTERVAL_MIN (20 ms) apart,
/// so medians differ by a frame from noise alone: mosh-rs passes if it is
/// within 5% or one frame of C++, whichever is larger.
const SEQ_TOLERANCE: f64 = 0.05;
const SEQ_FRAME_MS: f64 = 20.0;

const STARTUP_RUNS: usize = 20;
const KEYSTROKE_RUNS: usize = 30;
const SEQ_RUNS: usize = 5;
const IDLE_SECS: u64 = 60;
const RSS_SHORT_SECS: u64 = 300;
const RSS_HOUR_SECS: u64 = 3600;

/// One way delay of the keystroke proxy: 300 ms round trip.
const KEY_DELAY_MS: u64 = 150;
/// Writes within this time after a keystroke count as its render. It ends
/// well before the server's echo can arrive (one round trip, 300 ms).
const RENDER_WINDOW: Duration = Duration::from_millis(250);
/// Writes within this time count toward the "all writes" number, which
/// includes the redraw when the server confirms the prediction.
const SETTLE_WINDOW: Duration = Duration::from_millis(1200);

const METRICS: [&str; 6] = [
    "binary_size",
    "startup",
    "keystroke",
    "seq",
    "idle_cpu",
    "rss",
];

/// The remote side of the rss session (README "rss workload").
const RSS_WORKLOAD: &str = r#"n=0
while :; do
  n=$((n+1))
  printf '\033[H'
  i=1
  while [ $i -le 22 ]; do
    printf 'tick %6d line %2d %(%H:%M:%S)T %5d %5d\033[K\n' "$n" "$i" -1 "$RANDOM" "$RANDOM"
    i=$((i+1))
  done
  if [ $((n % 300)) -eq 60 ]; then seq 1 20000; fi
  if read -r -t 1 line; then printf 'typed: %s\033[K\n' "$line"; fi
done"#;

struct Imp {
    label: &'static str,
    client: PathBuf,
    server: PathBuf,
}

struct Row {
    metric: String,
    target: String,
    rust: String,
    cxx: String,
    verdict: String,
}

fn row(metric: &str, target: &str, rust: String, cxx: String, verdict: &str) -> Row {
    Row {
        metric: metric.to_string(),
        target: target.to_string(),
        rust,
        cxx,
        verdict: verdict.to_string(),
    }
}

fn pass(ok: bool) -> &'static str {
    if ok {
        "PASS"
    } else {
        "FAIL"
    }
}

fn usage() -> ! {
    eprintln!(
        "usage: bench [--only NAME] [--hour] [--json] [--rust-dir DIR]\n\
         NAME: {}",
        METRICS.join(", ")
    );
    exit(2);
}

pub fn main() {
    let mut only: Option<String> = None;
    let mut hour = false;
    let mut json = false;
    let mut rust_dir: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--only" => only = Some(args.next().unwrap_or_else(|| usage())),
            "--hour" => hour = true,
            "--json" => json = true,
            "--rust-dir" => rust_dir = Some(PathBuf::from(args.next().unwrap_or_else(|| usage()))),
            _ => usage(),
        }
    }
    if let Some(o) = &only {
        if !METRICS.contains(&o.as_str()) {
            usage();
        }
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));
    let repo = exe_dir.join("../..");
    let rust_dir_given = rust_dir.is_some();
    let rust_dir = rust_dir.unwrap_or_else(|| exe_dir.clone());
    let rust_dir = fs::canonicalize(&rust_dir).unwrap_or(rust_dir);
    let impair = exe_dir.join("udp-impair");

    let rust = Imp {
        label: "mosh-rs",
        client: rust_dir.join("mosh-client"),
        server: rust_dir.join("mosh-server"),
    };
    let cxx = Imp {
        label: "mosh 1.4.0",
        client: PathBuf::from("/usr/bin/mosh-client"),
        server: PathBuf::from("/usr/bin/mosh-server"),
    };
    for p in [
        &rust.client,
        &rust.server,
        &cxx.client,
        &cxx.server,
        &impair,
    ] {
        if !p.exists() {
            eprintln!(
                "bench: {} is missing (build with: cargo build --release -p mosh -p mosh-tools)",
                p.display()
            );
            exit(1);
        }
    }

    // Static binaries for binary_size: --rust-dir if given, else the host's
    // musl build when present, else the binaries being measured.
    let size_dir = if rust_dir_given {
        rust_dir.clone()
    } else {
        let musl = repo.join(format!(
            "target/{}-unknown-linux-musl/release",
            std::env::consts::ARCH
        ));
        if musl.join("mosh-client").exists() {
            fs::canonicalize(&musl).unwrap_or(musl)
        } else {
            rust_dir.clone()
        }
    };

    let header = machine_header(&repo, &rust_dir, &size_dir, hour);
    let lock = take_lock();
    if let Err(e) = check_ports_free() {
        eprintln!("bench: {e}");
        exit(1);
    }

    let mut ports = Ports { next: 0 };
    let mut rows = Vec::new();
    for m in METRICS {
        if only.as_deref().is_some_and(|o| o != m) {
            continue;
        }
        eprintln!("== {m}");
        let result = match m {
            "binary_size" => binary_size(&size_dir),
            "startup" => startup(&[&rust, &cxx], &mut ports),
            "keystroke" => keystroke(&[&rust, &cxx], &mut ports, &impair),
            "seq" => seq(&[&rust, &cxx], &mut ports),
            "idle_cpu" => idle_cpu(&[&rust, &cxx], &mut ports),
            "rss" => rss(&[&rust, &cxx], &mut ports, hour),
            _ => unreachable!(),
        };
        match result {
            Ok(mut r) => rows.append(&mut r),
            Err(e) => {
                eprintln!("bench: {m}: {e}");
                rows.push(row(m, "-", e, "-".into(), "ERROR"));
            }
        }
    }
    let leftover = check_ports_free();
    drop(lock);

    if json {
        print_json(&header, &rows);
    } else {
        for line in &header {
            println!("{line}");
        }
        print_table(&rows);
    }
    if let Err(e) = &leftover {
        eprintln!("bench: after the run: {e}");
    }
    let bad = rows
        .iter()
        .any(|r| r.verdict == "FAIL" || r.verdict == "ERROR");
    exit(if bad || leftover.is_err() { 1 } else { 0 });
}

// ---- Machine description, lock, ports.

fn command_line(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .stderr(Stdio::null())
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn machine_header(repo: &Path, rust_dir: &Path, size_dir: &Path, hour: bool) -> Vec<String> {
    let nproc = thread::available_parallelism().map_or(0, |n| n.get());
    let model = command_line("lscpu", &[])
        .lines()
        .find_map(|l| l.strip_prefix("Model name:").map(|s| s.trim().to_string()))
        .unwrap_or_else(|| "unknown".into());
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let repo_s = repo.to_string_lossy().to_string();
    let commit = command_line("git", &["-C", &repo_s, "rev-parse", "--short", "HEAD"]);
    let dirty = if command_line("git", &["-C", &repo_s, "status", "--porcelain"]).is_empty() {
        ""
    } else {
        " (with uncommitted changes)"
    };
    vec![
        format!(
            "machine: {nproc} cpus, {model}, {} kernel {kernel}",
            std::env::consts::ARCH
        ),
        format!("commit:  {commit}{dirty}"),
        format!(
            "mosh-rs: {} (binary_size: {})",
            rust_dir.display(),
            size_dir.display()
        ),
        "mosh 1.4.0: /usr/bin/mosh-client, /usr/bin/mosh-server".into(),
        format!("mode:    {}", if hour { "--hour" } else { "short" }),
    ]
}

/// The lock tests/interop.sh takes, so a bench run and an interop run never
/// share the machine's CPUs.
fn take_lock() -> File {
    let f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOCK_FILE)
        .unwrap_or_else(|e| {
            eprintln!("bench: open {LOCK_FILE}: {e}");
            exit(1);
        });
    // SAFETY: flock on a valid descriptor we own.
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        eprintln!("bench: another run holds {LOCK_FILE}, waiting");
        // SAFETY: as above.
        if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX) } != 0 {
            eprintln!(
                "bench: flock {LOCK_FILE}: {}",
                std::io::Error::last_os_error()
            );
            exit(1);
        }
    }
    f
}

/// Nothing may be bound in our port range: a leftover server would spend
/// CPU and memory during the measurements.
fn check_ports_free() -> Result<(), String> {
    let out = command_line("ss", &["-Hulpn"]);
    let busy: Vec<&str> = out
        .lines()
        .filter(|l| {
            l.split_whitespace()
                .nth(3)
                .and_then(|a| a.rsplit(':').next())
                .and_then(|p| p.parse::<u16>().ok())
                .is_some_and(|p| (PORT_FIRST..=PORT_LAST).contains(&p))
        })
        .collect();
    if busy.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "UDP ports {PORT_FIRST}-{PORT_LAST} in use: {}",
            busy.join(" | ")
        ))
    }
}

/// Slots of SLOT_PORTS ports in PORT_FIRST..=PORT_LAST, handed out in turn.
struct Ports {
    next: u16,
}

impl Ports {
    /// The first port of a slot whose ports are all free.
    fn slot(&mut self) -> Result<u16, String> {
        let slots = (PORT_LAST - PORT_FIRST + 1) / SLOT_PORTS;
        for _ in 0..slots {
            let base = PORT_FIRST + self.next * SLOT_PORTS;
            self.next = (self.next + 1) % slots;
            let free = (base..base + SLOT_PORTS).all(|p| UdpSocket::bind(("127.0.0.1", p)).is_ok());
            if free {
                return Ok(base);
            }
        }
        Err(format!("no free port slot in {PORT_FIRST}-{PORT_LAST}"))
    }
}

// ---- Processes.

fn kill(pid: i32) {
    // SAFETY: kill(2) with a pid we started.
    unsafe {
        libc::kill(pid, libc::SIGKILL);
    }
}

fn alive(pid: i32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
        && !fs::read_to_string(format!("/proc/{pid}/stat"))
            .unwrap_or_default()
            .rsplit(')')
            .next()
            .unwrap_or("")
            .trim_start()
            .starts_with('Z')
}

/// utime + stime of `pid`, in clock ticks.
fn cpu_ticks(pid: i32) -> Result<u64, String> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat"))
        .map_err(|e| format!("read /proc/{pid}/stat: {e}"))?;
    // Fields after the command name, which is in parentheses and may hold
    // spaces; the first one is field 3 (state), utime is 14, stime 15.
    let rest = stat.rsplit(')').next().unwrap_or("");
    let f: Vec<&str> = rest.split_whitespace().collect();
    let utime: u64 = f.get(11).and_then(|s| s.parse().ok()).ok_or("bad stat")?;
    let stime: u64 = f.get(12).and_then(|s| s.parse().ok()).ok_or("bad stat")?;
    Ok(utime + stime)
}

fn clk_tck() -> f64 {
    // SAFETY: sysconf has no preconditions.
    unsafe { libc::sysconf(libc::_SC_CLK_TCK) as f64 }
}

/// (VmRSS, VmHWM) of `pid` in kB.
fn memory_kb(pid: i32) -> Result<(u64, u64), String> {
    let status = fs::read_to_string(format!("/proc/{pid}/status"))
        .map_err(|e| format!("read /proc/{pid}/status: {e}"))?;
    let field = |name: &str| -> Result<u64, String> {
        status
            .lines()
            .find_map(|l| l.strip_prefix(name))
            .and_then(|v| v.split_whitespace().next())
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| format!("no {name} in /proc/{pid}/status"))
    };
    Ok((field("VmRSS:")?, field("VmHWM:")?))
}

/// `mosh-server new` on `port` running `sh -c script`; returns the key and
/// the pid of the detached server. Both servers print
/// `[mosh-server detached, pid = N]` and point their stdio at /dev/null
/// after detaching, so the output can be read to the end.
fn start_server(imp: &Imp, port: u16, script: &str) -> Result<(String, i32), String> {
    let out = Command::new(&imp.server)
        .args([
            "new",
            "-c",
            "256",
            "-i",
            "127.0.0.1",
            "-p",
            &port.to_string(),
        ])
        .args(["--", "sh", "-c", script])
        .env("LANG", "C.UTF-8")
        .env("TERM", "xterm-256color")
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("run {}: {e}", imp.server.display()))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let key = stdout
        .lines()
        .find_map(|l| l.strip_prefix("MOSH CONNECT "))
        .and_then(|r| r.split_whitespace().nth(1))
        .ok_or_else(|| format!("{}: no MOSH CONNECT line: {stdout} {stderr}", imp.label))?;
    let pid = stderr
        .lines()
        .find_map(|l| l.strip_prefix("[mosh-server detached, pid = "))
        .and_then(|r| r.trim_end_matches(']').parse().ok())
        .ok_or_else(|| format!("{}: no detached pid line: {stderr}", imp.label))?;
    Ok((key.to_string(), pid))
}

/// What the client wrote to its terminal. `text` is the output with escape
/// sequences and control characters removed (see `Strip`), and `chunks`
/// holds the time of every read from the pty master with the length of
/// `text` after it. One read is normally one write by the client; two
/// writes closer together than the reader thread can wake up would be
/// seen as one (README, "Limits").
struct Log {
    text: Vec<u8>,
    chunks: Vec<(Instant, usize)>,
    /// Keep only the last TAIL bytes of text and no chunks (long sessions).
    tail_only: bool,
    last_read: Option<Instant>,
}

const TAIL: usize = 8192;

impl Log {
    /// The time of the first read after which `needle` is in `text[from..]`.
    fn find(&self, needle: &[u8], from: usize) -> Option<Instant> {
        let hay = self.text.get(from..)?;
        let at = hay.windows(needle.len()).position(|w| w == needle)?;
        let end = from + at + needle.len();
        self.chunks.iter().find(|c| c.1 >= end).map(|c| c.0)
    }
    fn reads_between(&self, from: Instant, to: Instant) -> usize {
        self.chunks
            .iter()
            .filter(|c| c.0 >= from && c.0 < to)
            .count()
    }
}

/// Removes escape sequences and control characters from the client's
/// output, keeping printable ASCII (other characters become '?'). Cursor
/// motions and line ends become '\n', so text from different places on the
/// screen is never joined into a false match.
#[derive(Clone, Copy)]
enum Strip {
    Ground,
    Esc,
    Csi,
    Osc,
    OscEsc,
    Charset,
}

impl Strip {
    fn feed(&mut self, input: &[u8], out: &mut Vec<u8>) {
        for &b in input {
            *self = match (*self, b) {
                (Strip::Ground, 0x1b) => Strip::Esc,
                (Strip::Ground, 0x20..=0x7e) => {
                    out.push(b);
                    Strip::Ground
                }
                (Strip::Ground, 0xc0..=0xff) => {
                    out.push(b'?');
                    Strip::Ground
                }
                (Strip::Ground, b'\r' | b'\n') => {
                    out.push(b'\n');
                    Strip::Ground
                }
                (Strip::Ground, _) => Strip::Ground,
                (Strip::Esc, b'[') => Strip::Csi,
                (Strip::Esc, b']' | b'P' | b'_' | b'^') => Strip::Osc,
                (Strip::Esc, b'(' | b')' | b'*' | b'+' | b'#' | b'%') => Strip::Charset,
                (Strip::Esc, _) => {
                    out.push(b'\n');
                    Strip::Ground
                }
                (Strip::Csi, 0x40..=0x7e) => {
                    if b != b'm' {
                        out.push(b'\n');
                    }
                    Strip::Ground
                }
                (Strip::Csi, _) => Strip::Csi,
                (Strip::Osc, 0x07) => Strip::Ground,
                (Strip::Osc, 0x1b) => Strip::OscEsc,
                (Strip::Osc, _) => Strip::Osc,
                (Strip::OscEsc, b'\\') => Strip::Ground,
                (Strip::OscEsc, _) => Strip::Osc,
                (Strip::Charset, _) => Strip::Ground,
            };
        }
    }
}

/// A client running on a pty owned by this program.
struct Client {
    child: Child,
    master: File,
    log: Arc<Mutex<Log>>,
    reader: Option<JoinHandle<()>>,
}

impl Client {
    /// Starts `imp.client 127.0.0.1 port` with `MOSH_KEY`. Returns the
    /// client and the instant just before the fork.
    fn spawn(
        imp: &Imp,
        port: u16,
        key: &str,
        env: &[(&str, &str)],
        tail_only: bool,
    ) -> Result<(Client, Instant), String> {
        let ws = nix::pty::Winsize {
            ws_row: ROWS,
            ws_col: COLS,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        let pty = nix::pty::openpty(Some(&ws), None).map_err(|e| format!("openpty: {e}"))?;
        let master: OwnedFd = pty.master;
        // SAFETY: fcntl on a descriptor we own; later children must not
        // inherit the master.
        unsafe {
            libc::fcntl(master.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC);
        }
        let slave = pty.slave;
        let dup = |fd: &OwnedFd| fd.try_clone().map_err(|e| format!("dup: {e}"));
        let mut cmd = Command::new(&imp.client);
        cmd.arg("127.0.0.1")
            .arg(port.to_string())
            .env("MOSH_KEY", key)
            .env("TERM", "xterm-256color")
            .env("LANG", "C.UTF-8")
            .env_remove("MOSH_PREDICTION_DISPLAY")
            .stdin(Stdio::from(dup(&slave)?))
            .stdout(Stdio::from(dup(&slave)?))
            .stderr(Stdio::from(slave));
        for (k, v) in env {
            cmd.env(k, v);
        }
        // SAFETY: only async-signal-safe calls between fork and exec.
        unsafe {
            cmd.pre_exec(|| {
                if libc::setsid() < 0 || libc::ioctl(0, libc::TIOCSCTTY, 0) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let t0 = Instant::now();
        let child = cmd
            .spawn()
            .map_err(|e| format!("run {}: {e}", imp.client.display()))?;
        drop(cmd); // closes our copies of the slave

        let log = Arc::new(Mutex::new(Log {
            text: Vec::new(),
            chunks: Vec::new(),
            tail_only,
            last_read: None,
        }));
        let mut rd = File::from(master);
        let master = rd.try_clone().map_err(|e| format!("dup: {e}"))?;
        let log2 = Arc::clone(&log);
        let reader = thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let mut strip = Strip::Ground;
            loop {
                let n = match rd.read(&mut buf) {
                    Ok(0) | Err(_) => break, // EIO once the client is gone
                    Ok(n) => n,
                };
                let t = Instant::now();
                let mut l = log2.lock().unwrap();
                strip.feed(&buf[..n], &mut l.text);
                l.last_read = Some(t);
                if l.tail_only {
                    if l.text.len() > 2 * TAIL {
                        let cut = l.text.len() - TAIL;
                        l.text.drain(..cut);
                    }
                } else {
                    let len = l.text.len();
                    l.chunks.push((t, len));
                }
            }
        });
        Ok((
            Client {
                child,
                master,
                log,
                reader: Some(reader),
            },
            t0,
        ))
    }

    fn pid(&self) -> i32 {
        self.child.id() as i32
    }

    /// Writes `bytes` as typed input; returns the time just before the write
    /// and the length of the text log at that moment.
    fn type_bytes(&mut self, bytes: &[u8]) -> Result<(Instant, usize), String> {
        let from = self.log.lock().unwrap().text.len();
        let t = Instant::now();
        self.master
            .write_all(bytes)
            .map_err(|e| format!("write to client pty: {e}"))?;
        Ok((t, from))
    }

    /// Waits until `needle` is in the output after offset `from`; returns
    /// the time of the read that completed it.
    fn wait_for(&mut self, needle: &str, from: usize, limit: Duration) -> Result<Instant, String> {
        let start = Instant::now();
        loop {
            {
                let l = self.log.lock().unwrap();
                if l.tail_only {
                    if l.text.windows(needle.len()).any(|w| w == needle.as_bytes()) {
                        return Ok(Instant::now());
                    }
                } else if let Some(t) = l.find(needle.as_bytes(), from) {
                    return Ok(t);
                }
            }
            if let Ok(Some(st)) = self.child.try_wait() {
                return Err(format!("client exited ({st}) while waiting for {needle:?}"));
            }
            if start.elapsed() > limit {
                let l = self.log.lock().unwrap();
                let tail = String::from_utf8_lossy(&l.text[l.text.len().saturating_sub(200)..])
                    .replace('\n', " ");
                return Err(format!(
                    "timed out after {limit:?} waiting for {needle:?}; output ends: {tail}"
                ));
            }
            sleep(Duration::from_millis(1));
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // SIGKILL, not SIGHUP or SIGTERM: those make mosh-client send
        // shutdown packets for seconds (tests/interop.sh, kill_client).
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(r) = self.reader.take() {
            let _ = r.join();
        }
    }
}

/// A server, an optional udp-impair proxy, and a client.
struct Session {
    client: Client,
    server_pid: i32,
    proxy: Option<Child>,
}

impl Drop for Session {
    fn drop(&mut self) {
        // The client first, so nothing answers the server; then the server
        // (its session gets SIGHUP when the pty closes), then the proxy.
        let _ = self.client.child.kill();
        let _ = self.client.child.wait();
        kill(self.server_pid);
        if let Some(p) = &mut self.proxy {
            let _ = p.kill();
            let _ = p.wait();
        }
    }
}

/// Starts udp-impair from `listen` to `target`; waits for its startup line.
fn start_proxy(impair: &Path, listen: u16, target: u16, control: u16) -> Result<Child, String> {
    let mut p = Command::new(impair)
        .args(["--listen", &format!("127.0.0.1:{listen}")])
        .args(["--target", &format!("127.0.0.1:{target}")])
        .args(["--control", &format!("127.0.0.1:{control}")])
        .args(["--delay-ms", &KEY_DELAY_MS.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("run {}: {e}", impair.display()))?;
    let mut line = String::new();
    let _ = BufReader::new(p.stdout.take().unwrap()).read_line(&mut line);
    if !line.starts_with("udp-impair listen=") {
        let _ = p.kill();
        let _ = p.wait();
        return Err(format!("udp-impair did not start: {line:?}"));
    }
    Ok(p)
}

/// A whole session: server running `script`, then the client, directly or
/// through the keystroke proxy.
fn session(
    imp: &Imp,
    ports: &mut Ports,
    script: &str,
    env: &[(&str, &str)],
    impair: Option<&Path>,
    tail_only: bool,
) -> Result<Session, String> {
    let base = ports.slot()?;
    let (key, server_pid) = start_server(imp, base, script)?;
    let mut proxy = None;
    let mut port = base;
    if let Some(impair) = impair {
        match start_proxy(impair, base + 1, base, base + 2) {
            Ok(p) => proxy = Some(p),
            Err(e) => {
                kill(server_pid);
                return Err(e);
            }
        }
        port = base + 1;
    }
    match Client::spawn(imp, port, &key, env, tail_only) {
        Ok((client, _)) => Ok(Session {
            client,
            server_pid,
            proxy,
        }),
        Err(e) => {
            kill(server_pid);
            if let Some(p) = &mut proxy {
                let _ = p.kill();
                let _ = p.wait();
            }
            Err(e)
        }
    }
}

// ---- Statistics and formatting.

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// Nearest-rank percentile of `v` (0 < p <= 100).
fn percentile(v: &[f64], p: f64) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let rank = ((p / 100.0) * s.len() as f64).ceil().max(1.0) as usize;
    s[rank - 1]
}

fn fmt_ms(v: f64) -> String {
    format!("{v:.2} ms")
}

fn fmt_kb(v: u64) -> String {
    format!("{v} kB")
}

fn print_table(rows: &[Row]) {
    let head = ["metric", "target", "mosh-rs", "mosh 1.4.0", "verdict"];
    let cells: Vec<[&str; 5]> = rows
        .iter()
        .map(|r| {
            [
                r.metric.as_str(),
                r.target.as_str(),
                r.rust.as_str(),
                r.cxx.as_str(),
                r.verdict.as_str(),
            ]
        })
        .collect();
    let mut w = head.map(str::len);
    for c in &cells {
        for i in 0..5 {
            w[i] = w[i].max(c[i].len());
        }
    }
    let line = |c: [&str; 5]| {
        let s: Vec<String> = (0..5).map(|i| format!("{:<1$}", c[i], w[i])).collect();
        println!("{}", s.join("  ").trim_end());
    };
    println!();
    line(head);
    line(w.map(|n| "-".repeat(n)).each_ref().map(String::as_str));
    for c in cells {
        line(c);
    }
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn print_json(header: &[String], rows: &[Row]) {
    let h: Vec<String> = header.iter().map(|s| json_str(s)).collect();
    let r: Vec<String> = rows
        .iter()
        .map(|r| {
            format!(
                "{{\"metric\":{},\"target\":{},\"mosh-rs\":{},\"mosh-1.4.0\":{},\"verdict\":{}}}",
                json_str(&r.metric),
                json_str(&r.target),
                json_str(&r.rust),
                json_str(&r.cxx),
                json_str(&r.verdict)
            )
        })
        .collect();
    println!(
        "{{\"header\":[{}],\"rows\":[\n{}\n]}}",
        h.join(","),
        r.join(",\n")
    );
}

// ---- Metrics.

fn binary_size(dir: &Path) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for b in ["mosh", "mosh-client", "mosh-server"] {
        let rs = fs::metadata(dir.join(b))
            .map_err(|e| format!("{}: {e}", dir.join(b).display()))?
            .len();
        let cx = fs::metadata(Path::new("/usr/bin").join(b))
            .map(|m| m.len())
            .unwrap_or(0);
        let file = command_line("file", &["-b", &dir.join(b).to_string_lossy()]);
        let kind = if !file.starts_with("ELF") {
            "not an ELF file"
        } else if file.contains("static") {
            "static"
        } else {
            "dynamic"
        };
        let cx_kind = if b == "mosh" {
            "perl script"
        } else {
            "dynamic"
        };
        rows.push(row(
            &format!("binary_size {b}"),
            "< 4000000 B",
            format!("{rs} B ({kind})"),
            format!("{cx} B ({cx_kind})"),
            pass(rs < BINARY_MAX_BYTES),
        ));
    }
    Ok(rows)
}

/// Client start to the first write that puts the session's first output on
/// the client's terminal. The server is started (and its shell has printed
/// the marker) before the clock starts.
fn startup(imps: &[&Imp], ports: &mut Ports) -> Result<Vec<Row>, String> {
    const MARK: &str = "BENCHSTARTMARK";
    let script = format!("echo {MARK}; exec sleep 100000");
    // [implementation][run] = (to the marker, to the first write of any kind)
    let mut times: Vec<Vec<(f64, f64)>> = vec![Vec::new(); imps.len()];
    for run in 0..STARTUP_RUNS {
        for (i, imp) in imps.iter().enumerate() {
            let base = ports.slot()?;
            let (key, server_pid) = start_server(imp, base, &script)?;
            sleep(Duration::from_millis(300));
            let r = Client::spawn(imp, base, &key, &[], false).and_then(|(mut c, t0)| {
                let t = c.wait_for(MARK, 0, Duration::from_secs(10))?;
                let first = c.log.lock().unwrap().chunks[0].0;
                Ok((ms(t - t0), ms(first - t0)))
            });
            kill(server_pid);
            let v = r.map_err(|e| format!("{}: {e}", imp.label))?;
            eprintln!(
                "startup run {run} {}: {:.2} ms to the first frame, {:.2} ms to the first write",
                imp.label, v.0, v.1
            );
            times[i].push(v);
        }
    }
    let stat = |p: f64, first: bool| -> Vec<f64> {
        times
            .iter()
            .map(|t| {
                let v: Vec<f64> = t.iter().map(|x| if first { x.1 } else { x.0 }).collect();
                percentile(&v, p)
            })
            .collect()
    };
    let (med, p90) = (stat(50.0, false), stat(90.0, false));
    let (wmed, wp90) = (stat(50.0, true), stat(90.0, true));
    Ok(vec![
        row(
            "startup to first frame, median (20 runs)",
            "< 20 ms",
            fmt_ms(med[0]),
            fmt_ms(med[1]),
            pass(med[0] < STARTUP_MAX_MS),
        ),
        row(
            "startup to first frame, p90",
            "-",
            fmt_ms(p90[0]),
            fmt_ms(p90[1]),
            "INFO",
        ),
        row(
            "startup to first write, median",
            "-",
            fmt_ms(wmed[0]),
            fmt_ms(wmed[1]),
            "INFO",
        ),
        row(
            "startup to first write, p90",
            "-",
            fmt_ms(wp90[0]),
            fmt_ms(wp90[1]),
            "INFO",
        ),
    ])
}

/// Keystroke to screen at 300 ms round trip with prediction always shown.
fn keystroke(imps: &[&Imp], ports: &mut Ports, impair: &Path) -> Result<Vec<Row>, String> {
    let script = "echo KEYREADY; export PS1='$ '; exec bash --norc --noprofile -i";
    let env = [("MOSH_PREDICTION_DISPLAY", "always")];
    let mut sessions = Vec::new();
    for imp in imps {
        let mut s = session(imp, ports, script, &env, Some(impair), false)?;
        s.client.wait_for("KEYREADY", 0, Duration::from_secs(20))?;
        s.client.wait_for("$ ", 0, Duration::from_secs(20))?;
        sessions.push(s);
    }
    // Start a line with '#' so Enter runs a comment, and let the server's
    // echo confirm it: upstream shows no prediction of an unconfirmed epoch.
    let new_line = |sessions: &mut Vec<Session>| -> Result<(), String> {
        for s in sessions.iter_mut() {
            let (_, from) = s.client.type_bytes(b"#")?;
            s.client.wait_for("#", from, Duration::from_secs(10))?;
        }
        sleep(Duration::from_millis(1500));
        Ok(())
    };
    sleep(Duration::from_millis(1000));
    new_line(&mut sessions)?;

    let letters = b"abcdefgijk";
    let mut lat: Vec<Vec<f64>> = vec![Vec::new(); imps.len()];
    let mut renders: Vec<Vec<f64>> = vec![Vec::new(); imps.len()];
    let mut writes: Vec<Vec<f64>> = vec![Vec::new(); imps.len()];
    for k in 0..KEYSTROKE_RUNS {
        if k > 0 && k % letters.len() == 0 {
            for s in sessions.iter_mut() {
                s.client.type_bytes(b"\r")?;
            }
            sleep(Duration::from_millis(1500));
            new_line(&mut sessions)?;
        }
        let c = letters[k % letters.len()];
        let mut starts = Vec::new();
        for (i, s) in sessions.iter_mut().enumerate() {
            let (t0, from) = s.client.type_bytes(&[c])?;
            let t = s
                .client
                .wait_for(&(c as char).to_string(), from, Duration::from_secs(5))
                .map_err(|e| format!("{}: {e}", imps[i].label))?;
            lat[i].push(ms(t - t0));
            starts.push(t0);
        }
        sleep(SETTLE_WINDOW);
        for (i, s) in sessions.iter().enumerate() {
            let l = s.client.log.lock().unwrap();
            let r = l.reads_between(starts[i], starts[i] + RENDER_WINDOW);
            let w = l.reads_between(starts[i], starts[i] + SETTLE_WINDOW);
            eprintln!(
                "keystroke {k} {}: {:.2} ms, {r} write(s) before the echo, {w} in {} ms",
                imps[i].label,
                lat[i][k],
                SETTLE_WINDOW.as_millis()
            );
            renders[i].push(r as f64);
            writes[i].push(w as f64);
        }
    }
    let f =
        |v: &Vec<Vec<f64>>, p: f64| -> Vec<f64> { v.iter().map(|x| percentile(x, p)).collect() };
    let (med, p90) = (f(&lat, 50.0), f(&lat, 90.0));
    let rmax: Vec<f64> = renders
        .iter()
        .map(|x| x.iter().cloned().fold(0.0, f64::max))
        .collect();
    let rmin: Vec<f64> = renders
        .iter()
        .map(|x| x.iter().cloned().fold(f64::MAX, f64::min))
        .collect();
    let wmed = f(&writes, 50.0);
    let renders_s = |i: usize| {
        if rmin[i] == rmax[i] {
            format!("{} in every run", rmax[i])
        } else {
            format!("{}-{}", rmin[i], rmax[i])
        }
    };
    Ok(vec![
        row(
            "keystroke median (30 keys, 300 ms RTT)",
            "< 16 ms",
            fmt_ms(med[0]),
            fmt_ms(med[1]),
            pass(med[0] < KEYSTROKE_MAX_MS),
        ),
        row("keystroke p90", "-", fmt_ms(p90[0]), fmt_ms(p90[1]), "INFO"),
        row(
            "keystroke renders (writes in first 250 ms)",
            "1",
            renders_s(0),
            renders_s(1),
            pass(rmin[0] == 1.0 && rmax[0] == 1.0),
        ),
        row(
            "keystroke writes in 1200 ms (median)",
            "-",
            format!("{}", wmed[0]),
            format!("{}", wmed[1]),
            "INFO",
        ),
    ])
}

/// `seq 1 100000` from Enter to the end marker on the client's terminal.
fn seq(imps: &[&Imp], ports: &mut Ports) -> Result<Vec<Row>, String> {
    const DONE: &str = "ZZSEQFINISHEDZZ";
    let script = format!("echo SEQREADY; read x; seq 1 100000; echo {DONE}; exec sleep 100000");
    let mut times: Vec<Vec<f64>> = vec![Vec::new(); imps.len()];
    for run in 0..SEQ_RUNS {
        for (i, imp) in imps.iter().enumerate() {
            let mut s = session(imp, ports, &script, &[], None, false)?;
            let r = (|| {
                s.client.wait_for("SEQREADY", 0, Duration::from_secs(20))?;
                sleep(Duration::from_millis(500));
                let (t0, from) = s.client.type_bytes(b"\r")?;
                let t = s.client.wait_for(DONE, from, Duration::from_secs(300))?;
                Ok::<f64, String>(ms(t - t0))
            })();
            drop(s);
            let v = r.map_err(|e| format!("{}: {e}", imp.label))?;
            eprintln!("seq run {run} {}: {v:.0} ms", imp.label);
            times[i].push(v);
        }
    }
    let med: Vec<f64> = times.iter().map(|t| percentile(t, 50.0)).collect();
    let all = |t: &Vec<f64>| {
        t.iter()
            .map(|v| format!("{v:.0}"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    Ok(vec![
        row(
            "seq 1 100000 median (5 runs)",
            "<= C++ + max(5%, 20 ms)",
            fmt_ms(med[0]),
            fmt_ms(med[1]),
            pass(med[0] <= med[1] + (med[1] * SEQ_TOLERANCE).max(SEQ_FRAME_MS)),
        ),
        row("seq runs (ms)", "-", all(&times[0]), all(&times[1]), "INFO"),
    ])
}

/// CPU of a connected, idle session over IDLE_SECS. Both sessions run at
/// the same time, so they share the same machine conditions.
fn idle_cpu(imps: &[&Imp], ports: &mut Ports) -> Result<Vec<Row>, String> {
    let script = "echo IDLEREADY; export PS1='$ '; exec bash --norc --noprofile -i";
    let mut sessions = Vec::new();
    for imp in imps {
        let mut s = session(imp, ports, script, &[], None, true)?;
        s.client.wait_for("IDLEREADY", 0, Duration::from_secs(20))?;
        sessions.push(s);
    }
    sleep(Duration::from_secs(5));
    let pids: Vec<[i32; 2]> = sessions
        .iter()
        .map(|s| [s.client.pid(), s.server_pid])
        .collect();
    let before: Vec<[u64; 2]> = pids
        .iter()
        .map(|p| Ok([cpu_ticks(p[0])?, cpu_ticks(p[1])?]))
        .collect::<Result<_, String>>()?;
    eprintln!("idle_cpu: sampling for {IDLE_SECS} s");
    let t0 = Instant::now();
    sleep(Duration::from_secs(IDLE_SECS));
    let secs = t0.elapsed().as_secs_f64();
    let after: Vec<[u64; 2]> = pids
        .iter()
        .map(|p| Ok([cpu_ticks(p[0])?, cpu_ticks(p[1])?]))
        .collect::<Result<_, String>>()?;
    let pct = |i: usize, j: usize| (after[i][j] - before[i][j]) as f64 / clk_tck() / secs * 100.0;
    let mut rows = Vec::new();
    for (j, who) in ["client", "server"].iter().enumerate() {
        let (r, c) = (pct(0, j), pct(1, j));
        rows.push(row(
            &format!("idle_cpu {who} ({IDLE_SECS} s)"),
            "< 0.05%",
            format!("{r:.2}% ({} ticks)", after[0][j] - before[0][j]),
            format!("{c:.2}% ({} ticks)", after[1][j] - before[1][j]),
            pass(r < IDLE_CPU_MAX_PCT),
        ));
    }
    Ok(rows)
}

/// Memory after an active session of RSS_SHORT_SECS (or an hour). Both
/// sessions run at the same time with the same workload and typing.
fn rss(imps: &[&Imp], ports: &mut Ports, hour: bool) -> Result<Vec<Row>, String> {
    let secs = if hour { RSS_HOUR_SECS } else { RSS_SHORT_SECS };
    let mut sessions = Vec::new();
    for imp in imps {
        let s = session(
            imp,
            ports,
            &format!("exec bash -c '{}'", RSS_WORKLOAD.replace('\'', "'\\''")),
            &[],
            None,
            true,
        )?;
        sessions.push(s);
    }
    for s in sessions.iter_mut() {
        s.client.wait_for("tick", 0, Duration::from_secs(20))?;
    }
    eprintln!("rss: {secs} s session");
    let t0 = Instant::now();
    let mut typed = 0;
    let mut peak = vec![[0u64; 2]; imps.len()];
    loop {
        let e = t0.elapsed().as_secs();
        if e >= secs {
            break;
        }
        if e >= (typed + 1) * 30 {
            typed += 1;
            for s in sessions.iter_mut() {
                s.client
                    .type_bytes(format!("command {typed}\r").as_bytes())?;
            }
        }
        for (i, s) in sessions.iter().enumerate() {
            for (j, pid) in [s.client.pid(), s.server_pid].into_iter().enumerate() {
                if !alive(pid) {
                    return Err(format!(
                        "{}: process {pid} died during the session",
                        imps[i].label
                    ));
                }
                peak[i][j] = peak[i][j].max(memory_kb(pid)?.0);
            }
        }
        if e.is_multiple_of(60) {
            eprintln!("rss: {e} s");
        }
        sleep(Duration::from_secs(1));
    }
    // The sessions must still be live: the screen changed in the last 5 s.
    for (i, s) in sessions.iter().enumerate() {
        let last = s.client.log.lock().unwrap().last_read;
        if last.is_none_or(|t| t.elapsed() > Duration::from_secs(5)) {
            return Err(format!("{}: client output stopped", imps[i].label));
        }
    }
    let mut rows = Vec::new();
    let label = if hour { "1 h" } else { "5 min" };
    for (j, who) in ["client", "server"].iter().enumerate() {
        let m: Vec<(u64, u64)> = sessions
            .iter()
            .map(|s| memory_kb([s.client.pid(), s.server_pid][j]))
            .collect::<Result<_, String>>()?;
        rows.push(row(
            &format!("rss {who} VmRSS after {label}"),
            "<= C++",
            fmt_kb(m[0].0),
            fmt_kb(m[1].0),
            pass(m[0].0 <= m[1].0),
        ));
        rows.push(row(
            &format!("rss {who} VmHWM after {label}"),
            "<= C++",
            fmt_kb(m[0].1),
            fmt_kb(m[1].1),
            pass(m[0].1 <= m[1].1),
        ));
        rows.push(row(
            &format!("rss {who} VmRSS peak (1 s samples)"),
            "-",
            fmt_kb(peak[0][j]),
            fmt_kb(peak[1][j]),
            "INFO",
        ));
    }
    Ok(rows)
}
