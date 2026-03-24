# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| engine_state_ref | EngineStateRef | required | stable ref to one exported engine state record | must remain opaque to L6 and to tooling |
| state_kind | EngineStateKind | required | declares runtime, world, scene, entity, subsystem, or other closed state class | must use the shared registry enum |
| source_runtime_handle | EngineRuntimeHandle | required | identifies the export surface that resolved the state ref | must resolve through `engine_runtime_handles` |
| engine_identity_ref | EngineIdentityRef | optional | links the state ref to one stable identity when available | must resolve through `engine_identity_refs` when present |
| observation_anchor_ref | EgressObservationEnvelopeId | optional | anchors the exported state ref to one observed runtime emission | anchor only; must not become projection state |
| freshness_class | StateFreshnessClass | required | declares realtime, recent, or coarse freshness | must use the shared registry enum |
