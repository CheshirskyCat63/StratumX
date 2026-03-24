# Testing Model

## Required test classes
- package-layout audit
- terminology audit
- role-class audit
- dependency audit
- field-contract audit
- boundary-authority audit
- threading-model audit
- shared-type-registry audit
- packet/observation/metric naming audit
- opaque-upward handle/ref audit

## Required mechanical proofs
- per-session write-side order
- immutable read-side fanout
- compatibility verdict determinism
- legality-gate determinism
- opaque handle/ref round-trip stability
- no forbidden dependency import

## Non-goal
L5 tests do not prove editor workflows, import pipelines, preview generation, or release orchestration.
