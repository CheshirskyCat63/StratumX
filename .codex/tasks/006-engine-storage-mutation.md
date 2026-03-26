# 006-engine-storage-mutation

## Objective
Create the canonical `engine_storage_mutation` crate for L-0.5 Storage Mutation.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation/**`
- tests for `engine_storage_mutation`

## Forbidden scope
- canonical docs
- ecs/world/runtime code
- direct world ownership logic
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation`

## Canon read first
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

## Required implementation
Implement the minimal mutation substrate:
- mutation buffers
- deferred write model
- change set packaging
- apply payload model
- explicit staged/non-applied semantics

## Dependency law
- depend on `engine_core`, `engine_identity`, `engine_handle`, `engine_storage_layout`, `engine_storage_access`
- do not apply to world directly here

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_storage_mutation` crate
- workspace updated to include it
- tests covering buffer staging, deferred write semantics, change set formation, and apply payload validity
- `.codex/reports/006-engine-storage-mutation.md`

## Completion rule
PASS only if the crate compiles, tests pass, and direct world-ownership logic is absent.