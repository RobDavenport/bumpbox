# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Goal: verify that the workspace is coherent and then advance the highest-value deterministic geometry slice.

Steps:
1. Read `MASTER_SPEC.md`, `AGENTS.md`, and `docs/02-TECHNICAL-ARCHITECTURE.md`.
2. Run the baseline verification commands.
3. Confirm what is implemented versus scaffold-only in `bumpbox-core` and `bumpbox-grid`.
4. Select one narrow query or broadphase gap and move it forward with TDD.
5. Update docs and taskboard status if any requirement changes.

Verification:
- `cargo test --workspace --all-features`
- `cargo check --workspace --lib --no-default-features`
- `python3 scripts/validate_contract_fixtures.py`
