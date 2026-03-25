# Family Level

Canonical family: `content_meta_family`

## Composes
- content campaign composition and content workflow meta

## Data responsibility
- authority-facing minimal truth: content campaign refs only
- snapshot classes: content-campaign snapshots
- index classes: content-campaign indices
- derived classes: derived content campaign views
- artifact classes: content campaign artifacts
- preview classes: no frame-level preview ownership
- cache classes: content meta caches
- diagnostics classes: content meta diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
