# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| transport_policy_id | TransportPolicyId | required | stable identity of one transport policy | must remain stable for the declared policy tuple |
| ingress_order_mode | IngressOrderMode | required | declares write-side ordering mode | must use the shared registry enum |
| egress_fanout_mode | EgressFanoutMode | required | declares read-side fanout mode | must use the shared registry enum |
| envelope_size_class | EnvelopeSizeClass | required | declares expected envelope size discipline | must use the shared registry enum |
| backpressure_policy | BackpressurePolicy | required | declares what the bridge does under downstream pressure | must use the shared registry enum |
| cancellation_policy | CancellationPolicy | required | declares whether write-side requests may be cancelled before publication | must use the shared registry enum |
