# Dependencies

## Allowed imports
- compat_verdicts
- compat_capabilities
- compat_profiles
- transport_policies
- engine_runtime_handles
- engine_object_handles

## Allowed dependents or consumers
- link_ingress_packets
- link_ingress_controls

## Forbidden imports
- L6 tooling layers
- L7 studio layers
- engine internals beyond public L4 surfaces where not justified
- adjacent semantic classes not named above

## Local dependency law
May depend only on the layers named above. Must not absorb adjacent semantic classes or editor workflows.
