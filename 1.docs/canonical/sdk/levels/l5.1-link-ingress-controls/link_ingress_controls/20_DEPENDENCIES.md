# Dependencies

## Allowed imports
- compat_versions
- compat_capabilities
- transport_policies
- legality_gates
- engine_session_handles
- engine_runtime_handles
- engine_object_handles

## Allowed dependents or consumers
- public L4 control surfaces

## Forbidden imports
- L6 tooling layers
- L7 studio layers
- engine internals beyond public L4 surfaces where not justified
- adjacent semantic classes not named above

## Local dependency law
May depend only on the layers named above. Must not absorb adjacent semantic classes or editor workflows.
