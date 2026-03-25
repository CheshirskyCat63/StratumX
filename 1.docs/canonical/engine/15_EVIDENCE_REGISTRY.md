# Evidence Registry

## Purpose

This document records the evidence chain for package-level closure.
It also freezes the explicit package coverage matrices that support `ROOT-011`, `ROOT-014`, and `ROOT-015`.

## Registry

| Evidence ID | Proves | Evidence Sources |
|---|---|---|
| EVID-ROOT-001 | stack/role/glossary alignment | `02_CANONICAL_STACK.md`, `03_ROLE_MAP.md`, `09_GLOSSARY.md` |
| EVID-ROOT-002 | dependency-model closure across `L1.5`, `L2`, and `L2.5` | `05_DEPENDENCY_MODEL.md`, local dependency docs |
| EVID-ROOT-003 | runtime phase closure | `08_EXECUTION_FLOW.md`, `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `levels/l1-runtime-kernel/runtime/41_PHASE_MODEL.md` |
| EVID-ROOT-004 | repository/package alignment | `11_PACKAGE_LAYOUT.md`, `constitutions/STRATUMX_WORKSPACE_AND_REPOSITORY_CANON.md`, `implementation/00_INDEX.md` |
| EVID-ROOT-005 | startup-mediated content/runtime-pack relation | `05_DEPENDENCY_MODEL.md`, `06_COMMUNICATION_MODEL.md`, `levels/l4-startup/startup/42_RUNTIME_WIRING.md` |
| EVID-ROOT-006 | implementation-order legality against dependency graph, including intra-phase ordering law | `05_DEPENDENCY_MODEL.md`, `implementation/STRATUMX_IMPLEMENTATION_ORDER.md` |
| EVID-ROOT-007 | API-law to crate-skeleton legality across all canonical crates | `constitutions/STRATUMX_API_CONTRACT_LAW.md`, `implementation/STRATUMX_CRATE_CONTRACT_SKELETONS.md` |
| EVID-ROOT-008 | package closure documents and index/package registration | `00_INDEX.md`, `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` |
| EVID-ROOT-009 | implementation handoff closure against root, constitutional, implementation, and package-closure law | `implementation/STRATUMX_IMPLEMENTATION_HANDOFF.md`, `14_ACCEPTANCE_MATRIX.md`, `16_AUDIT_READINESS_MATRIX.md` |
| EVID-ROOT-010 | registered performance proof coverage across performance constitution, benchmark protocol, benchmark floors, fixture corpus, backend-keyed locked baselines, capture sheets, package-contained capture results, mechanical profile-composition proof, engine budget ledger, gate matrix, crate test/performance budget closure, numeric source, numeric validator rule/result, diagnostics ceiling law, and degradation law | `constitutions/STRATUMX_PERFORMANCE_CONSTITUTION.md`, `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md`, `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_PROFILE_COMPOSITION_PROOF.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md`, `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md`, `constitutions/STRATUMX_CRATE_TEST_MATRIX.md`, `constitutions/STRATUMX_CRATE_PERFORMANCE_BUDGETS.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `constitutions/STRATUMX_DIAGNOSTICS_CEILING_LAW.md`, `constitutions/STRATUMX_DEGRADATION_POLICY_LAW.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` |
| EVID-ROOT-011 | declared hardened affected-level coverage and local-tightening registration over the registered affected set, resolved through the explicit file-level coverage matrix below | `14_ACCEPTANCE_MATRIX.md` ROOT-011 evidence set, explicit hardened affected coverage matrix below, and all listed affected local docs |
| EVID-ROOT-012 | full implementation package closure including numeric-validator artifact contract, numeric source digest set, and package-contained validator result | `implementation/00_INDEX.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`, `implementation/STRATUMX_NUMERIC_SOURCE_DIGEST_SET.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` |
| EVID-ROOT-013 | profile-specific memory, residency, and network memory separation | `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` |
| EVID-ROOT-014 | registered constitutional package coverage and package-truth registration through the explicit constitutional coverage matrix below plus package-contained proof artifacts | `constitutions/00_INDEX.md`, explicit constitutional coverage matrix below, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` |
| EVID-ROOT-015 | registered root package coverage and package-truth registration through the explicit root-package coverage matrix below plus package truth artifacts | `00_INDEX.md`, explicit root-package coverage matrix below, `STACK_VERSION`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` |
| EVID-ROOT-016 | stack-version marker legality and baseline-table binding | `STACK_VERSION`, `constitutions/STRATUMX_STACK_VERSION_SOURCE.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` |
| EVID-ROOT-017 | benchmark reproducibility coverage by protocol, benchmark floor ids, backend ids, fixture ids, baseline ids, capture-sheet ids, capture-result ids, compiler/target tuples, power/driver posture capture, and stack binding | `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md`, `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`, `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md`, `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md`, `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md`, `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md` |
| EVID-ROOT-018 | numeric source-of-truth governance and validator closure for repeated shared constants in the hardened affected set, including the package-contained validator artifact, source digest set, and validator result execution output | `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`, `implementation/STRATUMX_NUMERIC_SOURCE_DIGEST_SET.md`, `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` |
| EVID-ROOT-019 | acceptance/evidence/readiness alignment on the same registered coverage claims | `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` |

