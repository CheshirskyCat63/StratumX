# 009-engine-ecs-assembly

## Objective
Create the canonical `engine_ecs` crate for L-0.2 ECS Assembly.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.2-ecs-assembly/ecs/engine_ecs/**`
- tests for `engine_ecs`

## Forbidden scope
- canonical docs
- world truth/runtime logic
- sdk/tooling/editor
- speculative hot-path optimization outside canonical minimum

## Target crate path
`2.engine/l-0.2-ecs-assembly/ecs/engine_ecs`

## Canon read first
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/40_ASSEMBLY_SURFACE.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/41_INTEGRATION_BOUNDARY.md`
- `1.docs/canonical/engine/levels/l-0.2-ecs-assembly/ecs/42_ECS_SUBSTRATE.md`

## Required implementation
Implement the minimal assembled ECS substrate:
- assembly surface
- integration boundary model
- substrate type(s) that compose registry/query/storage pieces together
- no world truth or runtime scheduling here

## Dependency law
- depend on lower substrate crates only
- do not bypass lower layer contracts

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_ecs` crate
- workspace updated to include it
- tests covering substrate assembly, boundary integrity, and basic assembled use cases
- `.codex/reports/009-engine-ecs-assembly.md`

## Completion rule
PASS only if the crate compiles, tests pass, and assembly remains below world truth.