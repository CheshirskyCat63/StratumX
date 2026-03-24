# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| engine_runtime_handle | EngineRuntimeHandle | required | stable handle for one public runtime surface | must remain opaque to L6 and to tooling |
| runtime_surface_kind | RuntimeSurfaceKind | required | declares packet, control, observation, or metric surface class | must use the shared registry enum |
| source_session_handle | EngineSessionHandle | required | identifies the owning engine session | must resolve through `engine_session_handles` |
| capability_set_ref | CompatCapabilitySetId | optional | links the runtime surface to one capability set when capability scoped | must resolve through `compat_capabilities` when present |
| liveness_state | HandleLivenessState | required | declares whether the runtime surface handle is live, draining, or dead | must use the shared registry enum |
