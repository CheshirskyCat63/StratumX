# Family Level

Canonical family: `performance_governance_authoring_family`

## Composes
- performance governance authoring, budget policies, and degradation policy authoring

## Data responsibility
- authority-facing minimal truth: budget/governance edit intents
- snapshot classes: budget/governance snapshots
- index classes: policy lookup indices
- derived classes: derived governance views
- artifact classes: governance artifacts and manifests
- preview classes: governance previews
- cache classes: governance caches
- diagnostics classes: governance diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
