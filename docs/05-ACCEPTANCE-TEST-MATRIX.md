# 05-ACCEPTANCE-TEST-MATRIX

| Requirement | Test(s) / fixture(s) | Owner / module | Seed status |
|---|---|---|---|
| Fixed-point arithmetic skeleton | `crates/bumpbox-core/tests/smoke.rs::fixed_arithmetic_round_trips` | `bumpbox-core` | green |
| AABB overlap semantics | `crates/bumpbox-core/tests/smoke.rs::aabb_overlap_is_symmetric_and_boundary_touch_counts` | `bumpbox-core` | green |
| Closest-point and distance semantics | `crates/bumpbox-core/tests/smoke.rs::closest_point_clamps_each_axis` | `bumpbox-core` | green |
| Deterministic raycast tie-breaking | `crates/bumpbox-core/tests/smoke.rs::raycast_corner_hit_prefers_x_normal_on_tie` | `bumpbox-core` | green |
| Capsule point containment semantics | `crates/bumpbox-core/tests/smoke.rs::capsule_contains_point_counts_endcaps_and_degenerate_segments` | `bumpbox-core` | green |
| Oriented-box point containment semantics | `crates/bumpbox-core/tests/smoke.rs::oriented_box_contains_point_respects_basis_coordinates_and_boundary`, `crates/bumpbox-core/tests/smoke.rs::oriented_box_rejects_parallel_axes` | `bumpbox-core` | green |
| Convex polygon point containment semantics | `crates/bumpbox-core/tests/smoke.rs::convex_polygon_contains_point_is_winding_agnostic_and_boundary_inclusive` | `bumpbox-core` | green |
| Deterministic grid ordering | `crates/bumpbox-grid/tests/grid_smoke.rs::grid_query_is_sorted_and_deduplicated` | `bumpbox-grid` | green |
| 3D AABB overlap semantics | `crates/bumpbox-core/tests/smoke.rs::aabb3_overlap_is_symmetric_and_boundary_touch_counts` | `bumpbox-core` | green |
| 3D closest-point semantics | `crates/bumpbox-core/tests/smoke.rs::closest_point_on_aabb3_clamps_each_axis` | `bumpbox-core` | green |
| 3D sphere-vs-AABB boundary semantics | `crates/bumpbox-core/tests/smoke.rs::sphere_overlaps_aabb3_counts_boundary_touch` | `bumpbox-core` | green |
| 3D deterministic raycast tie-breaking | `crates/bumpbox-core/tests/smoke.rs::raycast_aabb3_corner_hit_prefers_x_normal_on_three_way_tie` | `bumpbox-core` | green |
| Query-case fixtures | `fixtures/contracts/query-cases.valid.json`, `query-cases.invalid.json`, `scripts/validate_contract_fixtures.py` | `contracts/` | green |
| Grid-config fixtures | `fixtures/contracts/grid-config.valid.json`, `grid-config.invalid.json`, `scripts/validate_contract_fixtures.py` | `contracts/` | green |
| 3D broadphase companion | not yet implemented | future broadphase modules | seeded |
