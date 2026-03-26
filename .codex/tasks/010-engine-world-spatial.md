# 010-engine-world-spatial

## Objective
Create the canonical `engine_world_spatial` crate for L-0.1 World Spatial.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial/**`
- tests for `engine_world_spatial`

## Forbidden scope
- canonical docs
- world truth ownership
- runtime logic
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial`

## Canon read first
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/40_COORDINATES.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/41_TRANSFORMS.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/42_SPATIAL_ADDRESSING.md`
- `1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/43_SPATIAL_RELATIONS.md`

## Required implementation
Implement the minimal world-spatial substrate:
- coordinate primitives
- transform primitives
- spatial addressing primitives
- spatial relation helpers
- no region/chunk ownership here beyond what the canon requires

## Dependency law
- depend on lower substrate crates only
- do not implement world truth state here

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_world_spatial` crate
- workspace updated to include it
- tests covering coordinates, transforms, addressing, and relation invariants
- `.codex/reports/010-engine-world-spatial.md`

## Completion rule
PASS only if the crate compiles, tests pass, and the crate remains a spatial substrate only.