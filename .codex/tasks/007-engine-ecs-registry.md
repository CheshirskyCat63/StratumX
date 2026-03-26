# 007-engine-ecs-registry

## Objective
Create the canonical `engine_ecs_registry` crate for L-0.4 ECS Registry.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry/**`
- tests for `engine_ecs_registry`

## Forbidden scope
- canonical docs
- query engine logic beyond registry needs
- assembled ECS/world/runtime code
- sdk/tooling/editor

## Target crate path
`2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry`

## Canon read first
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/40_REGISTRY_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/41_COMPONENT_PRESENCE.md`
- `1.docs/canonical/engine/levels/l-0.4-ecs-registry/ecs-registry/42_MEMBERSHIP_MAPS.md`

## Required implementation
Implement the minimal registry substrate:
- registry model
- component presence tracking
- membership maps
- explicit structural truth, not assembled query execution

## Dependency law
- depend on lower substrate crates only
- do not implement query traversal here
- do not invent runtime scheduling behavior

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_ecs_registry` crate
- workspace updated to include it
- tests covering registry integrity, membership presence, and structural invariants
- `.codex/reports/007-engine-ecs-registry.md`

## Completion rule
PASS only if the crate compiles, tests pass, and no query/assembly leakage exists.