# Prompt instructions

- Inspect the current repo state before editing anything.
- Read the relevant spec docs and existing tests first.
- Continue from partial progress; do not redo completed work.
- Keep TDD discipline: create or update a failing test first.
- Preserve deterministic behavior and no_std constraints.
- End with concrete verification steps and any docs/taskboard updates required.

Focus: public API clarity and broadphase layering.

Actions:
- Keep `bumpbox-core` narrow and math-first.
- Extend `bumpbox-grid` only where ordering and storage semantics stay explicit.
- Do not add a global scene or framework-shaped API.

Verification:
- `cargo test -p bumpbox-grid --all-features`
- README/example review for API clarity
