# Dependency Model

## Global direction
- L5 depends downward on public L4 surfaces only.
- L5 never depends on L6 tooling, L7 studio logic, or L8 game logic.
- lateral imports are allowed only when justified by role-class law and by local layer contracts.

## Boundary authority law
- `link_ingress_packets` may depend only on compatibility facts, legality gates, transport policies, runtime handles, and public L4 packet surfaces.
- `link_ingress_controls` may depend only on compatibility facts, legality gates, transport policies, runtime handles, object handles, and public L4 control surfaces.
- `link_egress_observations` may depend only on public L4 observation surfaces and exported runtime/session handles.
- `link_egress_metrics` may depend only on public L4 metric surfaces and exported runtime/session handles.
- no other L5 layer may submit into engine or consume public engine observations or metrics directly unless its local contract names a public export surface.

## Fact-layer dependency law
- fact layers may depend on shared ids, enums, flags, and policies where justified.
- fact layers may not depend on boundary envelopes for their meaning.
- fact layers may not depend on editor concepts.

## Verdict and gate dependency law
- verdict layers may depend on facts only.
- gate layers may depend on facts, verdicts, policies, and runtime-facing handles only.
- gates may not absorb submission, retries, or queues.

## Handle/ref-layer dependency law
- handle/ref layers may depend only on lower-fidelity exported handles or public L4 export surfaces where justified.
- handle/ref layers may not depend on editor, content, assistant, preview, release, or product layers.
- state refs may depend on observation anchors only when the anchor stays a pure reference.

## Forbidden dependency shortcuts
- no L5 layer may import an L6 view, request, workflow, preview, import, command, delta, release, or assistant type.
- no L5 layer may import engine internals below public L4 surfaces.
- no handle/ref layer may depend on gates or verdicts for its meaning.
