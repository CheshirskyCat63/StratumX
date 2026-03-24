# Role-Class Separation Matrix

| Layer | Role class | Must not absorb |
|---|---|---|
| `link_ingress_packets` | boundary layer | controls, gates, retries, queues |
| `link_ingress_controls` | boundary layer | packets, gates, execution state |
| `link_egress_observations` | boundary layer | metrics, projections, dashboards |
| `link_egress_metrics` | boundary layer | observations, aggregators, dashboards |
| `compat_versions` | fact layer | capabilities, profiles, verdicts |
| `compat_capabilities` | fact layer | versions, profiles, verdicts |
| `compat_profiles` | fact layer | verdicts, gates, UI profiles |
| `compat_verdicts` | verdict layer | facts, gates, submissions |
| `transport_policies` | policy layer | profiles, gates, queues, retries |
| `legality_gates` | gate layer | compatibility truth, submissions, retries |
| `engine_session_handles` | handle/ref layer | runtime handles, state refs, UI sessions |
| `engine_object_handles` | handle/ref layer | identities, state refs, object metadata |
| `engine_runtime_handles` | handle/ref layer | observations, metrics, schedulers |
| `engine_identity_refs` | handle/ref layer | state payloads, editor identities |
| `engine_state_refs` | handle/ref layer | state payload bodies, projections |
