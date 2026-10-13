# mosh-rs

Rust reimplementation of mosh (upstream master commit decd9b7; compatible with mosh 1.4.0). Read `PLAN.md` first. Section numbers below refer to it.

## Rules

- Port from `upstream/` file by file. Keep names, constants, and order of operations. The only allowed differences are listed in PLAN.md section 9.
- Plain functions and structs. No trait without two real implementations. No single-caller abstractions. No async runtime.
- No unit tests of individual functions, no mocks, no test-only abstractions. Tests exercise features end to end (PLAN.md section 4) and live in `tests/`.
- Verify by running the oracle for your module (PLAN.md section 3) and paste the real output in your report.
- When upstream behavior is unclear, run upstream (`ref/` harnesses or the real `mosh-client`/`mosh-server`) and match it. Never guess.
- Never edit `upstream/` or another agent's directory.
- Commit messages are plain. No Co-Authored-By or other attribution lines.
- Every source file starts with the GPLv3 header and names the upstream file it was ported from.

## Layout

- `crates/terminal` emulator and display (upstream `src/terminal`)
- `crates/network` crypto, proto, transport (upstream `src/crypto`, `src/network`, `src/protobufs`)
- `crates/platform` tty, pty, process spawning; the only platform-specific crate
- `crates/mosh` statesync, client, server, launcher; builds `mosh`, `mosh-client`, `mosh-server`
- `ref/` C++ oracles linked against the upstream build
- `tools/` udp-impair, corpus recorder, bench
- `tests/` end-to-end and conformance tests, `tests/run-all.sh` runs everything

## Devbox facts

- Upstream built tree: `~/src/mosh` (symlink `upstream/`). Binaries at `upstream/src/frontend/mosh-client` and `mosh-server`. `make check` works there.
- Real mosh 1.4.0 from apt is on PATH.
- tmux 3.4 is on PATH and is the host-terminal oracle.
- Windows cross target: `x86_64-pc-windows-gnu` with the mingw linker from apt.

## Status, interventions, and GitHub

- Append to STATUS.md at every gate and to INTERVENTIONS.md every time the run needed outside help, was nudged after going idle, or deviated from PLAN.md.
- The remote is origin (private repo github.com/Arunjay4213/mosh-rs). Push to main after every commit. The user reviews progress on GitHub.
- Run `git pull --rebase --autostash` before every push; another machine also pushes to main.
- Commit often, not only at gates: after every agent delivery that compiles, and at least every 30 minutes while agents are working. Prefix work-in-progress commits with "WIP:" and name the module. Push after every commit.
- Agents commit their own work too: each agent commits its owned files whenever its crate builds, with a plain message naming the module.

## Code quality bar (checked at every commit, enforced at every gate)

- The bar is measured with current stable (`cargo fmt`, `cargo clippy`) and must also pass on the minimum supported Rust, `rust-version` in the workspace `Cargo.toml` (1.89): `cargo +1.89.0 clippy --workspace --all-targets -- -D warnings` and `cargo +1.89.0 test --release --workspace --no-run`.
- `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` must pass before any commit. A clippy error is a bug until proven otherwise; fix the code, do not add `#[allow]`.
- `#[allow(...)]` needs a one-line comment on the line above saying why. Blanket allows at module or crate level are forbidden.
- `unsafe` is allowed only in `crates/platform`, and every `unsafe` block has a `// SAFETY:` comment stating the invariant. Other crates carry `#![forbid(unsafe_code)]`.
- `.unwrap()` and `.expect()` in library code only where the invariant is provably local, with the reason in the `expect` message. Errors that can happen at runtime are returned, never panicked.
- Every `pub` item has a doc comment. Every module starts with a doc comment naming the upstream file and what the module does.
- Integer casts between signed and unsigned mirror upstream exactly and get a comment where the value could be negative or exceed the range.
- Independent code reviews land in `reviews/`. Before a gate is declared passed, every item in every `reviews/*.md` file is either fixed or answered in that file, with the commit hash.

## CI feedback (nothing lands on main without green CI)

- Never push to main.
  Main is protected and only moves when CI is green.
- Work on a branch named `agent/<topic>` (the laptop uses `laptop/<topic>`), commit, and push the branch.
- The push opens a PR against main by itself, and CI runs on the pushed commit (about 20 minutes).
- When CI is green, main is fast-forwarded to that exact commit, the PR shows as merged, and the branch is deleted.
- Check whether your commit landed with `scripts/landed.sh <sha>` (exit 0 landed, 1 not yet).
- If CI is red, read the failures with `scripts/ci-status.sh --pr <branch>` (add `--wait 25` to wait for the result), fix on the same branch, and push again.
- If the PR gets the comment "Rebase onto main and push again", main moved while CI ran: run `git pull --rebase origin main` on the branch and push again (`--force-with-lease`).
- `scripts/ci-status.sh <sha>` prints the result for any commit, including commits on main.
- Red CI is fixed before any new work starts, and no gate is declared passed while CI is red.
  Record each CI failure and its fix in INTERVENTIONS.md.

## Keeping the markdown current (stale information is the main agent failure)

- Before starting any task, every agent runs `git pull --rebase --autostash`, then reads: CLAUDE.md, the PLAN.md sections named in its brief, the newest two entries of STATUS.md, all of INTERVENTIONS.md, and every `reviews/*.md` that names its crate. Briefs must list these reads as the first step.
- Every commit that changes behavior updates the matching markdown in the same commit: STATUS.md (what changed, with the measured result), INTERVENTIONS.md (any deviation or outside help), the module `//!` doc, and the README status checkbox when a gate passes.
- A gate is not passed until STATUS.md has a "gate: PASSED" entry with the exact commands run and their numbers (fuzz cases and mismatches, interop cells, test counts). A commit message claiming a pass without that entry is reverted to WIP.
- PLAN.md is the design of record. When reality differs (a dependency replaced, a module split, a constant found wrong), the agent edits PLAN.md in the same commit and logs the change in INTERVENTIONS.md, so the next agent reads the truth.
- The orchestrator re-reads CLAUDE.md, STATUS.md and INTERVENTIONS.md before spawning each agent and includes their current state in the brief.

## Models and effort (updated 2026-10-13, user instruction)

- The orchestrator session runs Opus 5.5 at effort high, with Opus as the advisor.
- Every spawned agent is Opus 5.5 (model "opus") at effort "medium". Only the orchestrator runs at high. Fable agents are not spawned from devbox; Fable reviews are run from the laptop at gates.

## Things devbox cannot do (hand to the laptop via INTERVENTIONS.md "NEEDS THE USER")

- The deploy key cannot push changes under `.github/workflows/`.
  Commit such changes on a local branch named `ci/<topic>` and log "NEEDS THE USER: push branch ci/<topic>" in INTERVENTIONS.md.
  The laptop fetches it from devbox, reviews it, and pushes it as `laptop/<topic>`, which goes through the same PR and CI gate.
- Reference decision (2026-10-13): the upstream reference is mosh master at commit decd9b7 (2026-03-22, 39 commits after the 1.4.0 tag), because that is what the oracles and interop were built against and it carries fixes 1.4.0 lacks. Describe the project everywhere as "ported from mosh master (decd9b7, March 2026), wire-compatible with mosh 1.4.0". Update README, PLAN and module docs accordingly.
