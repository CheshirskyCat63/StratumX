# Family Level

Canonical family: `shell_and_view_host_family`

## Composes
- shell composition, window chrome, dock host, tab host, and saved workspaces

## Data responsibility
- authority-facing minimal truth: minimal shell authority refs only
- snapshot classes: shell and view composition snapshots
- index classes: view lookup indices
- derived classes: derived layouts and route views
- artifact classes: theme assets and deterministic UI assets when needed
- preview classes: optional shell previews only
- cache classes: shell caches only
- diagnostics classes: shell and host diagnostics
- degradation priority: `low`

This family composes editor product surfaces without owning hidden lower-stack truth.
