# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| engine_identity_ref | EngineIdentityRef | required | stable ref to one exported engine identity | must remain opaque to L6 and to tooling |
| identity_kind | EngineIdentityKind | required | declares runtime, world, scene, entity, subsystem, or other closed identity class | must use the shared registry enum |
| stable_engine_identity_id | StableEngineIdentityId | required | stable externalized identity id for bridge use | must not encode editor workflow state |
| source_runtime_handle | EngineRuntimeHandle | required | identifies the export surface that resolved the identity | must resolve through `engine_runtime_handles` |
| resolver_scope | ResolverScope | required | declares the identity resolution scope | must use the shared registry enum |
