# MASTER_SPEC.md

## Product intent

- **Product:** bumpbox
- **Repo slug:** bumpbox
- **Description:** Deterministic, no_std fixed-point geometry and collision-query kernel with a 2D broadphase companion crate, established 2D query surface, and an additive 3D core query skeleton.
- **Users:** Game engine authors, rollback/lockstep runtime builders, simulation developers, wasm-hosted game teams, and custom-tool authors who need deterministic spatial queries.
- **Problem:** Deterministic projects often need geometry and collision queries without hidden floating-point drift, framework lock-in, or a full rigid-body engine. Existing stacks are usually float-first or heavier than custom runtimes want.
- **Core value proposition:** A focused 2D fixed-point geometry and query layer that remains modular, deterministic, and useful even without a full physics engine.

## Architecture summary

- Rust workspace root with `bumpbox-core` and `bumpbox-grid`
- `no_std`-compatible core path
- 2D and 3D value types in the core crate, with the broadphase companion still scoped to 2D
- Contract fixtures and schemas under `contracts/` + `fixtures/`
- Agent execution pack under `codex/`
- CI-ready workflow under `.github/workflows/`

## Constraints

- solo-dev friendly, no_std compatible, deterministic, performance-sensitive, modular, minimal mandatory dependencies, permissive licensing
- Strong typing and deterministic behavior take precedence over convenience APIs.
- The repository must stay small enough for solo maintenance.
- Docs and tests must be specific to the product, not generic templates.

## Milestone shape

1. Workspace + tooling baseline
2. Core math and narrow-phase walking skeleton
3. Broadphase and contract validation
4. Examples + acceptance tests
5. CI + release hygiene

## Acceptance criteria

- The workspace is coherent and ready for agent-driven continuation.
- Core crates contain real code and real tests.
- Every must-have requirement is mapped in `docs/05-ACCEPTANCE-TEST-MATRIX.md`.
- Commands in the README and CI are aligned.
- The current state is honestly labeled as scaffolded, partially implemented, or complete.

## Implementation priorities

1. Preserve exact repeatability and explicit policies
2. Keep the API small and composable
3. Lock query semantics before widening shape support
4. Expand from proven tests, not speculative abstractions

## Risks

- Scope creep into full physics or giant math surface
- Ambiguous boundary and degenerate-input semantics
- Overdesign before the walking skeleton proves the value

## Open assumptions

- Rust stable remains the default toolchain for contributors.
- Python 3 is available in CI and local development environments.
- The first release optimizes for clarity and determinism over feature breadth.
