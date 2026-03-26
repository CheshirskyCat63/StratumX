# Editor Implementation Handoff

## Purpose

This document defines the implementation handoff package for engineers and agents implementing the editor package.

## Scope

This handoff covers the complete editor package (`L8` through `L11`) implementation requirements.

## Package Overview

The editor package is a production-grade game editor with:
- Multi-viewport 3D/2D editing
- Dockable panel system (outliner, content browser, inspector, diagnostics, assistant, build/release, collaboration)
- Domain authoring suites (world, terrain, material, environment, destruction, simulation, animation, audio, UI)
- Editor services (import/export, graph authoring, automation, script/hot-reload, plugin/extension, template/preset, package/market)
- Collaboration surfaces (session, review, annotation, playtest, production dashboard, learning/onboarding)
- Undo/redo with command journal
- Autosave and crash recovery
- Source control integration
- Workspace schema and migration
- Asset versioning and variants
- Performance budgets and activation discipline

## Authority Boundaries

The editor must NEVER:
- Own world truth or entity authority (consumed from `L6.0` authority-core)
- Own asset authority or compilation state (consumed from `L6.1` asset-compiler)
- Own package state or dependency resolution (consumed from `L6.2` package-manager)
- Maintain shadow truth or parallel state
- Bypass lower-stack authority

## Implementation Sequence

See `37_IMPLEMENTATION_SEQUENCE.md` for detailed build order.

## Critical Documents

### Foundation
- `01_SCOPE.md`: Package scope and boundaries
- `02_EDITOR_STACK.md`: Layer structure (L8-L11)
- `03_EDITOR_ROLE_MAP.md`: Role assignments
- `04_LIBRARY_BASELINE.md`: Allowed library classes
- `05_DEPENDENCY_MODEL.md`: Legal dependencies
- `06_COMMUNICATION_MODEL.md`: Message classes
- `07_THREADING_MODEL.md`: Threading discipline

### Product Model
- `08_EDITOR_PRODUCT_MODEL.md`: Product composition
- `09_EDITOR_VISUAL_COMPOSITION_MODEL.md`: Visual structure
- `10_VIEWPORT_AND_NAVIGATION_MODEL.md`: Viewport system
- `11_SELECTION_AND_INTERACTION_MODEL.md`: Selection and focus
- `12_TOOL_CONTEXT_AND_MODE_MODEL.md`: Tool modes
- `13_PANEL_AND_VIEW_MODEL.md`: Panel system
- `14_COMMAND_PALETTE_AND_SHORTCUT_MODEL.md`: Commands

### Domain Models
- `15_CONTENT_AND_ASSET_MODEL.md`: Asset consumption
- `16_OUTLINER_WORLD_BROWSER_MODEL.md`: Outliner panel
- `17_INSPECTOR_AND_DETAILS_MODEL.md`: Inspector panel
- `18_PLAY_SIMULATE_DEBUG_MODEL.md`: Play/simulate/debug
- `19_ASSISTANT_SURFACE_MODEL.md`: Assistant integration
- `20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`: Build/release/diagnostics
- `21_DOMAIN_SUITE_MODEL.md`: Suite catalog

### Runtime Models
- `22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`: Dataflow and activation
- `23_RESOURCE_DISCIPLINE.md`: Resource management
- `24_UNDO_REDO_AND_HISTORY_MODEL.md`: Undo/redo
- `25_AUTOSAVE_RECOVERY_AND_RESTORE_MODEL.md`: Autosave/recovery
- `26_SOURCE_CONTROL_AND_CONFLICT_MODEL.md`: Source control
- `27_WORKSPACE_SCHEMA_AND_MIGRATION_MODEL.md`: Workspace migration
- `28_ASSET_VERSIONING_AND_VARIANT_MODEL.md`: Asset versioning
- `29_EDITOR_BUDGET_RUNTIME_MODEL.md`: Performance budgets
- `30_EDITOR_ACTIVATION_AND_COLD_SURFACE_MODEL.md`: Activation discipline

### Shared Types
- `31_SHARED_TYPE_REGISTRY.md`: All typed IDs and message classes
- `32_FORBIDDEN_CONNECTIONS.md`: Explicit forbidden patterns
- `33_BOUNDARY_PRESERVATION_MATRIX.md`: Boundary verification
- `34_GLOSSARY.md`: Editor-specific terms

### Closure Documents
- `35_IMPLEMENTATION_BASELINE.md`: Implementation readiness
- `38_TESTING_MODEL.md`: Testing requirements
- `39_ACCEPTANCE_MATRIX.md`: Acceptance criteria
- `40_EVIDENCE_REGISTRY.md`: Evidence artifacts
- `41_BUILD_AND_FREEZE_CONDITIONS.md`: Freeze criteria
- `42_DOCUMENT_AUTHORITY_ORDER.md`: Authority hierarchy
- `99_AUDIT_READINESS_MATRIX.md`: Audit readiness

