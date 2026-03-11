# 03-WBS-AND-MILESTONES

| Work package | Dependencies | Deliverables | Done criteria |
|---|---|---|---|
| WP-01 repo bootstrap | none | Workspace root, docs set, CI workflow, taskboard, contract validator | Top-level commands and file layout are coherent and documented |
| WP-02 fixed scalar + vector | WP-01 | Q16.16 scalar, Vec2, unit tests for arithmetic and ordering | Core math primitives exist with repeatable tests |
| WP-03 shapes + narrow-phase | WP-02 | AABB, circle, ray, and other shape definitions plus query helpers | Overlap, closest-point, distance, and raycast tests pass |
| WP-04 broadphase companion | WP-03 | Uniform-grid crate with deterministic query ordering | Grid tests pass and duplicate elimination is verified |
| WP-05 contracts + fixtures | WP-03, WP-04 | Schemas, valid and invalid fixtures, validator script | Fixture validation passes |
| WP-06 docs + release hygiene | WP-01..WP-05 | README alignment, acceptance matrix, CI command parity | Docs and automation agree on real repo state |
