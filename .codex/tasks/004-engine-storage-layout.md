# 004-engine-storage-layout

## Objective
Create the canonical `engine_storage_layout` crate for L-0.7 Storage Layout.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout/**`
- tests for `engine_storage_layout`

## Forbidden scope
- canonical docs
- storage access/mutation behavior beyond what layout needs
- ecs/world/runtime code
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout`

## Canon read first
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

## Required implementation
Implement the minimal storage layout substrate:
- chunk layout descriptors
- column layout descriptors
- locality-oriented shape descriptors
- no read/write policy here beyond what layout metadata requires

## Dependency law
- depend only on `engine_core`, `engine_identity`, and `engine_handle` if canon requires it
- do not implement access or mutation APIs in this crate

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_storage_layout` crate
- workspace updated to include it
- tests covering layout descriptors, chunk/column shape invariants, and locality model basics
- `.codex/reports/004-engine-storage-layout.md`

## Completion rule
PASS only if the crate compiles, tests pass, and scope stayed inside storage layout.