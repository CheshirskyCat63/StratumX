# Family Level

Canonical family: `terrain_material_environment_family`

## Composes
- terrain, material, lookdev, weather, and environment suites

## Data responsibility
- authority-facing minimal truth: terrain/material authority refs only
- snapshot classes: terrain/material/environment snapshots
- index classes: terrain/material indices
- derived classes: derived environment authoring views
- artifact classes: terrain/material/environment artifacts
- preview classes: suite previews
- cache classes: suite caches only
- diagnostics classes: terrain/material/environment diagnostics
- degradation priority: `high`

This family composes editor product surfaces without owning hidden lower-stack truth.
