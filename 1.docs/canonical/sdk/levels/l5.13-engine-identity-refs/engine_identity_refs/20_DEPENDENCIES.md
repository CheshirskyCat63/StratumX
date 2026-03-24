# Dependencies

## Allowed imports
- engine_runtime_handles

## Allowed dependents or consumers
- engine_object_handles
- engine_state_refs
- L6 runtime readers

## Forbidden imports
- L6 tooling layers
- L7 studio layers
- engine internals beyond public L4 surfaces where not justified
- adjacent semantic classes not named above

## Local dependency law
May depend only on the layers named above. Must not absorb adjacent semantic classes or editor workflows.
