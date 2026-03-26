# Evidence Registry

## Purpose

This document records the active evidence chain for package-level closure.
It explicitly selects the active evidence contour and marks superseded versions as non-authoritative.

## Active Evidence Contour

The active evidence contour for SDK gold closure is **v12**.
All evidence artifacts referenced in this registry are from the v12 contour unless explicitly noted.

Previous versions (v9, v10, v11) are superseded and non-authoritative for current gold closure.

## Registry

| Evidence ID | Purpose | Covers Acceptance Rows | Active Artifact Path | Artifact Version | Status | Notes |
|-------------|---------|------------------------|----------------------|------------------|--------|-------|
| EVID-L5-001 | Proves all L5 levels present with complete local docs for critical levels | ROOT-001 | `evidence/layers/layer_traceability_log_v12.md`, `evidence/layers/layer_completeness_proof_v12.md` | v12 | active | Layer completeness verified via traceability log and completeness proof |
| EVID-L5-002 | Proves controls split from packets with distinct role boundaries | ROOT-002 | `evidence/root/root_traceability_log_v12.md`, `13_TYPOLOGY_SYSTEM.md`, `14_ROLE_CLASS_SEPARATION_MATRIX.md` | v12 | active | Role separation verified via typology system |
| EVID-L5-003 | Proves facts split from verdicts with distinct observation/compat boundaries | ROOT-003 | `evidence/root/root_traceability_log_v12.md`, `18_RESULT_ARTIFACT_VERDICT_SEPARATION.md` | v12 | active | Fact/verdict separation verified |
| EVID-L5-004 | Proves artifact refs split from state refs with distinct ref subtypes | ROOT-004 | `evidence/root/root_traceability_log_v12.md`, `17_REF_SUBTYPES.md` | v12 | active | Ref subtype separation verified |
| EVID-L5-005 | Proves public L4 synchronization is explicit | ROOT-005 | `evidence/root/root_traceability_log_v12.md`, `31_ENGINE_L4_BINDING_MAP.md`, `levels/l5.0-link-ingress-packets/41_L4_SYNC_SURFACES.md` | v12 | active | L4 sync surfaces verified |
| EVID-L5-006 | Proves physical data layout model is explicit | ROOT-006 | `evidence/root/root_traceability_log_v12.md`, `36_PHYSICAL_DATA_LAYOUT_MODEL.md` | v12 | active | Physical layout verified |
| EVID-L5-007 | Proves batch and cursor publication model is explicit | ROOT-007 | `evidence/root/root_traceability_log_v12.md`, `37_BATCH_AND_CURSOR_PUBLICATION_MODEL.md` | v12 | active | Batch/cursor model verified |
| EVID-L5-008 | Proves compiled gate and compatibility lookup model is explicit | ROOT-008 | `evidence/root/root_traceability_log_v12.md`, `38_COMPILED_GATE_PROGRAM_MODEL.md` | v12 | active | Gate lookup verified |
| EVID-L5-009 | Proves L5 to L6 snapshot and stream alignment is explicit | ROOT-009 | `evidence/root/root_traceability_log_v12.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | v12 | active | L6 alignment verified |
| EVID-L5-010 | Proves handle/ref opacity law explicit | ROOT-010 | `evidence/root/root_traceability_log_v12.md`, `33_HANDLE_AND_REF_OPACITY_LAW.md` | v12 | active | Opacity verified |
| EVID-L5-011 | Proves no hidden store law explicit | ROOT-011 | `evidence/root/root_traceability_log_v12.md`, `evidence/layers/layer_field_invariant_closure_v12.md` | v12 | active | No hidden store verified |
| EVID-L5-012 | Proves testing model covers all required test classes | ROOT-012 | `evidence/tests/sdk_test_closure_v12.md`, `24_TESTING_MODEL.md` | v12 | active | Test coverage verified |
| EVID-L5-013 | Proves build handoff keeps crate split narrow | ROOT-013 | `evidence/root/root_traceability_log_v12.md`, `25_IMPLEMENTATION_HANDOFF.md` | v12 | active | Build handoff verified |
| EVID-L5-014 | Proves tools consumption map is explicit | ROOT-014 | `evidence/root/root_traceability_log_v12.md`, `34_L5_TO_TOOLS_CONSUMPTION_MAP.md` | v12 | active | Tools consumption verified |
| EVID-L5-015 | Proves hot-path allocation and locality law is explicit | ROOT-015 | `evidence/root/root_traceability_log_v12.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | v12 | active | Hot-path law verified |
| EVID-L5-016 | Proves package closure documents exist and are indexed | ROOT-016 | `00_INDEX.md`, `27_ACCEPTANCE_MATRIX.md`, `30_EVIDENCE_REGISTRY.md`, `99_AUDIT_READINESS_MATRIX.md` | v12 | active | Package closure verified |
| EVID-L5-017 | Proves authority order is explicit and non-circular | ROOT-017 | `evidence/root/root_authority_alignment_v12.md`, `29_DOCUMENT_AUTHORITY_ORDER.md` | v12 | active | Authority order verified |
| EVID-L5-018 | Proves stack version marker is legal and aligned | ROOT-018 | `STACK_VERSION`, `evidence/root/root_traceability_log_v12.md` | v12 | active | Version marker verified |
| EVID-L5-019 | Proves active test execution evidence exists | ROOT-019 | `evidence/tests/sdk_test_result_posture_v12.md`, `evidence/tests/sdk_test_execution_run_v12.md` | v12 | active | Executed document-package validation run recorded |

## Superseded Evidence Versions

| Version | Status | Notes |
|---------|--------|-------|
| v9 | superseded | Historical; not authoritative for current gold closure |
| v10 | superseded | Historical; not authoritative for current gold closure |
| v11 | superseded | Historical; not authoritative for current gold closure |

## Artifact Status Legend
- `active`: Artifact exists, is maintained, and is the current source of truth for v12 contour
- `superseded`: Artifact is from a previous version and is not authoritative for current gold closure
- `planned`: Reserved for artifacts not yet eligible for activation in future revisions

## Notes
- All evidence artifacts must be from the v12 contour to be considered active
- No evidence ID may reference a superseded artifact for current gold closure
- The active artifact path must be relative to the sdk package root (`sdk/`)
