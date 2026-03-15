# bumpbox

Deterministic, `no_std`-compatible fixed-point geometry and collision queries with 2D and 3D broadphase support, plus a browser-facing wasm demo.

**[Live Demo](https://robdavenport.github.io/bumpbox/)** - open the GitHub Pages showcase in your browser.

## Workspace

- `crates/bumpbox-core`: fixed-point scalar, 2D and 3D vectors, shapes, and narrow-phase query helpers
- `crates/bumpbox-grid`: deterministic 2D and 3D uniform-grid broadphase helpers
- `demo-wasm`: browser showcase with interactive 2D and 3D query labs rendered from wasm-managed state, plus deterministic 2D and 3D snapshot exports used by tests and supporting browser assets

## Current surface

- 2D queries: AABB overlap, closest point, point-to-AABB distance squared, segment closest point, point-to-segment distance squared, circle closest point, point-to-circle distance squared, capsule closest point, point-to-capsule distance squared, convex polygon closest point, point-to-convex-polygon distance squared, oriented-box closest point, point-to-oriented-box distance squared, segment-vs-segment, circle-vs-AABB, circle-vs-segment, circle-vs-capsule, circle-vs-polygon, circle-vs-oriented-box, capsule point containment, capsule-vs-segment, capsule-vs-AABB, capsule-vs-capsule, capsule-vs-polygon, capsule-vs-oriented-box, oriented-box point containment, oriented-box-vs-AABB, oriented-box overlap, convex polygon point containment, convex-polygon-vs-AABB, convex-polygon overlap, circle raycasts, capsule raycasts, polygon raycasts, oriented-box raycasts, swept circle-vs-AABB, swept circle-vs-capsule, swept circle-vs-segment, swept circle-vs-circle, swept circle-vs-convex-polygon, swept circle-vs-oriented-box, swept capsule-vs-AABB, swept capsule-vs-circle, swept capsule-vs-segment, swept capsule-vs-capsule, swept capsule-vs-convex-polygon, swept capsule-vs-oriented-box, and AABB raycasts
- 3D queries: AABB overlap, closest point, point-to-AABB distance squared, segment closest point, point-to-segment distance squared, sphere closest point, point-to-sphere distance squared, capsule closest point, point-to-capsule distance squared, triangle closest point, point-to-triangle distance squared, segment-vs-triangle closest-point pairs (`ClosestPoints3` / `closest_points_segment3_triangle3`), segment-vs-triangle distance squared (`distance_squared_segment3_triangle3`), sphere-vs-triangle closest-point pairs (`ClosestPair3` / `closest_points_sphere_triangle3`), sphere-vs-triangle distance squared (`distance_squared_sphere_triangle3`), capsule-vs-triangle closest-point pairs (`ClosestPair3` / `closest_points_capsule3_triangle3`), capsule-vs-triangle distance squared (`distance_squared_capsule3_triangle3`), triangle-vs-triangle closest-point pairs (`ClosestPair3` / `closest_points_triangle3_triangle3`), triangle-vs-triangle distance squared (`distance_squared_triangle3_triangle3`), triangle-vs-AABB closest-point pairs (`ClosestPair3` / `closest_points_triangle3_aabb3`) with pinned face-interior/face-interior semantics, triangle-vs-AABB distance squared (`distance_squared_triangle3_aabb3`), segment-vs-AABB, segment-vs-triangle, sphere-vs-AABB, sphere-vs-segment, sphere-vs-sphere, sphere-vs-triangle, capsule point containment, sphere-vs-capsule, capsule-vs-AABB, capsule-vs-capsule, capsule-vs-triangle, triangle-vs-triangle, triangle-vs-AABB, sphere raycasts, capsule raycasts, triangle raycasts, swept segment-vs-AABB3, swept segment-vs-triangle3, swept triangle-vs-triangle3, swept sphere-vs-AABB3, swept sphere-vs-capsule3, swept sphere-vs-segment3, swept sphere-vs-sphere, swept sphere-vs-triangle3, swept capsule-vs-sphere, swept capsule-vs-triangle3, swept capsule-vs-AABB3, swept capsule-vs-segment3, swept capsule-vs-capsule3, and AABB raycasts
- 2D and 3D deterministic uniform-grid broadphase candidate queries
- `Fx32` backed by the `fixed` crate
- Contract schemas and fixture validation for query cases and grid config

## Validation

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --lib --no-default-features
python3 scripts/validate_contract_fixtures.py
```

## Wasm demo

The deployed demo is available at `https://robdavenport.github.io/bumpbox/`.

Build the wasm package:

```bash
wasm-pack build demo-wasm --target web --release --out-dir www/pkg
```

Serve the site:

```bash
cd demo-wasm/www
python3 -m http.server 8080
```

Open `http://localhost:8080` to use the drag-enabled 2D and 3D labs. The canvas is organized into overlap, containment, raycast, and broadphase lessons, with a dimension switch for 2D/3D and a depth slider for the projected 3D handles so you can move the query actor or target geometry directly and watch the deterministic wasm readout update after every drag.

The GitHub Pages deployment workflow lives in `.github/workflows/pages.yml`.

## Common commands

```bash
make fmt
make lint
make test
make test-no-default
make docs
make wasm-demo
make wasm-demo-serve
```

## Repo workflow

1. Read `MASTER_SPEC.md` and `AGENTS.md`.
2. Use `codex/taskboard.yaml` as the sequencing source of truth.
3. Start with a failing test when changing runtime behavior.
4. Re-run the validation commands before claiming completion.
