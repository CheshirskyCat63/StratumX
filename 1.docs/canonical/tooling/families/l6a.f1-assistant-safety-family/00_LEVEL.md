# Family Level

Canonical family: `assistant_safety_family`

## Composes
- assistant safety, approval, legality, and apply/revert policy composition

## Data responsibility
- authority-facing minimal truth: safety/approval refs only
- snapshot classes: safety snapshots
- index classes: safety lookup indices
- derived classes: derived safety views
- artifact classes: safety evidence artifacts
- preview classes: safety previews are optional
- cache classes: safety caches
- diagnostics classes: safety diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
