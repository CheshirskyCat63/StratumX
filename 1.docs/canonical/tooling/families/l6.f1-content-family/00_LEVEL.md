# Family Level

Canonical family: `content_family`

## Composes
- content browser, content refs, content authoring views, and content manifests

## Data responsibility
- authority-facing minimal truth: minimal content author refs and edit intents
- snapshot classes: content snapshots
- index classes: content lookup indices
- derived classes: content-derived listings and filtered views
- artifact classes: content manifests and compile-ready content artifacts
- preview classes: content previews
- cache classes: content caches
- diagnostics classes: content diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
