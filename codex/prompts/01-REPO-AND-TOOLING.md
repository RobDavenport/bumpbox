# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: keep the repo bootstrap sharp and low-friction.

Actions:
- Inspect Makefile, CI, and root metadata for drift.
- Keep local and CI commands identical where possible.
- Avoid adding toolchain dependencies that do not materially improve deterministic validation.

Verification:
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
