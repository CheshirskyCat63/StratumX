# Global Scope

## What this archive covers

`canonical/` is the single authoritative documentation archive for the StratumX engine and editor stack.  
It covers all four packages in their full authority:

- **engine**: runtime kernel, simulation, resource systems, startup — L-0 through L4
- **sdk**: data-oriented public bridge types, controls, observations, handles, refs — L5
- **tooling**: authority core, transaction ledger, snapshot/index/derived/artifact/stream/cache/budget planes, assistant runtimes, studio orchestration — L6, L6A, L7, L7A
- **editor**: authoring editor shell, viewports, panels, suites, services, collaboration — L8, L9, L10, L11

## What this archive does not cover

- Implementation source code (lives in workspace crates)
- Runtime executable binaries or build artifacts
- User documentation / manuals
- Project-specific content or data files
- Third-party library internals

## Global Gold Criterion

The archive reaches global gold when:

1. Every package has a complete root document set (index, scope, stack/role map, library baseline, dependency model, communication model, threading model, boundary preservation, testing model, acceptance matrix, evidence registry, build/freeze conditions, audit readiness, plus package-specific laws).
2. Every `pass` row in every acceptance matrix cites a specific **Evidence ID** and an **active, existing** artifact.
3. No document is self-certifying (readiness must reference acceptance; acceptance must reference evidence artifacts).
4. No evidence registry row points to a non-existent file.
5. No duplicate ordinals exist in any package root.
6. No wildcard/glob paths appear in root-level authority documents.
7. All hot/core layers have full local documentation packages (not a single `00_LEVEL.md` only).
8. All inter-package boundaries are formally stated and non-contradictory.
9. Version discipline is unified across all packages.
10. The global umbrella documents (this file and siblings) are complete and consistent.
11. All four package audit readiness matrices (engine/16, sdk/99, tooling/99, editor/99) show full pass status.
12. Active test-result artifacts exist for all packages with executed document-package validation evidence (not just test-class coverage).

## Scope Authority

This document is superseded only by constitutional law if a specific engine constitution overrides a global scope claim.  
Within `canonical/`, this document is second in authority only to `05_GLOBAL_AUTHORITY_ORDER.md`.
