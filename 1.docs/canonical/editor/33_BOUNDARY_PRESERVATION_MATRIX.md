# Editor Boundary Preservation Matrix

## Purpose

This document defines the editor package boundary preservation: what can be consumed from lower packages and what must never be owned.

## Scope

The editor package (`L8` through `L11`) consumes lower-stack surfaces but never weakens their authority laws.

## Editor vs Tooling Boundary

| Editor Surface | Consumes from Tooling | Must NOT Own |
|----------------|----------------------|--------------|
| L8.0 Shell | `L6.0` authority-core (read-only) | World truth, entity authority |
| L8.1 Viewport | `L6.0` authority-core, `L6.5` preview-runtime | World truth, rendering pipelines |
| L8.2 Outliner | `L6.0` authority-core, `L6.4` indexing-runtime | World truth, entity authority |
| L8.3 Content Browser | `L6.0` authority-core, `L6.2` package-manager, `L6.4` indexing-runtime | Asset authority, package state |
| L8.4 Inspector | `L6.0` authority-core | World truth, entity authority |
| L8.5 Diagnostics Panel | `L6.3` diagnostics-runtime | Diagnostic generation, validation logic |
| L8.8 Interaction Routing | `L6.0` authority-core | World truth, entity authority |
| L8.9 Assistant Surface | `L6A.0` assistant-runtime | Planning logic, automation execution |
| L8.11 Build/Release | `L7.0` build-orchestrator, `L7.1` release-packager | Build state, release state |
| L9.0-L9.3 Suites | `L6.0` authority-core, `L6.5` preview-runtime | World truth, asset authority |
| L10.1 Import/Export | `L6.1` asset-compiler | Asset compilation, transformation |
| L10.2 Graph Authoring | `L6.0` authority-core | World truth, entity authority |
| L10.4 Script/Hot-Reload | `L6.0` authority-core | Script execution, hot-reload logic |
| L10.5 Plugin/Extension | `L6.0` authority-core (sandboxed) | World truth, asset authority |
| L10.7 Package/Market | `L6.2` package-manager | Package state, dependency resolution |
| L11.0-L11.5 Collaboration | `L6.0` authority-core | World truth, asset authority |

## Editor vs SDK Boundary

| Editor Surface | Consumes from SDK | Must NOT Own |
|----------------|-------------------|--------------|
| L8.0 Shell | `L0` platform-abstraction (windowing, input) | OS primitives |
| L8.1 Viewport | `L0` platform-abstraction (windowing), `L3` math-and-geometry | Rendering primitives |
| All Panels | `L0` platform-abstraction (windowing, input) | OS primitives |
| All Surfaces | `L1` memory-allocators, `L2` collections-and-containers | Memory management |
| Serialization | `L4` serialization-and-io | Serialization primitives |
| Collaboration | `L5` link-ingress-packets (if needed) | Network transport |

## Editor vs Engine Boundary

| Editor Surface | Consumes from Engine | Must NOT Own |
|----------------|----------------------|--------------|
| All Surfaces | NONE (must flow through tooling) | Engine truth, engine authority |

The editor must NEVER consume engine surfaces directly. All engine consumption must flow through tooling package (`L6/L6A/L7/L7A`).

## Boundary Preservation Rules

### Read Discipline
- All world truth reads must flow through `L6.0` authority-core
- All asset authority reads must flow through `L6.0` authority-core
- All package state reads must flow through `L6.2` package-manager
- All diagnostic reads must flow through `L6.3` diagnostics-runtime
- All indexing reads must flow through `L6.4` indexing-runtime
- All preview reads must flow through `L6.5` preview-runtime
- All assistant reads must flow through `L6A.0` assistant-runtime
- All build reads must flow through `L7.0` build-orchestrator
- All release reads must flow through `L7.1` release-packager

### Write Discipline
- All world truth writes must flow through `L6.0` authority-core mutation requests
- All asset authority writes must flow through `L6.1` asset-compiler requests
- All package state writes must flow through `L6.2` package-manager requests
- All build writes must flow through `L7.0` build-orchestrator requests
- All release writes must flow through `L7.1` release-packager requests

### Ownership Discipline
- The editor must NOT own world truth or entity authority
- The editor must NOT own asset authority or compilation state
- The editor must NOT own package state or dependency resolution
- The editor must NOT own rendering pipelines or shader compilation
- The editor must NOT own simulation or physics state
- The editor must NOT own network transport or protocol
- The editor must NOT maintain shadow truth or parallel state

## Verification

This boundary preservation matrix is verified by:
- `evidence/root/editor_shell_composition_proof_v1.md`
- `evidence/root/suite_authority_and_scope_proof_v1.md`
- `evidence/root/service_and_extension_legality_proof_v1.md`
- `evidence/root/collaboration_non_authority_proof_v1.md`
- `evidence/root/hidden_parallel_state_audit_v1.md`

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
