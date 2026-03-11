# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: documentation coherence after code changes.

Actions:
- Update query semantics, rounding policy notes, and taskboard status where needed.
- Keep incomplete features clearly labeled.
- Ensure acceptance matrix, README, MASTER_SPEC, and docs agree.

Verification:
- Manual audit of README, MASTER_SPEC, docs, and taskboard
- Re-run `make ci`
