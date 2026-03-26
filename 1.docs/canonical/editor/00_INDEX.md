# StratumX Editor Product Canon Index

This package defines the editor as a product layer above the canonical tools stack and below any project-specific game stack.
It formalizes the universal editor from `L8` through `L11`:
- `L8` editor product shell and interaction layer
- `L9` domain authoring suites
- `L10` editor services and extensibility
- `L11` collaboration, review, and production operations

The package targets a large-scale Rust-native editor with the product completeness expected from high-end game tools:
multiple docked panels, viewport-centric world editing, scene and content browsers, inspectors, overlays and gizmos, play/simulate/debug flows, build/release surfaces, assistant integration, automation, collaboration, and domain suites broad enough to author many different games.

## Package laws
- `editor/` defines product surfaces, workflows, service surfaces, and suite composition only.
- `editor/` consumes `L6/L6A/L7/L7A` and never weakens their authority laws.
- `editor/` owns no hidden shadow-world and no lower-stack truth.
- Equal product data classes must co-reside by lifetime, locality, and invalidation regime.
- Closed views, inactive suites, and dormant services are cold.
- All mutation authority remains below the product layer.

## Global Canon Alignment

This editor package is part of the `SX-CANON/1.0.6/STACK-v12` unified canonical stack.
It aligns with:
- `../STACK_VERSION`: Global stack version marker
- `../02_STACK_MAP.md`: Global package structure
- `../03_PACKAGE_ROLE_MAP.md`: Package role assignments
- `../05_GLOBAL_AUTHORITY_ORDER.md`: Global authority hierarchy
- `../09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX.md`: Global boundary preservation

## Active Evidence Contour

The active evidence contour for editor gold closure is **v1**.
All evidence artifacts are located in `evidence/` and are from the v1 contour.

## Reading order

### Foundation
1. `STACK_VERSION`
2. `01_SCOPE.md`
3. `02_EDITOR_STACK.md`
4. `03_EDITOR_ROLE_MAP.md`
5. `04_LIBRARY_BASELINE.md`
6. `05_DEPENDENCY_MODEL.md`
7. `06_COMMUNICATION_MODEL.md`
8. `07_THREADING_MODEL.md`

### Product Models
9. `08_EDITOR_PRODUCT_MODEL.md`
10. `09_EDITOR_VISUAL_COMPOSITION_MODEL.md`
11. `10_VIEWPORT_AND_NAVIGATION_MODEL.md`
12. `11_SELECTION_AND_INTERACTION_MODEL.md`
13. `12_TOOL_CONTEXT_AND_MODE_MODEL.md`
14. `13_PANEL_AND_VIEW_MODEL.md`
15. `14_COMMAND_PALETTE_AND_SHORTCUT_MODEL.md`

### Domain Models
16. `15_CONTENT_AND_ASSET_MODEL.md`
17. `16_OUTLINER_WORLD_BROWSER_MODEL.md`
18. `17_INSPECTOR_AND_DETAILS_MODEL.md`
19. `18_PLAY_SIMULATE_DEBUG_MODEL.md`
20. `19_ASSISTANT_SURFACE_MODEL.md`
21. `20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`
22. `21_DOMAIN_SUITE_MODEL.md`

### Runtime Models
23. `22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`
24. `23_RESOURCE_DISCIPLINE.md`
25. `24_UNDO_REDO_AND_HISTORY_MODEL.md`
26. `25_AUTOSAVE_RECOVERY_AND_RESTORE_MODEL.md`
27. `26_SOURCE_CONTROL_AND_CONFLICT_MODEL.md`
28. `27_WORKSPACE_SCHEMA_AND_MIGRATION_MODEL.md`
29. `28_ASSET_VERSIONING_AND_VARIANT_MODEL.md`
30. `29_EDITOR_BUDGET_RUNTIME_MODEL.md`
31. `30_EDITOR_ACTIVATION_AND_COLD_SURFACE_MODEL.md`

### Shared Types and Boundaries
32. `31_SHARED_TYPE_REGISTRY.md`
33. `32_FORBIDDEN_CONNECTIONS.md`
34. `33_BOUNDARY_PRESERVATION_MATRIX.md`
35. `34_GLOSSARY.md`

### Implementation
36. `35_IMPLEMENTATION_BASELINE.md`
37. `36_IMPLEMENTATION_HANDOFF.md`
38. `37_IMPLEMENTATION_SEQUENCE.md`

### Testing and Evidence
39. `38_TESTING_MODEL.md`
40. `39_ACCEPTANCE_MATRIX.md`
41. `40_EVIDENCE_REGISTRY.md`
42. `evidence/` (all evidence artifacts)

### Closure
43. `41_BUILD_AND_FREEZE_CONDITIONS.md`
44. `42_DOCUMENT_AUTHORITY_ORDER.md`
45. `99_AUDIT_READINESS_MATRIX.md`

### Constitutions and Level Contracts
46. `constitutions/` (editor constitutions)
47. `levels/` (level-specific contracts)
48. `families/` (family-specific contracts)
