# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| egress_metric_envelope_id | EgressMetricEnvelopeId | required | stable identity of one emitted metric envelope | must remain unique per engine session |
| metric_class | MetricClass | required | closed metric class enum | must use the shared registry metric class enum |
| metric_payload_type | MetricPayloadType | required | closed payload-type enum for the metric body | must pair lawfully with `metric_class` |
| source_runtime_handle | EngineRuntimeHandle | required | identifies the originating public runtime surface | must resolve through `engine_runtime_handles` |
| source_session_handle | EngineSessionHandle | required | identifies the source bridge session | must resolve through `engine_session_handles` |
| measurement_window | MeasurementWindow | required | describes the metric aggregation window supplied by L4 | must be descriptive only and not an L5-owned aggregation state |
