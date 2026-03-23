# STRATUMX_PERFORMANCE_GATE_MATRIX

## 1. Purpose

This document defines canonical hard-gate and stretch-target classes for performance validation.

## 2. Gate Model

Every benchmark row declares:
- metric family;
- benchmark scope;
- Tier 1 hard gate;
- Tier 2 stretch target;
- failure class on breach;
- block class on breach.

Whole-engine profile rows are explicit proxy gates for:
- `engine_runtime_headless`
- `engine_runtime_realtime`

## 3. Matrix

| Benchmark Scope | Metric Family | Tier 1 Hard Gate | Tier 2 Stretch Target | Failure Class | Block Class |
|---|---|---|---|---|---|
| engine_core hot type operations | allocation | zero steady-state allocations | zero allocations in all canonical loops | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_identity id lookup / creation | latency | <= 3% regression vs locked baseline | <= 1% regression vs locked baseline | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_handle validation | latency | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_storage_layout descriptor assembly | allocation | zero avoidable allocations in steady-state descriptor paths | zero allocations in canonical descriptor loops | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_storage_access read view construction | allocation | zero new avoidable allocations | zero allocations including stress path | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_storage_mutation staging batch assembly | allocation | zero avoidable allocations in steady-state staging | zero allocations in canonical staging loops | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_ecs_registry membership update/read | latency | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_ecs_query traversal | throughput | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_ecs facade overhead | latency | <= 2% regression vs locked baseline | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_world_spatial descriptor and address reads | latency | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_world_region region metadata and dirty tracking | latency | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_world snapshot build | latency | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_world apply batch integration | latency | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_material immutable lookup | latency | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_runtime empty tick | latency | <= 25 microseconds on canonical bench profile | <= 10 microseconds on canonical bench profile | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_runtime minimal tick | latency | <= 50 microseconds | <= 20 microseconds | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_runtime scheduler overhead | latency | <= 2% of total tick cost in locked baseline scenario | <= 1% of total tick cost | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_runtime contention | contention | zero broad locks in compute phases | zero runtime-visible contention in canonical parallel path | FAIL_PERFORMANCE | B5_CRITICAL_STOP |
| engine_runtime_headless whole-profile progression | scaling | parallel efficiency must not fall below 0.80 of locked baseline | parallel efficiency target >= 0.90 | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime whole-profile progression | stability | jitter must not exceed 3% over locked baseline | jitter target <= 1% | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_kinetics batch compute | throughput | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_field regional solve | throughput | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_agents batch cycle | throughput | <= 3% regression | <= 1% regression | FAIL_PERFORMANCE | B4_MERGE_BLOCK |
| engine_inference request normalization | latency | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_generation output normalization | latency | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_imaging extraction baseline | latency | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_acoustics solve baseline | latency | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_content ingest/transform | throughput | <= 5% regression | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_startup legal assembly launch path | latency | <= 5% regression vs locked startup baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |

## 4. Canonical Rule

These gates are intentionally severe. They may be relaxed only through explicit proof and canon update.
