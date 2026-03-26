# Editor Dependency Model

## Purpose

This document defines the package-wide editor dependency law and legal dependencies to lower packages.

## Scope

The editor package (`L8` through `L11`) consumes lower-stack surfaces but never weakens their authority laws.

## Legal Dependencies

### To Tooling Package (L6/L6A/L7/L7A)
The editor may consume:
- `L6.0` authority-core: World truth, entity authority, asset authority
- `L6.1` asset-compiler: Asset compilation requests (read-only)
- `L6.2` package-manager: Package discovery and dependency resolution (read-only)
- `L6.3` diagnostics-runtime: Diagnostic message routing
- `L6.4` indexing-runtime: Asset and entity indexing (read-only)
- `L6.5` preview-runtime: Preview request/result coordination
- `L6A.0` assistant-runtime: Assistant request/result coordination
- `L7.0` build-orchestrator: Build request/result coordination
- `L7.1` release-packager: Release request/result coordination
- `L7.2` automation-runtime: Automation request/result coordination
- `L7A.0` planning-brain: Planning request/result coordination

The editor must NOT:
- Write directly to world truth or entity authority
- Write directly to asset authority or compilation state
- Write directly to package management state
- Bypass authority boundaries
- Maintain shadow truth or parallel state

### To SDK Package (L0-L5)
The editor may consume:
- `L0` platform-abstraction: OS, windowing, input
- `L1` memory-allocators: Memory allocation
- `L2` collections-and-containers: Data structures
- `L3` math-and-geometry: Math primitives
- `L4` serialization-and-io: Serialization primitives
- `L5` link-ingress-packets: Network primitives (if collaboration requires)

The editor must NOT:
- Consume engine-specific SDK surfaces (L0-L5 are universal)
- Bypass tooling authority to reach SDK directly for world/asset/entity operations

### To Engine Package (L0-L5)
The editor must NOT:
- Consume engine surfaces directly
- Bypass tooling authority to reach engine
- Own engine truth or authority

## Dependency Discipline

All editor dependencies must:
- Flow through sanctioned lower surfaces
- Never bypass authority boundaries
- Never maintain parallel truth
- Never weaken lower-stack authority laws
- Release resources when surfaces are cold

## Forbidden Patterns

The editor must NOT:
- Read world truth except via `L6.0` authority-core
- Write world truth except via `L6.0` authority-core mutation requests
- Read asset authority except via `L6.0` authority-core
- Write asset authority except via `L6.1` asset-compiler requests
- Read package state except via `L6.2` package-manager
- Write package state except via `L6.2` package-manager requests
- Maintain shadow world or parallel entity state
- Maintain shadow asset or parallel compilation state
- Maintain shadow package or parallel dependency state

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
