# Dependencies

## Allowed imports
- engine_session_handles

## Allowed dependents or consumers
- all boundary layers
- compat_capabilities
- legality_gates
- L6 runtime readers

## Forbidden imports
- L6 tooling layers
- L7 studio layers
- engine internals beyond public L4 surfaces where not justified
- adjacent semantic classes not named above

## Local dependency law
May depend only on the layers named above. Must not absorb adjacent semantic classes or editor workflows.
