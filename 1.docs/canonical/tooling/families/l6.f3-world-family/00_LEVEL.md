# Family Level

Canonical family: `world_family`

## Composes
- world composition, world-part selection, data layers, and world-state views

## Data responsibility
- authority-facing minimal truth: world edit intents and active world refs
- snapshot classes: world snapshots
- index classes: world lookup indices
- derived classes: derived world views
- artifact classes: world artifacts and manifests
- preview classes: world previews
- cache classes: world caches
- diagnostics classes: world diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
