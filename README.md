    # bumpbox

    Deterministic, no_std fixed-point 2D geometry and collision-query kernel with a small broadphase companion crate.

    ## Project purpose

    Deterministic projects often need geometry and collision queries without hidden floating-point drift, framework lock-in, or a full rigid-body engine. Existing stacks are usually float-first or heavier than custom runtimes want.

    bumpbox exists to provide a focused 2D fixed-point geometry and query layer that remains modular, deterministic, and useful even without a full physics engine.

    ## Users

    Game engine authors, rollback/lockstep runtime builders, simulation developers, wasm-hosted game teams, and custom-tool authors who need deterministic spatial queries.

    ## Delivery mode

    **scaffold + walking skeleton**

    This repository is intentionally narrow. It ships a compileable workspace, a working skeleton in code, boundary contracts, starter tests, CI wiring, and an agent execution pack. It does **not** claim the full product is complete.

    ## Repo layout

    ```text
    bumpbox-ready-monorepo
├── .editorconfig
├── .github/
│   └── workflows/
│       └── ci.yml
├── .gitignore
├── AGENTS.md
├── Cargo.toml
├── LICENSE
├── MASTER_SPEC.md
├── Makefile
├── README.md
├── clippy.toml
├── codex/
│   ├── 00-OVERNIGHT-RUNBOOK.md
│   ├── ENVIRONMENT-NOTES.md
│   ├── prompts/
│   │   ├── 00-LAUNCH-THIS-REPO.md
│   │   ├── 01-REPO-AND-TOOLING.md
│   │   ├── 02-CONTRACTS-AND-SCHEMAS.md
│   │   ├── 03-CORE-DOMAIN.md
│   │   ├── 04-APIS-OR-PLUGIN-LAYER.md
│   │   ├── 05-TESTS-AND-VALIDATION.md
│   │   ├── 06-CI-LINT-AND-RELEASE.md
│   │   └── 07-DOCS-FINAL-AUDIT.md
│   └── taskboard.yaml
├── contracts/
│   ├── grid-config.schema.json
│   └── query-cases.schema.json
├── crates/
│   ├── bumpbox-core/
│   │   ├── Cargo.toml
│   │   ├── examples/
│   │   │   └── queries.rs
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── queries.rs
│   │   │   ├── scalar.rs
│   │   │   ├── shapes.rs
│   │   │   └── vec2.rs
│   │   └── tests/
│   │       └── smoke.rs
│   └── bumpbox-grid/
│       ├── Cargo.toml
│       ├── src/
│       │   └── lib.rs
│       └── tests/
│           └── grid_smoke.rs
├── docs/
│   ├── 01-PRD.md
│   ├── 02-TECHNICAL-ARCHITECTURE.md
│   ├── 03-WBS-AND-MILESTONES.md
│   ├── 04-TDD-QUALITY-GATES.md
│   ├── 05-ACCEPTANCE-TEST-MATRIX.md
│   ├── 06-RISK-REGISTER.md
│   └── 07-REPO-BLUEPRINT.md
├── fixtures/
│   └── contracts/
│       ├── grid-config.invalid.json
│       ├── grid-config.valid.json
│       ├── query-cases.invalid.json
│       └── query-cases.valid.json
├── rust-toolchain.toml
├── rustfmt.toml
└── scripts/
    └── validate_contract_fixtures.py
    ```

    ## Prerequisites

    - Rust stable toolchain with `clippy` and `rustfmt`
    - Python 3.11+ for contract-fixture validation scripts
    - Standard POSIX shell environment for local automation

    ## Setup commands

    ```bash
    git clone <your-fork-url> bumpbox
    cd bumpbox
    make bootstrap
    make test
    ```

    ## Common commands

    ```bash
    make fmt
    make lint
    make test
    make test-no-default
    make docs
    ```

    ## Development workflow

    1. Pick the next open item from `codex/taskboard.yaml`.
    2. Write or extend a failing test first.
    3. Implement the smallest change that turns the test green.
    4. Refactor only after the behavior is locked by tests.
    5. Re-run `make ci` before claiming completion.
    6. Update the docs pack and taskboard when scope or status changes.

    ## How an agent should start

    1. Read `MASTER_SPEC.md`.
    2. Read `AGENTS.md`.
    3. Open `codex/00-OVERNIGHT-RUNBOOK.md`.
    4. Execute `codex/prompts/00-LAUNCH-THIS-REPO.md`.
    5. Continue through the numbered prompt pack without redoing finished work.

    ## Preferred stack

    Rust stable workspace, no required third-party runtime deps, and an explicit Q16.16 fixed-point scalar for the walking skeleton.

    ## What is scaffolded vs implemented

    Implemented now: workspace scaffold, `bumpbox-core`, `bumpbox-grid`, a working Q16.16 scalar type, core 2D primitives, overlap/closest-point/raycast helpers, deterministic grid query ordering, contract schemas, fixtures, CI, and starter tests.

    Partially implemented: capsule, OBB, and convex-polygon query expansion; more sweep/TOI coverage; and alternative math adapters.

    ## Next milestones

    1. Harden the fixed-point edge-case policy and expand degenerate-input tests.
    2. Add more narrow-phase queries for capsule, OBB, and polygon pairs.
    3. Expand the broadphase from build-per-frame usage into richer update flows.
    4. Decide whether backend abstraction over other fixed-point types is worth the complexity.
