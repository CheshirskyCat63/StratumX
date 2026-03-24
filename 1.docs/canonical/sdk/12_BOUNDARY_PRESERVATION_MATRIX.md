# Boundary Preservation Matrix

| Boundary | Must stay separate from | Why |
|---|---|---|
| `link_ingress_packets` | `link_ingress_controls` | packet envelopes and control envelopes obey different payload laws and stream-order domains |
| `link_egress_observations` | `link_egress_metrics` | observations are semantic runtime emissions; metrics are measurement emissions |
| `compat_versions` | `compat_capabilities` | versions and capabilities drift independently and must not collapse |
| `compat_profiles` | `transport_policies` | profiles select policy; policies describe transport behavior |
| `compat_verdicts` | `legality_gates` | compatibility answers contract truth; gates answer per-action permission now |
| `engine_session_handles` | `engine_runtime_handles` | session lifetime and runtime-surface addressing are different authorities |
| `engine_object_handles` | `engine_identity_refs` | live object addressing and stable identity export are different authorities |
| `engine_identity_refs` | `engine_state_refs` | stable identity and exported state references are different owned data kinds |
| all L5 layers | all L6 workflows | L5 is a truth bridge only and never an editor orchestration layer |
