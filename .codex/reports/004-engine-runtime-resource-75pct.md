# Task Report

## Task
004-engine-runtime-resource-75pct (L0.5 through L1.5 runtime middle stack)

## Scope
engine package only: workspace manifests + engine_material + engine_runtime + engine_runtime_headless + engine_runtime_realtime + engine_stream_control + engine_residency_control + engine_memory_control + engine_transfer_control

## Canon read
- AGENTS.md
- .codex/mission.md
- .codex/rules.md
- .codex/acceptance.md
- .codex/report-template.md
- .codex/tasks/001-engine-core-foundation.md
- 1.docs/canonical/00_INDEX.md
- 1.docs/canonical/03_PACKAGE_ROLE_MAP.md
- 1.docs/canonical/engine/00_INDEX.md
- 1.docs/canonical/engine/01_SCOPE.md
- 1.docs/canonical/engine/02_CANONICAL_STACK.md
- 1.docs/canonical/engine/03_ROLE_MAP.md
- 1.docs/canonical/engine/04_LIBRARY_BASELINE.md
- 1.docs/canonical/engine/05_DEPENDENCY_MODEL.md
- 1.docs/canonical/engine/07_THREADING_MODEL.md
- 1.docs/canonical/engine/08_EXECUTION_FLOW.md
- 1.docs/canonical/engine/17_TESTING_MODEL.md
- 1.docs/canonical/engine/25_IMPLEMENTATION_HANDOFF.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/00_LEVEL.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/00_LAYER.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/31_THREADING.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/40_MATERIAL_DESCRIPTORS.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/41_PROPERTY_DOMAINS.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/42_RESPONSE_PROFILES.md
- 1.docs/canonical/engine/levels/l0.5-shared-world-properties/material/43_LOOKUP_MODEL.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/00_LEVEL.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/00_LAYER.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/31_THREADING.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/40_LIFECYCLE.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/41_PHASE_MODEL.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/42_SCHEDULER.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/43_APPLY_MODEL.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/44_PROFILE_MODEL.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/45_RESOURCE_COORDINATION.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/46_LOW_LATENCY_FRAME_PATH.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime/47_QUEUE_OWNERSHIP.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/00_LAYER.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/31_THREADING.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/40_HEADLESS_PROFILE.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/41_SIMULATION_CADENCE.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/42_HEADLESS_OUTPUTS.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-headless/43_HEADLESS_ROLE_MODEL.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/00_LAYER.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/31_THREADING.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/40_REALTIME_PROFILE.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/41_FRAME_CADENCE.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/42_REALTIME_OUTPUTS.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/43_PRESENTATION_POLICY.md
- 1.docs/canonical/engine/levels/l1-runtime-kernel/runtime-realtime/44_REALTIME_ROLE_MODEL.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/00_LEVEL.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/00_LAYER.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/31_THREADING.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/40_STREAM_ACTIVATION.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/41_PREFETCH_AND_EVICTION.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/stream-control/42_IO_SCHEDULING.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/00_LAYER.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/31_THREADING.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/40_RESIDENCY_SETS.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/41_REFCOUNT_AND_BUDGETS.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/residency-control/42_PRESSURE_SIGNALS.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/00_LAYER.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/31_THREADING.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/40_ALLOCATOR_CLASSES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/41_LIFETIME_MODEL.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/memory-control/42_PRESSURE_RESPONSE.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/00_LAYER.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/31_THREADING.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/40_DECODE_STAGE.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/41_STAGING_AND_UPLOAD.md
- 1.docs/canonical/engine/levels/l1.5-runtime-resource-services/transfer-control/42_FENCE_RELEASE_MODEL.md

## Changes made
- Added workspace members and canonical dependency baselines for runtime-middle crates.
- Implemented `engine_material` with descriptor/profile/domain substrate, bounded lookup rows, and deterministic fallback behavior.
- Implemented `engine_runtime` with lifecycle states, canonical phase order, runtime profile model, apply queue ownership, and bounded transfer/connection/present queues.
- Implemented `engine_runtime_headless` with headless role model and 20 Hz simulation cadence output surface.
- Implemented `engine_runtime_realtime` with realtime role/presentation policy and low-latency presentable-frame stepping.
- Implemented `engine_stream_control` with immediate/predicted/maintenance activation classes, locality validation, queue ceilings, and scheduling priority.
- Implemented `engine_residency_control` with hot/warm/cold/quarantined set model, refcount+age transitions, quarantine release law, and pressure signal surface.
- Implemented `engine_memory_control` with canonical allocator classes, profile ceilings, usage accounting, and ordered pressure ladder.
- Implemented `engine_transfer_control` with decode legality gate, upload-byte ceilings, fence-visibility release flow, and completion queue ceiling.
- Added crate-level tests for each new crate’s public behavior.

