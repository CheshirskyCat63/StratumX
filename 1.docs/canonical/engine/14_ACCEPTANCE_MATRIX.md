# Acceptance Matrix

## Purpose

This document defines package-level closure conditions for `docs/canonical/engine/`.

## Scope

This matrix covers document-package closure for:
- stack and role identity;
- dependency legality;
- runtime phase closure;
- repository/package alignment;
- implementation-order legality;
- API-law to skeleton legality;
- performance reproducibility, naming legality, baseline-capture/result registration, mechanical profile-composition binding, budget-ledger binding, and contradiction-audit coverage for the registered performance set;
- declared hardened affected-level explicit coverage mapping and local-tightening registration over the registered affected set;
- full implementation package closure including numeric-validator artifact contract, numeric source digest set, and package-contained validator result;
- profile-specific memory and residency separation;
- registered constitutional package explicit coverage mapping and package-truth registration;
- registered root package explicit coverage mapping and package-truth registration;
- stack-version marker legality;
- benchmark reproducibility closure;
- numeric source-of-truth governance and validator closure;
- package evidence and readiness closure.

It does not replace compile, test, or performance obligations defined by constitutional law.
It also does not claim theorem-proof semantic closure beyond the cited coverage sets and package-contained artifacts.

## Matrix

| ID | Requirement | Canonical Evidence | Status |
|---|---|---|---|
| ROOT-001 | stack, level, role, and glossary identity are mutually aligned | `02_CANONICAL_STACK.md`, `03_ROLE_MAP.md`, `09_GLOSSARY.md` | pass |
| ROOT-002 | root dependency model matches local dependency baselines for runtime-resource, critical-simulation, and network-runtime crates | `05_DEPENDENCY_MODEL.md`, `levels/l1.5-runtime-resource-services/00_LEVEL.md`, `levels/l2-critical-simulation/00_LEVEL.md`, `levels/l2.5-network-runtime-services/00_LEVEL.md` | pass |
| ROOT-003 | runtime phase order is closed and identical across root, constitution, and local runtime docs | `08_EXECUTION_FLOW.md`, `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `levels/l1-runtime-kernel/runtime/41_PHASE_MODEL.md` | pass |
| ROOT-004 | canonical repository shape, package layout, and implementation package index are aligned | `11_PACKAGE_LAYOUT.md`, `constitutions/STRATUMX_WORKSPACE_AND_REPOSITORY_CANON.md`, `implementation/00_INDEX.md` | pass |
| ROOT-005 | `engine_content` relation to `L1.5` is defined as startup-mediated product flow, not upward crate dependency | `05_DEPENDENCY_MODEL.md`, `06_COMMUNICATION_MODEL.md`, `08_EXECUTION_FLOW.md`, `levels/l1.5-runtime-resource-services/00_LEVEL.md`, `levels/l3.2-resource-systems/content/00_LAYER.md`, `levels/l4-startup/startup/42_RUNTIME_WIRING.md` | pass |
| ROOT-006 | canonical implementation order is legal against the canonical crate dependency graph, including intra-phase order | `05_DEPENDENCY_MODEL.md`, `implementation/STRATUMX_IMPLEMENTATION_ORDER.md` | pass |
| ROOT-007 | constitutional API law and implementation-facing crate contract skeletons are legal and mutually aligned across all crates | `constitutions/STRATUMX_API_CONTRACT_LAW.md`, `implementation/STRATUMX_CRATE_CONTRACT_SKELETONS.md` | pass |
| ROOT-008 | package-level acceptance, evidence, and readiness closure documents exist and are indexed | `00_INDEX.md`, `11_PACKAGE_LAYOUT.md`, `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` | pass |
| ROOT-009 | implementation handoff document is consistent with root dependency law, runtime phase law, implementation order, package closure, and API contract law | `implementation/STRATUMX_IMPLEMENTATION_HANDOFF.md`, `05_DEPENDENCY_MODEL.md`, `08_EXECUTION_FLOW.md`, `implementation/STRATUMX_IMPLEMENTATION_ORDER.md`, `constitutions/STRATUMX_API_CONTRACT_LAW.md`, `14_ACCEPTANCE_MATRIX.md`, `16_AUDIT_READINESS_MATRIX.md` | pass |
| ROOT-010 | the registered performance proof set is indexed, present, benchmark-gate-complete against crate test/performance budgets, capture-sheet/result-complete, compiler/target-bound, backend-keyed, profile-keyed, mechanically bound across gate rows, baseline rows, and ledger rows where profile-owned binding is required, contradiction-audited against its cited sources of truth, and reproducible under the benchmark protocol | `00_INDEX.md`, `constitutions/STRATUMX_PERFORMANCE_CONSTITUTION.md`, `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md`, `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_PROFILE_COMPOSITION_PROOF.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md`, `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md`, `constitutions/STRATUMX_CRATE_TEST_MATRIX.md`, `constitutions/STRATUMX_CRATE_PERFORMANCE_BUDGETS.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `constitutions/STRATUMX_DIAGNOSTICS_CEILING_LAW.md`, `constitutions/STRATUMX_DEGRADATION_POLICY_LAW.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-011 | the declared hardened affected level set is indexed, present, locally tightened, and explicitly coverage-mapped against execution, memory, streaming, transfer, network, rendering, acoustics, simulation-family, resource-pack, replay, startup wiring, and numeric-governance law; this row proves registered coverage of the hardened set and its cited governing surfaces, not theorem-proof semantic sealing of every local authority doc beyond its cited matrices | `15_EVIDENCE_REGISTRY.md` EVID-ROOT-011 hardened affected coverage matrix | pass |
| ROOT-012 | full implementation package closure exists, including numeric-validator artifact contract, numeric source digest set, and package-contained validator result | `implementation/00_INDEX.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`, `implementation/STRATUMX_NUMERIC_SOURCE_DIGEST_SET.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-013 | profile-specific memory, residency, and network memory separation is explicit and legal | `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `levels/l1-runtime-kernel/runtime/44_PROFILE_MODEL.md` | pass |
| ROOT-014 | the registered constitutional package is indexed, present, explicitly coverage-mapped across package closure surfaces, and package-contained proof artifacts are registered; this row proves constitutional-package coverage closure over the cited matrices and artifacts, not machine theorem proof beyond the recorded package chain | `15_EVIDENCE_REGISTRY.md`, `constitutions/00_INDEX.md`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-015 | the registered root package is indexed, present, explicitly coverage-mapped against constitutional sources of truth and package truth artifacts; this row proves root-package coverage closure over the cited matrices and artifacts, not stronger semantic closure than the recorded package chain actually proves | `15_EVIDENCE_REGISTRY.md` EVID-ROOT-015 root-package coverage matrix, `00_INDEX.md`, `STACK_VERSION`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-016 | stack-version marker is legal and bound into the baseline/capture/validator package | `STACK_VERSION`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-017 | benchmark reproducibility is registered across protocol, benchmark floor ids, backend ids, fixture ids, baseline ids, capture-sheet ids, capture-result ids, compiler/target tuples, power/driver posture capture, and stack binding | `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md`, `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md` | pass |
| ROOT-018 | numeric source-of-truth governance is package-contained and validator-closed for repeated shared constants in the hardened affected set | `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`, `implementation/STRATUMX_NUMERIC_SOURCE_DIGEST_SET.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-019 | acceptance, evidence, and readiness documents are mutually aligned on the same registered coverage claims | `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` | pass |
| ROOT-020 | `L4` startup-owned public export surfaces for external `L5` bind are explicit, narrow, and invalidation-governed | `06_COMMUNICATION_MODEL.md`, `levels/l4-startup/startup/30_COMMUNICATION.md`, `levels/l4-startup/startup/42_RUNTIME_WIRING.md`, `levels/l4-startup/startup/46_L5_BRIDGE_EXPORT_SURFACES.md`, `levels/l4-startup/startup/47_L5_EXPORT_EPOCH_AND_INVALIDATION.md` | pass |
| ROOT-021 | active test execution evidence present with executed test-result artifact | `evidence/tests/engine_test_result_posture_v1.md`, `evidence/tests/engine_test_execution_run_v1.md`, `17_TESTING_MODEL.md`, `18_BUILD_AND_FREEZE_CONDITIONS.md` | pass |
