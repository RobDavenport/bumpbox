# 06-RISK-REGISTER

| Risk | Impact | Mitigation / descoping plan |
|---|---|---|
| Scope expansion into full physics | Trying to solve collision, dynamics, constraints, and 3D at once would stall the project. | Keep the current scope on 2D geometry plus queries plus one broadphase. Put dynamics out of scope in docs and taskboard. |
| Numerical edge-case ambiguity | Different assumptions around boundary-touching, zero-length inputs, or overflow can make the crate unreliable. | Document policies early and back them with explicit tests and fixture cases. |
| Math-surface bloat | A giant math API would dilute the main value proposition and slow maintenance. | Keep the internal vector type minimal and add adapters only when real use cases justify them. |
| Broadphase complexity growth | Dynamic update needs can become a separate product if introduced too early. | Keep the current grid focused on deterministic insertion and query semantics; revisit richer updates after the narrow-phase stabilizes. |
