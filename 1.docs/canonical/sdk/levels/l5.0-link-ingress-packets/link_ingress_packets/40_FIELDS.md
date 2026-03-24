# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| ingress_packet_envelope_id | IngressPacketEnvelopeId | required | stable identity of one submitted packet envelope | must remain unique within one engine session |
| packet_class | PacketClass | required | closed class enum for the submitted packet | must use the shared registry packet class enum |
| packet_payload_type | PacketPayloadType | required | closed payload-type enum for the submitted packet body | must pair lawfully with `packet_class` |
| target_runtime_handle | EngineRuntimeHandle | required | identifies the addressed public runtime surface | must resolve through `engine_runtime_handles` |
| source_session_handle | EngineSessionHandle | required | identifies the source bridge session | must resolve through `engine_session_handles` |
| submission_order_key | SubmissionOrderKey | required | preserves ordered write-side publication | must be monotonic per session and per stream |
