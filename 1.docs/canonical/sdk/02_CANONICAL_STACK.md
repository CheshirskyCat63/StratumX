# Canonical Stack

## L5A. Write boundary mesh
- `L5.0  link_ingress_packets`
- `L5.1  link_ingress_controls`

## L5B. Read boundary mesh
- `L5.2  link_egress_observations`
- `L5.3  link_egress_metrics`

## L5C. Compatibility fact mesh
- `L5.4  compat_versions`
- `L5.5  compat_capabilities`
- `L5.6  compat_profiles`

## L5D. Verdict and gate mesh
- `L5.7  compat_verdicts`
- `L5.8  transport_policies`
- `L5.9  legality_gates`

## L5E. Runtime-facing opaque mesh
- `L5.10  engine_session_handles`
- `L5.11  engine_object_handles`
- `L5.12  engine_runtime_handles`
- `L5.13  engine_identity_refs`
- `L5.14  engine_state_refs`
- `L5.15  engine_artifact_refs`

## Frozen semantic reading
L5 bridges engine L4 to the upper tools stack without widening into a store, scheduler, planner, cache, or artifact owner.
