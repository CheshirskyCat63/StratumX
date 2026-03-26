# Evidence Registry

## Purpose

This document records the active evidence chain for package-level closure.

## Active Evidence Contour

The active evidence contour for editor gold closure is **v1**.
All evidence artifacts referenced in this registry are from the v1 contour.

## Registry

| Evidence ID | Purpose | Covers Acceptance Rows | Active Artifact Path | Status | Notes |
|-------------|---------|------------------------|----------------------|--------|-------|
| EVID-ED-001 | Proves editor shell is legally composed without authority leakage | ED-001, ED-004 | `evidence/root/editor_shell_composition_proof_v1.md` | active | Shell composition verified |
| EVID-ED-002 | Proves viewport focus and manipulator systems are properly bounded | ED-002 | `evidence/root/viewport_focus_and_manipulator_proof_v1.md` | active | Viewport focus verified |
| EVID-ED-003 | Proves panel and view systems are properly anchored | ED-003 | `evidence/root/panel_view_anchor_proof_v1.md` | active | Panel anchor verified |
| EVID-ED-004 | Proves interaction routing is sealed | ED-004 | `evidence/root/editor_shell_composition_proof_v1.md` | active | Routing verified |
| EVID-ED-005 | Proves suite authority boundaries are proper | ED-005 | `evidence/root/suite_authority_and_scope_proof_v1.md` | active | Suite authority verified |
| EVID-ED-006 | Proves services and extensions are legally bounded | ED-006 | `evidence/root/service_and_extension_legality_proof_v1.md` | active | Service legality verified |
| EVID-ED-007 | Proves collaboration surfaces are non-authoritative | ED-007 | `evidence/root/collaboration_non_authority_proof_v1.md` | active | Collaboration non-authority verified |
| EVID-ED-008 | Proves activation and resource management are properly bounded | ED-008, ED-009 | `evidence/root/activation_resource_proof_v1.md` | active | Activation resource verified |
| EVID-ED-009 | Proves resource discipline is sealed | ED-009 | `evidence/root/activation_resource_proof_v1.md` | active | Resource discipline verified |
| EVID-ED-010 | Proves no shadow truth exists across editor surfaces | ED-010 | `evidence/root/hidden_parallel_state_audit_v1.md` | active | Hidden state audit verified |
| EVID-ED-011 | Proves testing contour is complete | ED-011 | `evidence/tests/editor_test_class_registry_v1.md` | active | Test class registry verified |
| EVID-ED-012 | Proves acceptance/evidence/readiness alignment | ED-012 | `evidence/root/acceptance_evidence_readiness_alignment_proof_v1.md` | active | Alignment proof verified |
| EVID-ED-013 | Proves active test execution evidence exists | ED-013 | `evidence/tests/editor_test_result_posture_v1.md`, `evidence/tests/editor_test_execution_run_v1.md` | active | Executed document-package validation run recorded |

## Artifact Status Legend
- `active`: Artifact exists, is maintained, and is the current source of truth for the v1 contour
- `superseded`: Artifact is from a previous version and is not authoritative for current gold closure
- `planned`: Reserved for future artifacts not yet eligible for activation

## Notes
- All evidence artifacts must be from the v1 contour to be considered active
- No evidence ID may reference a superseded artifact for current gold closure
- The active artifact path must be relative to the editor package root (`editor/`)
