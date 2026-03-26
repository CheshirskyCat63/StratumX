# 011-engine-world-region

## Objective
Create the canonical `engine_world_region` crate for L-0.05 World Region.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.05-world-region/world-region/engine_world_region/**`
- tests for `engine_world_region`

## Forbidden scope
- canonical docs
- world truth ownership beyond region/chunk substrate
- runtime logic
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.05-world-region/world-region/engine_world_region`

## Canon read first
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/40_REGION_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/41_CHUNK_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/42_DIRTY_REGIONS.md`
- `1.docs/canonical/engine/levels/l-0.05-world-region/world-region/43_REGION_VERSIONING.md`

## Required implementation
Implement the minimal region/chunk substrate:
- region model
- chunk model
- dirty-region tracking
- region versioning
- no full world truth snapshot ownership here

## Dependency law
- depend on `engine_core`, `engine_identity`, `engine_handle`, `engine_world_spatial`
- only add lower-substrate dependencies if canon clearly requires them

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_world_region` crate
- workspace updated to include it
- tests covering region/chunk structure, dirty-region marking, and versioning invariants
- `.codex/reports/011-engine-world-region.md`

## Completion rule
PASS only if the crate compiles, tests pass, and the crate stays at region/chunk substrate level.