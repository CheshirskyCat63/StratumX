# 005-engine-storage-access

## Objective
Create the canonical `engine_storage_access` crate for L-0.6 Storage Access.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access/**`
- tests for `engine_storage_access`

## Forbidden scope
- canonical docs
- mutation staging logic
- ecs/world/runtime code
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.6-storage-access/storage-access/engine_storage_access`

## Canon read first
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

## Required implementation
Implement the minimal deterministic access substrate:
- read views
- write window descriptors
- access mode model
- traversal/access entry descriptors
- explicit access legality checks

## Dependency law
- depend on `engine_core`, `engine_identity`, `engine_handle`, `engine_storage_layout`
- do not implement mutation buffering here

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_storage_access` crate
- workspace updated to include it
- tests covering read views, write windows, access mode legality, and traversal entry invariants
- `.codex/reports/005-engine-storage-access.md`

## Completion rule
PASS only if the crate compiles, tests pass, and no mutation semantics leaked into access.