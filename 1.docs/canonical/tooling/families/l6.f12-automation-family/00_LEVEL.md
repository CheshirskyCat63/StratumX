# Family Level

Canonical family: `automation_family`

## Composes
- automation editor views, automation specs, and automation diagnostics

## Data responsibility
- authority-facing minimal truth: automation edit intents
- snapshot classes: automation snapshots
- index classes: automation lookup indices
- derived classes: derived automation views
- artifact classes: automation artifacts and manifests
- preview classes: automation previews
- cache classes: automation caches
- diagnostics classes: automation diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
