# Document Rules

## Root-doc rules
- every root document is canonical unless it explicitly says otherwise
- every level folder must contain `00_LEVEL.md` and exactly one layer folder
- every layer folder must contain `00_LAYER.md`, `10_LIBRARIES.md`, `20_DEPENDENCIES.md`, `30_COMMUNICATION.md`, `31_THREADING.md`, `32_BOUNDARY_PRESERVATION.md`, and `40_FIELDS.md`

## Drift rules
- this package may not describe a level that does not exist
- this package may not omit a level that does exist
- a local layer doc may narrow authority but may never widen it against the root docs
- constitutions may freeze cross-cutting law but may not invent extra owned data kinds

## Naming rule
- level folder names use `l5.<index>-<slug>`
- layer folder names use the canonical snake_case layer name
