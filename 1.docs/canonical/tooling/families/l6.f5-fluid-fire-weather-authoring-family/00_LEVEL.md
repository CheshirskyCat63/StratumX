# Family Level

Canonical family: `fluid_fire_weather_authoring_family`

## Composes
- fluid, fire, weather authoring, field rules, and weather cell authoring

## Data responsibility
- authority-facing minimal truth: fluid/fire/weather edit intents
- snapshot classes: fluid/fire/weather snapshots
- index classes: field lookup indices
- derived classes: derived field views
- artifact classes: field artifacts and manifests
- preview classes: field previews
- cache classes: field caches
- diagnostics classes: field diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
