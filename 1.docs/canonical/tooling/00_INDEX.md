# StratumX Canonical Tools Stack Index

This package defines the canonical upper tools stack above SDK L5:
- `L6`  editor core / editor operating layer
- `L6A` assistant runtime / bridge
- `L7`  studio orchestrator / meta layer
- `L7A` assistant planner / brain

The stack is frozen around **ownership**, **typed data flow**, **transaction law**, **execution temperature**, and **resource discipline**.
It is intentionally anti-monolith, anti-shadow-world, anti-backdoor, anti-unbounded-growth, and explicitly data-oriented.
The package is designed for very large simulation-heavy worlds, parallel editor work, bounded GPU/RAM/disk growth, and assistant-mediated authoring without authority leakage.

## Reading order
1. `01_SCOPE.md`
2. `02_CANONICAL_STACK.md`
3. `03_ROLE_MAP.md`
4. `04_LIBRARY_BASELINE.md`
5. `05_DEPENDENCY_MODEL.md`
6. `06_COMMUNICATION_MODEL.md`
7. `07_THREADING_MODEL.md`
8. `08_ACTIVATION_MODEL.md`
9. `09_GLOSSARY.md`
10. `10_DOCUMENT_RULES.md`
11. `11_PACKAGE_LAYOUT.md`
12. `12_BOUNDARY_PRESERVATION_MATRIX.md`
13. `13_DATA_PLANE_MODEL.md`
14. `14_AUTHORITY_AND_TRANSACTION_MODEL.md`
15. `15_SNAPSHOT_INDEX_DERIVED_MODEL.md`
16. `16_L6A_ASSISTANT_RUNTIME_MODEL.md`
17. `17_L7_STUDIO_ORCHESTRATION_MODEL.md`
18. `18_L7A_ASSISTANT_BRAIN_MODEL.md`
19. `19_CROSS_LAYER_EXCHANGE_MODEL.md`
20. `20_MEMORY_GPU_DISK_DISCIPLINE.md`
21. `21_FORBIDDEN_CONNECTIONS.md`
22. `22_L5_SYNCHRONIZATION_MODEL.md`
23. `23_BUILD_AND_FREEZE_CONDITIONS.md`
24. `24_TESTING_MODEL.md`
25. `25_IMPLEMENTATION_HANDOFF.md`
26. `26_SHARED_TYPE_REGISTRY.md`
27. `27_ACCEPTANCE_MATRIX.md`
28. `28_LAYER_AND_PLANE_NAMING.md`
29. `29_DOCUMENT_AUTHORITY_ORDER.md`
30. `30_EVIDENCE_REGISTRY.md`
31. `31_ARTIFACT_MANIFEST_LAW.md`
32. `32_REPRESENTATION_LADDER_MODEL.md`
33. `33_BUDGET_RUNTIME_MODEL.md`
34. `34_DOMAIN_FAMILY_DATA_MODEL.md`
35. `35_DEGRADATION_POLICY_MODEL.md`
36. `constitutions/`
37. `levels/`
38. `families/`
39. `99_AUDIT_READINESS_MATRIX.md`

## Canonical rule
- Only `L6` owns editor mutation authority.
- `L6A` may package evidence, request models, lower proposals, and apply/revert only through legal `L6` command and transaction paths.
- `L7` owns compiled studio orchestration only. It never becomes frame-level runtime.
- `L7A` owns reasoning and planning only. It never becomes direct apply authority.
- Data is separated by plane and by exchange class. Equal data classes must co-reside when they share locality and lifetime. Shadow duplication is forbidden.
