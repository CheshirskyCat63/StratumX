# Family Level

Canonical family: `scene_family`

## Composes
- scene graph authoring, entity grouping, scene views, and scene manifests

## Data responsibility
- authority-facing minimal truth: scene edit intents and scene root refs
- snapshot classes: scene snapshots
- index classes: scene/spatial lookup indices
- derived classes: derived scene views
- artifact classes: scene artifacts and manifests
- preview classes: scene previews
- cache classes: scene caches
- diagnostics classes: scene diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
