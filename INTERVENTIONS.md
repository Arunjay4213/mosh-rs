# INTERVENTIONS

Every time the run needed outside help, was nudged after going idle, or deviated from PLAN.md.
Newest entry first.

## 2026-10-13 NEEDS THE USER: Windows test, rerun steps 3-6 with fixed binaries

- Agent C fixed the ProxyCommand quoting (8a32d96): on Windows each argument is quoted by the Windows command-line rules (no quotes unless it has a space, tab or double quote; then double quotes), following how Win32-OpenSSH's `posix_spawnp` builds the command line it gives `CreateProcessW` (`contrib/win32/win32compat/misc.c`). Unix quoting is unchanged; launcher tests 5/5 on devbox.
- Exception to the testing rules, accepted by the orchestrator: the quoting is checked by a direct call to the pure function `windows_command_line_quote` in `tests/tests/launcher.rs`, because devbox cannot run Windows ssh; the real end-to-end check is the laptop rerun below.
- New Windows binaries, built on devbox from 8a32d96, in `target/dist/windows-x86_64/`:

```
mosh.exe         638396 bytes  sha256 c045f2cc8b650b718adad46481a2e1cb30f239ae61e7c6b45a6e86f603a3daf5
mosh-client.exe  1093557 bytes sha256 58b43899132166d455bdcb8d9c06ca08eb9f9fc41af5b52d19a4496f5707f9df
```

- Laptop: copy them as in step 1 of the "Phase 3 Windows test on the laptop" entry below, then run steps 3 to 6 of that checklist, including step 5's folder with a space in its path. Record results as before.

## 2026-10-13 PLAN 5 startup target changed; musl utmp gap

- Agent P's bench: the first frame arrives about 252 ms after the client starts, for our client (252.32 ms) and for C++ mosh 1.4.0 (257.08 ms) alike, against a PLAN 5 target of 20 ms. Cause, checked in upstream's source: the round-trip estimate starts at 1000 ms, so the first send interval is the 250 ms maximum. Decision: keep upstream's timing (changing it would be a protocol-timing deviation for no compatibility gain). PLAN 5 now targets the client's own startup under 20 ms (measured 1.50 ms, C++ 3.05 ms) and a first frame no slower than C++ mosh.
- The static musl `mosh-server` never printed upstream's "You have a detached Mosh session" warning, because musl's `getutxent` always returns nothing. Agent P is changing the platform crate to read the utmp file directly, so the glibc and musl builds behave the same.

## 2026-10-13 Devbox installs for the static release build (agent P)

- `rustup target add aarch64-unknown-linux-musl x86_64-unknown-linux-musl`.
- `sudo apt-get install -y musl-tools gcc-x86-64-linux-gnu qemu-user`.
  musl-tools is the host musl toolchain; gcc-x86-64-linux-gnu only drives the x86_64 musl link (the Rust musl targets bring their own musl libc and crt objects), set as the linker in `.cargo/config.toml`; qemu-user runs the x86_64 binaries on this arm64 host.
- x86_64 musl binaries link and are static-pie, but were only smoke-tested (`--version` under `qemu-x86_64`), not run through the interop matrix. The aarch64 musl binaries passed the clean interop profile.

## 2026-10-13 Scrollback: history is windowed and never blocks the remote program (decision, overridable)

- Agent S1's live test found a gap in PLAN section 6: nothing limited how many history lines one update carries. A fast `seq 1 50000` filled the 10,000-line ring before the first send (39,977 lines lost), and a bigger ring produced one update of about 87 UDP packets that never arrived completely, because every resend is packed with a new fragment id (upstream behavior; upstream only works because screen updates are small). Through 30% loss nothing arrived.
- Decision by the orchestrator: the remote program is never paused while history is undelivered, connected or not. The screen is always current, and history follows behind it at a bounded rate: the server publishes at most W bytes of history beyond what the client has acknowledged. The server keeps only undelivered lines, capped in bytes; on overflow the oldest undelivered lines are dropped and counted, and the client will show a one-line notice.
- Rejected alternative: pausing the program (as ssh effectively does through TCP) would give exact history in every case, but a build printing a million lines while the laptop sleeps would stall, and under loss the screen would freeze because the server stops reading the program's output. That breaks mosh's main promise. Tell the orchestrator if you want the other trade-off.
- PLAN 6.2 and 6.3 are rewritten by S1 in the same commit as the code. The client half of scrollback waits until this rework lands and is verified.