## Explicit Coverage Matrices

### EVID-ROOT-011 Hardened Affected Coverage Matrix

| Local authority doc | Governing constitutional surfaces | Coverage basis |
|---|---|---|
| `levels/l-0.3-ecs-query/ecs-query/40_QUERY_DESCRIPTORS.md` | `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; lower-stack numeric and traversal constants bound |
| `levels/l-0.6-storage-access/storage-access/43_TRAVERSAL_ENTRY.md` | `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; traversal-entry numeric constants bound |
| `levels/l0.5-shared-world-properties/material/43_LOOKUP_MODEL.md` | `constitutions/STRATUMX_DATA_AND_STATE_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; shared lookup model bound to numeric authority |
| `levels/l1-runtime-kernel/runtime/45_RESOURCE_COORDINATION.md` | `constitutions/STRATUMX_STREAMING_AND_PACK_LOCALITY_LAW.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` | validator-scanned; runtime resource coordination tied to streaming/residency law |
| `levels/l1-runtime-kernel/runtime/46_LOW_LATENCY_FRAME_PATH.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md`, `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` | validator-scanned; low-latency path checked against execution/traversal/render law |
| `levels/l1-runtime-kernel/runtime-headless/40_HEADLESS_PROFILE.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; headless profile envelope bound to execution and residency law |
| `levels/l1-runtime-kernel/runtime-headless/41_SIMULATION_CADENCE.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; headless cadence constants tied to execution and numeric authority |
| `levels/l1-runtime-kernel/runtime-headless/42_HEADLESS_OUTPUTS.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_OBSERVABILITY_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; output cadence and retention constants bound |
| `levels/l1-runtime-kernel/runtime-headless/43_HEADLESS_ROLE_MODEL.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; role-model numeric bounds tied to constitutional runtime law |
| `levels/l1-runtime-kernel/runtime-realtime/40_REALTIME_PROFILE.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; realtime profile envelope bound to execution and residency law |
| `levels/l1-runtime-kernel/runtime-realtime/41_FRAME_CADENCE.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; realtime cadence tied to render/runtime law |
| `levels/l1-runtime-kernel/runtime-realtime/42_REALTIME_OUTPUTS.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_OBSERVABILITY_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; realtime output constraints bound |
| `levels/l1-runtime-kernel/runtime-realtime/43_PRESENTATION_POLICY.md` | `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md`, `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; presentation policy numeric thresholds bound |
| `levels/l1-runtime-kernel/runtime-realtime/44_REALTIME_ROLE_MODEL.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; realtime role limits bound |
| `levels/l2.5-network-runtime-services/net-transport/40_CONNECTION_MODEL.md` | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; network connection constants bound |
| `levels/l2.5-network-runtime-services/net-transport/41_PACKET_LANES.md` | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; packet-lane budgets and classes bound |
| `levels/l2.5-network-runtime-services/net-transport/42_SECURITY_AND_SESSION.md` | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; session timing constants bound |
| `levels/l2-critical-simulation/kinetics/40_COLLISION.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; collision thresholds and bridge limits bound |
| `levels/l2-critical-simulation/kinetics/41_RIGIDBODY.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; rigidbody numeric limits bound |
| `levels/l2-critical-simulation/kinetics/42_TRAJECTORY.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; trajectory constants bound |
| `levels/l2-critical-simulation/kinetics/43_IMPACT.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; impact thresholds bound |
| `levels/l2-critical-simulation/field/40_FLUID_FIELD.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; fluid-field constants bound |
| `levels/l2-critical-simulation/field/41_THERMAL_FIELD.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; thermal-field constants bound |
| `levels/l2-critical-simulation/field/42_TERRAIN_FIELD.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; terrain-field constants bound |
| `levels/l2-critical-simulation/field/43_STRUCTURAL_FIELD.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; structural-field constants bound |
| `levels/l2-critical-simulation/field/44_ATMOSPHERIC_FIELD.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; atmospheric-field constants bound |
| `levels/l2-critical-simulation/agents/40_NAVIGATION.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; navigation constants bound |
| `levels/l2-critical-simulation/agents/41_PERCEPTION.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; perception constants bound |
| `levels/l2-critical-simulation/agents/42_DECISION.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; decision constants bound |
| `levels/l2-critical-simulation/agents/43_SOCIETY.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; society constants bound |
| `levels/l2-critical-simulation/agents/44_SCHEDULE.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; schedule constants bound |
| `levels/l3.1-synthesis-systems/imaging/44_RESOURCE_RESIDENCY.md` | `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; imaging residency constants bound |
| `levels/l3.1-synthesis-systems/imaging/45_UPLOAD_STAGING.md` | `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; upload staging limits bound |
| `levels/l3.1-synthesis-systems/imaging/46_FRAME_RESOURCE_POLICY.md` | `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; frame-resource policy limits bound |
| `levels/l3.1-synthesis-systems/acoustics/40_PROPAGATION.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; acoustics propagation constants bound |
| `levels/l3.1-synthesis-systems/acoustics/41_REFLECTION_AND_OCCLUSION.md` | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; acoustics reflection constants bound |
| `levels/l3.1-synthesis-systems/acoustics/43_ACOUSTIC_OUTPUTS.md` | `constitutions/STRATUMX_OBSERVABILITY_CONSTITUTION.md`, `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; acoustic output limits bound |
| `levels/l4-startup/startup/41_PROFILE_SELECTION.md` | `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; profile selection constants bound |
| `levels/l4-startup/startup/42_RUNTIME_WIRING.md` | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_STREAMING_AND_PACK_LOCALITY_LAW.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md` | validator-scanned; runtime wiring tied to execution/streaming/memory law |
| `levels/l4-startup/startup/44_NETWORK_ROLE_SELECTION.md` | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`, `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`, `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | validator-scanned; network role bounds tied to constitutional law |
| `levels/l4-startup/startup/45_RESOURCE_SERVICE_WIRING.md` | `constitutions/STRATUMX_STREAMING_AND_PACK_LOCALITY_LAW.md`, `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` | validator-scanned; resource wiring tied to residency/transfer law |

### EVID-ROOT-014 Constitutional Package Coverage Matrix

| Constitutional doc | Package truth role | Covered by registered rows |
|---|---|---|
| `constitutions/00_INDEX.md` | constitutional index and registration anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md` | absolute budget ceiling law | `ROOT-010`, `ROOT-018` |
| `constitutions/STRATUMX_API_CONTRACT_LAW.md` | constitutional package truth anchor | `ROOT-007`, `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_BASELINE_CAPTURE_RESULTS.md` | capture-result registry anchor | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_BASELINE_CAPTURE_SHEETS.md` | capture-sheet registry anchor | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_BENCHMARK_FIXTURE_CORPUS.md` | fixture registry anchor | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md` | benchmark reproducibility governing law | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_CANONICAL_SHAPES.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_CONFIGURATION_CONSTITUTION.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_CONSTITUTION_PRECEDENCE_RULE.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_CRATE_PERFORMANCE_BUDGETS.md` | crate performance budget source | `ROOT-010` |
| `constitutions/STRATUMX_CRATE_TEST_MATRIX.md` | constitutional package truth anchor | `ROOT-010`, `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_CROSS_FAMILY_SOLVE_ORDER.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_DATA_AND_STATE_CONSTITUTION.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_DEGRADATION_POLICY_LAW.md` | degradation legality anchor | `ROOT-010` |
| `constitutions/STRATUMX_DIAGNOSTICS_CEILING_LAW.md` | diagnostics hard-ceiling law | `ROOT-010` |
| `constitutions/STRATUMX_ENABLEMENT_LEGALITY_MATRIX.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_ENGINE_BUDGET_LEDGER.md` | budget-ledger binding anchor | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md` | governing law for hardened affected local docs | `ROOT-011`, `ROOT-003` |
| `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md` | hardware-floor truth anchor | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_LAYER_TEST_MATRIX.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md` | locked baseline registry anchor | `ROOT-010`, `ROOT-017` |
| `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md` | governing law for hardened affected local docs | `ROOT-011`, `ROOT-013` |
| `constitutions/STRATUMX_NAMING_AND_TERMINOLOGY_FREEZE.md` | constitutional package truth anchor | `ROOT-001`, `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` | governing law for hardened affected local docs | `ROOT-011`, `ROOT-013` |
| `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` | numeric-governance source | `ROOT-010`, `ROOT-018` |
| `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | numeric-governance validation law | `ROOT-010`, `ROOT-018` |
| `constitutions/STRATUMX_OBSERVABILITY_CONSTITUTION.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_PERFORMANCE_CONSTITUTION.md` | performance proof governing law | `ROOT-010` |
| `constitutions/STRATUMX_PERFORMANCE_GATE_MATRIX.md` | performance gate registry | `ROOT-010` |
| `constitutions/STRATUMX_PERSISTENCE_AND_SNAPSHOT_CONSTITUTION.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_PERSISTENCE_COMPATIBILITY_LAW.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_PROFILE_COMPOSITION_PROOF.md` | gate/baseline/profile mechanical binding proof | `ROOT-010` |
| `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_REPLAY_AND_DETERMINISM_CONSTITUTION.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_STACK_VERSION_SOURCE.md` | stack-marker source of truth | `ROOT-014`, `ROOT-015`, `ROOT-016` |
| `constitutions/STRATUMX_STREAMING_AND_PACK_LOCALITY_LAW.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_TESTING_CONSTITUTION.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_TEST_SEVERITY_SYSTEM.md` | constitutional package truth anchor | `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_THREADING_CONSTITUTION.md` | governing law for hardened affected local docs | `ROOT-011` |
| `constitutions/STRATUMX_WORKSPACE_AND_REPOSITORY_CANON.md` | constitutional package truth anchor | `ROOT-004`, `ROOT-014`, `ROOT-015` |
| `constitutions/STRATUMX_WORLD_PROGRESSION_FLOW.md` | governing law for hardened affected local docs | `ROOT-011` |

### EVID-ROOT-015 Root Package Coverage Matrix

| Root package doc | Package truth role | Covered by registered rows |
|---|---|---|
| `00_INDEX.md` | package registration and gold-claim boundary | `ROOT-008`, `ROOT-014`, `ROOT-015`, `ROOT-019` |
| `01_SCOPE.md` | root package scope anchor | `ROOT-015` |
| `02_CANONICAL_STACK.md` | stack identity anchor | `ROOT-001`, `ROOT-016` |
| `03_ROLE_MAP.md` | role identity anchor | `ROOT-001` |
| `04_LIBRARY_BASELINE.md` | library baseline root truth | `ROOT-015` |
| `05_DEPENDENCY_MODEL.md` | dependency and startup-mediated flow law | `ROOT-002`, `ROOT-005`, `ROOT-006`, `ROOT-009` |
| `06_COMMUNICATION_MODEL.md` | content/runtime and boundary communication truth | `ROOT-005`, `ROOT-015` |
| `07_THREADING_MODEL.md` | root threading summary bound to constitutional threading law | `ROOT-015` |
| `08_EXECUTION_FLOW.md` | root runtime phase order and handoff anchor | `ROOT-003`, `ROOT-009` |
| `09_GLOSSARY.md` | canonical terminology anchor | `ROOT-001` |
| `10_DOCUMENT_RULES.md` | root document-governance anchor | `ROOT-015` |
| `11_PACKAGE_LAYOUT.md` | package topology and package registration | `ROOT-004`, `ROOT-008` |
| `11_CANON_TRANSITION_MAP.md` | transition-package truth anchor | `ROOT-015` |
| `12_BOUNDARY_PRESERVATION_MATRIX.md` | boundary preservation root anchor | `ROOT-015` |
| `13_ALIAS_AND_RENAME_MAP.md` | rename and alias continuity root anchor | `ROOT-015` |
| `14_ACCEPTANCE_MATRIX.md` | acceptance closure registry | `ROOT-008`, `ROOT-019` |
| `15_EVIDENCE_REGISTRY.md` | evidence-chain registry and explicit coverage matrices | `ROOT-008`, `ROOT-019` |
| `16_AUDIT_READINESS_MATRIX.md` | readiness closure registry | `ROOT-008`, `ROOT-019` |
| `STACK_VERSION` | stack-marker truth artifact | `ROOT-015`, `ROOT-016` |