# Family Level

Canonical family: `collaboration_and_production_family`

## Composes
- collaboration sessions, review, approvals, playtests, dashboards, and onboarding

## Data responsibility
- authority-facing minimal truth: session and review refs only
- snapshot classes: collaboration snapshots
- index classes: review and trace indices
- derived classes: derived dashboard and annotation views
- artifact classes: reports, captures, and review artifacts
- preview classes: preview captures only
- cache classes: bounded collaboration caches
- diagnostics classes: collaboration and production diagnostics
- degradation priority: `low`

This family composes editor product surfaces without owning hidden lower-stack truth.
