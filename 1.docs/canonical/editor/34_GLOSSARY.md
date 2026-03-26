# Editor Glossary

## Purpose

This document defines editor-specific terms used throughout the editor canonical package.

## Scope

This glossary covers terms specific to the editor package (`L8` through `L11`).

## Terms

### Shell
The top-level editor window and menu host. The shell owns layout, docking, and workspace persistence.

### Viewport
A 3D or 2D render surface for world visualization. Viewports consume world truth via `L6.0` authority-core and rendering via `L6.5` preview-runtime.

### Panel
A dockable UI surface (outliner, content browser, inspector, diagnostics, assistant, build/release, collaboration).

### Overlay
A visual element rendered on top of the viewport (grid, axis, bounds, wireframe, normals, colliders, lights, cameras).

### Gizmo
An interactive visual element for manipulation (translation, rotation, scale, bounds, path).

### Suite
A domain-specific authoring surface (world, scene, terrain, material, environment, destruction, simulation, animation, audio, UI). Suites are universal and non-project-specific.

### Service
An editor-wide service (project bootstrap, import/export, graph authoring, automation, script/hot-reload, plugin/extension, template/preset, package/market).

### Mode
A tool context (select, move, rotate, scale, paint, sculpt, place, measure).

### Tool Context
A domain context for tool activation (world, terrain, material, animation, audio, UI).

### Selection
The set of entities, assets, or components currently selected. Selection is cross-surface: viewport, outliner, inspector, content browser.

### Focus
The currently focused panel, viewport, or input target. Focus determines input routing and shortcut context.

### Command
A user-invokable action (file, edit, view, select, tool, build, debug, collaborate, assistant, help).

### Request/Result
A structured message pair for async operations (preview, diagnostics, build, assistant, collaboration).

### Invalidation
A signal that a surface needs to refresh (world changed, asset changed, selection changed, layout changed, mode changed).

### Activation
The process of loading resources for a surface (panel, viewport, suite, service).

### Deactivation
The process of releasing resources for a surface (panel, viewport, suite, service).

### Hot Surface
A surface that is visible and actively consuming resources.

### Warm Surface
A surface that is not visible but retains some resources for fast reactivation.

### Cold Surface
A surface that is not visible and has released all resources.

### Shadow Truth
Forbidden parallel state maintained by the editor. The editor must never maintain shadow truth.

### Authority Leakage
Forbidden ownership of lower-stack truth by the editor. The editor must never own world truth, asset authority, or package state.

### Capability
A permission granted to a plugin or extension (read world, write world, read assets, write assets, execute commands).

### Trust
A security level for a plugin or extension (trusted, untrusted, sandboxed).

### Sandbox
An isolated execution environment for untrusted plugins or extensions.

### Collaboration Session
A real-time multi-user editing session.

### Review Annotation
A comment or note attached to an entity, asset, or location.

### Playtest Session
A collaborative gameplay testing session with recording.

### Workspace
The editor's persistent state (layout, preferences, open files, recent projects).

### Layout
The arrangement of panels and viewports in the shell.

### Preference
A user-configurable setting (theme, shortcuts, autosave cadence, collaboration settings).

### Autosave
Automatic periodic saving of editor state.

### Recovery
Restoration of editor state after a crash.

### Undo/Redo
Reversible operations with a command journal.

### Migration
Conversion of workspace schema from one version to another.

### Variant
An alternative version of an asset (LOD, platform-specific, quality-specific).

### Budget
A resource limit for a surface (memory, CPU, GPU, disk, network).

### Cancellation
The process of stopping a background job (preview, import, export, build, diagnostics, indexing).

### Job
A background operation (preview, import, export, build, diagnostics, indexing, collaboration).

### Ownership
The relationship between a surface and its jobs or resources. Jobs and resources are cancelled/released when the owning surface is closed.

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
