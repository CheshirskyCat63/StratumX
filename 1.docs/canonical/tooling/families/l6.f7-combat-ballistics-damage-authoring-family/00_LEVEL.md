# Family Level

Canonical family: `combat_ballistics_damage_authoring_family`

## Composes
- combat, ballistics, and damage authoring graphs and tuning rules

## Data responsibility
- authority-facing minimal truth: combat/ballistics edit intents
- snapshot classes: combat snapshots
- index classes: combat lookup indices
- derived classes: derived combat views
- artifact classes: combat artifacts and manifests
- preview classes: combat previews
- cache classes: combat caches
- diagnostics classes: combat diagnostics
- degradation priority: `high`

This family composes canonical planes and sidecars without owning hidden truth.
