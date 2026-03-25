# Family Level

Canonical family: `world_meta_family`

## Composes
- world campaign composition and world workflow meta

## Data responsibility
- authority-facing minimal truth: world campaign refs only
- snapshot classes: world-campaign snapshots
- index classes: world-campaign indices
- derived classes: derived world campaign views
- artifact classes: world campaign artifacts
- preview classes: no frame-level preview ownership
- cache classes: world meta caches
- diagnostics classes: world meta diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
