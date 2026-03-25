# Family Level

Canonical family: `workspace_shell_family`

## Composes
- workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing

## Data responsibility
- authority-facing minimal truth: workspace session ownership only
- snapshot classes: workspace snapshots and session-scoped view snapshots
- index classes: workspace/session lookup indices
- derived classes: derived workspace panels and inspector views
- artifact classes: deterministic workspace layout artifacts when exported
- preview classes: workspace previews are disposable
- cache classes: workspace caches only
- diagnostics classes: workspace diagnostics and layout diagnostics
- degradation priority: `low`

This family composes canonical planes and sidecars without owning hidden truth.
