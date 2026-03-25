# Family Level

Canonical family: `world_scene_family`

## Composes
- world, scene, partition, placement, grouping, and residency authoring suites

## Data responsibility
- authority-facing minimal truth: world/scene authority refs only
- snapshot classes: world and scene suite snapshots
- index classes: world/scene indices
- derived classes: derived world authoring views
- artifact classes: world authoring artifacts
- preview classes: world previews
- cache classes: suite caches only
- diagnostics classes: world and scene diagnostics
- degradation priority: `high`

This family composes editor product surfaces without owning hidden lower-stack truth.
