## What this changes

<!-- One or two sentences. Name the upstream file(s) ported, if any. -->

## Oracle output

<!-- Paste the oracle command(s) you ran and their real output (PLAN.md section 3). -->

```text

```

## Checklist

- [ ] Ported code names its upstream file and keeps upstream names, constants, and order of operations.
- [ ] No behavior differs from upstream except deviations already listed in PLAN.md section 9.
- [ ] The relevant oracle was run and its command and output are pasted above.
- [ ] `cargo fmt --all --check` and `cargo clippy --all-targets -- -D warnings` pass.
- [ ] `cargo test --release --workspace` and `tests/run-all.sh` pass.
- [ ] No new dependency, or the reason for it is written here.
- [ ] No unit tests of single functions, mocks, or test-only abstractions.
- [ ] CHANGELOG.md updated under Unreleased for user-visible changes.
- [ ] Commit messages are plain, with no attribution trailers.
