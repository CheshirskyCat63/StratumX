# Family Level

Canonical family: `simulation_debug_family`

## Composes
- simulation debug views, probes, timeline views, and diagnostic overlays

## Data responsibility
- authority-facing minimal truth: no domain authority beyond debug-session refs
- snapshot classes: simulation debug snapshots
- index classes: simulation debug indices
- derived classes: derived simulation debug views
- artifact classes: debug/export artifacts when deterministic
- preview classes: simulation previews
- cache classes: debug caches
- diagnostics classes: simulation diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
