# Family Level

Canonical family: `assistant_intelligence_family`

## Composes
- goal understanding, planning, canon reasoning, optimization, migration, and routing composition

## Data responsibility
- authority-facing minimal truth: goal/plan refs only
- snapshot classes: planning snapshots
- index classes: planning lookup indices
- derived classes: derived plan and reasoning views
- artifact classes: deterministic planning/report artifacts when exported
- preview classes: no frame-level preview ownership
- cache classes: planning caches
- diagnostics classes: planning diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
