# Editor Library Baseline

## Purpose

This document defines the allowed editor-side library classes and prohibited library role leakage into lower packages.

## Scope

The editor package (`L8` through `L11`) may own library classes for:
- UI composition and layout
- Render surface management (viewport, panel, overlay)
- Editor runtime coordination
- Plugin and extension hosting
- Script integration surfaces
- Serialization of editor-local state (workspace, layout, preferences)
- Indexing and search for editor surfaces
- Diagnostics presentation and routing
- Collaboration session management

The editor package must NOT own library classes for:
- World truth or entity authority
- Asset compilation or transformation
- Core simulation or physics
- Rendering pipelines or shader compilation
- Network transport or protocol
- File system or package management (consumed from tooling)

## Allowed Editor Library Classes

### UI and Render Surface Classes
- `EditorShell`: Top-level window and menu host
- `ViewportSurface`: 3D/2D viewport render target
- `PanelHost`: Dockable panel container
- `OverlayRenderer`: Gizmo and overlay composition
- `LayoutManager`: Workspace layout persistence
- `ThemeProvider`: Editor theme and styling

### Editor Runtime Classes
- `EditorSession`: Editor lifetime and state coordination
- `SelectionManager`: Cross-surface selection state
- `FocusRouter`: Focus and input routing
- `CommandDispatcher`: Command palette and shortcut routing
- `ModeManager`: Tool mode and context management
- `PreviewCoordinator`: Preview request/result coordination

### Plugin and Extension Classes
- `PluginHost`: Plugin lifecycle and sandboxing
- `ExtensionRegistry`: Extension discovery and activation
- `CapabilityProvider`: Plugin capability and trust management
- `ScriptBridge`: Script integration surface

### Serialization Classes
- `WorkspaceSerializer`: Workspace schema and persistence
- `LayoutSerializer`: Layout state persistence
- `PreferenceSerializer`: User preference persistence

### Indexing and Search Classes
- `AssetIndexer`: Asset search index (read-only view of tooling index)
- `EntityIndexer`: Entity search index (read-only view of world state)
- `CommandIndexer`: Command palette search index

### Diagnostics Classes
- `DiagnosticRouter`: Diagnostic message routing
- `DiagnosticPresenter`: Diagnostic panel presentation
- `ValidationCoordinator`: Validation request/result coordination

### Collaboration Classes
- `CollaborationSession`: Collaboration session coordination
- `ReviewCoordinator`: Review and annotation coordination
- `PlaytestCoordinator`: Playtest session coordination

## Prohibited Library Role Leakage

The editor package must NOT:
- Own world truth or entity authority (consumed from engine via tooling)
- Own asset compilation or transformation (consumed from tooling)
- Own file system or package management (consumed from tooling)
- Own network transport or protocol (consumed from sdk)
- Own rendering pipelines or shader compilation (consumed from engine via tooling)
- Own simulation or physics (consumed from engine)
- Maintain shadow truth or parallel state
- Bypass lower-stack authority

## Boundary Preservation

All editor library classes must:
- Consume lower-stack truth via sanctioned surfaces
- Never write directly to lower-stack state
- Never maintain parallel truth
- Never bypass authority boundaries
- Release resources when surfaces are cold

## Concrete External Freeze Set

Pinned editor-local external baseline:
- `serde`
- `serde_json`
- `thiserror`
- `smallvec`
- `tracing`
- `uuid`

Conditionally allowed only when a lower editor document explicitly binds them to an editor-local surface:
- `egui`
- `egui-winit`
- `egui-wgpu`
- `winit`
- `parking_lot`

The editor package must not introduce additional third-party libraries until they are first declared in this root baseline and then bound to a concrete editor-local role.

This freeze set applies only to editor-local ownership. It does not authorize direct ownership of engine, sdk, or tooling responsibilities.

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
