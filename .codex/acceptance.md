# Acceptance

A task is complete only if all of the following are true:

## Build
- Workspace build succeeds for the affected scope.
- No syntax or compile failures remain.

## Quality
- Formatting passes.
- Linting passes.
- No new warnings are introduced in the affected scope when warnings are configured as errors.

## Tests
- Unit tests pass for affected code.
- Integration tests pass for affected code where integration surfaces exist.
- Smoke checks pass for affected runtime surfaces.
- Determinism-sensitive logic has determinism coverage where applicable.

## Runtime
- If the task touches hot path or scheduling behavior, benchmark handling is present.
- If the task touches startup/runtime loop, smoke startup and shutdown must pass.

## Scope integrity
- No unauthorized canon edits.
- No unauthorized package boundary violations.
- No unrelated file churn.

## Reporting
- `.codex/report-template.md` is filled with real results.
- Final status is `PASS` only if all required checks passed.
- Otherwise final status is `FAIL`.