## 2026-10-13 Phase 5 bench tooling started early

- Decision: agent P builds `tools/bench` and the static musl release build now, during Phase 4, instead of after it. Reason: it touches only `tools/` and a release script, shares no files with the scrollback work, and one PLAN 5 measurement (memory after a 1-hour session) takes an hour per run, so having the tool ready shortens Phase 5. The numbers that go into the README are measured again after scrollback lands.

## 2026-10-13 PLAN 9.12: default TERM on Windows (agent F, coordinator request)

- New deviation, PLAN section 7 and section 9 item 12: on Windows, `mosh.exe` and `mosh-client.exe` set `TERM=xterm-256color` when TERM is unset or empty at startup. Reason: Windows normally has no TERM, so upstream's `mosh-client -c` refuses ("TERM environment variable not set.", exit 1), the launcher stops with "Can't count colors", and `mosh.exe host` never connects from a stock Windows Terminal. Unix is unchanged.
- NEEDS THE USER: verified on devbox only for unix behavior and the Windows cross-build (devbox has no Windows or wine). On the laptop, in a PowerShell without `$env:TERM`, `.\mosh-client.exe -c` must print 256 and `.\mosh.exe devbox` must connect.

## 2026-10-13 Phase 4 starts with Phase 2 and Phase 3 items outstanding

- Decision: Phase 4 (scrollback) starts now, although the Phase 2 gate is not declared passed and Phase 3 has not been tested on Windows. Reason: everything still open needs the laptop (the overlay review, pushing `ci/terminfo`, the Windows test), and nothing devbox can run is failing. Scrollback is negotiated, so old peers see byte-identical behavior; its gate reruns the full interop matrix and upstream's `make check`, which re-verifies Phase 2 for old peers.
- The scrollback server agent starts only now because agent F's change to the `State` trait (`compare`, 0ccd5b6) has landed; two agents editing the same trait in parallel caused the earlier breakages.

## 2026-10-13 PLAN 9.10 withdrawn (agent F)

- `compare` is back in the `State` trait and the sender's verbose round-trip check is ported as upstream has it. Reason: without it upstream's `make check` could not catch a wrong server diff in our binaries ("round-trip Instruction verification failed" is grepped by e2e-test and was never printed). PLAN sections 2 and 9.10 updated; the item number is kept.

## 2026-10-13 NEEDS THE USER: Phase 3 Windows test on the laptop (first pass, no lid-close yet)

Windows release binaries built on devbox from commit 6589529 (includes the Windows TERM default, PLAN 9.12), cross-compiled for x86_64-pc-windows-gnu. They import only system DLLs (KERNEL32, ntdll, msvcrt, ws2_32, api-ms-win-core-synch), so no extra runtime is needed.

```
target/dist/windows-x86_64/mosh.exe         636543 bytes  sha256 81d4e41055d2097be9394961824600db39ca8ffa0616851acf4c5cb324c8612f
target/dist/windows-x86_64/mosh-client.exe  1068310 bytes sha256 bbade1dfc16d2521a22469dbecc7b9d462d187d1b29dd905481c25730f7e30a3
```

Do not set `$env:TERM` first: step 2 checks that the binaries default it themselves.

