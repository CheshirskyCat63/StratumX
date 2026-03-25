# Family Level

Canonical family: `browser_and_inspector_family`

## Composes
- content browser, outliner, world browser, details, and inspector surfaces

## Data responsibility
- authority-facing minimal truth: selection and browser authority refs only
- snapshot classes: browser and inspector snapshots
- index classes: search/filter and selection indices
- derived classes: derived property and dependency views
- artifact classes: thumbnail artifacts and exportable layout artifacts
- preview classes: preview panes only
- cache classes: thumbnail and search caches
- diagnostics classes: browser and inspector diagnostics
- degradation priority: `medium`

This family composes editor product surfaces without owning hidden lower-stack truth.
