# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: CI and release hygiene.

Actions:
- Confirm CI still reflects the real workspace and no-default-features story.
- Keep release metadata honest about scaffold maturity.
- Ensure docs and command surfaces stay in sync.

Verification:
- Review `.github/workflows/ci.yml`
- `make ci`
