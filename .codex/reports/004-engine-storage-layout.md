## Task
004-engine-storage-layout — Create the canonical `engine_storage_layout` crate for L-0.7 Storage Layout.

## Scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout/**`
- tests for `engine_storage_layout`

## Canon read
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/40_LAYOUT_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/41_CHUNK_LAYOUT.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/42_COLUMN_LAYOUT.md`
- `1.docs/canonical/engine/levels/l-0.7-storage-layout/storage-layout/43_LOCALITY_MODEL.md`

## Changes made
- Added `engine_storage_layout` crate with descriptor-only layout substrate.
- Implemented chunk layout descriptors with frozen-plan invariant validation.
- Implemented column layout descriptors with per-column validation and duplicate-component rejection.
- Implemented locality shape descriptors for cache/spatial/partition/traversal-lane locality classes.
- Added unit and integration tests for descriptor legality and shape invariants.
- Added workspace member wiring for `engine_storage_layout`.

## Files changed
- `Cargo.toml`
- `Cargo.lock`
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout/Cargo.toml`
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout/src/lib.rs`
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout/tests/layout_descriptors.rs`
- `.codex/reports/004-engine-storage-layout.md`

## Commands run
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Test results
- unit: pass (`engine_storage_layout` unit tests in `src/lib.rs`)
- integration: pass (`engine_storage_layout` integration tests in `tests/layout_descriptors.rs`)
- smoke: pass (`bash ./scripts/smoke-all.sh`)
- determinism: pass (descriptor validation logic is pure and deterministic; invariants covered by tests)
- benchmarks: pass (`bash ./scripts/bench-all.sh` completes and emits placeholder benchmark output)

## Metrics
- build time: N/A
- startup ms: N/A
- fixed tick mean ms: N/A
- fixed tick p95 ms: N/A
- peak rss mb: N/A
- allocs per tick: N/A
- binary size: N/A

## Known limitations
- `engine_identity` remains unavailable as a standalone crate in this repository phase.
- Benchmark and metrics scripts currently emit placeholder values.

## Blockers
- None.
- None.

## Completion status
PASS
