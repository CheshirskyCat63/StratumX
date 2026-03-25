# Family Level

Canonical family: `audio_family`

## Composes
- audio authoring, buses, routing views, and audio manifests

## Data responsibility
- authority-facing minimal truth: audio edit intents
- snapshot classes: audio snapshots
- index classes: audio lookup indices
- derived classes: derived audio views
- artifact classes: audio artifacts and manifests
- preview classes: audio previews
- cache classes: audio caches
- diagnostics classes: audio diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
