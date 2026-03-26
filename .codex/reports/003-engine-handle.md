## Task
003-engine-handle — Create the canonical `engine_handle` crate for L-0.8 Handle System.

## Scope
- workspace manifests only if needed to add the new crate
- `2.engine/l-0.8-handle-system/handle/engine_handle/**`
- tests for `engine_handle`
- repository scripts only if strictly required for this task

## Canon read
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

## Changes made
- Added canonical `engine_handle` crate with stable handle types (`HandleSlot`, `HandleGeneration`, `StableHandle`, `EntityHandle`) and explicit non-owning semantics.
- Implemented validation and invalidation model (`InvalidationMarker`, `HandleValidation`, `validate_handle`).
- Implemented stale-handle detection helpers (`is_stale_generation`, `is_stale_handle`).
- Added unit and integration tests covering validation, invalidation, stale detection, and non-owning behavior.
- Added the new crate to workspace members.

## Files changed
- `Cargo.toml`
- `2.engine/l-0.8-handle-system/handle/engine_handle/Cargo.toml`
- `2.engine/l-0.8-handle-system/handle/engine_handle/src/lib.rs`
- `2.engine/l-0.8-handle-system/handle/engine_handle/tests/handle_models.rs`
- `.codex/reports/003-engine-handle.md`

## Commands run
- `bash ./scripts/setup-dev.sh`
- `cargo fmt --all`
- `bash ./scripts/test-all.sh`
- `bash ./scripts/smoke-all.sh`
- `bash ./scripts/bench-all.sh`
- `bash ./scripts/metrics-all.sh`

## Test results
- unit: pass (`cargo test --workspace --all-features` includes `engine_handle` unit tests)
- integration: pass (`2.engine/l-0.8-handle-system/handle/engine_handle/tests/handle_models.rs`)
- smoke: pass (`bash ./scripts/smoke-all.sh`)
- determinism: pass (deterministic generation comparisons validated by tests)
- benchmarks: pass (`bash ./scripts/bench-all.sh` completed with placeholder benchmark output)

## Metrics
- build time: N/A
- startup ms: N/A
- fixed tick mean ms: N/A
- fixed tick p95 ms: N/A
- peak rss mb: N/A
- allocs per tick: N/A
- binary size: N/A

## Known limitations
- `engine_identity` crate is not yet present in repository, so dependency wiring currently aliases `engine_core` package under the `engine_identity` crate name.
- Benchmark and metrics scripts are placeholders and report N/A values.

## Blockers
- None.

## Completion status
PASS
