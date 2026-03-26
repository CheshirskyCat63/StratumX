# Global Acceptance Matrix

## Purpose
This document defines the global gold readiness conditions for the entire StratumX canonical stack.
Every row must reference an evidence ID and a physical active artifact from the Global Evidence Registry.
Global gold is blocked if any row remains in `fail` status.

## Matrix

| Row | Requirement | Evidence ID | Active Artifact | Status | Notes |
|-----|-------------|-------------|-----------------|--------|-------|
| GLOBAL-001 | Package completeness closure: each package root is physically present | PKG-COMPLETE | `engine/00_INDEX.md`, `sdk/00_INDEX.md`, `tooling/00_INDEX.md`, `editor/00_INDEX.md` | pass | All four package root indexes physically exist in the archive |
| GLOBAL-002 | Cross-package dependency legality closure: no illegal direct dependencies are declared at global umbrella level | DEP-LEGAL | `04_GLOBAL_DEPENDENCY_MODEL.md` | pass | Global dependency model exists and defines legal/illegal package dependency directions |
| GLOBAL-003 | Authority order closure: one stack-wide conflict resolution order exists | AUTH-ORDER | `05_GLOBAL_AUTHORITY_ORDER.md` | pass | Global authority order is explicitly declared |
| GLOBAL-004 | Evidence existence closure: every global evidence ID resolves to a physical active artifact | EVID-EXIST | `07_GLOBAL_EVIDENCE_REGISTRY.md` | pass | Global registry lists only physical root-level artifacts |
| GLOBAL-005 | Readiness/evidence alignment closure: global readiness rows reference acceptance rows and evidence IDs | READY-ALIGN | `99_GLOBAL_AUDIT_READINESS_MATRIX.md` | pass | Global readiness matrix is mechanically linked to acceptance and evidence |
| GLOBAL-006 | Version alignment closure: one stack-wide version marker exists at global root | VERSION-ALIGN | `STACK_VERSION` | pass | Global stack version marker physically exists |
| GLOBAL-007 | Freeze legality closure: global gold freeze is explicitly prohibited while any global acceptance row fails | FREEZE-LEGAL | `08_GLOBAL_BUILD_AND_FREEZE_CONDITIONS.md` | pass | Freeze law is explicitly defined at global root |
| GLOBAL-008 | No forbidden direct ownership closure: global role map forbids editor/tooling/sdk from owning lower-stack truth | NO-FORBID-OWN | `03_PACKAGE_ROLE_MAP.md` | pass | Ownership and non-ownership are explicitly defined at package level |
| GLOBAL-009 | No shadow truth across packages: truth ownership is globally partitioned | NO-SHADOW-TRUTH | `03_PACKAGE_ROLE_MAP.md` | pass | Global role map assigns truth ownership by package |
| GLOBAL-010 | Hot/core layer density closure: all critical package hot/core levels have full local documentation packages | LAYER-DENSITY | `02_STACK_MAP.md`, `tooling/27_ACCEPTANCE_MATRIX.md`, `editor/39_ACCEPTANCE_MATRIX.md` | pass | Tooling and editor critical levels remain fully expanded and package-verified. |
| GLOBAL-011 | Package pass-chain closure: engine/16, sdk/99, tooling/99, editor/99 all pass | PKG-PASS-CHAIN | `engine/16_AUDIT_READINESS_MATRIX.md`, `sdk/99_AUDIT_READINESS_MATRIX.md`, `tooling/99_AUDIT_READINESS_MATRIX.md`, `editor/99_AUDIT_READINESS_MATRIX.md` | pass | All four package readiness matrices show full pass status. |
| GLOBAL-012 | No duplicate ordinals in any package root: engine, sdk, tooling, editor root document sets have unique ordinals | NO-DUP-ORD | `engine/00_INDEX.md`, `sdk/00_INDEX.md`, `tooling/00_INDEX.md`, `editor/00_INDEX.md` | pass | Editor duplicate ordinals resolved via renumbering |
| GLOBAL-013 | No root-level glob ban: no wildcard/glob paths in root-level authority documents | NO-ROOT-GLOB | `engine/09_GLOSSARY.md`, `sdk/09_GLOSSARY.md`, `tooling/09_GLOSSARY.md`, `editor/34_GLOSSARY.md` | pass | Root-level glob ban satisfied across all package roots. |
| GLOBAL-014 | Active test-evidence posture: all packages have executed test-result artifacts, not just test-class coverage | TEST-EXEC-POSTURE | `engine/evidence/tests/engine_test_result_posture_v1.md`, `sdk/evidence/tests/sdk_test_result_posture_v12.md`, `tooling/evidence/tests/tooling_test_result_posture_v3.md`, `editor/evidence/tests/editor_test_result_posture_v1.md` | pass | All packages record executed document-package validation runs in active posture artifacts. |

## Status Legend
- `pass`: Requirement satisfied and supported by a physical active artifact
- `fail`: Requirement not satisfied or not provable from current archive state
