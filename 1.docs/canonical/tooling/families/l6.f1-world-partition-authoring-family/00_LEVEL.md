# Family Level

Canonical family: `world_partition_authoring_family`

## Composes
- world partitions, cells, regions, partition manifests, and streaming-unit authoring

## Data responsibility
- authority-facing minimal truth: partition author intents and partition rule refs
- snapshot classes: partition snapshots and region/cell snapshots
- index classes: region/cell lookup indices
- derived classes: derived partition views and authoring overlays
- artifact classes: partition manifests and streaming-unit artifacts
- preview classes: partition previews and cell previews
- cache classes: partition caches
- diagnostics classes: partition diagnostics and seam diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
