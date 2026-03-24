# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| engine_object_handle | EngineObjectHandle | required | stable object handle exposed by the public L4 boundary | must remain opaque to L6 and to tooling |
| object_kind | EngineObjectKind | required | declares runtime, world, scene, entity, or other closed object class | must use the shared registry enum |
| engine_identity_ref | EngineIdentityRef | optional | links the object handle to one stable identity ref when available | must resolve through `engine_identity_refs` when present |
| source_session_handle | EngineSessionHandle | required | identifies the owning engine session | must resolve through `engine_session_handles` |
| liveness_state | HandleLivenessState | required | declares whether the object handle is live, draining, or dead | must use the shared registry enum |
