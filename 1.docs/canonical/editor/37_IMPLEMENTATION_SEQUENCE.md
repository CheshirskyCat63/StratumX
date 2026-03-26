# Editor Implementation Sequence

## Purpose

This document defines the actual build order for the editor package implementation.

## Scope

This sequence covers the complete editor package (`L8` through `L11`) implementation order.

## Phase 1: Foundation (L8.0-L8.1)

### Step 1.1: Editor Shell (L8.0)
- Implement `EditorShell`: Top-level window and menu host
- Implement `LayoutManager`: Workspace layout persistence
- Implement `CommandDispatcher`: Command palette and shortcut routing
- Implement workspace schema and migration
- Implement autosave and recovery
- Test: Shell composition, layout persistence, workspace migration

### Step 1.2: Viewport System (L8.1)
- Implement `ViewportSurface`: 3D/2D viewport render target
- Implement viewport navigation (pan, zoom, rotate, fly)
- Implement multi-viewport support
- Implement viewport modes (perspective, orthographic, top, front, side)
- Implement viewport focus and input routing
- Test: Viewport rendering, navigation, multi-viewport, focus

## Phase 2: Core Panels (L8.2-L8.5)

### Step 2.1: Outliner System (L8.2)
- Implement `WorldOutliner`: Entity hierarchy browser
- Implement `EntityBrowser`: Entity search and filter
- Implement outliner-viewport selection sync
- Test: Outliner display, selection sync, search/filter

### Step 2.2: Content Browser System (L8.3)
- Implement `ContentBrowser`: Asset browser
- Implement `PackageBrowser`: Package browser
- Implement asset search and filter
- Implement asset preview
- Test: Content browser display, search/filter, preview

### Step 2.3: Inspector System (L8.4)
- Implement `PropertyInspector`: Entity property editor
- Implement `DetailsPanel`: Asset details editor
- Implement inspector-viewport selection sync
- Test: Inspector display, property editing, selection sync

### Step 2.4: Tool Context System (L8.5)
- Implement `ModeManager`: Tool mode and context management
- Implement tool activation and deactivation
- Implement mode-specific input routing
- Test: Tool activation, mode switching, input routing

## Phase 3: Interaction and Overlays (L8.6-L8.8)

### Step 3.1: Overlay and Gizmo System (L8.6)
- Implement `OverlayRenderer`: Gizmo and overlay composition
- Implement gizmos (translation, rotation, scale, bounds, path)
- Implement overlays (grid, axis, bounds, wireframe, normals, colliders, lights, cameras)
- Test: Gizmo rendering, gizmo manipulation, overlay rendering

### Step 3.2: Workspace Layout System (L8.7)
- Implement `PanelHost`: Dockable panel container
- Implement panel docking and undocking
- Implement panel resize and minimize
- Implement layout persistence
- Test: Panel docking, layout persistence, panel resize

### Step 3.3: Interaction Routing System (L8.8)
- Implement `SelectionManager`: Cross-surface selection state
- Implement `FocusRouter`: Focus and input routing
- Implement selection-viewport-outliner-inspector sync
- Implement undo/redo with command journal
- Test: Selection sync, focus routing, undo/redo

## Phase 4: Assistant, Diagnostics, Build (L8.9-L8.11)

### Step 4.1: Assistant Surface (L8.9)
- Implement `AssistantPanel`: Assistant UI
- Implement assistant request/result coordination
- Implement assistant-viewport integration
- Test: Assistant requests, assistant results, assistant-viewport integration

### Step 4.2: Diagnostics Surface (L8.10)
- Implement `DiagnosticsPanel`: Diagnostics UI
- Implement diagnostics request/result coordination
- Implement diagnostics-viewport integration
- Test: Diagnostics requests, diagnostics results, diagnostics-viewport integration

### Step 4.3: Build/Release Surface (L8.11)
- Implement `BuildPanel`: Build UI
- Implement `ReleasePanel`: Release UI
- Implement build request/result coordination
- Implement release request/result coordination
- Test: Build requests, build results, release requests, release results

## Phase 5: Domain Suites (L9.0-L9.3)

### Step 5.1: World/Scene Suite (L9.0)
- Implement `WorldSuite`: World authoring surface
- Implement `SceneSuite`: Scene authoring surface
- Implement suite activation and deactivation
- Test: Suite activation, suite deactivation, suite-viewport integration

### Step 5.2: Terrain/Material/Environment Suite (L9.1)
- Implement `TerrainSuite`: Terrain authoring surface
- Implement `MaterialSuite`: Material authoring surface
- Implement `EnvironmentSuite`: Environment authoring surface
- Test: Suite activation, suite deactivation, suite-viewport integration

