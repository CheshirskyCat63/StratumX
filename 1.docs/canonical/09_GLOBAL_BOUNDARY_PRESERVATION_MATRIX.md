# Global Boundary Preservation Matrix

## Purpose
This document formally preserves the boundaries between packages in the StratumX canonical stack.
It defines what can be transmitted across each boundary, what is prohibited, and the legal request/result/ref classes.

## Boundaries

### Engine -> SDK Boundary
- **What can be passed**: 
  - Engine truth via SDK bridge types: link packets, controls, observations, metrics, handles, refs
  - Only data-oriented, read-only adaptations of engine state
- **What cannot be passed**:
  - Direct engine internal types (ECS components, systems, raw pointers)
  - Engine mutation interfaces (editor must not mutate engine state)
  - Engine implementation details (crates, private modules)
- **Legal request/result/ref classes**:
  - Defined in `sdk/01_SCOPE.md`, `sdk/03_ROLE_MAP.md`, `sdk/13_TYPOLOGY_SYSTEM.md`, `sdk/14_ROLE_CLASS_SEPARATION_MATRIX.md`, `sdk/17_REF_SUBTYPES.md`, `sdk/33_HANDLE_AND_REF_OPACITY_LAW.md`:
    - Link packets (L5.0), Controls (L5.1), Observations (L5.2), Metrics (L5.3)
    - Handles (L5.10-L5.12), Refs (L5.13-L5.15)
    - Request: engine state read via L4 public export surfaces
    - Result: SDK bridge snapshots, batches, cursors

### SDK -> Tooling Boundary
- **What can be passed**:
  - SDK bridge types as opaque refs or read-only views
  - Tooling-specific derived state (snapshots, derivations, artifact manifests)
  - Assistant context packs and proposals
- **What cannot be passed**:
  - Direct SDK ownership of engine truth (SDK is only a bridge)
  - Tooling mutation of SDK bridge types (except through defined interfaces)
  - Tooling reads of engine truth without SDK mediation
- **Legal request/result/ref classes**:
  - Defined in `tooling/01_SCOPE.md`, `tooling/03_ROLE_MAP.md`, `tooling/13_DATA_PLANE_MODEL.md`, `tooling/14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `tooling/15_SNAPSHOT_INDEX_DERIVED_MODEL.md`, `tooling/19_CROSS_LAYER_EXCHANGE_MODEL.md`:
    - L6 authority core: transaction records, snapshot keys, index cursors
    - L6 planes: snapshot/index/derived/artifact/stream/cache/budget plane refs
    - Request: L5 intake via legal L6 authority surfaces
    - Result: tooling plane snapshots, derived views, artifact manifests

### Tooling -> Editor Boundary
- **What can be passed**:
  - Tooling orchestration state (transaction logs, budget planes, assistant proposals)
  - Editor-specific views (selection, focus, inspection)
  - Assistant-generated content proposals
- **What cannot be passed**:
  - Direct tooling ownership of SDK/engine truth (editor must not read lower-stack truth directly)
  - Editor mutation of tooling state (except through defined command channels)
  - Editor reads of tooling internal implementation details
- **Legal request/result/ref classes**:
  - Defined in `editor/01_SCOPE.md`, `editor/03_EDITOR_ROLE_MAP.md`, `editor/22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`:
    - L8 shell: viewport refs, panel state, tool context, interaction routing
    - L9 suites: suite-level edit commands, preview results, diagnostics events
    - L10 services: project orchestration refs, import/export results, automation outputs
    - L11 collaboration: session refs, review annotations, asset gate verdicts
    - Request: editor commands routed through legal L6/L7 surfaces
    - Result: tooling orchestration snapshots, assistant proposals, build/release results

## Boundary Integrity Rules
1. No package may read truth from another package outside the defined legal surfaces.
2. All cross-package data transfer must use explicitly defined request/result/ref classes.
3. No package may bypass the dependency chain to access truth (e.g., editor -> engine, editor -> sdk, tooling -> engine).
4. SDK is strictly a typed bridge and does not own engine truth.
5. Tooling owns only derived and orchestrated state, not engine-original truth.
6. Editor owns only authoring surface state and does not own lower-stack truth.