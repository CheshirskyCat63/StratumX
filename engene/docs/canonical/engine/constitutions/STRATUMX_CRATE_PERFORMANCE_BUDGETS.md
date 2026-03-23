# STRATUMX_CRATE_PERFORMANCE_BUDGETS

## 1. Purpose

This document defines canonical per-crate budget classes for implementation planning.

## 2. Budget Table

| Crate | Budget Class | Canonical Budget Direction |
|---|---|---|
| engine_core | zero-overhead substrate | zero avoidable allocations; branch-light; cache-friendly |
| engine_identity | ultra-light substrate | constant-time lookup target; low-memory descriptors |
| engine_handle | ultra-light substrate | deterministic validation; no hot-path heap churn |
| engine_storage_layout | descriptor-heavy substrate | no runtime-heavy rebuilds in hot paths |
| engine_storage_access | hot-path substrate | zero avoidable allocations; legal windows only |
| engine_storage_mutation | staging substrate | batch-oriented; merge-light |
| engine_ecs_registry | structural substrate | stable registration cost envelope |
| engine_ecs_query | hot-path substrate | traversal dominates, framework overhead minimized |
| engine_ecs | assembly substrate | façade overhead must remain tiny relative to query and access work |
| engine_world_spatial | spatial substrate | descriptor and addressing overhead tightly bounded |
| engine_world_region | partition substrate | region metadata and dirty tracking overhead tightly bounded |
| engine_world | authoritative owner | snapshot/apply cost tightly bounded |
| engine_material | shared lookup substrate | read-heavy immutable lookups |
| engine_runtime | orchestration kernel | orchestration overhead must remain tiny relative to family compute |
| engine_runtime_headless | profile shell | profile adaptation overhead must remain negligible vs runtime core |
| engine_runtime_realtime | profile shell | profile adaptation overhead must remain negligible vs runtime core |
| engine_kinetics | critical family | cost dominated by real local dynamics, not wrappers |
| engine_field | critical family | cost dominated by field solve, not orchestration |
| engine_agents | critical family | cost dominated by agent work, not dispatch |
| engine_inference | service layer | bounded service overhead with explicit budgets |
| engine_generation | service layer | bounded service overhead with explicit budgets |
| engine_imaging | synthesis layer | extraction/synthesis budgets separated from runtime overhead |
| engine_acoustics | synthesis layer | propagation/synthesis budgets separated from runtime overhead |
| engine_content | resource layer | pipeline cost bounded and optionally off critical cadence |
| engine_startup | startup | launch-time cost explicit and measurable |
