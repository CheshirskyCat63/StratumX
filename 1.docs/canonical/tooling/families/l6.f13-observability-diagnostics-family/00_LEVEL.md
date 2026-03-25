# Family Level

Canonical family: `observability_diagnostics_family`

## Composes
- observability views, logs, probes, metrics dashboards, and diagnostics routing

## Data responsibility
- authority-facing minimal truth: no domain authority beyond observability-session refs
- snapshot classes: observability snapshots
- index classes: observability lookup indices
- derived classes: derived observability views
- artifact classes: diagnostics artifacts and manifests when deterministic
- preview classes: diagnostic previews
- cache classes: observability caches
- diagnostics classes: observability diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
