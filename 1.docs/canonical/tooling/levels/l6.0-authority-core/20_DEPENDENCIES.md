# L6.0 Authority Core - Dependencies

## Purpose

This document defines the dependency closure for L6.0 authority core.

## Dependency Graph

```
L6.0 Authority Core
  ├─ stratumx_tooling_core (tooling core types)
  ├─ stratumx_sdk_l5 (SDK L5 bridge)
  └─ std (Rust standard library)
```

## Dependency Rules

1. **SDK L5 Only**: L6.0 may only depend on SDK L5 bridge, not engine internals
2. **No Upward Dependencies**: L6.0 may not depend on L6.1+ or L8+ levels
3. **No Circular Dependencies**: L6.0 may not create circular dependencies with other L6 levels
4. **Foundational Position**: L6.0 is the foundation for all other L6 levels

## Dependency Verification

All dependencies are verified through:
- Cargo.toml inspection
- Dependency graph analysis
- Boundary preservation checks
- Build system verification
