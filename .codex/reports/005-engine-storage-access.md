## Task
005-engine-storage-access — Create the canonical `engine_storage_access` crate for L-0.6 Storage Access.

## Scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access/**`
- tests for `engine_storage_access`

## Canon read
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/40_READ_VIEWS.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/41_WRITE_WINDOWS.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/42_ACCESS_MODES.md`
- `1.docs/canonical/engine/levels/l-0.6-storage-access/storage-access/43_TRAVERSAL_ENTRY.md`

## Changes made
- Added `engine_storage_access` crate with deterministic access descriptors and legality checks.
- Implemented read views, write window descriptors, access modes, and traversal entry descriptors.
- Added explicit legality validation for access modes, write-window bounds, and traversal-entry constraints.
- Added unit and integration tests for read views, write windows, access mode legality, and traversal invariants.
- Added workspace member wiring for `engine_storage_access`.

## Files changed
- `Cargo.toml`
- `Cargo.lock`
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access/Cargo.toml`
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access/src/lib.rs`
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access/tests/access_models.rs`
- `.codex/reports/005-engine-storage-access.md`

## Commands run
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Test results
- unit: pass (`engine_storage_access` unit tests in `src/lib.rs`)
- integration: pass (`engine_storage_access` integration tests in `tests/access_models.rs`)
- smoke: pass (`bash ./scripts/smoke-all.sh`)
- determinism: pass (validation functions are pure and deterministic)
- benchmarks: pass (`bash ./scripts/bench-all.sh` completed with placeholder benchmark output)

## Metrics
- build time: N/A
- startup ms: N/A
- fixed tick mean ms: N/A
- fixed tick p95 ms: N/A
- peak rss mb: N/A
- allocs per tick: N/A
- binary size: N/A

## Known limitations
- `engine_identity` crate is still not available as a standalone crate in current repository phase.
- Bench and metrics scripts currently emit placeholder values.

## Blockers
- None.
- None.

## Completion status
PASS
