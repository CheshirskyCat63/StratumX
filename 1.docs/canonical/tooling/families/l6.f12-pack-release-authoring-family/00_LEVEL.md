# Family Level

Canonical family: `pack_release_authoring_family`

## Composes
- packaging/release authoring, package specs, and release metadata authoring

## Data responsibility
- authority-facing minimal truth: pack/release edit intents
- snapshot classes: pack/release snapshots
- index classes: package lookup indices
- derived classes: derived package views
- artifact classes: package artifacts and manifests
- preview classes: package previews
- cache classes: package caches
- diagnostics classes: package diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
