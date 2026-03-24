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

## L5D. Compatibility and legality verdict mesh
- `L5.7  compat_verdicts`
- `L5.8  transport_policies`
- `L5.9  legality_gates`

## L5E. Runtime handle mesh
- `L5.10  engine_session_handles`
- `L5.11  engine_object_handles`
- `L5.12  engine_runtime_handles`
- `L5.13  engine_identity_refs`
- `L5.14  engine_state_refs`

## Ordering law
The stack is ordered by bridge visibility and by authority:
- write-side boundaries first
- read-side boundaries second
- compatibility facts next
- compatibility verdicts and transport/gate policy next
- opaque runtime handles and refs last

## Frozen semantic reading
- `link_ingress_packets` and `link_ingress_controls` together form the only canonical write-side bridge into public L4.
- `link_egress_observations` and `link_egress_metrics` together form the only canonical read-side bridge from public L4.
- `compat_verdicts` is the only compatibility verdict layer in L5.
- `legality_gates` is the only per-action legality gate layer in L5.
- all runtime-facing handles and refs are opaque upward and never encode editor workflow state.
