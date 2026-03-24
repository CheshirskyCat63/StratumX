# Communication Model

## Canonical communication surfaces
- write-side publication: `link_ingress_packets`, `link_ingress_controls`
- read-side publication: `link_egress_observations`, `link_egress_metrics`
- fact publication: `compat_versions`, `compat_capabilities`, `compat_profiles`
- verdict publication: `compat_verdicts`, `legality_gates`
- handle/ref publication: `engine_session_handles`, `engine_object_handles`, `engine_runtime_handles`, `engine_identity_refs`, `engine_state_refs`

## Communication law
- write-side boundary publication is ordered and single-owner.
- read-side boundary publication is immutable fanout.
- fact publication is low-churn and immutable after publication.
- verdict and gate publication may evaluate in parallel but publish one immutable answer per evaluated tuple.
- handle/ref publication is immutable fanout and never embeds editor workflow state.

## Forbidden communication
- no direct L6-to-L4 communication.
- no direct L4-to-L6 communication.
- no direct fact-to-engine submission.
- no direct handle/ref-to-engine submission.
- no verdict or gate layer may mutate runtime state.
