# L8.0 Editor Shell Boundary Preservation

## Purpose

This document defines the boundary preservation for the editor shell level.

## Scope

The editor shell (`L8.0`) consumes lower-stack surfaces but never weakens their authority laws.

## Boundary Preservation Rules

### Read Discipline
- All world truth reads must flow through `L6.0` authority-core (for status display only)
- All diagnostic reads must flow through `L6.3` diagnostics-runtime (for status display only)
- All assistant reads must flow through `L6A.0` assistant-runtime (for status display only)
- All build reads must flow through `L7.0` build-orchestrator (for status display only)

### Write Discipline
- The shell must NOT write to world truth or entity authority
- The shell must NOT write to asset authority or compilation state
- The shell must NOT write to package state or dependency resolution
- The shell may write to workspace layout state (owned by shell)

### Ownership Discipline
- The shell must NOT own world truth or entity authority
- The shell must NOT own asset authority or compilation state
- The shell must NOT own package state or dependency resolution
- The shell may own workspace layout state
- The shell may own shell-local preferences

## Forbidden Patterns

The editor shell must NOT:
- Read world truth except via `L6.0` authority-core (for status display only)
- Write world truth (must use `L6.0` mutation requests via panels/viewports)
- Maintain shadow world or parallel entity state
- Bypass authority boundaries

## Verification

This boundary preservation is verified by:
- `../../evidence/root/editor_shell_composition_proof_v1.md`
- `../../evidence/root/hidden_parallel_state_audit_v1.md`

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
