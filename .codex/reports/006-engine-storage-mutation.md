## Task
006-engine-storage-mutation — Create the canonical `engine_storage_mutation` crate for L-0.5 Storage Mutation.

## Scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation/**`
- tests for `engine_storage_mutation`

## Canon read
- `1.docs/canonical/00_INDEX.md`
- `1.docs/canonical/03_PACKAGE_ROLE_MAP.md`
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/40_MUTATION_BUFFERS.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/41_DEFERRED_WRITES.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/42_CHANGE_SETS.md`
- `1.docs/canonical/engine/levels/l-0.5-storage-mutation/storage-mutation/43_APPLY_PAYLOADS.md`

## Changes made
- Added canonical `engine_storage_mutation` crate with staged mutation buffers and deferred write validation.
- Implemented explicit staged/non-applied semantics (`MutationStage`) and validation errors for deferred writes, change sets, and apply payloads.
- Implemented change-set packaging from mutation buffers and apply-payload conversion/validation.
- Added idempotent coalesce behavior for mutation buffers.
- Added unit and integration tests for staging behavior, deferred write semantics, change set formation, and apply payload validity.
- Updated workspace members to include `engine_storage_mutation`.

## Files changed
- `Cargo.toml`
- `Cargo.lock`
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation/Cargo.toml`
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation/src/lib.rs`
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation/tests/mutation_models.rs`
- `.codex/reports/006-engine-storage-mutation.md`

## Commands run
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Test results
- unit: pass (`engine_storage_mutation` unit tests in `src/lib.rs`)
- integration: pass (`engine_storage_mutation` integration tests in `tests/mutation_models.rs`)
- smoke: pass (`bash ./scripts/smoke-all.sh`)
- determinism: pass (staging and packaging logic is pure and deterministic; invariants are test-covered)
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
- `engine_identity` and `engine_storage_access` are currently represented via canonical alias dependencies to existing lower crates in this repository phase.
- Benchmark and metrics scripts currently emit placeholder values.

## Blockers
- None.
- None.

## Completion status
PASS
