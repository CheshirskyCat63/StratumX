# Inspector and Details Model

## Requirements
- single-object details
- multi-object aggregated details
- component/details sections
- advanced property groups
- type-aware widgets
- transactional edits
- reset/revert/default controls
- per-property validation
- diff against template/archetype
- pinned inspectors
- live diagnostics badges

## Inspector law
The inspector never writes directly to truth.
It emits legal edit requests that enter the lower-stack command path.
