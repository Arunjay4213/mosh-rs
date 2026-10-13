# tools

Non-shipped tooling, built as the `mosh-tools` package:

- `udp-impair`: UDP proxy with loss, delay, jitter, reordering, blackouts, and client-address change, steered over a control socket.
- `pty-record`: the session corpus recorder; `corpus/record-all.sh` records every `corpus/*.script` into `tests/corpus/sessions/`.
- `bench`: the performance bench, described in `bench/README.md`.

All three are Unix-only; on Windows they print that and exit 1.
See PLAN.md sections 2, 3 and 5.
The static release build is `scripts/build-release.sh`.
