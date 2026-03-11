# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: core fixed-point math and narrow-phase queries.

Actions:
- Inspect existing tests in `crates/bumpbox-core/tests/`.
- Add one failing test for the next math or query invariant.
- Implement the minimal production change required to satisfy it.
- Refactor only after all related tests pass.

Verification:
- `cargo test -p bumpbox-core --all-features`
- `cargo check -p bumpbox-core --lib --no-default-features`
