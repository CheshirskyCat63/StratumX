# StratumX Canonical SDK L5 Index

This package defines the canonical thin L5 runtime truth bridge above engine L4 and below editor/tooling L6.
It is intentionally small, typed, non-UI, non-product, and non-editor.

## Reading order
1. `01_SCOPE.md`
2. `02_CANONICAL_STACK.md`
3. `03_ROLE_MAP.md`
4. `04_LIBRARY_BASELINE.md`
5. `05_DEPENDENCY_MODEL.md`
6. `06_COMMUNICATION_MODEL.md`
7. `07_THREADING_MODEL.md`
8. `08_EXECUTION_FLOW.md`
9. `09_GLOSSARY.md`
10. `10_DOCUMENT_RULES.md`
11. `11_PACKAGE_LAYOUT.md`
12. `12_BOUNDARY_PRESERVATION_MATRIX.md`
13. `13_TYPOLOGY_SYSTEM.md`
14. `14_ROLE_CLASS_SEPARATION_MATRIX.md`
15. `15_FIELD_CONTRACT_RULES.md`
16. `16_BOUNDARY_AUTHORITY.md`
17. `17_REF_SUBTYPES.md`
18. `18_RESULT_ARTIFACT_VERDICT_SEPARATION.md`
19. `19_TASK_GRAPH_EDGE_LAW.md`
20. `20_ASSISTANT_SEMANTIC_SEPARATION.md`
21. `21_GENERATION_TRIANGLE_SEPARATION.md`
22. `22_L4_SYNCHRONIZATION_MODEL.md`
23. `23_BUILD_AND_FREEZE_CONDITIONS.md`
24. `24_TESTING_MODEL.md`
25. `25_IMPLEMENTATION_HANDOFF.md`
26. `26_SHARED_TYPE_REGISTRY.md`
27. `27_ACCEPTANCE_MATRIX.md`
28. `28_PACKET_AND_OBSERVATION_NAMING.md`
29. `29_DOCUMENT_AUTHORITY_ORDER.md`
30. `30_EVIDENCE_REGISTRY.md`
31. constitutions
32. per-level docs
33. per-layer contracts
34. `99_AUDIT_READINESS_MATRIX.md`

## Canonical rule
L5 is not editor orchestration.
L5 is not content import.
L5 is not preview generation.
L5 is not assistant planning.
L5 is not release/product workflow state.
L5 is not engine law.

L5 owns only:
- write-side boundary envelopes
- read-side boundary envelopes
- compatibility facts
- compatibility verdicts
- transport policies
- per-action legality gates
- runtime-facing handles and opaque refs

## Package guarantees
This package guarantees:
- one owned data kind per layer
- one frozen role class per layer
- one explicit ref subtype for every handle/ref layer
- one shared canonical type registry for ids, enums, flags, handles, packet classes, packet payload types, observation classes, observation payload types, metric classes, control kinds, verdicts, and statuses
- one write-side boundary mesh and one read-side boundary mesh only
- explicit separation of compatibility truth from legality gating
- explicit separation of transport policy from boundary publication
- explicit separation of handles, identities, and state refs
- explicit per-layer field contracts with requiredness and invariants
- explicit per-layer dependency, communication, threading, and anti-drift contracts
- a testing model and an acceptance matrix for mechanical and semantic readiness
- an authority-order rule that resolves root-doc, constitution, and local-layer conflicts

## Readiness rule
This package may be treated as a sealed implementation canon only when every blocking row in `27_ACCEPTANCE_MATRIX.md` is marked `pass` and `99_AUDIT_READINESS_MATRIX.md` shows no open blockers.

## Non-goals
This package does not define:
- engine internals below public L4 surfaces
- L6 tool workflows, previews, imports, commands, or AI behavior
- L7 studio/product semantics
- UI rendering
- engine runtime scheduling