### Constitutions
- `constitutions/STRATUMX_EDITOR_PRODUCT_CONSTITUTION.md`: Product laws
- `constitutions/STRATUMX_EDITOR_VIEWPORT_AND_INTERACTION_CONSTITUTION.md`: Viewport laws
- `constitutions/STRATUMX_EDITOR_DOMAIN_SUITE_CONSTITUTION.md`: Suite laws
- `constitutions/STRATUMX_EDITOR_SERVICE_AND_EXTENSION_CONSTITUTION.md`: Service laws
- `constitutions/STRATUMX_EDITOR_COLLABORATION_CONSTITUTION.md`: Collaboration laws
- `constitutions/STRATUMX_EDITOR_RESOURCE_DISCIPLINE_CONSTITUTION.md`: Resource laws

### Level Contracts
- `levels/l8.0-editor-shell/`: Shell implementation
- `levels/l8.1-viewport-system/`: Viewport implementation
- `levels/l8.2-outliner-system/`: Outliner implementation
- `levels/l8.3-content-browser-system/`: Content browser implementation
- `levels/l8.4-inspector-system/`: Inspector implementation
- `levels/l8.5-tool-context-system/`: Tool context implementation
- `levels/l8.6-overlay-and-gizmo-system/`: Overlay/gizmo implementation
- `levels/l8.7-workspace-layout-system/`: Layout implementation
- `levels/l8.8-interaction-routing-system/`: Interaction routing implementation
- `levels/l8.9-assistant-surface/`: Assistant implementation
- `levels/l8.10-diagnostics-surface/`: Diagnostics implementation
- `levels/l8.11-build-release-surface/`: Build/release implementation
- `levels/l9.0-world-authoring-suite/`: World suite implementation
- `levels/l9.1-terrain-material-environment-suite/`: Terrain/material/environment implementation
- `levels/l9.2-destruction-simulation-suite/`: Destruction/simulation implementation
- `levels/l9.3-animation-audio-ui-suite/`: Animation/audio/UI implementation
- `levels/l10.0-project-bootstrap-service/`: Project bootstrap implementation
- `levels/l10.1-import-export-pipeline-service/`: Import/export implementation
- `levels/l10.2-graph-authoring-service/`: Graph authoring implementation
- `levels/l10.3-automation-and-batch-service/`: Automation implementation
- `levels/l10.4-script-and-hot-reload-service/`: Script/hot-reload implementation
- `levels/l10.5-plugin-and-extension-host/`: Plugin/extension implementation
- `levels/l10.6-template-preset-and-scaffold-service/`: Template/preset implementation
- `levels/l10.7-package-market-and-dependency-service/`: Package/market implementation
- `levels/l11.0-collaboration-session-surface/`: Collaboration session implementation
- `levels/l11.1-review-annotation-surface/`: Review/annotation implementation
- `levels/l11.2-asset-gate-and-approval-surface/`: Asset gate/approval implementation
- `levels/l11.3-playtest-and-capture-operations/`: Playtest/capture implementation
- `levels/l11.4-production-dashboard-and-traceability/`: Production dashboard implementation
- `levels/l11.5-learning-onboarding-and-help-surface/`: Learning/onboarding implementation

## Testing Requirements

See `38_TESTING_MODEL.md` for complete testing requirements.

Minimum test coverage:
- Shell composition and layout persistence
- Viewport rendering and navigation
- Panel docking and focus
- Selection and interaction routing
- Suite activation and deactivation
- Service lifecycle and cancellation
- Undo/redo correctness
- Autosave and recovery
- Workspace migration
- Budget enforcement
- No shadow truth audit

## Evidence Requirements

See `40_EVIDENCE_REGISTRY.md` for complete evidence requirements.

All evidence artifacts must exist and be active:
- `evidence/root/editor_shell_composition_proof_v1.md`
- `evidence/root/viewport_focus_and_manipulator_proof_v1.md`
- `evidence/root/panel_view_anchor_proof_v1.md`
- `evidence/root/suite_authority_and_scope_proof_v1.md`
- `evidence/root/service_and_extension_legality_proof_v1.md`
- `evidence/root/collaboration_non_authority_proof_v1.md`
- `evidence/root/activation_resource_proof_v1.md`
- `evidence/root/hidden_parallel_state_audit_v1.md`
- `evidence/tests/editor_test_class_registry_v1.md`
- `evidence/root/acceptance_evidence_readiness_alignment_proof_v1.md`

## Acceptance Criteria

See `39_ACCEPTANCE_MATRIX.md` for complete acceptance criteria.

All acceptance rows must pass:
- ED-001: Shell composition legal
- ED-002: Viewport system complete
- ED-003: Mandatory panels complete
- ED-004: Interaction routing sealed
- ED-005: Suite catalog complete
- ED-006: Services complete
- ED-007: Collaboration surfaces complete
- ED-008: Dataflow and activation sealed
- ED-009: Resource discipline sealed
- ED-010: No shadow truth
- ED-011: Testing contour complete
- ED-012: Freeze contour aligned

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.

