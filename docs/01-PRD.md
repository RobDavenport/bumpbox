# 01-PRD

## Product

**bumpbox**  
Deterministic, no_std fixed-point geometry and collision-query kernel with a small broadphase companion crate and a browser-facing wasm demo.

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
- Additive 3D primitives: AABB, ray, sphere, segment, capsule, triangle.
- Core query helpers for 2D AABB overlap, AABB closest point, point-to-AABB distance squared, segment closest point, point-to-segment distance squared, circle closest point, point-to-circle distance squared, capsule closest point, point-to-capsule distance squared, convex polygon closest point, point-to-convex-polygon distance squared, oriented-box closest point, point-to-oriented-box distance squared, segment-vs-segment intersection, circle-vs-AABB overlap, circle-vs-segment overlap, circle-vs-capsule overlap, circle-vs-polygon overlap, circle-vs-oriented-box overlap, capsule point containment, capsule-vs-segment overlap, capsule-vs-AABB overlap, capsule-vs-capsule overlap, capsule-vs-polygon overlap, capsule-vs-oriented-box overlap, oriented-box point containment, oriented-box-vs-AABB overlap, oriented-box overlap, convex polygon point containment, convex-polygon-vs-AABB overlap, convex-polygon overlap, circle raycasts, capsule raycasts, polygon raycasts, oriented-box raycasts, swept circle-vs-AABB, swept circle-vs-capsule, swept circle-vs-segment, swept circle-vs-circle, swept circle-vs-convex-polygon, swept circle-vs-oriented-box, swept capsule-vs-AABB, swept capsule-vs-circle, swept capsule-vs-segment, swept capsule-vs-capsule, swept capsule-vs-convex-polygon, swept capsule-vs-oriented-box, and AABB raycasts.
- Core query helpers for 3D AABB overlap, AABB closest point, point-to-AABB distance squared, segment closest point, point-to-segment distance squared, sphere closest point, point-to-sphere distance squared, capsule closest point, point-to-capsule distance squared, triangle closest point, point-to-triangle distance squared, segment-vs-AABB overlap, sphere-vs-AABB overlap, sphere-vs-segment overlap, sphere-vs-sphere overlap, sphere-vs-capsule overlap, capsule point containment, capsule-vs-AABB overlap, capsule-vs-capsule overlap, sphere raycasts, capsule raycasts, triangle raycasts, swept segment-vs-AABB3, swept sphere-vs-AABB3, swept sphere-vs-capsule3, swept sphere-vs-segment3, swept sphere-vs-sphere, swept capsule-vs-sphere, swept capsule-vs-AABB3, swept capsule-vs-segment3, swept capsule-vs-capsule3, and AABB raycasts.
- Deterministic uniform-grid broadphase companion crate with stable query ordering and duplicate elimination in 2D and 3D.
- Contract schemas and fixture validation for query cases and grid configuration.
- A browser demo crate that showcases both 2D and 3D queries in wasm, including explicit 2D AABB, round, capsule, polygon, and oriented-box raycast visualization plus 3D AABB, round, and capsule raycast visualization.

## Nice-to-have features

- Additional sweep or TOI pairs beyond the current round-vs-AABB, round-vs-capsule, round-vs-segment, round-vs-round, round-vs-polygon, round-vs-oriented-box, capsule-vs-AABB, capsule-vs-circle, capsule-vs-segment, capsule-vs-capsule, capsule-vs-polygon, capsule-vs-oriented-box, segment3-vs-AABB3, capsule3-vs-sphere, capsule3-vs-AABB3, capsule3-vs-segment3, and capsule3-vs-capsule3 coverage once the narrow-phase semantics harden.
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
- Additive 3D query coverage in `bumpbox-core` and `bumpbox-grid`.
- Browser demo coverage in `demo-wasm` for 2D and 3D scenes, including explicit 2D AABB, round, capsule, polygon, and oriented-box raycasts plus 3D AABB, round, and capsule raycasts.
- Contracts and fixtures that document the initial query surface and grid configuration.
- CI, docs, and agent prompts aligned with the walking skeleton state.

## Success criteria

- Core query tests are deterministic and self-consistent.
- The workspace compiles with `--no-default-features`.
- Grid queries return stable, deduplicated ordering.
- The public API remains small enough for piecemeal adoption.
