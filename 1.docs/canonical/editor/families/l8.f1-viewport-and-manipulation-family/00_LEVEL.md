# Family Level

Canonical family: `viewport_and_manipulation_family`

## Composes
- viewport hosts, camera controls, gizmos, overlays, and manipulation channels

## Data responsibility
- authority-facing minimal truth: viewport authority refs only
- snapshot classes: viewport snapshots and visualization states
- index classes: view mode and overlay indices
- derived classes: derived culling/overlay views
- artifact classes: deterministic overlay assets when needed
- preview classes: disposable visual previews
- cache classes: viewport UI caches only
- diagnostics classes: viewport and manipulation diagnostics
- degradation priority: `medium`

This family composes editor product surfaces without owning hidden lower-stack truth.
