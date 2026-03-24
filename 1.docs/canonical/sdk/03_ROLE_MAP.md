# Role Map

## Closed role classes
- boundary layer
- fact layer
- policy layer
- verdict layer
- gate layer
- handle/ref layer

## Role-class meanings
- boundary layer: transfers typed envelopes across a hard system boundary and owns no preparation runtime, no UI orchestration, and no engine execution logic.
- fact layer: states stable compatibility or profile truth and owns no submission, no legality decision, and no result interpretation.
- policy layer: owns transport-policy descriptors only and no queues, retries, schedulers, or dashboards.
- verdict layer: answers a closed compatibility question and owns no submission, no retries, and no runtime execution.
- gate layer: answers a closed per-action legality question and owns no compatibility fact truth, no UI policy, and no submission runtime.
- handle/ref layer: owns only opaque runtime-facing handles or exported refs and never owns payload bodies, projections, or editor identities.

## Mandatory semantic separations
- fact vs verdict: facts state stable truth; verdicts answer a closed compatibility question using those facts.
- verdict vs gate: compatibility verdict answers “is this contract tuple valid”; legality gate answers “may this concrete boundary action pass now”.
- policy vs gate: policy constrains transport behavior; gate decides legality now.
- boundary vs gate: boundary publishes typed envelopes only; gate decides legality only.
- handle vs identity ref: a handle addresses a live runtime surface or object; an identity ref points to a stable exported identity.
- identity ref vs state ref: identity is stable naming only; state ref points to exported runtime state without owning the payload body.

## Layer-class assignments
- boundary: `link_ingress_packets`, `link_ingress_controls`, `link_egress_observations`, `link_egress_metrics`
- fact: `compat_versions`, `compat_capabilities`, `compat_profiles`
- policy: `transport_policies`
- verdict: `compat_verdicts`
- gate: `legality_gates`
- handle/ref: `engine_session_handles`, `engine_object_handles`, `engine_runtime_handles`, `engine_identity_refs`, `engine_state_refs`

## Shared type rule
Any shared id, enum, hash, flag set, handle, packet class, observation class, metric class, control kind, verdict class, or status class used across layers must be defined by `26_SHARED_TYPE_REGISTRY.md`.
