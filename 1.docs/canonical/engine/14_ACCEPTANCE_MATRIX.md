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
- declared hardened affected-level contradiction-audit coverage and local-tightening registration over the registered affected set;
- full implementation package closure including numeric-validator artifact contract and package-contained validator result;
- profile-specific memory and residency separation;
- registered constitutional package contradiction-audit coverage and package-truth registration;
- registered root package contradiction-audit coverage and package-truth registration;
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
| ROOT-002 | root dependency model matches local dependency baselines for runtime-resource, critical-simulation, and network-runtime crates | `05_DEPENDENCY_MODEL.md`, `levels/l1.5-runtime-resource-services/**/20_DEPENDENCIES.md`, `levels/l2-critical-simulation/**/20_DEPENDENCIES.md`, `levels/l2.5-network-runtime-services/**/20_DEPENDENCIES.md` | pass |
| ROOT-003 | runtime phase order is closed and identical across root, constitution, and local runtime docs | `08_EXECUTION_FLOW.md`, `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `levels/l1-runtime-kernel/runtime/41_PHASE_MODEL.md` | pass |
| ROOT-004 | canonical repository shape, package layout, and implementation package index are aligned | `11_PACKAGE_LAYOUT.md`, `constitutions/STRATUMX_WORKSPACE_AND_REPOSITORY_CANON.md`, `implementation/00_INDEX.md` | pass |
| ROOT-005 | `engine_content` relation to `L1.5` is defined as startup-mediated product flow, not upward crate dependency | `05_DEPENDENCY_MODEL.md`, `06_COMMUNICATION_MODEL.md`, `08_EXECUTION_FLOW.md`, `levels/l1.5-runtime-resource-services/00_LEVEL.md`, `levels/l3.2-resource-systems/content/00_LAYER.md`, `levels/l4-startup/startup/42_RUNTIME_WIRING.md` | pass |
| ROOT-006 | canonical implementation order is legal against the canonical crate dependency graph, including intra-phase order | `05_DEPENDENCY_MODEL.md`, `implementation/STRATUMX_IMPLEMENTATION_ORDER.md` | pass |
| ROOT-007 | constitutional API law and implementation-facing crate contract skeletons are legal and mutually aligned across all crates | `constitutions/STRATUMX_API_CONTRACT_LAW.md`, `implementation/STRATUMX_CRATE_CONTRACT_SKELETONS.md` | pass |
| ROOT-008 | package-level acceptance, evidence, and readiness closure documents exist and are indexed | `00_INDEX.md`, `11_PACKAGE_LAYOUT.md`, `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` | pass |
| ROOT-009 | implementation handoff document is consistent with root dependency law, runtime phase law, implementation order, package closure, and API contract law | `implementation/STRATUMX_IMPLEMENTATION_HANDOFF.md`, `05_DEPENDENCY_MODEL.md`, `08_EXECUTION_FLOW.md`, `implementation/STRATUMX_IMPLEMENTATION_ORDER.md`, `constitutions/STRATUMX_API_CONTRACT_LAW.md`, `14_ACCEPTANCE_MATRIX.md`, `16_AUDIT_READINESS_MATRIX.md` | pass |
| ROOT-010 | the registered performance proof set is indexed, present, benchmark-gate-complete against crate test/performance budgets, capture-sheet/result-complete, compiler/target-bound, backend-keyed, profile-keyed, mechanically bound across gate rows, baseline rows, and ledger rows where profile-owned binding is required, contradiction-audited against its cited sources of truth, and reproducible under the benchmark protocol | `00_INDEX.md`, `constitutions/STRATUMX_PERFORMANCE_CONSTITUTION.md`, `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md`, `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_PROFILE_COMPOSITION_PROOF.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md`, `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md`, `constitutions/STRATUMX_CRATE_TEST_MATRIX.md`, `constitutions/STRATUMX_CRATE_PERFORMANCE_BUDGETS.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `constitutions/STRATUMX_DIAGNOSTICS_CEILING_LAW.md`, `constitutions/STRATUMX_DEGRADATION_POLICY_LAW.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-011 | the declared hardened affected level set is indexed, present, locally tightened, and covered by contradiction-audit evidence against execution, memory, streaming, transfer, network, rendering, acoustics, simulation-family, resource-pack, replay, startup wiring, and numeric-governance law; this row proves registered coverage of the hardened set, not theorem-proof semantic sealing of every local authority doc beyond its cited matrices | `levels/l-0.3-ecs-query/ecs-query/40_QUERY_DESCRIPTORS.md`, `levels/l-0.6-storage-access/storage-access/43_TRAVERSAL_ENTRY.md`, `levels/l0.5-shared-world-properties/material/43_LOOKUP_MODEL.md`, `levels/l1-runtime-kernel/runtime/45_RESOURCE_COORDINATION.md`, `levels/l1-runtime-kernel/runtime/46_LOW_LATENCY_FRAME_PATH.md`, `levels/l1-runtime-kernel/runtime-headless/4*.md`, `levels/l1-runtime-kernel/runtime-realtime/4*.md`, `levels/l2.5-network-runtime-services/net-transport/4*.md`, `levels/l2-critical-simulation/kinetics/4*.md`, `levels/l2-critical-simulation/field/4*.md`, `levels/l2-critical-simulation/agents/4*.md`, `levels/l3.1-synthesis-systems/imaging/{44_RESOURCE_RESIDENCY.md,45_UPLOAD_STAGING.md,46_FRAME_RESOURCE_POLICY.md}`, `levels/l3.1-synthesis-systems/acoustics/{40_PROPAGATION.md,41_REFLECTION_AND_OCCLUSION.md,43_ACOUSTIC_OUTPUTS.md}`, `levels/l4-startup/startup/{41_PROFILE_SELECTION.md,42_RUNTIME_WIRING.md,44_NETWORK_ROLE_SELECTION.md,45_RESOURCE_SERVICE_WIRING.md}` | pass |
| ROOT-012 | full implementation package closure exists, including numeric-validator artifact contract and package-contained validator result | `implementation/00_INDEX.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-013 | profile-specific memory, residency, and network memory separation is explicit and legal | `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `levels/l1-runtime-kernel/runtime/44_PROFILE_MODEL.md` | pass |
| ROOT-014 | the registered constitutional package is indexed, present, contradiction-audit-covered against its cited sources of truth, and package-contained proof artifacts are registered; this row proves constitutional-package coverage closure, not machine theorem proof beyond the explicitly cited constitutional registration and proof-artifact surfaces | `00_INDEX.md`, `constitutions/00_INDEX.md`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-015 | the registered root package is indexed, present, contradiction-audit-covered against constitutional sources of truth and package truth artifacts; this row proves root-package coverage closure, not stronger semantic closure than the cited proof chain actually records | `00_INDEX.md`, `STACK_VERSION`, root docs `01..16`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-016 | stack-version marker is legal and bound into the baseline/capture/validator package | `STACK_VERSION`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-017 | benchmark reproducibility is registered across protocol, benchmark floor ids, backend ids, fixture ids, baseline ids, capture-sheet ids, capture-result ids, compiler/target tuples, power/driver posture capture, and stack binding | `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md`, `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md` | pass |
| ROOT-018 | numeric source-of-truth governance is package-contained and validator-closed for repeated shared constants in the hardened affected set | `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` | pass |
| ROOT-019 | acceptance, evidence, and readiness documents are mutually aligned on the same registered coverage claims | `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` | pass |
