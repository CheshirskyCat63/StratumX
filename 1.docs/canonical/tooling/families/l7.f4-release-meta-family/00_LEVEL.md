# Family Level

Canonical family: `release_meta_family`

## Composes
- release campaign composition, release governance, and release reporting

## Data responsibility
- authority-facing minimal truth: release campaign refs only
- snapshot classes: release-campaign snapshots
- index classes: release-campaign indices
- derived classes: derived release campaign views
- artifact classes: release campaign artifacts
- preview classes: no frame-level preview ownership
- cache classes: release meta caches
- diagnostics classes: release meta diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
