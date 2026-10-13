# Handoff: running the build on devbox

This file is historical: it records how the devbox orchestrator session was first started on 2026-10-13, and the first prompt below is kept as it was given.
Several settings have changed since, so to restart the session today follow "Restarting today" and the current rules in CLAUDE.md, not the original steps.

The orchestrator session runs on devbox inside tmux so it survives the laptop sleeping.
Everything below is already in place on devbox: Claude Code logged in to your account, your global CLAUDE.md, the key memory files, real mosh 1.4.0, the built upstream tree, rustup, and the Windows cross-compiler.

## Restarting today

- Model: Opus 5.5 (`"model": "opus"`) at effort high, with Fable as the advisor (`"advisorModel": "fable"`); these are set in `~/.claude/settings.json` on devbox.
  Spawned agents are Opus 5.5 at effort medium (CLAUDE.md, "Models and effort").
- The original "model set to Fable" and `/effort medium` steps below are no longer what runs.
- main is protected: nothing is pushed to main.
  Work goes on `agent/<topic>` branches and lands only through the PR and green CI (CLAUDE.md, "CI feedback").
  The original prompt's "Commit on devbox after each gate" now means commit and push the branch.
- The session runs a watchdog with `/loop 20m`.
  Its text as first given ended with "After every commit, git push origin main.", which main's branch protection now refuses.
  When restarting, use this text instead:

```text
/loop 20m Watchdog check. Re-read CLAUDE.md. If the plan in PLAN.md is not finished and you are not already mid-task, continue with the next step. If you had stopped and were waiting on nothing the user must decide, append an entry to INTERVENTIONS.md saying what you stopped on and why, then continue. If you are stopped at one of the two user decisions (Windows lid-close test, publishing the repo), do nothing. After every commit, push the agent/<topic> branch (never main) and check it with scripts/ci-status.sh.
```

## Start the session (as done on 2026-10-13)

```text
ssh devbox
tmux new -s mosh          # later: tmux attach -t mosh
cd ~/mosh-rs
claude
```

Inside Claude Code, run `/effort medium`, confirm the status line says Fable, then paste the prompt below. If you want it to run without permission prompts overnight, start it with `claude --dangerously-skip-permissions` and accept the confirmation yourself.
Detach from tmux with Ctrl-b then d. The session keeps running. Re-attach from any machine with `ssh devbox -t tmux attach -t mosh`.
Check `/usage` at each phase gate.

## First prompt (as given on 2026-10-13)

```text
Read PLAN.md and CLAUDE.md in full before doing anything else.

You are the orchestrator. You do not write implementation code yourself. You decompose work into agent briefs, spawn Opus 5.5 agents (model "opus", effort "medium", as many as justified; write the justification for each spawn in STATUS.md). You may use at most one Fable agent (model "fable", effort "medium") and only for the line-by-line review of the terminal, transport sender and overlay ports, verify each result yourself by rerunning its oracle, and integrate. Micromanage the briefs: exact upstream files to read, exact files to own, the exact oracle command that must pass, and a required report with real command output. Never accept a report without rerunning its oracle. Send corrections back to the same agent with SendMessage instead of accepting partial work.

Start with Phase 0 (PLAN.md section 10), then launch Phase 1 agents A, B, C and H in parallel. Continue through the phases in order. At each gate, write me a plain-English status in this file's directory as STATUS.md: what passed (with the measured numbers), what failed, what is next. Stop and ask me only for the manual Windows lid-close test and before publishing the repo.

Teach as you go: before each new concept (OCB, SSP, forkpty, protobuf unknown fields, differential fuzzing), one short paragraph in plain English on what it is and why it is needed.

Commit on devbox after each gate with a plain message, no attribution lines.
```
