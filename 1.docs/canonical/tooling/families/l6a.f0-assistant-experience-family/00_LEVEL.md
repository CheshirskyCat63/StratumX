# Family Level

Canonical family: `assistant_experience_family`

## Composes
- assistant sessions, evidence pack composition, proposal experience, and assistant UI composition

## Data responsibility
- authority-facing minimal truth: assistant session refs only
- snapshot classes: assistant-runtime snapshots
- index classes: assistant-runtime lookup indices
- derived classes: derived assistant proposal views
- artifact classes: assistant evidence artifacts when deterministic
- preview classes: assistant previews are disposable
- cache classes: assistant caches
- diagnostics classes: assistant diagnostics
- degradation priority: `medium`

This family composes canonical planes and sidecars without owning hidden truth.
