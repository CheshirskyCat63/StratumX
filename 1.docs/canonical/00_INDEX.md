# StratumX Canonical Archive — Global Index

**Stack version:** `SX-CANON/1.0.6/STACK-v12`

## Purpose

This is the root index for the entire `canonical/` archive.  
It serves as the single navigation entry point for all four packages and the global authority contour that binds them.

## Package Map

| Package | Path | Stack layers | Role |
|---|---|---|---|
| engine | `engine/` | L-0 through L4 | Runtime truth owner |
| sdk | `sdk/` | L5 | Data-oriented bridge; legal adapters from engine to tooling |
| tooling | `tooling/` | L6, L6A, L7, L7A | Studio tooling and assistant orchestration |
| editor | `editor/` | L8, L9, L10, L11 | Authoring editor and its surfaces |

## Global Root Documents

| Ordinal | File | Purpose |
|---|---|---|
| 00 | `00_INDEX.md` | This file |
| 01 | `01_SCOPE.md` | Global scope and gold criterion |
| 02 | `02_STACK_MAP.md` | Layer-range map and data-flow directions |
| 03 | `03_PACKAGE_ROLE_MAP.md` | Authority roles and truth-ownership per package |
| 04 | `04_GLOBAL_DEPENDENCY_MODEL.md` | Legal inter-package read-paths |
| 05 | `05_GLOBAL_AUTHORITY_ORDER.md` | Global conflict-resolution order |
| 06 | `06_GLOBAL_ACCEPTANCE_MATRIX.md` | Global gold acceptance ledger |
| 07 | `07_GLOBAL_EVIDENCE_REGISTRY.md` | Global active evidence artifact registry |
| 08 | `08_GLOBAL_BUILD_AND_FREEZE_CONDITIONS.md` | Global freeze law |
| 09 | `09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX.md` | Inter-package boundary preservation table |
| 99 | `99_GLOBAL_AUDIT_READINESS_MATRIX.md` | Global readiness ledger |

## Package Indexes

- [engine/00_INDEX.md](engine/00_INDEX.md)
- [sdk/00_INDEX.md](sdk/00_INDEX.md)
- [tooling/00_INDEX.md](tooling/00_INDEX.md)
- [editor/00_INDEX.md](editor/00_INDEX.md)

## Gold Criterion

The archive is at global gold when every row in `06_GLOBAL_ACCEPTANCE_MATRIX.md` is `pass` and every `pass` references an active artifact in `07_GLOBAL_EVIDENCE_REGISTRY.md`.
