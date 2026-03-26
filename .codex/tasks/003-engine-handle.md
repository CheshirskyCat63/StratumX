# 003-engine-handle

## Objective
Create the canonical `engine_handle` crate for L-0.8 Handle System.

## Allowed scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.8-handle-system/handle/engine_handle/**`
- tests for `engine_handle`
- repository scripts only if strictly required for this task

## Forbidden scope
- canonical docs
- crates above L-0.8 except dependency wiring from `engine_core` and `engine_identity`
- sdk/tooling/editor
- speculative runtime ownership or cache logic

## Target crate path
`2.engine/l-0.8-handle-system/handle/engine_handle`

## Canon read first
- `1.docs/canonical/00_INDEX.md`
- `1.docs/canonical/03_PACKAGE_ROLE_MAP.md`
- `1.docs/canonical/engine/00_INDEX.md`
- `1.docs/canonical/engine/02_CANONICAL_STACK.md`
- `1.docs/canonical/engine/03_ROLE_MAP.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/00_LEVEL.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/00_LAYER.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/10_LIBRARIES.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/20_DEPENDENCIES.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/31_THREADING.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/40_HANDLE_TYPES.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/41_VALIDATION_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/42_INVALIDATION_MODEL.md`
- `1.docs/canonical/engine/levels/l-0.8-handle/handle/43_STALE_HANDLE_DETECTION.md`

## Required implementation
Implement the minimal canonical handle substrate:
- stable handle types
- validation model
- invalidation markers/model
- stale handle detection helpers
- explicit non-owning semantics

## Dependency law
- depend only on `engine_core` and `engine_identity`
- do not depend on storage/ecs/world crates
- no hidden registry or ownership shortcuts

## Required checks
- `bash ./scripts/setup-dev.sh`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Deliverables
- compilable `engine_handle` crate
- workspace updated to include it
- tests covering handle validation, invalidation, stale detection, and non-owning behavior
- `.codex/reports/003-engine-handle.md` filled using the report template

## Completion rule
PASS only if the crate compiles, tests pass, and no canonical docs were modified.