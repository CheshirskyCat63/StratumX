# Testing Model

## Purpose

This document defines the **documentation-package** testing model for the StratumX editor canon.
It governs validation of the editor product documentation package itself.
It does **not** claim compiled runtime execution for an implementation editor binary.

## Required test classes

| Test ID | Required class | Primary target | Blocking docs | Result artifact |
|---|---|---|---|---|
| `TEST-ED-001` | Shell composition tests | shell composition, workspace host legality, and editor non-authority posture | `08_EDITOR_PRODUCT_MODEL.md`, `33_BOUNDARY_PRESERVATION_MATRIX.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-002` | Viewport focus tests | viewport focus, navigation, and manipulator legality | `10_VIEWPORT_AND_NAVIGATION_MODEL.md`, `11_SELECTION_AND_INTERACTION_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-003` | Panel anchor tests | mandatory panel/view anchors and routing legality | `13_PANEL_AND_VIEW_MODEL.md`, `33_BOUNDARY_PRESERVATION_MATRIX.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-004` | Suite authority tests | domain suite boundaries and non-project-specific authority | `21_DOMAIN_SUITE_MODEL.md`, `32_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-005` | Service legality tests | service surfaces and extension legality | `20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`, `32_FORBIDDEN_CONNECTIONS.md`, `33_BOUNDARY_PRESERVATION_MATRIX.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-006` | Collaboration non-authority tests | collaboration and production surfaces remain non-authoritative | `19_ASSISTANT_SURFACE_MODEL.md`, `32_FORBIDDEN_CONNECTIONS.md`, `33_BOUNDARY_PRESERVATION_MATRIX.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-007` | Activation resource tests | activation, resource discipline, and cold-surface boundedness | `22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`, `23_RESOURCE_DISCIPLINE.md`, `30_EDITOR_ACTIVATION_AND_COLD_SURFACE_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-008` | Hidden state audit tests | no shadow truth across shell, panels, suites, services, and collaboration | `32_FORBIDDEN_CONNECTIONS.md`, `33_BOUNDARY_PRESERVATION_MATRIX.md`, `34_GLOSSARY.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-009` | Undo/redo tests | undo/redo and history legality | `24_UNDO_REDO_AND_HISTORY_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-010` | Autosave/recovery tests | autosave, restore, and recovery legality | `25_AUTOSAVE_RECOVERY_AND_RESTORE_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-011` | Source control tests | source control and conflict handling legality | `26_SOURCE_CONTROL_AND_CONFLICT_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-012` | Workspace migration tests | workspace schema and migration legality | `27_WORKSPACE_SCHEMA_AND_MIGRATION_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-013` | Asset versioning tests | asset versioning and variant model legality | `28_ASSET_VERSIONING_AND_VARIANT_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-014` | Budget envelope tests | budget and runtime envelope for hot editor surfaces | `29_EDITOR_BUDGET_RUNTIME_MODEL.md`, `30_EDITOR_ACTIVATION_AND_COLD_SURFACE_MODEL.md` | `evidence/tests/editor_test_execution_run_v1.md` |
| `TEST-ED-015` | Plugin capability tests | plugin and extension host legality | `21_DOMAIN_SUITE_MODEL.md`, `32_FORBIDDEN_CONNECTIONS.md`, `33_BOUNDARY_PRESERVATION_MATRIX.md` | `evidence/tests/editor_test_execution_run_v1.md` |

## Gold rule

The editor package is gold only when:
- all 15 required test classes above have executed verdicts;
- the execution posture artifact is active in the evidence registry;
- acceptance, evidence, and readiness rows for test execution evidence are fully aligned.
