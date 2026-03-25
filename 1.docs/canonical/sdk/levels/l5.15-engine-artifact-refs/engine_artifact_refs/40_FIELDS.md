# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| engine_artifact_ref | EngineArtifactRef | required | stable ref to one exported engine artifact record | must remain opaque to L6 and to tooling |
| artifact_kind | EngineArtifactKind | required | declares mesh, image, acoustic, trace, capture, or other closed artifact class | must use the shared registry enum |
| source_runtime_handle | EngineRuntimeHandle | required | identifies the export surface that resolved the artifact ref | must resolve through `engine_runtime_handles` |
| engine_identity_ref | EngineIdentityRef | optional | links the artifact ref to one stable identity when available | must resolve through `engine_identity_refs` when present |
| production_anchor_ref | EgressObservationEnvelopeId | optional | anchors the exported artifact ref to one observed runtime emission when available | anchor only; must not become build ownership |
| artifact_epoch | ArtifactEpoch | required | declares the production or publication epoch of the artifact ref | must be monotonic inside one source runtime handle |
| artifact_stability_class | ArtifactStabilityClass | required | declares transient, retained, or frozen artifact posture | must use the shared registry enum |
