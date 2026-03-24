# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| compat_profile_id | CompatProfileId | required | stable identity of one compatibility profile | must remain stable for the declared profile tuple |
| profile_kind | CompatProfileKind | required | declares baseline, strict, or reduced bridge profile class | must use the shared registry profile enum |
| compatibility_mode | CompatibilityMode | required | declares the allowed negotiation mode | must use the shared registry compatibility mode enum |
| concurrency_profile | ConcurrencyProfile | required | declares ordered, fanout, or mixed boundary concurrency profile | must use the shared registry concurrency profile enum |
| transport_policy_ref | TransportPolicyRef | required | links the profile to one transport policy record | must resolve through `transport_policies` |
