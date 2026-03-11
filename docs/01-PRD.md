# 01-PRD

## Product

**bumpbox**  
Deterministic, no_std fixed-point geometry and collision-query kernel with a small 2D broadphase companion crate.

## Users

Game engine authors, rollback/lockstep runtime builders, simulation developers, wasm-hosted game teams, and custom-tool authors who need deterministic spatial queries.

## Primary use cases

- Run deterministic 2D and incremental 3D overlap, containment, closest-point, distance, and raycast queries without adopting a full physics engine.
- Use a fixed-point scalar and vector type that keeps correctness-critical paths free of hidden float fallback.
- Build deterministic broadphase candidate sets with stable query ordering.
- Embed the kernel inside wasm guests, native tools, or custom engines with no mandatory allocator.

## Core jobs to be done

- Own a small set of reliable 2D primitives and queries for common game use cases.
- Preserve exact repeatability across targets and builds.
- Adopt only the narrow-phase crate or only the broadphase companion crate as needed.
- Document degenerates, ordering, and boundary semantics explicitly.

## Must-have features

- Walking-skeleton Q16.16 scalar type with deterministic arithmetic semantics.
- 2D primitives: AABB, segment, ray, circle, capsule, oriented box, convex polygon.
- Additive 3D primitives: AABB, ray, sphere.
- Core query helpers for 2D AABB overlap, AABB closest point, point-to-AABB distance squared, circle-vs-AABB overlap, capsule and oriented-box point containment, polygon point containment, and AABB raycasts.
- Core query helpers for 3D AABB overlap, AABB closest point, point-to-AABB distance squared, sphere-vs-AABB overlap, and AABB raycasts.
- Deterministic uniform-grid broadphase companion crate with stable query ordering and duplicate elimination.
- Contract schemas and fixture validation for query cases and grid configuration.

## Nice-to-have features

- Additional sweep or TOI pairs once the narrow-phase semantics harden.
- Alternative math-adapter crate if the internal vector type proves too narrow.
- Optional serde or fixture import or export support after the contracts settle.

## Explicit non-goals

- Full rigid-body physics, constraints, joints, or solver stacks.
- Float fallback inside correctness-critical code paths.
- Mesh-heavy collision or 2D plus 3D simultaneous launch scope.
- Replacing every existing math crate in the ecosystem.

## Launch scope

- Two focused crates: `bumpbox-core` and `bumpbox-grid`.
- One working example plus tests for arithmetic, overlap symmetry, raycast tie-breaking, and grid query ordering.
- Additive 3D query coverage in `bumpbox-core`; broadphase remains 2D-only for now.
- Contracts and fixtures that document the initial query surface and grid configuration.
- CI, docs, and agent prompts aligned with the walking skeleton state.

## Success criteria

- Core query tests are deterministic and self-consistent.
- The workspace compiles with `--no-default-features`.
- Grid queries return stable, deduplicated ordering.
- The public API remains small enough for piecemeal adoption.
