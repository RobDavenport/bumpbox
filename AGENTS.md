# AGENTS.md

## Repo working rules

- Read `MASTER_SPEC.md`, then the relevant file in `docs/`, before changing code.
- Treat the taskboard as the source of truth for sequencing.
- Preserve existing interfaces unless a failing test or spec update justifies a change.
- Prefer additive, reversible changes over broad rewrites.
- Keep commits logically scoped by work package.

## Architecture invariants

- The repo must remain a Rust workspace.
- Core crates must stay `no_std` compatible.
- No hidden floating-point fallback may appear in correctness-critical code.
- Deterministic ordering and degenerate-input behavior must be test-backed.
- Avoid framework-shaped scene or world abstractions in the core crates.

## TDD expectations

- Default loop: **Red -> Green -> Refactor**.
- Start with the narrowest failing test that proves the requirement.
- Do not add production code without a test or contract that justifies it.
- Prefer table-driven tests for edge cases and invariant tests for numerical semantics.
- When a bug is fixed, add a regression test before or alongside the fix.

## Forbidden shortcuts

- No `todo!()`, `unimplemented!()`, or fake happy-path shims in shipped code.
- No silent float conversions in the narrow-phase or broadphase core.
- No “ordering is unspecified” escape hatches for query or broadphase results.
- No expanding scope into full rigid-body physics without spec and milestone changes.

## File ownership / subsystem boundaries

- `crates/bumpbox-core`: fixed-point scalar, vector, shapes, and narrow-phase queries
- `crates/bumpbox-grid`: deterministic uniform-grid broadphase
- `contracts/`: externalized contracts and schemas
- `fixtures/`: valid and invalid examples
- `docs/`: product and engineering intent
- `codex/`: agent runbook, prompts, and execution tracking
- `scripts/`: deterministic helper scripts only

## Validation checklist before claiming completion

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo check --workspace --lib --no-default-features`
- `python3 scripts/validate_contract_fixtures.py`
- Update acceptance matrix and taskboard status where relevant

## How to resume from partial repo state

- Inspect `codex/taskboard.yaml` for the latest seeded status.
- Inspect existing tests before touching implementation files.
- Continue from the highest-priority item that is not `done`.
- Preserve partial progress; do not delete incomplete but coherent work unless a spec mismatch requires replacement.
