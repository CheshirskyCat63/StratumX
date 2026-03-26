# L5.0 Link Ingress Packets - Dependencies

## Purpose

This document defines the dependency closure for L5.0 link ingress packets.

## Dependency Graph

```
L5.0 Link Ingress Packets
  ├─ stratumx_sdk_core (SDK core types)
  ├─ stratumx_engine_l4_export (L4 export surfaces)
  └─ std (Rust standard library)
```

## Dependency Rules

1. **L4 Export Only**: L5.0 may only depend on L4 export surfaces, not engine internals
2. **No Upward Dependencies**: L5.0 may not depend on L5.1+ or L6+ levels
3. **No Circular Dependencies**: L5.0 may not create circular dependencies with other L5 levels
4. **Minimal External Dependencies**: L5.0 should minimize external crate dependencies

## Dependency Verification

All dependencies are verified through:
- Cargo.toml inspection
- Dependency graph analysis
- Boundary preservation checks
- Build system verification
