# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| engine_session_handle | EngineSessionHandle | required | stable session handle exposed by the public L4 boundary | must remain opaque to L6 and to tooling |
| runtime_instance_handle | RuntimeInstanceHandle | required | identifies the linked runtime instance | must remain opaque and bridge-resolved only |
| session_epoch | SessionEpoch | required | monotonic epoch for the linked session | must be monotonic per runtime instance |
| ownership_domain | OwnershipDomain | required | declares which boundary domain owns publication for the handle | must use the shared registry enum |
| liveness_state | HandleLivenessState | required | declares whether the session handle is live, draining, or dead | must use the shared registry enum |
