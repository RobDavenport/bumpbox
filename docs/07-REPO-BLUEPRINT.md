# 07-REPO-BLUEPRINT

## Full repo tree

```text
bumpbox
├── Cargo.toml
├── Makefile
├── README.md
├── MASTER_SPEC.md
├── crates/
│   ├── bumpbox-core/
│   │   ├── src/
│   │   ├── tests/
│   │   └── examples/
│   └── bumpbox-grid/
│       ├── src/
│       └── tests/
├── demo-wasm/
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/
│   └── www/
├── contracts/
├── fixtures/
├── docs/
├── codex/
└── scripts/
```

## Top-level directory purposes

- `crates/`: production Rust workspace members
- `demo-wasm/`: browser showcase crate and static site assets
- `contracts/`: versioned external schemas and boundary documentation
- `fixtures/`: valid and invalid contract examples
- `docs/`: product, architecture, quality, and delivery guidance
- `codex/`: agent runbook, prompts, and task tracking
- `scripts/`: deterministic developer and CI helper scripts
- `.github/workflows/`: CI definitions

## Naming conventions

- Workspace members use the product prefix (`bumpbox-*`) to keep ownership obvious.
- Demo-facing crates keep the `*-demo-wasm` suffix and place browser assets in `www/`.
- Contracts use kebab-case file names ending in `.schema.json`.
- Prompts are numbered so agents can resume from partial progress without re-planning the whole repo.

## Future extension points

- Add new crates only when they own a stable boundary.
- Keep examples, demos, and fixtures aligned with real acceptance cases.
- Prefer sibling crates for optional tooling instead of bloating the core crate.
