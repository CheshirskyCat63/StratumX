# Testing Model

## Purpose

This document defines the **documentation-package** testing model for the StratumX SDK canon.
It governs validation of the L5 bridge documentation package itself.
It does **not** claim compiled runtime execution for downstream implementation crates.

## Required documentation validation classes

| Test ID | Required class | Primary target | Blocking docs | Result artifact |
|---|---|---|---|---|
| `TEST-L5-001` | Lookup correctness | handles, refs, packets, controls, observations, and metrics are mapped through explicit legal lookup surfaces | `31_ENGINE_L4_BINDING_MAP.md`, `38_COMPILED_GATE_PROGRAM_MODEL.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-002` | Pressure testing | hot-path and batch publication laws remain bounded and non-owning | `36_PHYSICAL_DATA_LAYOUT_MODEL.md`, `37_BATCH_AND_CURSOR_PUBLICATION_MODEL.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-003` | Replay testing | observations and metrics remain replay-safe and classification-clean | `18_RESULT_ARTIFACT_VERDICT_SEPARATION.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-004` | Snapshot swap | public L4 synchronization and snapshot alignment remain explicit | `31_ENGINE_L4_BINDING_MAP.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-005` | Allocation posture | hot-path publication types remain locality-aware and allocation-bounded | `36_PHYSICAL_DATA_LAYOUT_MODEL.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-006` | Opacity preservation | handles and refs remain opaque and non-leaking | `17_REF_SUBTYPES.md`, `33_HANDLE_AND_REF_OPACITY_LAW.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-007` | Boundary legality | L5 public bridge stays narrow and legal across `L4` and tooling consumers | `12_BOUNDARY_PRESERVATION_MATRIX.md`, `16_BOUNDARY_AUTHORITY.md`, `34_L5_TO_TOOLS_CONSUMPTION_MAP.md` | `evidence/tests/sdk_test_execution_run_v12.md` |
| `TEST-L5-008` | Field invariant | documented field invariants remain explicit and non-contradictory across layers and packet families | `15_FIELD_CONTRACT_RULES.md`, `evidence/layers/layer_field_invariant_closure_v12.md` | `evidence/tests/sdk_test_execution_run_v12.md` |

## Implementation-facing extension classes (non-blocking for document-package gold)

The following extension classes remain implementation-facing and may be added by downstream code/test pipelines without changing documentation-package gold closure:
- packet publication legality tests
- control publication legality tests
- legality lookup correctness tests
- ingress ordering tests
- observation batch immutability tests
- metric batch immutability tests
- cursor progression and lag-bound tests
- handle/ref opacity stress tests
- artifact ref opacity tests
- no hidden store tests
- invalidation and epoch rollover tests

## Closure rule

Documentation-package testing closure exists only when:
- all eight required validation classes above have executed verdicts;
- the executed verdicts are recorded in the active execution posture artifact;
- the execution posture artifact is active in the evidence registry;
- acceptance and readiness rows for active execution evidence are `pass`.