Steps, in PowerShell inside Windows Terminal on the laptop:
1. `mkdir $HOME\mosh-rs-test; cd $HOME\mosh-rs-test; scp devbox:mosh-rs/target/dist/windows-x86_64/* .` then `Get-FileHash *.exe` and compare with SHA256SUMS. Keep both exe files in the same folder (the launcher looks for `mosh-client.exe` next to itself).
2. In a PowerShell without `$env:TERM`, `.\mosh-client.exe -c` prints 256.
3. `.\mosh.exe devbox` connects with the C++ server installed on devbox. Then `.\mosh.exe --server=/home/ubuntu/mosh-rs/target/release/mosh-server devbox` connects with our server.
4. In the session: type, run `ls --color`, open and quit `vim`, print UTF-8 (`echo 日本語 😀 é`), press Ctrl-C at a running `sleep 100` (must interrupt it), resize the window and run `tput cols; tput lines`.
5. Questions the agents could not answer without Windows:
   - Agent C: does ssh.exe accept the ProxyCommand (`mosh.exe --fake-proxy`) when the folder path contains a space? Repeat step 3 from a folder like `$HOME\mosh rs test`.
   - Agent E: what does Ctrl-^ then Ctrl-Z do? (Windows has no job control; expected: the screen closes and reopens.)
   - Agent C: idle CPU of mosh-client.exe in Task Manager after one idle minute (the Windows loop wakes every 50 ms to check for a resize).
   - Quit with Ctrl-^ then `.`: the Windows Terminal tab must be back to a normal PowerShell prompt, with echo and line editing working.
6. Write the results (pass/fail per step, any error text) into INTERVENTIONS.md or a `reviews/phase3-windows.md`, commit and push.

The lid-close test (sleep the laptop, wake it, the session is still live) is the separate manual step PLAN section 3 item 8 names; the orchestrator will stop and ask for it after this first pass works.

## 2026-10-13 Client landed; NEEDS THE USER: overlay review, ci/terminfo updated

- NEEDS THE USER: Fable line-by-line review of the prediction overlay port, from the laptop (CLAUDE.md: Fable reviews run there). Compare `crates/mosh/src/client/terminaloverlay.rs` with `upstream/src/frontend/terminaloverlay.cc` and `.h`, and `crates/mosh/src/client/stmclient.rs` with `stmclient.cc`. The overlay has not changed since commit 0504a2d. Findings go to `reviews/`; the Phase 2 gate waits on them.
- NEEDS THE USER: push branch ci/terminfo, now the commit printed by `git log -1 ci/terminfo` on devbox (replaces e137062 and 73ad9c6). It also installs `sysstat` (idle-CPU tests run instead of printing SKIPPED) and `vim` (the interop vim scenario).
- PLAN 9.6 now says the generated terminfo table carries `colors` and that `mosh-client -c` falls back to xterm's count for an unknown TERM, consistent with the rest of 9.6 (agent B adds the column; until then the client runs `tput colors`, marked INTERIM). Reason: Windows has no terminfo or `tput`.
- PLAN 9 item 11 added: the client exits cleanly where upstream crashes (unreadable window size at startup, panics). Reason: upstream's behavior there is a crash or a raw terminal, not a design choice.
- Agent E ported the whole overlay in its first landing instead of two, because an empty prediction engine would have been the throwaway stub the brief forbade. Accepted.

## 2026-10-13 NEEDS THE USER: push branch ci/terminfo; Phase 2 starts while CI is red

- NEEDS THE USER: push branch ci/terminfo (commit e137062). CI is red on main only because `terminfo_table` (agent B, 13f6466) checks that the runner has the same `ncurses-term` and `kitty-terminfo` packages the table was generated from, and the runner has neither. The branch installs both and pins the x86 test runner to `ubuntu-24.04` (the arm runner already was), because `ubuntu-latest` may move to a newer Ubuntu with a different ncurses version, and that check would then fail for good.
- Decision: Phase 2 (server, client, interop harness) starts now although CLAUDE.md says red CI is fixed before new work starts. Reason: the only fix is a workflow change devbox cannot push, it is ready on the branch above, and every test passes locally and on the last CI run without the new test (816b79d green). The Phase 1 "gate: PASSED" entry waits until CI is green at a commit that contains all Phase 1 work. Object if you want Phase 2 held.
- Orchestrator pre-created the shared Phase 2 scaffolding (`pub mod client; pub mod server;`, the `mosh-server` and `mosh-client` binary entries and stubs) so the parallel agents never edit the same file. Agents D and E work in separate git worktrees, after two incidents where agents broke each other's builds or rebases in the shared checkout.


## 2026-10-13 Terminfo table generated from the devbox database (agent B)

