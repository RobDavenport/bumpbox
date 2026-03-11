# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: deterministic invariants and regression coverage.

Actions:
- Add edge-case tests for degenerate inputs, tie-breaking, and ordering.
- Add regression tests for any numerical or ordering bug fixed in this run.
- Prefer deterministic table-driven tests over broad speculative abstractions.

Verification:
- `cargo test --workspace --all-features`
- `python3 scripts/validate_contract_fixtures.py`
