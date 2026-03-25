# Family Level

Canonical family: `editor_shell_family`

## Composes
- shell composition, command bar, view hosting, docking, and editor frame routing

## Data responsibility
- authority-facing minimal truth: minimal shell authority refs only
- snapshot classes: shell snapshots and panel/view composition snapshots
- index classes: panel/view lookup indices
- derived classes: derived shell layouts and command routing views
- artifact classes: theme/style and editor-frame artifacts when deterministic
- preview classes: shell previews are optional and disposable
- cache classes: shell/UI caches only
- diagnostics classes: shell diagnostics and host diagnostics
- degradation priority: `low`

This family composes canonical planes and sidecars without owning hidden truth.
