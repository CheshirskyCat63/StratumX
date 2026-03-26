# Global Evidence Registry

## Purpose
This document records the active evidence artifacts for the global acceptance matrix.
Each evidence ID must point to a physical artifact that exists in the archive.

## Registry

| Evidence ID | Purpose | Covers rows | Active artifact path | Artifact status | Notes |
|-------------|---------|-------------|----------------------|-----------------|-------|
| PKG-COMPLETE | Verifies package root presence across the canonical stack | GLOBAL-001 | `engine/00_INDEX.md`, `sdk/00_INDEX.md`, `tooling/00_INDEX.md`, `editor/00_INDEX.md` | active | Root package indexes are physically present |
| DEP-LEGAL | Defines legal and illegal cross-package dependency directions | GLOBAL-002 | `04_GLOBAL_DEPENDENCY_MODEL.md` | active | Global dependency law is physically present |
| AUTH-ORDER | Establishes the stack-wide conflict resolution order | GLOBAL-003 | `05_GLOBAL_AUTHORITY_ORDER.md` | active | Global authority order is physically present |
| EVID-EXIST | Verifies that global evidence IDs resolve to physical active artifacts | GLOBAL-004 | `07_GLOBAL_EVIDENCE_REGISTRY.md` | active | Registry self-existence is physical; row validity is checked through listed artifact paths |
| READY-ALIGN | Verifies readiness linkage to global acceptance and evidence | GLOBAL-005 | `99_GLOBAL_AUDIT_READINESS_MATRIX.md` | active | Global readiness matrix is physically present |
| VERSION-ALIGN | Verifies the stack-wide version marker | GLOBAL-006 | `STACK_VERSION` | active | Root stack version marker is physically present |
| FREEZE-LEGAL | Defines legal conditions for global gold freeze | GLOBAL-007 | `08_GLOBAL_BUILD_AND_FREEZE_CONDITIONS.md` | active | Freeze law is physically present |
| NO-FORBID-OWN | Verifies forbidden ownership constraints at package level | GLOBAL-008 | `03_PACKAGE_ROLE_MAP.md` | active | Role map is physically present |
| NO-SHADOW-TRUTH | Verifies global truth partitioning across packages | GLOBAL-009 | `03_PACKAGE_ROLE_MAP.md` | active | Role map is physically present |
| LAYER-DENSITY | Anchors the root-level statement about hot/core layer density | GLOBAL-010 | `02_STACK_MAP.md`, `tooling/27_ACCEPTANCE_MATRIX.md`, `tooling/30_EVIDENCE_REGISTRY.md`, `editor/39_ACCEPTANCE_MATRIX.md`, `editor/40_EVIDENCE_REGISTRY.md` | active | Root stack map plus package acceptance/evidence matrices prove critical level density closure across tooling and editor packages. |
| PKG-PASS-CHAIN | Verifies all four package audit readiness matrices show full pass status | GLOBAL-011 | `engine/16_AUDIT_READINESS_MATRIX.md`, `sdk/99_AUDIT_READINESS_MATRIX.md`, `tooling/99_AUDIT_READINESS_MATRIX.md`, `editor/99_AUDIT_READINESS_MATRIX.md` | active | Package pass-chain verified across engine, sdk, tooling, and editor. |
| NO-DUP-ORD | Verifies no duplicate ordinals exist in any package root document set | GLOBAL-012 | `engine/00_INDEX.md`, `sdk/00_INDEX.md`, `tooling/00_INDEX.md`, `editor/00_INDEX.md` | active | Editor duplicate ordinals resolved via renumbering |
| NO-ROOT-GLOB | Verifies no wildcard/glob paths appear in root-level authority documents | GLOBAL-013 | `engine/09_GLOSSARY.md`, `sdk/09_GLOSSARY.md`, `tooling/09_GLOSSARY.md`, `editor/34_GLOSSARY.md` | active | Root-level glob ban satisfied across package roots |
| TEST-EXEC-POSTURE | Verifies all packages have executed test-result artifacts, not just test-class coverage | GLOBAL-014 | `engine/evidence/tests/engine_test_result_posture_v1.md`, `sdk/evidence/tests/sdk_test_result_posture_v12.md`, `tooling/evidence/tests/tooling_test_result_posture_v3.md`, `editor/evidence/tests/editor_test_result_posture_v1.md` | active | Active test-result artifacts record executed document-package validation runs across all packages. |

## Artifact Status Legend
- `active`: Artifact exists and is the current root-level source of truth
- `inactive`: Artifact exists but must not be used for current closure
- `pending`: Reserved for future artifacts not yet activated
