# Ref Subtypes

## Closed ref subtypes used in this package
- policy ref
- session handle ref
- object handle ref
- runtime handle ref
- identity ref
- state ref

## Per-layer subtype assignments
- `transport_policies` -> policy ref
- `engine_session_handles` -> session handle ref
- `engine_object_handles` -> object handle ref
- `engine_runtime_handles` -> runtime handle ref
- `engine_identity_refs` -> identity ref
- `engine_state_refs` -> state ref

## Opaque-upward rule
All handles and refs in L5 are opaque upward contracts.
L6 may store, compare, and route them, but may not derive hidden internal structure from them.
