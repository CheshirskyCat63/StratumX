# Family Level

Canonical family: `simulation_meta_family`

## Composes
- simulation campaign composition and simulation workflow meta

## Data responsibility
- authority-facing minimal truth: simulation campaign refs only
- snapshot classes: simulation-campaign snapshots
- index classes: simulation-campaign indices
- derived classes: derived simulation campaign views
- artifact classes: simulation campaign artifacts
- preview classes: no frame-level preview ownership
- cache classes: simulation meta caches
- diagnostics classes: simulation meta diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
