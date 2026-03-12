# 02-TECHNICAL-ARCHITECTURE

## System decomposition

### Core crate

`bumpbox-core` owns the fixed-point scalar, minimal vector types, 2D and additive 3D primitive definitions, and core query helpers. It stays intentionally narrow.

### Broadphase crate

`bumpbox-grid` depends on `bumpbox-core` and provides deterministic 2D and 3D uniform-grid candidate generators with stable result ordering.

### Demo crate

`demo-wasm` depends on `bumpbox-core` and `bumpbox-grid` and exposes browser-friendly deterministic scene snapshots through `wasm-bindgen`. Static assets in `demo-wasm/www/` render those snapshots on canvas for 2D and 3D showcase scenarios with explicit 2D AABB, round, capsule, polygon, and oriented-box raycast overlays plus 3D AABB, round, and capsule raycast overlays.

### Boundaries

The public API stays type-first. No global world, registry, or physics scene is required. Users own higher-level storage and simulation policy.

### Data flow

User constructs 2D or 3D shapes -> calls narrow-phase query helpers or 2D/3D grid insertion/query methods -> receives deterministic plain-data results.
The demo path runs deterministic scene builders in wasm -> serializes snapshot state -> renders it in browser canvas views with per-target raycast hit markers and normals.

### Contracts

JSON schemas describe query-case fixtures and grid configuration. The walking skeleton validates fixtures with a deterministic Python helper script.

### Storage strategy

Core math and shapes are plain value types. The grid stores fixed-capacity 2D or 3D cell contents in arrays and deduplicates query results deterministically.

### Security and performance

No hidden I/O or thread usage. No float fallback in the walking skeleton. Broadphase ordering is explicit and tested.

### Rationale for stack choice

Rust stable plus no external runtime deps keeps the core portable, auditable, and easy to bootstrap. The scaffold favors exactness and clarity over breadth.
