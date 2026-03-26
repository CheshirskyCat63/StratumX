# Task Report

## Task
003-engine-lower-stack-50pct (L-0.4 through L0 engine substrate continuation)

## Scope
engine package only: workspace manifests + engine_ecs_registry + engine_ecs_query + engine_ecs + engine_world_spatial + engine_world_region + engine_world

## Canon read
- 1.docs/canonical/00_INDEX.md
- 1.docs/canonical/03_PACKAGE_ROLE_MAP.md
- 1.docs/canonical/engine/00_INDEX.md
- 1.docs/canonical/engine/01_SCOPE.md
- 1.docs/canonical/engine/02_CANONICAL_STACK.md
- 1.docs/canonical/engine/03_ROLE_MAP.md
- 1.docs/canonical/engine/04_LIBRARY_BASELINE.md
- 1.docs/canonical/engine/05_DEPENDENCY_MODEL.md
- 1.docs/canonical/engine/07_THREADING_MODEL.md
- 1.docs/canonical/engine/17_TESTING_MODEL.md
- 1.docs/canonical/engine/25_IMPLEMENTATION_HANDOFF.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/00_LEVEL.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/00_LAYER.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/31_THREADING.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/40_FIELDS.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/40_REGISTRY_MODEL.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/41_COMPONENT_PRESENCE.md
- 1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/42_MEMBERSHIP_MAPS.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/00_LEVEL.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/00_LAYER.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/31_THREADING.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/40_FIELDS.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/40_QUERY_DESCRIPTORS.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/41_FILTERS.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/42_JOINS.md
- 1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/43_ACCESS_DESCRIPTORS.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/00_LEVEL.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/00_LAYER.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/31_THREADING.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/40_FIELDS.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/40_ASSEMBLY_SURFACE.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/41_INTEGRATION_BOUNDARY.md
- 1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/42_ECS_SUBSTRATE.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/00_LEVEL.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/00_LAYER.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/31_THREADING.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/40_FIELDS.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/40_COORDINATES.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/41_TRANSFORMS.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/42_SPATIAL_ADDRESSING.md
- 1.docs/canonical/engine/levels/l-0.1-world-spatial/world-spatial/43_SPATIAL_RELATIONS.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/00_LEVEL.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/00_LAYER.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/31_THREADING.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/40_FIELDS.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/40_REGION_MODEL.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/41_CHUNK_MODEL.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/42_DIRTY_REGIONS.md
- 1.docs/canonical/engine/levels/l-0.05-world-region/world-region/43_REGION_VERSIONING.md
- 1.docs/canonical/engine/levels/l0-world-truth/00_LEVEL.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/00_LAYER.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/10_LIBRARIES.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/20_DEPENDENCIES.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/31_THREADING.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/40_FIELDS.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/40_WORLD_STATE.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/41_SNAPSHOTS.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/42_READ_MODEL.md
- 1.docs/canonical/engine/levels/l0-world-truth/world/43_APPLY_BOUNDARY.md

## Changes made
- Added canonical workspace members for L-0.4 through L0 engine crates.
- Implemented `engine_ecs_registry` with explicit registry model, component presence truth, and membership maps.
- Implemented `engine_ecs_query` with query descriptors, filters, joins, access descriptors, and legality checks.
- Implemented `engine_ecs` as thin assembly boundary integrating registry + query into ECS substrate surface.
- Implemented `engine_world_spatial` with coordinates, transforms, deterministic addressing, and spatial relation classification.
- Implemented `engine_world_region` with region/chunk substrate, dirty-region tracking, canonical constants, and region versioning.
- Implemented `engine_world` with authoritative world state root, snapshot/read model surfaces, and apply-boundary ceilings.
- Added tests for all new public behavior including determinism-sensitive and legality-sensitive paths.
- Added `Ord` derivations to lower identity/type/spatial IDs required for deterministic BTree-based structures.

## Files changed
- Cargo.toml
- Cargo.lock
- 2.engine/l-1-foundation/core/engine_core/src/types.rs
- 2.engine/l-0.9-identity/identity/engine_identity/src/lib.rs
- 2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry/Cargo.toml
- 2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry/src/lib.rs
- 2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry/tests/registry.rs
- 2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query/Cargo.toml
- 2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query/src/lib.rs
- 2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query/tests/query.rs
- 2.engine/l-0.2-ecs-assembly/ecs/engine_ecs/Cargo.toml
- 2.engine/l-0.2-ecs-assembly/ecs/engine_ecs/src/lib.rs
- 2.engine/l-0.2-ecs-assembly/ecs/engine_ecs/tests/ecs.rs
- 2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial/Cargo.toml
- 2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial/src/lib.rs
- 2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial/tests/spatial.rs
- 2.engine/l-0.05-world-region/world-region/engine_world_region/Cargo.toml
- 2.engine/l-0.05-world-region/world-region/engine_world_region/src/lib.rs
- 2.engine/l-0.05-world-region/world-region/engine_world_region/tests/region.rs
- 2.engine/l0-world-truth/world/engine_world/Cargo.toml
- 2.engine/l0-world-truth/world/engine_world/src/lib.rs
- 2.engine/l0-world-truth/world/engine_world/tests/world.rs
- .codex/reports/003-engine-lower-stack-50pct.md

## Commands run
- bash ./scripts/setup-dev.sh
- bash ./scripts/test-all.sh
- bash ./scripts/smoke-all.sh
- bash ./scripts/bench-all.sh
- bash ./scripts/metrics-all.sh

## Test results
- unit: pass (`cargo test --workspace --all-features` via `scripts/test-all.sh`)
- integration: pass (integration tests for each new crate and existing lower crates)
- smoke: pass (`scripts/smoke-all.sh` placeholder executed)
- determinism: pass (deterministic addressing, query filtering, snapshot byte stability checks, BTree ordered membership)
- benchmarks: pass (`scripts/bench-all.sh` placeholder executed)

## Metrics
- build time: N/A
- startup ms: N/A
- fixed tick mean ms: N/A
- fixed tick p95 ms: N/A
- peak rss mb: N/A
- allocs per tick: N/A
- binary size: N/A

## Known limitations
- Runtime scheduler logic remains unimplemented by design for this phase.
- smoke/bench/metrics scripts are placeholders and do not yet emit runtime performance artifacts.

## Blockers
- None

## Completion status
PASS
