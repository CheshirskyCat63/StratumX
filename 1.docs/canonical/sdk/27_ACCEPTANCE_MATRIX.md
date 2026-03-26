# Acceptance Matrix

## Purpose

This document defines package-level closure conditions for the SDK canonical package root.

## Scope

This matrix covers document-package closure for:
- L5 level completeness and role legality;
- role/class separation closure;
- L4 public bundle closure and binding legality;
- opacity closure for handles and refs;
- no hidden store closure;
- pressure and replay test closure;
- physical data-plane closure;
- batch and cursor publication closure;
- compiled gate lookup closure;
- L6 snapshot/stream alignment closure;
- hot-path allocation and locality closure;
- package evidence and readiness closure.

It does not replace compile, test, or runtime obligations.

## Matrix

| Row | Requirement | Evidence ID | Active Artifact | Status | Notes |
|-----|-------------|-------------|-----------------|--------|-------|
| ROOT-001 | all L5 levels present with complete local docs for critical levels (L5.0, L5.1, L5.2, L5.10, L5.13, L5.15) | EVID-L5-001 | `evidence/layers/layer_traceability_log_v12.md`, `evidence/layers/layer_completeness_proof_v12.md` | pass | Level completeness verified via layer traceability log and completeness proof |
| ROOT-002 | controls split from packets with distinct role boundaries | EVID-L5-002 | `evidence/root/root_traceability_log_v12.md`, `13_TYPOLOGY_SYSTEM.md`, `14_ROLE_CLASS_SEPARATION_MATRIX.md` | pass | Role separation verified via typology system and separation matrix |
| ROOT-003 | facts split from verdicts with distinct observation/compat boundaries | EVID-L5-003 | `evidence/root/root_traceability_log_v12.md`, `18_RESULT_ARTIFACT_VERDICT_SEPARATION.md` | pass | Fact/verdict separation verified via result/artifact/verdict separation doc |
| ROOT-004 | artifact refs split from state refs with distinct ref subtypes | EVID-L5-004 | `evidence/root/root_traceability_log_v12.md`, `17_REF_SUBTYPES.md` | pass | Ref subtype separation verified via ref subtypes doc |
| ROOT-005 | public L4 synchronization is explicit as sinks, snapshots, batches, and invalidation signals | EVID-L5-005 | `evidence/root/root_traceability_log_v12.md`, `31_ENGINE_L4_BINDING_MAP.md`, `levels/l5.0-link-ingress-packets/41_L4_SYNC_SURFACES.md` | pass | L4 sync surfaces verified via engine L4 binding map and level sync docs |
| ROOT-006 | physical data layout model is explicit for all L5 bridge types | EVID-L5-006 | `evidence/root/root_traceability_log_v12.md`, `36_PHYSICAL_DATA_LAYOUT_MODEL.md` | pass | Physical layout verified via data layout model doc |
| ROOT-007 | batch and cursor publication model is explicit for observations and metrics | EVID-L5-007 | `evidence/root/root_traceability_log_v12.md`, `37_BATCH_AND_CURSOR_PUBLICATION_MODEL.md` | pass | Batch/cursor model verified via publication model doc |
| ROOT-008 | compiled gate and compatibility lookup model is explicit | EVID-L5-008 | `evidence/root/root_traceability_log_v12.md`, `38_COMPILED_GATE_PROGRAM_MODEL.md` | pass | Gate lookup verified via compiled gate program model doc |
| ROOT-009 | L5 to L6 snapshot and stream alignment is explicit | EVID-L5-009 | `evidence/root/root_traceability_log_v12.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | pass | L6 alignment verified via snapshot alignment doc |
| ROOT-010 | handle/ref opacity law explicit with no internal exposure | EVID-L5-010 | `evidence/root/root_traceability_log_v12.md`, `33_HANDLE_AND_REF_OPACITY_LAW.md` | pass | Opacity verified via opacity law doc |
| ROOT-011 | no hidden store law explicit with all state externalized | EVID-L5-011 | `evidence/root/root_traceability_log_v12.md`, `evidence/layers/layer_field_invariant_closure_v12.md` | pass | No hidden store verified via field invariant closure |
| ROOT-012 | testing model covers lookup correctness, pressure, replay, snapshot swap, and allocation posture | EVID-L5-012 | `evidence/tests/sdk_test_closure_v12.md`, `24_TESTING_MODEL.md` | pass | Test coverage verified via test closure doc and testing model |
| ROOT-013 | build handoff keeps crate split narrow and physically non-overlapping | EVID-L5-013 | `evidence/root/root_traceability_log_v12.md`, `25_IMPLEMENTATION_HANDOFF.md` | pass | Build handoff verified via implementation handoff doc |
| ROOT-014 | tools consumption map is explicit by plane and access type | EVID-L5-014 | `evidence/root/root_traceability_log_v12.md`, `34_L5_TO_TOOLS_CONSUMPTION_MAP.md` | pass | Tools consumption verified via consumption map doc |
| ROOT-015 | hot-path allocation and locality law is explicit | EVID-L5-015 | `evidence/root/root_traceability_log_v12.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | pass | Hot-path law verified via allocation and locality law doc |
| ROOT-016 | package-level acceptance, evidence, and readiness closure documents exist and are indexed | EVID-L5-016 | `00_INDEX.md`, `27_ACCEPTANCE_MATRIX.md`, `30_EVIDENCE_REGISTRY.md`, `99_AUDIT_READINESS_MATRIX.md` | pass | Package closure docs verified via index and closure docs |
| ROOT-017 | authority order is explicit and non-circular | EVID-L5-017 | `evidence/root/root_authority_alignment_v12.md`, `29_DOCUMENT_AUTHORITY_ORDER.md` | pass | Authority order verified via authority alignment doc |
| ROOT-018 | stack version marker is legal and aligned with global canon | EVID-L5-018 | `STACK_VERSION`, `evidence/root/root_traceability_log_v12.md` | pass | Version marker verified as SX-CANON/1.0.6/STACK-v12 |
| ROOT-019 | active test execution evidence present with executed test-result artifact | EVID-L5-019 | `evidence/tests/sdk_test_result_posture_v12.md`, `evidence/tests/sdk_test_execution_run_v12.md` | pass | Executed document-package validation run recorded |

## Status Legend
- `pass`: Requirement satisfied and evidence artifact verified
- `fail`: Requirement not satisfied or evidence artifact is stale/inactive
