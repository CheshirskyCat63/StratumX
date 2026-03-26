# Rules

## Scope discipline
- Change only files required for the assigned task.
- Do not rename or reorganize unrelated code.
- Do not touch canonical docs unless explicitly instructed.
- Do not modify CI rules to make failures disappear.

## Package law
- `engine` owns engine truth and runtime implementation.
- `sdk` owns typed publication and bridge contracts.
- `tooling` owns orchestration and derived tooling behavior.
- `editor` owns editor-local product/runtime surfaces.
- No package may steal responsibility from another package.

## Dependency law
- No new third-party crate without justification against the package baseline.
- No debug-only shortcuts committed as production architecture.
- No hidden service locators.
- No reflection-heavy runtime registries as architecture shortcuts.

## Testing law
- New public types require tests where behavior exists.
- New runtime systems require smoke coverage.
- Hot-path changes require benchmark handling.
- State and replay-sensitive logic requires determinism handling.

## Reporting law
A task is not complete unless the report contains:
- canon read list
- files changed
- commands actually run
- actual test results
- actual benchmark results or reason they were not applicable
- actual blockers
- final PASS or FAIL

## Forbidden evasions
Forbidden:
- silent skipped tests
- fake benchmark claims
- empty report sections
- "not enough time"
- "should be fine"
- claiming success without artifacts