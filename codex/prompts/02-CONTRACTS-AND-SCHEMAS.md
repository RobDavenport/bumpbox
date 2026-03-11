# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: harden contracts for query cases and grid configuration.

Actions:
- Review current schemas and the validator script.
- Add valid and invalid fixtures for every new boundary condition.
- Keep schemas narrow and version-conscious.

Verification:
- `python3 scripts/validate_contract_fixtures.py`
- Update `docs/05-ACCEPTANCE-TEST-MATRIX.md` if coverage changes
