# Family Level

Canonical family: `terrain_deformation_authoring_family`

## Composes
- terrain layers, deformation intents, brush results, and terrain manifests

## Data responsibility
- authority-facing minimal truth: terrain edit intents only
- snapshot classes: terrain snapshots
- index classes: terrain indices
- derived classes: terrain-derived overlays
- artifact classes: terrain artifacts and manifests
- preview classes: terrain previews
- cache classes: terrain caches
- diagnostics classes: terrain diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
