# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| ingress_control_envelope_id | IngressControlEnvelopeId | required | stable identity of one submitted control envelope | must remain unique within one engine session |
| control_kind | IngressControlKind | required | closed control kind enum | must use the shared registry control kind enum |
| target_object_handle | EngineObjectHandle | optional | identifies an addressed engine object when the control is object-scoped | must resolve through `engine_object_handles` when present |
| target_runtime_handle | EngineRuntimeHandle | required | identifies the addressed public runtime surface | must resolve through `engine_runtime_handles` |
| source_session_handle | EngineSessionHandle | required | identifies the source bridge session | must resolve through `engine_session_handles` |
| submission_order_key | SubmissionOrderKey | required | preserves ordered write-side publication | must be monotonic per session and per control domain |
