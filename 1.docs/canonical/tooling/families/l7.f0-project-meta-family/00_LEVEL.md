# Family Level

Canonical family: `project_meta_family`

## Composes
- project orchestration, governance, and reporting composition

## Data responsibility
- authority-facing minimal truth: project meta refs only
- snapshot classes: project-orchestration snapshots
- index classes: project-orchestration lookup indices
- derived classes: derived project-meta views
- artifact classes: project governance/reporting artifacts
- preview classes: no frame-level preview ownership
- cache classes: project meta caches
- diagnostics classes: project meta diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
