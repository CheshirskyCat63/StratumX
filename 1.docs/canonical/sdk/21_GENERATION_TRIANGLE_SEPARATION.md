# Generation Triangle Separation

## Canonical rule
L5 owns none of the generation triangle:
- generation request
- generation policy
- generated delta or artifact

## Why
Generation is editor/tooling orchestration and must remain above the runtime truth bridge.

## Allowed relation
L5 may gate a concrete boundary action produced by higher layers after generation.
That does not move generation semantics into L5.