## Files changed
- Cargo.toml
- Cargo.lock
- 2.engine/l0.5-shared-world-properties/material/engine_material/Cargo.toml
- 2.engine/l0.5-shared-world-properties/material/engine_material/src/lib.rs
- 2.engine/l0.5-shared-world-properties/material/engine_material/tests/material.rs
- 2.engine/l1-runtime-kernel/runtime/engine_runtime/Cargo.toml
- 2.engine/l1-runtime-kernel/runtime/engine_runtime/src/lib.rs
- 2.engine/l1-runtime-kernel/runtime/engine_runtime/tests/runtime.rs
- 2.engine/l1-runtime-kernel/runtime-headless/engine_runtime_headless/Cargo.toml
- 2.engine/l1-runtime-kernel/runtime-headless/engine_runtime_headless/src/lib.rs
- 2.engine/l1-runtime-kernel/runtime-headless/engine_runtime_headless/tests/headless.rs
- 2.engine/l1-runtime-kernel/runtime-realtime/engine_runtime_realtime/Cargo.toml
- 2.engine/l1-runtime-kernel/runtime-realtime/engine_runtime_realtime/src/lib.rs
- 2.engine/l1-runtime-kernel/runtime-realtime/engine_runtime_realtime/tests/realtime.rs
- 2.engine/l1.5-runtime-resource-services/stream-control/engine_stream_control/Cargo.toml
- 2.engine/l1.5-runtime-resource-services/stream-control/engine_stream_control/src/lib.rs
- 2.engine/l1.5-runtime-resource-services/stream-control/engine_stream_control/tests/stream.rs
- 2.engine/l1.5-runtime-resource-services/residency-control/engine_residency_control/Cargo.toml
- 2.engine/l1.5-runtime-resource-services/residency-control/engine_residency_control/src/lib.rs
- 2.engine/l1.5-runtime-resource-services/residency-control/engine_residency_control/tests/residency.rs
- 2.engine/l1.5-runtime-resource-services/memory-control/engine_memory_control/Cargo.toml
- 2.engine/l1.5-runtime-resource-services/memory-control/engine_memory_control/src/lib.rs
- 2.engine/l1.5-runtime-resource-services/memory-control/engine_memory_control/tests/memory.rs
- 2.engine/l1.5-runtime-resource-services/transfer-control/engine_transfer_control/Cargo.toml
- 2.engine/l1.5-runtime-resource-services/transfer-control/engine_transfer_control/src/lib.rs
- 2.engine/l1.5-runtime-resource-services/transfer-control/engine_transfer_control/tests/transfer.rs
- .codex/reports/004-engine-runtime-resource-75pct.md

## Commands run
- bash ./scripts/setup-dev.sh
- bash ./scripts/test-all.sh
- bash ./scripts/smoke-all.sh
- bash ./scripts/bench-all.sh
- bash ./scripts/metrics-all.sh

## Test results
- unit: pass (`cargo test --workspace --all-features` via `scripts/test-all.sh`)
- integration: pass (new integration tests for material/runtime/headless/realtime/stream/residency/memory/transfer crates)
- smoke: pass (`scripts/smoke-all.sh` placeholder command completed)
- determinism: pass (deterministic fallback/phase/order and quarantine-release invariants covered in tests)
- benchmarks: pass (`scripts/bench-all.sh` placeholder command completed)

## Metrics
- build time: N/A
- startup ms: N/A
- fixed tick mean ms: N/A
- fixed tick p95 ms: N/A
- peak rss mb: N/A
- allocs per tick: N/A
- binary size: N/A

## Known limitations
- Smoke and benchmark scripts are still repository placeholders and do not execute real runtime loop workloads yet.
- Numeric constitution values not explicitly present in local code docs are represented as conservative constants and queue ceilings from layer fields.

## Blockers
- None

## Completion status
PASS
