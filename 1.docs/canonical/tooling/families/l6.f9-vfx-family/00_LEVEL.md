# Family Level

Canonical family: `vfx_family`

## Composes
- VFX authoring, effect graphs, effect views, and VFX manifests

## Data responsibility
- authority-facing minimal truth: VFX edit intents
- snapshot classes: VFX snapshots
- index classes: VFX lookup indices
- derived classes: derived VFX views
- artifact classes: VFX artifacts and manifests
- preview classes: VFX previews
- cache classes: VFX caches
- diagnostics classes: VFX diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