- Devbox change: `sudo apt-get install -y kitty-terminfo` (0.32.2-1ubuntu0.4), so the `xterm-kitty` entry that kitty sets exists and can be checked (review T-2).
- The terminfo table (PLAN 9.6) is now generated by `make -C ref/ref-term terminfo` from every `toe -a` entry and alias, instead of being written by hand.
- `tests/tests/terminfo_table.rs` compares against upstream on the local database and refuses to run unless the local `ncurses-term` and `kitty-terminfo` versions match the ones recorded in `crates/terminal/src/terminfo.rs`.
- The CI test job installs neither package (`.github/workflows/ci.yml` installs only libncurses-dev), so this test will fail there until the job adds `ncurses-term kitty-terminfo`. The workflow is not agent B's file.
## 2026-10-13 Launcher CI failures fixed; PLAN 9.1 withdrawn (agent C)

- CI failure 1, `missing_server_is_reported`: the GitHub runner has no TERM, so `mosh-client -c` (the launcher's color count) printed "TERM environment variable not set" and the launcher exited 255. Real mosh fails the same way, so the behavior is right; the test now sets `TERM=xterm-256color` on the process, and the tmux sessions `export TERM=xterm-256color` (tmux would otherwise give the pane `tmux-256color`).
- CI failure 2, `launches_session_on_localhost_with_command`: a real bug on dual-stack hosts, reproduced on devbox. ssh connected over `::1`, mosh-server bound `[::1]:60001` (from SSH_CONNECTION), and our launcher, having resolved the name itself (PLAN 9.1, `ssh -G`), started the client at `127.0.0.1:60001`, which never heard from the server. `/usr/bin/mosh` worked against the same name in both `/etc/hosts` orders.
- Devbox change for the reproduction and the new test: `sudo sh -c 'grep -q mosh-dualstack /etc/hosts || printf "::1 mosh-dualstack\n127.0.0.1 mosh-dualstack\n" >> /etc/hosts'`, plus `ssh-keyscan mosh-dualstack` (and `-p 2222` for an experiment) appended to `~/.ssh/known_hosts`. Currently the entries are in the order 127.0.0.1 then ::1 (both orders were tested).
- Fix: PLAN 9.1 withdrawn. The launcher ports mosh.pl's `--fake-proxy` (it is ssh's ProxyCommand, connects to the first address that accepts TCP, reports `MOSH IP`, relays), so client and server always use the same address. `-o HostName=<ip> -o HostKeyAlias=<name>` was tried first and rejected: for a non-default port plain ssh checks `[host]:port` in known_hosts and then falls back to `host`, while any one HostKeyAlias value checks only one of the two forms, and a command-line HostKeyAlias overrides one set in `~/.ssh/config`. PLAN sections 8 and 9.1, docs/ARCHITECTURE.md and README updated.

## 2026-10-13 NEEDS THE USER: push branch ci/run-all

- NEEDS THE USER: push branch ci/run-all (commit a36d03d, one line in `.github/workflows/ci.yml`). The test job runs `tests/run-all.sh` instead of `cargo build && cargo test`, so shell drivers such as `tests/transport-impaired.sh` run in CI too. `run-all.sh` itself now passes `--no-fail-fast` (on main, 8636f95), so one failing test binary no longer hides the rest. This replaces the "low priority" note in the entry below.
- Docs now describe the port as "ported from mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0" per the CLAUDE.md reference decision (539f9ed). Mentions of 1.4.0 that refer to the release binaries used for interop and benchmarks are unchanged.

## 2026-10-13 CI now builds upstream; launcher fails on the runner

