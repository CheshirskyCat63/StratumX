
# Evidence Registry

## Active evidence pack
- `evidence/root/root_traceability_log_v3.md`
- `evidence/layers/layer_traceability_log_v3.md`
- `evidence/root/tooling_acceptance_evidence_v3.md`
- `evidence/tests/tooling_test_closure_v3.md`
- `evidence/tests/tooling_test_result_posture_v3.md`
- `evidence/tests/tooling_test_execution_run_v3.md`

## Blocking evidence registry

| Evidence id | Purpose | Active artifact |
|---|---|---|
| `EVID-TOOLS-001` | `L6` core completeness | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-002` | `L6` sidecar completeness | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-003` | `L6A` completeness | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-004` | `L7` completeness | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-005` | `L7A` completeness | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-006` | family completeness and composition-only closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-007` | authority and transaction closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-008` | plane separation closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-009` | command schema and transaction output closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-010` | snapshot/index/derived/artifact/cache closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-011` | stream boundedness closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-012` | workspace/validation/preview/build/release separation closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-013` | `L5 -> L6` intake closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-014` | `L6A` assistant runtime closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-015` | `L7` orchestration closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-016` | `L7A` planning closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-017` | forbidden-path closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-018` | memory/GPU/disk discipline closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-019` | representation ladder closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-020` | family responsibility closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-021` | budget and degradation closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-022` | testing model and active test-closure coverage | `evidence/tests/tooling_test_closure_v3.md` |
| `EVID-TOOLS-023` | evidence and authority-order closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-024` | freeze and handoff closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-025` | lower-stack `L5` compatibility closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-026` | upper consumer `L8+` compatibility closure | `evidence/root/tooling_acceptance_evidence_v3.md` |
| `EVID-TOOLS-027` | active test execution evidence coverage | `evidence/tests/tooling_test_result_posture_v3.md`, `evidence/tests/tooling_test_execution_run_v3.md` |

## Supporting evidence artifacts
- `evidence/root/root_traceability_log_v3.md` provides root-document traceability
- `evidence/layers/layer_traceability_log_v3.md` provides per-layer and per-family completeness traceability
- `evidence/tests/tooling_test_closure_v3.md` provides active test-class coverage closure
- `evidence/tests/tooling_test_result_posture_v3.md` provides active test execution posture
- `evidence/tests/tooling_test_execution_run_v3.md` records the executed document-package validation run

## Historical artifacts (not part of active evidence pack)
- `evidence/root/archival_note_pre_v3.md` explains the superseded pre-evidence state of the package
