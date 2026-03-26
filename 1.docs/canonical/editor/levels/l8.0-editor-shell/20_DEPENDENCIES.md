# L8.0 Editor Shell Dependencies

## Purpose

This document defines the dependencies for the editor shell level.

## Scope

The editor shell (`L8.0`) consumes lower-stack surfaces but never weakens their authority laws.

## Legal Dependencies

### To Tooling Package (L6/L6A/L7/L7A)
The editor shell may consume:
- `L6.0` authority-core: World truth (read-only, for status display)
- `L6.3` diagnostics-runtime: Diagnostic message routing (for status display)
- `L6A.0` assistant-runtime: Assistant status (for status display)
- `L7.0` build-orchestrator: Build status (for status display)

The editor shell must NOT:
- Write to world truth or entity authority
- Write to asset authority or compilation state
- Write to package state or dependency resolution
- Bypass authority boundaries

### To SDK Package (L0-L5)
The editor shell may consume:
- `L0` platform-abstraction: OS, windowing, input
- `L1` memory-allocators: Memory allocation
- `L2` collections-and-containers: Data structures
- `L4` serialization-and-io: Serialization primitives (for layout persistence)

### To Other Editor Levels
The editor shell may consume:
- `L8.1` viewport-system: Viewport host integration
- `L8.2` outliner-system: Panel integration
- `L8.3` content-browser-system: Panel integration
- `L8.4` inspector-system: Panel integration
- `L8.5` tool-context-system: Tool mode display
- `L8.6` overlay-and-gizmo-system: Overlay display
- `L8.7` workspace-layout-system: Layout persistence
- `L8.8` interaction-routing-system: Command dispatch
- `L8.9` assistant-surface: Panel integration
- `L8.10` diagnostics-surface: Panel integration
- `L8.11` build-release-surface: Panel integration

## Dependency Discipline

All editor shell dependencies must:
- Flow through sanctioned lower surfaces
- Never bypass authority boundaries
- Never maintain parallel truth
- Never weaken lower-stack authority laws

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
