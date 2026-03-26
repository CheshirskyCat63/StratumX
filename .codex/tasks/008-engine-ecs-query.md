# 008-engine-ecs-query

## Objective
Create the canonical `engine_ecs_query` crate for L-0.3 ECS Query.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query/**`
- tests for `engine_ecs_query`

## Forbidden scope
- canonical docs
- assembled ECS/world/runtime code
- speculative parallel scheduler logic
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query`

## Canon read first
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/40_QUERY_DESCRIPTORS.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/41_FILTERS.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/42_JOINS.md`
- `1.docs/canonical/engine/levels/l-0.3-ecs-query/ecs-query/43_ACCESS_DESCRIPTORS.md`

## Required implementation
Implement the minimal query substrate:
- query descriptors
- filter model
- join model
- access descriptors
- deterministic query shape validation

## Dependency law
- depend on lower substrate crates including `engine_ecs_registry`
- do not assemble full ECS or world state here

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_ecs_query` crate
- workspace updated to include it
- tests covering descriptor legality, filters, joins, and access descriptor invariants
- `.codex/reports/008-engine-ecs-query.md`

## Completion rule
PASS only if the crate compiles, tests pass, and query behavior stays descriptor-level and deterministic.