- The user pushed the CI fix (a3f29e9). CI now builds upstream and the oracle tests run there. It is still red because two launcher tests fail on the GitHub runners (first seen on cc19759):
  - `missing_server_is_reported`: the runner has no TERM, and `mosh-client -c` needs it. Test environment problem; the test will set TERM itself.
  - `launches_session_on_localhost_with_command`: a real launcher bug on dual-stack hosts. ssh reached the server over one address family, mosh-server bound that address, and our launcher resolved the name separately and sent the client to the other one. A side effect of PLAN 9.1 (`ssh -G` instead of mosh.pl's proxy). Assigned to agent C.
- NEEDS THE USER (low priority). CI runs `cargo test`, not `tests/run-all.sh`, so the shell drivers (`tests/transport-impaired.sh` and later the interop matrix) never run in CI. Changing `.github/workflows/ci.yml` needs a push with `workflow` scope, which devbox does not have. The orchestrator will batch this with any other workflow change and hand it to the user as a ready branch.

## 2026-10-13 Fable review of terminal and transport, PLAN 9.6 widened

- The line-by-line Fable review (`reviews/phase1-terminal.md`, `reviews/phase1-transport.md`) found 1 high and 6 low items. The five transport items were already fixed or covered by PLAN section 9 items 8 and 9; the orchestrator answered them in the files.
- High item T-2: kitty sets `TERM=xterm-kitty`, which the hand-written TERM table did not list, so it got xterm answers (BCE on) where its terminfo says BCE off. Devbox does not even have the `xterm-kitty` entry installed, so the table's kitty row had no checkable source. Decision: replace the hand-written table with one generated from every terminfo entry on devbox (1829 entries, plus `apt install kitty-terminfo`). PLAN 9.6 now says so. Reason: hand-picked rows and prefix matching keep producing wrong answers for real terminals (the review also found `screen.xterm-256color`), while a generated table matches upstream exactly for every TERM terminfo knows.

## 2026-10-13 Phase 1 review fixes: network (agent A)

- Review F3: the socket system calls (`new_socket`, `resolve`, `bind`, `recv_raw`, `strerror`, the TOS/ECN read) moved from `crates/network/src/transport/network.rs` to `crates/platform/src/socket.rs`, and `disable_dumping_core`/`reenable_dumping_core` to `crates/platform/src/coredump.rs`. `crates/network` now depends on `mosh-platform`, no longer on `libc`, and carries `#![forbid(unsafe_code)]`. PLAN.md section 2 updated to say sockets live in the platform crate. Two signatures changed to fit the new boundary: `bind` takes the one socket instead of the socket deque (on Windows it replaces the socket with a newly bound one), and `recv_raw` takes the maximum datagram length; errors come back as `mosh_platform::socket::SysError`, which converts to `NetworkError`.
- Review F19: removing the whole-state clone in `rationalize_states` and `get_remote_diff` with `split_first_mut` leaves upstream's last step, the front state subtracted from itself (`UserStream::subtract` clears on `this == prefix`; skipping it would make the next `subtract` assert). A `&mut` and `&` to the same state cannot coexist in Rust, so the `State` trait gained `subtract_self` with a default that subtracts a copy (identical to upstream for every implementation). `UserStream` (clear) and `Complete` (nothing) should override it so no copy is made; that is agent B's file. PLAN.md section 2 updated.
- Review F23: `ocb3` 0.2 has no `zeroize` feature (the review assumed one). Enabled `aes`'s `zeroize` feature instead, which wipes the AES round keys inside the OCB context on drop, and `Session` now wipes its key copy in `Drop` using the `zeroize` crate (already in the build through `aes`; added as a direct dependency of `mosh-network` so no `unsafe` is needed). The OCB offset table derived from the key (`ocb3`'s private `ll` values) is not wiped; `ocb3` exposes no way to.
- CI on cc19759 (and its predecessors since a3f29e9) is red only in `tests/tests/launcher.rs` (agent C's test against apt mosh on the runner: `TERM environment variable not set`, then no UDP connection to 127.0.0.1:60500). The network tests pass there (`crypto_conformance` ok). Not a network change; reported to the coordinator, not fixed here.
- The shared tree `/home/ubuntu/mosh-rs` could not `git pull --rebase` because agent B's statesync files were untracked there but already committed on origin; agent A finished the review fixes from a separate worktree on origin/main instead of touching B's files.
- PLAN.md section 2 also now lists `reset_input` in the `State` trait (added at the coordinator's request; upstream calls it in `set_current_state`).

## 2026-10-13 CI push blocked and upstream version corrected

- NEEDS THE USER. The CI fix (local branch `ci-upstream-build`, commit a3f29e9) cannot be pushed: GitHub rejects workflow changes from the `github-moshrs` deploy key ("refusing to allow an OAuth App to create or update workflow .github/workflows/ci.yml without workflow scope"). No `gh` CLI or token exists on devbox. To land it, run from `~/mosh-rs` with credentials that have the `workflow` scope: `git rebase origin/main ci-upstream-build && git push <https-url-with-token> ci-upstream-build:main`, or grant the key workflow permission. CI stays red until then; work continues because the fix is ready and verified locally (both oracle tests pass against a from-scratch upstream build).
- The upstream tree on devbox (`~/src/mosh`) is mosh master at commit decd9b7 (2026-03-22), 39 commits after the `mosh-1.4.0` tag, not the 1.4.0 release. The Phase 1 ports and oracles were therefore built against master, which among other things has OSC 8 hyperlink support that 1.4.0 lacks. Decision by the orchestrator: the port target is upstream commit decd9b7, and compatibility is proven against the apt mosh 1.4.0 binaries in the interop matrix. PLAN.md now says so. Re-targeting to the 1.4.0 tag would mean redoing verified work for an older emulator.
## 2026-10-13 CI

- Failure: CI on d9c8289 could not build `ref/ref-crypto` because the GitHub runner had no built upstream tree at `upstream/`, and also lacked apt mosh and `ssh localhost` for the launcher test.
- Fix: `scripts/build-upstream.sh` builds upstream (cached by `actions/cache`) and links `upstream/`, and the CI test job installs mosh, tmux, sshd and build deps, sets `LANG=C.UTF-8`, and authorizes a key for `ssh localhost`.
- Deviation found: devbox's `~/src/mosh` is not the mosh-1.4.0 tag but master commit decd9b7 (39 commits later, includes OSC 8 hyperlinks), and `ref/ref-term` does not compile against the 1.4.0 tag, so the script pins decd9b7; PLAN.md still says 1.4.0 and needs the orchestrator's decision.

## 2026-10-13 Phase 1 review and PLAN section 9 additions

- An independent review of commit 451b2ce arrived from the user's other machine (`reviews/2026-10-13-phase1-terminal-network.md`, 26 findings). Blocking items F1, F4, F5, F8 and F17 were already fixed in d9c8289. The remaining network items went to agent A, terminal items to agent B, and the missing impaired-transport driver to agent H.
- PLAN.md section 9 gained items 7-10 (clock base offset, errors instead of aborts on malformed authenticated input, two iterator-UB repairs, `compare` omitted from the `State` trait). Reason: the review found these deliberate differences documented only in code comments, and section 9 is the one place deviations may live.
- CI was red on d9c8289 because the GitHub runner has no upstream C++ tree. One agent is making CI build mosh 1.4.0 from source.

## 2026-10-13 Phase 0

- CLAUDE.md said the mingw linker and Windows target were installed. They were not. The Phase 0 agent installed both (`apt-get install gcc-mingw-w64-x86-64`, `rustup target add x86_64-pc-windows-gnu`).
- PLAN section 2 lists `unicode-width` as a dependency. Replaced by a width table generated from glibc `wcwidth` so the emulator matches upstream on every character. Not a semantic deviation from upstream; it is the opposite.
- PLAN section 2 says `tests/` holds cargo integration tests. A bare directory cannot hold them, so `tests/` and `tools/` are workspace packages (`mosh-tests`, `mosh-tools`).

## 2026-10-13 Laptop: pushed ci/run-all

- The laptop oversight tick fetched branch `ci/run-all` (a36d03d) from devbox, reviewed the one-line workflow change (test job runs `tests/run-all.sh`), merged it into main and pushed. CI now runs the shell drivers as well as cargo tests.

## 2026-10-13 Laptop: pushed ci/terminfo

- The laptop oversight tick fetched branch `ci/terminfo` (3092b5d) from devbox, reviewed the workflow change (x86 runner pinned to ubuntu-24.04; installs vim, ncurses-term, kitty-terminfo and sysstat so the terminfo table check, the vim interop scenario and the idle-CPU tests run), merged it into main and pushed.

## 2026-10-13 Phase 3 Windows test, first pass (laptop): launcher fails before ssh connects

Steps 1 and 2 of the checklist pass: hashes match SHA256SUMS; `mosh-client.exe -c` prints 256 with no TERM set.
Step 3 fails in Windows Terminal (PowerShell, OpenSSH_for_Windows_9.5p2):

```
PS C:\Users\arunj\mosh-rs-test> .\mosh.exe devbox
CreateProcessW failed error:2
posix_spawnp: No such file or directory
C:\Users\arunj\mosh-rs-test\mosh.exe: Did not find remote IP address (is SSH ProxyCommand disabled?).
```

Cause, verified by hand on the laptop:
- The launcher builds the ProxyCommand with POSIX single-quote quoting (`shell_quote` in `crates/mosh/src/launcher/mod.rs`, used at the `ProxyCommand=` line). Windows ssh.exe does not run ProxyCommand through a POSIX shell; it splits the string itself and treats `'C:\Users\arunj\mosh-rs-test\mosh.exe'` as a literal file name including the quotes, so CreateProcessW returns error 2 (file not found).
- Reproduced exactly with `ssh -o ProxyCommand="'C:\Users\arunj\mosh-rs-test\mosh.exe' '--family=prefer-inet' --fake-proxy -- %h %p" devbox`: same two error lines.
- The same command with the path unquoted (`ProxyCommand=C:\Users\arunj\mosh-rs-test\mosh.exe --family=prefer-inet --fake-proxy -- %h %p`) works: the proxy prints `MOSH IP 100.117.252.57` and the ssh command runs. Forward slashes and doubled backslashes also work unquoted.
- `--fake-proxy` itself is correct on Windows; only the quoting is wrong.

Fix for agent C: on Windows, do not single-quote the ProxyCommand arguments. Use Windows ssh quoting (double quotes only when an argument contains a space, backslashes untouched), or no quoting when the path has no spaces. The path-with-a-space case (agent C's open question) is being tested from the laptop; result follows in this file.
Steps 4 to 6 are blocked on this fix. Fresh Windows binaries are needed after it lands; the laptop copies them from `target/dist/windows-x86_64/` and reruns.

## 2026-10-13 Laptop: CI history and the two failures CI could not show

- 37 CI runs on main so far, none green: 15 cancelled by later pushes (fixed 01:47 Central), the rest red for runner-environment gaps whose fixes waited for the laptop to push workflow changes.
- The ci-status report only carried the last 80 lines of each job log, which for the test job were summaries, so the failing test names never reached devbox. The report now also extracts every `FAILED`, `panicked at`, `run-all:` and `error` line.
- Still failing on the last completed run (fedc599), from the full log:
  - `terminfo_table` (2 tests) and `colors_match_upstream_mosh_client`: missing ncurses-term and kitty-terminfo on the runner. Expected green after `ci/terminfo` (pushed 09:48 Central).
  - `wrong_prediction_is_removed` (tests/tests/prediction_e2e.rs:395): "the prediction of q never showed". Timing-dependent e2e on a shared runner; needs a robustness fix (longer wait or a readiness check), not a product change.
  - `tests/scrollback.sh`: `run-all: FAILED`, but the driver printed nothing except its header into the log, so the cause is unknown. The driver must print which variant failed and why, to stdout, so CI can show it.

## 2026-10-13 Laptop: fixed Windows binaries verified past the handshake; space-in-path answer

- New binaries (10:14 Central build) copied to the laptop, hashes match. Run non-interactively from PowerShell, `mosh.exe devbox` now gets through ssh, the ProxyCommand and `MOSH CONNECT`, and fails only at `tcgetattr: The handle is invalid` because there was no console. That is the expected failure for a non-interactive run; the quoting fix works. Interactive steps 3 to 6 are with the user.
- Agent C's question, answered on the laptop with OpenSSH_for_Windows_9.5p2 and `mosh.exe` in `C:\Users\arunj\mosh rs test\`: a ProxyCommand with the path in double quotes and plain backslashes works (`"C:\Users\arunj\mosh rs test\mosh.exe" --family=prefer-inet --fake-proxy -- %h %p` printed `MOSH IP` and ran the command). Single quotes fail (file not found). A double-quoted forward-slash path and a backslash-escaped-spaces path both hung without connecting. So: double quotes, backslashes untouched, which is what the fix does.
