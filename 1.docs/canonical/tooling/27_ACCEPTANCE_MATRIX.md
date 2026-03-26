# Acceptance Matrix

Statuses in this matrix are valid only through the active evidence pack declared by `29_DOCUMENT_AUTHORITY_ORDER.md` and `30_EVIDENCE_REGISTRY.md`.

| Row | Requirement | Evidence ID | Active Artifact | Status | Notes |
|-----|-------------|-------------|-----------------|--------|-------|
| ROOT-001 | all `L6` core levels present | EVID-TOOLS-001 | `evidence/root/l6_plane_separation_proof_v3.md` | pass | L6 core levels verified |
| ROOT-002 | all `L6` sidecar levels present | EVID-TOOLS-002 | `evidence/root/l6_plane_separation_proof_v3.md` | pass | L6 sidecar levels verified |
| ROOT-003 | all `L6A` levels present | EVID-TOOLS-003 | `evidence/root/l6_plane_separation_proof_v3.md` | pass | L6A levels verified |
| ROOT-004 | all `L7` levels present | EVID-TOOLS-004 | `evidence/root/l6_plane_separation_proof_v3.md` | pass | L7 levels verified |
| ROOT-005 | all `L7A` levels present | EVID-TOOLS-005 | `evidence/root/l6_plane_separation_proof_v3.md` | pass | L7A levels verified |
| ROOT-006 | all declared families present and composition-only | EVID-TOOLS-006 | `families/` | pass | Family composition verified |
| ROOT-007 | `L6` authority and transaction law explicit | EVID-TOOLS-007 | `levels/l6.0-authority-core/` | pass | Authority law verified |
| ROOT-008 | plane-separated data model explicit | EVID-TOOLS-008 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Plane separation verified |
| ROOT-009 | command schema and transaction outputs explicit | EVID-TOOLS-009 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Command schema verified |
| ROOT-010 | snapshot/index/derived/artifact/cache separation explicit | EVID-TOOLS-010 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Plane separation verified |
| ROOT-011 | stream boundedness explicit | EVID-TOOLS-011 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Stream boundedness verified |
| ROOT-012 | workspace/validation/preview/build/release separation explicit | EVID-TOOLS-012 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Runtime separation verified |
| ROOT-013 | `L5 -> L6` intake law explicit | EVID-TOOLS-013 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | L5 intake verified |
| ROOT-014 | `L6A` evidence/proposal/lowering/safety/apply model explicit | EVID-TOOLS-014 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Assistant model verified |
| ROOT-015 | `L7` cold orchestration and campaign model explicit | EVID-TOOLS-015 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Orchestration verified |
| ROOT-016 | `L7A` goal/plan/reasoning/routing model explicit | EVID-TOOLS-016 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Planning verified |
| ROOT-017 | forbidden path matrix explicit | EVID-TOOLS-017 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Forbidden paths verified |
| ROOT-018 | memory/GPU/disk discipline explicit | EVID-TOOLS-018 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Resource discipline verified |
| ROOT-019 | representation ladder explicit | EVID-TOOLS-019 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Representation ladder verified |
| ROOT-020 | family data responsibility law explicit | EVID-TOOLS-020 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Family responsibility verified |
| ROOT-021 | budget and degradation enforcement explicit | EVID-TOOLS-021 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Budget enforcement verified |
| ROOT-022 | testing contour and active test-closure explicit | EVID-TOOLS-022 | `evidence/tests/tooling_test_closure_v3.md` | pass | Test closure verified |
| ROOT-023 | active evidence pack and authority order explicit | EVID-TOOLS-023 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Evidence pack verified |
| ROOT-024 | freeze and implementation handoff contour explicit | EVID-TOOLS-024 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Freeze contour verified |
| ROOT-025 | lower-stack compatibility with `L5` explicit | EVID-TOOLS-025 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | L5 compatibility verified |
| ROOT-026 | upper consumer compatibility for `L8+` editor surfaces explicit | EVID-TOOLS-026 | `evidence/root/tooling_acceptance_evidence_v3.md` | pass | Editor compatibility verified |
| ROOT-027 | active test execution evidence present with executed test-result artifact | EVID-TOOLS-027 | `evidence/tests/tooling_test_result_posture_v3.md`, `evidence/tests/tooling_test_execution_run_v3.md` | pass | Executed document-package validation run recorded |

## Status Legend
- `pass`: Requirement satisfied and evidence artifact verified
- `fail`: Requirement not satisfied or evidence artifact is stale/inactive