### Step 5.3: Destruction/Simulation Suite (L9.2)
- Implement `DestructionSuite`: Destruction authoring surface
- Implement `SimulationSuite`: Simulation authoring surface
- Test: Suite activation, suite deactivation, suite-viewport integration

### Step 5.4: Animation/Audio/UI Suite (L9.3)
- Implement `AnimationSuite`: Animation authoring surface
- Implement `AudioSuite`: Audio authoring surface
- Implement `UISuite`: UI authoring surface
- Test: Suite activation, suite deactivation, suite-viewport integration

## Phase 6: Editor Services (L10.0-L10.7)

### Step 6.1: Project Bootstrap Service (L10.0)
- Implement `ProjectBootstrap`: Project creation and initialization
- Test: Project creation, project initialization

### Step 6.2: Import/Export Pipeline Service (L10.1)
- Implement `ImportExport`: Import/export request/result coordination
- Implement import/export UI
- Test: Import requests, import results, export requests, export results

### Step 6.3: Graph Authoring Service (L10.2)
- Implement `GraphAuthoring`: Graph authoring surface
- Test: Graph authoring, graph-viewport integration

### Step 6.4: Automation and Batch Service (L10.3)
- Implement `Automation`: Automation request/result coordination
- Test: Automation requests, automation results

### Step 6.5: Script and Hot-Reload Service (L10.4)
- Implement `ScriptHotReload`: Script integration and hot-reload
- Test: Script execution, hot-reload

### Step 6.6: Plugin and Extension Host (L10.5)
- Implement `PluginHost`: Plugin lifecycle and sandboxing
- Implement `ExtensionRegistry`: Extension discovery and activation
- Implement `CapabilityProvider`: Plugin capability and trust management
- Test: Plugin lifecycle, plugin sandboxing, extension activation

### Step 6.7: Template/Preset and Scaffold Service (L10.6)
- Implement `TemplatePreset`: Template and preset management
- Test: Template creation, preset application

### Step 6.8: Package/Market and Dependency Service (L10.7)
- Implement `PackageMarket`: Package discovery and installation
- Test: Package discovery, package installation

## Phase 7: Collaboration Surfaces (L11.0-L11.5)

### Step 7.1: Collaboration Session Surface (L11.0)
- Implement `CollaborationSession`: Collaboration session coordination
- Test: Session start, session end, session sync

### Step 7.2: Review/Annotation Surface (L11.1)
- Implement `ReviewCoordinator`: Review and annotation coordination
- Test: Review creation, annotation creation, review-viewport integration

### Step 7.3: Asset Gate and Approval Surface (L11.2)
- Implement asset gate and approval UI
- Test: Asset gate, asset approval

### Step 7.4: Playtest and Capture Operations (L11.3)
- Implement `PlaytestCoordinator`: Playtest session coordination
- Test: Playtest start, playtest end, playtest recording

### Step 7.5: Production Dashboard and Traceability (L11.4)
- Implement production dashboard UI
- Test: Production dashboard display, traceability

### Step 7.6: Learning/Onboarding and Help Surface (L11.5)
- Implement learning and onboarding UI
- Test: Learning content display, onboarding flow

## Phase 8: Integration and Polish

### Step 8.1: Integration Testing
- Test all surfaces together
- Test cross-surface interactions
- Test performance budgets
- Test activation discipline
- Test no shadow truth

### Step 8.2: Performance Optimization
- Optimize viewport rendering
- Optimize panel updates
- Optimize background jobs
- Optimize memory usage
- Optimize disk usage

### Step 8.3: Polish
- Polish UI
- Polish UX
- Polish error messages
- Polish documentation

## Phase 9: Evidence and Closure

### Step 9.1: Evidence Generation
- Generate all evidence artifacts
- Verify all acceptance criteria
- Verify all readiness criteria

### Step 9.2: Gold Freeze
- Freeze implementation
- Freeze documentation
- Freeze evidence

## Dependencies

Each phase depends on the previous phase:
- Phase 2 depends on Phase 1 (panels need shell and viewport)
- Phase 3 depends on Phase 2 (interaction needs panels)
- Phase 4 depends on Phase 3 (assistant/diagnostics/build need interaction)
- Phase 5 depends on Phase 4 (suites need assistant/diagnostics/build)
- Phase 6 depends on Phase 5 (services need suites)
- Phase 7 depends on Phase 6 (collaboration needs services)
- Phase 8 depends on Phase 7 (integration needs all surfaces)
- Phase 9 depends on Phase 8 (evidence needs integration)

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
