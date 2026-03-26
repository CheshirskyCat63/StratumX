# 002-engine-identity

## Objective
Create the canonical `engine_identity` crate for L-0.9 Identity Primitives.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.9-identity-primitives/identity/engine_identity/**`
- tests for `engine_identity`
- repository scripts only if strictly required for this task

## Forbidden scope
- canonical docs
- crates above L-0.9 except dependency wiring from `engine_core`
- sdk/tooling/editor
- speculative helpers unrelated to identity primitives

## Target crate path
`2.engine/l-0.9-identity-primitives/identity/engine_identity`

## Canon read first
- `1.docs/canonical/00_INDEX.md`
- `1.docs/canonical/03_PACKAGE_ROLE_MAP.md`
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/01_SCOPE.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/40_ENTITY_ID.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/41_COMPONENT_ID.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/42_GENERATION_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.9-identity/identity/43_IDENTITY_DOMAINS.md`

## Required implementation
Implement the minimal canonical identity substrate:
- entity identity type(s)
- component identity type(s)
- generation model primitives
- identity domain tagging/separation
- validation helpers for stale/invalid identity cases where the canon clearly requires them

## Dependency law
- depend only on `engine_core`
- do not depend on handle/storage/ecs/world crates
- do not add external crates unless canon baseline requires them

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_identity` crate
- workspace updated to include it
- tests covering entity ID, component ID, generation semantics, and domain separation
- `.codex/reports/002-engine-identity.md` filled using the report template

## Completion rule
PASS only if the crate compiles, tests pass, and no canonical docs were modified.