# Family Level

Canonical family: `extension_and_automation_family`

## Composes
- automation, scripting, hot reload, plugin host, and package services

## Data responsibility
- authority-facing minimal truth: extension authority refs only
- snapshot classes: extension snapshots
- index classes: extension indices
- derived classes: derived extension health and automation views
- artifact classes: plugin/package artifacts
- preview classes: none required
- cache classes: extension caches only
- diagnostics classes: extension and automation diagnostics
- degradation priority: `medium`

This family composes editor product surfaces without owning hidden lower-stack truth.
