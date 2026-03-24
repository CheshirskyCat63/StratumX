# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| egress_observation_envelope_id | EgressObservationEnvelopeId | required | stable identity of one emitted observation envelope | must remain unique per engine session |
| observation_class | ObservationClass | required | closed observation class enum | must use the shared registry observation class enum |
| observation_payload_type | ObservationPayloadType | required | closed payload-type enum for the observation body | must pair lawfully with `observation_class` |
| source_runtime_handle | EngineRuntimeHandle | required | identifies the originating public runtime surface | must resolve through `engine_runtime_handles` |
| source_session_handle | EngineSessionHandle | required | identifies the source bridge session | must resolve through `engine_session_handles` |
| emission_epoch | EmissionEpoch | required | preserves source-side temporal order | must be monotonic per source runtime handle |
