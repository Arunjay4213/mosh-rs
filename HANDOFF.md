# Handoff: running the build on devbox

The orchestrator session runs on devbox inside tmux so it survives the laptop sleeping.
Everything below is already in place on devbox: Claude Code 2.1.296 logged in to your account, model set to Fable, your global CLAUDE.md, the key memory files, real mosh 1.4.0, the built upstream tree, rustup, and the Windows cross-compiler.

## Start the session (you do this once)

```
ssh devbox
tmux new -s mosh          # later: tmux attach -t mosh
cd ~/mosh-rs
claude
```

Inside Claude Code, run `/effort medium`, confirm the status line says Fable, then paste the prompt below. If you want it to run without permission prompts overnight, start it with `claude --dangerously-skip-permissions` and accept the confirmation yourself.
Detach from tmux with Ctrl-b then d. The session keeps running. Re-attach from any machine with `ssh devbox -t tmux attach -t mosh`.
Check `/usage` at each phase gate.

## First prompt

```
Read PLAN.md and CLAUDE.md in full before doing anything else.

You are the orchestrator. You do not write implementation code yourself. You decompose work into agent briefs, spawn Opus 5.5 agents (model "opus", effort "high", as many as justified; write the justification for each spawn in STATUS.md). You may use at most one Fable agent (model "fable", effort "medium") and only for the line-by-line review of the terminal, transport sender and overlay ports, verify each result yourself by rerunning its oracle, and integrate. Micromanage the briefs: exact upstream files to read, exact files to own, the exact oracle command that must pass, and a required report with real command output. Never accept a report without rerunning its oracle. Send corrections back to the same agent with SendMessage instead of accepting partial work.

Start with Phase 0 (PLAN.md section 10), then launch Phase 1 agents A, B, C and H in parallel. Continue through the phases in order. At each gate, write me a plain-English status in this file's directory as STATUS.md: what passed (with the measured numbers), what failed, what is next. Stop and ask me only for the manual Windows lid-close test and before publishing the repo.

Teach as you go: before each new concept (OCB, SSP, forkpty, protobuf unknown fields, differential fuzzing), one short paragraph in plain English on what it is and why it is needed.

Commit on devbox after each gate with a plain message, no attribution lines.
```
