# AGENTS.md

## Mission
Implement the StratumX engine strictly according to the canonical documentation in `1.docs/canonical/`.

## Canon is law
- The canonical documentation is the source of truth.
- Read canon before changing code.
- Do not modify canon unless the task explicitly requires canonical repair.
- If code and canon disagree, canon wins.
- If canon is incomplete or contradictory for the assigned task, stop and report the blocker with exact file paths.

## Required read order
1. `1.docs/canonical/00_INDEX.md`
2. `1.docs/canonical/03_PACKAGE_ROLE_MAP.md`
3. the relevant package root (`engine`, `sdk`, `tooling`, `editor`)
4. the relevant level documents
5. the assigned task file in `.codex/tasks/`

## Hard architectural rules
- Respect package boundaries: `engine`, `sdk`, `tooling`, `editor`.
- Do not invent cross-layer APIs.
- Do not move responsibilities across packages.
- Do not add hidden global state.
- Do not add silent caches, hidden stores, hidden databases, or hidden background services.
- Do not add dependencies unless they are allowed by the package baseline.
- Do not use `unsafe` unless the task explicitly permits it and the code is isolated and justified.
- Do not rewrite unrelated modules.
- Do not broaden task scope on your own.

## Delivery rules
- Every public behavior change must include tests.
- Every runtime hot-path change must include benchmark coverage or benchmark impact statement.
- Every state-model change must include serialization/determinism coverage where applicable.
- Do not claim completion if any required check fails.
- Use `.codex/report-template.md` exactly.
- Final status must be `PASS` or `FAIL`, never vague language.

## Required commands before completion
Run the repository scripts, not ad hoc substitutes:
- `powershell -ExecutionPolicy Bypass -File .\scripts\setup-dev.ps1`
- `powershell -ExecutionPolicy Bypass -File .\scripts\test-all.ps1`
- `powershell -ExecutionPolicy Bypass -File .\scripts\smoke-all.ps1`
- `powershell -ExecutionPolicy Bypass -File .\scripts\bench-all.ps1`
- `powershell -ExecutionPolicy Bypass -File .\scripts\metrics-all.ps1`

## Forbidden completion patterns
Do not say:
- "done except"
- "mostly complete"
- "tests were not run but should pass"
- "left TODOs for later"
- "canon probably means"

Either prove completion or report a blocker.