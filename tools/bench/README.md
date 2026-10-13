# bench

`tools/bench` measures the PLAN.md section 5 performance targets for mosh-rs and for mosh 1.4.0 on the same machine, in the same harness, and prints one table.

```sh
cargo build --release -p mosh -p mosh-tools
target/release/bench [--only NAME] [--hour] [--json] [--rust-dir DIR]
```

- mosh-rs is `target/release/mosh-client` and `mosh-server` (the directory of the `bench` binary), or the ones in `--rust-dir DIR`.
- mosh 1.4.0 is `/usr/bin/mosh-client` and `/usr/bin/mosh-server` from apt.
- `--only NAME` runs one metric: `binary_size`, `startup`, `keystroke`, `seq`, `idle_cpu`, `rss`.
- `--hour` makes the `rss` session one hour long instead of five minutes.
- `--json` prints the header and rows as JSON instead of the table.
- Progress goes to stderr, the table to stdout.
  The exit status is 1 if any verdict is FAIL or ERROR.

The header names the machine (cpus, model, architecture, kernel) and the git commit.
The metrics run in the order above, so the long `rss` session is last and shares no time with the latency metrics.

## How a session is run

Like `tests/interop.sh`: `mosh-server new -c 256 -i 127.0.0.1 -p PORT -- sh -c SCRIPT` prints `MOSH CONNECT PORT KEY` and `[mosh-server detached, pid = N]`, and the client is started as `mosh-client 127.0.0.1 PORT` with `MOSH_KEY`, `TERM=xterm-256color` and `LANG=C.UTF-8`.

The client runs on an 80x24 pseudoterminal that `bench` owns, not in tmux.
A reader thread reads the pty master and records the time of every read.
Escape sequences and control characters are removed from what the client wrote, and markers (plain capital letters that never share a screen cell with the text they replace) are searched in what is left.
So "on the screen" means "the client wrote it to its terminal".

Teardown is SIGKILL for the client and then the server, never SIGHUP or SIGTERM: both answer those by sending shutdown packets for seconds, which a later session on the same port would receive.

`bench` uses UDP ports 61400-61499 only, and refuses to start if anything is bound there.
It holds `/tmp/mosh-rs-interop.lock` (the lock `tests/interop.sh` takes) for the whole run, so a bench run and an interop run never share the machine.
`--hour` therefore blocks interop runs for over an hour.

## Metrics and verdicts

Every metric that runs sessions alternates between the two implementations (one C++ run, one Rust run, and so on), or runs both sessions at the same time, so machine noise falls on both.

### binary_size

Bytes of `mosh`, `mosh-client` and `mosh-server`.
The mosh-rs binaries are the static musl ones from `target/<arch>-unknown-linux-musl/release/` when they exist (build them with `scripts/build-release.sh`), else the measured ones; the header says which directory.
Verdict: each under 4,000,000 bytes.
The C++ column is for reference only: those binaries are dynamically linked (protobuf, OpenSSL, ncurses and libstdc++ are not counted) and `/usr/bin/mosh` is a Perl script.

### startup

The server is started first and its shell has printed a marker 300 ms before the clock starts.
The clock starts just before the client is forked, and stops at the first read from the pty that completes the marker, that is, the first frame showing the remote screen.
20 runs per implementation, each with a new server.
Rows: median and p90 to the first frame, and median and p90 to the client's first write of any kind (its terminal setup), which shows the client's own start-up cost.
Verdict, PLAN section 5 as decided for this project: the median to the first frame is no slower than C++ (the same tolerance as `seq`: 5% or 20 ms, whichever is larger), and the median to the first write is under 20 ms.
The first frame cannot be much faster than 250 ms with upstream's timing: the server sends after its send interval, `ceil(SRTT/2)` clamped to 20-250 ms, and SRTT starts at 1000 ms (upstream `transportsender-impl.h`, `network.cc`).

### keystroke

Both sessions run through `udp-impair --delay-ms 150` (300 ms round trip), with `MOSH_PREDICTION_DISPLAY=always` and an interactive `bash --norc` on the server.
Each line starts with `#`, which is echoed and confirmed before measuring (mosh shows no prediction until the server has confirmed one), so Enter only runs a comment.
Then 30 letters are typed, 10 per line, alternating between the implementations.
For each letter: the time from the write of that byte to the pty until the read in which the letter appears, and the number of reads (client writes) in the first 250 ms.
250 ms ends before the server's echo can come back, so those writes are the prediction alone.
Rows: median and p90 latency, the render count, and for information the number of writes in 1200 ms, which includes the redraw when the server's echo confirms the prediction (at 300 ms RTT the prediction is first shown underlined, and the underline is removed on confirmation).
Verdict: median latency under 16 ms (one frame at 60 Hz: "no visible delay") and exactly one write in the first 250 ms for every key.

### seq

`seq 1 100000` on loopback, no proxy: the clock starts with the Enter that starts `seq` and stops when the end marker that follows it has been written by the client.
5 runs per implementation, alternating, each with a new session.
This is screen completion: the time until the marker is on the client's screen, not until any scrollback history (PLAN section 6) has arrived.
The clients are started without any option or variable that requests scrollback.
Verdict: the mosh-rs median is at most the C++ median plus 5% or 20 ms, whichever is larger.
The tolerance exists because the server sends frames at least 20 ms apart (upstream `SEND_INTERVAL_MIN`), so the time is a whole number of frames and two equal implementations differ by a frame from noise alone.

### idle_cpu

One connected session per implementation, both at the same time, each an idle `bash` prompt.
After 5 s of settling, utime + stime of the client and the server process (from `/proc/PID/stat`) over 60 s, as a percentage of one CPU.
The tick count is shown too; one tick is 10 ms, 0.017% of 60 s.
Verdict: under 0.05% (rounds to 0.0%, PLAN's target, and the same bound as `tests/tests/client_e2e.rs`).

### rss

One session per implementation, both at the same time, each running the same workload for 5 minutes (`--hour`: 60 minutes):

- The remote script redraws 22 lines of changing text (a tick counter, the time and random numbers) from the top of the screen every second.
- Every 300 ticks, starting at tick 60, it prints `seq 1 20000`.
- Every 30 s `bench` types `command N` and Enter into the client's terminal; the script reads it and prints it.

The script is `RSS_WORKLOAD` in `unix.rs`.
Every second `bench` checks that all four processes are alive and records VmRSS; at the end the client must still be receiving output.
Rows: VmRSS and VmHWM (peak resident) from `/proc/PID/status` at the end, for client and server, and the highest 1-second VmRSS sample for information.
Verdict: mosh-rs at most the C++ value, for VmRSS and VmHWM.

## Limits

- Two client writes closer together than the reader thread can wake up are read as one, so the render count can only undercount.
- The server's child processes (the shell and the workload) are not counted in CPU or memory; only the `mosh-client` and `mosh-server` processes are.
- With `--rust-dir` pointing at wrapper scripts, the process measured is whatever the wrapper `exec`s; a wrapper that does not `exec` would be measured instead of the client.
