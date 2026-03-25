# Family Level

Canonical family: `assistant_diagnostics_build_family`

## Composes
- assistant dock, diagnostics surfaces, build/release surfaces, and task monitors

## Data responsibility
- authority-facing minimal truth: assistant/build authority refs only
- snapshot classes: assistant and diagnostics snapshots
- index classes: task and artifact indices
- derived classes: derived proposal/release views
- artifact classes: task and release report artifacts
- preview classes: preview task views
- cache classes: diagnostics caches
- diagnostics classes: assistant/build/release diagnostics
- degradation priority: `medium`

This family composes editor product surfaces without owning hidden lower-stack truth.
