# Document Authority Order

## Purpose

This document defines the formal authority hierarchy for the editor canonical package.

## Authority Hierarchy

Highest authority inside `editor/`:

### Level 1: Global Canon (External)
- `../STACK_VERSION`
- `../00_INDEX.md` through `../09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX.md`
- `../99_GLOBAL_AUDIT_READINESS_MATRIX.md`

Lower-stack law (engine, sdk, tooling) always overrides `editor/` if any contradiction appears.

### Level 2: Constitutions
- `constitutions/STRATUMX_EDITOR_PRODUCT_CONSTITUTION.md`
- `constitutions/STRATUMX_EDITOR_VIEWPORT_AND_INTERACTION_CONSTITUTION.md`
- `constitutions/STRATUMX_EDITOR_DOMAIN_SUITE_CONSTITUTION.md`
- `constitutions/STRATUMX_EDITOR_SERVICE_AND_EXTENSION_CONSTITUTION.md`
- `constitutions/STRATUMX_EDITOR_COLLABORATION_CONSTITUTION.md`
- `constitutions/STRATUMX_EDITOR_RESOURCE_DISCIPLINE_CONSTITUTION.md`

Constitutions define immutable laws that cannot be contradicted by lower-authority documents.

### Level 3: Package Root Index and Scope
- `STACK_VERSION`
- `00_INDEX.md`
- `01_SCOPE.md`
- `02_EDITOR_STACK.md`
- `03_EDITOR_ROLE_MAP.md`

### Level 4: Foundation Models
- `04_LIBRARY_BASELINE.md`
- `05_DEPENDENCY_MODEL.md`
- `06_COMMUNICATION_MODEL.md`
- `07_THREADING_MODEL.md`

### Level 5: Product Models
- `08_EDITOR_PRODUCT_MODEL.md`
- `09_EDITOR_VISUAL_COMPOSITION_MODEL.md`
- `10_VIEWPORT_AND_NAVIGATION_MODEL.md`
- `11_SELECTION_AND_INTERACTION_MODEL.md`
- `12_TOOL_CONTEXT_AND_MODE_MODEL.md`
- `13_PANEL_AND_VIEW_MODEL.md`
- `14_COMMAND_PALETTE_AND_SHORTCUT_MODEL.md`

### Level 6: Domain Models
- `15_CONTENT_AND_ASSET_MODEL.md`
- `16_OUTLINER_WORLD_BROWSER_MODEL.md`
- `17_INSPECTOR_AND_DETAILS_MODEL.md`
- `18_PLAY_SIMULATE_DEBUG_MODEL.md`
- `19_ASSISTANT_SURFACE_MODEL.md`
- `20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`
- `21_DOMAIN_SUITE_MODEL.md`

### Level 7: Runtime Models
- `22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`
- `23_RESOURCE_DISCIPLINE.md`
- `24_UNDO_REDO_AND_HISTORY_MODEL.md`
- `25_AUTOSAVE_RECOVERY_AND_RESTORE_MODEL.md`
- `26_SOURCE_CONTROL_AND_CONFLICT_MODEL.md`
- `27_WORKSPACE_SCHEMA_AND_MIGRATION_MODEL.md`
- `28_ASSET_VERSIONING_AND_VARIANT_MODEL.md`
- `29_EDITOR_BUDGET_RUNTIME_MODEL.md`
- `30_EDITOR_ACTIVATION_AND_COLD_SURFACE_MODEL.md`

### Level 8: Shared Types and Boundaries
- `31_SHARED_TYPE_REGISTRY.md`
- `32_FORBIDDEN_CONNECTIONS.md`
- `33_BOUNDARY_PRESERVATION_MATRIX.md`
- `34_GLOSSARY.md`

### Level 9: Implementation Baseline
- `35_IMPLEMENTATION_BASELINE.md`
- `36_IMPLEMENTATION_HANDOFF.md`
- `37_IMPLEMENTATION_SEQUENCE.md`

### Level 10: Testing and Evidence
- `38_TESTING_MODEL.md`
- `39_ACCEPTANCE_MATRIX.md`
- `40_EVIDENCE_REGISTRY.md`
- `evidence/` (all evidence artifacts)

### Level 11: Freeze and Closure
- `41_BUILD_AND_FREEZE_CONDITIONS.md`
- `99_AUDIT_READINESS_MATRIX.md`

### Level 12: Level Contracts
- `levels/` (all level-specific documents)
- `families/` (all family-specific documents)

## Authority Discipline

- Higher-level documents override lower-level documents
- Constitutions cannot be contradicted by any lower-level document
- Global canon cannot be contradicted by any editor document
- Lower-stack law (engine, sdk, tooling) cannot be contradicted by any editor document
- Evidence artifacts verify but do not override acceptance/readiness matrices
- Level contracts must conform to all higher-level documents

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
