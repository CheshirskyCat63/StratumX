# Family Level

Canonical family: `render_lookdev_authoring_family`

## Composes
- render lookdev authoring, lighting/look rules, and render previews

## Data responsibility
- authority-facing minimal truth: lookdev edit intents
- snapshot classes: lookdev snapshots
- index classes: lookdev lookup indices
- derived classes: derived lookdev views
- artifact classes: lookdev artifacts and manifests
- preview classes: lookdev previews
- cache classes: lookdev caches
- diagnostics classes: lookdev diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
